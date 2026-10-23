# Session 43 brief: a cheaper grouping step, exact

Written by the agent for itself on 2026-10-23, so the plan survives
context compaction. Work autonomously and never stop to ask: where a
decision is needed, take the conservative option, write it down here and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-43-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 41" and "Session 42"),
   `docs/status.md` (newest entries), experiments 0096, 0103, 0113,
   0167-0174, 0189-0193 and this session's (0199 and up),
   `docs/research/2026-10-21-low-bandwidth-search.md`, the roadmap's
   speed bullet ("Where search time goes").
4. The code of `crates/gunfinger-core/src/search/lines.rs` (all of it).

`docs/brief-2.md` to `docs/brief-42.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 43 of Gunfinger: a cheaper grouping step, exact. Since session 41 (experiments 0189-0193) the pre-filter counts integer offsets and the search is 28-30% shorter at NAS scale; grouping a part by asset (`group_part` in `crates/gunfinger-core/src/search/lines.rs`: a count per asset, a prefix sum, then a scatter of every hit into `grouped`) is now the largest stage: 40% of the first pass's CPU at 16 workers and 35% at 32 (0193), then the pre-filter and query analysis (20-22% each), taking parts (10-11%) and sorting with lines (9%). Narrower hits (0171), grouping straight from the lists (0172) and counting assets while taking a part (G1, 0190) were slower, so this needs other ideas. This session designs, measures and, where it pays, adopts exact changes to grouping (second, to taking parts or the pre-filter's remaining loads), with every detection bit for bit identical.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads; two core complexes of 8 cores, each with a 32 MiB L3; 32 KiB L1d and 512 KiB L2 per core), 64 GB RAM, DDR4-3600; default jobs (32). `perf` is not installed and installing it needs root: use timers in a diagnostic build (`work/s41/timers.diff`, `diag.diff`). Timing runs need an otherwise idle machine: run nothing else beside them, and repeat (A B B A or better).

Time box: started about 07:35; aim to finish by about 11:00 (the user's soft deadline; work in progress may run past it). If a step would run past that, scale it down (fewer candidates, skip the full clustering run), record the decision in the brief and continue.

Environment (sessions 33-42's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records), library `/mnt/atlas/Music/dnb` (read only), reached with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: since the listening of 2026-10-22 (docs/status.md, "after session 38") SHA-256 `d9c85c4586ad...`, 284 clusters, Dilusion's upload joined to its cluster. Baselines `session-32-linux` and `session-33-linux-nas`. Build `work/bin/s42-final/` (master; since session 41 the search is about 30% faster: a NAS-scale development scan searches in about 27 s, `regress session-33-linux-nas` takes about 4:12). `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- Session 38's opt-in `--apart-in-speed` and session 37's `--overlap-by-chains` are variants, not defaults; their adoption waits for an ADR (ours).
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans, and sweep reports of seeds other than 2026 (calibrate reads every sweep report there, session 40), must not sit in a reports directory during `regress`.
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them. This session needs no renders.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule, no SIMD and no `unsafe`), docs/brief.md (never edit it), docs/brief-41.md and docs/brief-42.md, docs/session-notes.md ("Session 41" and "Session 42"), docs/status.md (newest entries), `docs/research/2026-10-21-low-bandwidth-search.md`, experiments 0096, 0103, 0113, 0167-0174 and 0189-0193, the roadmap's speed bullet, and the code of `search/lines.rs` (all of it). Session 41's scripts are in `work/s41/` (`ab.sh`, `ab.py`, `validate.sh`, `devscans.sh`, `corpusscans.sh`, `timing.sh`, `full.sh`, `timers.sh`; the start reports in `work/s41/dev-start/` and `corpus-start/` are from the build before 0192, whose reports are identical to master's apart from `wall_seconds`).
2. Write docs/brief-43.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-43-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s43/listening-digests-start.txt; it must be the same at the end. Experiments number from 0199.
3. Build `work/bin/s43-start`; `regress session-32-linux` and `regress session-33-linux-nas` identical.

Steps:

1. Measure first, with a diagnostic build (never committed): within grouping, the count pass, the prefix sum and the scatter apart; the distribution of hits per part and per asset; how many distinct cache lines the scatter writes per part; and the same split at parts of 512, 1,024 and 2,048 assets now that the filter is cheaper. Write experiment 0199 before any candidate is timed.

2. Pre-registration, committed before any candidate is timed: at least three candidates with their exactness arguments and tests (the grouped order within an asset must stay the order the hits are taken in, or an argument and tests that every later stage gives the same result in another order). Candidates to consider, not a limit: a scatter through small per-asset write buffers flushed a cache line at a time (the usual way to make a radix sort's scatter cheap); counts kept in a narrower type or folded into taking a part where 0190's G1 was not; a different part size now that the filter costs less; filtering per asset straight from the counts where a count proves no line can form; fusing the pre-filter's counting into the scatter for assets whose hits are already contiguous. The timing protocol (the Stakka scan at NAS scale with the saved index, 32 and 16 jobs, A B B A at least, otherwise idle) and the stop point: kept only if it cuts the scan's search time at 32 workers by at least 3% against the start build in every pair of runs, with byte-identical reports apart from `wall_seconds`.

3. The candidates, one at a time, each tested and timed; combine those that pass and time the combination.

4. Adoption of what passes, in master: its tests; the 11 development scans at NAS scale and at 262 tracks byte for byte apart from `wall_seconds`; `clusters --from-peaks --sample 48` byte-identical; `regress` at both scales identical; a full NAS clustering into a scratch copy only (never replacing the clusters file in use), if time allows.

5. Experiments (at most 40 lines each, written before the next starts): step 1's breakdown, each candidate against the stop point, and the adopted combination's times (scan search at 16 and 32 workers, CPU, the clusters sample, `identify` of the Stakka mix through the CLI).

6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.

7. Wrap-up: a "Session 43" section at the end of docs/session-notes.md with a handoff (times before and after, what was kept and dropped, where the search's CPU goes now); update docs/status.md, the checklist, the roadmap's speed bullet and the low-bandwidth note. Final checks: `scripts/check.sh` green, `regress session-32-linux` and `regress session-33-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: what was kept with its times, what was dropped and why, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's behaviour does not change: no threshold, parameter, rule, default or output change. Exact speed changes to the core are what this session is for: they are adopted only with tests and byte-identical reports (apart from `wall_seconds`) at both scales, as sessions 36 and 41 adopted theirs. Diagnostic and candidate builds are never committed unless adopted. Harness additions are tested and keep `regress` identical.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user runs among them).
- Scratch data goes under work/s43/ (your session number).
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00. Start no new candidate or timing round after about 10:30; finish what is running, then do the review and wrap-up (a candidate that has not passed every validation step is not adopted), and list what is left in the handoff.
```

## State at the start of session 43

- 2026-10-23, 07:33. `master` at `0137d16`, clean. Git identity `Jacek
  Galanciak <jacek.galanciak@gmail.com>`. No tmux server running; load
  average 2.6 (falling from an earlier session's runs).
- `docs/pair-verdicts.txt`: SHA-256 `f4b1867956ba...`
  (`work/s43/listening-digests-start.txt`). The NAS clusters file:
  SHA-256 `d9c85c4586ad...` (`work/s43/clusters-digest-start.txt`).
- Experiments 0001-0198; this session numbers from 0199. Test set: 4 of
  5 evaluations used (none in this session).
- The start build `work/bin/s43-start/` is byte-identical to
  `s42-final` (cargo had nothing to rebuild).
- Grouping today (`WindowHits::lines`, `take_part`, `group_part`): per
  window and rung, parts of 1,024 assets; `take_part` copies each
  list's run of the part into `part` as 8-byte `Hit`s (the posting with
  its integer offset in the frame bits, and the anchor), list by list;
  `group_part` counts the part's hits per asset into `ends` (1,025
  `u32`), turns the counts into ends by a prefix sum, and scatters every
  hit into `grouped` (to one of 1,024 places), so that each asset's hits
  keep the order they were taken in. The per-asset loop then reads each
  asset's slice: assets with 3 hits or more are filtered
  (`may_form_line`), the 2% that pass are aligned, sorted and turned
  into lines.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build, both regressions and the start build's 11 development scans at both scales and clusters sample as references (tmux `s43start`) | 07:33-08:05 |
| 1 | diagnostic build (stage timers within grouping, counts); parts of 512, 1,024, 2,048; experiment 0199 | 08:05-08:45 |
| 2 | pre-registration in this brief, committed | 08:45-09:00 |
| 3 | candidates, each tested and timed (A B B A, 32 and 16 jobs); their combination | 09:00-10:00 |
| 4 | adoption and validation | 10:00-10:30 |
| 5-7 | experiments, review, notes, final checks | 10:30-11:00 |

## Decisions

- **(07:34) Start build and references** (`work/s43/start.sh`, tmux
  `s43start`): the start build, both regressions, then the start
  build's 11 development scans at NAS scale with the clusters sample
  (scratch work directory `work/s43/w`: the saved index hard-linked,
  the clusters files and library map copied, panels from a copy of
  `work/s33/panels` in `work/s43/panels`) and the 11 at corpus scale
  (`work/s43/wc`), kept in `work/s43/dev-start/` and
  `work/s43/corpus-start/` as the references for every later build,
  and each compared with session 41's start reports. Session 43's
  scripts are session 41's with the paths changed.
- **(07:49) Start checks.** `regress session-32-linux` (1:08) and
  `regress session-33-linux-nas` (4:20) identical with
  `work/bin/s43-start/`, each report identical to its baseline's apart
  from `wall_seconds`; the 11 development scans at both scales and the
  clusters sample identical to session 41's start reports.
- **(07:49) Step 1, the diagnostic build** (`work/s43/diag.diff`, never
  committed): per part, timers around taking, the count pass, the
  prefix sum, the scatter and the rest; counts from `ends` (hits per
  part and asset, list runs, cache lines written, hits beyond rooms of
  32-128). Builds at parts of 256, 512, 1,024 and 2,048 assets; the
  Stakka scan at NAS scale at 32 then 16 jobs (`work/s43/diag.sh`,
  summary `work/s43/diag.py`). Parts of 256 were added to the three the
  prompt names, as the smallest that keeps each part's places in L1.
- **(07:56) Step 1 result** (experiment 0199): the scatter is 4.0-4.4 ns
  a hit at 32 workers at every part size, 27.6% of the first pass at
  1,024 assets (34.0% at 16 jobs); the count pass 7%, the prefix sum
  0.1%. 1,024 assets has the lowest first-pass CPU at both job counts.
- **(08:00) Step 2, the pre-registration** (committed before any
  candidate is timed; the candidates' code and tests were written
  during the start runs and step 1, and have not been timed).

  *The candidates.* Each changes only how a part's hits are taken and
  grouped; `may_form_line`, aligning, sorting, `densest_clusters`,
  `line_of`, `distinct` and everything after them are unchanged.
  Sources `work/s43/lines.rs.<name>` (generators in `work/s43/py/`),
  builds `work/bin/s43-<name>/`.

  - **C1, rooms: hits taken straight into a room per asset.** Each asset
    of a part has room for 120 hits (`ROOM_HITS`) in a fixed array
    (`Rooms`); `take_part` reads each list's run and puts each hit in
    its asset's room after the hits taken before it, and the hits beyond
    a room go to an overflow vector in the order they are taken. No
    `part` buffer, no count pass over the part, no prefix sum, no
    scatter: each hit is written once instead of twice and read once (by
    the filter) instead of three times (0199: taking, count pass and
    scatter). The overflow (with 120, between the 2.6% of hits a room of
    96 and the 1.1% a room of 128 leave, 0199; 120 hits are 15 cache
    lines, so rooms do not start 1 KiB apart) is grouped by today's
    counting sort; an asset with more hits than its room gets them
    gathered (room, then its overflow) into one buffer. *Order:* the
    room holds an asset's first 120 hits in the order taken, the
    overflow the rest in the order taken, and the counting sort of the
    overflow is stable, so each asset's hits are in the order they were
    taken, as today. *Tests:* the comparison-sort test (four windows,
    three parts, an empty middle part, 5,000 hits on the index's last
    asset, so rooms overflow heavily), now also checking that counts,
    overflow and its ends are empty after each window; a new test of an
    asset's hits in taken order for the part's first asset (121 hits),
    second (120), a middle one (367), last (119) and the one before it
    (2), against the hits filtered by asset from the lists in list
    order; the blocks test; the rest of the core's tests.
  - **C2, C1 with parts of 512 assets.** Rooms change what a part costs
    (no `part` buffer, half the rooms' memory), so the part size is
    tried again with them; a part size alone is not a candidate, since
    1,024 assets had the lowest first-pass CPU at both job counts in
    0199. `front_partition` gives each list's run of the new part, so
    each asset gets the same hits in the same order, and assets are
    still visited in order. *Tests:* C1's (the tests use
    `ASSETS_PER_PART` for their parts).
  - **C3, the scatter through a line of hits per asset** (the usual
    write-combining of a radix sort's scatter). `group_part` counts and
    sums as today, then writes each hit into its asset's block of 8
    staged hits (64 bytes); when a block's last position is written,
    the block is copied into `grouped` (a whole cache line: blocks start
    where `grouped`'s address is a multiple of 64), and after the
    scatter each asset's partial block is copied. Only the asset's own
    positions are copied (from its start, if the block starts before
    it). *Order:* every hit is written to the same position as today.
    *Tests:* the comparison-sort test; a new test that `grouped` equals
    a stable sort of the part by asset, on every part of a window of
    100,000 hits over 3,000 assets.

  Considered and not made candidates: counts in a narrower type (the
  1,025 counters are 4 KiB and stay in L1; a part can hold more than
  65,535 hits, so `u16` needs a fallback); filtering straight from the
  counts (0189: assets with fewer than 3 hits hold 0.02% of the hits,
  and no other count proves that no line can form); fusing the filter's
  counting into the scatter (it needs a count per slot per asset, 4 MiB
  for a part); counting while taking without rooms (G1, 0190).

  *Timing protocol.* `work/s43/ab.sh` (session 41's): `gunfinger-eval
  --other-peaks-dir <NAS store> --work work/s43/w --jobs N scan
  stakka-skynet-knowledge` (saved index), the search time the report's
  `wall_seconds`; for N = 32 then 16, the start build, each candidate,
  the candidates in reverse order, the start build (A B ... B A);
  nothing else running. Every report is compared with the start
  build's (`work/s43/dev-start/`) apart from `wall_seconds`.

  *Stop point.* A candidate is kept only if its search time at 32
  workers is at least 3% below the start build's in every pair (the
  forward half's runs and the reversed half's), with byte-identical
  reports apart from `wall_seconds`. 16 workers are reported, not
  decided on. The three are alternatives (C1 and C2 remove the scatter
  C3 changes, and C2 is C1 at another part size), so there is no
  combination: of those that pass, the one with the lowest mean search
  time at 32 workers is adopted; another that passes replaces it only if
  it is faster in both pairs at 32 workers and not slower at 16 in both.
  A second round is run if a candidate's cuts straddle 3%.
- **(08:00) Candidates built** (`work/s43/build-candidates.sh`): the
  core crate's 152 tests green for each of C1, C2 and C3 (rustfmt and
  clippy run only for the code adopted, as in session 41); builds
  `work/bin/s43-c1/`, `s43-c2/`, `s43-c3/`. Round 1 next:
  `work/s43/ab.sh r1 s43-start s43-c1 s43-c2 s43-c3`.
- **(08:12) Round 1** (experiment 0200): no candidate passes the stop
  point at 32 workers (C1 1.9-2.6%, C2 0.5-1.6%, C3 -15 to -17%); all
  three are slower at 16 workers. All three are dropped. Decided now:
  before any further candidate, a single-threaded microbenchmark of the
  scatter alone on synthetic parts shaped as 0199 counted them
  (`work/s43/bench/`, never committed; experiment 0201), to find what a
  scattered write costs and why, then a second pre-registration (round
  2) if it points to a candidate. Exploratory; it decides nothing by
  itself.
- **(08:27) The microbenchmark** (experiment 0201): 40.3% of real hits
  have the asset of the hit before them in `part` (a count in a second
  diagnostic build); on synthetic parts with such repeats, today's
  count and scatter cost 2.36 ns a hit on one core against 1.66 without
  repeats, and a scatter to start plus rank 1.98 at every repeat rate;
  places in another order and branches on runs lose.
- **(08:28) Round 2, pre-registered** (committed before C4 is timed;
  its code and tests were written after 0201 and have only been
  tested).

  - **C4, ranks.** `group_part`'s count pass keeps each hit's rank
    among the part's hits of its asset (`ranks`, a `u32` per hit, the
    count before the hit's own); after the prefix sum, `starts[i]` is
    where the part's `i`th asset's hits start, and the scatter writes
    each hit to `starts[asset] + rank` without changing `starts`. The
    per-asset loop reads `grouped[starts[i]..starts[i + 1]]`. *Order:*
    a hit's rank is the number of hits of its asset taken before it, so
    `starts[asset] + rank` is the place today's scatter gives it: the
    same `grouped`, hit for hit. *Tests:* the comparison-sort test (now
    checking that `starts` is all zero after each window); a new test
    that, on every part of a window of 3,000 assets whose lists hold
    runs of three postings of every fifth asset, `grouped` equals a
    stable sort of `part` by asset and each asset's slice holds only
    its hits; the rest of the core's tests.

  *Timing protocol.* As round 1, with two rounds (`ROUNDS="1 2"
  work/s43/ab.sh r2 s43-start s43-c4`): start, C4, C4, start, twice, at
  32 then 16 jobs: four pairs at each job count.

  *Stop point.* As round 1: kept only if its search time at 32 workers
  is at least 3% below the start build's in every pair, with
  byte-identical reports apart from `wall_seconds`; 16 workers
  reported, not decided on.
