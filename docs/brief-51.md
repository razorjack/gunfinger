# Session 51 brief: the edge of the ladder, measured for a decision

Written on 2026-10-25, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-51-checklist.md` (progress).
3. Experiments 0033 and 0059 and this session's (0226 and up); the
   roadmap's "The edge of the ladder" and "Behaviour at scale";
   `docs/calibration.md`; `work/s49/cluster-speeds.json`.
4. `docs/session-notes.md` ("Session 49", "Session 50"),
   `docs/status.md` (newest entries); the code of
   `crates/gunfinger-core/src/speed.rs` (the ladders,
   `ladder_with_extra_rungs`), `crates/gunfinger-eval/src/sweep.rs`
   (`--other-rips`, the speeds it plays), `calibrate.rs`, and
   `scripts/analysis/relative_speed.py` and `cluster_speeds.py`.

`docs/brief-2.md` to `docs/brief-50.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 51 of Gunfinger: the edge of the ladder, measured for a decision. The search's ladder reaches about ±8.4% of relative speed (experiment 0033; recall gone at ±9%), and `--extra-rungs 3` recovers every speed to ±9% for 14% more search CPU, measured before sessions 36-46 made the search about 40% cheaper. With other rips only (experiment 0059, session 7), recall holds to 9% of relative speed and is gone from 10.2%. Session 49 (step 5, the roadmap's "The edge of the ladder") counted the NAS clusters' speed spans: of 284 clusters, 75, 12, 1 and 1 span more than 2, 4, 6 and 8% (Falcon - The Stand 11.5%); with a deck at ±5%, 50 clusters hold two copies of which one, indexed alone, would miss a play of the other, and 38 copies (35 of them one channel's uploads, 3-5% fast) have no other copy in reach at one extreme. Which copy runs at the record's true speed is unknown, and replay speed changes by accident when analogue media is digitised. This session measures what extra rungs cost and recover today, at NAS scale, for plays near and beyond the ladder's edge against the copies the library actually holds, and what they do to wrong answers, so that a decision (a wider ladder, a targeted one, or none) can be taken from current numbers. It is a measurement session: the default ladder and the product do not change; a change is ours, with an ADR.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32). Timing comparisons need an otherwise idle machine.

Time box: started about 07:40 (CET; the clocks went back an hour tonight); aim to finish by about 11:30 (the user's soft deadline is 11:59; work in progress may run past it). If a step would run past that, scale it down (fewer speeds, fewer extra-rung counts), record the decision in the brief and continue.

Environment (sessions 33-50's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records), library `/mnt/atlas/Music/dnb` (read only; on Ethernet again tonight), reached with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: after the listening of 2026-10-25 (docs/status.md, "after session 46") SHA-256 `fdf7428a6322...`, the same 284 clusters as before. Baselines `session-32-linux` and `session-33-linux-nas`. Build `work/bin/s50-final/` (master). Since sessions 41, 43 and 46 a NAS-scale development scan searches in about 24 s and `regress session-33-linux-nas` takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans, sweep reports of seeds other than 2026 (calibrate reads every sweep report there), and reports of opt-in variants must not sit in a reports directory during `regress`; run variants in a scratch work directory.
- Session 38's opt-in `--apart-in-speed` (confirmed by session 45; adoption ours, with an ADR) and session 37's `--overlap-by-chains` are variants, not defaults.
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen tonight. Pending listening (items 4, 5, 7 and 9 of session 40's pack, in `work/s47/questions/05-pack-s40.json`, session 39's questions, Star Trails, Into Music) stays for later.
- The shell's safety check has failed to return verdicts for minutes at a time tonight; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-49.md and docs/brief-50.md, docs/session-notes.md ("Session 49" and "Session 50"), docs/status.md (newest entries), the roadmap's "The edge of the ladder" and "Behaviour at scale", `docs/calibration.md`, experiments 0033, 0059, 0219-0221 and 0222-0225, `work/s49/cluster-speeds.json`, and the code of `speed.rs` (the ladders, `--extra-rungs`), `sweep.rs` (`--other-rips`, the speeds it plays), `recall.rs` and `calibrate.rs`.
2. Write docs/brief-51.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-51-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s51/listening-digests-start.txt; it must be the same at the end. Experiments number from 0226.
3. Build `work/bin/s51-start`; `regress session-32-linux` and `regress session-33-linux-nas` identical.

Steps:

1. Pre-registration, committed before any run: the conditions (relative speeds from about ±5% to ±12% between the played excerpt and the indexed copies, built either by playing excerpts at those speeds against the full index, or with `sweep --other-rips` so that only other copies of the recording stay indexed, which measures the copies the library really holds); the ladders compared (the default, `--extra-rungs` 1, 2, 3 and 5, and if cheap a targeted variant such as extra rungs searched only for windows with no confident line, as a harness-only option); the scores (confident and possible recall by relative speed, wrong confident answers and calibrate's strongest false candidates at both scales, search CPU and wall time against the default on the idle machine); and what result would make a wider or targeted ladder worth an ADR proposal (for example: recall recovered beyond ±8.4% with no new wrong confident answer and CPU within a stated bound).
2. The harness, tested, only if needed (speeds beyond today's sweep list, the targeted variant); harness only, `regress` identical.
3. The runs, at 262 tracks and at NAS scale, under tmux with logs; timings on the idle machine, A B B A for the CPU comparison of the default against the chosen extra-rung counts on the Stakka scan.
4. Experiments (at most 40 lines each, written before the next starts): recall by relative speed per ladder; wrong answers and calibrate per ladder; CPU per ladder; the real clusters: how many of session 49's at-risk copies each ladder brings back into reach for a deck at ±5% and ±8%.
5. Write a short decision note (`docs/research/2026-10-25-ladder-edge-decision.md`): the options, their measured costs and gains, the risks, and a recommendation that is ours to take. Do not write the ADR and do not change the default.
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 51" section at the end of docs/session-notes.md with a handoff and the decision left for us; update docs/status.md, the checklist and the roadmap's "The edge of the ladder". Final checks: `scripts/check.sh` green, `regress session-32-linux` and `regress session-33-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: recall, wrong answers and CPU per ladder in a few lines, the recommendation, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's behaviour does not change: no threshold, parameter, rule, default or output change; variants are opt-in, tested, and keep `regress` identical when off. Harness additions are tested and keep `regress` identical.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
- Scratch data goes under work/s51/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:59. Start no new run after about 11:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 51

- 2026-10-25, 07:36 by the Linux machine's clock (CET). The time box
  ends at about 11:30; no new run starts after that.
- `master` at `af5e7e5`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s51/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `d3bb1fe061f7...`, the NAS clusters file
  `fdf7428a6322...`, `work/audition/answers.jsonl` `78c3fb0e7ce0...`.
- Experiments 0001-0225; this session numbers from 0226. Test set: 4 of
  5 evaluations used (none in this session).
- The ladders today (`speed.rs`): turntable rungs from 0.92 to 1.08,
  0.4% apart (41 rungs), and the same speeds as key-locked tempo ratios
  (41 rungs); `identify` and the harness search both (82 rungs).
  `--extra-rungs N` (harness, global) adds N rungs past either end of
  both ladders (82 + 4N rungs: 86, 90, 94 and 102 for N = 1, 2, 3, 5;
  turntable ends ±8.4%, ±8.8%, ±9.2% and ±10.0%); its reports go to
  `reports/[...]/extra-rungs-N/`.
- The sweep plays its 9 speeds (`SPEEDS_PERCENT`, ±8% at most);
  `--other-rips` (NAS scale) leaves each indexed excerpt's source and its
  identical copies out and searches only the excerpts whose recording
  keeps another copy. `robust --only speed+8.2pct,...` plays single
  speeds past the ends (±8.2 to ±9 by 0.2, and ±10, ±12, ±16) on 40
  indexed and 10 held-out excerpts. `relative_speed.py` (0059) gives each
  other-rips query the relative speed to the nearest remaining copy from
  the clusters' pair speeds; `cluster_speeds.py` (session 49) counts the
  copies with no other copy within a ladder of ±8% for a deck at ±d.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build and both regressions (tmux `s51start`) | 07:36-07:55 |
| 1 | pre-registration in this brief; commit | 07:55-08:10 |
| 2 | harness: `sweep --speeds edge`; `cluster_speeds.py` reach; `ladder_edge.py`; tests; both regressions | 08:10-08:45 |
| 3 | runs: 262 tracks, then NAS scale, one queue; CPU orders on the idle machine | 08:45-10:45 |
| 4 | experiments 0226 and up, each before the next | 09:15-11:00 |
| 5 | decision note | 11:00-11:15 |
| 6-7 | review, notes, status, roadmap, final checks | 11:10-11:55 |

## Decisions

- **(07:39) Start build and regressions** (`work/s51/start.sh`, tmux
  `s51start`, logs in `work/s51/logs/`; `run.sh` and `same.py` are
  session 50's with the paths changed). The start build is
  byte-identical to `work/bin/s50-final/` (cargo had nothing to
  rebuild). `regress session-32-linux` (0:50) and `regress
  session-33-linux-nas` (3:52, 5.3 GiB peak) identical: every report
  identical to its baseline's apart from `wall_seconds`.
- **(07:50) Step 1, the pre-registration** (committed before any run).

  *Ladders.* L0, the default (82 rungs); L1, L2, L3 and L5, today's
  `--extra-rungs 1, 2, 3, 5` (86, 90, 94, 102 rungs; the turntable and
  key-locked ladders both extended, as `identify` searches both). A
  targeted ladder T_k (the extra rungs searched only where the default
  finds no confident line) is not built: it is estimated from the runs
  (below), and a proposal could only be to build and measure it.

  *Conditions.* The sweep panel of seed 2026 (`docs/panels/`, 60
  indexed and 20 held-out 30 s excerpts), the default matcher, every
  ladder:
  - C1, the full index: the sweep's index (held-out recordings out,
    every other file in, the excerpt's source among them) with the
    excerpts played at the edge speeds ±8.4, ±8.8, ±9.2, ±9.6, ±10.0,
    ±10.4, ±10.8, ±11.2 and ±12.0% (18 speeds, 1,440 queries; a new
    harness option, `sweep --speeds edge`), at 262 tracks and at NAS
    scale. The relative speed is the played speed at 262 tracks; at NAS
    scale it is also given to the nearest indexed copy of the recording
    (the clusters' pair speeds, as `relative_speed.py` computes them).
  - C2, other copies only (NAS scale): `sweep --other-rips`, at the
    sweep's 9 speeds and at the edge speeds (423 and 846 queries for
    seed 2026, from 47 excerpts): the source and its identical copies
    left out, so that only the copies the library holds can answer.
    Relative speed to the nearest remaining copy, as in 0059. If time
    allows after every other run: seeds 2027 and 2028 for L0, L3 and L5.
  - C3, the standard protocol: `regress session-32-linux` and `regress
    session-33-linux-nas` with each ladder in scratch work directories
    (the sweep's 9 speeds, the Stakka development scan and its
    leave-outs 3 and 11), what changes against the default, and
    `calibrate` on those reports per ladder and scale. C1's and C2's
    reports sit in other scratch directories, so calibrate reads only
    C3's.
  - C4, the cost: the Stakka development scan at both scales in the
    palindromic order L0, L1, L2, L3, L5, L5, L3, L2, L1, L0 (A B B A
    generalised), alone in its queue on the otherwise idle machine:
    the scan report's `wall_seconds` (decode and search on 32 workers)
    and the process's user + system CPU (`/usr/bin/time`; at NAS scale
    it includes loading the saved index, the same for every ladder).
  - C5, the real clusters: `cluster_speeds.py` with the ladder's reach
    as a parameter, on the NAS clusters file: copies with no other copy
    within reach (`alone`) and clusters holding a copy beyond reach of
    another, for a deck at ±5% and ±8%, at each ladder's measured reach
    R_k and at its nominal end (8 + 0.4k%); session 49's figures used
    8.0%.

  *Scores.* Per ladder: confident recall (an own-cluster confident
  detection) and possible-or-better recall (own-cluster hits of 60 or
  more, `MIN_POSSIBLE_HITS`) by played speed (C1) and by |relative
  speed| bin: [0, 8), [8, 8.4), [8.4, 8.8), [8.8, 9.2), [9.2, 9.6),
  [9.6, 10.0), [10.0, 10.4), [10.4, 10.8), [10.8, 11.2), [11.2, 12.4),
  12.4 and above (C1 at NAS scale, C2); wrong confident answers (any
  confident detection outside the own cluster, held-out queries
  included) and wrong possible answers, with which are new against L0
  (the same query and file not confident under L0); C3's changes and
  calibrate's weakest identifying, strongest false candidate, margin
  and possible-tier figures; C4's medians and ranges and the ratios to
  L0; C5's counts. The measured reach R_k: the largest edge speed at
  which C1 at 262 tracks keeps at least 57 of 60 (95%) confident at
  both signs, at it and at every edge speed below it (R_0 = 8.0 if 8.4
  already fails).

  *The targeted estimate T_k* (derived, not run): for each query, L0's
  answers if L0 has any confident detection, otherwise L_k's. Its cost
  on the Stakka scan: CPU_0 + u × (CPU_k - CPU_0), with u the share of
  the mix's time that no confident detection of the default scan covers;
  an estimate, since a real targeted search also pays for finding the
  uncovered windows and searches them at every extra rung.

  *Criteria.* A wider ladder L_k is worth an ADR proposal (adoption
  ours) only if all hold:
  - (G1) R_k > R_0;
  - (G2) in C2 (both speed sets of seed 2026 pooled), the queries with
    |relative speed| in (R_0, R_k + 0.4]: L_k's confident recall is at
    least 30 points above L0's, and within R_0 L_k recalls no fewer
    queries than L0;
  - (W) no new wrong confident answer in any run at either scale (C1,
    C2, C3); calibrate's strongest false candidate at each scale at most
    10% above L0's and its weakest identifying not lower; the
    development scans (C3) keep the same confident plays;
  - (C) on the NAS-scale Stakka scan, the median `wall_seconds` and
    the median process CPU each within +15% of L0's.

  Among the ladders that pass, the recommendation is the smallest k
  whose reach brings at least 80% of session 49's 38 copies alone (deck
  ±5%) back into reach; if none does, the passing ladder that brings
  back most, with its cost stated. T_k is worth a proposal to build and
  measure if, with the same k, it meets G1, G2 and W and its estimated
  CPU is within +5% of L0's. If nothing passes, the recommendation is
  no change, with the criterion that failed.

  *Expected, declared now.* R_k about 8.4 + 0.4k% (0033: recall holds
  0.4-0.6% past the last rung); CPU about +5% per extra rung pair (4
  rungs per k), so L3 about +15% and L5 about +24%, against the bound
  of 15%; at NAS scale some new wrong answers where extra rungs reach
  copies or versions at other speeds (0059: China Cup's Prototype
  upload, 16 wrong to 18 with L3).
- **(07:48) Step 2, the harness** (harness and scripts only):
  - `sweep --speeds edge` (`sweep::Speeds`): the excerpts played at the
    18 edge speeds (`EDGE_SPEEDS_PERCENT`) instead of the sweep's 9;
    `-edge` in the report's name. `render_all` takes the speeds; a
    render of a speed that is not a whole percent is named to a tenth
    (`000-+8.4.mp3`), whole ones as before, so the sweep's and
    `recall`'s renders keep their names.
  - `scripts/analysis/ladder_edge.py` (`recall`, `uncovered`) with tests:
    the registered scores, the targeted estimate and the share of a scan
    no confident detection covers. Relative speeds are rounded to 1e-6
    so that a speed played on a bin's edge falls in its bin (found by
    the tests: 1 - 0.084 is not exactly 0.916).
  - `cluster_speeds.py` takes the reach as a fourth argument (default
    0.08, the output unchanged: the same JSON as session 49's).
  - Scratch work directories (`work/s51/setup.sh`): `w262` and `wnas`
    (C3, C4), `w262e` and `wnase` (C1, C2); their `sweep/` links to
    `work/s51/renders/`, which holds links to the sweep's renders of
    seeds 2026-2028 and the edge renders, made there.
- **(07:48) The order of runs and experiments.** To keep each experiment
  written before the next one's runs start, the runs go in three queues:
  3a (C1, C2; experiment 0226, recall by relative speed with the wrong
  answers of those runs), 3b (C3 and calibrate; 0227), 3c (C4 alone on
  the idle machine; 0228); C5 has no runs (0229, after 0226 gives the
  reaches). Seeds 2027 and 2028 of C2 only if time is left after 0229.
- **(07:57) Step 2's checks** (`work/s51/step2.sh`, tmux `s51run`):
  `scripts/check.sh` green (3:54); run build `work/bin/s51-run/`, its
  `gunfinger` byte-identical to the start build's; `regress
  session-32-linux` (0:50) and `regress session-33-linux-nas` (3:45)
  identical, every report identical to its baseline's apart from
  `wall_seconds`. Queue 3a started at 07:57 in the same tmux session.
- **(08:13) A fix in `ladder_edge.py`, after reading L0's NAS-scale
  results.** `recall` took the recordings from every clusters file
  given, so the corpus clusters (read second, for their pair speeds)
  replaced the NAS clusters for corpus files, and the relative speeds of
  other-rips queries were computed without the larger library's copies
  (3 queries recalled at "10-11%"). The recordings now come from the
  first file only (`clusters_and_pairs`, tested); later files add pairs.
  No registered definition changes. With it, L0 at NAS scale recalls 1 of
  79 other-rips queries in 9.2-9.6% and none beyond.
- **(08:53) Experiment 0226** (queue 3a, 07:57-08:50, every run exit 0):
  R_k = 8.4, 8.8, 9.2, 9.6 and 10.4% for k = 0, 1, 2, 3, 5; G1 holds for
  every k; G2's gain holds (+33 to +75 points), its second clause fails
  by one query for every L_k (Siren Bass's upload near -8%) and holds for
  T_k; no new wrong confident answer in C1 or C2. Queue 3b starts after
  this commit.
- **(09:19) Experiment 0227** (queue 3b, 08:53-09:18, every run exit 0):
  W fails for L2, L3 and L5 (and T2, T3, T5): one new wrong confident
  answer at NAS scale, the held-out Cool Rock Stuff excerpt at +5%
  against a bootleg that holds it, which the default already answers
  wrongly at 7 other speeds. W holds for L1 and T1. Calibrate and the
  development scans' plays are unchanged under every ladder. With 0226,
  no wider ladder passes (G2's second clause, and W for k >= 2); T1 is
  the only candidate left, pending its estimated CPU (0228). Queue 3c
  starts after this commit, on the idle machine.
- **(09:29) Experiment 0228** (queue 3c, 09:20-09:26, every run exit 0):
  CPU follows the rungs (NAS scale ×1.046, ×1.094, ×1.141, ×1.229 for
  L1, L2, L3, L5); C holds for L1-L3, fails for L5; T_k's estimate is
  ×1.001-1.005. `work/s51/cpu-copy.sh` copied each scan report as its run
  exited (the report is overwritten by the next run of the same ladder);
  it stopped at the queue's last line before copying run 10's, which
  was copied by hand at 09:27 (still the current file).
