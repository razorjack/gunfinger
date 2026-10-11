//! The page's files, embedded in the binary: plain HTML, CSS and
//! JavaScript modules, no build step. `web/app.js` is the entry point;
//! each module's header says what it does.

pub struct Asset {
    pub body: &'static str,
    pub content_type: &'static str,
}

const HTML: &str = "text/html; charset=utf-8";
const CSS: &str = "text/css; charset=utf-8";
const JS: &str = "text/javascript; charset=utf-8";

/// The embedded file at `name`, the route after the token.
pub fn asset(name: &str) -> Option<Asset> {
    let (body, content_type) = match name {
        "" | "index.html" => (include_str!("web/index.html"), HTML),
        "style.css" => (include_str!("web/style.css"), CSS),
        "app.js" => (include_str!("web/app.js"), JS),
        "api.js" => (include_str!("web/api.js"), JS),
        "keys.js" => (include_str!("web/keys.js"), JS),
        "lanes.js" => (include_str!("web/lanes.js"), JS),
        "player.js" => (include_str!("web/player.js"), JS),
        "queue.js" => (include_str!("web/queue.js"), JS),
        "timeline.js" => (include_str!("web/timeline.js"), JS),
        "windows.js" => (include_str!("web/windows.js"), JS),
        _ => return None,
    };
    Some(Asset { body, content_type })
}
