# Session 41 brief: a cheaper pre-filter, exact

Written by the agent for itself on 2026-10-23, so the plan survives
context compaction. Work autonomously and never stop to ask: where a
decision is needed, take the conservative option, write it down here and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-41-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 36" and "Session 40"),
   `docs/status.md` (newest entries), experiments 0096, 0103, 0108,
   0113, 0167-0174 and this session's (0189 and up),
   `docs/research/2026-10-21-low-bandwidth-search.md`, proposal 1 of
   `docs/research/2026-10-14-matching-proposals.md`, the roadmap's speed
   bullet ("Where search time goes").
4. The code of `crates/gunfinger-core/src/search/lines.rs` (all of it)
   and `search.rs`.

`docs/brief-2.md` to `docs/brief-40.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 41 of Gunfinger: a cheaper pre-filter, exact. Since session 36 (experiment 0170) the pre-filter (`OffsetSlots::may_form_line` in `crates/gunfinger-core/src/search/lines.rs`) is the largest stage of the search on the Linux machine: 43% of the search's CPU at 16 workers and 46% at 32, about 5.8 ns of CPU per hit over 49.8 G hits in the Stakka scan at NAS scale, where 1.12 G assets with 3 hits or more are filtered and 2.2% pass. Grouping a part (`group_part`) is 24-28%. Narrower hits (0171), grouping straight from the lists (0172) and reading repeated hashes once (0173) were measured and dropped; integer phases for the slots were 13% cheaper on the Mac in session 19 (0108), below that note's 20% stop point, and were never tried here. This session designs, measures and, where it pays, adopts changes to the pre-filter (and, second, to grouping) that keep every detection bit for bit identical. A faster search here also shortens every later session.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads; two core complexes of 8 cores, each with a 32 MiB L3; 32 KiB L1d and 512 KiB L2 per core), 64 GB RAM, DDR4-3600; default jobs (32). `perf` is not installed and installing it needs root: use timers in a diagnostic build, as 0170 did (`work/s36/timers.diff`). Timing runs need an otherwise idle machine: run nothing else beside them, and repeat (A B B A or better).

Time box: started about 03:30; aim to finish by about 07:30, because another session may follow you. If a step would run past that, scale it down (fewer candidates, skip grouping), record the decision in the brief and continue.

Environment (sessions 33-40's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records), library `/mnt/atlas/Music/dnb` (read only), reached with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: since the listening of 2026-10-22 (docs/status.md, "after session 38") SHA-256 `d9c85c4586ad...`, 284 clusters, Dilusion's upload joined to its cluster; `regress session-33-linux-nas` is identical with it (5:56). Baselines `session-32-linux` and `session-33-linux-nas`. Build `work/bin/s40-final/` (master). `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- Session 38's opt-in `--apart-in-speed` and session 37's `--overlap-by-chains` are variants, not defaults; their adoption waits for an ADR (ours).
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans, and sweep reports of seeds other than 2026 (calibrate reads every sweep report there, session 40), must not sit in a reports directory during `regress`.
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them. This session needs no renders.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule, no SIMD and no `unsafe`), docs/brief.md (never edit it), docs/brief-40.md, docs/session-notes.md ("Session 36" and "Session 40"), docs/status.md (newest entries), `docs/research/2026-10-21-low-bandwidth-search.md`, `docs/research/2026-10-14-matching-proposals.md` (proposal 1), experiments 0096, 0103, 0108, 0113 and 0167-0174, the roadmap's speed bullet ("Where search time goes" and what follows it), and the code of `search/lines.rs` (all of it: `take_part`, `group_part`, `front_partition`, `OffsetSlots`, `densest_clusters`, `distinct`), `search.rs` and `query.rs`'s points. Session 36's scripts in `work/s36/` (timers, A/B) and `work/perf-design/` are there to reuse.
2. Write docs/brief-41.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-41-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s41/listening-digests-start.txt; it must be the same at the end. Experiments number from 0189.
3. Build `work/bin/s41-start`; `regress session-32-linux` and `regress session-33-linux-nas` identical.

Steps:

1. Measure first, with a diagnostic build (never committed): within the pre-filter, what the time per hit goes to (the slot arithmetic, the loads of the grouped hit and its anchor's point, the counting, the `touched` bookkeeping and clearing, the early exit), and the distribution of hits per filtered asset and of where in its hits a passing asset passes. Write experiment 0189 before any candidate is timed.

2. Pre-registration, committed before any candidate is timed: the candidates (at least three, from step 1 and the proposals; for example clearing by generation stamps instead of a `touched` list, integer slots from per-point phases as 0108 did, counting in the order hits are scattered so the filter needs no second read, a cheaper first test that rejects most assets before the exact one, or a tighter grouped-hit layout for the filter's reads), each with its exactness argument and the tests that check it against today's `may_form_line` and `densest_clusters` (random sets, negative offsets, bucket edges, the slot wrap, equal offsets, float edges); the timing protocol (the Stakka scan at NAS scale, `scan stakka-skynet-knowledge` with the saved index, 32 and 16 jobs, A B B A at least, the machine otherwise idle); and the stop point: a candidate is kept only if it cuts the scan's search time at 32 workers by at least 3% against the start build in every pair of runs, with byte-identical reports apart from `wall_seconds`.

3. The candidates, one at a time, each tested (`scripts/check.sh` green) and timed; combine the ones that pass and time the combination. Then, if time allows, the same for grouping (`group_part`), with the same stop point.

4. Adoption of what passes, in master: its tests; the three development scans and their leave-outs at NAS scale at 32 workers, byte for byte apart from `wall_seconds`; the 262-track scans; `clusters --from-peaks --sample 48` (the sample report byte-identical); `regress` at both scales identical. A full NAS clustering run (about 23 minutes) only into a scratch copy, never replacing the clusters file in use, and only if time allows.

5. Experiments (at most 40 lines each, written before the next starts): step 1's breakdown, each candidate's timing against the stop point, and the adopted combination's times (scan search at 16 and 32 workers, CPU, the clusters sample, `identify` of one development mix through the CLI).

6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.

7. Wrap-up: a "Session 41" section at the end of docs/session-notes.md with a handoff (the times before and after, what was kept and dropped, what is left of the search's CPU and where); update docs/status.md, the checklist, the roadmap's speed bullet and the low-bandwidth note. Final checks: `scripts/check.sh` green, `regress session-32-linux` and `regress session-33-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: what was kept with its times, what was dropped and why, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's behaviour does not change: no threshold, parameter, rule, default or output change. Exact speed changes to the core are what this session is for: they are adopted only with tests and byte-identical reports (apart from `wall_seconds`) at both scales, as session 36 adopted 0169. Diagnostic and candidate builds are never committed unless adopted. Harness additions are tested and keep `regress` identical.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user runs among them).
- Scratch data goes under work/s41/ (your session number).
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00. Start no new run after about 10:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 41

- 2026-10-23, 03:28. `master` at `1b0db5f`, clean. Git identity `Jacek
  Galanciak <jacek.galanciak@gmail.com>`. No tmux server running; load
  average 0.15.
- `docs/pair-verdicts.txt`: SHA-256 `f4b1867956ba...`
  (`work/s41/listening-digests-start.txt`). The NAS clusters file:
  SHA-256 `d9c85c4586ad...` (`work/s41/clusters-digest-start.txt`).
- Experiments 0001-0188; this session numbers from 0189. Test set: 4 of
  5 evaluations used (none in this session).
- The start build `work/bin/s41-start/` is byte-identical to
  `s40-final` (cargo had nothing to rebuild).
- The pre-filter today (`OffsetSlots::may_form_line`): for each grouped
  hit, `offset = frame - points[anchor].frame` in `f64`, slot
  `floor(offset / 4) mod 4096`, a count per slot (`u8`), a pass when a
  slot and its larger neighbour hold 3, each counted slot pushed to
  `touched` and cleared after the asset. On the baseline x86-64 target
  (SSE2, no SSE4.1 `roundsd`) `f64::floor` compiles to an indirect call
  (`callq *floor@GOTPCREL(%rip)`, checked on a two-line program with
  rustc 1.98.0 `-O`), followed by a saturating `f64` to `i64` cast; on
  the Mac (AArch64) `floor` is one instruction (`frintm`). This is one
  reason the per-hit cost may differ between the two machines (an
  inference to be measured in step 1).
- A posting is a `u32`: the asset in the high 16 bits, the frame in the
  low 16 (`index.rs`).

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build, both regressions and the start build's 11 development scans and clusters sample as references (tmux `s41start`) | 03:28-04:10 |
| 1 | diagnostic build (timers, variant passes, counts); experiment 0189 | 04:10-04:50 |
| 2 | pre-registration in this brief, committed | 04:50-05:05 |
| 3 | candidates, each tested and timed (A B B A, 32 and 16 jobs); their combination; grouping if time allows | 05:05-06:40 |
| 4 | adoption and validation | 06:40-07:10 |
| 5-7 | experiments, review, notes, final checks | 07:10-07:40 |

## Decisions

- **(03:33) Start build and checks** (`work/s41/start.sh`, tmux
  `s41start`): the start build, both regressions, then the start build's
  11 development scans at NAS scale and its clusters sample in the
  scratch work directory `work/s41/w` (the saved index hard-linked, the
  current clusters file and library map copied, panels from a copy of
  `work/s33/panels`), kept in `work/s41/dev-start/` as the references
  for every later build (`work/s41/devscans.sh`). Session 36's
  references were made with the clusters file before the listening of
  2026-10-22, so they are not reused.
- **(03:51) Start checks.** `regress session-32-linux` (1:09) and
  `regress session-33-linux-nas` (6:06) identical with
  `work/bin/s41-start/`, every report identical to its baseline's apart
  from `wall_seconds`. The 11 development scans and the clusters sample
  ran beside compiles (not timing runs).
- **(03:52) Step 1, the diagnostic build** (`work/s41/diag.diff`, never
  committed): per part, after grouping, seven passes over the part's
  assets with 3 hits or more, each timed, in an order rotating by part
  (hit loads alone; with the anchor's point and the offset; with the
  slot; today's filter; today's filter without the early exit; today's
  filter clearing its 4 KiB of counts with one fill instead of the
  `touched` list; the fill alone), then the counts (hits per filtered
  asset, the hit at which a passing asset passes). The real filter and
  everything after it run unchanged, so the reports stay identical. The
  Stakka scan at NAS scale at 32 then 16 jobs (`work/s41/diag.sh`,
  summary `work/s41/diag.py`).
- **(03:56) Step 1 result** (experiment 0189): of today's filter at 16
  (32) jobs, loads 14% (15%), the slot arithmetic with its `floor` call
  37% (46%), the `touched` list and clearing 27% (16%), counting and the
  test 23% (24%); 44 hits a filtered asset on average, the early exit
  sparing 1.4% of them.
- **(03:57) Step 2, the pre-registration** (committed before any
  candidate is timed; the candidates' code and tests were written
  during the start runs and step 1, and have only been tested).

  *The candidates.* Each keeps `densest_clusters`, sorting, `line_of`,
  `distinct` and everything after them unchanged; sources in
  `work/s41/lines.rs.<name>`, builds in `work/bin/s41-<name>/`.

  - **C1, integer slots from per-point phases** (proposal 1, 0108). Once
    per block and rung, each point's phase `floor(frame) mod 2^16`
    (`u16`); a hit's slot is `((frame - phase) mod 2^16) / 4 mod 4096`
    in integers. No `floor` call, `f64` subtraction or cast per hit; the
    filter loads 2 bytes per anchor instead of the 16-byte point.
    Exactness: the integer offset exceeds the offset by less than one
    frame; a cluster's computed offsets differ by at most 2, so with
    every point within 2^31 frames of frame 0 (computed offsets then err
    by less than 2^-20 frames) the integer offsets differ by at most 3
    and lie in one bucket or two neighbouring ones; folding modulo 2^16
    folds whole buckets (2^16 = 4 x 4,096 x 4). When a point lies
    farther out, no asset is filtered (all are sorted). The filter may
    pass other assets without a line than today's; those are sorted and
    give no line, so lines are unchanged.
  - **C2, integer offsets in the grouped hit** (a tighter layout for the
    filter's reads). As C1, but `take_part` stores each hit's integer
    offset in its posting's 16 frame bits (the phase loaded once per
    posting list), so the filter reads only the grouped hit, 4 of its 8
    bytes; the hits of the 2% of assets that pass get their frames back
    by adding the phase (exact modulo 2^16). Exactness: C1's, and the
    inverse is exact.
  - **C3, generation stamps instead of the `touched` list.** Slots as
    `u16`, an 8-bit stamp of the asset in the high byte and its count in
    the low byte; a slot with another stamp counts zero; all slots are
    cleared once every 255 assets. Exactness: the same slots and counts
    as today, so the same pass or fail for every asset.
  - **C4, C2 with clearing by re-reading the hits.** No `touched`
    list: after the test, the counted hits' slots (cheap with C2) are
    computed again and zeroed. Exactness: C2's.
  - **C5, one fill instead of the `touched` list** (from 0189's pass).
    After the test, all 4,096 counts are zeroed with one fill (4 KiB)
    instead of zeroing the touched slots. Exactness: the same slots and
    counts as today.
  - **G1 (grouping), counting the part's assets while taking it.**
    `take_part` adds each taken posting to its asset's counter, from the
    short list run it has just read, and `group_part` drops its first
    pass over the part. Exactness: the same counts, so the same grouped
    order.

  *Tests* (each candidate: the core crate's tests green before it is
  timed; `scripts/check.sh`, with fmt and clippy, for the code adopted,
  a scaling down under the time box): the existing grouping test against a comparison sort (four
  windows, three parts, an empty middle part, every hit on the last
  asset; mixed hits on anchors a quarter frame apart); the blocks
  test; and for C1, C2 and C4 the filter tests rewritten on hits with
  reference and anchor frames: no line from scattered hits; a line
  wherever its offsets fall (bucket edges, negative offsets, the wrap at
  bucket -1 and 4,096, equal offsets, a span of exactly 2 frames,
  anchors with and without a fraction); the cluster of offsets 4, 3 and
  2 - 2^-52; 200,000 random sets (reference frames over the whole
  16-bit range, anchors at rung speeds 1, 1.0123 and 0.9375) where
  every set with a cluster passes, and today's `f64` filter (kept in
  the tests) agrees on at least 90% of sets; the phases themselves
  (fractions, negative frames, beyond 2^16, the 2^31 guard, NaN); and a
  window with a point beyond 2^31 frames giving the comparison sort's
  lines. For C3 and C5 the existing filter tests run unchanged (the random
test wraps C3's stamp hundreds of times); before adoption they also
compare every random set's pass or fail with today's filter.

  *Timing protocol.* `work/s41/ab.sh`: `gunfinger-eval --other-peaks-dir
  <NAS store> --work work/s41/w --jobs N scan stakka-skynet-knowledge`
  (saved index), the search time being the report's `wall_seconds`; for
  N = 32 then 16, the start build, each candidate, the candidates in
  reverse order, the start build (A B ... B A); nothing else running.
  Every report is compared with the start build's
  (`work/s41/dev-start/`) apart from `wall_seconds`.

  *Stop point.* A candidate is kept only if its search time at 32
  workers is at least 3% below the start build's in every pair (the
  forward half's runs and the reversed half's), with byte-identical
  reports apart from `wall_seconds`. 16 workers are reported, not
  decided on. The candidates that pass are combined (C2 or C4 replaces
  C1, which it contains; of the clearings C3, C4 and C5 only one, the
  fastest that passes) and the
  combination timed against the start build the same way; it is
  adopted if it passes the same stop point. A second round is run if a
  candidate's cuts straddle 3%.
- **(04:20) Step 3, round 1** (experiment 0190): C1-C5 pass the stop
  point at 32 workers in both pairs (C4 by 27.8-28.0%, the least C3 by
  4.6%); G1 is slower and dropped. By the pre-registered rule the
  combination is C4. Decided now, before timing: an exploratory round
  (0191) times C2 with one fill (`s41-c2fill`) and C2 with stamps
  (`s41-c2stamp`) against the start build and C4; one of them replaces
  C4 only if it is 3% faster than C4 in every pair at 32 workers.
  Otherwise C4 is adopted. The round also repeats C4 against the start
  build.
- **(04:20) Script fix.** `work/s41/ab.sh` set the shell variable
  `round` inside its run function, which garbled round 1's half labels;
  runs pair by position in `ab.py`, and the function now uses `half`.
- **(04:30) Round 2** (experiment 0191): on integer slots one fill is
  0.7-0.9% faster than C4 at 32 workers and slower at 16, stamps slower
  than both; neither meets the 3% rule, so C4 is adopted. Step 4 starts
  with its source tidied (`work/s41/lines.rs.final`: `Hit::taken`,
  `frame` and `slot`; a test-only `Found` for hits as found in a list;
  a test of the round trip and the slot), then `scripts/check.sh`, and
  the validation queue `work/s41/validate.sh`.
- **(04:30) The 262-track scans** are read as the 11 development scans
  at corpus scale (the corpus store, no `--other-peaks-dir`), in the
  scratch work directory `work/s41/wc` (corpus clusters copied), with
  the start build's reports as references (`work/s41/corpusscans.sh`).
  Grouping (G1) failed its stop point; no other grouping candidate is
  tried, since the session's time box favours the adoption checks.
- **(04:51) Step 4, validation of the adopted build** `work/bin/s41-adopt/`
  (`work/s41/validate.sh`): `scripts/check.sh` green (4:00, after one
  clippy fix: `is_multiple_of` in a const assertion); `regress
  session-32-linux` (1:09) and `regress session-33-linux-nas` (4:12
  instead of 6:06) identical, each report identical to its baseline's
  apart from `wall_seconds`; the 11 development scans at NAS scale, the
  11 at corpus scale and the clusters sample (1:22 instead of 2:00)
  identical to the start build's. Committed to master; then the timing
  runs (`work/s41/timing.sh`) and the full NAS clustering into a scratch
  copy (`work/s41/full.sh`).
