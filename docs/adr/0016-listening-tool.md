# 0016: The listening tool (`gunfinger-eval audition`)

## Status

Accepted (2026-10-11, session 14). We approved the design in
`docs/brief-14.md`; the details below are the agent's, chosen in session
14. `gunfinger-eval pair-review` and `work/listening-decisions/listen.zsh`
stay as fallbacks. Since ADR 0017 (2026-10-13) the tool is a development
tool for evaluation ground truth, and the report edits named under "Left
out" are out of scope for the product. Amended by ADR 0020 (2026-10-14,
macOS and Linux): on Linux `xdg-open` opens the page, the reveal button
opens the file's folder (the Finder, with `open -R`, still selects the
file), the page is meant for Chrome or Chromium there (not yet tried),
and the file locks are `flock` there too.

## Context

Open questions are settled by ear: whether two library files are
one recording, whether a release folder is a DJ-mixed CD, whether a
detection is right. Until now that meant 8 s clips played one after
another through `ffplay` (`pair-review`, `listen.zsh`). These tools could
not go back a few seconds, switch to the other file at the same moment,
or loop a bar, so listening was slow and error-prone, and items 1 and 2
of the listening pack and seven folder questions waited two sessions.
Future sessions will add new kinds of question, so the tool must be easy
to extend from its documentation alone.

## Decision

### A local web page, played through Web Audio

`gunfinger-eval audition` starts an HTTP server on `127.0.0.1` at a free
port, prints the page's URL and opens it in the default browser (`open`).
The page plays audio through Web Audio:

- **Gapless switching at a sample-exact moment.** Every source is
  scheduled on the audio context's clock. A lane switch stops the old
  lane's sources and starts the new lane's at the same context time, at
  the file position that maps to the same moment of the timeline.
- **Turntable speed matching.** An `AudioBufferSourceNode`'s
  `playbackRate` changes pitch and tempo together, as a turntable does
  and as the collection's vinyl sets were played. A lane placed at speed
  `s` plays at rate `s` while matching is on, so it stays on the
  reference lane's clock; with matching off it plays at rate 1 and the
  playhead follows its own clock.
- **Both lanes at once.** A stereo blend plays two lanes together, the
  lower-numbered left and the other right, as the pack's stereo clips
  did.

A terminal UI was the roadmap's alternative ("A full-screen TUI"). It
would need a terminal UI crate and an audio output crate (or `ffplay`
processes, which cannot switch at a sample-exact moment or change speed
without restarting), and it could not draw waveforms. Safari and Chrome
already hold a sample-accurate audio scheduler and a canvas.

### Server-side decoding

The browser never opens the audio files. The server decodes a window
of a file with the `ffmpeg` executable (a child process with explicit
arguments, never a shell, as `gunfinger_core::decode` runs it) to 16-bit
stereo PCM at 44.1 kHz and sends it as a WAV file, which every browser
decodes. Safari does not reliably decode Ogg Opus, and the collection
holds opus, m4a and mp3.

- **Windows.** A window is 30 s of one file, numbered from the file's
  start (window `k` holds file time `30k` to `30k + 30`). The page asks
  for the windows it will need soon: those of the sounding lanes from
  10 s behind the playhead to 40 s ahead, those of the other lanes at
  the playhead and 12 s ahead (so a switch finds them), and those at the
  markers before and after the playhead and at the loop's start. It keeps
  at most 24 windows (about 250 MB of decoded audio) and drops the least
  recently used window it does not need now. A 70-minute mix is never
  held whole.
- **Pre-roll.** FFmpeg's input seek lands on the exact sample, but MP3
  and Opus decoders need a few frames to settle: decoded from its own
  start, a window's first 25 ms (MP3) or 195 ms (Opus) differ from the
  same samples decoded continuously, which would click at every window
  joint. The server starts each window 0.5 s early and drops those
  frames; the joints are then bit-exact for MP3 and Opus and the first
  0.86 s match for AAC (measured in session 14 on one file of each).
- **Envelopes.** The waveform of a file is its minimum, maximum and RMS
  level per 50 ms bucket, computed by the server from a mono decode at
  4 kHz (the shape of the music, not its treble) and cached under
  `work/audition/envelopes/`, one JSON file per file keyed by an FNV-1a
  digest of its path, size and modification time. The page draws the
  extremes dimmed and the RMS level bright: the extremes of a loud master
  fill the lane, and the RMS level still shows its breaks and drops.

### No build step, no new crate

The page is plain HTML, CSS and JavaScript ES modules, embedded in the
binary with `include_str!` and served from memory: no Node, npm, bundler
or CDN. The HTTP server is a minimal HTTP/1.1 implementation on
`std::net` (one local user, a handful of routes, `Connection: close`
on every response, one thread per connection in a `thread::scope`; a
browser opens at most six). A crate such as `tiny_http` would add a
dependency tree for about 150 lines of request parsing that the tests
cover. The token's randomness comes from `/dev/urandom`.

### Security

- The server binds `127.0.0.1` only.
- Every request's path starts with a 128-bit random token
  (`/<token>/...`), printed in the URL and never stored. A page on
  another origin cannot guess it, so it cannot read or answer.
- The `Host` header must be `127.0.0.1:<port>` or `localhost:<port>`,
  which defeats DNS rebinding.
- The server decodes, describes or reveals (`open -R`) only files named
  in the loaded question files (`Listed`). A request naming any other
  path is refused, whatever its spelling; paths are compared exactly,
  never resolved.
- Responses carry a content security policy that allows only the page's
  own scripts and connections, `no-store` caching and `no-referrer`.

### Questions: data the page draws and plays

A question file (JSON, `schema_version` 1, one Rust module:
`audition::schema`) holds questions; `docs/audition-questions.md`
documents every field. The principles:

- **The page knows no kinds.** It draws and plays what a question says:
  lanes of pieces on a timeline, markers, shaded spans, choices. Pairs,
  folders and detections exist only in their producers.
- **One placement rule.** A piece places a span of one file on the
  timeline: file time = `offset + speed × t` for timeline time `t` in
  `[start, end)`. A lane is a sequence of pieces: one piece for a whole
  file, several for the alignments of an edit, one per excerpt for a
  folder's boundaries. The producers compute pieces in Rust
  (`audition::placement`); the page only evaluates that formula, in one
  module (`timeline.js`). A lane follows only alignments with the hits of
  a possible play (60, `FITTED_RULE.min_possible_hits`); weaker ones are
  chance alignments or repeated sections (added 2026-10-12, after a chance
  alignment of 8 hits started a pack item's lane B 3:50 into its file).
- **Stable ids.** A question's id is its kind and a 64-bit FNV-1a digest
  of its kind and its files (with a distinguishing part where one kind
  can ask twice about the same files), so making the questions again
  keeps the answers attached.
- **Files by their harness names.** A question's `files` are the files
  its answer is about, named as `docs/pair-verdicts.txt` names them
  (corpus library paths, `second-library/<path>` for the NAS library,
  absolute paths otherwise). Lanes hold absolute paths, which the server
  lists.
- **Checked on load.** The server and every producer check a question
  file before using it: ids unique, pieces inside the timeline and in
  order, speeds between 0.5 and 2, markers and spans inside the
  timeline, choice keys single letters, unique, and not a control key of
  the page.
- **Producers** (`gunfinger-eval questions pack|folders|detections|shared`)
  turn evidence into question files and know nothing of HTTP or the
  page. Adding a kind of question needs a producer, and an answer effect
  only if its answers must be written somewhere new.

### Answers, effects and undo

- **Only on the listener's keypress.** An answer is written when the listener
  presses a choice's key, immediately. Agents never write answers or
  verdicts. After an answer the page opens the next question without an
  answer; `Z` reopens the question it takes back. A held key does not
  repeat answers or undos, and a key pressed while an answer is being
  written is ignored, so neither lands on the question opened next.
- **The answer log** (`work/audition/answers.jsonl`, `--answers`) is
  append-only JSON lines: each record has a sequence number, the time,
  the action (`answer` or `undo`), the question's id, kind, title and
  files, the choice, and what its effect did. The state of a question is
  replayed from the log: its latest answer that no undo took back.
- **Effects.** A question names its effect. `log` writes the log only
  (folders, detections). `pair_verdict` also appends `same` or
  `different` with the question's two files to the verdicts file
  (`docs/pair-verdicts.txt`, `--verdicts`) in its exact format, as
  `pair-review` writes it; `unsure` writes the log only. A pair whose
  verdicts file already judges it (its own pair or a related pair, by
  `pair-review`'s rule) shows as answered with that verdict. Answering it
  again with the same verdict logs without a second line; another
  answer is refused: a contradicting verdict because the file would no
  longer parse or `clusters` would stop on the chain, and `unsure`
  because the file's verdict would still be what the page shows.
- **The order of writes.** The log is opened first, so a log that cannot
  be written stops an answer before the effect. Then the effect writes,
  then the log record. If the record still fails, the verdict line
  stays without a record; the error says so, and `Z` cannot take it
  back.
- **Undo (`Z`)** takes back the most recent answer still in force, then
  the one before, if its question is loaded: the page opens the
  question, and the message names it. An answer to a question not loaded
  is left for a session that loads it. Undo appends an undo record; if
  the answer appended a verdict line, it removes those bytes only if
  they are still the verdicts file's whole last line, byte for byte.
  Otherwise the file is left alone and the page says so.
- **One writer.** Answers and undos run under one lock in the server and
  read the log and the verdicts file again each time, so a hand edit in
  between is seen. Since 2026-10-13 (session 16, the workspace on Rust
  1.89) files are locked too, with the standard library's advisory file
  lock (`flock` on macOS):
  - A server holds the answer log's lock while it runs
    (`AnswerBook::lock_log`). A second server on the same log says that
    another server is answering into it and exits before it listens; it
    does not wait, because the server already running is the one to
    answer in, and a waiting server would look like one that hangs.
  - A verdict is checked against the verdicts file and appended under
    that file's lock (`clusters::VerdictsWriter`), and undo compares and
    shortens the file under it. `pair-review` reads the file again under
    it before appending and writes nothing for an item judged meanwhile.
    These writers wait for each other, for the milliseconds a write
    takes.

## Consequences

- The listener answers with one key per question and can hear any moment of
  either file, switch at the same musical moment, loop, blend and match
  speed. `pair-review` and `listen.zsh` remain for a terminal.
- A new kind of question is a producer (Rust, tested on fixtures) plus a
  `questions` subcommand; a new place to write answers is a new variant
  of `Effect`. AGENTS.md lists the steps.
- The page needs a current Safari or Chrome. Web Audio cannot change
  tempo without pitch: a key-locked detection is matched in tempo, its
  pitch shifted with it, and the question says so.
- Each window costs the server one FFmpeg run and the page a decoded
  buffer; the measured costs are in `docs/audition-questions.md`.

## Left out

- Report edits (confirming, rejecting or renaming plays and
  keeping them across identifications; roadmap "Report edits"): detection
  answers go to the log only, and turning them into edits is for later.
- Zoom on the waveform, nudging a lane's offset by ear, tempo change
  without pitch change, playing more than two lanes at once.
- Several users, remote access, HTTPS, persistence of the page's state
  (lane, position) across reloads.
