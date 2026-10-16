//! FFmpeg and FFprobe as child processes that must keep writing.
//!
//! A run that writes nothing, to stdout or stderr, for the stall timeout is
//! killed and reported as stalled. The timeout counts silence, not the whole run, so a
//! two-hour mix on a slow share still decodes. A file on a network share
//! that hangs can leave the child in an uninterruptible read, where even a
//! kill waits for the share to answer; so stdout and stderr are read on
//! threads of their own and the child is reaped on another, none of which
//! the caller waits for once the run has stalled. They end when the child
//! does.

use std::io::{self, Read};
use std::process::{Child, ChildStdout, Command, ExitStatus, Stdio};
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::thread;
use std::time::{Duration, Instant};

/// How a watched run ended.
#[derive(Debug)]
pub enum Run<T> {
    /// The child closed its output and exited: what `read` made of stdout,
    /// the exit status and stderr.
    Finished {
        output: io::Result<T>,
        status: ExitStatus,
        diagnostics: String,
    },
    /// Nothing came for the stall timeout; the child was killed.
    Stalled,
}

/// Stdout (or stderr) of a watched child: every read that returns data
/// tells the watcher that the child is still writing.
pub struct Heartbeat<T, R = ChildStdout> {
    pipe: R,
    events: Sender<Event<T>>,
}

impl<T, R: Read> Read for Heartbeat<T, R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let read = self.pipe.read(buffer)?;
        if read > 0 {
            // The watcher stops listening only after a stall.
            let _ = self.events.send(Event::Beat);
        }
        Ok(read)
    }
}

enum Event<T> {
    Beat,
    Output(io::Result<T>),
    Diagnostics(String),
}

/// Runs `command` with stdin closed and stdout and stderr piped, passing
/// stdout to `read` on its own thread. Errors only when the child or a
/// thread to read it cannot be started.
pub fn run<T: Send + 'static>(
    command: &mut Command,
    stall: Duration,
    read: impl FnOnce(Heartbeat<T>) -> io::Result<T> + Send + 'static,
) -> io::Result<Run<T>> {
    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let (events, watched) = mpsc::channel();
    if let Err(error) = start_readers(&mut child, read, events) {
        give_up(child);
        return Err(error);
    }
    let mut output = None;
    let mut diagnostics = None;
    while output.is_none() || diagnostics.is_none() {
        match watched.recv_timeout(stall) {
            Ok(Event::Beat) => {}
            Ok(Event::Output(result)) => {
                if result.is_err() {
                    // It may still be writing to a pipe nobody reads.
                    let _ = child.kill();
                }
                output = Some(result);
            }
            Ok(Event::Diagnostics(text)) => diagnostics = Some(text),
            Err(RecvTimeoutError::Timeout) => {
                give_up(child);
                return Ok(Run::Stalled);
            }
            // Both readers have ended, so nothing more will come.
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }
    let Some(status) = exit_status(&mut child, stall) else {
        give_up(child);
        return Ok(Run::Stalled);
    };
    Ok(Run::Finished {
        output: output.unwrap_or_else(|| Err(io::Error::other("stdout was not captured"))),
        status: status?,
        diagnostics: diagnostics.unwrap_or_default(),
    })
}

fn start_readers<T: Send + 'static>(
    child: &mut Child,
    read: impl FnOnce(Heartbeat<T>) -> io::Result<T> + Send + 'static,
    events: Sender<Event<T>>,
) -> io::Result<()> {
    if let Some(stdout) = child.stdout.take() {
        let events = events.clone();
        spawn(move || {
            let output = read(Heartbeat {
                pipe: stdout,
                events: events.clone(),
            });
            let _ = events.send(Event::Output(output));
        })?;
    }
    if let Some(stderr) = child.stderr.take() {
        spawn(move || {
            let mut bytes = Vec::new();
            // A read error only loses diagnostics; the exit status decides.
            let _ = Heartbeat {
                pipe: stderr,
                events: events.clone(),
            }
            .read_to_end(&mut bytes);
            let text = String::from_utf8_lossy(&bytes).into_owned();
            let _ = events.send(Event::Diagnostics(text));
        })?;
    }
    Ok(())
}

/// Starts a thread, or says why the system refused one: a share that stays
/// hung leaves a few threads behind for each file that stalled.
fn spawn(work: impl FnOnce() + Send + 'static) -> io::Result<()> {
    thread::Builder::new().spawn(work).map(drop)
}

/// The child's exit status, polled until `stall` has passed: with both
/// pipes closed it is exiting, unless it hangs on its input.
fn exit_status(child: &mut Child, stall: Duration) -> Option<io::Result<ExitStatus>> {
    const POLL: Duration = Duration::from_millis(5);
    let deadline = Instant::now() + stall;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Some(Ok(status)),
            Ok(None) if Instant::now() < deadline => thread::sleep(POLL),
            Ok(None) => return None,
            Err(error) => return Some(Err(error)),
        }
    }
}

/// Kills a stalled child and reaps it on a thread nobody waits for: the
/// kill takes effect only when the child's pending read returns.
fn give_up(mut child: Child) {
    // It may have exited already.
    let _ = child.kill();
    // Without a thread the child stays unreaped until Gunfinger exits.
    let _ = spawn(move || {
        let _ = child.wait();
    });
}

/// Calls `work` on a thread of its own and waits at most `stall` for its
/// result: for reads of a file that can block in-process, such as the
/// first bytes `Container::of` reads. `None` if it did not finish in time;
/// the thread then ends whenever the read returns. An error only when the
/// thread cannot be started.
pub fn within<T: Send + 'static>(
    stall: Duration,
    work: impl FnOnce() -> T + Send + 'static,
) -> io::Result<Option<T>> {
    let (sender, result) = mpsc::channel();
    spawn(move || {
        let _ = sender.send(work());
    })?;
    Ok(result.recv_timeout(stall).ok())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    use super::*;
    use crate::decode::{DecodeError, Decoder, Excerpt, read_all};

    /// The stall timeout of runs meant to stall: long enough for what
    /// comes before the silence (the first bytes read on a thread, the
    /// child started) on a heavily loaded machine (a load average
    /// of 250), and short enough to keep the tests quick.
    const STALL: Duration = Duration::from_secs(2);

    /// The stall timeout of runs meant to finish, which only a real stall
    /// reaches: under heavy load a stand-in can take seconds to start.
    const PATIENT: Duration = Duration::from_secs(60);

    /// A folder of its own for each test, so tests in parallel never share
    /// a stand-in program; `PATH` is left alone.
    fn scratch(test: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("gunfinger-watch-{test}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// An executable shell script standing in for FFmpeg or FFprobe.
    fn stand_in(dir: &Path, name: &str, script: &str) -> PathBuf {
        let path = dir.join(name);
        fs::write(&path, format!("#!/bin/sh\n{script}\n")).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        path
    }

    fn decoder(ffmpeg: PathBuf, ffprobe: PathBuf) -> Decoder {
        Decoder {
            ffmpeg,
            ffprobe,
            stall: STALL,
        }
    }

    /// Whether process `pid` still runs, asked for up to 30 s: under heavy
    /// load a killed child can take seconds to exit and be reaped.
    fn still_runs(pid: &str) -> bool {
        for _ in 0..3000 {
            let alive = Command::new("kill")
                .args(["-0", pid])
                .stderr(Stdio::null())
                .status()
                .unwrap()
                .success();
            if !alive {
                return false;
            }
            thread::sleep(Duration::from_millis(10));
        }
        true
    }

    #[test]
    fn a_decode_that_writes_nothing_is_stopped_after_the_stall_timeout() {
        let dir = scratch("silent");
        let audio = dir.join("on-a-hung-share.mp3");
        fs::write(&audio, b"ID3").unwrap();
        let pid = dir.join("pid");
        let hangs = stand_in(
            &dir,
            "ffmpeg",
            &format!("echo $$ > '{}'\nexec sleep 120", pid.display()),
        );
        let started = Instant::now();

        let error = decoder(hangs.clone(), hangs)
            .decode(&audio, 8000, Excerpt::default())
            .unwrap_err();

        // Stopped after the stall timeout, not waited for.
        assert!(started.elapsed() >= STALL, "{:?}", started.elapsed());
        assert!(
            started.elapsed() < Duration::from_secs(60),
            "{:?}",
            started.elapsed()
        );
        assert!(
            matches!(
                error,
                DecodeError::Stalled {
                    from: "`ffmpeg`",
                    ..
                }
            ),
            "{error}"
        );
        assert!(!error.is_about_the_file());
        let message = error.to_string();
        assert!(
            message.contains(
                "on-a-hung-share.mp3 could not be read: nothing came from `ffmpeg` for 2 s"
            ),
            "{message}"
        );
        // The stand-in was killed, not left sleeping. (On a loaded machine
        // it may be killed before it writes its pid.)
        if let Ok(pid) = fs::read_to_string(&pid) {
            assert!(!still_runs(pid.trim()), "process {} still runs", pid.trim());
        }
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_slow_decode_that_keeps_writing_finishes() {
        let dir = scratch("slow");
        let audio = dir.join("slow.mp3");
        fs::write(&audio, b"ID3").unwrap();
        // One sample every 0.1 s, 40 in all: at least 4 s, longer than the
        // stall timeout, and silent for a thirtieth of it at a time, so
        // heavy load can stretch each gap thirtyfold.
        let slow = stand_in(
            &dir,
            "ffmpeg",
            "i=0; while [ $i -lt 40 ]; do printf '\\000\\000\\200\\077'; sleep 0.1; i=$((i + 1)); done",
        );
        let watched = Decoder {
            stall: Duration::from_secs(3),
            ..decoder(slow.clone(), slow)
        };
        let started = Instant::now();

        let audio = watched.decode(&audio, 8000, Excerpt::default()).unwrap();

        assert_eq!(audio.samples, [1.0; 40]);
        assert!(started.elapsed() > watched.stall, "{:?}", started.elapsed());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_child_that_writes_only_diagnostics_is_still_running() {
        let dir = scratch("diagnostics");
        // Silent on stdout for at least 4 s, longer than the stall timeout,
        // while it writes to stderr every 0.1 s.
        let chatty = stand_in(
            &dir,
            "chatty",
            "i=0; while [ $i -lt 40 ]; do echo 'damaged frame' >&2; sleep 0.1; i=$((i + 1)); done\nprintf done",
        );
        let stall = Duration::from_secs(3);
        let started = Instant::now();

        let run = run(&mut Command::new(chatty), stall, read_all).unwrap();

        let Run::Finished {
            output,
            status,
            diagnostics,
        } = run
        else {
            panic!("{run:?}")
        };
        assert_eq!(output.unwrap(), b"done");
        assert!(status.success());
        assert_eq!(diagnostics.lines().count(), 40, "{diagnostics}");
        assert!(started.elapsed() > stall, "{:?}", started.elapsed());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_probe_that_writes_nothing_is_an_error() {
        let dir = scratch("probe");
        let audio = dir.join("tune.m4a");
        fs::write(&audio, b"....ftyp").unwrap();
        let hangs = stand_in(&dir, "ffprobe", "exec sleep 120");
        let fails = stand_in(&dir, "fails", "exit 1");
        let patient = Decoder {
            stall: PATIENT,
            ..decoder(fails.clone(), fails)
        };

        let stalled = decoder(hangs.clone(), hangs).probe(&audio);
        let unreadable = patient.probe(&audio);

        assert!(
            matches!(
                stalled,
                Err(DecodeError::Stalled {
                    from: "`ffprobe`",
                    ..
                })
            ),
            "{stalled:?}"
        );
        assert!(matches!(unreadable, Ok(None)), "{unreadable:?}");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_child_that_closes_its_output_but_does_not_exit_is_stopped() {
        let dir = scratch("lingers");
        let lingers = stand_in(&dir, "lingers", "exec >&- 2>&-\nexec sleep 120");

        let run = run(&mut Command::new(lingers), STALL, read_all).unwrap();

        assert!(matches!(run, Run::Stalled), "{run:?}");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn work_that_does_not_finish_in_time_is_given_up() {
        let slow = within(Duration::from_millis(50), || {
            thread::sleep(Duration::from_secs(2));
        })
        .unwrap();
        let quick = within(PATIENT, || 42).unwrap();

        assert_eq!(slow, None);
        assert_eq!(quick, Some(42));
    }
}
