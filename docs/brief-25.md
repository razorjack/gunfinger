# Session 25 brief: three harness fixes, the NAS clusters rebuilt, the second shared panel's development half

Written by the agent for itself on 2026-10-16, so the plan survives
context compaction. Work autonomously and never stop to ask. When
something is ambiguous, take the conservative option, write the decision
down and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-25-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 20" to the end; session 24's
   handoff).
4. `docs/status.md` (newest entries at the bottom).
5. `docs/research/2026-10-16-version-verification.md`; experiments 0110,
   0111, 0120 and 0124-0127, and this session's (0128, 0129, 0132 and
   up); `docs/panels/`; `docs/nas-plan.md` (how the clusters are built).
6. `docs/brief-20.md` (the first panel's oracle), `docs/brief-23.md`
   (the clusters rebuilt, the oracle under fresh clusters) and
   `docs/brief-24.md` (the verifier as pre-registered, T = 0.121).

`docs/brief-2.md` to `docs/brief-24.md` hold rules that still apply
unless this brief changes them (among them: edit no configuration file;
test fixtures are generated, never copied from the corpus or the NAS;
runs longer than 30 minutes start detached under `screen`; heavy runs one
at a time; NAS-scale harness runs overwrite
`work/reports/library-nas-dnb-peaks/`, so copy it first). Session 24's
handoff item 1 (merge `s24b-panako`) is overridden by this prompt:
session 26 merges it.

## The prompt, verbatim

```text
Start session 25 of Gunfinger, an overnight session: three harness and test fixes from sessions 23-24, the NAS clusters rebuilt at the current revision, and the first run of the second shared-material panel's development half, with session 24's verifier measured on it as it stands. It runs unattended from about 20:50 with a planned end at about 03:30. The end is soft: finish a run or review in progress rather than kill it, but plan to end by then. Session 26 follows until 11:00 (verifier version 2), so leave everything committed and a handoff in the notes. The NAS is mounted at /Volumes/atlas. Work autonomously and never stop to ask. When something is ambiguous, take the conservative option, write the decision down and continue.

Context.
- Session 24 (experiments 0124-0127, docs/session-notes.md "Session 24" with its handoff) built `gunfinger-eval versions`, a diagnostic version verifier (band dynamics along a play's fitted line, a gain per mixer EQ group fitted on one half and tested on the other, states supported, contradicted, killed, masked, unobservable; S1, the share of contradicted patches, rejects at T = 0.121, set on the shared panel's development half). Against its pre-registered criteria it rejected 5 of 108 true plays in the three development mixes (limit 5) and 43 of 87 wrong related plays in rendered mixes (44 required), so it failed by one. It catches only Fire's family (remix basslines differ under shared drums); Dozer's, Pyro's and Sonar's wrong versions play the same content along the matched stretch. All five rejected true plays have reported lines 98-193 ms off (the opening-window anchor; alien5ive uploads drifting 0.2-0.5% in speed). Session 26 will build a version 2 (its own line from the audio, drift); this session prepares the ground and gives it a fresh evaluation set.
- The second shared-material panel, docs/panels/shared-material-2-seed-2026.json (session 23, step 6b), was drawn from the `different` verdicts by ear outside the frozen panel's families, split by seed 2026 and frozen before any search: development Phantom Force, Star Trails, We R The Ones; validation Killa Bees, Stab 2 Def, Tsunami. Its passages are not cut yet, and its recordings come from the duplicate clusters only, so some rips are listed as unjudged files of the title; before a first run, `pair` should join them, as session 20 did for the first panel (experiment 0110). Its validation half stays unsearched.
- The NAS store holds 40,126 records, revision fed26c76b8fe89ea (772 more than the 39,354 the clusters of experiment 0120 were built on). New files are indexed from time to time; if the revision changes during the session, record it and carry on.
- No listening was done since session 23: the six questions in work/audition/s23/ and 0120's cut Sonar join stay open. Plan nothing that waits on a judgement by ear, and never answer a question.
- A parallel session, 24b, runs a Panako benchmark in its own git worktree on branch `s24b-panako` (it may be waiting for a Java 11 install, or finished). Never merge, touch or delete its worktree or branch; session 26 merges it. It owns experiment numbers 0130 and 0131: number this session's experiments 0128, 0129, then 0132 and up.
- State at the start: git clean at 6f204d5 (check it).

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it), docs/brief-24.md and docs/session-24-checklist.md (brief-2 to brief-23 hold rules that still apply unless this prompt changes them; brief-20, brief-23 and brief-24 matter most), docs/session-notes.md ("Session 20" to the end), docs/status.md (newest entries at the bottom), docs/research/2026-10-16-version-verification.md, experiments 0110, 0111, 0120 and 0124-0127, docs/panels/, and docs/nas-plan.md (how the clusters are built).
2. Write docs/brief-25.md (this prompt verbatim, the state at the start, the plan with times) and docs/session-25-checklist.md. Point CLAUDE.md's reading list at them; keep brief-24 among the older briefs. Commit.
3. Build with --release and copy the binaries to work/bin/s25-start/. Run `gunfinger-eval baseline session-25-start` (record its wall time; session 24's took 153 s). Record the NAS store's records and revision.

Steps:

1. Setup (until about 21:20): the steps above.

2. The clusters rebuilt (start it early, under `screen`, and work on step 3 while it runs): session 23's command (work/s23/clusters.sh: session 18's queries, full, every verdict) with work/bin/s25-start at the current revision; copy work/reports/library-nas-dnb-peaks/ to work/s25/before/ first. If it stops on the Sonar chain again, take 0120's decision (`--reuse-pairs --cut-sparsest` on the run's own pairs) and say so. Write down what changed against 0120 (members added or gone, splits, joins, the gap, new borderline pairs; borderline pairs become listening questions, never verdicts). Experiment 0128.

3. Three fixes, each with tests, while step 2 runs (about 1 h 30 min):
   a. The `decode::watch` tests in gunfinger-core fail under heavy load (session 23: at load 100-250 a stand-in that only runs `exit 1` was reported stalled after the 0.5 s timeout, and two other tests in the module failed once). Make them robust without changing the watcher's behaviour: test code only, unless a real defect turns up (then stop, record it and leave the product alone). Check under load: run the module repeatedly while busy loops you start occupy every core, then stop them.
   b. `holders` silently accepts a file to leave out that is not indexed, so a misspelt name leaves nothing out (session 23's review). Refuse or report it, using the copy map so that a corpus file without a NAS copy, which stands for itself, is not refused.
   c. The harness's `render_samples` (used by `versions` and `identifiability`) seeks by timestamp, which for a file with damaged frames differs from the timeline the store was built on (session 24: Do Drop In runs 235 ms ahead). Make it follow the store's timeline (decode from the start, as indexing does, or detect the case and say so), and check whether any file in 0112's or 0124-0127's results is affected; report any count that changes.
   No change to crates/gunfinger, and none to gunfinger-core's behaviour; `regress` identical. Commit each.

4. The second panel's development half (about 2 h 30 min), after step 2's clusters:
   a. Join its families' rips with `pair` as session 20 did (0110), and cut its passages the same way. Write the result as a new file beside the frozen one (for example docs/panels/shared-material-2-seed-2026-development.json, as the first panel's development half is), changing nothing in the frozen file. Commit it before any search.
   b. `shared` on it at NAS scale (one run, as 0111 and 0122 ran the first panel's half), scored with scripts/analysis/shared_outcomes.py under the fresh clusters. The question: does the first panel's picture hold on new families (how often a related version becomes a confident wrong title with the played recording left out, and the outcomes table)? Experiment 0129.
   c. Session 24's verifier exactly as it stands (T = 0.121, frozen; nothing changed or tuned) on this run's wrong confident plays and positive controls: a fresh test of its wrong-play side on families it has never seen. Pre-register what you count before running it. Experiment 0132.

5. A read-only review by a subagent, starting by about 02:15; fix what it finds.

6. Wrap-up (from about 02:45): a "Session 25" section at the end of docs/session-notes.md (the fixes, what the clusters changed, the second panel's numbers beside the first's, the verifier on it, the handoff for session 26), docs/status.md, the checklist, the roadmap. `regress session-25-start` identical with the final build; scripts/check.sh green with rustup's 1.98.0 first on PATH and RUSTUP_TOOLCHAIN=1.98.0, as session 24 ran it (never install a toolchain); the NAS store's revision. Commit. Last, print what was done, the checks, and anything left.

Rules for this session:
- Planned end about 03:30, soft. Steps shrink from the end: 4c, then 4b. An unfinished step is reverted, or committed only in a working, tested state, and named in the notes.
- The product does not change: crates/gunfinger and gunfinger-core behave as at the start; `regress session-25-start` identical; scripts/check.sh green.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or reports, rescore nothing, and never run test evaluation 5. Never identify, decode or index EM_1999_02_07_-_Ed_Rush_Optical.mp3 at the NAS root. Run nothing on the candidate mixes of experiment 0065 (including the mixed CDs it lists and the NAS's sety/vertical.mp3).
- Unsearched: the recall panel's validation half, the first shared panel's validation half and the second panel's validation half (Killa Bees, Stab 2 Def, Tsunami): no run, score or look at their detections. Clustering may query their files as cluster members, as before; say so if it happens. The frozen panel files do not change.
- The five test-derived versions are never used to tune or calibrate anything: Terminal 2, the Orders remixes, the Marky & Bungle Funktion, the Clear Skyz Break remix and 1996 VIP, and the Black Barrel Beachball.
- The NAS (/Volumes/atlas/Music/dnb) is read, never written (its .gunfingerignore included). The NAS peak store is read-only for this session: no `index`, `prune` or `--retry-skipped`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt and work/audition/answers.jsonl do not change. Manifests do not change, including corpus/sets/nocny-trans-vertical-2002-08-23/tracklist.toml.
- AGENTS.md: no SIMD, no `unsafe`, optimise only from measurement, an experiment file of at most 40 lines written before the next experiment starts, evaluation JSON in docs holds file names only, a new ADR to change a decision, cross-platform code (ADR 0020), no filename- or tag-derived logic in the product.
- Out of scope (ADR 0017): report edits, review workflows, and features that fix tags or the library.
- corpus/ is private and never committed.
- Leave alone every process this session did not start, including session 24b's (Java, Gradle, Panako, its scripts).
- Runs longer than 30 minutes start detached under `screen`; heavy runs one at a time (24b's heavy runs, if any, are niced; carry on). NAS-scale harness runs overwrite work/reports/library-nas-dnb-peaks/, so copy what you need first. Keep scratch data under a few GB.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step. Check `git status` before every commit and never use `git add -A`. Do not push. No AI attribution in commits.
```

## State at the start of session 25

- 20:44. `git status` clean at `6f204d5` on `master` (35 commits ahead
  of `origin/master`, never pushed). Experiments 0001-0127; session 24b
  owns 0130 and 0131, so this session numbers 0128, 0129, 0132 and up.
  ADRs 0001-0020. Test set: 4 of 5 evaluations used (none in this
  session).
- Branch `s24b-panako` at `87bfa55` ("Session 24b checklist: waiting
  for Java 11"), worktree `/Users/razorjack/.t3/worktrees/gunfinger/s24b-panako`;
  its session's script `work/s24b/wait-java11.sh` is running. Not
  touched.
- The NAS peak store at 20:46: 40,126 records, revision
  `fed26c76b8fe89ea` (`work/s25/store-revision-start.txt`), as the
  prompt says; the newest record written at 17:53. The NAS is mounted
  (SMB) at `/Volumes/atlas`.
- Processes this session
  did
  not start are left alone,
  including session 24b's
  agent
and its Java 11 wait.
- Listening files: `docs/pair-verdicts.txt` sha256 `d48ffb313396...`,
  `work/audition/answers.jsonl` sha256 `ee2d5bd9fe0d...`, as at session
  24's end (`work/s25/listening-digests-start.txt`).
- Build: `cargo build --release` (Homebrew's 1.98.0) was current;
  `work/bin/s25-start/` holds `gunfinger` and `gunfinger-eval`
  byte-identical to `work/bin/s24-final/` (`gunfinger` also to
  `s24-start`).
- `baseline session-25-start` copies the last `regress`'s reports (0.00
  s wall), so, as in session 24, `regress session-25-start` is the speed
  check: identical (720/720 sweep queries, all 87, 93 and 79 scan
  detections, the calibrate line known since session 8), 153.8 s wall,
  1,303 s user (session 24: 153 s). The machine runs at its usual speed.
- The NAS report folder `work/reports/library-nas-dnb-peaks/` (1.6 GB)
  was cloned to `work/s25/before/library-nas-dnb-peaks/` (APFS clone)
  at 20:47, before any run.

## Plan, with times

Start 20:44; planned end about 03:30, soft.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1 | setup: brief, checklist, build, baseline, `regress` as the speed check, store revision | 0:35 | 21:20 |
| 2 | clusters at the current revision under `screen` (started 20:49; 26 min in session 18, 2 h 54 min in session 23's slow night); then the comparison with 0120, experiment 0128 | run + 0:30 | about 22:30 |
| 3 | three fixes while step 2 runs, each tested and committed: (a) `decode::watch` tests, (b) `holders` left-out names, (c) `render_samples` on the store's timeline and the affected files | 1:30 | 22:50 |
| 4a | the second panel's development half: `pair` joins, passages, the new file beside the frozen one; committed before any search | 0:45 | 23:40 |
| 4b | `shared` at NAS scale (one run, about 30-40 min), scored under the fresh clusters; experiment 0129 | 1:15 | 00:55 |
| 4c | pre-registration, then `versions` exactly as built on 4b's wrong confident plays and positive controls; experiment 0132 | 1:15 | 02:10 |
| 5 | read-only review by a subagent from 02:15 at the latest; fixes | 0:30 | 02:45 |
| 6 | wrap-up: notes, status, checklist, roadmap, `regress`, `check.sh`, store revision, commit | 0:45 | 03:30 |

Steps shrink from the end: 4c, then 4b. Heavy runs (the clusters run,
`shared`, `regress`, `check.sh`, the load test of 3a) never overlap;
builds and unit tests of step 3 run beside the clusters run, which only
slows it.

## Decisions

- **(20:52) The clusters search runs with `--cut-sparsest` from the
  start.** Session 23's command (`work/s25/clusters.sh`, `map-library`
  first: 247 of 262 corpus files have a copy among the 40,126 records)
  exited 1 in 0.85 s without searching: `clusters` checks the verdicts
  against the last run's pairs before a search (`main.rs`,
  `check_verdicts_against_last_run`), and 0120's pairs hold the Sonar
  chain, so it named the same three chains as session 23's stopped
  search (the corpus, `full_scene` PRO 012 and dfect rips of Sonar ~
  the Mark System Revision, each through the-hifi-junglist's PRO 012 A
  upload; the revision joined at 99% with 406 hits, 1.1 per second).
  The prompt's instruction for a stop on this chain is 0120's decision,
  the run's own pairs clustered with `--cut-sparsest`; here no run's
  own pairs exist yet. So the full search (no `--reuse-pairs`) runs
  again with `--cut-sparsest`, which passes the pre-check and, at the
  end, clusters the search's own pairs with each conflicting chain's
  sparsest join cut: the same procedure in one run. Check afterwards:
  the cut links are 0120's (PRO 012 A upload ~ revision) and nothing
  else; any other cut is a finding, listed, and a listening question,
  never a verdict. The pre-check's output is in `work/s25/precheck/`.
- **(21:00) Step 3c: a second render function, not a changed one.**
  `render_samples` also renders `shared`'s, `mixes`'s and `robust`'s
  queries, whose renders are cached under a digest of the plan, not of
  the code; changing it would let a later run mix old and new renders
  silently and would change earlier experiments' queries. So
  `render_samples` stays as it is (FFmpeg's seek by timestamps, right for
  a query, which only needs the audio), and `render_samples_as_indexed`
  counts the start in samples decoded from the beginning
  (`asetpts=N/SR/TB,atrim=start=...`), the timeline of the peak record;
  `versions` and `identifiability`, which compare audio with a line
  fitted on stored peaks, use it. The mix side of both commands decodes
  an excerpt with the core's decoder (an input seek too); the core does
  not change, so that side is checked, not changed.
- **(21:12) Step 3c's check** (`scripts/analysis/render_timeline.py`,
  `work/s25/timeline/s24-and-0112.json`): every reference and mix that
  0124-0126's plays and 0112's report rendered (594 files, 1,638
  positions, each rendered both ways). Off by more than 1 ms: Do Drop In
  (-26.1 ms at 112 s, -235.1 ms at 429 s: one and nine MP3 frames of
  1,152 samples) and `extra/a-)eib(-miami-sour.mp3` (-26.1 ms at 296 s);
  Pyro - Time Is Broken's butch-french m4a 0.1-0.6 ms; every mix,
  including the three development mixes, at 0. `versions` again with the
  fixed build on the 33 plays of these references
  (`work/s25/timeline/runs-*.json`): one decision changes, Do Drop In's
  drawn true play in `seed-2101-pyro-2` (S1 0.889 -> 0, rejected ->
  kept), so E2's rejected drawn true plays go from 3 to 2 of 480; Pyro's
  plays move by at most 0.005 in S1 (all below 0.03), so T and every
  calibration and E2 decision stand; Miami's S1 unchanged. E1 holds none
  of these references. 0112 is rerun after the clusters search.
- **(21:20) Step 3b refined before any use.** The first version refused
  every left-out name the index does not hold. `shared` leaves out the
  played recording's whole file set, which can include a NAS copy that a
  corpus file stands for, so session 23's `lookups-shared` files would
  have been refused (2 such copies). A copy is now accepted when its
  corpus file is left out in the same lookup (its audio is then left out
  under the corpus file's name); a copy without its corpus file and any
  other unindexed name are refused. Emulated on session 23's 606 shared
  and 101 identify lookups with the copy map: none refused.

## Step 4c's pre-registration (written at 21:17, before any `shared` run on the second panel)

Nothing below may change once 4b's run exists; anything computed later
and not named here is exploratory and decides nothing. Nothing here
decides anything about the product.

- **The plays**, from 4b's `shared` report on
  `docs/panels/shared-material-2-seed-2026-development.json`, formed as
  session 24 formed its calibration plays (`version_plays.py`, its
  `calibration` logic with this run's queries directory): each query's
  detections at least possible, joined per file across gaps of at most
  90 s, a play confident when one of its segments is; roles by session
  20's oracle (`shared_outcomes.py --clusters`) under step 2's fresh
  clusters and the development file's recordings; control directions
  left out. *Negatives*: the confident plays, other than the played
  recording's, of `played left out` queries whose outcome is wrong
  confident. *Positives*: the played recording's confident plays of the
  `source left out` index (the positive control).
- **Siblings** (for S2 and S3 only): `ambiguity_note.py lookups-shared`
  on the report, `gunfinger-eval holders` on those lookups, files outside
  the play's own recording (fresh clusters and the panel's recordings)
  with 60 hits or more, as session 24.
- **The verifier**: `gunfinger-eval versions` with this session's build
  after step 3c. Its code (`versions.rs`, `versions/compare.rs`) is
  unchanged since session 24; only its renderer now counts the start on
  the decoded timeline (step 3c). **T = 0.12100456621**, frozen: a play
  is rejected when S1 >= T; a play with S1 undefined is kept. Run beside
  it, deciding nothing: `work/bin/s24-final/gunfinger-eval` on the same
  plays; every play whose S1 differs between the two is listed.
- **Counted** (the first test of the wrong-play side on families the
  verifier has never seen): negatives rejected at T, of all negatives,
  in total, by family (Phantom Force, Star Trails, We R The Ones), by
  kind of query and by related file; positives kept at T.
- **Reading, fixed now**: the wrong-play side *holds on new families*
  when at least half of the negatives are rejected (the calibration's
  rate by T's construction: 157 of 312) and at least 95% of the
  positives are kept; otherwise it *does not hold*. With fewer than 10
  negatives the test is *too small to read*, and the counts are reported
  as they are. Plays of one query or one direction are not independent;
  the family is the unit, and three families cannot bound a rate, so the
  counts by family are reported beside the total.
- **Reported beside, deciding nothing**: S1 quantiles, S2, S3, the
  patch states by EQ group for negatives and positives, and for each
  rejected or kept negative its family and related file.
- **Expected before the data**, written so it cannot be claimed later:
  in session 24 the verifier rejected wrong plays only where the low
  bands disagree under shared drums (Fire's family; Dozer's on the
  panel), never Pyro's or Sonar's. The new related versions are remixes,
  an edit and a VIP; whether their basslines differ along the matched
  stretch is not known before the run. The passages are shorter than
  the first panel's (15-62 s, 108-195 hits, from `pair` at 21:10), so
  fewer wrong confident queries than 0111's 106 of 218 are expected.
