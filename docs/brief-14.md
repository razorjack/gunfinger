# Session 14 brief: an interactive listening tool

Written by the agent for itself on 2026-10-11, so the plan survives context
compaction. Work autonomously and never stop to ask. When something is
ambiguous, take the conservative option, write the decision down and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-14-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 11" to the end).
4. `docs/status.md` (newest entries at the bottom).
5. `docs/adr/0016-listening-tool.md` and `docs/audition-questions.md`
   once written (the design the code follows).
6. The listening tools of before: `crates/gunfinger-eval/src/pair_review.rs`,
   `clusters.rs` (`Verdicts`), `scripts/analysis/listening_pack.py`,
   `work/listening/README.md`, `work/listening-decisions/make_listen.py`.

`docs/brief-2.md` to `docs/brief-13.md` hold rules that still apply
unless this brief changes them (among them: edit no configuration file;
test fixtures are generated, never copied from the corpus or the NAS;
long runs start detached under `screen`).

## The prompt, verbatim

```text
Start session 14 of Gunfinger: build an interactive listening tool,
a local web page served by gunfinger-eval, in which the user
answers listening questions (are these two files the same recording? is
this folder a mixed CD? is this detection right?) by switching between
files at the same musical moment, seeking with keys and the mouse, and
answering with one key. The session may run up to 6 hours, unattended.
The NAS is mounted at /Volumes/atlas. Files
may be added to the NAS during the session; that is expected. Work autonomously and never stop to ask.
When something is ambiguous, take the conservative option, write the
decision down and continue.

Why: judging is by ear (pairs of library files, folder boundaries,
detections), and today that means clips played one after another through
ffplay (`gunfinger-eval pair-review`, a zsh helper in
work/listening-decisions/listen.zsh). They cannot go back a few seconds,
switch to the other file at the same moment, or loop a bar, so listening
is slow and error-prone. Items 1 and 2 of the listening pack and seven
folder questions have waited on this for two sessions. We asked
for this tool, approved its design (below), and asked for three things
in particular: dark mode, `Z` (not Cmd-Z) to undo, and code that is
elegant and easy to extend, because future sessions will add new kinds
of listening questions. Future agent sessions must be able to work with
it from the documentation alone.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it),
   docs/brief-13.md and docs/session-13-checklist.md (brief-2 to brief-12
   hold rules that still apply unless this prompt changes them),
   docs/session-notes.md (from "Session 11" to the end), docs/status.md
   (newest entries at the bottom), docs/roadmap.md ("Report edits" and
   "A full-screen TUI"), and the current listening tools:
   crates/gunfinger-eval/src/pair_review.rs, crates/gunfinger-eval/src/clusters.rs
   (`Verdicts`), scripts/analysis/listening_pack.py, work/listening/README.md
   and three of its sheet.txt files (one borderline, one control join,
   the cut join of item 1), work/listening/items-s10-kept.json and
   items-s11-items.json, scripts/analysis/folder_edges.py,
   work/listening-decisions/make_listen.py (its QUESTIONS and FOLDERS hold
   the questions and folders waiting to be judged by ear),
   work/listening-decisions/boundaries.py, and the `gunfinger listen` and
   `review` commands of the CLI. Look at the JSON of one `identify -f json`
   report (work/merge-check/2003-s12.json, the 2003 development mix).
2. `git status` should be clean at 633ebdd. Write docs/brief-14.md (this
   prompt verbatim, the state at the start, the plan with times) and
   docs/session-14-checklist.md. Point CLAUDE.md's reading list at them;
   keep brief-13 among the older briefs. Commit.
3. Build with --release and copy the binaries to work/bin/s14-start/. Run
   `gunfinger-eval baseline session-14-start`. Record the NAS peak store's
   state (scripts/analysis/store_revision.py): 33,596 records and revision
   f9f9bc706195a71b expected.

The design (approved; details are yours, but write each
choice down in the ADR):

- Architecture. A gunfinger-eval subcommand (suggested name `audition`;
  choose and document) starts a small HTTP server bound to 127.0.0.1 on
  a free port, prints the URL and opens it in the default browser with
  `open`. Every request carries a random token from that URL and is
  checked against it and the Host header. The page is static files
  embedded in the binary with include_str! (HTML, CSS, plain JavaScript
  as ES modules if useful): no Node, no npm, no bundler, no CDN, no build
  step. Prefer no new crate dependency (a minimal HTTP/1.1 server on
  std::net is enough for one local user) or at most one small,
  well-maintained one; justify the choice in the ADR. It must work in
  current Safari and Chrome on macOS with standard Web Audio only.
- Audio. The server decodes with the ffmpeg executable (as core's decode
  module does: child process, never a shell) to a format every browser
  decodes (for example 16-bit stereo WAV at 44.1 kHz), because Safari does
  not reliably decode Ogg Opus and the collection has opus, m4a and mp3.
  The page plays through Web Audio, which gives gapless switching at a
  sample-exact moment and turntable-style speed matching (playbackRate
  changes pitch and tempo together, as the collection's vinyl sets do).
  Memory stays bounded: mixes are 45-80 minutes, so audio is loaded in
  windows of a few minutes around the playhead and the markers, with
  prefetching; measure what a window costs and write it down. The server
  only decodes, describes or reveals files named in the loaded question
  files; any other path is refused. Waveform envelopes (min/max per
  bucket) are computed on the server and cached under work/ keyed by
  path, size and modification time.
- Data model. A question file (JSON, versioned, one schema in one Rust
  module with serde, documented field by field) holds questions. A
  question has a stable id (derived from its kind and files, so making
  the questions again keeps answers attached), a kind, a title and the
  question text, a timeline measured on a reference lane, lanes (each a
  file, or a sequence of file excerpts, placed on that timeline with an
  offset and a speed, possibly several alignment pieces), markers (where
  the files stop lining up, track boundaries, detection starts), shaded
  spans (where lanes align), and the choices with their keys and labels.
  The page knows nothing about pairs, folders or reports: it draws and
  plays what the question says. Producers turn evidence into question
  files and know nothing about HTTP or the page. Adding a new kind of
  question should need a new producer and, only if its answers must be
  written somewhere new, a new answer effect; no page change unless it
  needs a new feature of lanes, markers or choices.
- Producers needed now:
  1. The listening pack: the 23 items of work/listening/README.md, in its
     order, as pair questions (lane A, lane B aligned through the sheet's
     alignments, markers where they stop lining up and at the clips'
     moments). Item 1 (the Sonar cut join) gets a third lane: the corpus
     original, /Users/razorjack/Projects/OpenSource/gunfinger/corpus/library/DJ Trace - Sonar.m4a.
     The question texts of items 1-13 are in make_listen.py's QUESTIONS;
     items 14-23 are control joins (optional: what the same recording
     sounds like). Items with a verdict in docs/pair-verdicts.txt (by
     pair-review's rules, related pairs included) appear as answered,
     with that verdict. NAS files are named `second-library/<path>` in
     the verdicts file and live under /Volumes/atlas/Music/dnb.
  2. Folder boundaries: the seven folders of make_listen.py's FOLDERS,
     each one question "is this a mixed CD (each track runs into the
     next)?", with one lane that plays every boundary as the CD would:
     the last 20 s of track n straight into the first 20 s of track n+1,
     for every n, a marker at each boundary. Use the folder's audio files
     in name order (Jonny L's interludes 05 and 10 are not indexed but
     are on the CD). Answers: mixed, separate tracks, unsure.
  3. If time allows, after everything else: detections of a report (one
     question per play: the mix as the reference lane, the play's file
     aligned at its fitted offset and speed; confirmed, wrong, unsure;
     answers to the answer log only). Verify it only with the 2003
     development mix's report. If there is no time, document how it
     would be added in the agents' documentation instead.
- Answers. Written on a keypress, immediately, to an
  append-only answer log (JSONL under work/; each line the question id,
  kind, choice, files, time). A pair answer of same or different is also
  appended to docs/pair-verdicts.txt in its exact format
  (`same|different<TAB>path<TAB>path`, NAS files as `second-library/...`,
  the same path forms pair-review writes); unsure writes the log only.
  Folder and report answers go to the log only; agents turn them into
  ignore patterns or manifest notes when asked. `Z` undoes the
  most recent answer, then the one before: it appends an undo record to
  the log, and removes the pair-verdicts line only if it is still the
  file's last line byte for byte; otherwise it leaves the file alone and
  says so on the page. One writer at a time on the server. Both paths
  are options with the real files as defaults.
- The page. Dark theme. A queue on the left (open and answered, the
  answer shown; open the first unanswered at start). For the current
  question: its text; one waveform lane per lane on the reference
  timeline with the shaded aligned spans and the markers; the playhead;
  each lane's full absolute path with a copy button and a reveal-in-
  Finder button (`open -R`, served for listed files only); the alignment
  in words (offset, speed as a percentage). A key legend, and `?` for
  help.
- Keys: Space play or pause; 1, 2, 3 switch to that lane at
  the same moment; Tab back to the lane played before; Left and Right
  seek (Shift for a larger step); Up and Down previous and next marker;
  L loop around the current moment; M speed matching on or off; S, D, U
  same, different, unsure for pair questions (other kinds take their keys
  from their choices; the schema check rejects a choice key that is
  already a control key); J and K next and previous question; Z undo.
  Add a key that plays the current and the previous lane together, one
  left and one right, as the pack's stereo clips did. Mouse: click or
  drag on a waveform to seek, click a marker to jump, click a queue item
  to open it.
- `pair-review` and work/listening-decisions/listen.zsh stay as fallbacks.

Code quality, as requested: write it so that a future session
reading only the docs can add a question kind in an hour. Small modules
with one job each (schema, producers, answers and their effects, server,
audio and envelopes, embedded web files), clear names, module docs on
each, the house style and lints of AGENTS.md, no unwrap or expect in
production paths. The JavaScript is plain, organised and commented where
the reason is not obvious (Web Audio scheduling, windows, timeline
mapping); keep timeline arithmetic in one place, in Rust if that makes
the page simpler.

Documentation for agents:
- An ADR (0016): the tool's purpose, the architecture and why (browser
  and Web Audio over a TUI or ffplay; server-side decoding; no build
  step; security), the question schema's principles, answer effects and
  undo, what was left out.
- A schema reference (a doc under docs/, field by field, with a small
  example question file of each kind, written with generic file names).
- An AGENTS.md section: where the tool lives, its invariants (answers only
  on a keypress, never written by agents; listed files only;
  127.0.0.1 and the token), and step by step how to add a producer or a
  question kind, how to test it, and how to try the page.
- The README: how to start it, in one command, and the keys.
- Mark the roadmap's "Report edits" and "A full-screen TUI" items with what
  this changes.

Tests: the schema and its check (including the key collision), each
producer on small fixtures (fixture sheets and folders of short generated
audio, temporary folders only), stable ids, the answer log, the
pair-verdicts effect and undo (last line or refusal), the path list
(refusing an unlisted path), the token and Host checks, the HTTP request
handling. Use temporary files for every answer and verdict file.

Checking the page: use T3's browser preview tools (preview_open,
preview_snapshot, preview_press, preview_evaluate, device_screenshot) to
load the page, press the keys, and read the player's state (current lane,
position, playbackRate, loop, the queue) through preview_evaluate; you
cannot hear the audio, so check that switching lanes keeps the musical
moment by the computed positions, and that seeking, markers, the loop and
speed matching change what they should. Check the real pack items 1, 2
and one control join, and two of the seven folders, from the NAS. Run
these checks with the answer log and the verdicts file pointed at
temporary files. Put two or three screenshots in work/s14/ and name them
in the notes.

Rules for this session:
- Agents never write verdicts or listening answers. Nothing in this
  session may write to the real docs/pair-verdicts.txt or the real answer
  log; check `git diff docs/pair-verdicts.txt` is empty before every
  commit. Manifests do not change.
- The NAS (/Volumes/atlas/Music/dnb) is read, never written. The NAS peak
  store is read-only: no `index`, `prune` or `--retry-skipped` against
  it; check its revision at the end. Never touch the held-out test set
  (ed-rush-optical-essential-mix), never identify, decode or index
  EM_1999_02_07_-_Ed_Rush_Optical.mp3 at the NAS root, and run nothing on
  the candidate mixes of experiment 0065.
- The matcher, the confidence rule and detections do not change:
  `regress session-14-start` must be identical at the end;
  scripts/check.sh green.
- Out of scope: everything we have not decided (the containment
  rule, shared material, the possible tier, edits, clusters, test
  evaluation 5), and any change to pair-review beyond what sharing code
  needs.
- Commit after each step; check `git status` before every commit and
  never use `git add -A`. Do not push.

Steps:
1. Setup (20 min): the steps above.
2. Design (30 min): the ADR and the schema reference first, so the code
   follows a written design. Commit.
3. Schema, answers and producers with tests (about 1.5 h). Commit.
4. Server, audio windows, envelopes and the page (about 2 h). Commit.
5. Checks on the page with real items from the NAS (about 40 min).
6. The report producer if time allows (about 45 min).
7. A read-only review of the whole change by a subagent (correctness of
   the timeline mapping and undo, security of the server, clarity for the
   next agent); fix what it finds.
8. Wrap-up (30 min): the AGENTS.md section, the README, the roadmap, a
   "Session 14" section at the end of docs/session-notes.md (how the
   tool is started, in one command line, and what to listen to
   first: pack items 1 and 2, then the seven folders), docs/status.md,
   the checklist; `regress session-14-start`; the NAS store's revision.
   Commit. Last, print what was done, the command to start
   the tool, the checks, and anything left.
```

## State at the start of session 14

- 23:40. `git status` clean at `633ebdd` on `master`. Experiments
  0001-0089; ADRs 0001-0015. Test set: 4 of 5 evaluations used (none in
  this session).
- `/Volumes/atlas/Music/dnb` mounted
  (935 entries at the
  root).
- The saved NAS index of `identify`:
  `~/.cache/gunfinger/indexes/nas-dnb-peaks-23935cfc56936b67.index`
  (untouched by this session's plan).
- The listening tools today:
  - `gunfinger-eval pair-review` reads `work/listening/README.md` (the
    order) and each item's `sheet.txt`, plays its 8 s clips with `ffplay`
    and appends `same` or `different` with the sheet's two paths to
    `docs/pair-verdicts.txt` (`pair_review::append`). An item counts as
    judged when a verdict names its pair or one of the sheet's "Other
    pairs between the same two clusters" (`Verdicts::judges`).
  - `work/listening-decisions/listen.zsh` (written by `make_listen.py`) plays
    each item's clip moments and two boundaries per folder with `ffplay`.
  - The sheets give alignments rounded to 0.1 s and 4 decimals of speed;
    the exact alignments are in the `pair` reports
    `work/reports/library-nas-dnb-peaks/pairs-s10-pairs.json` (21 items of
    session 10) and `pairs-s11-items-pairs.json` (session 11's 2 items),
    under the search "peaks, at the fitted speed".
  - A `pair` report of the corpus "DJ Trace - Sonar.m4a" against the Mark
    System Revision exists (`pairs-supported-pairs.json`); none against
    the PRO012 rip.
  - The verdicts file holds 11 verdicts, none on the 23 items' pairs
    themselves (item 1 is the cut join between two files of clusters already
    judged by ear).
  - `identify -f json` reports (schema 3) list `plays`, each with
    `asset` (relative to `library`), `start_seconds`, `end_seconds`,
    `track_start_seconds`, `speed`, `playback` and `segments` (the same
    fields per segment).

## Plan, with times

Start 23:40; the session may run until 05:40.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1 | setup: brief, checklist, build, baseline, store revision | 0:20 | 00:00 |
| 2 | design: ADR 0016, schema reference `docs/audition-questions.md` | 0:30 | 00:30 |
| 3 | schema, placement arithmetic, answers and effects, the pack and folder producers; tests | 1:30 | 02:00 |
| 4 | HTTP server, audio windows, envelopes, the page | 2:00 | 04:00 |
| 5 | checks in the preview browser: pack items 1, 2, one control join, two folders | 0:40 | 04:40 |
| 6 | the detections producer, if time allows | 0:30 | 05:10 |
| 7 | read-only review by a subagent; fixes | 0:15 | 05:25 |
| 8 | wrap-up: AGENTS.md, README, roadmap, notes, status, regress, store revision | 0:15 | 05:40 |

Step 6 is the first to give way: if step 5 ends after 04:40, the
detections producer is documented instead of built.

## Conservative decisions taken up front

- The subcommands: `gunfinger-eval audition` serves the page;
  `gunfinger-eval questions <pack|folders|detections>` writes question
  files. Producers and the server share only the schema module.
- No new crate: the HTTP server is a minimal HTTP/1.1 on `std::net`, one
  thread per connection (the browser opens at most six), `Connection:
  close` on every response. The token comes from `/dev/urandom`.
- The pack producer reads the sheets (what `pair-review` reads) and takes
  the exact numbers of an alignment from a `pair` report when the report's
  alignment rounds to the sheet's; otherwise the sheet's rounded numbers.
  Item 1's third lane is placed by a `pair` report against lane A, run
  read-only for the corpus original against the two NAS files.
- A pair answer that contradicts a verdict already in the verdicts file is
  refused (the file would no longer parse); an answer that repeats one is
  logged without appending a second line.
- Every page check runs with `--answers` and `--verdicts` on temporary
  files; the real answer log is never created by this session.
