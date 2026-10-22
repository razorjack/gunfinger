# Session 36 brief: a faster search on the Linux machine, exactly

Written by the agent for itself on 2026-10-22, so the plan survives
context compaction. Work autonomously and never stop to ask: where a
decision is needed, take the conservative option, write it down here and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-36-checklist.md` (progress).
3. `docs/research/2026-10-21-low-bandwidth-search.md`, experiments
   0167, 0168 and this session's (0169 and up).
4. `docs/session-notes.md` ("Session 35"), `docs/status.md` (newest
   entries), `crates/gunfinger-core/src/search/lines.rs`.

`docs/brief-2.md` to `docs/brief-35.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 36 of Gunfinger: make the search faster on the Linux machine without changing any detection. `docs/research/2026-10-21-low-bandwidth-search.md` consolidates two design reviews of the search for a machine with low memory bandwidth; experiment 0167 measured its items 1 and 2 (each part's end found by a doubling search from the front of the list, parts of 1,024 assets: the Stakka scan at NAS scale 2.0 times faster at 32 workers, identical reports), and 0168 measured the memory at its rated speed (XMP now on, 3600 MT/s: the change still gives 37.9 s against 49.7 s at 32 workers). This session adopts items 1 and 2 in master with tests and full validation, then measures the next exact proposals.

You run unattended overnight: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes to plan the next session; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads, two core complexes with a 32 MiB L3 each), 64 GB RAM, DDR4-3600 with XMP on since experiment 0168 (times before 0168 were at 2133 MT/s and do not compare). `perf` is not installed and there is no sudo: measure with timers you add in a diagnostic build (worker-local, aggregated after the run; no atomics or clocks per hit), not with a profiler.

Environment (sessions 33-35's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- NAS store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records), reached with `--other-peaks-dir`; NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json` (session 33, SHA-256 `37bc419d3d25...`); saved index `work/indexes/library-nas-dnb-peaks.index`. Baselines `session-32-linux` (corpus) and `session-33-linux-nas` (NAS scale). Builds `work/bin/s35-final/` (current master's code). `recall` and `clusters` need `--panels work/s33/panels` on this machine.
- 0167's patch: `work/perf-design/verify/lines.diff`, its scripts `work/perf-design/verify/ab.sh` and `clusters.sh`; the design reports and their scratch in `work/perf-design/` (`gpt-6-astra.md`, `claude-fable-5-1.md`).
- `gunfinger-eval regress` prints what changed and returns success either way: read its output; never rely on its exit status.
- Slot leave-out scans must not sit in a reports directory during `regress` (they change `calibrate`'s counts; session 34).

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule, "Index-size rule" and "Optimise only from measurement"), docs/brief.md (never edit it), docs/brief-35.md, docs/session-notes.md ("Session 33" to the end), docs/status.md (newest entries), `docs/research/2026-10-21-low-bandwidth-search.md`, both design reports in `work/perf-design/`, experiments 0096, 0103, 0108, 0113, 0158, 0167 and 0168, the roadmap's "Where search time goes" and "Search memory at scale", and `crates/gunfinger-core/src/search/lines.rs`, `search.rs`, `index.rs` and `parallel.rs`.
2. Write docs/brief-36.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-36-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s36/listening-digests-start.txt; it must be the same at the end. Experiments number from 0169.
3. Build `work/bin/s36-start`; `regress session-32-linux` and `regress session-33-linux-nas` identical (read the output). Keep `work/bin/s35-final` as the reference build for timing.

Steps:

1. Items 1 and 2 in master. `front_partition` in `search/lines.rs` (`take_part`), with unit tests against `partition_point` on random sorted lists, empty lists and boundaries at the start, at the end and past it; the part size chosen from measurements at 16 and 32 workers (1,024, 2,048, and if cheap the research note's per-window hit budget), at 3600 MT/s. Validation before committing the change to master, every comparison byte for byte apart from timing fields:
   - `scripts/check.sh` green (with the macOS clippy pass);
   - `regress` at both scales identical;
   - the three development scans at NAS scale and their leave-outs (session 33's seeds; 0157);
   - the 48-query clusters sample (`clusters --from-peaks --sample 48`);
   - a full NAS clusters run (session 33's run 2 command with `--cut-sparsest`, in a scratch work directory with the clusters and map copied, never over the shared reports) whose `duplicate-clusters.json` must equal session 33's byte for byte; it is also the session's longest timing (session 33: 52:30 at 32 workers, 2133 MT/s, current code);
   - the sweep (seed 2026) at both scales, through `regress`.
   Commit only if every comparison is identical; otherwise find the cause and do not commit the change. Experiment file: the change, its validation and its times (A/B against `s35-final`, interleaved, at 16 and 32 workers).

2. Stage timers on this machine, in a diagnostic build only (never committed to master unless they cost nothing measurable and read well): the split of a NAS-scale scan's CPU between query analysis, taking parts, grouping, the pre-filter, sorting and lines, and the second pass, at 16 and 32 workers, with items 1 and 2 in. This replaces the profile the research note asks for and decides which of steps 3-5 are worth doing.

3. Narrower hits (the research note's item 4): first the grouped hits as 4 bytes (frame and an anchor index relative to the window's first anchor, with a per-window fallback to the wide form when an index does not fit), then raw hits as 6 bytes. Each measured A/B at 16 and 32 workers against step 1's result, kept only above the note's stop point (3% at 16 workers or removing a 32-worker penalty) and only with identical output and the same validation as step 1 (the full clusters run may be replaced by the clusters sample if the full run was identical in step 1 and the change touches only the hit representation).

4. Grouping without the `part` buffer (item 5), only if step 2 shows taking and grouping above about 15% together; same rules.

5. Repeated hashes read once (item 6): first count, on real windows, how many requested posting bytes are repeats of a hash already requested in the window (and within 2-4 adjacent windows); implement only if the count makes the note's stop point reachable.

6. Ahead, only when 1-5 are done: the same A/B for `identify` with the CLI on the three development mixes at NAS scale (the user's real command; reports byte-identical apart from timing), and a measured recommendation on the default worker count (one per hardware thread or per core) after this session's changes. The default itself is our decision: do not change it.

An experiment file (at most 40 lines) for each of steps 1-5 that produced a measurement, written before the next experiment starts. Delete losing code from the tree (AGENTS.md: no dead code); its result lives in the experiment.

7. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.

8. Wrap-up: a "Session 36" section at the end of docs/session-notes.md with a handoff (what is in master, the new times at 16 and 32 workers, what was dropped and why, what is left of the research note); update docs/status.md, the checklist, the roadmap's "Where search time goes" and the research note. Final checks: `scripts/check.sh` green, `regress session-32-linux` and `regress session-33-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, `git status` clean. Commit. Your final message: what changed in master, the measured times, what was kept and dropped, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- Only exact changes reach master: detections, evidence, boundaries, speeds and ordering bit for bit identical, as shown by the validation above. No threshold, parameter, rule or default changes; no change to postings, `hash::design()` or the saved index format. A change that alters output is not committed.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Timing runs one at a time on an otherwise idle machine, interleaved A/B, repeated where the ranges overlap.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes are the only real mixes this session searches; name them explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration. Do not replace session 33's clusters file in `work/reports/`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening.
- AGENTS.md: no SIMD, no `unsafe`, synchronous Rust, an experiment file of at most 40 lines written before the next experiment starts, evaluation JSON in docs holds file names only, cross-platform code (ADR 0020).
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start.
- Scratch data goes under work/s36/; keep it under about 20 GB. Session 35's renders in `work/mixes/` (11 GB) may be deleted.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: plan for about 2.5 hours. The user's soft deadline is 11:00, but the orchestrating thread wants to launch more sessions after this one: when steps 1-3 are done and it is past 05:30, skip what is left of 4-6, list it in the handoff and wrap up.
```

## State at the start of session 36

- 2026-10-22, 02:34. `master` at `e0c6316`, clean. Git identity `Jacek
  Galanciak <jacek.galanciak@gmail.com>`. Load average 0.04; no tmux
  server running.
- `docs/pair-verdicts.txt`: SHA-256 `ad0fc785d26a...`
  (`work/s36/listening-digests-start.txt`). No answer log in this clone.
- Experiments 0001-0168; this session numbers from 0169. Test set: 4 of
  5 evaluations used (none in this session).
- Session 35's renders in `work/mixes/` (11 GB) deleted at 02:35.
- `work/reports/library-nas-dnb-peaks/` holds session 33's clusters and
  map, and the 11 development reports at NAS scale (3 scans, 8
  leave-outs) that 0157 scored; validation scans run in a scratch work
  directory (`work/s36/w`, the saved index hard-linked, the clusters and
  the map copied) and are compared with those.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; brief, checklist, `CLAUDE.md`; digest; start build; both regressions | 02:34-03:05 |
| 1 | `front_partition` and tests; part size from A/B (1,024, 2,048, a hit budget) against `s35-final` at 16 and 32 workers | 03:00-03:35 |
| 1 | validation queue: `check.sh`, both regressions, 11 development scans, clusters sample, full clusters run; experiment 0169; commit | 03:35-04:35 |
| 2 | stage timers (diagnostic build, not committed); experiment 0170 | beside and after the queue |
| 3 | grouped hits as 4 bytes, then raw hits as 6 bytes; A/B; experiments | 04:35-05:20 |
| 4-6 | only if 1-3 are done before 05:30 | |
| 7 | read-only review by a subagent | |
| 8 | notes, status, roadmap, research note, checklist; final checks | |

## Decisions

- **(02:46) Start build and regressions.** `work/bin/s36-start/` is
  byte-identical to `s35-final/` (no code changed since). `regress
  session-32-linux` identical (1:12; 720 of 720 sweep queries, 89, 97 and
  83 scan detections, the calibrate lines) and `regress
  session-33-linux-nas` identical (8:16 at the default 32 workers; 720
  sweep queries, 4,253, 4,244 and 4,212 detections, the calibrate lines).
- **(02:46) The reference reports** for the development scans at NAS
  scale are copied to `work/s36/ref-reports/` before any run, so that a
  regression writing over `work/reports/` cannot change what the
  validation compares with.
- **(02:47) Part-size candidates**: constants 1,024 and 2,048 with the
  front search, and a per-window hit budget (parts per window the next
  power of two of the window's hits times 16 bytes over 1.5 MiB, from 256
  to 4,096 assets; `work/s36/budget.diff`), each against `s35-final`,
  interleaved, two rounds at 16 and 32 workers (`work/s36/ab-parts.sh`).
- **(03:00) Parts of 1,024 assets.** All 16 A/B reports are identical to
  session 33's apart from `wall_seconds`. Search time at 16 and 32 jobs:
  `s35-final` 47.2 and 49.0-49.2 s; 1,024 45.3-45.4 and 37.9-38.0 s;
  2,048 45.5-45.6 and 38.3-38.8 s; the hit budget 44.1 and 37.0-37.1 s.
  The budget beats 1,024 by 2.7% and 2.3%, below the 3% the design report
  set for adopting it, so the constant 1,024 is kept (the simpler code).
- **(03:03) `check.sh` failed once** in
  `decode::watch::tests::a_child_that_writes_only_diagnostics_is_still_running`
  with "Text file busy": the test race ADR 0020 describes (a stand-in
  script held open by a child forked by another test thread). The rerun
  was green (179 s; clippy for Linux and macOS, every test).
- **(03:06) Variants built before the timed runs**, so that nothing
  compiles while a run is timed: grouped hits as 4 bytes
  (`s36-narrow4`), raw hits as 6 bytes on top of it (`s36-raw6`),
  grouping straight from the lists on top of it (`s36-nopart`, step 4,
  run only if step 2 allows), the stage timers (`s36-timers`) and the
  repeated-hash count (`s36-repeats`). Their sources are in
  `work/s36/lines.rs.*` and `work/s36/timers.diff`.
- **(03:14) Both regressions identical with `s36-step1`** (corpus 1:16,
  NAS scale 6:11 against 8:16 with the start build), and every report
  they write is identical to its baseline's apart from `wall_seconds`,
  the sweeps included.
- **(03:27) Step 2's timers ran inside step 1's queue**, after the
  development scans and before the full clusters run, so that the full
  run was the last timed run of step 1. Taking and grouping are 35% of the
  CPU at 16 jobs (31% at 32), above 15%: step 4 is measured. Experiment
  0170.
- **(03:27) Steps 3 and 4 in one A/B** (`work/s36/queue-step3.sh`, started
  after the full clusters run): `s36-step1` against `s36-narrow4`,
  `s36-raw6` and `s36-nopart`, one round at 16 and 32 jobs, each
  candidate twice between two base runs; then step 5's count. A second
  round only where ranges overlap.
- **(03:50) Step 1 committed.** The full NAS clusters run is
  byte-identical to session 33's (23:02 at 32 jobs; session 33 52:30 at
  2133 MT/s with the old code). Every comparison of the validation was
  identical (0169).
- **(03:51) 0169's last lines and the step 3 A/B.** 0169 was written
  while step 1's queue ran; its validation lines were completed at 03:50,
  two minutes after the chained A/B of steps 3 and 4 had started, with
  only step 1's results in it.
- **(04:03) Steps 3 and 4 lose; nothing kept.** Grouped hits as 4 bytes
  47.3 s against 45.2-45.3 s at 16 jobs, raw hits as 6 bytes 48.8-49.1 s,
  grouping from the lists 48.7-48.9 s; all slower at 32 jobs too, with
  identical reports (0171, 0172). The ranges do not overlap, so no second
  round. The queue was stopped after the A/B so that the experiment files
  were written before step 5's count; the A/B's last base run, which had
  failed to start (a run label past the file name limit), was run again
  alone.
- **(04:04) Step 5 counted, not implemented** (0173): 5.1% of requested
  postings repeat a hash of the same window, 14.9% within 4 windows; the
  saving is bounded by taking parts' 6.3% of CPU, so about 1% at most.
- **(04:05) Step 6 started** at 04:04, with steps 1-5 done before 05:30:
  `gunfinger --config ~/.config/gunfinger/nas-dnb.toml identify --jobs N
  -f json <mix>` (the user's command, turntable playback by the
  configuration) on the three development mixes' audio in
  `corpus/sets/`, named one by one, `s35-final` against `s36-step1` at 16
  and 32 jobs (`work/s36/identify-ab.sh`). The CLI only reads the NAS (the
  library listing) and the store; its saved index in
  `~/.cache/gunfinger/indexes/` was current and was not rewritten. A
  warm-up run took 8:54, nearly all of it waiting for the NAS's
  directory listing (process state `D` in `wait_for_response`); the
  next run took 45 s, so the warm-up is left out of the comparison.
- **(04:33) Step 6's result** (0174): reports byte-identical across
  builds, job counts and two rounds; user CPU 3-4% lower at 16 jobs and
  16-22% at 32; wall time within its noise at 16 jobs. Recommended: keep
  one worker per hardware thread; the default is not changed.
- **(04:34) Final build** `work/bin/s36-final/` from `HEAD`; it differs
  from `s36-step1` only because the committed comment shifts line
  numbers. Final checks (`work/s36/final.sh`) started beside the
  read-only review.
- **(04:45) The review's findings** (a separate agent, read-only,
  04:35-04:44): the code correct and no losing code left; five figures
  wrong in the documents (0169's 2,048 CPU range; the hit budget's 2.3%,
  not 2.2%; narrow hits 4.4% and 8.2% slower, not 4.5% and 8%;
  `identify` 1.8-6.2 s, not 1.7-6.3 s; 7.3 s between rounds, not 7 s),
  and wording: 0169's two tests, an unmarked inference in 0170, "16%
  faster" for 16% less time, the times table's column label, a code
  comment that held only after its loop, and the part size's comment
  citing 0096 and stating an inference as fact. All fixed. The binaries
  built after the comment edits are not byte-identical to `s36-final`'s,
  so `s36-final` was rebuilt from the new `HEAD` and the final checks run
  again with it.
