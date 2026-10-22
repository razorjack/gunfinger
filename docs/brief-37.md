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

- **(05:06) Start build and regressions.** `work/bin/s37-start/` is
  byte-identical to `s36-final/` (no code changed since). `regress
  session-32-linux` identical (1:09; 720 of 720 sweep queries, 89, 97 and
  83 scan detections, the calibrate lines) and `regress
  session-33-linux-nas` identical (6:05 at the default 32 workers; 720
  sweep queries, 4,253, 4,244 and 4,212 detections, the calibrate lines);
  every report they write is identical to its baseline's apart from
  `wall_seconds`.
- **(05:06) Step 1 on session 35's seeds.** Step 1 is a diagnosis, not a
  test against criteria, so it renders session 35's rides seeds 3521 and
  3522 again (with `--panels work/s37/panels`, a copy of
  `work/s35/panels`) and checks that the reports equal session 35's
  (copied to `work/s37/s35-reports/` first, since the run writes over
  them). The diagnostic build (`work/s37/diag`, a worktree; never
  committed) only appends, with `GUNFINGER_TRACE_DIR`, each stage's
  detections of 30 hits or more: the chains before the first overlap
  suppression (with their lines), after it, after the second pass, and
  after the second suppression.
- **(05:10) Step 2's generator options.** `mixes --kind rides
  --ride-change A[,B] --ride-seconds A[,B] --ride-direction up|down|either`.
  Every draw takes the same four numbers from the seed's generator as
  session 35's, so one seed plans the same plays, speeds, crossfades and
  ride starts under every condition; only the size, ramp and direction
  differ (a controlled comparison). Non-default draws write
  `mixes-rides-<label>-seed-<seed>.json` and render to
  `work/mixes/rides-<label>-seed-<seed>/`. The default draw is unchanged
  (a test; the step 1 reports compare with session 35's).
- **(05:13) Step 1's answer** (0175): both reports equal session 35's;
  all 192 lost sides are confident after the second pass and dropped by
  the second `strongest_per_moment`, because the second pass's 10 s
  margin moves both sides' facing ends into the ramp, a median 4.7-5.2 s
  of overlap; the chains never overlap. So step 4 has a rule to vary.
- **(05:13) Step 2's design.** One new seed, 3701, 16 mixes per
  condition, every condition planning the same plays: a 0% ride (the
  linear-interpolation render without a change, the control), 0.25, 0.5,
  1, 2 and 3% over 2-5 s and over 10-20 s, and a drift of 0.5% over 60 s
  (cut short where the play ends first). Directions drawn either way and
  split in the analysis, which halves the renders (12 instead of 23).
  Runs with the trace build (`work/s37/target-diag2`, logging only).
- **(05:20) Step 4's variant and criteria, committed before it runs
  anywhere.** The variant: `--overlap-by-chains` (harness;
  `Options::overlap_by_chains` in the core, off by default): after the
  second pass, a detection overlaps a stronger one of the same asset only
  where their first-pass chains overlap. The first suppression has
  already left no two chains of one asset overlapping, so the variant
  skips the second suppression. It cannot remove a detection; it can only
  keep ones the default drops. Test seeds: rides 3741 and 3742 (default
  draw, 16 mixes each, never rendered before). Criteria:
  - V1, recovery: of the sides of 30 s or more that the default loses on
    the test seeds, the variant covers at least 90% with a confident
    detection of the play's own file, and at least 90% of their seconds.
  - V2, no new wrong confident answer: the count of wrong confident
    detections does not rise, and no confident detection of an asset
    that is wrong at its time appears that the default lacks, on the
    sweep (seed 2026) at 262 tracks and at NAS scale, the three
    development scans and their eight leave-outs at NAS scale, the short
    and long generated mixes of seed 2026, the first shared-material
    panel's development half at NAS scale, and the test seeds.
  - V3, no right answer lost: every confident identification of the
    default stays (by construction; checked on the same runs).
  - Reported beside, not criteria: confident segments added to right
    plays, possible detections added, search time.
  Where a run gives the default's output only through the trace build's
  `default_final` stage (the default's suppression applied to the same
  refined list), that is said, and the trace is checked against a real
  default report wherever one exists.
- **(05:24) `check.sh` green** with the generator options and the
  variant (4:52; the variant's test, a 10 s ride from 1.0 to 1.02 in
  synthetic audio, keeps one confident side by default and two with the
  variant). Committed (`8899de4`) with the criteria above, before any
  variant run. Run build `work/bin/s37-run/` (its CLI binary differs
  from the start build's because the core's `Options` gained a field);
  step 3's queue starts with both regressions on it.
- **(05:40) Lost sides counted two ways.** At 0.5% over 2-5 s some sides
  lose most of their time while the kept side's widened end still reaches
  about 10 s into them, so 0166's count (a side with no confident
  detection at all) misses them. Both counts are reported from now on:
  no detection, and 30 s or more uncovered. `ride_stages.py` attributes
  each chain to the side it overlaps more (its first version took the
  other side's chain where it reached a few seconds in); on seeds 3521
  and 3522 every lost side, by either count, is dropped at the second
  suppression (0175 updated, `98cb1ab`).
- **(06:01) A queue fault, fixed.** Step 2b's wait (`tmux has-session -t
  s37step2`) matched its own session `s37step2b` by prefix and waited on
  itself; it was restarted at 06:01 without the wait, after step 2 had
  finished at 05:59. Step 3 had not started (its wait names `s37step2b`
  exactly enough). No run overlapped another.
- **(06:10) Regressions with the run build** (`s37-run`, the variant
  off): `regress session-32-linux` (1:11) and `regress
  session-33-linux-nas` (5:57) identical, every report identical to its
  baseline's apart from `wall_seconds`.
- **(06:10) Step 3's design.** Two of step 2's conditions, 1% over 10-20 s
  and 3% over 2-5 s (renders reused): the default again (for search
  time on cached renders), `--speed-per-stretch`, `--trim-ends`,
  `--single-pass`, `--extra-rungs 3`, and the variant (step 4's
  development run, after its criteria commit). The 2% conditions lose
  more but sit between these two in both counts; two conditions keep
  step 3 to about 25 minutes.
- **(07:12) Step 4 so far, against the criteria.** V1 on the test seeds:
  the variant keeps all 183 sides the default loses (87 + 96), 100% of
  their 12,609 s; by the partial count 223 of 225. V2 and V3: no wrong
  confident detection on the test seeds; the sweep at 262 tracks
  identical; at NAS scale 4 of 720 sweep queries gain a weak detection
  (6-15 hits), recall 540 and wrong answers 7 unchanged; the Stakka scan
  and its leave-outs gain Star Trails' slower side at both scales, wrong
  0 and identified unchanged, calibrate's false candidates at 30 hits or
  more 18 to 19 at 262 tracks (95 at NAS scale; strongest false
  unchanged); the 8 other development reports at NAS scale: wrong 0,
  right unchanged, confident additions only Into Music and Phantom Force
  (right).
- **(07:20) Step 4's generated mixes.** Short and long mixes of seed 2026
  were rendered again and searched with the default and the variant
  (`work/bin/s37-run`): short identical; long, wrong confident 0 -> 0,
  right 191 -> 191, 42 weak or possible detections added (one mix: a
  second possible segment of a version already possible there).
- **(07:21) Step 5 skipped, review started early.** Step 5 (a ride in
  `robust`) is left for a later session; steps 1-4 fill the time. The
  read-only review was started at 07:21 on the commits and the
  uncommitted documents while the shared run (the last step 4 run)
  finished, to save time; its pending figure was named to it.
- **(07:40) The review's findings** (a separate agent, read-only,
  07:21-07:36; it wrote one scratch file in `/tmp`, outside the
  repository). No code defect. Fixed: "from 1%, 21-43% of the sides" was
  false for 3% over 10-20 s (4%), now stated per condition; the notes
  said `--speed-per-stretch` changes nothing (it moves the counts by 1-5
  sides); the development mixes' rides were stated as fact (candidates);
  the sides per condition (248 for the drift); 0176's "up to 20 s" (19.5 s,
  from the traces); 0175's chain overlap is over 191 sides (the 192nd only
  touches); 0175's segment counts added to its data; directory paths
  removed from 0175's data; times and memory in the notes' table; "the
  same plays under every condition" (the files, mix times, speeds and
  ride starts are the same; the start in the track and a few directions
  follow the size); 0176's trace build named correctly; 0178 trimmed to
  40 lines; the variant's code comment (first-pass detections, trimmed
  with `--trim-ends`). Two corrections to this brief: at 05:10 the
  default draw's identity rests on the identical reports of 0175, not
  on a test; and the experiment files 0175 and 0177 were committed one
  minute after the queued runs of the next step had started (05:13 and
  06:44; the queues were chained), against AGENTS.md's order; their
  content used only the earlier step's data. `check.sh` was started at
  05:18, stopped and restarted at 05:19 for a code edit, and ran beside
  two renders (4:30 and 5:02 instead of 3:40).
- **(07:58) The shared run's result** (38:30, 622 queries). The trace's
  searches were split by stage order (477 with detections) and matched
  to the report's queries (603 exactly, 19 by file and hits). No query
  gains a confident file the default lacks; 56 confident detections are
  added, 52 of the played recording and 4 of Fire's Rogue Unit remix
  (role `other`, a wrong version the default already names confidently)
  in fire-6's two "beyond" queries, indexed and played left out. The
  count of wrong confident detections rises, so V2 fails by its letter;
  0178 says so. No default shared run was started for a direct
  comparison: it is past 07:45, and the trace gives the default's list
  from the same searches.
- **(08:11) Final checks** (`work/s37/final.sh`, build `work/bin/s37-final/`
  from `c684370`): `check.sh` green (3:21); both regressions identical
  (1:09; 5:56) and every report identical to its baseline's apart from
  `wall_seconds`; `docs/pair-verdicts.txt` SHA-256 equal to the start's.
