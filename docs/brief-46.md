# Session 46 brief: cheaper query analysis, exact

Written by the agent for itself on 2026-10-24, so the plan survives
context compaction. Work autonomously and never stop to ask: where a
decision is needed, take the conservative option, write it down here and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-46-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 41", "Session 43" and "Session
   45"), `docs/status.md` (newest entries), experiments 0097, 0109,
   0189-0193, 0199-0205 and this session's (0212 and up),
   `docs/research/2026-10-21-low-bandwidth-search.md`, the roadmap's
   speed bullet ("Where search time goes").
4. The code of `crates/gunfinger-core/src/spectrogram.rs`, `peaks.rs`,
   `speed.rs` and `search.rs` (`points_near`, `KeyLockPeaks`), and
   `search/lines.rs` (`in_blocks`, `in_windows`).

`docs/brief-2.md` to `docs/brief-45.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 46 of Gunfinger: cheaper query analysis, exact. After sessions 41 and 43 (experiments 0189-0205) the search's first pass on the Linux machine spends 20-24% of its CPU in query analysis (`Query::points_near` in `crates/gunfinger-core/src/search.rs`: per rung, an STFT of the query block at the rung's hop and peak picking, `Rung::points_in` in `speed.rs`, `peaks.rs`, `spectrogram.rs`; key-locked rungs share one analysis per rounded hop through `KeyLockPeaks`), second only to grouping (36-43%, where sessions 36, 41 and 43 found no further exact gain). Session 41 found that on the baseline x86-64 target `f64::floor` is a function call, not one instruction (no SSE4.1 `roundsd`), and that this was most of the old pre-filter's cost; the analysis has `floor`, `round` and `ceil` too. Session 19 (0109) found that FFT lengths with small prime factors make the turntable analysis 36.5% faster, but a length change changes peaks and output, so that stays a retrieval study and our decision. This session measures where the analysis's time goes and adopts what makes it cheaper while every detection stays bit for bit identical.

You run unattended: the user is away and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads; two core complexes of 8 cores, 32 MiB L3 each; 32 KiB L1d and 512 KiB L2 per core), 64 GB RAM, DDR4-3600; default jobs (32). `perf` is not installed and installing it needs root: use timers in a diagnostic build (`work/s41/timers.diff`, `work/s43/timers.diff`). Timing runs need an otherwise idle machine: run nothing else beside them, and repeat (A B B A or better).

Time box: started about 23:05; aim to finish by about 02:00 (the user's soft deadline; work in progress may run past it). If a step would run past that, scale it down (fewer candidates, skip the full clustering run), record the decision in the brief and continue.

Environment (sessions 33-45's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records), library `/mnt/atlas/Music/dnb` (read only), reached with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json` (SHA-256 `d9c85c4586ad...`, 284 clusters). Baselines `session-32-linux` and `session-33-linux-nas`. Build `work/bin/s45-final/` (master; since sessions 41 and 43 the search is about a third faster than in session 40: a NAS-scale development scan searches in about 26 s, `regress session-33-linux-nas` takes about 4 minutes). `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans, sweep reports of seeds other than 2026 (calibrate reads every sweep report there, session 40), and reports of opt-in variants must not sit in a reports directory during `regress`; run variants in a scratch work directory.
- Session 38's opt-in `--apart-in-speed` (confirmed by session 45, adoption ours) and session 37's `--overlap-by-chains` are variants, not defaults.
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them. This session needs no renders.
- The user is away for the evening and cannot listen to anything tonight. The pending listening (session 40's pack in `work/listening-s40/`, session 39's three questions, Star Trails and Into Music) stays for later; do not build new packs unless a step asks.
- Tonight the Linux machine is on a slow Wi-Fi link, not Ethernet: the NAS at `/mnt/atlas` is mounted but slow. The NAS peak store, the saved index, the corpus and the development mixes' audio are local and unaffected. Keep reads of NAS audio to what a step needs; never list or decode the whole NAS library; timings that compare builds must not include NAS reads.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule, no SIMD and no `unsafe`), docs/brief.md (never edit it), docs/brief-41.md, docs/brief-43.md and docs/brief-45.md, docs/session-notes.md ("Session 41", "Session 43" and "Session 45"), docs/status.md (newest entries), `docs/research/2026-10-21-low-bandwidth-search.md`, `docs/research/2026-10-14-matching-proposals.md` (the FFT-length proposal), ADR 0020, experiments 0097, 0103, 0109, 0117, 0119, 0189-0193 and 0199-0205, the roadmap's speed bullet, and the code of `search.rs` (`points_near`, `KeyLockPeaks`, blocks), `speed.rs` (`points_at_speed_in`, `points_at_tempo_in`, resampling if any), `peaks.rs`, `spectrogram.rs` and `profile.rs`. Session 41's and 43's scripts (`work/s41/ab.sh`, `ab.py`, `validate.sh`, `devscans.sh`, `corpusscans.sh`, `timers.sh`; `work/s43/`) are there to reuse.
2. Write docs/brief-46.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-46-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s46/listening-digests-start.txt; it must be the same at the end. Experiments number from 0212.
3. Build `work/bin/s46-start`; `regress session-32-linux` and `regress session-33-linux-nas` identical.

Steps:

1. Measure first, with a diagnostic build (never committed): within query analysis, per playback (turntable and key-locked), the time in windowing, the FFT, magnitudes, peak picking, peak refinement and the conversion to points, and how often an analysis is repeated (the same block and hop analysed again, block edges, rungs sharing a hop). Also, as a diagnostic only, the whole scan built for `x86-64-v2` (SSE4.1, no FMA) beside the baseline, with its reports compared byte for byte: it tells how much the rounding functions cost everywhere. Changing the release target is a platform decision (ADR 0020, ours) and is not adopted here. Write experiment 0212 before any candidate is timed.

2. Pre-registration, committed before any candidate is timed: at least three candidates with their exactness arguments and tests, for example: rounding written so it compiles to integer or comparison code on the baseline target with results equal to `floor`/`round` on every input the code meets (tests at halves, negatives, large values, NaN guards); analyses reused where the same block, hop and frames are computed again; buffers and plans kept per worker; magnitudes or the peak test compared without a square root or logarithm where the order is the same. Results must be bit for bit identical: no change of FFT length, window, hop or the order of floating-point operations. The timing protocol (the Stakka scan at NAS scale with the saved index, 32 and 16 jobs, A B B A at least, otherwise idle) and the stop point: kept only if it cuts the scan's search time at 32 workers by at least 3% against the start build in every pair of runs, with byte-identical reports apart from `wall_seconds`. Also `identify` of one development mix through the CLI with the corpus configuration (local; the NAS configuration lists the NAS library over tonight's slow link), since it analyses on both ladders.

3. The candidates, one at a time, each tested and timed; combine those that pass and time the combination.

4. Adoption of what passes, in master: its tests; the 11 development scans at NAS scale and at 262 tracks byte for byte apart from `wall_seconds`; `clusters --from-peaks --sample 48` byte-identical; `regress` at both scales identical. Indexing analyses files the same way: if an adopted change touches code `index` runs, show on a few corpus files decoded into a scratch store that the stored peaks are byte-identical (never write `work/peaks` or the NAS store).

5. Experiments (at most 40 lines each, written before the next starts): step 1's breakdown and the `x86-64-v2` diagnostic, each candidate against the stop point, and the adopted combination's times.

6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.

7. Wrap-up: a "Session 46" section at the end of docs/session-notes.md with a handoff (times before and after, what was kept and dropped, where the search's CPU goes now, and what the `x86-64-v2` diagnostic means for a platform decision); update docs/status.md, the checklist, the roadmap's speed bullet and the low-bandwidth note. Final checks: `scripts/check.sh` green, `regress session-32-linux` and `regress session-33-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: what was kept with its times, what was dropped and why, the diagnostic's result, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's behaviour does not change: no threshold, parameter, rule, default, output or release-target change. Exact speed changes to the core are what this session is for: they are adopted only with tests and byte-identical reports (apart from `wall_seconds`) at both scales, as sessions 36, 41 and 43 adopted theirs. Diagnostic and candidate builds are never committed unless adopted. Harness additions are tested and keep `regress` identical.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
- Scratch data goes under work/s46/.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 02:00. Start no new candidate or timing round after about 01:30; finish what is running, then do the review and wrap-up (a candidate that has not passed every validation step is not adopted), and list what is left in the handoff.
```

## State at the start of session 46

- 2026-10-24, 23:03. `master` at `867ac8d`, clean. Git identity `Jacek
  Galanciak <jacek.galanciak@gmail.com>`. No tmux server running; load
  average falling from an earlier session's runs (0.24 over the last
  minute at 23:05).
- `docs/pair-verdicts.txt`: SHA-256 `f4b1867956ba...`
  (`work/s46/listening-digests-start.txt`). The NAS clusters file:
  SHA-256 `d9c85c4586ad...` (`work/s46/clusters-digest-start.txt`).
- Experiments 0001-0211; this session numbers from 0212. Test set: 4 of
  5 evaluations used (none in this session).
- The start build `work/bin/s46-start/` is byte-identical to
  `s45-final` (cargo had nothing to rebuild).
- Query analysis today: each job of the first pass is one block of 12
  windows (120 s) on one turntable rung or one group of key-locked
  rungs with the same rounded hop. `for_each_frame` plans a real FFT
  (RealFFT 3.5.0 over RustFFT 6.4.1, whose AVX code is chosen at run
  time), makes the Hann window, and per frame windows the samples,
  transforms them, takes `10 log10(|X|^2 + 1e-20)` of every bin and
  hands the row to `PeakPicker`, which copies it into a ring of 25 rows,
  keeps the bins that beat their four nearest neighbours and the floor,
  tests each against its 25 x 25 neighbourhood, and refines and
  quantises the survivors (`f32::round`). Turntable rungs use an FFT of
  `round(1024 / s)` samples (948-1113), key-locked rungs 1,024 at a hop
  of `round(128 / s)`. 0109 measured the FFT as 54.7 s of 73.2 s of the
  turntable analysis in one `identify` (16 us of 18.7 at 1,113 samples,
  1.1 us of 3.7 at 1,024) and plans as 0.05 s of 128 s.
- On the baseline target the start build has no `roundss` or `roundsd`
  instruction; the `x86-64-v2` build has 205.

## Plan

| Step | What | Expected |
|---|--|---|
| First | reading; digests; brief; start build, both regressions and the start build's 11 development scans at both scales and clusters sample as references (tmux `s46start`) | 23:05-23:35 |
| 1 | diagnostic build (timers and counts within the analysis, by playback); the `x86-64-v2` build beside the start build (A B B A); experiment 0212 | 23:35-00:05 |
| 2 | pre-registration in this brief, committed | 00:05-00:15 |
| 3 | candidates, each tested and timed (A B B A, 32 and 16 jobs); their combination | 00:15-01:15 |
| 4 | adoption and validation | 01:15-01:45 |
| 5-7 | experiments, review, notes, final checks | 01:45-02:15 |

## Decisions

- **(23:06) Start build and references** (`work/s46/start.sh`, tmux
  `s46start`): the start build, both regressions, then the start
  build's 11 development scans at NAS scale with the clusters sample
  (scratch work directory `work/s46/w`: the saved index hard-linked,
  the clusters file and library map copied from `work/s43/w`, panels
  from a copy of `work/s33/panels` in `work/s46/panels`) and the 11 at
  corpus scale (`work/s46/wc`), kept in `work/s46/dev-start/` and
  `work/s46/corpus-start/` and each compared with session 43's start
  reports. Session 46's scripts are session 43's with the paths
  changed; `timing.sh` runs `identify` with the corpus configuration
  and `--playback both` (the configuration's playback is turntable),
  as the prompt asks.
- **(23:10) Step 1, the diagnostic build** (`work/s46/diag.diff` and
  `work/s46/diag.rs`, never committed; `work/bin/s46-diag/`): per
  playback (turntable, key-locked, other: the second pass), seconds in
  plan and window, windowing, FFT, log power, the row copy, the
  four-neighbour test, the neighbourhood test, refinement with the
  peak's push, and the conversion to points (turntable) or the
  key-locked filter; counts of analyses, frames, frames picked,
  candidates, neighbourhood tests and rows read, peaks, and key-locked
  analyses reused or made. Timers add about 5 `Instant::now` calls a
  frame, so the shares are read as shares, not as times of the start
  build.
- **(23:11) The `x86-64-v2` diagnostic build**: master's tree with
  `RUSTFLAGS="-C target-cpu=x86-64-v2"` (`work/bin/s46-v2/`, target
  directory `work/s46/target-v2`); timed against the start build with
  `ab.sh` (A B B A at 32 and 16 jobs), its reports compared byte for
  byte apart from `wall_seconds`.
- **(23:28) Step 1 done** (experiment 0212). `diag-32` ran beside a
  compile of C1's tests; it was repeated idle as `diag-32b` (the same
  shares), which 0212 uses. The `x86-64-v2` build is not faster (0.8-1.9%
  slower at 32 jobs), with byte-identical reports.
- **(23:30) Step 2, the pre-registration** (before any candidate is
  timed; C1-C3 were written and passed the core crate's tests in
  scratch worktrees under `work/s46/`, and C2's binaries were built,
  before this entry, none of them timed). From 0212: at 32 workers the
  first pass's analysis splits FFT 37-38%, log power 27-30%, the
  four-neighbour test 18-19%, the neighbourhood test 12% (timers
  included), windowing 2%, the row copy 0.6%, plans and windows 0.05%;
  17.6 candidates and 0.61 peaks a frame.
  - **C1, rounding without a library call in the analysis.**
    `quantise` (three `f32::round` a peak) through `rounded`: values
    under 2^22 in magnitude truncated to `i32` and back (exact), the
    fraction compared with one half, the sign copied so that values
    rounding to zero keep it; larger values, infinities and NaN go to
    `round`. The STFT frame of a peak (the key-locked filter, the
    stored peaks' search) as `(frame + 0.5) as usize`: a refined frame
    is a multiple of 1/64 within 31/64 of its STFT frame, so the sum is
    exact and truncating it rounds. Test: `rounded` equals `round` bit
    for bit on the halves, negatives, signed zeros, 2^22 and 2^23
    neighbours, large values, infinities, every 1/128 in +-1,000 and a
    million random bit patterns; NaN gives NaN. Expected small (about
    two calls a frame); timed because the prompt names it.
  - **C2, the peak test on powers.** The STFT gives each bin's power
    (`|X|^2 + 1e-20`, the same operations); `decibels(p) = 10 log10 p`
    is applied only where a level is needed. Exactness: if `decibels`
    never decreases as power grows (monotone), then a power no greater
    than another has no greater level, the loudest power of a set has
    the loudest level, and a power below the lowest power whose level
    reaches the floor (`floor_power`, found by halving bit patterns) is
    exactly a level below it. So the four-neighbour test on powers
    rejects only bins the test on levels rejects; each bin it passes
    (17.6 a frame) is tested again on levels against the loudest
    neighbour; the neighbourhood test stops at any point with at least
    the power and otherwise compares the level of the loudest other
    point with the bin's; refinement and magnitudes use the levels of
    the same powers. NaN: `max` passes over NaN as the comparisons on
    levels did, and a NaN bin passes both tests in both forms. Tests:
    every non-negative `f32` (infinity included) on the machine
    running the tests has a level no lower than the one before
    (`decibels_never_decrease_as_power_grows`; monotonicity is the
    platform maths library's, so the test runs everywhere); a tie in
    decibels between different powers, as a nearest neighbour and
    farther away, gives no peak; `floor_power` is the boundary; the
    existing tests against the definition on levels. About 38
    logarithms a frame instead of 513-557.
  - **C3, the four-neighbour test as one loop over equal slices.** The
    same predicate per bin written into a `bool` per bin over slices of
    the same length (no running count or bounds checks, so the compiler
    can test several bins at once), then the passing bins gathered in
    bin order as before. Exact: the same comparisons per bin, the same
    candidates in the same order. Tests: the existing ones against the
    definition and on stretches.
  - **Not candidates**, from 0212: per-worker plans, windows and
    buffers (0.05% of the analysis); reusing analyses: the edges of
    blocks repeat about 1% of frames, and the one analysis two
    playbacks repeat per block (turntable at 1.0 and key-locked at hop
    128) is 1 of 62, at most about 0.4% of the search; the row copy
    (0.6% of the analysis). None can reach 3%.
  - **Timing protocol and stop point.** One round with `ab.sh`: the
    start build, C1, C2, C3, C3, C2, C1, the start build, at 32 then 16
    jobs, the machine otherwise idle; each report compared with the
    start build's reference apart from `wall_seconds` (a different
    report drops the candidate). A candidate passes if it cuts the
    scan's search time (`wall_seconds`) at 32 workers by at least 3%
    against the start build in both pairs. The combination of those
    that pass (C2 with C3 if both pass) is timed in a second round
    (start, combination, combination, start, twice, at 32 and 16 jobs)
    and must pass the same rule; if it does not, the single candidate
    with the larger smallest cut is adopted if it passed. The adopted
    build is also timed with `identify` of the Stakka mix through the
    CLI with the corpus configuration and `--playback both`
    (`timing.sh`: warm-up, then A B B A), whose JSON reports must be
    byte-identical.
- **(23:36) Pre-registration, addendum: C2b**, written while round 1
  ran and registered before it is timed. C2 folds each row of the
  neighbourhood with `max` to keep the loudest power, which the
  compiler may not vectorise as it does the old fold of comparisons.
  C2b is C2 with the neighbourhood test's early exits on the old fold
  of comparisons (`other >= value` on powers), and the loudest other
  point found by a second scan of the neighbourhood only for bins that
  pass it (0.61 peaks a frame and the rare ties). Same exactness
  argument and tests as C2. It is timed in a round of its own (start,
  C2b, C2b, start at 32 and 16 jobs) against the same stop point. If
  C2 and C2b both pass, the combination with C3 uses the one with the
  larger smallest cut at 32 workers.
