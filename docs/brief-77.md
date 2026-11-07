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
