//! A minimal HTTP/1.1 for one local user: read one request from a stream,
//! write one response, close. Enough for the page's GETs and small JSON
//! POSTs; no keep-alive, chunked bodies or compression.

use std::io::{BufRead, Read, Write};

/// Request line and headers together are at most this long.
const MAX_HEAD_BYTES: usize = 16 * 1024;
/// The page posts small JSON objects.
const MAX_BODY_BYTES: usize = 64 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Other,
}

#[derive(Debug, PartialEq)]
pub struct Request {
    pub method: Method,
    /// Percent-decoded, without the query.
    pub path: String,
    /// Percent-decoded pairs, in order.
    pub query: Vec<(String, String)>,
    /// Names in lowercase.
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(header, _)| header == name)
            .map(|(_, value)| value.as_str())
    }

    pub fn parameter(&self, name: &str) -> Option<&str> {
        self.query
            .iter()
            .find(|(parameter, _)| parameter == name)
            .map(|(_, value)| value.as_str())
    }
}

/// Reads one request: the request line, the headers up to the blank line,
/// and a body of `Content-Length` bytes.
pub fn read_request(stream: &mut impl BufRead) -> Result<Request, String> {
    let mut head = Vec::new();
    loop {
        // Room for the rest of the head and its blank line; a longer line
        // stops the read without its newline.
        let room = (MAX_HEAD_BYTES - head.len()) as u64 + 2;
        let mut line = Vec::new();
        let read = stream
            .by_ref()
            .take(room)
            .read_until(b'\n', &mut line)
            .map_err(|error| format!("cannot read the request: {error}"))?;
        if !line.ends_with(b"\n") {
            return Err(if read as u64 == room {
                String::from("the request's head is too long")
            } else {
                String::from("the connection closed before the request ended")
            });
        }
        if line == b"\r\n" || line == b"\n" {
            break;
        }
        if head.len() + line.len() > MAX_HEAD_BYTES {
            return Err(String::from("the request's head is too long"));
        }
        head.extend_from_slice(&line);
    }
    let head =
        String::from_utf8(head).map_err(|_| String::from("the request's head is not UTF-8"))?;
    let mut lines = head.lines();
    let request_line = lines.next().ok_or("an empty request")?;
    let mut parts = request_line.split(' ');
    let (Some(method), Some(target), Some(version), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return Err(format!("not a request line: `{request_line}`"));
    };
    if !version.starts_with("HTTP/1.") {
        return Err(format!("unsupported version {version}"));
    }
    let headers: Vec<(String, String)> = lines
        .map(|line| {
            line.split_once(':')
                .map(|(name, value)| (name.trim().to_ascii_lowercase(), value.trim().to_owned()))
                .ok_or_else(|| format!("not a header: `{line}`"))
        })
        .collect::<Result<_, _>>()?;
    let length = headers
        .iter()
        .find(|(name, _)| name == "content-length")
        .map_or(Ok(0), |(_, value)| value.parse::<usize>())
        .map_err(|_| String::from("Content-Length is not a number"))?;
    if length > MAX_BODY_BYTES {
        return Err(String::from("the request's body is too long"));
    }
    let mut body = vec![0; length];
    stream
        .read_exact(&mut body)
        .map_err(|error| format!("cannot read the request's body: {error}"))?;
    let (path, query) = target.split_once('?').unwrap_or((target, ""));
    Ok(Request {
        method: match method {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Other,
        },
        path: percent_decode(path).ok_or("the path is not valid percent-encoded UTF-8")?,
        query: query
            .split('&')
            .filter(|pair| !pair.is_empty())
            .map(|pair| {
                let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
                Some((percent_decode(name)?, percent_decode(value)?))
            })
            .collect::<Option<_>>()
            .ok_or("the query is not valid percent-encoded UTF-8")?,
        headers,
        body,
    })
}

/// `%XX` escapes and `+` (in queries, a space) decoded.
fn percent_decode(text: &str) -> Option<String> {
    let mut bytes = Vec::with_capacity(text.len());
    let mut rest = text.as_bytes();
    while let Some((&byte, after)) = rest.split_first() {
        match byte {
            b'%' => {
                let hex = std::str::from_utf8(after.get(..2)?).ok()?;
                bytes.push(u8::from_str_radix(hex, 16).ok()?);
                rest = &after[2..];
            }
            b'+' => {
                bytes.push(b' ');
                rest = after;
            }
            _ => {
                bytes.push(byte);
                rest = after;
            }
        }
    }
    String::from_utf8(bytes).ok()
}

#[derive(Debug, PartialEq)]
pub struct Response {
    pub status: u16,
    pub content_type: &'static str,
    pub body: Vec<u8>,
}

impl Response {
    pub fn ok(content_type: &'static str, body: Vec<u8>) -> Response {
        Response {
            status: 200,
            content_type,
            body,
        }
    }

    pub fn json(status: u16, value: &impl serde::Serialize) -> Response {
        match serde_json::to_vec(value) {
            Ok(body) => Response {
                status,
                content_type: "application/json",
                body,
            },
            Err(error) => Response::text(500, &format!("cannot write JSON: {error}")),
        }
    }

    /// A plain-text message, for errors.
    pub fn text(status: u16, message: &str) -> Response {
        Response {
            status,
            content_type: "text/plain; charset=utf-8",
            body: message.as_bytes().to_vec(),
        }
    }

    /// The status line, the headers every response carries, the body.
    pub fn write_to(&self, stream: &mut impl Write) -> std::io::Result<()> {
        let head = format!(
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nReferrer-Policy: no-referrer\r\nContent-Security-Policy: default-src 'none'; script-src 'self'; style-src 'self'; connect-src 'self'; img-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'\r\nConnection: close\r\n\r\n",
            self.status,
            reason(self.status),
            self.content_type,
            self.body.len()
        );
        stream.write_all(head.as_bytes())?;
        stream.write_all(&self.body)?;
        stream.flush()
    }
}

fn reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        400 => "Bad Request",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        409 => "Conflict",
        421 => "Misdirected Request",
        503 => "Service Unavailable",
        _ => "Internal Server Error",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_get_is_read_with_its_query_and_headers() {
        let raw = "GET /tok/api/audio?file=%2Fmusic%2FA%20b.mp3&window=3 HTTP/1.1\r\nHost: 127.0.0.1:8000\r\nAccept: */*\r\n\r\n";

        let request = read_request(&mut raw.as_bytes()).unwrap();

        assert_eq!(request.method, Method::Get);
        assert_eq!(request.path, "/tok/api/audio");
        assert_eq!(request.parameter("file"), Some("/music/A b.mp3"));
        assert_eq!(request.parameter("window"), Some("3"));
        assert_eq!(request.header("host"), Some("127.0.0.1:8000"));
        assert!(request.body.is_empty());
    }

    #[test]
    fn a_post_is_read_with_its_body() {
        let raw = "POST /tok/api/answer HTTP/1.1\r\nHost: x\r\nContent-Type: application/json\r\nContent-Length: 13\r\n\r\n{\"key\": \"s\"}\nleft over";

        let request = read_request(&mut raw.as_bytes()).unwrap();

        assert_eq!(request.method, Method::Post);
        assert_eq!(request.body, b"{\"key\": \"s\"}\n");
    }

    #[test]
    fn malformed_or_oversized_requests_are_refused() {
        assert!(read_request(&mut "GET /\r\n\r\n".as_bytes()).is_err());
        assert!(read_request(&mut "GET / HTTP/2\r\n\r\n".as_bytes()).is_err());
        assert!(read_request(&mut "GET / HTTP/1.1\r\nno colon\r\n\r\n".as_bytes()).is_err());
        assert!(read_request(&mut "GET /%zz HTTP/1.1\r\n\r\n".as_bytes()).is_err());
        assert!(read_request(&mut "GET / HTTP/1.1\r\nHost: x".as_bytes()).is_err());
        let long = format!("GET /{} HTTP/1.1\r\n\r\n", "a".repeat(MAX_HEAD_BYTES));
        assert!(
            read_request(&mut long.as_bytes())
                .unwrap_err()
                .contains("too long")
        );
        let big = format!(
            "POST / HTTP/1.1\r\nContent-Length: {}\r\n\r\n",
            MAX_BODY_BYTES + 1
        );
        assert!(
            read_request(&mut big.as_bytes())
                .unwrap_err()
                .contains("too long")
        );
    }

    #[test]
    fn a_response_carries_its_length_and_the_security_headers() {
        let mut written = Vec::new();

        Response::text(403, "no").write_to(&mut written).unwrap();

        let text = String::from_utf8(written).unwrap();
        assert!(text.starts_with("HTTP/1.1 403 Forbidden\r\n"), "{text}");
        assert!(text.contains("Content-Length: 2\r\n"));
        assert!(text.contains("Content-Security-Policy: default-src 'none'"));
        assert!(text.contains("Connection: close\r\n"));
        assert!(text.ends_with("\r\n\r\nno"));
    }
}
