# Session 23 brief: fresh NAS clusters, shared material re-scored, the ambiguity note measured

Written by the agent for itself on 2026-10-16, so the plan survives
context compaction. Work autonomously and never stop to ask. When
something is ambiguous, take the conservative option, write the decision
down and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-23-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 20" to the end).
4. `docs/status.md` (newest entries at the bottom).
5. `docs/nas-plan.md` (steps 3-5); ADRs 0017 and 0019; experiments 0090,
   0110-0119 and this session's; `docs/panels/`.
6. `docs/brief-20.md` (the panel's oracle) and `docs/brief-21.md` (the
   pre-registration: (c)'s definition, the oracle per play).

`docs/brief-2.md` to `docs/brief-22.md` hold rules that still apply
unless this brief changes them (among them: edit no configuration file;
test fixtures are generated, never copied from the corpus or the NAS;
runs longer than 30 minutes start detached under `screen`; heavy runs one
at a time; NAS-scale harness runs overwrite
`work/reports/library-nas-dnb-peaks/`, so copy it first). The
prompt below changes one earlier rule: the NAS duplicate clusters are
rebuilt this session.

## The prompt, verbatim

```text
Start session 23 of Gunfinger, an overnight session: rebuild the NAS duplicate clusters at the current revision, re-score the shared-material results of sessions 20 and 21 with them, and measure the "reporting ambiguity" option for shared material. It runs unattended from about 02:30 and must finish by 13:00. The NAS is mounted at /Volumes/atlas. Work autonomously and never stop to ask. When something is ambiguous, take the conservative option, write the decision down and continue.

Context.
- Shared material is the open problem (sessions 20 and 21, experiments 0110-0118). With the played recording not indexed, a related version (remix, VIP, edit) becomes a confident wrong title in 106 of 218 queries of the panel's development half (0111). Session 21 found no statistic that separates wrong plays from true plays in real mixes. Its shared-passage map, (c) (0116, 0118: for a confident play, another indexed file outside its recording that holds its passage), flagged 61 of 87 wrong plays in rendered mixes but also 17 of the 97 true plays in the three development mixes. docs/session-notes.md ("Session 21", "What it means for our decision") says reclustering would most likely remove most of (c)'s marks on true plays: of its 79 marks in 0116, 62 are drawn plays marked through uploads newer than the clusters. Session 20 also left nine confident detections scored wrong only because their files are newer than the clusters.
- We have decided: (1) recluster the NAS library at the current revision (the duplicate clusters are rebuilt this session; earlier briefs forbade that); (2) "statistic or reporting ambiguity" stays our decision: this session MEASURES the ambiguity note and reports; it changes no product code and writes no ADR for it.
- Since session 22: session 20's 17 listening questions were answered on the listening page (all `different`; 17 lines appended to docs/pair-verdicts.txt by the page). The comments by ear, recorded in docs/status.md (entry of 2026-10-16): Ryme Tyme - We Enter (Optical RMX) and the Optical Remix v.3.0 Full Instrumental are almost identical (A ends with a vocal over a stopped drum loop, B has no vocal and keeps the loop); technically different, but a DJ mix would play the identical middle. The Kraft - Stab 2 Def and Mampi Swift - 2000 are a different EQ and a slightly different arrangement of the same music; technically different. Session 21's two questions (the hifi-junglist Sonar upload; Kemal - Animation (Silent Extent Remix) against the corpus original) are still open.
- On request `/1.mp3` was added to the NAS's .gunfingerignore and pruned (the store went from 38,105 records, `b285da9984422fb5`, to 38,104, `5fd86c54350c819c`). It is a file of 30 s cuts from several tracks, not a track. The verdict Cannibal ~ 1.mp3 now adds nothing.
- Then `gunfinger --config ~/.config/gunfinger/nas-dnb.toml index` was started to index about 1,345 new NAS files (at 02:15 the store held 38,894 records, still growing). This session did not start it.

State at the start:
- git at 868abc9 with two uncommitted files: docs/pair-verdicts.txt (the 17 new verdict lines, written by the listening page) and docs/status.md (the 2026-10-16 entry, written by the agent that launched you). Check that the diff is exactly that, then commit those two files as the session's first commit ("Listening of 2026-10-16: 17 pair verdicts; 1.mp3 left out of the NAS library; status"). Then the tree is clean.
- The `index` run (process `gunfinger --config /Users/razorjack/.config/gunfinger/nas-dnb.toml index`, pid 40404 at launch) must have exited before you record the store's state, take the baseline or run anything at NAS scale. Wait for it (poll every minute or two; reading docs and writing the brief may go on meanwhile). Never start, stop or signal it. Then record the store's records and revision (`scripts/analysis/store_revision.py`), add a line to the status entry for the indexing (records before and after, revision), and run `gunfinger --config ~/.config/gunfinger/nas-dnb.toml doctor` (read-only) for the counts.
- Files may be added to the NAS during the session; they stay unindexed and do not change the store.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it), docs/brief-22.md and docs/session-22-checklist.md (brief-2 to brief-21 hold rules that still apply unless this prompt changes them; brief-20 and brief-21 matter most), docs/session-notes.md ("Session 20" to the end), docs/status.md (newest entries at the bottom), docs/nas-plan.md (steps 3-5, how the clusters are built: `map-library`, `clusters --from-peaks`, rounds until no new members, `--reuse-pairs`), ADRs 0017 and 0019, experiments 0090 and 0110-0119, and docs/panels/.
2. Write docs/brief-23.md (this prompt verbatim, the state at the start, the plan with times) and docs/session-23-checklist.md. Point CLAUDE.md's reading list at them; keep brief-22 among the older briefs. Commit.
3. After the index run has exited: build with --release and copy the binaries to work/bin/s23-start/. Run `gunfinger-eval baseline session-23-start`.

Steps:

1. Setup (until about 03:15): the steps above.

2. Recluster (about 2 h; full NAS clustering took 85 minutes in session 6 on fewer records). Copy work/reports/library-nas-dnb-peaks/ (the old map and duplicate clusters) and the other reports the re-scoring needs to work/s23/before/ first. Then `map-library` and `clusters --from-peaks` with the NAS store at the new revision, full (not `--reuse-pairs`, which keeps the old pairs and misses the new uploads), with every verdict in docs/pair-verdicts.txt. The 17 corpus clusters must reappear; any difference is a bug or a finding. Write down what changed against the old clusters: members added (newer uploads, the newly indexed files), members gone (1.mp3), clusters split or joined by the new verdicts, the weakest same and strongest different pair, new borderline pairs (40-80% coverage). Borderline pairs become listening questions (step 6), never verdicts. Experiment file.

3. Re-score with the fresh clusters, offline, without searching again (about 1 h): the saved reports of 0111, 0112, 0116 and 0118, and the three development mixes at NAS scale, scored with the old and the new clusters (`scripts/analysis/devscore.py` or the session's scripts; check first that the old clusters reproduce the published numbers). This isolates the clusters' effect: the nine confident detections session 20 scored wrong; (c)'s 79 marks in 0116 and its 17 marks on the development mixes' true plays; 0111's 106 of 218; 0116's (a1), (a2), (c) and (c)+(a1) counts. Every changed count with its cause. Experiment file.

4. Run again at the new revision (about 3 h, one heavy run at a time): the store now holds about 800 more files, which can add related versions and wrong answers. In this order: a. the three development mixes at NAS scale; b. `shared` on the panel's development half (0111's runs); c. `shared-mixes` on docs/panels/shared-mixes-s21.json (0116), then the rips variant (0118) if time allows. Compare each with step 3's re-scored numbers, so the library's effect is separate from the clusters'. Experiment file.

5. The ambiguity note, measured (about 1 h 30 min). Before any count, write down in the experiment file: the note's definition, which is (c) exactly as session 21 defined it, unchanged; what the note would print on a tracklist line (draft wording, for example "(shares this passage with: Artist - Title)"); and the criteria for calling it worth proposing, for example at most 1 in 20 real true plays flagged and at least half the wrong confident plays flagged. Then measure on step 4's runs (or step 3's where step 4 did not finish): true confident plays flagged in the three real development mixes (each with the file it names and why, such as a remix, VIP or edit, another rip the clusters missed, or the Ryme Tyme kind of near-identical version); wrong confident plays flagged in the panel's development half and in the rendered mixes; and the three development mixes' default tracklists as they would read with the note (an analysis script, as scripts/analysis/tracklist_view.py does; no product change). Report against the criteria. Adopting it is our decision. Experiment file.

6. If time remains, in this order: a. a question file for the listening page (`gunfinger-eval questions ...`, docs/audition-questions.md) with session 21's two questions (Sonar: is the hifi-junglist PRO 012 A upload the original or the Mark System Revision; Kemal - Animation (Silent Extent Remix) against corpus/library/extra/a-kemal-animation-sour.mp3) and step 2's borderline pairs, under work/audition/s23/, never answered by an agent; the notes give the exact command, full absolute paths, the question and the times. b. A proposed second shared-material panel from the new `different` verdicts (families not in the frozen panel), split into development and validation halves by a seeded draw before any search, frozen in docs/panels/, not searched this session. Leave out the five test-derived versions and every file of the frozen panel's validation half.

7. A read-only review by a subagent, starting by 11:45; fix what it finds.

8. Wrap-up (from 12:20): a "Session 23" section at the end of docs/session-notes.md (what the fresh clusters changed; how much of the shared-material problem was stale clusters and how much the library; the ambiguity note's numbers against its criteria, with the tracklists as they would read; listening requests with full paths, question and times; decisions waiting). Update docs/status.md, the checklist and the roadmap. `regress session-23-start` identical with the final build; scripts/check.sh green (with rustup's toolchain first on PATH so the Linux clippy pass runs); the NAS store's revision unchanged from the one recorded after the index run. Commit. Last, print what was done, the checks, and anything left.

Rules for this session:
- Hard end 13:00. Steps shrink from the end: 6, then 4c, then 4b. The review starts by 11:45 and the wrap-up by 12:20 whatever is left. An unfinished step is reverted, or committed only in a working, tested state, and named in the notes.
- The product does not change: no change to crates/gunfinger or gunfinger-core behaviour; harness and analysis scripts may change (with tests where they are code). `regress session-23-start` identical; scripts/check.sh green.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or reports, rescore nothing, and never run test evaluation 5. Never identify, decode or index EM_1999_02_07_-_Ed_Rush_Optical.mp3 at the NAS root. Run nothing on the candidate mixes of experiment 0065 (including the mixed CDs it lists and the NAS's sety/vertical.mp3).
- The recall panel's and the shared-material panel's validation halves stay unsearched: no panel run, score or look at their detections. Clustering may query their files as new cluster members, as it did before the panels existed; that is clustering, not a panel run; say in the notes if it happened.
- The five test-derived versions are never used to tune anything: Terminal 2, the Orders remixes, the Marky & Bungle Funktion, the Clear Skyz Break remix and 1996 VIP, and the Black Barrel Beachball.
- The NAS (/Volumes/atlas/Music/dnb) is read, never written (its .gunfingerignore included). The NAS peak store is read-only for this session: no `index`, `prune` or `--retry-skipped`. The index run is the only writer tonight; record the revision after it exits and check it at the end.
- Agents never write verdicts or listening answers (docs/pair-verdicts.txt and work/audition/answers.jsonl change only by hand; the first commit only records the 17 new verdict lines). Manifests do not change, including corpus/sets/nocny-trans-vertical-2002-08-23/tracklist.toml. The frozen panels do not change.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines, written before the next experiment starts; evaluation JSON in docs holds file names only; cross-platform code (ADR 0020); no filename- or tag-derived logic in the product.
- Out of scope (ADR 0017): report edits, review workflows, and features that fix tags or the library.
- corpus/ is private and never committed.
- Leave alone every process this session did not start, including the index run.
- Runs longer than 30 minutes start detached under `screen`; heavy runs one at a time. NAS-scale harness runs overwrite work/reports/library-nas-dnb-peaks/, so copy what you need first. Keep scratch data under a few GB.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step. Check `git status` before every commit and never use `git add -A`. Do not push. No AI attribution in commits.
```

## State at the start of session 23

- 02:18. `git status`: the two expected files only. The diff was the
  17 new `different` lines at the end of `docs/pair-verdicts.txt` and
  the status entry of 2026-10-16, nothing else; committed as `d39fdca`
  with the prompt's message. Experiments 0001-0119 (the next is 0120);
  ADRs 0001-0020. Test set: 4 of 5 evaluations used (none in this
  session).
- The `index` run (pid 40404) is running: 39,072 records at
  02:19, 39,282 at 02:23. Waited for; never signalled.
- Processes this session did not start are left alone.
  - The harness's saved NAS index is 4.0 GB and is rebuilt by the first
  NAS-scale run; the CLI's is rebuilt by the first store-only
  `identify`.

## Plan, with times

Start 02:18; the session proper from the end of the index run;
hard end 13:00.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1 | setup: brief, checklist; after the index run: store revision, `doctor`, build, baseline | 0:45 | 03:15 |
| 2 | copy the NAS reports to `work/s23/before/`; `map-library`; `clusters --from-peaks` (session 18's queries) under `screen`; what changed; experiment 0120. Meanwhile: the harness's holder list and the re-scoring scripts, tested | 1:45 | 05:00 |
| 3 | re-score 0111, 0112, 0116, 0118 and the development mixes' NAS scans with old and new clusters, offline; experiment 0121 | 1:00 | 06:00 |
| 4 | at the new revision, one at a time: a. the three development mixes (`--verify scan`, and every referenced track left out); b. `shared` on the development half; c. `shared-mixes` s21, then the rips plan; experiment 0122 | 3:00 | 09:15 |
| 5 | the ambiguity note: definition, wording and criteria written first in experiment 0123; then the counts, the tracklists | 1:30 | 10:45 |
| 6 | if time remains: a. questions in `work/audition/s23/`; b. a proposed second panel, frozen, not searched | 1:00 | 11:45 |
| 7 | read-only review by a subagent, from 11:45 at the latest; fixes | 0:35 | 12:20 |
| 8 | wrap-up: notes, status, checklist, roadmap, `regress`, checks, store revision, commit | 0:40 | 13:00 |

Steps shrink from the end: 6, then 4c, then 4b. The review starts by
11:45 and the wrap-up by 12:20 whatever is left. Offline analysis of a
finished run may overlap the next heavy run; heavy runs never overlap.

## Decisions taken up front (written at 02:30, before any run)

- **The clustering command is session 18's** (`work/s18/clusters.sh`):
  `clusters --from-peaks --manifest razorjack-2003-03-29 --manifest
  stakka-skynet-knowledge --manifest nocny-trans-vertical-2002-08-23
  --recall-panel 2026`, full, no `--reuse-pairs`, with the verdicts file
  as it is after `d39fdca`. The same queries as the clusters in use, so a
  difference comes from the store and the verdicts, not from the query
  list. The shared-material panel's files are not added as queries
  (conservative: the panel is frozen with its recordings); corpus files
  of its validation half (Clockwork, The Nine) are corpus queries, as in
  every clusters run, which is clustering, not a panel run.
  `--cut-sparsest` is not used: if a verdict conflicts with a chain of
  joins, the run stops and names the chain, and that is a finding.
- **Members added** are classified by the store's record dates: written
  after session 18's clusters run (2026-10-14 03:38) and before tonight's
  index run (newer uploads), written by tonight's index run (from 01:52
  on 2026-10-16), or older (a record the old clusters had and did not
  join: a finding). **Members gone**: no record now (`1.mp3`), judged
  `different`, or other (a finding).
- **What step 3 can and cannot do offline.** The reports keep, for each
  confident play, only the strongest holder outside its recording by the
  old clusters. Where the fresh clusters put that holder in the play's
  own recording, the mark is *removed at most*: a weaker holder at 60
  hits or more may still mark it, which no saved report records. Step 3
  reports such marks as an upper bound on removals, and step 4's runs
  give the exact count. The panel's recordings (`docs/panels/`) stay as
  frozen; the fresh clusters join them where they are used together (the
  oracle's played recording is the panel's list and the file's cluster).
- **A harness change for step 4, before its runs:** `shared-mixes`
  stores every file its holder lookup finds (hits at least 30, strongest
  first) beside the strongest outside the own recording, so (c) can be
  computed under either clusters file on one run, and a small command
  runs the same lookup for a list of files and spans (for the panel's
  `shared` queries, which have no holder, and for the `identify`
  reports' tracklists). The lookup itself, its index and its 60-hit mark
  do not change. Tested; harness only.
- **The panel's oracle with fresh clusters** (0111 and step 4b): a
  detection of a file in the fresh cluster of a file of the played
  recording is the played recording (right), as the oracle's "any rip of
  the played recording" says; in the *played left out* index such a file
  (a rip the old clusters missed and the index kept) makes the query
  right confident, and it is listed by name.

## Step 5's pre-registration: the ambiguity note (written at 02:50, before any count under the fresh clusters)

Also the first part of experiment 0123, committed now; nothing below
changes after a count is seen. Anything computed later that is not
named here is exploratory and decides nothing.

- **The note's definition is (c), as `docs/brief-21.md` defines it.**
  For each confident segment of a play: the file's stored peaks within
  the segment's span in the file (`track_start_seconds` to
  `track_end_seconds`), searched over the turntable ladder (0.92-1.08)
  against the run's own index (the files present after its leave-outs,
  the fullest 1% of lists skipped as in the run); the value is the hits
  of the strongest detection of any file outside the file's own
  recording (its NAS duplicate cluster, the fresh one from step 2, and
  the panel recording that holds it). A segment is marked at 60 hits or
  more. A play (one file's detections at least possible, joined across
  gaps of at most 90 s, confident when a segment is) takes the most
  favourable value over its confident segments, the lowest; the play is
  *flagged* when that value is at least 60, so every confident segment
  is marked.
- **What the note would print** (draft): the tracklist entry (ADR 0017's
  grouping) of a flagged confident play ends with `(shares this passage
  with: Artist - Title; Artist - Title)`, naming each file at 60 hits or
  more outside the play's own recording by its tags (its file name when
  untagged), distinct names, strongest first, at most three and then
  `and N more`. An entry is noted when any of its confident plays is
  flagged. Nothing else in the tracklist changes; the table and JSON
  would carry the holders per play.
- **Criteria for calling it worth proposing**, counted per play, both
  required:
  1. *Real true plays*: at most 1 in 20 (5%) of the true confident plays
     of the three development mixes flagged. True plays: the plays
     holding a confident detection that the verified NAS scan credits to
     a referenced track under the fresh clusters (session 21's second
     sample), everything indexed, from step 4's runs (4a's scans and
     4c's `shared-mixes` runs of the three mixes).
  2. *Wrong plays*: at least half of the wrong related confident plays
     of the rendered mixes flagged, with the played recording left out
     (0116's design and oracle, from step 4c's run).
  With fewer than 20 true plays or fewer than 10 wrong related plays,
  the counts are reported and nothing is called worth proposing. Where
  step 4 did not finish, step 3's re-scored reports stand in, with
  their (c) values bounded as written above, and that is said.
- **Reported beside the criteria, deciding nothing**: the panel's
  `shared` queries of the main case (4b; per wrong confident query and
  per wrong play, the note looked up with `gunfinger-eval holders` and
  the query's own leave-out); 0118's rips variant; the rendered true
  plays (drawn tracks) and the positive control; the *everything*
  index; the possible tier is out of scope (the note is defined for
  confident plays).
- **Each flagged true play** of the development mixes is listed with the
  file it names and why, in this order: another rip the clusters missed
  (`pair` covers 80% or more of the shorter file); a version
  judged `different` that lines up almost whole (the Ryme Tyme kind:
  60-80% coverage or a `different` verdict above 80%); a remix, VIP or
  edit by its title (coverage under 60%); anything else, named.
- **The tracklists** of the three development mixes as they would read
  with the note: `gunfinger --config ~/.config/gunfinger/nas-dnb.toml
  identify <mix> --store-only -f json` at the new revision (the
  configuration, turntable playback), entries rebuilt as
  `scripts/analysis/tracklist_view.py` does, and the holders of each
  confident play's confident segments looked up with `gunfinger-eval
  holders` (everything indexed). No product code changes.
