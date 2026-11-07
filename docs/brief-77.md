# Session 77 brief: two harness checks that measure what the sessions assume

Written on 2026-11-07, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it), `AGENTS.md` and
   `docs/session-rules.md`.
2. This file, and `docs/session-77-checklist.md` (progress).
3. `docs/research/2026-11-05-external-review.md`, "Harness findings,
   confirmed in the code", items 1 and 2 (why this session runs), and
   `docs/decisions-queue.md`.
4. The code: `crates/gunfinger-eval/src/mixes.rs` (`score`,
   `score_play`, `print_summary`), `counterexamples.rs` (`Outcome::of`
   and its test), `grid.rs`, `mixes/residual.rs`, `regress.rs`,
   `sweep.rs`'s report types and `commands/evaluation.rs`.

## The prompt, verbatim

```text
Start session 77 of Gunfinger: make two harness checks measure what the sessions assume they measure. The external review of 2026-11-05 (`docs/research/2026-11-05-external-review.md`, "Harness findings, confirmed in the code", items 1 and 2; the roadmap's "The active queue", item 1) found, by reading the code: (1) generated-mix scoring takes a play's level from its strongest matching detection by hits (`crates/gunfinger-eval/src/mixes.rs`, `score_play`), so a play that has a confident detection can be scored as possible when a stronger-by-hits detection covers fewer windows; `counterexamples.rs` works around this and its test builds exactly that case, and experiment 0266 records a failure that changed after the counterexample runner was corrected; (2) `regress` compares the sweep's queries and speed rows by position (`regress.rs`, `compare_sweeps`, `zip`) without checking their counts or identity, so a shorter report can print "N of N queries with identical detections", its baselines carry no provenance, and it returns success whatever it finds. Neither finding has been reproduced by running anything. This session fixes both in the harness, measures what the scoring fix changes in saved reports, and keeps the product unchanged. Session 78 follows with the render cache, the harness's FFmpeg watchdog and `regress`'s coverage.

You run unattended: the user cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).

Time box: started about 13:08 on 2026-11-07; aim to finish within about 2.5 hours, because more sessions follow you tonight. If a step would run past that, scale it down (keep both fixes and their tests; rescore the reports of experiments 0255-0275 first and list the rest), record the decision in the brief and continue.

Environment (sessions 33-76's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,666 records, revision `0bd3f0c23fabc698` (session 75 pruned the records of DJ sets and a mixed CD's track fix). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-75-linux-nas` (NAS scale); a session that adds baselines names them in its handoff, and the next prompt says which to use.
- Build `work/bin/s75-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Since `45f92d5` (2026-10-28) its calibrate reads only the reports its baseline holds; still run variants and other seeds in a scratch work directory, never in the reports directory `regress` compares.
- The user restructured the code on 2026-10-28 without meaning to change behaviour (commits `b5a84e3` to `f94604a`: harness subcommands in modules, named results, `Search` passed through the core's search, enums for closed sets). Session 60 confirmed both regressions identical on a build of master after it.
- Since sessions 53-56: `SpansAtSpeed(0.006)` is the default second suppression (ADR 0021); same-title detections (`docs/same-titles.txt`) are scored right (ADR 0022); `identify` prints gaps as "ID - ID" lines with an estimate of their tracks (ADR 0023, 0024). The minimum span stays undecided. A change to an existing decision needs a new ADR (the next number is in `docs/adr/`).
- Since 2026-11-05: `docs/session-rules.md` holds the standing rules (the older briefs are history), `docs/decisions-queue.md` the decisions waiting for the user, and the roadmap begins with "The active queue" (`docs/research/2026-11-05-external-review.md` explains why). A session that leaves a decision for us adds it to the decisions queue. The six proposals of `docs/research/2026-10-29-matching-and-archive-proposals.md` have all been measured; a session working on one follows its "Evaluation and adoption boundaries".
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- Tonight (2026-11-07 to 11:00 on 2026-11-08) sessions run one after another, unattended; the user cannot decide or listen. Every listening question asked so far has an answer (docs/status.md, the entries of 2026-10-31 and 2026-11-02), apart from the Planet Dust (DC Breaks Remix) cut (unsure). The user's decisions on a quiet track under an identified one are in docs/status.md, "2026-11-02: the user's answers on a quiet track under an identified one". When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.
- The repository root holds an untracked `yafc20261031.log` that is not this project's work: leave it alone, never commit or delete it; "`git status` clean" means clean apart from it.

First:
1. Read CLAUDE.md and follow its reading list (session 76 rewrote it, and the roadmap, and moved the older notes and status entries under `docs/history/`; the index at the top of `docs/session-notes.md` says where each session's section lives). Then the review note, ADR 0013 and 0014, experiments 0264-0266 and 0273-0275, notes "Session 68", and the code of `mixes.rs` (scoring and the recall summary), `counterexamples.rs` (the workaround and its test), every consumer of `PlayScore` and its `level`, `regress.rs`, `sweep.rs`'s report types and `commands/evaluation.rs`.
2. Write docs/brief-77.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-77-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s77/listening-digests-start.txt. Experiments number from 0279.
3. Build `work/bin/s77-start`; the start regressions follow docs/session-rules.md (skip them if the build is byte-identical to the build named in the environment below and the store is unchanged, and say so).

Steps:

1. Pre-registration, in the brief before the fix: the rule (a truth play's level is the highest confidence among its matching detections; the representative detection for start, speed and hits stays the strongest by hits unless a stated reason says otherwise, and the report keeps both); which saved reports are rescored (every generated-mix report a committed experiment cites that can be rescored from its saved detections without searching again; list them, with the experiments that cite them); what counts as a changed conclusion (any figure an experiment or an ADR states that moves), and how each is recorded.
2. The scoring fix, tested: a unit test with the two-window and five-window case at the common scorer; the counterexample runner's workaround removed if it becomes redundant (its test kept and passing); every consumer of `level` checked. An offline `rescore` path (or an option of an existing subcommand) that reads a saved report and writes the rescored one beside it, never over it.
3. The rescoring: run it on the listed reports; a table per experiment of the figures before and after. Where a figure that an ADR or a registered bar used moves, say whether the decision's reading changes; write an amendment line into that experiment file only if a stated figure is wrong, and never change a registered bar or a verdict.
4. `regress`, tested: before comparing, check that the sweep's query lists match in count and identity (asset, start, speed, held-out status) and that the scan reports describe the same set and leave-outs; print a mismatch as "incomparable" apart from "changed" and "identical". A provenance file written by `regress save` (build digest, NAS and corpus store revisions, the digests of the clusters file, `docs/pair-verdicts.txt`, `docs/same-titles.txt` and the panels, the profile and the matcher settings) and compared by `regress compare`; baselines without one stay readable, with a one-line note. A `--strict` option that exits non-zero on any difference or incomparability, for refactors; the default keeps today's behaviour. Unit tests: a deleted query, reordered queries, changed truth with unchanged detections, a missing provenance file.
5. Final regressions with the final build: `regress session-54-linux` and `regress session-75-linux-nas` identical (run once more with `--strict`); then save provenance for both baselines, if that can be done without changing their reports (write the provenance beside them and say so in the handoff; if it cannot be done safely, save new baselines `session-77-linux` and `session-77-linux-nas` and name them in the handoff).
6. Experiment (at most 40 lines): what the scoring fix changes in the saved reports, by experiment.
7. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
8. Wrap-up: a "Session 77" section at the end of docs/session-notes.md with a handoff (how to run `regress --strict`, the baselines to use, the rescored reports' paths); update docs/status.md, the roadmap's active queue (item 1's state), docs/decisions-queue.md if a decision's evidence moved, and the checklist. Final checks: `scripts/check.sh` green, `regress session-54-linux` and `regress session-75-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the scoring fix and what it changed in saved reports, the `regress` checks, the baselines to use next, and anything left.

Rules for this session:
- `docs/session-rules.md` binds as well; where it and this list differ, this list wins for this session.
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's behaviour does not change: no threshold, parameter, rule, default or output change; `identify --under` stays off by default (the default is ours, in ADR 0025). Harness changes and variants are tested, opt-in where they are variants, and keep `regress` identical.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
- Scratch data goes under work/s77/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00 on 2026-11-08. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 77

- 2026-11-07, 13:07 by the Linux machine's clock (CET). The time box is
  about 2.5 hours: aim to finish by about 15:40.
- `master` at `786d056`, clean (apart from the untracked
  `yafc20261031.log`, which is not this project's work). Git identity
  `Jacek Galanciak <jacek.galanciak@gmail.com>`.
- Digests (`work/s77/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `c4d3da2e7e37...`, `work/audition/answers.jsonl` `3e99ebca589b...`.
- Stores (`scripts/analysis/store_revision.py`,
  `work/s77/store-revision-start.txt`): NAS 40,666 records, revision
  `0bd3f0c23fabc698`; corpus 265 records, revision `526a3b978eb3e411`;
  no file in `work/peaks` is newer than `work/bin/s75-final/`.
- Start build `work/bin/s77-start/` (master at `786d056`): both
  binaries byte-identical to `work/bin/s75-final/`'s (SHA-256
  `46fdc5eac4a7...` for `gunfinger`, `7c221bbceca6...` for
  `gunfinger-eval`), and the store revisions are unchanged, so the start
  regressions are skipped (`docs/session-rules.md`, "Runs").
- Experiments 0001-0278; this session numbers from 0279. ADRs
  0001-0025. Test set: 4 of 5 evaluations used (none in this session).
- What the code does today (read at `786d056`):
  - `mixes::score_play` takes `best`, the matching detection with the
    most hits, and the play's `level` from `best.confidence()`. A play
    with a confident detection (3 windows or more, ADR 0013's rule for
    segments) scores possible when a detection with more hits spans
    fewer windows. ADR 0013 made the product's plays take the highest
    confidence of any segment in 2026-10-11 and says "the evaluation
    harness already used maximum confidence"; the generated-mix scorer,
    written later, does not.
  - Consumers of `PlayScore::level`: `mixes::print_summary` (the
    counts by level and the confident count per length band),
    `grid.rs` (`GridQuery::level`, the brief play's level),
    `mixes/residual.rs` (the pilot's cases are the confident indexed
    plays; `ArmPlay::level`), `counterexamples.rs` (`Outcome::of`
    overrides the level with "any confident detection of the accepted
    files overlaps the target", the workaround of session 68).
    `residual::detected_reference` places the reference from `best`.
    The scan scorer (`scoring::score_track`) already counts a track
    identified when any detection in its slot is confident.
  - `regress::compare_sweeps` zips the baseline's and the new sweep's
    `queries` and `per_speed` and prints "N of M queries with identical
    detections" with M the new report's count; nothing checks that the
    lists have the same length or the same queries. `compare_scans`
    compares scores and detections without checking set or leave-outs.
    `gunfinger-eval baseline NAME` copies the four reports and nothing
    else; `regress` returns success whatever it prints.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief, checklist, `CLAUDE.md` | 13:07-13:30 |
| 1 | pre-registration of the rule and the rescoring, committed | 13:30-13:50 |
| 2 | the scoring fix, its tests, `rescore` | 13:30-14:15 |
| 3 | rescoring the listed reports; tables | 14:15-14:40 |
| 4 | `regress`: identity checks, provenance, `--strict`, tests | 14:15-14:55 |
| 5 | final regressions; provenance for both baselines | 14:55-15:10 |
| 6 | experiment 0279 | 15:10-15:20 |
| 7 | read-only review by a subagent; fixes | 15:10-15:30 |
| 8 | wrap-up, final checks | 15:30-15:45 |

## Decisions

- **(13:10) `regress save` is `gunfinger-eval baseline`.** The prompt's
  "a provenance file written by `regress save`" is read as the
  `baseline` subcommand, which saves the reports `regress` compares.
- **(13:12) Inventory of saved generated-mix reports.**
  `work/s77/find_reports.py` found 202 reports under `work/` that parse
  as a `mixes` report (`work/s77/report-inventory.txt`, 541 MB) and
  three grid reports in `work/s57/`; no committed file in
  `docs/experiments/data/` is a whole report with detections. A
  read-only subagent maps each report to the experiments that cite it
  (`work/s77/report-map.tsv`); the list goes into the registration.

## Pre-registration (step 1), before the fix is committed and before any rescoring

Registered at 13:18 (`3c93455`), before `rescore` had read any saved report (its first run started at 13:19:32).

- **The rule.** A truth play of a generated mix takes the highest
  confidence among its matching detections (ADR 0013's rule for plays):
  confident when any matching detection is confident, else possible
  when any is possible, else weak when any matches, else none. A
  detection matches a play as today: its file is in the play's cluster
  or judged to carry its title, and it overlaps the play's audible span.
  The representative detection (`best`: position error, speed, the
  residual pilot's placement) stays the matching detection with the
  most hits, because it holds the most aligned evidence. The report
  keeps both: `best`, and `at_level`, the detection with the most hits
  at the play's level, written only when it is not `best`. The old
  level stays readable as `best`'s confidence.
- **Where.** `mixes::score_play`, which `mixes`, `grid`, the residual
  pilot and `counterexamples` share. `counterexamples`' override in
  `Outcome::of` becomes redundant (its accepted files are the scorer's
  cluster and title files, and its overlap test is the scorer's) and is
  removed; its test is kept, rewritten to score through `mixes::score`.
  `grid`'s `GridQuery::level` and the residual pilot's case selection
  (confident indexed plays) follow the scorer. The scan scorer already
  counts a track identified when any detection in its slot is confident
  and does not change.
- **Reports rescored.** Every report in `work/s77/report-inventory.txt`
  (202 `mixes` reports) and the three `grid` reports of `work/s57/`
  (`wgrid32/reports/grid-seed-2026.json`,
  `wrobust/reports/grid-seed-2026.json`,
  `wrobust/reports/variant-spans/grid-seed-2026.json`), with
  `gunfinger-eval rescore`, which reads the saved detections and
  searches nothing. Clusters and same-title judgments are today's
  (`work/reports/duplicate-clusters.json` for reports of the corpus
  index, with `--other-peaks-dir` the NAS clusters for reports of an
  index of more than 1,000 assets). The experiments that cite each
  report are mapped in `work/s77/report-map.tsv` (a read-only
  subagent's map, checked against the experiments' commands); the
  tables cover the reports mapped to a committed experiment, 0255-0275
  first. Not rescorable without searching again, and listed as such:
  the residual pilot's arms (they keep each play's strongest detection
  only), its choice of cases, `counterexamples` reports (their
  outcomes were scored with the override, which gave the registered
  definition; nothing to rescore), `shared-mixes` and `passages`
  reports (their own scorers, which do not read `PlayScore`), and
  `identify` reports of rendered mixes (scored by
  `scripts/analysis/tracklist_score.py` from the product's plays, which
  take the highest confidence already).
- **Two effects kept apart.** `rescore` counts, per report, the plays
  whose level changed against the saved report, the plays whose
  rescored level is above their strongest detection's (the rule's own
  effect) and the plays or mixes that drifted for another reason (a
  different strongest detection, boundaries, wrong answers or credits
  under today's clusters, judgments or thresholds). Only the rule's
  effect is attributed to the fix; drift is reported beside it.
- **A changed conclusion.** Any figure that a committed experiment or an
  ADR states and that the rescoring moves, by any amount. Each is
  recorded in experiment 0279's table, by experiment, before and after.
  Where the figure was one an ADR or a registered bar used, 0279 and
  this brief say whether the decision's reading changes. An amendment
  line goes into the experiment file only where a figure it states is
  wrong under the registered rule it measured (it counted "confident"
  plays that had a confident detection as possible); registered bars
  and verdicts are never changed.

## Step 3: the rescoring (13:19-13:23)

- **(13:19) The run.** `work/s77/rescore.sh` with `work/bin/s77-run/`
  (master at `0bcfa44` plus the drift check's tolerance, committed
  with step 3): 184 reports with the corpus clusters, 21 with
  `--other-peaks-dir` and the NAS clusters, 4 s in all. Each rescored
  report is `<stem>-rescored.json` beside its saved one
  (`work/s77/rescored-files.txt`).
- **(13:20) Decision: a tolerance in the drift check.** The first run
  counted 23-155 drifted plays in nearly every report: each was a
  boundary or position error computed again from floats that serde_json
  reads one unit in the last place off (0265). Seconds now compare
  within 1e-9 s; with it no play and no mix drifted in any of the 205
  reports, so every saved level equals its strongest detection's under
  today's clusters, judgments and thresholds, and every change below is
  the rule's.
- **(13:21) What moved.** 6 plays in 5 reports, all possible to
  confident, all at NAS scale in session 62's stereo-fault mixes of
  seed 2028 (0250): Bad Company - Oxygen (0:00-0:24 of mix 02; strongest
  detection 521 hits in 2 windows, a confident one of 487 hits in 3; under
  power-sum queries 702 and 495) in the control and inverted mixes under
  both front ends, and Balderdash (5:26-5:53 of mix 06, 329 hits in 2
  windows against 250 in 3) in the inverted mix under power-sum queries;
  the sixth is the same Oxygen play in `work/s62/check/nas-2028-run.json`,
  an identity check no experiment cites. The other 199 reports, the three
  grid reports among them, change in no play.
- **(13:21) The tables, by experiment.** 38 experiments cite at least
  one report (`work/s77/report-map.tsv`; 197 of 205 reports mapped).
  For 37 of them no play's level moves, so no figure they state moves:
  0159-0166, 0175-0179, 0181, 0182, 0184, 0185, 0198, 0210, 0211,
  0215-0217, 0235, 0237, 0239, 0240, 0245, 0246, 0255-0258, 0269-0271
  and 0274. For 0250, `scripts/analysis/query_side_bar.py` on the saved
  reports gives its committed figures exactly, and on the rescored ones:

  | 0250, NAS scale, plays confident | Downmix before; after | Power-sum queries before; after |
  |---|--:|--:|
  | Control: first half, across, second half | 91/99, 23/24, 95/100; 92/99, 23/24, 95/100 | the same plays |
  | Inverted from the middle: the same | 91/99, 16/24, 1/100; 92/99, 16/24, 1/100 | 91/99, 23/24, 93/100; 92/99, 23/24, 94/100 |
  | Lost under the variant; new wrong files | 0; 0 | 0; 0 |

  Brief 62's bar items 1, 3, 5 and 6 pass as before (the plays gained
  are gained under both front ends, or by the variant), and 0249 still
  fails item 2: no decision's reading changes. 0250's figures were
  wrong under the rule it measured (confident plays), so it gains an
  amendment, appended to its last line to keep the file at 40 lines.
- **(13:22) Not rescorable without searching again.** The residual
  pilots of 0255-0258 and 0269-0271 (22 reports): their cases are the
  confident indexed plays, and no such play's level moved, so the cases
  are the ones the pilots ran; but their arms keep each play's strongest
  detection only. Of 60,337 arm plays, 463 are below confident with a
  strongest detection of 240 hits or more in fewer than 3 windows (0255:
  52, 0256: 27, 0257: 23, 0258: 25, 0269-0270: 250, 0271: 86); for those
  a weaker confident detection may exist and only a search can tell.
  This is an upper bound on the arm plays the rule could move, not a
  count of figures that move. Also as registered: `counterexamples`
  (0264-0266; 0266 already scored the registered definition),
  `shared-mixes`, `passages` and `identify` reports.
- **(13:22) The glob hazard.** Scripts of earlier sessions glob report
  directories (`query_side_bar.py` reads `mixes-stereo-*-seed-*.json`),
  and the rescored copies match those patterns. They are kept beside
  their reports as the prompt asks; `work/s77/rescored-files.txt` lists
  all 205 for deletion, and the handoff says so.

## Step 4: `regress` (committed in `1ee36e1`)

- **(13:24) What "incomparable" means.** A sweep pair is incomparable
  when the seeds differ, the speed rows differ, or the queries differ in
  count or in any query's asset, start, speed or held-out status, in
  order (a reordered list is incomparable: the comparison is
  positional). A scan pair is incomparable when the set, the audio
  file, the left-out tracks, slots or files, or the manifest's tracks
  (label, start, referenced) differ. Other differences are "changed";
  a pair is "identical" only when the whole reports are equal apart
  from `wall_seconds` (the check `same.py` made by hand since session
  36, now in `regress`). A query whose detections are unchanged apart
  from how they were scored (right, same title, partner) is changed,
  and counted on its own line.
- **(13:25) Provenance.** `gunfinger-eval baseline` writes
  `provenance.json` beside the reports: the SHA-256 of the running
  `gunfinger-eval`, the set and seed, the corpus store's and (with
  `--other-peaks-dir`) the other store's current records and revision
  (`indexing::library_revision`, which `store_revision.py` reproduces),
  the SHA-256 of the clusters file the run reads, of
  `docs/pair-verdicts.txt`, of `docs/same-titles.txt` and of a listing
  of the panels' files, the profile, `hash::design()`, the ladder,
  extra rungs, the matcher's options (their `Debug` form) and
  `--verify`. SHA-256 is computed by a small module of the harness
  (`digest.rs`, tested on the standard's vectors), since no crate in the
  tree provides it; its digests equal `sha256sum`'s (checked on the
  build, the clusters file, both documents and the panels listing).
- **(13:26) Decision: the build is not an input.** `regress` prints a
  different build on its own line and does not count it: a refactor
  changes the build by design, and `--strict` exists for refactors.
  Every other field that differs makes the comparison incomparable.
- **(13:26) Decision: `--strict`.** Exits with an error on any changed
  or incomparable pair and any differing input; without it `regress`
  prints the same lines and returns success, as before.
- **(13:27) Decision: provenance beside the old baselines.** `baseline
  --provenance-only NAME` writes only `provenance.json`, marked
  `after_the_fact`, and is refused when the file exists or when any
  current standard report differs from the baseline's apart from
  `wall_seconds`. It then records the build and inputs that reproduced
  the baseline's reports, not those of the session that saved them
  (sessions 54 and 75); the reports are not to be touched, and their
  SHA-256s are to be checked before and after
  (`work/s77/baselines-before-provenance.txt`; the outcome is under
  "Step 5").
  This is safe in the sense of the prompt, so no new baselines are
  saved.

## Step 7: the read-only review (13:32-13:40) and its fixes (`82dc9c6`)

A subagent read the commits and documents (no runs, no agents). It
found no bug in the scorer, `regress` or the SHA-256 code, and confirmed
that the product's crates are unchanged, that 0279's and 0250's numbers
match the data, and that every test the prompt asked for exists. What
it found, and what was done:

- `scripts/analysis/span_rule.py` (`mix_score`, `grid`) and
  `span_recall.py` still gave a play its strongest detection's level, so
  the minimum-span analyses scored by the old rule and would report
  false "stored, computed" problems on new reports. Fixed: the highest
  level, and the stored check accepts either rule. (The Rust consumers
  had been checked; the Python ones had not.)
- `rescore`'s drift check counted every raised play as drift on a report
  written by today's scorer. Fixed: a saved level of either rule is
  accepted; a test rescoring a rescored report expects no change.
- `baseline --provenance-only` could not tell reports of another build.
  Fixed: it refuses reports older than the running executable. Full
  `baseline` keeps its help text ("save it with the build that wrote the
  reports").
- The panels walk could follow a symbolic link into a cycle. Fixed.
- Wrong clock times in this brief (the registration read as later than
  the rescoring), a doc comment of `Verdict::changed`, the untracked
  report lists and the 24 mappings marked "likely" (now stated in
  0279), the arm bound without a script (now `work/s77/arm_bound.py`,
  output `work/s77/arm-bound.txt`) and the grid reports' saved matching
  lists (now stated in `rescore.rs` and 0279). Fixed.
- **Kept, decisions (13:45).** `pair_verdicts` and the whole panels
  directory stay provenance inputs, as the prompt lists them, although
  the standard evaluation reads the verdicts only through the clusters
  file and one panel only: a verdict or another panel's change makes
  `regress` incomparable. The matcher's settings stay in their `Debug`
  form, so a refactor that adds a field to `Matching` makes `--strict`
  fail on the provenance; the printed line names the field, and a new
  baseline then follows. The rescored `score` is written from today's
  structure (stated in `rescore.rs`). The 0250 amendment stays one long
  last line, to keep the file at 40 lines. Not unit-tested: the exit
  path of `--strict` in `commands/evaluation.rs` (it is run in step 5)
  and a changed manifest track in a scan (the check is one comparison
  beside the tested leave-out check).
- The fixes change the harness, so the final queue ran again with a
  new final build (`work/s77/final2.sh`); the first queue's runs with
  the pre-fix build (`check.sh` green in 7:55; both regressions
  identical) are kept in `work/s77/logs/first-final/`, and it was stopped
  before its strict and provenance steps.

## Step 5: the final regressions and the provenance (13:43-14:01)

- `work/s77/final2.sh` with `work/bin/s77-final/` (master at `82dc9c6`;
  `gunfinger` SHA-256 `46fdc5eac4a7...`, byte-identical to
  `s75-final`'s; `gunfinger-eval` `12a2d5426aa3...`): `scripts/check.sh`
  green (7:42); `regress session-54-linux` (0:51) and `regress
  session-75-linux-nas` (3:47) end "every report equal to its baseline's
  apart from timing", and `same.py` agrees for all eight reports; both
  again with `--strict` (0:51, 3:48) exit 0. `rescore` under the final
  build prints what the first run printed, for all 205 reports.
- `baseline --provenance-only` wrote `provenance.json` beside
  `session-54-linux` and `session-75-linux-nas`; the eight reports'
  SHA-256s are unchanged (`sha256 -c` of
  `work/s77/baselines-before-provenance.txt`: all OK). `regress
  --no-rerun --strict` against both then compares the provenance and
  exits 0 with no provenance line (same build, same inputs).
- The failure path: `regress --no-rerun --strict session-64-linux-nas`
  (the baseline before session 75's prune; it only reads) exits 1 and
  lists the four reports as changed, with "689 of 720 queries with
  identical detections", as 0278 recorded
  (`work/s77/logs/strict-negative.log`).
- No decision's evidence moved (step 3), so `docs/decisions-queue.md`
  does not change.
