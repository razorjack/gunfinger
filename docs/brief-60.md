# Session 60 brief: reject impossible alignments before scattering hits

Written on 2026-10-30, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-60-checklist.md` (progress).
3. `docs/research/2026-10-29-matching-and-archive-proposals.md` (§2 and
   "Evaluation and adoption boundaries"); the roadmap's "Where search
   time goes"; `docs/research/2026-10-21-low-bandwidth-search.md`.
4. `docs/session-notes.md` ("Session 41", "Session 43", "Session 46",
   "Session 59"); experiments 0167-0174, 0199-0205, 0212-0214 and this
   session's (0243 and up).
5. `crates/gunfinger-core/src/search/lines.rs` (`WindowHits::lines`,
   `OffsetSlots::may_form_line`, `Phases`).

`docs/brief-2.md` to `docs/brief-59.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 60 of Gunfinger: reject impossible alignments before scattering hits (proposal 2 of `docs/research/2026-10-29-matching-and-archive-proposals.md`). Grouping a part's hits by asset is still the largest stage of the first pass: 40.2% of first-pass CPU at 32 workers and 45.9% at 16 (experiment 0214, the Linux machine). `WindowHits::lines` in `search/lines.rs` scatters every hit by asset, then runs `OffsetSlots::may_form_line` on each asset; a session 36 trace passed only 2.2% of the assets with at least three hits on to sorting. Narrower hits (0171), grouping straight from the lists (0172) and several scatter layouts (0200-0202) lost because they still scattered essentially every hit. The proposal: before grouping, build a small conservative counting sketch over (asset, folded offset) and scatter only the hits of assets that might hold a line, keeping today's precise filter and line construction for those. The required property is one-sided: folding or collisions may admit extra assets but must never exclude an asset with a real line. This session tests that, exactly: reports must stay byte-identical (evidence, order and float bits), and a speed-up is adopted only if it is substantial and every check holds, as sessions 41, 43 and 46 adopted theirs.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600 (low memory bandwidth for its core count: `docs/research/2026-10-21-low-bandwidth-search.md`); default jobs (32). Timing comparisons need an otherwise idle machine. The Mac is not available tonight; say so where the proposal asks for macOS timings.

Time box: started about 01:25 (CET); aim to finish within about 2.5 hours, because other sessions follow you tonight. If a step would run past that, scale it down (fewer sketch layouts, corpus scale first), record the decision in the brief and continue.

Environment (sessions 33-59's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,678 records, revision `1fc6fe50293ad330`. NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `ce293fe6165e...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-54-linux-nas`.
- Build `work/bin/s59-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Since `45f92d5` (2026-10-28) its calibrate reads only the reports its baseline holds; still run variants and other seeds in a scratch work directory, never in the reports directory `regress` compares.
- The user restructured the code on 2026-10-28 without meaning to change behaviour (commits `b5a84e3` to `f94604a`: harness subcommands in modules, named results, `Search` passed through the core's search, enums for closed sets). The newest build in `work/bin/` (`s59-final`) predates it: build from master. If `regress` is not identical with a build of today's master, that is the first finding: trace it, record it, and fix it only if it is plainly a harness slip, with a test.
- Since sessions 53-56: `SpansAtSpeed(0.006)` is the default second suppression (ADR 0021); same-title detections (`docs/same-titles.txt`) are scored right (ADR 0022); `identify` prints gaps as "ID - ID" lines with an estimate of their tracks (ADR 0023, 0024). The minimum span stays undecided. A change to an existing decision needs a new ADR (the next number is in `docs/adr/`).
- The research note `docs/research/2026-10-29-matching-and-archive-proposals.md` (2026-10-29) records six unmeasured proposals and its evaluation boundaries; any session working on one follows its "Evaluation and adoption boundaries".
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen. Pending listening (Star Trails, Into Music) stays for later. When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-59.md, docs/session-notes.md ("Session 41", "Session 43", "Session 46" and "Session 59"), docs/status.md (the newest entries), the proposals note (all of it, with its "Evaluation and adoption boundaries"), `docs/research/2026-10-21-low-bandwidth-search.md`, experiments 0167-0174 and 0199-0205 and 0212-0214, the roadmap's "Where search time goes", and the code of `search/lines.rs` (`WindowHits::lines`, `OffsetSlots`, `may_form_line` and its fallbacks for phases that cannot be represented exactly), the part and scratch layout, and the harness's stage timers.
2. Write docs/brief-60.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-60-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s60/listening-digests-start.txt. Experiments number from 0243.
3. Build `work/bin/s60-start` from master; `regress session-54-linux` and `regress session-54-linux-nas` identical.

Steps:

1. Pre-registration, committed before any timing: the sketch layouts and sizes tried (a few, no more), the proof obligation (the rejection condition is conservative, including neighbouring buckets across wraparound, counter saturation and the existing fallback), the replay data (real hit buffers captured from representative parts at 262 tracks and at NAS scale, kept under work/s60/), the equality checks (lines and their order identical on the replays; every report of `regress` byte-identical; the three development mixes' CLI JSON identical), the timing protocol (A B B A on the idle machine, the Stakka scan at NAS scale and at 262 tracks, 16 and 32 jobs, wall and user CPU, with the stage timers), the cost of clearing and updating the sketch counted in, skewed or dense buffers that make the sketch pass most assets, scratch bytes per worker and peak memory, and the bar for adoption (for example, at least 5% less wall time at NAS scale at 32 jobs with every check identical; otherwise a recorded negative result).
2. The prototype, tested: unit tests that the sketch never rejects an asset with a line (property tests over random and adversarial buffers, wraparound, saturation), and a replay harness comparing lines with today's path. No SIMD, no `unsafe`.
3. The measurements, in order: replays (pass rate and line equality per layout), then full searches for the layout that wins the replays.
4. Experiments (at most 40 lines each, written before the next starts): the replays; the full-search timings.
5. Adoption: only if the registered bar is met and every equality check holds, make it the search's path on master (no option kept unless it costs nothing), with the roadmap's "Where search time goes" and the low-bandwidth note updated; otherwise leave master's search unchanged, keep the prototype only if it is cheap to keep and clearly marked, and record the negative result.
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 60" section at the end of docs/session-notes.md with a handoff; update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-54-linux` and `regress session-54-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the sketch's pass rate, the timings against the bar, whether it was adopted, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's behaviour does not change: reports stay byte-identical, and no threshold, parameter, rule, default or output changes. A faster search path with identical output may replace today's on master only under step 5. Harness additions are tested and keep `regress` identical.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
- Scratch data goes under work/s60/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 10:00. Start no new run after about 09:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 60

- 2026-10-30, 01:18 by the Linux machine's clock (CET). The time box is
  about 2.5 hours: aim to finish by about 03:50. The soft deadline is
  10:00; no new run starts after about 09:30.
- `master` at `55dcee7`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s60/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `ce293fe6165e...`, `work/audition/answers.jsonl` `4b83d2fc9a2e...`.
- Experiments 0001-0242; this session numbers from 0243. ADRs
  0001-0024. Test set: 4 of 5 evaluations used (none in this session).
- Baselines `session-54-linux` (262 tracks) and `session-54-linux-nas`.
  The newest build, `work/bin/s59-final/`, predates the restructuring
  of 2026-10-28 (`b5a84e3` to `f94604a`); the start build is made from
  master.
- What the search does today (`search/lines.rs`): for each window and
  rung, `WindowHits::lines` takes the hits of 1,024 assets at a time
  off the fronts of the window's posting lists (`take_part`, 8 bytes a
  hit), groups them by asset with a counting sort (`group_part`: a count
  pass and a scatter into `grouped`), then for each asset with 3 hits or
  more runs `OffsetSlots::may_form_line` (pairs of neighbouring slots
  of 4-frame buckets folded into 4,096 slots; passes when a pair holds
  3 hits), and aligns, sorts and builds lines only for the assets it
  passes. When a point lies `MAX_PHASE_FRAMES` (2^31 frames) or more
  from frame 0 (`Phases::exact` false) the filter is skipped and every
  asset with 3 hits is sorted.
- Where the first pass's CPU goes (0214, 32 and 16 workers): grouping a
  part 40.2% and 45.9% (about four fifths of it the scatter, 0199),
  the pre-filter 20.7% and 18.3%, query analysis 14.9% and 13.1%,
  taking parts 13.1% and 11.6%, aligning, sorting and lines 10.5%. A
  part holds 45,060 hits on average, 1,017 assets with a hit, 44 hits
  each, most assets 16-63 hits (0199).
- The Mac is not available tonight: no macOS timings (the proposal asks
  for them; recorded as not done).

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build, both regressions (tmux `s60start`) | 01:18-01:40 |
| 1 | pre-registration (layouts, proof, replays, equality checks, timing protocol, bar), committed | 01:40-01:55 |
| 2 | capture of real windows (a diagnostic build, never committed); the prototype with its tests and a replay harness, in a scratch worktree | 01:55-02:40 |
| 3 | replays (pass rates, line equality, replay CPU); full searches A B B A for the replay winner | 02:40-03:10 |
| 4 | experiments 0243 (replays) and 0244 (full searches) | with step 3 |
| 5 | adoption or the negative result | 03:10-03:25 |
| 6-7 | review by a separate agent; notes, status, checklist, final checks | 03:25-03:55 |

## Decisions

