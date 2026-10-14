# 0020: Gunfinger runs on macOS and Linux

## Status

Accepted (2026-10-14, our requirement). Extends the environment
of the brief ("macOS with Homebrew", `docs/brief.md` §2) to
Linux. Amends ADR 0016 where it names `open`, `open -R`, the Finder,
`flock` on macOS and the browsers. Detections, reports, the peak store's
format, the saved index and the rule that decides whether a peak record
is current are unchanged.

## Context

From the prompt, 2026-10-14:

> Please fix all macos-specific code. Also add a rule that the app is
> cross platform, and should support at least mac and linux.

We will also run Gunfinger on an x86-64 Linux machine:
first with a copy of the NAS peak store and store-only
searches, later possibly with the NAS mounted there. Until now all work
ran on one Mac. An audit of every tracked file found these parts that
only macOS has:

- the listening page opened itself with `open` and showed a file with
  `open -R` (the Finder), and its button said "Show the file in the
  Finder";
- `gunfinger-eval robust` made its speech with `say`;
- four messages told the user to `brew install ffmpeg`;
- `memory`'s documentation measured the kernel's peak with
  `/usr/bin/time -l`, whose option and units are macOS's.

Everything else already uses what both systems have: FFmpeg, FFprobe
and FFplay as child processes, `ps -o rss=`, `/dev/urandom`, the
standard library's file lock (`flock` on both), XDG folders for the
configuration and the saved index, and `/bin/sh` in tests. Two harness
defaults name the Mac's mount of the NAS, `/Volumes/atlas/Music/dnb`
(`gunfinger-eval --second-library-root` and `listening_pack.py --nas`);
they are options, and on Linux the mount's path is given instead.

## Decision

### Supported platforms

Gunfinger, its harness and the listening tool support at least macOS
and Linux. Other Unix systems take Linux's branch; Windows is not
supported.

### One function per need

Where code runs a program only one platform has, one function per need
chooses it and returns the command, with a branch for each platform.
In the harness, `platform::Platform` names the platform (`CURRENT`, from
`cfg!(target_os = "macos")`), and the functions take it as an argument:
`audition::desktop::browser`, `audition::desktop::reveal` and
`robust::speech_synthesis`. So every branch compiles and is tested on
either system; a test per platform under `#[cfg(target_os)]` checks
`CURRENT`.

- **Opening the page:** `open <url>` on macOS, `xdg-open <url>` on Linux,
  run on a thread of its own: outside a desktop environment `xdg-open`
  may run the browser itself and wait until it closes, and the server
  must not wait with it. A failure prints a warning naming the command;
  the URL is printed anyway, and `--no-open` skips the step.
- **Showing a file:** the Finder still opens the file's folder with the
  file selected (`open -R <file>`). On Linux, `xdg-open <folder>` opens
  the folder in the default file manager. Selecting the file needs the
  D-Bus call `org.freedesktop.FileManager1.ShowItems`, which only some
  file managers answer. It is left out: trying it first would need a
  second program (such as `gdbus`) and file URIs, more than a
  development tool needs. The button's tooltip reads "Show the file in
  its folder". Errors name the
  command run, such as "`xdg-open /music/label` failed (exit status:
  4)" or "cannot run `xdg-open`: ...". Outside a desktop environment
  `xdg-open` may wait for the file manager to close; that request waits
  with it, and the rest of the page keeps working.
- **Browsers:** on Linux, a current Chrome or Chromium. ADR 0016 tried
  Safari and Chrome; Firefox has not been tried.
- **Speech for `robust`:** `say -o speech.aiff` on macOS, `espeak-ng -w
  speech.wav` (eSpeak NG) on Linux. The voices differ, so results of the
  speech conditions compare only between runs on one platform.
- **Install hints** name a command for each platform:
  `gunfinger_core::decode::INSTALL_FFMPEG` (`brew install ffmpeg` on
  macOS, `sudo apt install ffmpeg` on Debian or Ubuntu) in the core's,
  the CLI's and the listening tool's messages.
- **Peak memory:** `/usr/bin/time -l` on macOS ("maximum resident set
  size", in bytes), `/usr/bin/time -v` on Linux (GNU time, "Maximum
  resident set size (kbytes)"). The harness's own `ps -o rss=` reports
  kilobytes on both.

### Checking Linux's branches from macOS

`scripts/check.sh` also runs clippy for the other platform's target
(`x86_64-unknown-linux-gnu` on macOS, `aarch64-apple-darwin` on Linux)
when that target's standard library is installed, and otherwise prints
one line saying it skipped. Clippy type-checks without linking, so no
Linux linker or C library is needed. A type error planted in the
`#[cfg(target_os = "linux")]` test failed this pass and passed the host's
(2026-10-14).

The development Mac has Homebrew's Rust, which ships only macOS's standard
library and no `rustup`. Homebrew's `rustup` (keg-only, so Homebrew's
`cargo` stays first on `PATH`) holds a 1.98.0 toolchain, the version of
Homebrew's, with the Linux target; AGENTS.md has the commands. From
macOS this is a type check only. The tests of both branches' commands
run on either system, but running `xdg-open` and `espeak-ng`, the test
of `CURRENT` on Linux and the rest of the test suite on Linux happen
only on the Linux machine, in its first `scripts/check.sh`.

### Open risks, for the Linux machine to check

Asset identity and the rule that decides whether a record is current do
not change here. Two parts of them may behave differently when Linux
reaches the same NAS; only that machine can tell.
`scripts/analysis/store_identity.py <store> <library root>` makes both
checks, read-only: it counts the store's paths with two Unicode forms
and its times with fractions of a second, then looks those files and a
sample of the others up in the library. On the Mac, 2026-10-14, it found
all 169 it looked up under the stored names, with size and time equal to
the nanosecond.

1. **Unicode form of file names.** The store keys a record by the path
   the operating system lists. Through macOS's SMB client, every one of
   the 149 stored paths (of 38,105 records, store revision
   `b285da9984422fb5`) whose characters have a decomposed form is stored
   decomposed (NFD; for example "Łódź" as "Ło" + U+0301 + "dz" + U+0301),
   none precomposed. If Linux's SMB client lists those files precomposed
   (NFC), their records are not found: `index` would index them again
   under new names, the old records would look like files gone from the
   library, and verdicts or manifest references naming them would not
   match. Patterns of `.gunfingerignore` with such characters would be
   affected the same way. Store-only runs are not affected. The check,
   with the NAS mounted on Linux: list such files and compare their
   bytes with the records' stored paths. A fix would identify files by a
   normalized path; it needs its own ADR and a way to find the existing
   records under their stored names.
2. **Modification-time precision.** 32,000 of the 38,105 records hold
   modification times with fractions of a second, as macOS's SMB client
   reported them. If Linux's client reports the same files at another
   precision, those records look out of date and `index` would decode
   up to those 32,000 files (84% of the records) again. The check: `stat` a few files on the
   Linux mount and compare with their records, before any `index` from
   Linux.

A third difference is expected rather than a risk. Search is
deterministic on one machine, but between the Mac (Apple silicon) and
the Linux machine (x86_64) FFmpeg's build, the FFT library's vector code
(NEON against AVX or SSE) and the C maths library differ. A mix's peaks
may therefore differ in rare frames, and hit counts by a few, so a
`regress` baseline is compared only on the machine that made it. The
check: the same `identify --store-only -f json` of a development mix on
both machines, compared play by play.

## Consequences

- The user can build and run `gunfinger` and `gunfinger-eval`, and
  answer listening questions, on Linux. Nothing has run on Linux yet:
  the first `scripts/check.sh` there is the first test. Tests that write
  a stand-in script and run it at once (`decode::watch`'s and
  `audition::audio`'s tests, `tests/stalls.rs`) may then fail now and
  then with "Text file busy" (`ETXTBSY`): on Linux a child forked by
  another test thread can hold the script open for writing until it
  runs its own program (rust-lang/rust#114554). macOS has no such
  error. A failure of that kind is a test race, not a platform defect;
  running the tests again tells them apart.
- Store-only searches with a copy of the NAS peak store work on Linux
  with `--store-only`, with no library configured, or with the
  configuration keeping the library path the store names
  (`/Volumes/atlas/Music/dnb`, in `library.txt`) even where that path
  does not exist (ADR 0012). Without `--store-only`, a configuration
  naming another path is refused, as the store names its library. A
  copied store at another path gets its own saved index, built once.
- With the NAS mounted on Linux, the library's path must be the one the
  store names: mount the share at `/Volumes/atlas` (a mount point, not a
  symbolic link: the library's path is compared after links are
  resolved), or put the Linux path in the copy's `library.txt`.
- Before any `index` from Linux against the NAS store, the two checks
  above. If either fails, index from the Mac only until a decision
  fixes it.
- Not measured on Linux: speed, and memory with the default `jobs`. The
  Mac's figures are at 10 workers; the Linux machine defaults to 32, and each
  worker keeps its own hit buffers (`search::lines`), so memory grows
  with workers (`--jobs` or `GUNFINGER_JOBS` limits them).
- A change to platform-specific code is checked by the other platform's
  clippy pass and tested on both branches; AGENTS.md carries the rule.
