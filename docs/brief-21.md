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

## Pre-registration (written at 05:51, before anything was rendered)

Everything below is fixed before a mix is rendered or searched. No
candidate, measure, combination or threshold rule may be added after
seeing the data; anything computed later is reported as exploratory and
decides nothing.

### The mixes

- Seeds 2101, 2102 and 2103. For each seed, one mix per development
  played recording that has a shared passage (12: Dozer, the Notjamie
  bootleg, Fire's original mix, Rogue Unit Remix, Nasty Jungle Remix and
  Rogue Unit VIP, Time Is Broken, Ignorance, Sonar, the Mark System
  Revision, Watermelon, Monotone); 36 mixes. Every played file is
  3:26-7:27 long, so each plays whole, from 0 s to its end (the whole
  file fits). Where a passage starts at the file's start (Dozer, Fire's
  original, Time Is Broken, Sonar), the file has no own material before
  it; the whole file is the most there is.
- Each mix: three drawn plays, the played file, three drawn plays. Drawn
  plays, speeds, levels and transitions follow the `mixes` harness's
  rules unchanged (plays of 20-60 s from 10-95% of a track; turntable
  speed within 6% of native, the played file's speed drawn the same way;
  bass swaps 40%, crossfades 30%, cuts 30%; each play at equal RMS;
  128 kbps MP3). Two departures, both to keep the denominators clean:
  no held-out plays and no returning play (every drawn track is
  indexed, so each drawn play is a true play).
- The drawn pool: corpus records of at least 150 s, leaving out every
  file of the panel (both halves: recordings, other versions, files of a
  family's title) with their duplicate clusters, and the five
  test-derived versions (`shared_panel.py`'s pattern). No validation-half
  recording and no test-derived version is ever placed.
- NAS scale: the harness's saved index of the corpus and the NAS store's
  records less the copies corpus files stand for (ADR 0014), at the
  store's revision `b285da9984422fb5`. Three indexes per mix:
  *everything*; *played left out* (the played recording's files: the
  panel's recording list and the NAS duplicate cluster), the main case;
  *source left out* (the played file alone), only where another file of
  its recording is indexed (Dozer, Fire's original, Time Is Broken,
  Sonar), the positive control.

### The oracle (session 20's, applied per play)

A *play* is the product's (ADR 0006): one file's detections in one mix
and index, joined across gaps of at most 90 s, taking part only when at
least possible; it is confident when one of its segments is (ADR 0013).
Each confident play is, in this order:

1. *right*: of a file of the played recording (the panel's recording
   list) overlapping the played file's audible span, or of a file in a
   drawn track's duplicate cluster overlapping that drawn play's span;
2. *wrong related*: of a file of another recording of the played
   recording's family, or of its listed other versions (session 20's
   roles `related` and `version`), overlapping the played file's span;
3. *wrong, unjudged title*: of a file listed under the family's
   `unjudged_same_title`; counted and named apart (session 20's rule);
4. *wrong, other*: anything else; named and counted apart. A file that
   covers 80% or more of a planned file by `gunfinger-eval pair` is the
   same recording (the clustering rule, as 0115) and is set aside.

With the played recording left out, the right answer over the played
file's span is possible only or nothing; a confident related play there
is the failure being studied.

### The denominators

- *True confident plays* (the primary sample): the right confident
  plays of drawn tracks in the *played left out* runs, and the right
  confident plays of the played recording's other rip in the *source
  left out* runs. (Drawn tracks in the *everything* and *source left
  out* runs repeat the same mixes over nearly the same index and are not
  counted again.)
- *Wrong related confident plays*: the wrong related confident plays in
  the *played left out* runs.
- Reported apart, outside the decision: the *everything* runs (each
  wrong related play beside the played file's own play) and the wrong
  related plays of the *source left out* runs; the second sample (below).
- Each counted per play: every file's play counts once, so two rips both
  confident over one drawn play are two true plays.

### The candidates, computed the same way for every confident segment

A play's value is the most favourable value over its confident segments
(the play stays confident if any confident segment would), so a rule on
a play is a rule on its segments.

- **(a1) hits per second of the span**: the segment's hits divided by
  its span in query time (end minus start). Low is suspicious.
- **(a2) the peak verifier's aligned share** (`verified.aligned.share`,
  tolerance 1 bin x 1 frame, experiment 0054). Low is suspicious.
- **(b) the continuation share**: on each side of the segment (the 10 s
  of the file's own material after the segment's end in file time, and
  the 10 s before its start), the side is *testable* when the file holds
  those 10 s and the query holds the query time they map to. For a
  testable side, the verifier's share of the file's reference peaks in
  those 10 s found in the query, on the segment's line extended (the
  verifier's line through its ends, at its speed, with the offset of its
  nearest window), divided by the segment's own aligned share. The
  segment's value is the minimum over its testable sides; a segment with
  no testable side has no value and (b) never rejects it. Low is
  suspicious ("the file's own material beyond, at the alignment, finds
  nothing in the mix although the mix plays on"). 10 s is one window of
  the confidence rule and of the verifier, chosen a priori.
- **(c) the library's map of shared passages**: the file's stored peaks
  within the segment's file span (`track_start_seconds` to
  `track_end_seconds`), searched with `search_peaks` over the turntable
  ladder (0.92-1.08, the duplicate search's method) against the run's
  own index: the files present after the leave-out, with the fullest 1%
  of lists skipped as in the run. Detections of the file's own recording
  (its NAS duplicate cluster, and the panel's recording list where it is
  in one) are ignored. The value is the hits of the strongest detection
  of any other file. High is suspicious; at 60 hits (the possible tier,
  ADR 0006) the segment is *marked*: another indexed version holds its
  passage. Never built from a file left out.

Combinations, named now (none added later): each makes one statistic
whose threshold is searched the same way.

- **(c)+(a1)**: a play the map does not mark (value under 60) is kept;
  a marked play takes its (a1) value.
- **(c)+(a2)**: the same with (a2).
- **(c)+(b)**: the same with (b).

### The decision criterion (the research note's feasibility test)

For each candidate and combination, on the primary sample: the threshold
is the strictest one that keeps at least 95% of the true confident plays
(for low-is-suspicious statistics, the largest value v with at least
95% of true plays at v or above; for (c), the smallest value v with at
least 95% of true plays at v or below); a play at the threshold is kept. The
candidate is **promising** only if that one threshold rejects at least
half of the wrong related confident plays. (c) is also reported at its
natural point, 60 hits; that does not decide.

- The second sample, reported apart: the confident detections credited
  to referenced tracks in the three development mixes at NAS scale
  (0114's 102 detections, from session 20's `verified/` scans), grouped
  into plays the same way, with (a1), (a2), (b) and (c) computed on the
  same build with everything indexed. For each candidate: the share of
  them its primary threshold keeps. A candidate that is promising on the
  primary sample but keeps under 95% of the second sample is reported as
  promising in rendered mixes only, with that share beside it.
- Fewer than 10 wrong related confident plays in the primary sample:
  reported, and no candidate is called promising.
- If a candidate is promising: its definition and threshold are frozen
  in the experiment and the notes; validating it needs the panel's
  validation half rendered the same way (seeds fixed then), which this
  session does not do.
- Expected before the data, written so it cannot be claimed later: drawn
  plays are rendered from the indexed files themselves, so they are
  denser than a real rip; that favours (a). In mixes a true play also
  ends at a transition, where its own material beyond finds nothing;
  that works against (b). (c) can mark a wrong play only where a third
  version of the family stays indexed (Fire).

## Setup (done at 05:50)

- Release build: `cargo build --release` was current; binaries in
  `work/bin/s21-start/` (`gunfinger` byte-identical to session 20's
  final build). `baseline session-21-start` saved from the reports of
  session 20's final `regress`, made by that identical build.
- The NAS peak store: 38,105 records, revision `b285da9984422fb5`
  (`work/s21/store-revision-start.txt`), as expected.
- Listening files: `docs/pair-verdicts.txt` sha256 `d9af100a…9586`,
  `work/audition/answers.jsonl` sha256 `dff04586…09be`, the manifests'
  modification times as at session 20's start
  (`work/s21/listening-digests-start.txt`).

## Step 6's comparison (written at 06:07, before the variant ran)

The variant: session 19's first-pass turntable lengths "factor2" (the
smallest largest prime factor of the complex FFT within 2 samples of
`round(1024 / s)`, experiment 0109), applied by session 19's
`work/s19/py/fft_variant.py factor2` to a detached worktree at `49f4c77`
(`work/s21-fft-tree`, never merged; the diff is
`work/s21/fft-factor2.diff`). Refinement and the key-locked rungs are
unchanged. It runs `gunfinger-eval --other-peaks-dir <NAS store> shared
docs/panels/shared-material-seed-2026-development.json` at the store's
revision `b285da9984422fb5`, on the same rendered queries as session
20's final run (the plan and the render key are unchanged).

Compared with session 20's final report of the same plan (copied to
`work/s21/reports-before/` before anything ran), query by query and
index by index (`scripts/analysis/shared_compare.py`): wrong confident
titles added and removed, right confident files lost and gained, and
outcomes changed (session 20's oracle). The roadmap's criterion: anything
that changes evidence may add no wrong confident query on the
development half. This measures; nothing is adopted, and the rest of
the retrieval study (other rips, damage, short plays, the development
mixes) is not run.
