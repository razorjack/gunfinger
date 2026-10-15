# Session 20 brief: the shared-material challenge set and proposal 5's diagnostic

Written by the agent for itself on 2026-10-15, so the plan survives
context compaction. Work autonomously and never stop to ask. When
something is ambiguous, take the conservative option, write the decision
down and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-20-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 18" to the end).
4. `docs/status.md` (newest entries at the bottom).
5. `docs/research/2026-10-14-matching-proposals.md` (proposal 5 closely);
   the roadmap's "A confidence statistic for shared material",
   "Evidence beyond exact pair hashes" and "A minimum span instead of 3
   windows"; ADRs 0006, 0008, 0016, 0017, 0019, 0020; experiments 0026,
   0055, 0058, 0076, 0091, 0100, 0101, 0109.
6. `docs/pair-verdicts.txt`, `docs/audition-questions.md`,
   `crates/gunfinger-eval/src/shared.rs`, `pair.rs`, `verifier.rs`.
7. The newest file in `docs/experiments/`; the panel in `docs/panels/`.

`docs/brief-2.md` to `docs/brief-19.md` hold rules that still apply
unless this brief changes them (among them: edit no configuration file;
test fixtures are generated, never copied from the corpus or the NAS;
runs longer than 30 minutes start detached under `screen`; heavy runs one
at a time; NAS-scale harness runs overwrite
`work/reports/library-nas-dnb-peaks/`, so copy it first; each timing
series records the load and the busiest processes).

## The prompt, verbatim

```text
Start session 20 of Gunfinger, an overnight session on precision: build the shared-material challenge set from the judgements by ear and run proposal 5 of docs/research/2026-10-14-matching-proposals.md (the identifiability diagnostic) on its development half. It runs unattended from about 02:45 and must finish by 11:59. The NAS is mounted at /Volumes/atlas. Work autonomously and never stop to ask. When something is ambiguous, take the conservative option, write the decision down and continue.

Context.
- Precision comes first (ADR 0017): nobody reviews the output, so a confident wrong title is the worst result. The main way Gunfinger still gives one at NAS scale is a related version that shares a passage with the played record: The Nine's Evol Intent VIP is confident inside The Nine's own play (experiment 0076: in 18 of 34 directions the other version's play over a shared passage is confident). Neither the confidence rule nor the containment rule (0091, 0101; not adopted) fixes it.
- The roadmap ("A confidence statistic for shared material") asks for a challenge set of originals, VIPs, remixes and records sharing breaks, each pair confirmed by ear, before anything that adds evidence is adopted: a minimum span, the band-profile peaks, evidence beyond pair hashes, and now the FFT lengths of session 19 (experiment 0109; its retrieval study waits on a criterion that includes shared passages). Session 7 ran four scenarios on two judged pairs (`gunfinger-eval shared`, experiment 0055); session 11 measured shared passages played whole at NAS scale (0076). Since then 13 pairs have been judged `different` by ear (docs/pair-verdicts.txt), and the listening page has two `shared` answers (work/audition/answers.jsonl: Watermelon ~ 300 Baud - Monotone (Mobiuz & Cycom remix); Falcon - Fractles ~ Ryme Tyme - We Enter (Optical remix v3.0)).
- What a verdict means (our note, 2026-10-12): two rips of the same vinyl by different people, however different they sound, are `same`; files that differ in even a few bars, such as two edits, are `different`. A pair whose relation has not been judged by ear is a candidate, not a panel member.
- Proposal 5 asks, before any new statistic: does the observed audio hold evidence that tells the two versions apart, and does it survive into peaks and pairs? Three outcomes: no distinguishing information in the observation (the honest answer is ambiguity); information in the audio that does not survive into peaks and pair hashes (a representation question); information that survives but the evidence ignores (only this justifies a new statistic). The note's review: it is a diagnostic separation, not a deployable rule; define the oracle and the denominator exactly first; split by recording family; build any reuse map only from files present after a leave-out; the case that matters most is the played recording absent from the index, where the related file is the only candidate.
- The development mixes' related versions at NAS scale (session 18 and ADR 0019's 9 lines) are candidates for the panel once judged: Clockwork and its Stakka Rmx; Phantom Force, Phantom 2018 and Fracture's edit; Rift feat. Souldrop - Meltdown against Phantom Force; Star Trails and its Synergy Remix; The Kraft - Stab 2 Def and Mampi Swift - 2000; Klute - We R The Ones (VIP, "3", Klute Remix); Sonic - Cannibal and `1.mp3`; the Vertical manifest's `# related:` versions (session 18 searched each against its track's reference, experiments 0100-0102).
- State at the start (checked at 02:32): git clean at 2836414; the NAS peak store holds 38,105 records, revision b285da9984422fb5; the CLI's saved index (~/.cache/gunfinger/indexes/, 4.3 GB) was written at that revision's time; the harness's saved index (work/indexes/library-nas-dnb-peaks.index, 3.8 GB) was made at 33,723 records and is stale, so the first NAS-scale harness run rebuilds it. The disk has 28 GB free; keep scratch data under a few GB.
- ADR 0020 (2026-10-14): Gunfinger supports macOS and Linux; any platform-specific code goes through `platform::Platform` with both branches tested.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it), docs/brief-19.md and docs/session-19-checklist.md (brief-2 to brief-18 hold rules that still apply unless this prompt changes them), docs/session-notes.md (from "Session 18" to the end), docs/status.md (newest entries at the bottom), the research note (all, proposal 5 closely), the roadmap's "A confidence statistic for shared material", "Evidence beyond exact pair hashes" and "A minimum span instead of 3 windows", ADRs 0006, 0008, 0016, 0017, 0019 and 0020, experiments 0026, 0055, 0058, 0076, 0091, 0100, 0101 and 0109, docs/pair-verdicts.txt, docs/audition-questions.md, and crates/gunfinger-eval/src/shared.rs, pair.rs and verifier.rs.
2. `git status` should be clean at 2836414. Write docs/brief-20.md (this prompt verbatim, the state at the start, the plan with times) and docs/session-20-checklist.md. Point CLAUDE.md's reading list at them; keep brief-19 among the older briefs. Commit.
3. Build with --release and copy the binaries to work/bin/s20-start/. Run `gunfinger-eval baseline session-20-start`. Record the NAS peak store's state (scripts/analysis/store_revision.py; 38,105 records and revision b285da9984422fb5 expected) and the load.

Steps:

1. Setup (30 min): the steps above.

2. The panel (about 1.5 h, until about 04:45).
   a. Families: every pair judged `different` by ear, the two `shared` answers, 0055's and 0076's pairs, each with its other indexed versions and rips (the NAS duplicate clusters). The development mixes' related versions above join only where a verdict or answer covers them; the rest become listening questions (step 5). Leave out the five test-derived versions (below).
   b. For each pair, the shared passages from `gunfinger-eval pair` (alignments, supported time), and the distinctive material around them.
   c. Before measuring anything, write down the oracle and the denominator (what counts as right, wrong confident, possible only and nothing, per scenario and index), then split the families into a development half and a validation half by recording family with a fixed seed. Freeze the panel as JSON under docs/panels/ (file names only, as AGENTS.md requires). The validation half stays unsearched tonight, as the recall panel's validation half does; its outcomes are for a later rule's validation.
   d. An experiment file for the panel (next number after 0109).

3. The scenarios with today's matcher at NAS scale, development half (about 2.5 h, until about 07:15). `shared` already runs the passage alone (10, 20, 30 s), looped (60, 120, 240 s) and reaching 10-30 s into the played file's own material, against three indexes (everything; the played cluster left out, the related file kept; the played file alone left out while another rip stays, the positive control). Extend it where the roadmap's scenarios need it: context before the passage as well as after (5, 10, 20 s), a weak rip of the played recording as the query, a blend at equal level that masks the context, and a short shared break repeated. Keep each extension small and tested; the plan file stays the input, so the panel runs in one command. Report per scenario and index: right confident, wrong confident (the related file), possible only, nothing, with hits and windows against the rule (240 hits in 3 windows) and the possible tier (60). Name the cases where a wrong title is confident today. An experiment file.

4. Proposal 5's diagnostic on the development half (about 2 h, until about 09:15). For each family, where the aligned evidence falls: inside the shared passage, outside it, across repeats; and over the passage, how the two versions compare in the audio and in their peaks and pairs, with the rips of one recording as the noise floor (two rips of the same record are `same`, however different they sound). Classify each family as one of the three outcomes above, with the measurement that decides it. Treat the played recording absent from the index as the main case. No rule is proposed for adoption tonight: if outcome 3 appears, describe what the unused evidence is and what a validation on the frozen half would need. An experiment file.

5. Listening questions (about 30 min): the candidate pairs from the development mixes and any family whose relation or passage edges only a judgement by ear can settle. Use the existing producers of `gunfinger-eval questions` (ADR 0016, docs/audition-questions.md). Write the files outside the default queue (work/audition/s20/), and check that the page loads them with a temporary answer log and a copy of the verdicts file. Never answer. In the notes, give each question with full absolute paths, the question and the times.

6. If time remains (until about 10:30): proposal 3's first step only, exact and short: a profile of the 2003 mix at NAS scale (`--store-only --playback both`) that splits `WindowHits::take_part` into its binary searches and its copying (0103's split-profile method). Stop if the searches are under 5% of the CPU; otherwise record the share and the next step. No code is kept unless it passes session 19's acceptance rules (identical reports, A B B A timings). An experiment file either way.

7. A read-only review of the session's code, panel and experiments by a subagent, starting by 10:45; fix what it finds.

8. Wrap-up (from 11:15): a "Session 20" section at the end of docs/session-notes.md, covering:
   - the panel (families, the split, the oracle) and how to run it in one command;
   - today's outcomes, and every scenario where a wrong title is confident;
   - the diagnostic's outcome per family, and what it means for the next step (a statistic, a representation change, or reporting ambiguity, each our decision);
   - the listening requests with full paths and times;
   - any proposed manifest change, as a proposal only.
   Also update docs/status.md, the checklist, the roadmap ("A confidence statistic for shared material": the challenge set exists; the FFT lengths' retrieval study and the other evidence items now have it as a criterion), and the research note (proposal 5: measured, with experiment numbers; the note's other text stays). `regress session-20-start` identical with the final build; scripts/check.sh green; the NAS store's revision. Commit. Last, print what was done, the checks, and anything left.

Rules for this session:
- Hard end 11:59. If a step overruns by more than 20 minutes, step 6 goes first, then step 5 shrinks to the most important questions. The review starts by 10:45 and the wrap-up by 11:15 whatever is left. An unfinished step is reverted, or committed only in a working, tested state, and named in the notes.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or reports, rescore nothing, and never run test evaluation 5. Never identify, decode or index EM_1999_02_07_-_Ed_Rush_Optical.mp3 at the NAS root. Run nothing on the candidate mixes of experiment 0065, including the NAS's sety/vertical.mp3. The recall panel's validation half stays unsearched, and so does the new panel's validation half.
- The five test-derived versions are never used to tune anything and stay out of the panel: Terminal 2, the Orders remixes, the Marky & Bungle Funktion, the Clear Skyz Break remix and 1996 VIP, and the Black Barrel Beachball.
- The NAS (/Volumes/atlas/Music/dnb) is read, never written; its .gunfingerignore is edited only by hand. The NAS peak store is read-only: no `index`, `prune` or `--retry-skipped` against it. Index only into temporary stores. Check its revision at the end; if it changes during the session, write it down and compare only runs made at the same revision.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt and work/audition/answers.jsonl do not change. Manifests do not change. Propose any change in the notes instead.
- The product does not change: the matcher, the confidence rule, the possible tier's threshold, the peak records and the detections stay as they are; `regress session-20-start` identical at the end; scripts/check.sh green. Code changes belong to the harness (gunfinger-eval) and analysis scripts, tested, cross-platform (ADR 0020).
- AGENTS.md: no SIMD, no `unsafe`, optimise only from measurement, an experiment file of at most 40 lines, a losing experiment's code deleted; evaluation JSON in docs holds file names only.
- Out of scope (ADR 0017): report edits, review workflows, and features that fix tags or the library.
- corpus/ is private and never committed.
- Leave alone every process this session did not start.
- Runs longer than 30 minutes start detached under `screen`; heavy runs one at a time. NAS-scale harness runs overwrite work/reports/library-nas-dnb-peaks/, so copy what you need first.
- Commit after each step. Check `git status` before every commit and never use `git add -A`. Do not push. No AI attribution in commits.
```

## State at the start of session 20

- 02:36. `git status` clean at `2836414` on `master`. Experiments
  0001-0109 (the next is 0110); ADRs 0001-0020. Test set: 4 of 5
  evaluations used (none in this session).
-
  - **Files may be added to the NAS during the session.** They do not
  change the NAS peak store, so the store's revision should not move.
- The NAS peak store: checked in setup step 3 below. The CLI's saved
  index `nas-dnb-peaks-23935cfc56936b67.index` (4.29 GB, 2026-10-14
  15:16); the harness's `work/indexes/library-nas-dnb-peaks.index`
  (3.81 GB, 2026-10-14 03:41, 33,723 records, stale).
- Toolchain: rustc 1.98.0 (Homebrew).
- Listening files: `docs/pair-verdicts.txt` sha256 `d9af100a…9586`,
  `work/audition/answers.jsonl` sha256 `dff04586…09be`, with the
  manifests' modification times (`work/s20/listening-digests-start.txt`).

## Plan, with times

Start 02:45; hard end 11:59.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1 | setup: brief, checklist, build, baseline, store revision, load | 0:30 | 03:15 |
| 2 | the panel: families, oracle, split, `pair` passages, frozen JSON, experiment 0110 | 1:30 | 04:45 |
| 3 | `shared` at NAS scale with the new scenarios, tested; the development half in one command; experiment 0111 | 2:30 | 07:15 |
| 4 | proposal 5's diagnostic on the development half; experiment 0112 | 2:00 | 09:15 |
| 5 | listening questions in `work/audition/s20/`, page checked on temporary files | 0:30 | 09:45 |
| 6 | if time remains: proposal 3's first step (split profile of `take_part`); an experiment | 0:45 | 10:30 |
| 7 | read-only review by a subagent; fixes | 0:30 | 11:15 |
| 8 | wrap-up: notes, status, checklist, roadmap, research note, `regress`, checks, store revision, commit | 0:44 | 11:59 |

If a step overruns by more than 20 minutes, step 6 goes first, then step
5 shrinks to the most important questions. The review starts by 10:45
and the wrap-up by 11:15 whatever is left.

## Conservative decisions taken up front

- **Panel members are judged pairs only.** A pair joins the panel when the
  judgement of it by ear is on record: a `different` line of
  `docs/pair-verdicts.txt`, a `shared` answer in the answer log, or a
  judgement recorded in the docs (Clockwork against its Stakka Rmx:
  experiment 0026 and ADR 0019; China Cup against The Nine: "Verdict
  by ear: China Cup and The Nine share their drums", notes after session
  4). 0076's pairs without one (the Fire versions not judged against the
  original, Killa Bees, Tsunami, The Nine's ChaseR Bootleg, Star Trails)
  are candidates for listening (step 5), not scenarios. Where one of their
  files belongs to a judged family (The Nine's bootleg, the other Fire
  versions), it is listed as one of the family's other versions, so a
  confident detection of it is seen; it is never a scenario's played or
  related file.
- **A family is a connected group of recordings**: judged pairs joined
  through shared files or the NAS duplicate clusters (session 18's
  `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`, read,
  never rewritten). A file in no cluster is its own recording; its rips,
  if any, are unknown, and the positive control needs a cluster of two.
- **Order in step 2**: the oracle and denominator are written, then the
  split is drawn from the family list alone (seed 2026), and only then is
  `pair` run on every pair, to freeze each passage's bounds. For the
  validation half `pair`'s output defines passages and nothing else; no
  scenario query of it is rendered or searched.
- **Scenarios run in both directions** of each judged pair (each file in
  turn the played one), as 0076 did.
- **Scoring follows the brief's rule**: any confident detection outside
  the played recording's cluster is wrong, whether or not the played one
  is also confident; a confident detection of a file that may be an
  unclustered rip of the played recording is still counted wrong and
  listed by name to check manually.
- The harness's stale saved index is rebuilt by the first NAS-scale
  `shared` run (over the old file), as the prompt expects.

## The oracle and the denominator (written at 02:41, before any measurement)

Units. A *query* is one rendered excerpt of a scenario direction (played
file, related file, passage) of one kind and length, searched against
one *index*. The denominator of every rate is the number of queries of
one kind and index over the development half's directions; each
direction's queries are also listed on their own. No query is dropped
after the fact; a query that cannot be rendered is reported as such.

Roles of an indexed file in a result:

- *played*: a file of the played recording, its NAS duplicate cluster
  (session 18's file), or the file alone when it is in no cluster;
- *related*: a file of the related recording, by the same rule;
- *version*: a file of another recording of the same family (another
  judged recording, or another version listed with the family);
- *unjudged*: a file outside the clusters whose tags give the title of
  one of the family's recordings, whose relation no one has judged (it
  may be an unclustered rip);
- *other*: anything else.

Levels, today's rule: *confident* at 240 hits in 3 windows (ADR 0008);
*possible* at 60 hits (ADR 0006); below that, not reported.

Each query has exactly one outcome, tested in this order:

1. *wrong confident*: some confident detection's file is not *played*;
   labelled by the strongest such detection's role (related, version,
   unjudged, other), and marked when the played recording is confident
   in the same query too. It counts as wrong either way (brief §7.6).
2. *right confident*: a confident detection of a *played* file, and no
   other confident detection.
3. *possible only*: nothing confident, at least one possible detection.
4. *nothing*: no detection at possible or above.

Per index, the answer that is right:

- *everything*: right confident; possible only and nothing are misses
  (expected for short queries), not errors.
- *played left out* (the played recording's cluster removed, the related
  file kept): possible only or nothing (abstaining is right; a possible
  play of the related file is what ADR 0006's tier claims); wrong
  confident is the failure that matters most.
- *source left out* (only the played file removed, another rip kept; the
  positive control): right confident through the other rip.

For each query and role the strongest detection's hits and windows are
kept, so the margin to 240 and to 60 can be read for every outcome.
Proposal 5's diagnostic (step 4) classifies families, not queries: its
"true confident examples" are the development queries with right
confident outcomes in *everything* and *source left out*, and its
"constructed wrong ones" are those with wrong confident outcomes; no
rule is fitted to them tonight.

## Decisions taken in step 2 (the panel)

- Six files whose tags give a family's titles but which session 18's
  clusters never saw (most are uploads indexed since) join a recording
  by the clustering rule, measured with `pair` from library audio alone
  (experiment 0110). The panel lists them under `joined_in_session_20`;
  the clusters file is not rewritten. `shared`'s plan names each
  recording's files, so "played left out" removes them too.
- A direction's passage follows a fixed rule (0110). Directions below 25
  hits are controls, still searched for the development half (two:
  Playground Shades ~ Critikal - Interlude).
- The seeded split put The Nine and Clockwork in the validation half.
  The draw is not repeated.
- Scenario extensions (step 3), chosen before any search: context
  *before* the passage, 5, 10 and 20 s, followed by up to 30 s of the
  passage (skipped where the passage starts too early); a *weak rip*:
  the 30 s alone query and the 20 s beyond query rendered through a dull
  transfer (`highpass=f=40,lowpass=f=3000` twice, `bass=g=6:f=100`) and
  MP3 at 64 kbps; a *blend*: the 20 s beyond query with a partner at
  equal level (RMS) over its context only, the corpus file
  `Skynet & Stakka - Decoy.mp3` from 60 s, outside every family; a
  *break*: the passage's first 5 s repeated to 60 and 240 s.
- Amendment to the oracle for the blend: the partner's cluster has the
  role *partner*; its detections are never wrong, as in `robust`'s
  blends.
