# Session 18 brief: a third development mix, the containment rule and the possible tier on it

Written by the agent for itself on 2026-10-14, so the plan survives
context compaction. Work autonomously and never stop to ask. When
something is ambiguous, take the conservative option, write the decision
down and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-18-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 15" to the end).
4. `docs/status.md` (newest entries at the bottom).
5. ADRs 0004, 0006, 0008, 0017 and 0018; experiments 0060, 0076, 0091,
   0092, 0094 and 0099; the roadmap's "The possible tier at NAS scale"
   and "More development mixes"; `docs/calibration.md`'s possible-tier
   row.
6. The new manifest,
   `corpus/sets/nocny-trans-vertical-2002-08-23/tracklist.toml` (read
   only; never edit it).
7. The newest file in `docs/experiments/`.

`docs/brief-2.md` to `docs/brief-17.md` hold rules that still apply
unless this brief changes them (among them: edit no configuration file;
test fixtures are generated, never copied from the corpus or the NAS;
runs longer than 30 minutes start detached under `screen`; heavy runs one
at a time; NAS-scale harness runs overwrite
`work/reports/library-nas-dnb-peaks/`, so copy it first). This session
changes one of session 17's: each timing series records the load and the
busiest processes.

## The prompt, verbatim

```text
Start session 18 of Gunfinger, an overnight session on a new development mix. It runs unattended from about 03:15 and must finish by 12:00 (noon). The NAS is mounted at /Volumes/atlas. Work autonomously and never stop to ask. When something is ambiguous, take the conservative option, write the decision down and continue.

Context.
- A third development set was added on 2026-10-14: corpus/sets/nocny-trans-vertical-2002-08-23/ (vertical.mp3, 66:48, DJ Vertical's set from Nocny Trans, Radio Łódź, 2002-08-23, a YouTube upload of the broadcast; a cassette copy also exists). Its role is development, decided before anything searched it. The tracklist was written by hand, without Gunfinger ("more or less" on the times). The agent who launched this session transcribed it into tracklist.toml on request. References were found by tags and file names in the NAS peak store, never by searching the mix: the corpus file and every NAS file tagged with the same artist, title and version. `# related:` comments name other versions in the library, and `# unclear:` names the TI029 C side ("Descent", version not named). `gunfinger-eval validate` passes: 9 referenced and 9 absent at 262 tracks; 17 referenced (45 files) and 1 absent at NAS scale. The absent one is track 17, an ID that could not be named (1:01:25 to 1:04:40).
- Our decisions in the manifest: Blindside starts at 38:00, where it is teased (it plays in full from 40:35), so the scoring window covers the tease. Stab 2 Def starts at 46:53 and is blended in from 44:40; The Kraft is Mampi Swift. Mo-Funk - Slipstream is the original, not the Gridlok remix. "Dieselboy - The Descent (Phunckateck VIP)" is the canonical title (Discogs, checked manually); whether the TI029 C files hold this VIP or the original is not known.
- This mix is what experiment 0091 asked for before deciding the containment rule: it plays versions whose other versions are in the library. The Descent (Phunckateck VIP), Invid (E-Sassin VIP), Nephelim (Konflict remix) and We R The Ones (VIP) have their originals and other remixes indexed. Mutated (Version X), Final Judgement, Horns 2000, Lords of Motion (Tech Itch remix) and Slipstream have a remix or the original indexed.
- The NAS file sety/vertical.mp3 (40:06) is a different set by DJ Vertical (our note, 2026-10-14). It stays a candidate mix of experiment 0065: run nothing on it.
- The listening on 2026-10-14 (work/audition/answers.jsonl, seq 35 and 36): Watermelon ~ 300 Baud - Monotone (Mobiuz & Cycom remix) and Falcon - Fractles ~ Ryme Tyme - We Enter (Optical remix v3.0) both "shared". So every strongest match between different recordings checked so far, from 40 hits up, shares material, and no chance match has been heard above 40 hits. We have not decided the possible tier's threshold; it stays at 60.
- Mixed CDs: on 2026-10-14 we noted that Skynet & Stakka - Point of Arrival is a mixed CD. Hard Leaders - Listen Up (va-listen_up-hlcd9-2000-sour) is not mixed: it holds full tracks with crossfades of about 5 s, a continuous CD. The answer log still says "mixed" for Listen Up (seq 23); that note supersedes it, and agents do not edit the log. On request the launching agent added `skynet_and_stakka-point_of_arrival-1999-sour/` to the NAS's .gunfingerignore and ran `prune` on the NAS store, which deleted only those 14 records and tag notes. The store now holds 33,723 records, revision c835c5531c325274. Before the prune it held 33,737 (fd04f630699eacdb), 141 more than at session 17's end, because new files were indexed after session 17.
- docs/panels/recall-seed-2026.json names a Point of Arrival file (02-skynet_and_stakka_-_hal-sour.mp3) that the prune removed, so a NAS-scale `recall` run will not find its record. Do not redraw the panel. If you run recall, leave that item out, name it, and say how the counts compare with earlier runs.
- `regress` covers stakka-skynet-knowledge at 262 tracks, so the store change does not affect it. Saved indexes of the NAS store are stale after the prune: the CLI's (~/.cache/gunfinger/indexes/nas-dnb-peaks-23935cfc56936b67.index, 3.8 GB) and the harness's (work/indexes/library-nas-dnb-peaks.index, 3.8 GB, 2026-10-11). The disk has about 21 GB free. Once a new saved index of the current store exists, delete the stale ones. Keep scratch data under a few GB.
- Files may be added to the NAS during the session. Leave alone every process this session did not start. New NAS files are not in the store and do not affect store-only searches.
- Our principle (ADR 0017): Gunfinger is fired and forgotten. When several files match one played track, the user checks them all and finds the right title among them. Report the new mix from that view as well: for each entry of the default tracklist, is the right title among its files, and does any entry show only a wrong title?

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it), docs/brief-17.md and docs/session-17-checklist.md (brief-2 to brief-16 hold rules that still apply unless this prompt changes them), docs/session-notes.md (from "Session 15" to the end), docs/status.md (newest entries at the bottom), ADRs 0004, 0006, 0008, 0017 and 0018, experiments 0060, 0076, 0091, 0092, 0094 and 0099, the roadmap's "The possible tier at NAS scale" and "More development mixes", and docs/calibration.md's possible-tier row.
2. `git status` should be clean at 300e008. Write docs/brief-18.md (this prompt verbatim, the state at the start, the plan with times) and docs/session-18-checklist.md. Point CLAUDE.md's reading list at them; keep brief-17 among the older briefs. Commit.
3. Build with --release and copy the binaries to work/bin/s18-start/. Run `gunfinger-eval baseline session-18-start`. Record the NAS peak store's state (scripts/analysis/store_revision.py): 33,723 records and revision c835c5531c325274 expected.

Steps:

1. Setup (30 min): the steps above.

2. The new development set (about 3 h).
   a. Library-only checks (no search of the mix): link NAS copies to the manifest's references the way session 10 did for razorjack-2003-03-29 (`clusters --from-peaks` with the manifest). Search each `# unclear:` file against the VIP and the original, and each `# related:` file against the reference of its track: same recording, shared material, or unrelated?
   b. Scan the set at 262 tracks and at NAS scale, with leave-outs at NAS scale (`--leave-out 3`, a few seeds), and run `identify` with the CLI's defaults at NAS scale (`--store-only`, the default tracklist, and `-f json` for the record). Score by ADR 0004: referenced tracks identified, wrong confident identifications, possible plays. Give every miss and every wrong identification a cause (a related version, shared material, a blend, radio processing, speed, a missing record). Report track 17's slot apart from the others: anything Gunfinger finds there, confident or possible, is a candidate to judge by ear, not a scored result.
   c. The fire-and-forget view above, entry by entry.
   d. An experiment file (next number after 0099).

3. The containment rule (about 1 h). Apply 0091's recommended setting (passages, 10 s tolerance, with and without a 0.5 bound on the ratio of hits; scripts/analysis/containment.py) to the new mix's reports at both sizes and its leave-outs, and again to every stored report as 0091 did. Report what it removes (wrong plays of related versions) and what it costs (right plays demoted, and a right recording lost where only a weaker copy is indexed). End with a recommendation to adopt or not. Do not implement it: adopting it is our decision. An experiment file.

4. The possible tier (30 min). Record the two listening answers in the roadmap's "The possible tier at NAS scale" and docs/calibration.md's row: every match checked from 40 hits up shares material. Count the new mix's possible plays per hour at 60, 80, 100 and 130 hits (0092's method), and whether any true play in it is found only as possible. Change no threshold.

5. Listening questions (30 min), for what only a judgement by ear can settle: candidates in track 17's slot, the TI029 C side if the pair search does not decide it, and any wrong confident play whose cause is unclear. Use the existing producers of `gunfinger-eval questions` (ADR 0016, docs/audition-questions.md). Write the files outside the default queue (for example work/audition/s18/), and check that the page loads them with a temporary answer log and a copy of the verdicts file. Never answer. In the notes, give each question with full absolute paths, the question and the times.

6. If time remains (until about 10:15): search speed, with session 17's acceptance rules. The next targets after experiment 0098 are the pre-filter (about 22% of the search's CPU), grouping (20%), reading postings (16%) and analysing the mix (26%). Profile first. Keep a change only if `regress session-18-start` is identical, the 2003 mix's report at NAS scale (`--store-only --playback both -f json`) is byte-identical between the old and new builds, timed A B B A, and peak memory grows by at most 3%. Record the load and the busiest processes during each timing series. An experiment file per change kept or rejected.

7. A read-only review of the session's code and analysis by a subagent; fix what it finds.

8. Wrap-up (45 min): a "Session 18" section at the end of docs/session-notes.md, covering:
   - the new mix's score and causes, at both sizes;
   - the fire-and-forget view;
   - the containment recommendation;
   - the possible tier's facts;
   - the listening requests with full paths and times;
   - any proposed manifest change, as a proposal only.
   Also update docs/status.md, the checklist, the roadmap ("More development mixes", the possible tier), and ADR 0004's "Later" list (the third development set, its role and how its tracklist was written). `regress session-18-start` identical with the final build; scripts/check.sh green; the NAS store's revision. Commit. Last, print what was done, the checks, and anything left.

Rules for this session:
- Hard end 12:00. If a step overruns by more than 20 minutes, step 6 shrinks first, then step 5. The review starts by 10:45 and the wrap-up by 11:15 whatever is left. An unfinished step is reverted, or committed only in a working, tested state, and named in the notes.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or reports, rescore nothing, and never run test evaluation 5. Never identify, decode or index EM_1999_02_07_-_Ed_Rush_Optical.mp3 at the NAS root. Run nothing on the candidate mixes of experiment 0065, including the NAS's sety/vertical.mp3.
- The five test-derived versions are never used to tune any rule: Terminal 2, the Orders remixes, the Marky & Bungle Funktion, the Clear Skyz Break remix and 1996 VIP, and the Black Barrel Beachball.
- The NAS (/Volumes/atlas/Music/dnb) is read, never written; its .gunfingerignore is edited only by hand. The NAS peak store is read-only: no `index`, `prune` or `--retry-skipped` against it. Index only into temporary stores. Check its revision at the end.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt and work/audition/answers.jsonl do not change. Manifests do not change, the new one included. Propose any change in the notes instead.
- The matcher, the confidence rule, the possible tier's threshold, the peak records and the detections do not change: `regress session-18-start` identical at the end; scripts/check.sh green. The containment rule and the tier are measured, not adopted.
- Out of scope (ADR 0017): report edits, review workflows, and features that fix tags or the library.
- corpus/ is private and never committed.
- Leave alone every process this session did not start.
- Runs longer than 30 minutes start detached under `screen`; heavy runs one at a time. NAS-scale harness runs overwrite work/reports/library-nas-dnb-peaks/, so copy it first.
- Commit after each step. Check `git status` before every commit and never use `git add -A`. Do not push. No AI attribution in commits.
```

## State at the start of session 18

- 03:10. `git status` clean at `300e008` on `master`. Experiments
  0001-0099 (the next is 0100); ADRs 0001-0018. Test set: 4 of 5
  evaluations used (none in this session).
- `/Volumes/atlas` mounted, 935 entries at
  `/Volumes/atlas/Music/dnb`. Processes this session did not start were left alone.
- The NAS peak store (`~/.local/share/gunfinger/nas-dnb-peaks`): 33,723
  records, revision `c835c5531c325274` expected (checked in setup).
  Stale saved indexes of it: the CLI's
  `~/.cache/gunfinger/indexes/nas-dnb-peaks-23935cfc56936b67.index`
  (3.81 GB, 2026-10-14 02:23) and the harness's
  `work/indexes/library-nas-dnb-peaks.index` (3.80 GB, 2026-10-11).
- Toolchain: rustc 1.98.0; `rust-version = "1.89"`.

## Plan, with times

Start 03:15; hard end 12:00.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1 | setup: brief, checklist, build, baseline, store revision | 0:30 | 03:45 |
| 2a | `clusters --from-peaks` with the new manifest (detached); `pair` on the unclear and related files | 0:45 | 04:30 |
| 2b | scans at 262 and NAS scale, NAS leave-outs (3 seeds), `identify` CLI at NAS scale; scoring and causes | 1:30 | 06:00 |
| 2c-d | the fire-and-forget view; experiment 0100 | 0:45 | 06:45 |
| 3 | containment rule on the new reports and every stored report; experiment | 1:00 | 07:45 |
| 4 | the possible tier: the listening answers in the roadmap and calibration; counts at 60-130 | 0:30 | 08:15 |
| 5 | listening questions in `work/audition/s18/`; page checked with temporary files | 0:30 | 08:45 |
| 6 | search speed, if time remains (profile first; A B B A) | to 10:15 | 10:15 |
| 7 | read-only review by a subagent; fixes | 0:30 | 10:45-11:15 |
| 8 | wrap-up: notes, status, checklist, roadmap, ADR 0004, regress, checks, store revision | 0:45 | 12:00 |

If a step overruns by more than 20 minutes, step 6 shrinks first, then
step 5. The review starts by 10:45 and the wrap-up by 11:15 whatever is
left. An unfinished step is reverted or committed only in a working,
tested state, and named in the notes.

## Conservative decisions taken up front

- The new manifest is read, never edited. Proposed changes go to the
  notes.
- The new set is scored as a development set by ADR 0004. Track 17's
  slot (1:01:25-1:04:40, `reference = []`) is scored by the harness as an
  absent slot (a confident play there counts as wrong), and reported
  apart as candidates to judge by ear.
- NAS-scale harness reports: `work/reports/library-nas-dnb-peaks/` is
  copied to `work/s18/before/` before the first NAS run.
- The harness's saved index is rebuilt by its first NAS run (the stale
  one is not used for any result: `--saved-index rebuild` on that run).
  The CLI's saved index is rebuilt by the first `identify --store-only`.
  The stale files are deleted once their replacements exist.
- The recall panel is not run unless step 2a's `clusters` needs it; if
  it does, the pruned Point of Arrival item is left out and named.
- Containment and the tier are measured offline with the existing
  scripts; no product or harness rule changes.
- Listening questions are written to `work/audition/s18/`, outside the
  default queue; the page is checked only with a temporary answer log and
  a copy of the verdicts file, and never answered.
- Timings (step 6): each series records the
  load average and the top processes; old and new
  builds alternate A B B A.
