# Session 37 brief: lost sides of a pitch ride

Written by the agent for itself on 2026-10-22, so the plan survives
context compaction. Work autonomously and never stop to ask: where a
decision is needed, take the conservative option, write it down here and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-37-checklist.md` (progress).
3. Experiment 0166 and this session's (0175 and up); the roadmap's
   "Speed that wanders within a play".
4. `docs/session-notes.md` ("Session 35" and "Session 36"),
   `docs/status.md` (newest entries),
   `crates/gunfinger-core/src/search.rs`, `search/chains.rs`,
   `search/refine.rs`, `crates/gunfinger-eval/src/mixes.rs` (`ride`).

`docs/brief-2.md` to `docs/brief-36.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 37 of Gunfinger: find out why the search drops one side of a play whose speed changes within it, how small a change does it, and whether an opt-in variant recovers it without new wrong answers. Experiment 0166 (session 35) found it in generated long mixes whose every play rides the pitch once by 1-3% over 5-15 s: all 382 ridden indexed plays have a confident play, but 192 lose a side of 30 s or more, and on one mix the CLI finds Figurehead and Sin on one side of their rides only although each side searched alone is confident (2,130 and 3,082 hits). DJs ride the pitch on vinyl, so this may cost real identifications and real plays' time. This is a measurement session: the product's rules and defaults do not change.

You run unattended overnight: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes to plan the next session; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600 (XMP on since experiment 0168). Since session 36 the search is faster (0169): 32 workers beat 16, so use the default jobs. Long mixes render at about 14 GiB per seed of 16; one heavy job at a time.

Environment (sessions 33-36's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- NAS store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records), `--other-peaks-dir`; NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`; saved index `work/indexes/library-nas-dnb-peaks.index`. Baselines `session-32-linux` (corpus) and `session-33-linux-nas` (NAS scale). Build `work/bin/s36-final/` (master). `recall`, `shared` and `clusters` need `--panels work/s33/panels` on this machine.
- Session 35's rides generator: `mixes --kind rides --seed <s> --count 16` (and `--kind long` for its control); its scripts `scripts/analysis/ride_coverage.py`, `ride_gaps.py`; the CLI check `work/s35/logs/ride-cli-check.log`. Session 35's renders were deleted; render again (seeds 3521-3522 rides and 3531-3532 control are session 35's; use new seeds for anything tested against criteria).
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans must not sit in a reports directory during `regress`.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-36.md, docs/session-notes.md ("Session 34" to the end), docs/status.md (newest entries), experiment 0166 and its data, the roadmap's "Speed that wanders within a play", "The edge of the ladder" and "Detection boundaries", experiments 0022, 0024, 0030, 0032, 0033 and 0059, ADR 0006 and ADR 0008, and the code of `search.rs`, `search/chains.rs`, `search/refine.rs` (`speed_per_stretch`), `plays.rs`, `confidence.rs`, `speed.rs`, the CLI's `explain`, and `mixes.rs` (`--kind rides`).
2. Write docs/brief-37.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-37-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s37/listening-digests-start.txt; it must be the same at the end. Experiments number from 0175.
3. Build `work/bin/s37-start`; `regress session-32-linux` and `regress session-33-linux-nas` identical.

Steps:

1. Where the side is lost. On re-rendered rides mixes, follow ridden plays that lose a side through the pipeline: lines per window and rung, chains (ADR 0008's link rules across a speed change), the second pass at one fitted speed (`refine`), overlap suppression and confidence, plays and same-audio merging. Use `explain` at the lost side, harness traces, or a diagnostic build (never committed). Name the stage and the rule that drops it, with counts over all lost sides, and check that the generator renders a ride as a turntable would (pitch and tempo together, continuous). An experiment file.

2. How small a change does it. A dose-response on new seeds: rides of 0.25, 0.5, 1, 2 and 3%, over short (2-5 s) and long (10-20 s) ramps, up and down, and a slow drift (for example 0.5% over 60 s); per condition, ridden plays losing a side of 30 s or more, the time lost, and confident plays lost entirely. Compare with the within-play speed variation measured in the development mix (0022: ±0.2%) and with what the development scans show: in the three development mixes at NAS scale, list plays of one asset found as separate segments at different fitted speeds, or with unexplained holes, as candidates (no listening; say they are candidates). An experiment file.

3. What exists already. On the same mixes, the opt-in options that might recover a side: `--speed-per-stretch`, `--extra-rungs`, `--trim-ends`, `--single-pass` and any other the harness has, each measured for sides recovered, time recovered and cost. An experiment file.

4. A candidate fix, only if step 1 names a rule a variant can change: an opt-in variant in the harness (a matcher option, never the default), with its criteria written and committed before it runs on new test seeds: sides and time recovered on the rides mixes, and no new wrong confident answer and no lost right one on the sweep (seed 2026) at both scales, the three development scans and leave-outs at NAS scale, the long and short generated mixes, and the first shared-material panel's development half at NAS scale (session 25's `shared` run, where related versions are the risk). Search time beside. Not adopted: adoption is our decision and needs an ADR; say what it would decide. An experiment file.

5. Ahead, only when 1-4 are done: whether a sustained ride also costs recall in the sweep's kind of excerpts (a `robust` condition with a ride inside a 30 s excerpt, if `robust` has none), at 262 tracks and NAS scale.

6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.

7. Wrap-up: a "Session 37" section at the end of docs/session-notes.md with a handoff; update docs/status.md, the checklist and the roadmap ("Speed that wanders within a play" and anything else whose facts changed). Final checks: `scripts/check.sh` green, `regress session-32-linux` and `regress session-33-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, `git status` clean. Commit. Your final message: where the side is lost, from how small a ride, what the existing options and the variant recover at what cost, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's behaviour does not change: no threshold, parameter, rule or default changes; a variant is opt-in, harness-reachable, tested, and keeps `regress` identical when off. Harness additions (generator options, `robust` conditions, scripts) are tested and keep `regress` identical.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; where a finding would need it, list it as a question for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts, evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start.
- Scratch data goes under work/s37/ and renders under `work/mixes/`; delete renders you no longer need and keep the session's data under about 40 GB.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: plan for about 2.5 hours. The user's soft deadline is 11:00 and the orchestrating thread may launch one more session after this one: when steps 1-3 are done and it is past 07:45, skip what is left of 4-5, list it in the handoff and wrap up.
```

## State at the start of session 37

- 2026-10-22, 04:56. `master` at `a0b91e4`, clean. Git identity `Jacek
  Galanciak <jacek.galanciak@gmail.com>`. Load average falling from
  session 36's runs; no tmux server running.
- `docs/pair-verdicts.txt`: SHA-256 `ad0fc785d26a...`
  (`work/s37/listening-digests-start.txt`). No answer log in this clone.
- Experiments 0001-0174; this session numbers from 0175. Test set: 4 of
  5 evaluations used (none in this session).
- `work/mixes/` is empty of session 35's renders (deleted in session 36).
  Disk: 826 GB free on `/home`.

## What the code says before any run

Read from the code, to be checked by measurement in step 1:

- Chains (`chains.rs`) link lines of successive windows that predict the
  same reference time within 4 frames plus 0.4% of the distance, and
  (ADR 0008) only from rungs at most 1.5 steps (0.6%) apart. A ride that
  changes the speed by more than that between two lines splits the play
  into two chains, as Star Trails split in 0030; each side is then a
  detection of its own.
- The second pass (`refine.rs`) measures each detection at one fitted
  speed over its chain plus 10 s (`MARGIN_SECONDS`) either side, hits
  within 24 frames of the alignment.
- `strongest_per_moment` (`search.rs`) then drops every detection that
  overlaps a stronger one of the same asset in query time, after the
  first pass and again after the second. ADR 0008 already notes this
  for a needle skip ("the part before the skip, which then overlaps it,
  is dropped"). The two sides of a ride are adjacent in query time, so
  if either side's measurement reaches into the other's span, the
  weaker side is dropped whole: the first hypothesis.
- The generator (`mixes.rs`, `ridden`) reads a native-speed render at
  the integrated position `source_at` by linear interpolation: pitch and
  tempo move together and continuously.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; brief, checklist, `CLAUDE.md`; digest; start build; both regressions | 04:56-05:20 |
| 1 | render rides seeds 3721-3722 (scratch, not tested against criteria); a diagnostic build that logs each detection before and after `refine` and each drop by `strongest_per_moment`; count over all lost sides; experiment 0175 | 05:20-06:05 |
| 2 | generator options for the size, ramp, direction and a drift (tested; `regress` identical); dose-response on new seeds; development scans' split plays as candidates; experiment 0176 | 06:05-06:50 |
| 3 | existing options on the same mixes; experiment 0177 | 06:50-07:20 |
| 4 | a variant, if step 1 names a rule; criteria committed before its test seeds | 07:20-07:45 if time |
| 5 | only if 1-4 are done | |
| 6-7 | review, notes, final checks | |

## Decisions

