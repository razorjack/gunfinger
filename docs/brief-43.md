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
