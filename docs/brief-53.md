# Session 53 brief: the second suppression keeps both sides of a pitch ride by default

Written on 2026-10-27, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-53-checklist.md` (progress).
3. The decision note `docs/research/2026-10-22-pitch-rides-decision.md`
   (all of it), ADR 0008, ADR 0006, this session's ADR (0021) once
   written, the roadmap's "Speed that wanders within a play" and the
   newest entries of `docs/status.md`.
4. `docs/session-notes.md` ("Session 38", "Session 45", "Session 52");
   experiments 0175-0180 and 0209-0211; this session's experiment
   (0232).

`docs/brief-2.md` to `docs/brief-52.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 53 of Gunfinger: make `--apart-in-speed 0.006` the default, with an ADR. Our decision of 2026-10-27 (docs/status.md, "session 39's listening questions, and four decisions"; `docs/research/2026-10-22-pitch-rides-decision.md`, "Decision"): DJs speed a set up while a track is audible, because the set is continuous, so the second suppression should keep both sides of a pitch ride. Today `Matcher::Fitted`'s second suppression (`strongest_per_moment` on the measured spans) drops a detection that overlaps a stronger one of the same asset; with `SecondOverlap::SpansAtSpeed(0.006)` it drops it only when their fitted speeds also differ by at most 0.6% (1.5 rung steps, the chain link rule's limit). Sessions 38 and 45 measured it under criteria written first (experiments 0179-0180, 0209-0211): it keeps every side of a 1-3% ride the default loses, adds no wrong confident detection, loses no right one, leaves calibrate's false candidates unchanged and costs under 1% CPU; the CLI's 64 drop-mix reports were byte-identical. This session writes the ADR, changes the default in the product (`identify`, `explain` and every harness command that uses the default matcher), and records new baselines after explaining every difference. The decision is taken; this session implements it, carefully.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).

Time box: started about 02:20 (CET); aim to finish within about 2 hours, because other sessions follow you tonight. If a step would run past that, scale it down, record the decision in the brief and continue.

Environment (sessions 33-52's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,678 records, revision `1fc6fe50293ad330` (session 52 pruned the five files the ignore file leaves out). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `ce293fe6165e...`, 284 clusters. Baselines `session-32-linux` (262 tracks) and `session-52-linux-nas`.
- Build `work/bin/s52-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans, sweep reports of seeds other than 2026 (calibrate reads every sweep report there), and reports of opt-in variants must not sit in a reports directory during `regress`; run variants in a scratch work directory.
- The user's decisions of 2026-10-27 (docs/status.md, "session 39's listening questions, and four decisions"; the roadmap's bullets carry them): `--apart-in-speed 0.006` becomes the default; gaps are printed by default ("ID - ID", 45 s or more, the mix's start and end, possible lines kept, an estimate of the tracks a gap holds, JSON yes, CSV and cue sheet unchanged); a confident detection of a near-identical version counts as right; the minimum span stays undecided. Each is implemented with an ADR (the next number is in `docs/adr/`).
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen. Pending listening (Star Trails, Into Music) stays for later. When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-38.md, docs/brief-45.md and docs/brief-52.md, docs/session-notes.md ("Session 38", "Session 45" and the newest session), docs/status.md (the newest entries), ADR 0006, 0008 and 0017, the decision note `docs/research/2026-10-22-pitch-rides-decision.md` (all of it: its "What an ADR would decide" lists the questions the ADR answers), experiments 0175-0180 and 0209-0211, the roadmap's "Speed that wanders within a play", `docs/calibration.md`, and the code of `search/` (`strongest_per_moment`, `SecondOverlap`, `Matcher`), the CLI's options and the harness's `--apart-in-speed` and `--overlap-by-chains`.
2. Write docs/brief-53.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-53-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s53/listening-digests-start.txt. Experiments number from 0232.
3. Build `work/bin/s53-start`; `regress session-32-linux` and `regress session-52-linux-nas` identical.

Steps:

1. The ADR (the next number in `docs/adr/`), accepted on 2026-10-27 as our decision: the second suppression becomes `SpansAtSpeed(0.006)` for the default matcher; the share and what it rests on (1.5 steps of the ladder, between 0.40% measured outside rides and 0.90% of a 1% ride; a ladder with another step moves it); how a play shows two overlapping segments (the decision note recommends as measured; take that unless the code shows a reason not to, and write the reason); that ADR 0008's split consequence stays and its consequence for overlapping parts is superseded (mark ADR 0008 accordingly without rewriting it); the needle-skip consequence unchanged; the evidence by experiment. Commit it before the code change.
2. The change: the default matcher uses `SpansAtSpeed(0.006)`; `--single-pass` is unchanged. Keep today's rule reachable as an opt-in harness variant for comparison (name it plainly, for example `--second-overlap spans`), so that the old behaviour can still be measured; decide whether `--apart-in-speed` stays as an option to vary the share or goes, and write the decision down. Tests: a unit test of the predicate at the boundary (0.6% kept apart or not, by the rule's own inequality), and an end-to-end or search test where a ride's second side is kept. Help texts, README and docs that describe the rule are updated. `scripts/check.sh` green.
3. The differences, explained before any baseline: `regress session-32-linux` and `regress session-52-linux-nas` with the changed build. Every changed detection must be a segment kept because its speed differs by more than 0.6% from the stronger overlapping detection of the same file (expected: Star Trails' side at 13:13-14:37 in the Stakka mix, Into Music's middle segment in the 2003 mix, weak or possible segments); no confident identification may be lost, no wrong confident detection added, and calibrate's numbers should not move. Anything else is a finding to stop on: revert the default, record it and report. Also run the CLI's `identify` on the three development mixes at 262 tracks and NAS scale before and after, and compare the printed tracklists line by line.
4. Record baselines `session-53-linux` and `session-53-linux-nas` with the final build. One experiment (at most 40 lines): what the default change changed in the sweeps, development scans, calibrate and the printed tracklists, with CPU against the previous build (A B B A on the Stakka scan at NAS scale, idle machine).
5. Update `docs/calibration.md` (the second suppression's row), the roadmap's "Speed that wanders within a play" (done, with the ADR), and the decision note's status line.
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 53" section at the end of docs/session-notes.md with a handoff (the new baseline names, the build); update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-53-linux` and `regress session-53-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the ADR, what changed in the reports and tracklists and why, the new baselines, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product changes only as decided: the default matcher's second suppression becomes `SpansAtSpeed(0.006)`, with its ADR. No other threshold, parameter, rule, default or output change. Harness additions are tested; every `regress` difference is explained before a baseline is recorded.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
- Scratch data goes under work/s53/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 15:00. Start no new run after about 14:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 53

- 2026-10-27, 02:17 by the Linux machine's clock (CET). The time box
  ends at about 04:20; no new run starts after that.
- `master` at `3414a03`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s53/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `ce293fe6165e...`, `work/audition/answers.jsonl` `3e50aaa62181...`.
- Experiments 0001-0231; this session numbers from 0232. ADRs
  0001-0020; this session's is 0021. Test set: 4 of 5 evaluations used
  (none in this session).
- The NAS store: 40,678 records, revision `1fc6fe50293ad330`.
  Baselines `session-32-linux` (262 tracks) and `session-52-linux-nas`.
- Builds: `work/bin/s52-final/` (master).
- The code today: `Matcher::Fitted.options()` leaves `second_overlap`
  at `SecondOverlap::default()`, which is `Spans` (the second
  suppression drops every detection whose measured span overlaps a
  stronger one of the same asset). The harness reaches `Chains` with
  `--overlap-by-chains` and `SpansAtSpeed(share)` with
  `--apart-in-speed SHARE`; the CLI has no option for it.
  `search::design` (the `matching` setting that CLI reports record and
  `identify --save-dir` compares) is `lines-chains-v1` and does not
  name the second suppression.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build and both regressions (tmux `s53start`); the CLI's tracklists before (tmux `s53clibefore`) | 02:17-02:45 |
| 1 | ADR 0021, ADR 0008's status line | 02:45-03:00 |
| 2 | the change, the opt-in old rule, tests, help, README and docs; `scripts/check.sh` | 03:00-03:25 |
| 3 | `regress` at both scales, the other two development scans at NAS scale, the CLI's tracklists after; every difference explained | 03:25-03:45 |
| 4 | baselines `session-53-linux` and `session-53-linux-nas`; CPU A B B A; experiment 0232 | 03:45-04:00 |
| 5 | `docs/calibration.md`, the roadmap, the decision note's status | 04:00-04:05 |
| 6-7 | review, notes, status, checklist, final checks | 04:05-04:25 |

## Decisions

- **(02:17) Start build and regressions** (`work/s53/start.sh`, tmux
  `s53start`, logs in `work/s53/logs/`; `run.sh` and `same.py` are
  session 52's with the paths changed). Cargo had nothing to rebuild:
  the start build is byte-identical to `work/bin/s52-final/`.
- **(02:22) Both start regressions identical** (`work/bin/s53-start`):
  `regress session-32-linux` (0:50) and `regress session-52-linux-nas`
  (3:58, 5.4 GiB peak), every report identical to its baseline's apart
  from `wall_seconds`.
- **(02:22) The runs before the change** (`work/s53/before.sh`, tmux
  `s53before`, `work/bin/s53-start`): the CLI's `identify` on the three
  development mixes (`work/s53/cli.sh`), at 262 tracks with the corpus
  store's own records (`--peaks-dir work/peaks --store-only --saved-index
  off`, as session 45's Stakka control) and at NAS scale with the NAS
  store's own records (`--peaks-dir` the NAS store, `--store-only`; the
  CLI's saved index of that store is built again once, since session
  52's prune); the printed output and `show` of each saved report kept
  in `work/s53/cli-before/`. Then the harness's scans of the other two
  development mixes at NAS scale (`work/s53/devscans.sh`), which
  `regress` does not run, in the scratch work directory
  `work/s53/wdev-start` (copies of both clusters files and the library
  map, a link to the saved index), so the reports in `work/reports/`
  stay as they are.
- **(02:25) The share as a constant.** `search::SAME_SPEED_SHARE` is
  the chain link rule's limit, `chains::NEARBY_SPEEDS` (1.5 ×
  `speed::STEP`), so a ladder with another step moves both together, as
  the decision note says; in f64, 1.5 × 0.004 equals 0.006 exactly, so
  the default is the measured variant bit for bit (a unit test pins it).
- **(02:25) The matching design's version.** CLI reports record
  `search::design` as `search.matching`, and `identify --save-dir`
  reuses a saved report only when it is equal. The string does not name
  the second suppression, so a report saved under the old rule would be
  reused and keep the dropped side. The version goes from
  `lines-chains-v1` to `lines-chains-v2`, as its doc comment asks for a
  change the numbers do not show. The harness's reports do not record
  it, so `regress` is not affected; the CLI's JSON reports differ in
  that field, which step 3's comparison allows for.
- **(02:25) The old rule as an opt-in variant**: `--overlap-by-spans`,
  beside the existing `--overlap-by-chains` (reports in
  `variant-spans/`). `--apart-in-speed SHARE` stays, to vary the share
  (for example when the ladder's step changes, as the calibration
  register asks); it now refuses the default's own share with a message,
  so that no variant directory holds the default's reports. Both stay
  harness options; the CLI keeps only `--single-pass`.
- **(02:30) How a play shows two overlapping segments: as measured,
  with one fix the code needs.** Until now both suppressions left no
  two detections of one asset overlapping, so a play's segments, in
  order of start, also ended in order. `Play::end_seconds` and
  `track_end_seconds` take the last segment's, and the CLI's report
  check (`Report::validate`, run whenever a saved report is read by
  `show`, `review`, `listen` or `identify --save-dir`) rejects a segment
  that ends after its play. Under the new rule a weaker segment at
  another speed can lie wholly inside a stronger one (sessions 38 and
  45 kept weak and possible ones at 0.60-12.1% from a stronger one of
  their file; possible ones take part in plays); its play would then
  end early, and its saved report could not be read. So a play ends
  where its segment that ends last ends (`plays.rs`, tested); for
  disjoint segments that is the last one, as before, so nothing changes
  where segments do not overlap. Clipping for display was not taken:
  it changes the measured spans that scoring and the JSON report use,
  and the decision note recommends showing them as measured.
- **(02:35) Step 1, ADR 0021** (`docs/adr/0021-second-suppression-keeps-another-speed.md`),
  accepted on 2026-10-27 as our decision, answering the decision note's
  questions 1-5: the rule, the share and what it rests on, segments as
  measured (with the play's end fixed), a wrong file's added confident
  segment counted as wrong (the conservative reading, 0178's V2), ADR
  0008's split and needle-skip consequences standing. ADR 0008's status
  gains a paragraph naming what ADR 0021 supersedes; its text is not
  rewritten. Committed before the code change.
- **(02:28) Step 2, the change** (uncommitted until `scripts/check.sh`
  is green): `SecondOverlap`'s default is `SpansAtSpeed(SAME_SPEED_SHARE)`,
  which `Matcher::Fitted` takes, so `identify`, `explain` and every
  harness search with the default matcher use it; `--single-pass` has
  no second pass and is unchanged. The harness gains `--overlap-by-spans`;
  `--apart-in-speed 0.006` is refused as the default. Tests: the
  predicate both sides of 0.6% (0.5999% dropped, 0.6001% kept, either
  direction) and exactly at a share (binary fractions, dropped); the
  ride test now asserts both sides kept by the default matcher and one
  by `Spans`; a play containing a segment keeps its end; the property
  test of `plays::group` checks each segment against the latest end
  before it and lies inside its play. README's "How it works" item 6.
- **(02:35) Step 3, the differences** (`work/s53/step3.sh`, tmux
  `s53step3`, `work/bin/s53-run` = the working tree;
  `scripts/analysis/overlap_changes.py` lists each detection added or
  removed beside the stronger overlapping one of its file). Every
  difference is of the expected kind; nothing to stop on.
  - `regress session-32-linux` (0:51): the sweep identical; the Stakka
    scan and leave-out 3 each gain Star Trails' 793.4-877.3 s side
    (confident, 315 hits, 1.34% slower than the kept 585.1-803.7 s side)
    and 3 weak detections (6 hits, 0.68-0.78% from a stronger one of
    their file), and lose one weak Star Trails detection (18 hits,
    871.8-894.1 s), now within 0.22% of the kept side; leave-out 11
    identical. Identified 11, 8, 0 and wrong 0 unchanged; calibrate
    unchanged (690, 100, 18 false candidates at 30 hits or more, 19).
  - `regress session-52-linux-nas` (3:46): 716 of 720 sweep queries
    identical, the other 4 each gain one weak detection (6-15 hits) of
    a file not playing, 0.75-2.0% from another of that file; the Stakka
    scan and leave-out 3 gain the slower side on all three files of
    Star Trails' recording (306-315 hits, 1.34-1.47% apart) and 3 weak
    (6-7 hits, 0.68-1.39%), and lose 3 weak Star Trails detections
    (9-18 hits, within 0.22-0.52% of the kept side); leave-out 11 gains
    2 weak. Identified 11, 8, 0, wrong 0; calibrate unchanged (704, 282,
    7 false accepted, 95).
  - The other development scans at NAS scale (`work/s53/wdev-start`
    against `wdev-run`): the 2003 mix 12/12, 0 wrong, both builds; it
    gains Into Music's 1,556.9-1,616.4 s segment (confident, 1,392 hits,
    1.05% from the stronger 1,604.9-1,772.1 s one) and 12 weak (6-20
    hits, 0.69-5.8% apart). Nocny 17/17, 0 wrong, both builds; 5 weak
    added (6-20 hits, 0.61-1.51%). Nothing removed in either.
  - The CLI (`work/s53/cli-before/` against `cli-after/`): the printed
    tracklists and `show` of all six reports identical line by line. The
    JSON reports differ in `search.matching` (v1 to v2) and in three
    plays: Star Trails' play gains its 793-877 s segment (at 262 tracks;
    at NAS scale on two files, ending at 877.3 and 879.0 s instead of
    803.7 and 799.7 s), and Into Music's play in the 2003 mix at NAS
    scale gains its middle segment (its span unchanged). No play is
    added, removed or changes level.
- **(02:39) `scripts/check.sh` green** (3:58) on the changed tree;
  steps 2 and 3 committed.
- **(02:41) Step 4, baselines** (`work/s53/step4.sh`, tmux `s53step4`):
  the final build `work/bin/s53-final` (`3f02901`) is byte-identical to
  `s53-run` (no code changed after step 3's build), so `baseline
  session-53-linux` and `baseline session-53-linux-nas` copy the reports
  of step 3's regressions, made by the same binary; the final
  regressions (step 7) run them again.
- **(02:44) CPU.** A B B A on the Stakka scan at NAS scale (idle; the
  start build in `wdev-start`, the final in `wdev-run`): the final build
  used about 5% less user CPU than the start build (659-661 s against
  693-701 s). A predicate cannot account for that, so a second A B B A
  ran with one binary (`work/s53/cpu2.sh`, the final build with
  `--overlap-by-spans` against its default): +0.2%, within the runs'
  spread. The difference between the builds is the binaries', not the
  rule's (not traced; by inference code layout). Each run's report
  equals its baseline's, and the `--overlap-by-spans` report equals
  session 52's baseline exactly: the opt-in reproduces the old rule.
- **(02:50) Experiment 0232** and its data file
  (`work/s53/data0232.py`, file names only).
- **(02:50) Step 5**: `docs/calibration.md` (a row in "Changes since
  `poc-freeze-1`", and the share among the choices that do not depend
  on the library), the roadmap's "Speed that wanders within a play"
  (done for pitch rides; wander within a side and rides in short plays
  stay open) and the decision note's status line.
