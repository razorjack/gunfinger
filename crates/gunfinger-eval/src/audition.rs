//! The listening tool (ADR 0016): a local web page on which
//! listening questions are answered by ear, switching between files at
//! the same musical moment.
//!
//! - `schema`: the question file, its check and stable ids.
//! - `placement`: where lanes play on a question's timeline.
//! - `names`: harness names (`second-library/...`) and absolute paths.
//! - `pack`, `folders`, `detections`, `shared`: the producers, which turn
//!   evidence into question files and know nothing of HTTP or the page.
//! - `answers` and `effects`: the answer log, undo, and what an answer
//!   writes elsewhere (the pair verdicts file).
//! - `audio`: windows of a file as WAV and waveform envelopes, by FFmpeg.
//! - `server` (with `http` and `session`): the HTTP server on 127.0.0.1,
//!   its token and `Host` checks, and the files it may serve; `web`: the
//!   page's embedded files (`audition/web/`); `desktop`: the commands that
//!   open the page and show a file, for macOS and Linux.
//!
//! `docs/audition-questions.md` is the format's reference; AGENTS.md says
//! how to add a kind of question.

pub mod answers;
pub mod audio;
mod desktop;
pub mod detections;
pub mod effects;
pub mod folders;
mod http;
pub mod names;
pub mod pack;
pub mod placement;
pub mod schema;
pub mod server;
mod session;
pub mod shared;
mod web;
