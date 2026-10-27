# Session 54 brief: a confident detection of a near-identical version counts as right

Written on 2026-10-27, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-54-checklist.md` (progress).
3. ADR 0004, ADR 0017, this session's ADR (0022) once written, the
   roadmap's "The verdicts by ear on borderline pairs", `docs/calibration.md`
   and the newest entries of `docs/status.md` ("session 39's listening
   questions, and four decisions").
4. `docs/session-notes.md` ("Session 40", "Session 53"); experiments
   0059, 0186-0188 and 0230; this session's experiment (0233).

`docs/brief-2.md` to `docs/brief-53.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 54 of Gunfinger: a confident detection of a near-identical version counts as right, with an ADR. Our decision of 2026-10-27 (docs/status.md, "session 39's listening questions, and four decisions"; the roadmap's "The verdicts by ear on borderline pairs"): Gunfinger identifies track titles. When a mix plays Bad Company - China Cup and the search names ƆƐI3C's China Cup upload, which the user judged `different` by ear only because its hi-hats differ for a few bars, the identification is correct. Today the harness credits a detection only through the truth's duplicate cluster (one recording), so that upload counts as a wrong confident answer: 18 of the 25 false confident detections over three NAS sweeps (session 40), the wrong answers of seed 2027 in session 51 (0230), and 2 in experiment 0059. This session decides how the harness learns that two recordings carry the same title, implements it in the scoring of `sweep`, `scan`, `calibrate`, `recall` and wherever else a detection is judged right or wrong, writes the ADR, and records new baselines. Clusters stay what they are (one recording each); the product's search and output do not change.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).

Time box: started about 03:10 (CET); aim to finish within about 2 hours, because other sessions follow you tonight. If a step would run past that, scale it down, record the decision in the brief and continue.

Environment (sessions 33-53's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,678 records, revision `1fc6fe50293ad330` (session 52 pruned the five files the ignore file leaves out). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `ce293fe6165e...`, 284 clusters. Baselines `session-53-linux` (262 tracks) and `session-53-linux-nas` (session 53: `SpansAtSpeed(0.006)` became the default second suppression, ADR 0021; the old rule is the harness option `--overlap-by-spans`).
- Build `work/bin/s53-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans, sweep reports of seeds other than 2026 (calibrate reads every sweep report there), and reports of opt-in variants must not sit in a reports directory during `regress`; run variants in a scratch work directory.
- The user's decisions of 2026-10-27 (docs/status.md, "session 39's listening questions, and four decisions"; the roadmap's bullets carry them): `--apart-in-speed 0.006` becomes the default; gaps are printed by default ("ID - ID", 45 s or more, the mix's start and end, possible lines kept, an estimate of the tracks a gap holds, JSON yes, CSV and cue sheet unchanged); a confident detection of a near-identical version counts as right; the minimum span stays undecided. Each is implemented with an ADR (the next number is in `docs/adr/`).
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen. Pending listening (Star Trails, Into Music) stays for later. When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), the newest brief, docs/session-notes.md (the newest session, and "Session 40"), docs/status.md (the newest entries, and those of the listenings of 2026-10-22 and 2026-10-25), ADR 0004 (what a test evaluation scores) and 0017, `docs/calibration.md`, the roadmap's "The verdicts by ear on borderline pairs", "Clustering precision" and "The possible tier at NAS scale", experiments 0059, 0186-0188 and 0230, `docs/pair-verdicts.txt` (read only), and the code of `clusters.rs` (`cluster_of`, how verdicts are read), the scoring in `sweep.rs`, `scan`, `calibrate.rs`, `recall.rs`, `grid.rs` and `loss.rs`.
2. Write docs/brief-54.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-54-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s54/listening-digests-start.txt. Experiments number from 0233.
3. Build `work/bin/s54-start`; `regress session-53-linux` and `regress session-53-linux-nas` identical.

Steps:

1. The design, written in the brief before code: how the harness knows two recordings carry the same title. Guidance: the relation must come from a judgment, not from file names or tags matched by code (AGENTS.md), and must stay apart from the clusters (a cluster is one recording; joining ƆƐI3C's upload into China Cup's cluster would change clustering and the verdicts' meaning). A plain choice: a small committed file of same-title judgments between files (or their clusters), read by the harness's scoring only, with a format, a parser and errors like the verdicts file's. Agents never write verdicts; this file holds the user's title judgments, and this session writes exactly one entry, the one the user decided on 2026-10-27 (China Cup against ƆƐI3C's upload, which the user's verdict line of 2026-10-22 names), citing the status entry. Every other candidate (for example the judged `different` pairs that the user described as the same tune in another mix: Tower Bass (Millennium Mix), Cool Rock Stuff, the Stalker version) is a question for the user, listed in the handoff, not an entry. Decide how a same-title detection is scored: right, and also reported apart (a count of same-title credits beside right and wrong), so nothing is hidden. Decide what calibrate does with them (a same-title detection is not a false candidate) and say whether any threshold of `docs/calibration.md` would move; thresholds and the product do not change in this session.
2. The ADR (the next number in `docs/adr/`), accepted on 2026-10-27 as our decision: the principle (Gunfinger identifies titles; a near-identical version found for its title is right), the file and its owner (the user), the scoring, what stays (clusters, verdicts, the search, the test set's ledger: the held-out evaluation's rules in ADR 0004 do not change, and this session runs no test evaluation). Commit it before the code.
3. The harness, tested: the file's parser (unit tests on fixtures, errors naming file and line), the scoring change in every place that judges a detection, the separate count in the human output and the JSON reports, and a test where a same-title detection is credited and a different-title one is not. `scripts/check.sh` green.
4. The differences, explained before any baseline: `regress session-53-linux` and `regress session-53-linux-nas` with the changed build. Every change must be a China Cup detection of ƆƐI3C's upload moving from wrong to right (same-title); anything else is a finding to stop on. Record what calibrate's strongest false candidate and false accepted count become at both scales.
5. Record baselines `session-54-linux` and `session-54-linux-nas` with the final build. One experiment (at most 40 lines): the wrong confident answers before and after at both scales and over the sweeps of seeds 2026-2028 (run seeds other than 2026 in a scratch work directory), and calibrate's numbers before and after.
6. Update `docs/calibration.md` where a number it quotes changed, and the roadmap's bullets that name China Cup's wrong answers (mark them with the decision; older experiments stay as written).
7. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
8. Wrap-up: a "Session 54" section at the end of docs/session-notes.md with a handoff (the new baselines, the file, the questions for the user); update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-54-linux` and `regress session-54-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the design and ADR, the wrong answers before and after, calibrate's numbers, the questions for the user, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's search and output do not change: no threshold, parameter, rule, default or output change in `gunfinger`. The harness's scoring changes as decided, with its ADR; harness additions are tested, and every `regress` difference is explained before a baseline is recorded.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. The one exception in this session: the new same-title file gets exactly one entry, the user's decision of 2026-10-27 on China Cup against ƆƐI3C's upload (step 1); every other candidate is a question for the user. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
- Scratch data goes under work/s54/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 15:00. Start no new run after about 14:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 54

- 2026-10-27, 03:07 by the Linux machine's clock (CET). The time box
  ends at about 05:10; no new run starts after that.
- `master` at `081a109`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s54/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `ce293fe6165e...`, `work/audition/answers.jsonl` `3e50aaa62181...`.
- Experiments 0001-0232; this session numbers from 0233. ADRs
  0001-0021; this session's is 0022. Test set: 4 of 5 evaluations used
  (none in this session).
- The NAS store: 40,678 records, revision `1fc6fe50293ad330`.
  Baselines `session-53-linux` (262 tracks) and `session-53-linux-nas`.
  Build `work/bin/s53-final/` (master).
- The code today: every scorer credits a detection through the truth's
  duplicate cluster alone (`Clusters::cluster_of`). The sweep's
  `Outcome.correct` is "the asset is in the excerpt's own cluster";
  `speed_row` counts a confident detection that is not `correct` (and
  not a damaged query's partner) as a wrong answer; calibrate takes
  every detection that is not `correct` as a false candidate. The scan
  (`scoring::score`) accepts the clusters of a track's references in
  its window; anything else confident is wrong. Generated mixes
  (`mixes::score`, which `grid` and `mixes` use), `recall` (through the
  sweep's `outcome` and `speed_row`), `robust`, `seconds` and `loss` do
  the same with their own truths.
- The China Cup files: the corpus has three rips (`Bad Company - China
  Cup.mp3`, `extra/02. Bad Company - China Cup.mp3`,
  `extra/b-bad_company-china_cup.mp3`), one cluster with the Prototype
  upload (a `same` verdict of 2026-10-09). ƆƐI3C's upload
  (`second-library/__youtube_archivists/sicknote-classic-dnb/ƆƐI3C -
  China Cup.opus`) is its own cluster: `docs/pair-verdicts.txt` line 84
  judges it `different` from `extra/b-bad_company-china_cup.mp3`
  (2026-10-22: almost identical, the hi-hats differ for a few bars).
  Seed 2026's panel draws no China Cup excerpt, so the baselines hold no
  China Cup query; seeds 2027 (China Cup indexed) and 2028 (held out)
  do (0188, 0230).

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build, both regressions and the start build's sweeps of seeds 2027 and 2028 at both scales (tmux `s54start`) | 03:07-03:30 |
| 1 | the design, below, before code | 03:20-03:30 |
| 2 | ADR 0022, committed before the code | 03:30-03:40 |
| 3 | `docs/same-titles.txt`, its parser, the scoring in every judging place, tests; `scripts/check.sh` | 03:40-04:20 |
| 4 | `regress` at both scales with the changed build; every difference explained; calibrate's numbers | 04:20-04:30 |
| 5 | baselines `session-54-linux` and `session-54-linux-nas`; the changed build's sweeps of seeds 2027 and 2028 at both scales; experiment 0233 | 04:30-04:45 |
| 6 | `docs/calibration.md`, the roadmap | 04:45-04:50 |
| 7-8 | review, notes, status, checklist, final checks | 04:50-05:10 |

## The design (step 1)

**What the harness knows.** A new committed file, `docs/same-titles.txt`,
holds the user's judgments that two library files carry the same track
title although they are different recordings (a near-identical version,
as the verdicts' `different` keeps them apart). It is written by hand by
the user; agents never add entries. This session writes exactly one,
the user's decision of 2026-10-27 (`docs/status.md`, "session 39's
listening questions, and four decisions", decision 4): the corpus rip
`extra/b-bad_company-china_cup.mp3` against ƆƐI3C's upload, the two
files the verdict line of 2026-10-22 names. Nothing derives an entry
from file names or tags.

**Format.** Like the verdicts file: `#` starts a comment line, empty
lines are ignored; one judgment per line, `same-title` and the two
files, separated by tabs, named as in the clusters files (corpus library
paths, or `second-library/<path>`); the order of the two files does not
matter. The parser (`titles::SameTitles`) refuses, naming the file and
the line: a line that does not start with `same-title`, a line without
exactly two files, a file judged against itself, and a pair already
judged on an earlier line. A missing file means no judgments (as for the
verdicts), so the scoring is then as before.

**Meaning.** A judgment relates the two files' duplicate clusters: a
detection of any file in one cluster, where the truth is the other
cluster, names the right title. It is not followed through chains of
judgments (A ~ B and B ~ C do not credit C for A): each pair of
recordings needs its own judgment, the conservative reading. The
clusters, the verdicts, `clusters` and every command that draws or
leaves out recordings by cluster (the sweep's held-out fifth, recall's
left-out copies, the mixes' draws) do not read the file; only scoring
does. A file a run does not index adds nothing.

**Scoring.** A same-title detection counts as right, and every report
counts it apart:

- Sweep (and `recall`, which builds its queries with the sweep's
  `outcome`): `Outcome.same_title` is true for a detection of a file the
  judgments credit to the excerpt's cluster (written only when true;
  `correct` keeps its meaning, the excerpt's own cluster). A confident
  same-title detection is not a wrong answer; on an indexed excerpt it
  counts toward recall like a detection of the own cluster.
  `SpeedRow.same_title` counts the confident same-title detections
  (written only when not zero); the human table gains a column.
- Scan (`scoring::score`): a slot accepts the clusters its references
  are credited to by title beside their own; a confident same-title
  detection in the window identifies the track and is listed in the
  track's `credited_through_title`; `Score.same_title` counts them
  (written only when not zero); they are neither wrong nor false
  candidates. The human output prints the count.
- Generated mixes (`mixes::score`, used by `mixes`, `grid` and the
  drops): the same per play, with `MixScore.same_title`.
- `robust`: a confident same-title detection is neither right for the
  own track's count nor wrong; counted apart.
- Calibrate: a same-title detection is right. In the sweep it is not a
  false candidate and may be a query's identifying detection; in scans
  it follows the score (it is in the window, so it can be a track's
  strongest candidate and never a false candidate). Calibrate prints how
  many confident same-title credits its reports hold. The detections on
  held-out excerpts (`not_indexed`, the possible tier's null on audio
  whose recording is not indexed) keep every detection, the
  conservative choice: that null is a measurement of audio, not a
  judgment.
- Thresholds: the confident rule (240 hits) and the possible tier rest
  on false candidates that are not versions of the played recording;
  the strongest false candidate and the weakest identifying detection do
  not involve ƆƐI3C's upload. No threshold of `docs/calibration.md`
  moves, and none changes in this session. (Corrected after the review:
  this said the upload's detections all lay above the rule; 6 of the 24
  that leave the false candidates lie below it, 0233.)
- Not changed, with the reason: `seconds` (time scored second by
  second, a diagnostic of spans) and `loss` (searches the corpus library
  alone and picks identified plays to measure) keep the cluster alone;
  `shared`, `shared-mixes`, `identifiability`, `holders`, `related`,
  `pair` and `clusters` ask whether two files are one recording or
  share material, which is what they measure. Listed for later in the
  handoff.

**Questions for the user, not entries.** The judged `different` pairs
that the user described as the same tune in another mix: Tower Bass
(Millennium Mix) against `a - aphrodite - tower bass.mp3` (2026-10-25),
Aphrodite - Cool Rock Stuff against the Mobb Deep / Urban Takover
upload (2026-10-22), the two Aphrodite - Stalker versions; and any
other `different` pair whose titles agree. Each is a question for the
user.

## Decisions

- **(03:10) Start build and regressions** (`work/s54/start.sh`, tmux
  `s54start`, logs in `work/s54/logs/`; `run.sh` and `same.py` are
  session 53's with the paths changed). Cargo had nothing to rebuild:
  the start build is byte-identical to `work/bin/s53-final/`. `regress
  session-53-linux` (0:51) and `regress session-53-linux-nas` (3:46,
  5.3 GiB peak) identical, every report identical to its baseline's
  apart from `wall_seconds`.
- **(03:21) The start build's sweeps of seeds 2027 and 2028**
  (`work/s54/seeds.sh`, in the scratch work directory
  `work/s54/wseeds-start` with copies of both clusters files and the
  library map and a link to the saved NAS index, so that calibrate's
  reports directories hold seed 2026 alone): at 262 tracks 0 wrong
  answers each (0:48); at NAS scale (2:26 each) seed 2027 has 11 wrong
  (`Bad Company - China Cup.mp3` 4, `extra/02. Bad Company - China
  Cup.mp3` 7, indexed) and seed 2028 7 (`extra/b-bad_company-china_cup.mp3`,
  held out), every one ƆƐI3C's upload, as in 0188.
- **(03:20) Held-out excerpts and left-out slots.** A same-title
  detection on a held-out sweep excerpt or in a left-out scan slot is
  right too, counted apart: a hold-out removes a recording, and the
  version stays indexed with the title. Leaving the judged files out as
  well would change what the harness searches, not how it scores, which
  this session does not do. No development set plays China Cup, so no
  scan or leave-out of the three development mixes is affected.
- **(03:25) Reports.** The counts and the `same_title` flags are written
  only when not zero or true (`titles::no_credits`), so reports without
  a same-title credit read byte for byte as before and `regress` shows
  exactly the detections that change. `correct` keeps its meaning (the
  own cluster); `Outcome::is_right` and `Outcome::is_wrong` hold the
  rule for the sweep, `recall` and calibrate.
- **(03:25) Calibrate's identifying detections** take the strongest
  right detection of a sweep query, a same-title one included, as a
  scan's strongest candidate in a track's window already does after the
  change; `not_indexed` (the possible tier's null) keeps every
  detection on a held-out excerpt.
- **(03:25) The harness option** `--same-titles FILE` (default
  `docs/same-titles.txt`), beside `--verdicts`. Errors read
  `<file>, line N: ...`, as `Verdicts::on`'s messages do.
- **(03:25) Recall and robust partners.** A damaged query's partner
  (recall) and a blend's second track (robust) accept the files judged
  to carry the partner's title, so neither counts as wrong; the own
  title comes first. Robust keeps its own-cluster counts (`confident`,
  `best_hits`: how a transform damages the recording's evidence) and
  counts confident same-title detections apart.
- **(03:28) The harness change** committed (`91d5e80`) after
  `scripts/check.sh` was green (3:57; the first run stopped at clippy's
  `trivially_copy_pass_by_ref` on four helpers, now one,
  `titles::no_credits`). ADR 0004's status line names the amendment;
  AGENTS.md names `titles` and the rule. ADR 0022 also states that it
  changes `docs/brief.md` section 7.6 by our decision (the brief is not
  edited) and that a later test evaluation would be scored by the same
  code (found while writing the docs, 03:31).
- **(03:33) Step 4: `regress` with the changed build** (`work/bin/s54-run`,
  `work/s54/step4.sh`, tmux `s54step4`): `regress session-53-linux`
  (0:51) and `regress session-53-linux-nas` (3:53) identical, every
  report identical to its baseline's apart from `wall_seconds`, and
  calibrate unchanged: 690, 100, 0 missed, 0 false accepted at 262
  tracks; 704, 282 (Cool Rock Stuff ~ the Mobb Deep / Urban Takover
  upload), 0 missed, 7 false accepted (the same pair) at NAS scale;
  0 same-title credits. As expected: seed 2026's panel draws no China
  Cup excerpt and no development mix plays China Cup, so no detection of
  the baselines can change. The `gunfinger` binary is byte-identical to
  the start build's.
- **(03:40) The changed build's sweeps of seeds 2027 and 2028**
  (`work/s54/wseeds-run`): at 262 tracks identical to the start build's;
  at NAS scale 16 (seed 2027) and 8 (seed 2028) detections of ƆƐI3C's
  upload gain `same_title` and nothing else differs
  (`work/s54/sweepdiff.py`); wrong 11 -> 0 and 7 -> 0, recall 540 each
  as before. Calibrate over seeds 2026-2028 and the Stakka scans
  (`work/s54/calib.sh`, the seed 2026 sweep and scans copied from the
  session-53 baselines): at NAS scale false accepted 25 -> 7, false
  candidates 130,663 -> 130,639, at 30 hits or more 308 -> 284,
  strongest false 282 and weakest identifying 675 unchanged, 18
  same-title credits; at 262 tracks unchanged.
- **(03:40) Baselines** `session-54-linux` and `session-54-linux-nas`
  saved from the reports of step 4's regressions (`work/bin/s54-run`,
  the harness commit), identical to the session-53 baselines
  (`work/s54/step5.sh`). The final checks run `regress` against them
  with the final build.
- **(03:42-03:50) The read-only review** (a separate agent, no further
  agents) confirmed every number of 0233, the rules (one entry, the
  verdicts file unchanged, no change outside the harness, no AI
  attribution) and the asset naming, and found:
  1. ADR 0022 and `docs/calibration.md` counted 0059's 16 (18) wrong
     answers as ƆƐI3C's; 0059's data names the PRO 001 upload, joined
     into China Cup's cluster by the `same` verdict of 2026-10-09
     (0230's remark is mistaken, and so was the prompt's count).
     Corrected.
  2. The ADR and this brief said every ƆƐI3C detection lay above the
     rule; 6 of 24 lie below. Corrected.
  3. The ADR grouped `robust` with the scorers that credit; the roadmap
     and AGENTS.md said "every scorer". Qualified.
  4. `grid` carried no same-title count: `GridQuery.same_title` and a
     summary line added.
  5. No test of `sweep::outcome` setting the flag (with a padding copy)
     or of recall's partner title: both added (`partner_files`
     extracted). Robust's and grid's branches stay inline in their runs,
     without a unit test.
  6. A misspelt judged file would credit nothing silently: every scoring
     run now warns of a judged file it neither indexes nor leaves out
     (`SameTitles::warn_outside`); the committed-file test checks the
     China Cup entry instead of the count of entries.
  7. Calibrate's credit count used the stored `confident` flag: it uses
     the recomputed evidence now; its module comment says it reads the
     judgments from the reports.
  8. Possible plays of a judged file now count toward the track:
     documented in the ADR (not counted apart).
  9. The scan's count could count one detection twice (also found
     before the review; fixed: distinct detections, as in mixes); a
     stale doc comment; the analysis scripts that read `correct` are
     listed in the ADR as unchanged.
  10. The checklist and CLAUDE.md were behind: updated.
  11. "The user" for our judgments: "we/our" now.
