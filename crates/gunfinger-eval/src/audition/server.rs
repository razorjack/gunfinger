//! `gunfinger-eval audition`: serves the page on 127.0.0.1 at a free
//! port, one thread per connection, and opens it in the default browser.
//!
//! Every request must name the server's address in its `Host` header
//! (which defeats DNS rebinding) and start its path with the random token
//! of the printed URL (which a page of another origin cannot know). Only
//! the files the loaded questions list are decoded, described or revealed.

use std::fs::File;
use std::io::{BufReader, Read};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::Stdio;
use std::thread;
use std::time::Duration;

use serde::Deserialize;

use crate::audition::answers::AnswerBook;
use crate::audition::audio::{self, Envelopes};
use crate::audition::desktop;
use crate::audition::effects::Refusal;
use crate::audition::http::{Method, Request, Response, read_request};
use crate::audition::session::Session;
use crate::audition::web;
use crate::platform::Platform;

pub struct Options {
    pub question_files: Vec<PathBuf>,
    pub answers: PathBuf,
    pub verdicts: PathBuf,
    pub work: PathBuf,
    pub open_browser: bool,
    /// 0 for a free port.
    pub port: u16,
}

/// A client that sends nothing for this long is dropped.
const READ_TIMEOUT: Duration = Duration::from_secs(30);

pub fn serve(options: &Options) -> Result<(), String> {
    let book = AnswerBook {
        log: options.answers.clone(),
        verdicts: options.verdicts.clone(),
    };
    let _log_lock = book.lock_log()?;
    let session = Session::load(&options.question_files, book)?;
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, options.port))
        .map_err(|error| format!("cannot listen on 127.0.0.1:{}: {error}", options.port))?;
    let port = listener
        .local_addr()
        .map_err(|error| error.to_string())?
        .port();
    let app = App {
        guard: Guard {
            token: random_token()?,
            port,
        },
        session,
        envelopes: Envelopes::below(&options.work),
    };
    let url = app.guard.url();
    println!("The listening page: {url}");
    println!(
        "Answers go to {}; pair verdicts to {}. Ctrl-C stops the server.",
        options.answers.display(),
        options.verdicts.display()
    );
    thread::scope(|scope| {
        if options.open_browser {
            scope.spawn(|| open_in_browser(&url));
        }
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let app = &app;
                    scope.spawn(move || app.handle(&stream));
                }
                Err(error) => eprintln!("warning: a connection failed: {error}"),
            }
        }
    });
    Ok(())
}

/// Opens the page in the default browser, on a thread of its own: outside
/// a desktop environment `xdg-open` may run the browser itself and wait
/// until it closes.
fn open_in_browser(url: &str) {
    let mut command = desktop::browser(Platform::CURRENT, url);
    match command.stdin(Stdio::null()).status() {
        Ok(status) if status.success() => {}
        Ok(status) => eprintln!(
            "warning: `{}` failed ({status}); open the address above",
            desktop::shown(&command)
        ),
        Err(error) => eprintln!(
            "warning: cannot open a browser with `{}` ({error}); open the address above",
            command.get_program().to_string_lossy()
        ),
    }
}

/// 128 random bits as hex, from the system's generator.
fn random_token() -> Result<String, String> {
    let mut bytes = [0_u8; 16];
    File::open("/dev/urandom")
        .and_then(|mut source| source.read_exact(&mut bytes))
        .map_err(|error| format!("cannot read /dev/urandom for the page's token: {error}"))?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}

/// The checks every request passes before it is served.
pub struct Guard {
    pub token: String,
    pub port: u16,
}

impl Guard {
    pub fn url(&self) -> String {
        format!("http://127.0.0.1:{}/{}/", self.port, self.token)
    }

    /// The route after the token, for a request to this server's address
    /// that carries the token.
    pub fn admit<'r>(&self, request: &'r Request) -> Result<&'r str, Response> {
        let host = request.header("host").unwrap_or_default();
        let ours = [
            format!("127.0.0.1:{}", self.port),
            format!("localhost:{}", self.port),
        ];
        if !ours.iter().any(|address| address == host) {
            return Err(Response::text(
                421,
                "this server answers requests to its own address only",
            ));
        }
        if request.method == Method::Post
            && let Some(origin) = request.header("origin")
            && !ours
                .iter()
                .any(|address| origin == format!("http://{address}"))
        {
            return Err(Response::text(403, "a request from another page"));
        }
        let path = request.path.strip_prefix('/').unwrap_or_default();
        let (token, route) = path.split_once('/').unwrap_or((path, ""));
        if !same_secret(token, &self.token) {
            return Err(Response::text(
                403,
                "open the address `gunfinger-eval audition` printed; it carries the page's token",
            ));
        }
        Ok(route)
    }
}

/// Compares in time independent of where the strings differ.
fn same_secret(given: &str, secret: &str) -> bool {
    given.len() == secret.len()
        && given
            .bytes()
            .zip(secret.bytes())
            .fold(0, |difference, (a, b)| difference | (a ^ b))
            == 0
}

pub struct App {
    pub guard: Guard,
    pub session: Session,
    pub envelopes: Envelopes,
}

#[derive(Deserialize)]
struct AnswerBody {
    question: String,
    key: char,
}

#[derive(Deserialize)]
struct RevealBody {
    file: String,
}

impl App {
    fn handle(&self, stream: &TcpStream) {
        // A failed timeout only lets a silent client hold its thread.
        let _ = stream.set_read_timeout(Some(READ_TIMEOUT));
        let mut reader = BufReader::new(stream);
        let response = match read_request(&mut reader) {
            Ok(request) => self.respond(&request),
            Err(error) => Response::text(400, &error),
        };
        if let Err(error) = response.write_to(&mut &*stream) {
            eprintln!("warning: cannot answer the page: {error}");
        }
    }

    /// The response to one request.
    pub fn respond(&self, request: &Request) -> Response {
        let route = match self.guard.admit(request) {
            Ok(route) => route,
            Err(refusal) => return refusal,
        };
        // A plain HTML form cannot post JSON.
        let json = request
            .header("content-type")
            .is_some_and(|kind| kind.starts_with("application/json"));
        if request.method == Method::Post && !json {
            return Response::text(400, "send JSON");
        }
        match (request.method, route) {
            (Method::Get, "api/session") => match self.session.overview() {
                Ok(overview) => Response::json(200, &overview),
                Err(error) => Response::text(500, &error),
            },
            (Method::Get, "api/audio") => self.audio(request),
            (Method::Get, "api/envelope") => self.envelope(request),
            (Method::Post, "api/answer") => self.answer(request),
            (Method::Post, "api/undo") => match self.session.undo() {
                Ok(reply) => Response::json(200, &reply),
                Err(error) => Response::text(500, &error),
            },
            (Method::Post, "api/reveal") => self.reveal(request),
            (Method::Get, name) => web::asset(name).map_or_else(
                || Response::text(404, "no such page"),
                |asset| Response::ok(asset.content_type, asset.body.as_bytes().to_vec()),
            ),
            _ => Response::text(405, "not a request this page makes"),
        }
    }

    fn listed_file(&self, path: Option<&str>) -> Result<PathBuf, Response> {
        let path = path.ok_or_else(|| Response::text(400, "name a file"))?;
        self.session
            .listed()
            .get(path)
            .map(PathBuf::from)
            .ok_or_else(|| Response::text(403, "the loaded questions do not list this file"))
    }

    fn audio(&self, request: &Request) -> Response {
        let file = match self.listed_file(request.parameter("file")) {
            Ok(file) => file,
            Err(refusal) => return refusal,
        };
        let Some(window) = request
            .parameter("window")
            .and_then(|window| window.parse().ok())
        else {
            return Response::text(400, "name a window: 0, 1, 2 ...");
        };
        // 404 tells the page the file has ended; 503 that it may try again.
        match audio::window_wav(&file, window) {
            Ok(Some(wav)) => Response::ok("audio/wav", wav),
            Ok(None) => Response::text(404, "the file ends before this window"),
            Err(error) => Response::text(503, &error),
        }
    }

    fn envelope(&self, request: &Request) -> Response {
        match self.listed_file(request.parameter("file")) {
            Ok(file) => match self.envelopes.get(&file) {
                Ok(envelope) => Response::json(200, &envelope),
                Err(error) => Response::text(500, &error),
            },
            Err(refusal) => refusal,
        }
    }

    fn answer(&self, request: &Request) -> Response {
        let body: AnswerBody = match json_body(request) {
            Ok(body) => body,
            Err(refusal) => return refusal,
        };
        match self.session.answer(&body.question, body.key) {
            Ok(reply) => Response::json(200, &reply),
            Err(Refusal::NoSuchKey(message)) => Response::text(400, &message),
            Err(Refusal::Contradicts(message)) => Response::text(409, &message),
            Err(Refusal::Io(message)) => Response::text(500, &message),
        }
    }

    fn reveal(&self, request: &Request) -> Response {
        let body: RevealBody = match json_body(request) {
            Ok(body) => body,
            Err(refusal) => return refusal,
        };
        let file = match self.listed_file(Some(&body.file)) {
            Ok(file) => file,
            Err(refusal) => return refusal,
        };
        let mut command = desktop::reveal(Platform::CURRENT, &file);
        match command.stdin(Stdio::null()).status() {
            Ok(status) if status.success() => Response::json(200, &"revealed"),
            Ok(status) => Response::text(
                500,
                &format!("`{}` failed ({status})", desktop::shown(&command)),
            ),
            Err(error) => Response::text(
                500,
                &format!(
                    "cannot run `{}`: {error}",
                    command.get_program().to_string_lossy()
                ),
            ),
        }
    }
}

fn json_body<T: for<'de> Deserialize<'de>>(request: &Request) -> Result<T, Response> {
    serde_json::from_slice(&request.body).map_err(|error| Response::text(400, &error.to_string()))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use super::*;
    use crate::audition::audio::tests::tone;
    use crate::audition::schema::tests::pair_question;
    use crate::audition::schema::{self, QuestionFile, SCHEMA_VERSION};

    const TOKEN: &str = "0123456789abcdef0123456789abcdef";

    fn scratch(test: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "gunfinger-audition-server-{test}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// A server of one pair question whose lanes play `a.mp3` and
    /// `b.mp3` in `dir`, with its answers and verdicts in `dir`.
    fn app(dir: &Path) -> App {
        let mut question = pair_question();
        question.lanes[0].pieces[0].file = dir.join("a.mp3");
        for piece in &mut question.lanes[1].pieces {
            piece.file = dir.join("b.mp3");
        }
        let file = QuestionFile {
            schema_version: SCHEMA_VERSION,
            title: String::from("Pairs"),
            made_by: String::from("test"),
            questions: vec![question],
        };
        let path = dir.join("questions.json");
        schema::save(&path, &file).unwrap();
        App {
            guard: Guard {
                token: String::from(TOKEN),
                port: 8123,
            },
            session: Session::load(
                &[path],
                AnswerBook {
                    log: dir.join("answers.jsonl"),
                    verdicts: dir.join("pair-verdicts.txt"),
                },
            )
            .unwrap(),
            envelopes: Envelopes {
                dir: dir.join("envelopes"),
            },
        }
    }

    fn request(method: Method, target: &str, host: &str, body: &str) -> Request {
        let (path, query) = target.split_once('?').unwrap_or((target, ""));
        Request {
            method,
            path: path.to_owned(),
            query: query
                .split('&')
                .filter(|pair| !pair.is_empty())
                .map(|pair| {
                    let (name, value) = pair.split_once('=').unwrap();
                    (name.to_owned(), value.to_owned())
                })
                .collect(),
            headers: vec![
                (String::from("host"), host.to_owned()),
                (
                    String::from("content-type"),
                    String::from("application/json"),
                ),
            ],
            body: body.as_bytes().to_vec(),
        }
    }

    fn get(app: &App, route: &str) -> Response {
        app.respond(&request(
            Method::Get,
            &format!("/{TOKEN}/{route}"),
            "127.0.0.1:8123",
            "",
        ))
    }

    fn post(app: &App, route: &str, body: &str) -> Response {
        app.respond(&request(
            Method::Post,
            &format!("/{TOKEN}/{route}"),
            "127.0.0.1:8123",
            body,
        ))
    }

    fn text(response: &Response) -> String {
        String::from_utf8(response.body.clone()).unwrap()
    }

    #[test]
    fn a_server_refuses_to_start_on_a_log_another_server_holds() {
        let dir = scratch("second-server");
        let book = AnswerBook {
            log: dir.join("answers.jsonl"),
            verdicts: dir.join("pair-verdicts.txt"),
        };
        let _held = book.lock_log().unwrap();

        let refused = serve(&Options {
            question_files: Vec::new(),
            answers: book.log.clone(),
            verdicts: book.verdicts.clone(),
            work: dir.clone(),
            open_browser: false,
            port: 0,
        })
        .unwrap_err();

        assert!(refused.contains("is answering into"), "{refused}");
    }

    #[test]
    fn requests_need_the_token_and_the_servers_own_host() {
        let dir = scratch("guard");
        let app = app(&dir);

        assert_eq!(get(&app, "").status, 200);
        assert_eq!(get(&app, "").content_type, "text/html; charset=utf-8");
        assert_eq!(
            get(&app, "player.js").content_type,
            "text/javascript; charset=utf-8"
        );
        let wrong_token = format!("/{}/api/session", "f".repeat(32));
        assert_eq!(
            app.respond(&request(Method::Get, &wrong_token, "127.0.0.1:8123", ""))
                .status,
            403
        );
        assert_eq!(
            app.respond(&request(Method::Get, "/api/session", "127.0.0.1:8123", ""))
                .status,
            403
        );
        let route = format!("/{TOKEN}/api/session");
        assert_eq!(
            app.respond(&request(Method::Get, &route, "localhost:8123", ""))
                .status,
            200
        );
        assert_eq!(
            app.respond(&request(Method::Get, &route, "evil.example:8123", ""))
                .status,
            421
        );
        assert_eq!(
            app.respond(&request(Method::Get, &route, "127.0.0.1:9999", ""))
                .status,
            421
        );
        let mut foreign = request(
            Method::Post,
            &format!("/{TOKEN}/api/undo"),
            "127.0.0.1:8123",
            "{}",
        );
        foreign
            .headers
            .push((String::from("origin"), String::from("http://evil.example")));
        assert_eq!(app.respond(&foreign).status, 403);
        assert_eq!(get(&app, "secrets.txt").status, 404);
        assert_eq!(app.guard.url(), format!("http://127.0.0.1:8123/{TOKEN}/"));
    }

    #[test]
    fn only_files_the_questions_list_are_served() {
        let dir = scratch("listed");
        let app = app(&dir);

        for refused in [
            "/etc/passwd",
            "",
            &format!("{}/../a.mp3", dir.join("x").display()),
        ] {
            let envelope = get(&app, &format!("api/envelope?file={refused}"));
            let audio = get(&app, &format!("api/audio?file={refused}&window=0"));
            let reveal = post(&app, "api/reveal", &format!("{{\"file\": \"{refused}\"}}"));
            assert_eq!(
                (envelope.status, audio.status, reveal.status),
                (403, 403, 403),
                "{refused}"
            );
        }
        assert_eq!(get(&app, "api/envelope").status, 400);
    }

    #[test]
    fn a_listed_file_is_decoded_in_windows_and_described() {
        let dir = scratch("audio");
        let app = app(&dir);
        if tone(&dir.join("a.mp3"), 2.0, 440).is_none() {
            eprintln!("skipped: ffmpeg cannot generate audio here");
            return;
        }
        let file = dir.join("a.mp3").display().to_string();

        let window = get(&app, &format!("api/audio?file={file}&window=0"));
        let envelope = get(&app, &format!("api/envelope?file={file}"));

        assert_eq!((window.status, window.content_type), (200, "audio/wav"));
        assert_eq!(&window.body[..4], b"RIFF");
        assert_eq!(envelope.status, 200, "{}", text(&envelope));
        assert!(text(&envelope).contains("\"bucket_seconds\":0.05"));
        assert_eq!(
            get(&app, &format!("api/audio?file={file}&window=1")).status,
            404
        );
        assert_eq!(get(&app, &format!("api/audio?file={file}")).status, 400);
    }

    #[test]
    fn answers_and_undo_go_through_the_session() {
        let dir = scratch("answers");
        let app = app(&dir);
        let id = pair_question().id;

        let refused = post(
            &app,
            "api/answer",
            &format!("{{\"question\": \"{}\", \"key\": \"m\"}}", id.as_str()),
        );
        let answered = post(
            &app,
            "api/answer",
            &format!("{{\"question\": \"{}\", \"key\": \"d\"}}", id.as_str()),
        );

        assert_eq!(refused.status, 400);
        assert_eq!(answered.status, 200, "{}", text(&answered));
        assert!(
            text(&answered).contains(r#""value":"different""#),
            "{}",
            text(&answered)
        );
        assert_eq!(
            fs::read_to_string(dir.join("pair-verdicts.txt")).unwrap(),
            "different\tsecond-library/a.mp3\tsecond-library/b.opus\n"
        );
        let session = text(&get(&app, "api/session"));
        assert!(
            session.contains(r#""state":{"value":"different""#),
            "{session}"
        );

        let undone = post(&app, "api/undo", "{}");
        assert_eq!(undone.status, 200);
        assert!(
            text(&undone).contains("removed its line"),
            "{}",
            text(&undone)
        );
        assert_eq!(
            fs::read_to_string(dir.join("pair-verdicts.txt")).unwrap(),
            ""
        );

        let mut form = request(
            Method::Post,
            &format!("/{TOKEN}/api/undo"),
            "127.0.0.1:8123",
            "a=b",
        );
        form.headers.retain(|(name, _)| name != "content-type");
        assert_eq!(app.respond(&form).status, 400);
        let unknown = post(
            &app,
            "api/answer",
            r#"{"question": "pair-0000000000000000", "key": "s"}"#,
        );
        assert_eq!(unknown.status, 400);
    }

    #[test]
    fn a_request_is_read_from_a_real_connection() {
        let dir = scratch("socket");
        let app = app(&dir);
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let address = listener.local_addr().unwrap();

        let reply = thread::scope(|scope| {
            scope.spawn(|| {
                let (stream, _) = listener.accept().unwrap();
                app.handle(&stream);
            });
            let mut client = TcpStream::connect(address).unwrap();
            std::io::Write::write_all(
                &mut client,
                format!("GET /{TOKEN}/api/session HTTP/1.1\r\nHost: 127.0.0.1:8123\r\n\r\n")
                    .as_bytes(),
            )
            .unwrap();
            let mut reply = String::new();
            client.read_to_string(&mut reply).unwrap();
            reply
        });

        assert!(reply.starts_with("HTTP/1.1 200 OK\r\n"), "{reply}");
        assert!(reply.contains("\"window_seconds\":30.0"), "{reply}");
    }
}
