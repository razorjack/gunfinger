# Session 57 brief: the evaluation protocol again, under tonight's changes

Written on 2026-10-27, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-57-checklist.md` (progress).
3. ADR 0021, 0022, 0023 and 0024; `docs/calibration.md`.
4. `docs/session-notes.md` ("Session 52" to "Session 56"); experiments
   0231-0235 and this session's (0236 and up); the pre-registration
   `docs/research/2026-10-27-rerun-under-tonights-changes.md`.

`docs/brief-2.md` to `docs/brief-56.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 57 of Gunfinger: run the whole evaluation protocol again under tonight's changes, and look for anything they broke outside `regress`. Sessions 52-56 changed the ground the evaluations stand on: session 52 pruned five files from the NAS store; session 53 made `SpansAtSpeed(0.006)` the default second suppression (ADR 0021); session 54 scores a detection of a recording judged to carry the same title as right (ADR 0022, `docs/same-titles.txt`); sessions 55 and 56 print gaps with an estimate of the tracks they hold (their ADRs). `regress` covers the sweep of seed 2026, the Stakka scan and its leave-outs, and calibrate; the saved reports of `recall`, `shared`, `shared-mixes`, `robust`, `grid`, `mixes` (drops, rides, long mixes), the other development scans (Nocny, the 2003 mix) and the sweeps of seeds 2027-2029 predate the changes. ADR 0021's residual risk (a related version's second alignment more than 0.6% apart reaching 240 hits on its own) was to be counted when the shared panels or drops run again. This session reruns them, compares each with its old report, explains every difference by tonight's changes, and records what it finds. It is a measurement session.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).

Time box: started about 06:50 (CET); aim to finish by about 10:45, because another session follows you (the user's soft deadline is 15:00). If the runs would take longer, put them in this order and stop where time runs out, recording what was not run: the shared panels' development halves (both panels, NAS scale), the drops mixes of sessions 39 and 48 (262 tracks and NAS scale), `recall`, the other development scans at NAS scale, the sweeps of seeds 2027-2029 at NAS scale, `robust`, `grid`, the long and ridden generated mixes.

Environment (sessions 33-56's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,678 records, revision `1fc6fe50293ad330`. NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `ce293fe6165e...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-54-linux-nas`. Tonight's changes: session 52 pruned five ignored files (0231); session 53 made `SpansAtSpeed(0.006)` the default second suppression (ADR 0021, old rule `--overlap-by-spans`); session 54 scores same-title detections as right (ADR 0022, `docs/same-titles.txt`); session 55 prints gaps of 45 s or more as "ID - ID" lines (ADR 0023); session 56 adds each gap's estimate of the tracks it holds (ADR 0024).
- Build `work/bin/s56-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans, sweep reports of seeds other than 2026 (calibrate reads every sweep report there), and reports of opt-in variants must not sit in a reports directory during `regress`; run variants in a scratch work directory.
- The user's decisions of 2026-10-27 (docs/status.md, "session 39's listening questions, and four decisions"; the roadmap's bullets carry them): `--apart-in-speed 0.006` becomes the default; gaps are printed by default ("ID - ID", 45 s or more, the mix's start and end, possible lines kept, an estimate of the tracks a gap holds, JSON yes, CSV and cue sheet unchanged); a confident detection of a near-identical version counts as right; the minimum span stays undecided. Each is implemented with an ADR (the next number is in `docs/adr/`).
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen. Pending listening (Star Trails, Into Music) stays for later. When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-52.md to the newest brief, docs/session-notes.md ("Session 52" to the newest), docs/status.md (the newest entries), ADR 0021 and 0022 and the gap ADRs, `docs/calibration.md`, the roadmap's "Behaviour at scale", "Recall beyond the corpus recordings" and "A confidence statistic for shared material", and the harness's commands and where each saves its reports.
2. Write docs/brief-57.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-57-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s57/listening-digests-start.txt. Experiments number from 0236.
3. Build `work/bin/s57-start`; `regress session-54-linux` and `regress session-54-linux-nas` identical.

Steps:

1. Pre-registration, committed before the runs: the list of runs and the old report each is compared with; what counts as explained (a difference traced to the prune, to a kept segment more than 0.6% apart, to a same-title credit, or to a gap line); what counts as a finding (anything else, and in particular a new wrong confident detection, a lost confident identification, or a level change). The held-out halves of panels stay unsearched.
2. The runs, in a scratch work directory (never in the reports directory `regress` reads), under tmux with logs where long; the old reports are read, never overwritten.
3. Experiments (at most 40 lines each, written before the next starts): one per group of runs (shared panels; drops; recall and scans; sweeps, robust and grid; long and ridden mixes), each with the counts before and after and every difference explained or flagged. For the shared panels and drops, count ADR 0021's residual risk explicitly: second alignments of a related version kept by the new rule, by hits, and any that reach 240.
4. If a finding appears: do not change the product. Describe it, reproduce it with the smallest command, and put it first in the handoff.
5. Update the roadmap's bullets whose numbers changed (with the session and experiment), and `docs/calibration.md` if a number it quotes changed.
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 57" section at the end of docs/session-notes.md with a handoff; update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-54-linux` and `regress session-54-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: what changed in each group of runs and why, any finding, and what was not run.

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
- Scratch data goes under work/s57/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 15:00. Start no new run after about 14:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 57

- 2026-10-27, 06:44 by the Linux machine's clock (CET). The time box
  ends at about 10:45; the soft deadline is 15:00, and no new run
  starts after about 14:30.
- `master` at `0b14fcb`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s57/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `ce293fe6165e...`, `work/audition/answers.jsonl` `3e50aaa62181...`.
- Experiments 0001-0235; this session numbers from 0236. ADRs
  0001-0024; this session writes none (a measurement session). Test
  set: 4 of 5 evaluations used (none in this session).
- Baselines `session-54-linux` (262 tracks) and `session-54-linux-nas`.
  Build `work/bin/s56-final/` (master); `work/bin/s57-start/` is the
  same binaries (`cmp`).
- What `regress` covers: the sweep of seed 2026, the Stakka scan and its
  leave-outs (slots 3 and 11, seed 2026), and calibrate. Every other
  saved report predates some of tonight's changes; the inventory of
  which, with the command that made each, is in the pre-registration.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build, both regressions (tmux `s57start`) | 06:44-07:10 |
| 1 | the inventory of old reports and their commands; the pre-registration, committed before any run | 07:10-07:40 |
| 2-3 | the runs in the prompt's order, in scratch work directories under `work/s57/`, each group's experiment written before the next group's runs | 07:40-10:15 |
| 4-5 | findings (if any), roadmap, `docs/calibration.md` | 10:15-10:30 |
| 6-7 | review by a separate agent; notes, status, checklist, final checks | 10:30-11:00 |

## Decisions

- **(06:45) Start build and regressions** (`work/s57/start.sh`, tmux
  `s57start`, logs in `work/s57/logs/`; `run.sh` and `same.py` are
  session 56's with the paths changed). The build was a no-op: master
  is `s56-final`'s code, and both binaries `cmp` equal to
  `work/bin/s56-final/`.
- **(06:50) Start checks.** `regress session-54-linux` (0:51) and
  `regress session-54-linux-nas` (3:59, 5.6 GB peak) identical: every
  report equals its baseline's apart from `wall_seconds` (`same.py`).
- **(06:55) The inventory** (a read-only agent, 06:49-06:57; checked
  against the scripts of sessions 33-48): the old report and command of
  each run are in the pre-registration, committed at `7a47656` before
  any run. No `grid` report and no full `robust` report exist on this
  machine, no sweep of seed 2029, and no full `shared-mixes` report;
  the renders of several mixes were deleted. Those runs are listed as
  not run or, for `grid` and seed 2029, compared within today's build
  against `--overlap-by-spans`.
- **(06:59) Group 1 started**: both panels in parallel (tmux `s57sh1`,
  `s57sh2`), scratch `work/s57/wshared/` with reflink copies of the
  renders (btrfs: the copies share the old renders' blocks, so they add
  nothing to the disk's use).
- **(07:05) Scripts**: `diffs.py` sorts every detection in one report and
  not the other into the pre-registration's causes (`prune`, `apart`,
  `displaced`, `other`), and pairs the same detection at a fitted speed
  less than 0.01% apart as `refit`; `residual.py` counts ADR 0021's
  residual risk; `mixes_score.py` and `cli_compare.py` compare scores,
  CLI reports and printed tracklists (gap lines set aside).
- **(07:08) An added run for group 4** (a deviation, recorded here):
  session 40's NAS sweep of seed 2027 differs from session 54's start
  build (after the prune and ADR 0021) in 9 confident detections'
  fitted speeds (under 0.01%) and 2 confident detections' hits (by 2),
  and indexes 5 fewer assets (the prune). To tell the prune from code
  changes, `work/bin/s40-final/gunfinger-eval` sweeps seeds 2027 and
  2028 again on today's store (`--saved-index off`, so the saved index
  is never written by an old binary): what it shares with today's
  `--overlap-by-spans` run is the prune's.
- **(07:30) Group 1 done** (0236): both panels are their variant runs
  less the pruned files' weak detections; no confident change, residual
  risk at most 61 hits (a related version: 55). `diffs.py` and
  `residual.py` now count an overlapping detection of equal hits (the
  old rule ranked one of the two first).
- **(08:10) Group 2 done** (0237): harness scores identical in all 12
  reports; the CLI's tracklists change only by gap lines and, at NAS
  scale, the pruned T-Minus vocal mix upload's entries. An added trace
  (a deviation): session 48's CLI binary (`work/bin/s48-run`) on today's
  pruned store, seed 2032 at NAS scale, gives today's re-measured
  segments, so the prune moves them (`work/s57/trace-cli.sh`). Session
  39's NAS other pairs were made with the NAS configuration; today's use
  `--peaks-dir --store-only` (no NAS listing, as sessions 45 and 48 did).
- **(08:12) Group 3 started**: recall in two queues (tmux `s57rca`,
  `s57rcb`; the second runs the scans first), scratch `work/s57/wrecall`
  and `wscans`.
- **(08:24) An added trace for group 3** (a deviation): recall's clean
  queries move two weak 6-hit candidates of files that were never pruned;
  session 33's harness binary (`work/bin/s33-start`) runs the same
  queries on today's store (`--saved-index off`, tmux `s57trr`), so that
  what it shares with today's build is the prune's.
- **(08:58) Two more traces for group 3** (deviations): recall's brief
  query 911 (Organikismness - Echos at +1%) changes one confident right
  detection, 455 hits at +1.147% -> 503 at +0.933%. Today's build with
  `--overlap-by-spans` (`trace-brief.sh`) gives today's report exactly,
  so ADR 0021 did not cause it; session 33's binary on today's store
  (`trace-brief33.sh`, tmux `s57trc`) tells the prune from later code.
  The clean queries' trace (`trace-recall.sh`): session 33's binary on
  today's store moves the same two weak candidates, so the prune moves
  them.
- **(09:12) Group 3 done** (0238): recall lost 6 excerpts of other rips
  only (Known Unknown - Rollers Edit), whose only identifying rip was
  the pruned Drumtrip copy; session 33's binary on today's store
  reproduces every other moved detection of clean and brief, including
  brief query 911's confident right detection (455 -> 503 hits). The
  scans change only by ADR 0021's segments and one pruned weak detection.
  `diffs.py` and `counts.py` now key sweep and recall queries by their
  position too (two brief queries share asset, start and speed).
- **(09:11) Group 4 started** in three queues (tmux `s57g4a` sweeps and
  calibrate, `s57g4b` robust and grid, `s57g4c` session 40's binary).
- **(09:30) A trace for `grid`** (a deviation): today's build gives 0,
  65, 144 of 160 confident at 10, 15 and 20 s under both second
  suppressions (the reports are identical), where `docs/calibration.md`
  quotes 0047's 0, 67, 146 and session 27's 0137-0138 (the Mac) repeat
  them. Session 32's start build, the first on the Linux machine
  (`trace-grid.sh`, tmux `s57trg`), tells the machine from later code.
- **(09:50) Group 4 done** (0239): every difference is the prune's, ADR
  0021's or ADR 0022's; session 40's binary on today's store and today's
  `--overlap-by-spans` agree detection for detection on seeds 2027 and
  2028 at NAS scale. `grid` gives 0, 65, 144 of 160 under both rules and
  with session 32's build: a machine difference against the Mac's 0047,
  noted in `docs/calibration.md` with calibrate over seeds 2026-2029.
- **(10:05) Group 5 done** (0240): ADR 0021 alone; every report with a
  session 38 variant equals it. All five groups ran; what was not run is
  listed in the pre-registration and in 0240.
