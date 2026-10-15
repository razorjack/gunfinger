# Session 21 brief: a like-for-like test of shared-material statistics in rendered mixes

Written by the agent for itself on 2026-10-15, so the plan survives
context compaction. Work autonomously and never stop to ask. When
something is ambiguous, take the conservative option, write the decision
down and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-21-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 20" to the end).
4. `docs/status.md` (newest entries at the bottom).
5. `docs/research/2026-10-14-matching-proposals.md` (proposal 5, and
   proposal 2 for step 6); the roadmap's "A confidence statistic for
   shared material"; ADRs 0004, 0006, 0008, 0014, 0017, 0019, 0020;
   experiments 0020, 0091, 0107, 0109-0115 and this session's.
6. `crates/gunfinger-eval/src/mixes.rs`, `shared.rs`, `verifier.rs`,
   `identifiability.rs`; `scripts/analysis/shared_panel.py`,
   `shared_outcomes.py`, `shared_identifiability.py`,
   `shared_true_plays.py`.
7. The panel `docs/panels/shared-material-seed-2026.json`.

`docs/brief-2.md` to `docs/brief-20.md` hold rules that still apply
unless this brief changes them (among them: edit no configuration file;
test fixtures are generated, never copied from the corpus or the NAS;
runs longer than 30 minutes start detached under `screen`; heavy runs one
at a time; NAS-scale harness runs overwrite
`work/reports/library-nas-dnb-peaks/`, so copy it first; each timing
series records the load and the busiest processes).

## The prompt, verbatim

```text
Start session 21 of Gunfinger, a morning session on precision that continues session 20: test, like for like, whether any candidate statistic separates a related version's wrong confident play over a shared passage from true confident plays, in rendered mixes. It runs unattended from about 05:50 and must finish by 11:59. The NAS is mounted at /Volumes/atlas. Work autonomously and never stop to ask. When something is ambiguous, take the conservative option, write the decision down and continue.

Context.
- Session 20 (experiments 0110-0115, notes "Session 20") built the shared-material panel (docs/panels/shared-material-seed-2026.json; 12 families from the judgements by ear, split by family with seed 2026 into a development half: Dozer, Fire, Playground Shades, Pyro, Sonar, Watermelon; and a validation half that has never been searched). At NAS scale, with the played recording left out, a related version is a confident wrong title in 106 of 218 development queries (12 of 14 directions), once about a minute of the shared passage is in the query. Proposal 5's diagnostic (0112): in 10 of 14 directions the distinguishing information survives into peaks and pairs but the confidence rule ignores it (outcome 3); Pyro's 30 s passage carries none (outcome 1).
- What is not settled (0114, 0115): true confident plays in the three development mixes are as sparse as the wrong ones, so a sparseness cut keeping 95% of them rejects only 8-12 of the 106; but that comparison is not like for like (excerpts against whole mixes), and the development mixes hold no wrong confident related play even with every credited track left out. Session 20's recommendation, which this session carries out: render mixes that play a related version's family over a long shared passage, then measure the candidate statistics on them against true plays in the same mixes, with the decision rule written first.
- The candidates named so far: (a) sparseness of the evidence (0114's measures: hits per second of the span, the peak verifier's share); (b) where the related file's alignment stops while the music continues (seen with Fire): the file's own material beyond the shared passage, at the play's alignment, finds nothing in the mix although the mix plays on; (c) a map of passages that indexed files share, built only from the files present after the leave-out (the research note's review: never from files left out), marking a confident play whose passage another indexed version also holds. Reporting ambiguity is the fallback if nothing separates; any change to the report is our decision (ADR 0017).
- State at the start (checked at 05:39): git clean at 8537226; the NAS peak store holds 38,105 records, revision b285da9984422fb5; the harness's saved NAS index was rebuilt by session 20 for that revision; the disk has 61 GB free; files may be added to the NAS during the session. Session 20's `shared` runs peak at 5.4 GB of memory.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it), docs/brief-20.md and docs/session-20-checklist.md (brief-2 to brief-19 hold rules that still apply unless this prompt changes them), docs/session-notes.md ("Session 20" to the end), docs/status.md (newest entries at the bottom), the research note's proposal 5, the roadmap's "A confidence statistic for shared material", ADRs 0004, 0006, 0008, 0014, 0017, 0019 and 0020, experiments 0020, 0091, 0107 and 0110-0115, crates/gunfinger-eval/src/mixes.rs, shared.rs and the `identifiability` command, and scripts/analysis/shared_panel.py, shared_outcomes.py, shared_identifiability.py and shared_true_plays.py.
2. `git status` should be clean at 8537226. Write docs/brief-21.md (this prompt verbatim, the state at the start, the plan with times) and docs/session-21-checklist.md. Point CLAUDE.md's reading list at them; keep brief-20 among the older briefs. Commit.
3. Build with --release and copy the binaries to work/bin/s21-start/. Run `gunfinger-eval baseline session-21-start`. Record the NAS peak store's state (38,105 records and revision b285da9984422fb5 expected) and the load.

Steps:

1. Setup (20 min): the steps above.

2. Pre-registration (about 30 min, before anything is rendered). Write into docs/brief-21.md and commit: the exact definition of each candidate (a), (b) and (c), computed the same way for every confident play; the oracle (session 20's: a confident play of any rip of the played recording is right; of the related recording, another version of the family or any other file is wrong; with the played recording left out, possible only or nothing is right); the denominators (true confident plays, wrong related confident plays, each counted per play); and the decision criterion, the research note's proposed feasibility test: a candidate is promising on the development half only if one threshold keeps at least 95% of true confident plays and rejects at least half of the wrong related confident plays. Name any combination of candidates you will test now; none may be added after seeing the data.

3. Rendered mixes (about 1.5 h, until about 08:15). Generated mixes as the `mixes` harness makes them (crossfades, turntable speeds, its realism), in which each development-half played recording with a shared passage plays long enough to cover that passage and distinctive material on both sides (the whole file where it fits, or at least 3 minutes), among other tracks the harness draws, with fixed seeds. Search at NAS scale (`--other-peaks-dir`) with the played recording's whole cluster left out while its related versions stay indexed (the main case), and again with everything indexed. True plays: every other track of the same mixes, plus the positive control where a family has one (the played file left out, another rip kept). Extend `mixes` minimally if it cannot place NAS files or leave out per mix; tested, harness only. Never place a validation-half recording or one of the five test-derived versions.

4. Measurement (about 1.5 h, until about 09:45). Score every confident play by the oracle; compute (a), (b) and (c) for each; apply the pre-registered criterion to each candidate and to each pre-registered combination. Add 0114's true plays from the three development mixes as a second sample of true plays, reported apart. An experiment file (next number after 0115) with the counts, the distributions and the decision.

5. The outcome (about 15 min). If a candidate passes, freeze it in writing (definition and threshold) in the experiment and the notes, and say what validating it on the panel's validation half would need. Do not search the validation half in this session: whether the next session validates is our decision. If none passes, say so plainly; reporting ambiguity remains the fallback, our call.

6. If time remains (until about 10:30): the first stage of the FFT lengths' retrieval study. Build session 19's first-pass variant ("small prime factors, within 2 samples", experiment 0109) in a worktree that is never merged, run the panel's development half at NAS scale with it, and compare the outcomes query by query with 0111's: wrong confident titles added or removed, right ones lost. An experiment file. This measures; it adopts nothing.

7. A read-only review of the session's code, pre-registration and experiments by a subagent, starting by 10:45; fix what it finds.

8. Wrap-up (from 11:15): a "Session 21" section at the end of docs/session-notes.md (the rendered mixes, the pre-registered rule, each candidate's result against it, what it means for our decision between a statistic and reporting ambiguity, step 6 if run); update docs/status.md, the checklist, the roadmap ("A confidence statistic for shared material"; the FFT lengths' item if step 6 ran), and the research note (proposal 5's status line). `regress session-21-start` identical with the final build; scripts/check.sh green (with rustup's toolchain first on PATH, so the Linux clippy pass runs, as session 20 found); the NAS store's revision. Commit. Last, print what was done, the checks, and anything left.

Rules for this session:
- Hard end 11:59. If a step overruns by more than 20 minutes, step 6 goes first. The review starts by 10:45 and the wrap-up by 11:15 whatever is left. An unfinished step is reverted, or committed only in a working, tested state, and named in the notes.
- The panel's validation half stays unsearched, as does the recall panel's validation half. The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or reports, rescore nothing, and never run test evaluation 5. Never identify, decode or index EM_1999_02_07_-_Ed_Rush_Optical.mp3 at the NAS root. Run nothing on the candidate mixes of experiment 0065, including the NAS's sety/vertical.mp3.
- The five test-derived versions are never used to tune anything: Terminal 2, the Orders remixes, the Marky & Bungle Funktion, the Clear Skyz Break remix and 1996 VIP, and the Black Barrel Beachball.
- The NAS (/Volumes/atlas/Music/dnb) is read, never written; its .gunfingerignore is edited only by hand. The NAS peak store is read-only: no `index`, `prune` or `--retry-skipped` against it. Index only into temporary stores. Check its revision at the end; if it changes during the session, write it down and compare only runs made at the same revision.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt and work/audition/answers.jsonl do not change. Manifests do not change. The duplicate clusters are not rebuilt (whether to recluster is our decision).
- The product does not change: the matcher, the confidence rule, the possible tier's threshold, the report, the peak records and the detections stay as they are; `regress session-21-start` identical at the end; scripts/check.sh green. Code changes belong to the harness and analysis scripts, tested, cross-platform (ADR 0020).
- AGENTS.md: no SIMD, no `unsafe`, optimise only from measurement, an experiment file of at most 40 lines, a losing experiment's code deleted; evaluation JSON in docs holds file names only.
- Out of scope (ADR 0017): report edits, review workflows, and features that fix tags or the library.
- corpus/ is private and never committed.
- Leave alone every process this session did not start.
- Runs longer than 30 minutes start detached under `screen`; heavy runs one at a time. NAS-scale harness runs overwrite work/reports/library-nas-dnb-peaks/, so copy what you need first. Keep scratch data under a few GB.
- Commit after each step. Check `git status` before every commit and never use `git add -A`. Do not push. No AI attribution in commits.
```

## State at the start of session 21

- 05:46. `git status` clean at `8537226` on `master`. Experiments
  0001-0115 (the next is 0116); ADRs 0001-0020. Test set: 4 of 5
  evaluations used (none in this session).
-
- **Files may be added to
  the NAS during the session**, as
  the
  prompt expected. They do not change the NAS peak store.
- Another session's process (a resumed session) runs; not
  started by this session, left alone.
- The NAS peak store, the build and the baseline: setup step 3 below.

## Plan, with times

Start 05:50; hard end 11:59.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1 | setup: brief, checklist, build, baseline, store revision, load | 0:20 | 06:10 |
| 2 | pre-registration: candidates (a), (b), (c) and combinations, oracle, denominators, criterion; committed before any render | 0:30 | 06:40 |
| 3 | rendered mixes: `mixes` extended (a plan of long plays, NAS files, leave-outs per mix), tested; rendered and searched at NAS scale under `screen` | 1:35 | 08:15 |
| 4 | measurement: oracle, (a), (b), (c), the criterion; 0114's true plays apart; experiment 0116 | 1:30 | 09:45 |
| 5 | the outcome, frozen if a candidate passes | 0:15 | 10:00 |
| 6 | if time remains: the FFT lengths' first stage on the panel's development half (a worktree never merged); an experiment | 0:30 | 10:30 |
| 7 | read-only review by a subagent, from 10:45 at the latest; fixes | 0:30 | 11:15 |
| 8 | wrap-up: notes, status, checklist, roadmap, research note, `regress`, checks, store revision, commit | 0:44 | 11:59 |

If a step overruns by more than 20 minutes, step 6 goes first. The
review starts by 10:45 and the wrap-up by 11:15 whatever is left.
