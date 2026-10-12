# Session 16 brief: the product's principle, one entry per track, the tracklist by default, a stall timeout

Written by the agent for itself on 2026-10-13, so the plan survives context
compaction. Work autonomously and never stop to ask. When something is
ambiguous, take the conservative option, write the decision down and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-16-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 14" to the end).
4. `docs/status.md` (newest entries at the bottom).
5. `docs/roadmap.md`, ADRs 0006, 0013, 0016 and (once written) 0017.
6. The newest file in `docs/experiments/`.

`docs/brief-2.md` to `docs/brief-15.md` hold rules that still apply
unless this brief changes them (among them: edit no configuration file;
test fixtures are generated, never copied from the corpus or the NAS;
runs longer than 30 minutes start detached under `screen`; heavy runs one
at a time; NAS-scale harness runs overwrite
`work/reports/library-nas-dnb-peaks/`, so copy it first). This session
changes one of them: no wall time or CPU figures are reported as
findings.

## The prompt, verbatim

```text
Start session 16 of Gunfinger, an overnight session: make the output and
unattended runs fit our product principle (below). It runs
unattended from about 01:30 and must finish by 10:30. The NAS is mounted at /Volumes/atlas. Do not report
timings as findings, and do not touch processes this session did not
start. Work autonomously and never stop to ask. When something is
ambiguous, take the conservative option, write the decision down and
continue.

Our principle (2026-10-13), to record in the repository:
"I want gunfinger to be something you fire, get results and forget. Not
something you work with. If multiple files are matched for one track
detected in the DJ set, the user will check all of them and most likely
find the correct title among the detected files. If the user has
mis-managed tags or library, it's their fault, and gunfinger should not
be a tool to fix the library. It's not a set detection studio. Users are
responsible for tagging their files, and the quality of titles detected
by gunfinger follows the quality of the data they indexed."
Consequences: precision matters most (nobody reviews the output, so a
confident wrong title is the worst result); the output lists every
matching file and flags disagreeing titles without choosing; speed and
unattended reliability matter. Report edits (confirm, reject, rename
plays) and the Track/AudioAsset model are out of scope for the product.
The listening page (`gunfinger-eval audition`) stays a development tool
for evaluation ground truth.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it),
   docs/brief-15.md and docs/session-15-checklist.md (brief-2 to brief-14
   hold rules that still apply unless this prompt changes them),
   docs/session-notes.md (from "Session 14" to the end), docs/status.md
   (newest entries at the bottom), docs/roadmap.md, ADRs 0006, 0013 and
   0016, crates/gunfinger/src/export.rs, crates/gunfinger/src/report.rs
   (`same_recording`, `compared_title`), crates/gunfinger-core/src/decode.rs,
   the `index` command and how it remembers failed files, and
   crates/gunfinger-eval/src/audition/answers.rs and audio.rs.
2. `git status` should be clean at 5b09bc3. Write docs/brief-16.md (this
   prompt verbatim, the state at the start, the plan with times) and
   docs/session-16-checklist.md. Point CLAUDE.md's reading list at them;
   keep brief-15 among the older briefs. Commit.
3. Build with --release and copy the binaries to work/bin/s16-start/. Run
   `gunfinger-eval baseline session-16-start`. Record the NAS peak store's
   state (scripts/analysis/store_revision.py): 33,596 records and revision
   f9f9bc706195a71b expected.

Steps:

1. Setup (20 min): the steps above.

2. The principle recorded (30 min). A short ADR (0017) for the product's
   scope and the default output (step 4): our words, what is out
   of scope and why, what follows for priorities. Mark the roadmap's
   "Report edits" and "The Track/AudioAsset model" items out of scope (the
   listening work stays as history), and say in "Titles of mislabelled
   files" that the product flags and never chooses. A short AGENTS.md
   paragraph so future sessions do not propose editing workflows or
   tag-fixing features. Commit.

3. One tracklist entry per track (about 2 h). Seen by the
   reviewer on the 2003 development mix at NAS scale (`gunfinger --config
   ~/.config/gunfinger/nas-dnb.toml identify
   corpus/sets/razorjack-2003-03-29/razorjack-2003-03-29.mp3 --store-only
   -f tracklist`; a saved report is work/merge-check/2003-s15.json, which
   `gunfinger show ... -f tracklist` renders):
      15.   42:13  DJ HIDDEN - skektics
      16.   42:15  Dj Hidden - Skektics
   One track, listed twice. The alien5ive upload plays at +0.98% from
   1:01 of its file, the rips at +4.32% from 0:21: the upload's timing
   differs by about 40 s (a longer lead-in), so `same_recording`'s
   alignment test (`track_started_at` within SAME_RECORDING_SECONDS)
   keeps them apart, though the titles compare equal under
   `compared_title`.
   - Confirm that cause, then group into one tracklist entry the plays
     that overlap the same passage of the mix (the existing overlap test)
     and whose artist and title compare equal after the existing
     normalisation (`compared_title`'s letters and digits, lower case), even
     when their alignment differs. Every file stays listed under the
     entry ("also: ..."); tags are never rewritten; a play that shares
     material, or whose title is missing, is not grouped by title. Two
     different recordings that carry the same title (an edit, a re-press)
     become one entry: by our rule the title is right, and every
     file is still named. Write the rule down (ADR 0017 or 0006's note).
   - Measure on every stored report of the development mixes at both
     library sizes how many tracklist entries change, and list each
     change. The JSON report and the detections do not change (`regress`
     identical); this is output only. Tests.

4. The tracklist as the default output (about 1 h; agreed).
   `identify` and `show` print the tracklist unless `-f` says otherwise;
   the current table stays available under its name (`-f human`; decide
   whether a clearer name with the old one kept as an alias is worth it,
   conservatively). Check what the default must handle: several
   recordings in one run (a heading per recording), `--quiet`, colour,
   recordings with no plays, `--save-dir`. Update the README, the help
   text, docs that show the default output, and tests. Check that no
   script or harness code parses the default output (they should use
   `-f json`). Commit.

5. A stall timeout on FFmpeg (about 1.5 h). A decode that produces no
   output for a long time (a hung NAS) must fail that file with a clear
   error instead of stalling the run. Use a stall timeout (no bytes for N
   seconds), not a total one, so long mixes on a slow NAS still decode;
   choose N conservatively and document it. `index` records such a file
   as failed for this run and continues; decide whether a timeout counts
   as a permanent failure the way a corrupt file does, conservatively (a
   NAS hiccup is often transient), and document it. `identify` fails with
   the message. The listening page's decoding gets the same timeout.
   Tests through a seam (a fake decoder that hangs; temporary folders;
   no change to the process-wide PATH in parallel tests). Commit.

6. Rust 1.89 and the answer-log lock (30 min; agreed). Raise
   the workspace's rust-version to 1.89 and lock the listening page's
   answer log (and the verdicts file while writing) so that two servers
   cannot write one log at once; a second server says so and exits, or
   waits, decided and documented. Test. Commit.

7. A read-only review of the session's code by a subagent; fix what it
   finds.

8. Wrap-up (30 min): a "Session 16" section at the end of
   docs/session-notes.md (what changed in what the user sees, with the
   2003 mix's new default output as an example; the timeout and what
   happens to a hung file; anything that needs doing). docs/status.md, the
   checklist, the roadmap. `regress session-16-start` identical;
   scripts/check.sh green; the NAS store's revision. Commit. Last, print
   what was done, the checks, and anything left.

Rules for this session:
- No performance work or timing claims; the
  next speed step waits.
- The held-out test set (ed-rush-optical-essential-mix) is out of every
  step: do not open its manifest or reports, do not rescore any
  evaluation, and run test evaluation 5 never. Never identify, decode or
  index EM_1999_02_07_-_Ed_Rush_Optical.mp3 at the NAS root, and run
  nothing on the candidate mixes of experiment 0065.
- The NAS (/Volumes/atlas/Music/dnb) is read, never written. The NAS peak
  store is read-only: no `index`, `prune` or `--retry-skipped` against
  it; test `index` only on temporary stores and folders. Check the
  store's revision at the end.
- Agents never write verdicts or listening answers:
  docs/pair-verdicts.txt and work/audition/answers.jsonl do not change.
  Manifests do not change.
- The matcher, the confidence rule and the detections do not change:
  `regress session-16-start` identical at the end; scripts/check.sh
  green.
- Out of scope: report edits, the Track/AudioAsset model, the containment
  rule and the possible tier (waiting for our decision), development-mix
  choices, speed.
- Commit after each step; check `git status` before every commit and
  never use `git add -A`. Do not push.
```

## State at the start of session 16

- 01:46. `git status` clean at `5b09bc3` on `master`. Experiments
  0001-0094; ADRs 0001-0016. Test set: 4 of 5 evaluations used (none in
  this session).
- `/Volumes/atlas` mounted over SMB (`smbfs`, not `soft`), 935 entries at
  `/Volumes/atlas/Music/dnb`. This session leaves every process it did not
  start alone and never kills by name.
- Toolchain: rustc 1.98.0 (Homebrew, `stable` in `rust-toolchain.toml`);
  the workspace declares `rust-version = "1.85"`.
- Stored `identify` reports of the development mixes (for step 3): 21 of
  the 2003 mix and 1 of the stakka mix at NAS scale (`work/merge-check/`,
  `work/s9/`, `work/s12/`, `work/s13/`, `work/s15/accept/`,
  `work/razorjack-2003-03-29/`), 8 of the stakka mix at 262 tracks
  (`work/demo/`, `work/reports/identify-dev.json`, `work/s3-savedir/`,
  `work/s4/`), older builds among them. Harness scan reports hold no tags
  or track times, so no tracklist can be rendered from them.

## Plan, with times

Start 01:50; hard end 10:30.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1 | setup: brief, checklist, build, baseline, store revision | 0:25 | 02:15 |
| 2 | the principle recorded: ADR 0017, roadmap, AGENTS.md | 0:30 | 02:45 |
| 3 | one tracklist entry per track: cause, rule, measurement, tests | 2:00 | 04:45 |
| 4 | the tracklist as the default output | 1:00 | 05:45 |
| 5 | a stall timeout on FFmpeg and FFprobe | 1:30 | 07:15 |
| 6 | Rust 1.89 and the answer-log lock | 0:30 | 07:45 |
| 7 | read-only review by a subagent; fixes | 0:45 | 08:30 |
| 8 | wrap-up: notes, status, checklist, roadmap, regress, checks, store revision | 0:45 | 09:15 |

About an hour of slack. If a step overruns by more than 30 minutes, the
later steps keep their order and the wrap-up starts by 09:30 whatever is
left; an unfinished step is reverted or committed only in a working,
tested state, and named in the notes.

## Conservative decisions taken up front

- Step 3 reads stored reports only and adds fresh `identify` reports of
  the two development mixes at both sizes with the start build, one run
  at a time (the NAS store read-only through `--store-only`; the saved
  index in `~/.cache/gunfinger/indexes/` is used as it is). No timings
  are reported.
- Grouping by title compares the tags each play's own file carries in
  the report (the name the tracklist prints for it); a play without a
  stored artist and title tag is not grouped by title, as its name would
  come from the file name.
- Every check of `index` uses temporary stores and folders. Nothing runs
  `index`, `prune` or `--retry-skipped` with the NAS configuration.
- `regress session-16-start` runs once at the end with the final build;
  if the code a step touches could change detections, it runs after that
  step too.
