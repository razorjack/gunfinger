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
  rebuild).
