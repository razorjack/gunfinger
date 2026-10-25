# Session 47 brief: lane alignment that can be turned off

Written by the agent for itself on 2026-10-25, so the plan survives
context compaction. Work autonomously and never stop to ask: where a
decision is needed, take the conservative option, write it down here and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`
   ("The listening tool" above all).
2. This file, and `docs/session-47-checklist.md` (progress).
3. `docs/adr/0016-listening-tool.md`, `docs/audition-questions.md`,
   `docs/status.md` ("after session 46"),
   `work/audition/listening-comments-2026-10-25.md`.
4. The code of `crates/gunfinger-eval/src/audition/` (`placement.rs`,
   `pack.rs`, `shared.rs`, `schema.rs`, `session.rs`) and every file in
   `audition/web/` with its tests; `scripts/analysis/listening_pack.py`
   (`render`).

`docs/brief-2.md` to `docs/brief-46.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 47 of Gunfinger: the listening page's lane alignment can be turned off. The user's request after the listening of 2026-10-25 (docs/status.md, "after session 46"; verbatim in `work/audition/listening-comments-2026-10-25.md`): "There should be ability to remove track offsetting. It can be on by default but sometimes the tool tried to be helpful and offsets *identical* tracks with -90s offset and produces false arrangement mismatches. for these cases, it's difficult to say if the tracks are the same (even if they are the same) because the tool shifts one track 90s and lane1 plays without hihat, lane 2, same track (just different rip, different riaa amplifier, different turntable etc) but shifted 90s, plays with hihat". By the question format (`docs/audition-questions.md`), a pair question's lane B is a sequence of pieces, one per `pair` alignment, each with its own `offset` and `speed`; where the main alignment ends, a piece of another alignment (a section repeated elsewhere in the track, tens of seconds away) takes over, so two rips of one recording can play different parts of the arrangement. This session adds a way to hear the lanes without that, keeps today's behaviour the default, and fixes a related pack problem. The listening page is a development tool (ADR 0016): it changes; the product does not.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run. A listening server the user started may still be running (`gunfinger-eval audition` on 127.0.0.1): leave it alone; start your own on another port with a scratch answer log and scratch verdicts file when you need one.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600.

Time box: started about 03:00; aim to finish within about 1.5 hours, because other sessions follow you tonight. If a step would run past that, scale it down, record the decision in the brief and continue.

Environment (sessions 33-46's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records), library `/mnt/atlas/Music/dnb` (read only; on Ethernet again tonight), reached with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: after the listening of 2026-10-25 (docs/status.md, "after session 46") SHA-256 `fdf7428a6322...`, the same 284 clusters as before. Baselines `session-32-linux` and `session-33-linux-nas`. Build `work/bin/s46-final/` (master). Since sessions 41, 43 and 46 a NAS-scale development scan searches in about 24 s and `regress session-33-linux-nas` takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans, sweep reports of seeds other than 2026 (calibrate reads every sweep report there), and reports of opt-in variants must not sit in a reports directory during `regress`; run variants in a scratch work directory.
- Session 38's opt-in `--apart-in-speed` (confirmed by session 45; adoption ours, with an ADR) and session 37's `--overlap-by-chains` are variants, not defaults.
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen tonight. Pending listening (items 4, 5, 7 and 9 of session 40's pack, session 39's questions, Star Trails, Into Music) stays for later.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule and "The listening tool"), docs/brief.md (never edit it), docs/brief-46.md, docs/session-notes.md (the sessions that built and changed the listening page: search for "audition" and "listening page"; and "Session 40"), docs/status.md (newest entries, including "after session 46"), ADR 0016, `docs/audition-questions.md`, every file in `crates/gunfinger-eval/src/audition/web/` (and its tests), `crates/gunfinger-eval/src/audition/` (`pack.rs`, `placement.rs`, `schema.rs`, `server.rs`) and `scripts/analysis/listening_pack.py`.
2. Write docs/brief-47.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-47-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of `work/audition/answers.jsonl` in work/s47/listening-digests-start.txt; both must be the same at the end. Experiments, if any, number from 0215.
3. Build `work/bin/s47-start`; `regress session-32-linux` and `regress session-33-linux-nas` identical.

Steps:

1. Find out why two rips of one recording are placed 90 s apart: from session 40's pack (`work/listening-s40/`, question file `work/audition/questions/05-pack-s40.json`), list the pair questions whose lane B uses a piece at an offset far (say more than 10 s) from the main alignment's, and say for which of them the verdict was `same`. Write it in the brief.
2. Design, written in the brief before code: a key (and a visible control) that switches lane B's placement between at least today's pieces (the default) and one offset and speed throughout, those of the main alignment (the one with the most hits), so identical rips play the same moment end to end; consider also a third state, no offset (each file from its own start at the matched speed), if it is cheap and clear. The playhead, seeking, the windows fetched, speed matching (`M`) and the lane labels must stay right in every state; the state shows on the page and in the help; it applies per question or for the session (decide and say which); an answer's record does not change. Keep the question file format as it is unless the main alignment cannot be told from it, in which case add an optional field, documented in `docs/audition-questions.md`, that `questions pack` writes and old files without it still load.
3. Build it with tests (the page's Node tests and the Rust tests; `scripts/check.sh` green), and check it by hand in your own server on a scratch copy: a pair from session 40's pack whose lane B has a far piece, in each state, with the API responses and the computed file times showing the lanes in step.
4. The pack problem: `listening_pack.py render` writes sheet paths relative to `--corpus` when it is given a relative path, and `questions pack` then refuses them ("is not an absolute path"; it happened on 2026-10-22 and 2026-10-25). Make render resolve `--corpus` and `--nas` to absolute paths (or `questions pack` resolve them against the repository root, whichever is safer), with a test.
5. Update ADR 0016's consequences or `docs/audition-questions.md` where the behaviour is described, and the page's help text.
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 47" section at the end of docs/session-notes.md with a handoff (the key, what each state does, how to rebuild the pending question file if needed); update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-32-linux` and `regress session-33-linux-nas` identical with the final build, docs/pair-verdicts.txt and the user's answer log unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: what was built, the key and states, step 1's finding, and the exact command for the user's next listening session with the pending items (4, 5, 7 and 9 of session 40's pack) using the final build.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's behaviour does not change: no threshold, parameter, rule, default or output change in the core or the CLI. The listening page and the harness change, tested, and keep `regress` identical.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt and `work/audition/answers.jsonl` do not change. To test the page, use your own server with `--answers` and `--verdicts` pointing at scratch copies under work/s47/, and send test answers only there. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
- Scratch data goes under work/s47/.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:59. Start no new run after about 11:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 47

- 2026-10-25, 02:00 by the Linux machine's clock (CET; the prompt's
  "about 03:00" is another clock). The time box is read as 1.5 hours
  from 02:00: finish by about 03:30.
- `master` at `dc4f7dc`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s47/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `d3bb1fe061f7...` (97 lines),
  `work/audition/answers.jsonl` `78c3fb0e7ce0...`, the NAS clusters
  file `fdf7428a6322...`.
- Experiments 0001-0214; this session needs none (a page change, no
  measurement), so none is written unless a step measures something.
- The start build `work/bin/s47-start/` is `target/release` of
  `dc4f7dc` (cargo had nothing to rebuild). It differs in bytes from
  `s46-final`, which was built before the review commit `dd7a5f3`
  changed comments and test code; the regressions decide.
- `work/audition/questions/` holds `01-pack.json` and `05-pack-s40.json`
  (made with `gunfinger-eval questions pack --pack work/listening-s40
  --pair-report work/s40/pack/pairs-report.json --second-library-root
  /mnt/atlas/Music/dnb --out work/audition/questions/05-pack-s40.json`).
  No producer's choice uses the letter `o`; no loaded question file
  does either (keys in use: `s d u`, and in the producers also `c y n`).

## Plan

| Step | What | Expected |
|---|--|---|
| First | reading; digests; brief; start build and both regressions (tmux `s47start`) | 02:00-02:15 |
| 1 | the far pieces of session 40's pack | 02:15 |
| 2 | design in this brief, committed | 02:15-02:25 |
| 3 | schema, producers, page, tests; a check in a scratch server | 02:25-03:05 |
| 4 | `listening_pack.py render` paths, with a test | 03:05-03:10 |
| 5 | ADR 0016, `docs/audition-questions.md`, help text | with step 3 |
| 6-7 | review, notes, status, final checks | 03:10-03:35 |

## Decisions

- **(02:02) Start build and regressions** (`work/s47/start.sh`, tmux
  `s47start`, logs in `work/s47/logs/`; `run.sh` and `same.py` are
  session 46's with the paths changed).
- **(02:10) Step 1: the far pieces of session 40's pack.** In
  `05-pack-s40.json` lane B has a piece whose offset lies more than
  10 s from the main alignment's (the one with the most hits) in 5 of
  the 14 questions:

  | Item | Main alignment | Far pieces (A's time) | Answer |
  |---|---|---|---|
  | 1. Dead Air ~ the RH 66 upload | 1: +0.4 s, 3,298 hits | alignment 2 at -235.2 s (361 hits), A 5:20-6:27 | `same` |
  | 3. Alien Girl ~ the PRO014 rip | 1: -0.2 s, 1,495 hits | alignments 6, 7, 4, 5 at -175.5, -198.1, -220.5, -340.7 s (155-233 hits), A 4:50-7:02 | `same` |
  | 5. Fallout ~ the Sinthetix file | 1: -0.6 s, 987 hits | alignment 3 at -11.9 s (107 hits), A 2:10-2:30; alignment 4 at -300.3 s (99 hits), A 6:30-6:52 | none yet |
  | 10. Tower Bass, two mixes | 1: -81.8 s, 118 hits | alignment 2 at -59.6 s (75 hits), A 0:59-1:30 | `different` |
  | 11. We R The Ones VIP ~ the Klute Remix | 1: +0.4 s, 125 hits | alignment 3 at -132.4 s (64 hits), A 5:10-5:40; alignment 2 at -21.9 s (77 hits), A 6:10-7:28 | `different` |

  Two of them were answered `same` (Dead Air and Alien Girl): in both
  the last minute or two of A plays against a section of B heard
  minutes earlier. None of the far offsets is 90 s; the nearest are
  Tower Bass's 22 s and Klute's 132 s, so the user's "90 s" is read as
  "a minute or more".

  The mechanism: `placement::tile` lets each moment follow the
  strongest alignment covering it, and goes on with the previous one
  only where none covers it. Where the main alignment ends, a weaker
  alignment of 60 hits or more (`placement::followed`) that covers the
  next stretch takes over, though B under the main alignment still has
  audio there: Dead Air's B under alignment 1 would play to A 6:31,
  past A's end (6:27.5), and Alien Girl's to A 7:02. The weaker
  alignments are the outro played against a section of B that repeats
  it (an intro or a break). Why the `pair` search's main alignment ends
  at A 5:20 and A 4:50 although both files go on is not settled here;
  by inference the repeated material matches at both offsets and the
  search's line for the repeat is the denser in those windows. The
  page change does not depend on it.

## Step 2: the design

**What the listener gets.** A new control key `O` (for "offset")
cycles the placement of the aligned lanes through up to three states,
in this order, starting from the first on every question:

1. **alignments** (the default, today's behaviour): the lane plays its
   `pieces`, following every alignment the producer tiled.
2. **main alignment**: the lane plays one piece over the whole
   timeline at the main alignment's offset and speed (the alignment
   with the most hits, the first of equals), clipped to the file, so
   two rips of one recording play the same moment end to end.
3. **from the start**: the lane plays its file from its own start at
   the timeline's start, at the main alignment's speed (offset 0 for a
   pair, whose timeline is A's own time from 0), clipped to the file:
   no offset at all, for rips whose alignment is in doubt.

`O` passes over states no lane of the question has; on a question
without them it says so. The transport line shows the state ("lanes
placed by every alignment", "by the main alignment throughout", "from
their files' starts"), each aligned lane's label gains it ("2 · B ·
main alignment"), and the lane's info line already shows the playing
piece's offset, speed and label, so it follows the state. The legend
and the help list `O`.

**Per question.** The state resets to "alignments" when a question
opens, as speed matching, the loop and the blend do (`Player.open`).
Conservative: every question first plays as before, and a placement
chosen for one pair cannot carry into the next unnoticed.

**The question format: two optional lane fields.** The main alignment
cannot be told from today's files: a piece carries `offset`, `speed`
and a free-text `label`, not its hits, and the producers' labels
differ ("alignment 1: 3298 hits" in the pack, "alignment 1" in
`shared`). So a lane gains two optional fields, each a list of pieces
under the same rules as `pieces`:

- `main`: the lane at its main alignment throughout;
- `from_start`: the lane's file from its start at that alignment's
  speed.

The producers compute them with `audition::placement` (new
`placement::main`, `placement::throughout`, `placement::from_start`),
so the page still only evaluates `offset + speed × t` and picks a list;
it gains no placement arithmetic. `schema::check` checks them as it
checks `pieces`, refuses them on the reference lane (which must play
its own time in every state), and an empty list is refused. Old files
without them load unchanged (`serde(default)`); files with them do not
load in older builds (`deny_unknown_fields`), which is acceptable for a
development tool whose files are made again with `questions`.
`schema_version` stays 1: the change only adds optional fields.

Who writes them: `questions pack` for lane B and for the plan's extra
lanes (each placed by alignments), and `questions shared` for lane B.
Not `questions detections` (the mix is the reference and a play's
segments are the evidence; out of this session's scope) nor `folders`
(excerpts, no alignment). A lane B with no alignment to follow ("not
aligned", from its start) gets neither field.

**Staying right in every state.** The player and the lanes view read
the lanes through one function, `placedLanes(question, placement)` in
`timeline.js`, which replaces each lane's `pieces` by its `main` or
`from_start` list when it has one. Scheduling, the playhead clock,
seeking, the windows wanted (`prefetch`), speed matching's rates, the
waveform and the lane info all use the placed lanes, so they follow
the state; a change of state restarts the sound at the same timeline
moment (`restartHere`), as `M` does. The reference lane A never
changes, so the playhead's label (A's own time) stays right. Markers
and shaded spans still describe the alignments; the help says so.

**Answers.** Unchanged: an answer is the question's id and the key;
the record holds no placement. The server lists the files of `main`
and `from_start` too (`session::Listed`), though in every producer
they name the lane's own file.

**The pending question file.** `05-pack-s40.json` lacks the fields.
Making it again with the final build gives the same ids (they derive
from the files), so the answers stay attached. The user's copy in
`work/audition/questions/` is left alone; the session writes a fresh
one to `work/s47/questions/05-pack-s40.json`, checks that it differs
from the user's only by the new fields, and the handoff gives the
command to load it or to replace the user's copy.

**Step 4.** `listening_pack.py render` resolves `--corpus` and `--nas`
with `os.path.abspath` before writing sheets (safer than `questions
pack` resolving against the repository root, which would guess the
directory render ran from). Its default paths are already absolute.

## Decisions after the design

- **(02:07) Start regressions identical** (`regress session-32-linux`
  0:50, `regress session-33-linux-nas` 3:45; all 8 reports identical
  to their baselines apart from `wall_seconds`).
- **(02:20) A visible control beside the key**: a button after the
  transport line ("O · placed by every alignment"), hidden on
  questions with one placement; a click does what `O` does. The
  transport line itself is unchanged.
- **(02:25) `Lane::new`** for lanes with one placement, so the
  producers that write neither field (detections, folders, lane A)
  name only what they set.
- **(02:31) Wording**: the scratch check showed "placed by from the
  file's start"; the page now names the lanes' placement ("placed by
  every alignment", "placed by the main alignment throughout", "placed
  from the files' starts") apart from a re-placed lane's label ("main
  alignment throughout", "from its start").
- **(02:30) The check by hand** (`work/s47/cdp/check.mjs`, uncommitted:
  headless Chrome over the DevTools protocol with Node's own
  WebSocket, against a scratch server on `work/s47/questions/05-pack-s40.json`
  with `--answers` and `--verdicts` on copies in `work/s47/scratch/`;
  results in `work/s47/cdp/check.json` and two screenshots). Item 1
  (Dead Air), lane 2 from A 5:38, playing:

  | State | A (playhead) | B, page | B, from the pieces | Rate | Windows fetched for B |
  |---|--:|--:|--:|--:|---|
  | every alignment | 340.448 | 91.626 | 91.626 (alignment 2) | 0.96008 | 2, 3, 4, and 10 for the marker before |
  | main alignment | 343.449 | 329.952 | 329.952 | 0.95942 | 11, 12 |
  | from the start | 346.450 | 332.390 | 332.390 | 0.95942 | none (11 held) |
  | from the start, `M` off | 349.576 | 335.389 | 335.389 | 1 | 12 |
  | every alignment again | 352.580 | 103.274 | 103.274 | 0.96008 | none (held) |

  Each state was read 3 s after the key: the playhead moved 3.00 s per
  3 s with matching on and 3.13 s (3 / 0.9594) with it off, so the
  playhead follows B's own clock then. The lane label, its info line
  (file time, offset, speed, piece label) and the button showed each
  state; the clock stayed A's time. No answer was sent.
- **(02:33) Step 4** done as designed: `library_roots` in
  `listening_pack.py`, two tests in `test_listening_pack.py`.
- **(02:35) `scripts/check.sh` green** (3:57) after `cargo fmt`; the
  first run stopped at the format check.
