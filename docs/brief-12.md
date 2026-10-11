# Session 12 brief: a faster search, the harness on the saved index, `doctor` and the saved index

Written by the agent for itself on 2026-10-11, so the plan survives context
compaction. Work autonomously and never stop to ask. When something is
ambiguous, take the conservative option, write the decision down and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-12-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 11" to the end).
4. `docs/status.md` (newest entries at the bottom).
5. The newest file in `docs/experiments/`.
6. `docs/roadmap.md` ("Where search time goes", "Search memory at
   scale", "On-disk index"); ADRs 0005, 0010, 0011, 0012, 0013;
   experiments 0075, 0080, 0082, 0083, 0084.

`docs/brief-2.md` to `docs/brief-11.md` hold rules that still apply
unless this brief changes them (among them: edit no configuration file;
test fixtures are generated, never copied from the corpus or the NAS;
long runs start detached under `screen`).

## The prompt, verbatim

```text
Start session 12 of Gunfinger: code only. A faster search with identical
detections, the harness using the saved index, and `doctor` reporting the
saved index. The session may run up to 8 hours, unattended. The NAS
is mounted at /Volumes/atlas, if a step needs it.
Work autonomously and never stop to ask. When something is ambiguous,
take the conservative option, write the decision down and continue.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it),
   docs/brief-11.md and docs/session-11-checklist.md (brief-2 to brief-10
   hold rules that still apply unless this prompt changes them),
   docs/session-notes.md (from "Session 11" to the end, including
   "After session 11: the pre-release fixes, merged"), docs/status.md
   (newest entries at the bottom), docs/roadmap.md ("Where search time
   goes", "Search memory at scale", "On-disk index"), ADRs 0005, 0010,
   0011, 0012 and 0013, and experiments 0075, 0080, 0082, 0083 and 0084.
2. `git status` should be clean at 7ef893f (the merge of the pre-release
   fixes and its follow-up). Write docs/brief-12.md (this prompt
   verbatim, the state at the start, the plan with times) and
   docs/session-12-checklist.md. Point CLAUDE.md's reading list at them;
   keep brief-11 among the older briefs. Commit.
3. Build with --release and copy the binaries to work/bin/s12-start/.
   Run `gunfinger-eval baseline session-12-start`, then `regress
   session-12-start` once with the start build as a check (its calibrate
   line differs from the baseline's because it reads every report in
   work/reports/; status, session 8). Record the NAS peak store's state
   (scripts/analysis/store_revision.py and store_census.py): 33,596
   records, 1,582 skip notes and revision f9f9bc706195a71b expected,
   unless the 395 new NAS files were indexed before the session; then
   record the new state and use it. Check free disk space (77 GB at
   14:26; the saved NAS index is 3.8 GB).

Rules for this session:
- The NAS (/Volumes/atlas/Music/dnb) is read, never written. The NAS peak
  store is read-only: no `index`, `prune` or `--retry-skipped` against
  it; check its revision at the end. If the revision changes during the
  session (new files being indexed), write it down and compare only runs made
  against the same revision.
- Saved index files are written only under ~/.cache/gunfinger/indexes/
  (or where the ADR you write decides, on a local disk), never on
  the NAS. Keep session 11's saved NAS index usable, or rebuild it once.
- Never touch the held-out test set (ed-rush-optical-essential-mix), and
  never identify, decode or index EM_1999_02_07_-_Ed_Rush_Optical.mp3 at
  the NAS root. Run nothing on the candidate mixes of experiment 0065.
- Verdicts are made by ear only: never add or change a line in
  docs/pair-verdicts.txt. Manifests do not change.
- The matcher, the confidence rule and the possible tier do not change.
  Every change must keep detections identical: `regress
  session-12-start` identical after every code change, and each speed
  change also checked at NAS scale as experiment 0084 did (the 2003 mix,
  `gunfinger --config ~/.config/gunfinger/nas-dnb.toml identify
  corpus/sets/razorjack-2003-03-29/razorjack-2003-03-29.mp3 --store-only
  --playback both -f json`, every segment compared with
  work/merge-check/compare.py against the build before the change).
- Out of scope: the containment rule for confident plays, the
  experiments on shared material, the possible tier and edits, any
  clusters search, and test evaluation 5. They wait for our
  decisions and verdicts.
- NAS-scale harness runs write reports into
  work/reports/library-nas-dnb-peaks/. Copy that folder to
  work/s12/before/ first and put session 11's reports back after any
  acceptance run that overwrites them; keep this session's reports under
  work/s12/.
- Heavy runs (searches, regress, scans, profiling) go one at a time,
  timings only on an idle machine, each with wall time, CPU time and peak
  memory recorded. Run anything longer than 30 minutes detached under
  `screen` (the agent's shell stops background commands after about 40
  minutes).
- Each measurement is an experiment (docs/experiments/NNNN-slug.md, at
  most 40 lines, starting at 0085; data with file names only in
  docs/experiments/data/). Commit after each item; check `git status`
  before every commit and never use `git add -A`. Do not push.
- Time: items are in priority order. At 7 hours stop what is running at
  a safe point, write down what is left and do the wrap-up.

Items, in order:

1. Setup (25 min): the steps above.

2. Profile the merged search (30 min). Experiment 0075's method
   (work/s11/item8b.sh: `sample` every 10 ms over the NAS identify of the
   2003 mix, stacks attributed to stages; find 0075's attribution script
   under work/ or scripts/) with work/bin/s12-start/gunfinger. Where does
   the search's CPU go now that a window's hits are sorted by asset first
   above 50,000 hits (0080) and key-locked peaks are shared per hop
   (0082)? Experiment, compared with 0075.

3. A faster grouping of hits (about 2-3 h), if item 2 shows the sort
   still dominates. Asset numbers are dense integers below 65,536, so a
   counting sort (or a radix pass) can group a window's hits by asset in
   linear time instead of comparison sorts; each asset's offsets are then
   sorted as now. Lines depend only on each asset's sorted offsets (equal
   offsets always fall into the same cluster; see the review of 47bc4f0
   in the status entry of the merge), so detections stay identical.
   Measure where it wins (0083's synthetic and real-derived buffers, and
   a per-thread histogram reused across windows) and keep the threshold,
   if any, that the measurement supports. Unit tests comparing the lines
   with the existing path on large mixed-asset buffers. Acceptance:
   scripts/check.sh green, `regress session-12-start` identical, and the
   2003 mix at NAS scale identical in every segment, timed against
   work/bin/s12-start/ on an idle machine. If item 2 shows another stage
   now dominates, write down which and take the largest step with
   identical detections that it suggests instead, under the same
   acceptance; if there is none, stop at the experiment.

4. The harness uses the saved index (about 2 h; roadmap "On-disk index",
   session 11's "Left"). Runs with --other-peaks-dir (scan, sweep,
   recall, and any other command that builds the same index) load a
   saved index of the set they search when it is current, and save it
   when they build one. Read how the harness builds its index first;
   leave-out runs build a different index: either filter at search time
   if that is provably identical, or build without saving; decide in an
   ADR (amending 0011 or a new one), including where the files live, how
   many are kept and what disk space they take. Tests: stale and damaged
   files, a changed store, a leave-out. Acceptance: the development scan
   (stakka-skynet-knowledge) at NAS scale with a rebuilt and with a saved
   index gives identical reports; time both; regress identical.
   Experiment.

5. `doctor` reports the saved index (about 40 min): for the library's
   store, whether a saved index exists, its size and whether it is
   current or stale; other files in the index folder that belong to no
   current store, with their size. It reports only and deletes nothing.
   Tests.

6. Wrap-up (40 min): experiment files; a "Session 12" section at the end
   of docs/session-notes.md, leading with what changed
   (speed, disk use, `doctor`'s new lines); docs/status.md, the
   checklist, the roadmap, the README. Check the NAS store's revision.
   Commit. Last, print what was done, the numbers, the
   decisions waiting and what is left.
```

## State at the start of session 12

- 14:26. `git status` clean at `7ef893f` on `master`. Experiments
  0001-0084; ADRs 0001-0013. Test set: 4 of 5 evaluations used (none in
  this session).
- `/Volumes/atlas/Music/dnb` mounted
  (935 entries
  at the root).
- Release build of `7ef893f` (18 s, incremental) in `work/bin/s12-start/`,
  byte-identical to `work/bin/merged/` (experiment 0084's merged build).
  `baseline session-12-start` saved from the reports the merged build's
  `regress` wrote at 12:58-13:00. `regress session-12-start` with the
  start build (212.7 s wall, 1,915 s user, 0.47 GB): 720 of 720 sweep
  queries and 87, 93 and 79 scan detections identical; calibrate line
  680 -> 658, 18 -> 46, 20 -> 22, the known difference (it reads every
  report in `work/reports/`).
- The NAS store (`~/.local/share/gunfinger/nas-dnb-peaks`, on the
  local disk, 2.4 GB): 33,596 `.peaks`, 33,596 `.tags`, 1,582 `.skip`,
  `library.txt`; revision `f9f9bc706195a71b`, as expected (the 395 new
  files are not indexed); census `work/s12/census-start.json`
  (3,523.4 h).
- The saved NAS index: `~/.cache/gunfinger/indexes/nas-dnb-peaks-23935cfc56936b67.index`,
  3,795,332,786 bytes, written 2026-10-11 03:25.
- What the code does today (read before planning):
  - `search::lines` sorts a window's hits by asset and offset; above
    50,000 hits it sorts by asset first, then each asset's offsets
    (`47bc4f0`, experiments 0080, 0083). Key-locked rungs with one
    rounded hop share their peak extraction (`34cb49c`, 0082).
  - `identify`, `explain` and `stats` load or save the index through
    `index::saved` (ADR 0011); the harness builds in memory.
  - `doctor` does not look at `~/.cache/gunfinger/indexes/`.

## Plan, with times

Start 14:26; the stop for wrap-up is 21:26 (7 hours), the end 22:26.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1 | setup: build, baseline, regress check, store, brief, checklist | 0:25 | 14:51 |
| 2 | profile the merged search on the 2003 mix at NAS scale; experiment 0085 | 0:30 | 15:25 |
| 3 | faster grouping of hits (or the stage item 2 shows); measurement, code, tests, acceptance; experiments | 2:30 | 18:00 |
| 4 | the harness on the saved index: read, ADR, code, tests, acceptance; experiment | 2:00 | 20:00 |
| 5 | `doctor` reports the saved index; tests | 0:40 | 20:40 |
| - | buffer for overruns and review | 0:46 | 21:26 |
| 6 | wrap-up | 0:40 | 22:06 |

## Conservative decisions taken up front

- Every timing compared across builds runs on an idle machine, one heavy
  run at a time, the builds alternated where a pair is timed.
- The NAS 2003-mix check of each speed change keeps its reports under
  `work/s12/`, not `work/merge-check/`, which holds experiment 0084's.
- Session 11's saved NAS index stays in place; an acceptance run that
  needs a rebuilt index uses `--saved-index off` (identify) or the
  harness's own path, so the file is rebuilt at most once.
