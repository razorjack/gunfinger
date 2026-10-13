# Session 17 brief: speed (search and first index), small items if time allows

Written by the agent for itself on 2026-10-13, so the plan survives context
compaction. Work autonomously and never stop to ask. When something is
ambiguous, take the conservative option, write the decision down and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-17-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 15" to the end).
4. `docs/status.md` (newest entries at the bottom).
5. ADRs 0011, 0014, 0017 and 0018; experiments 0064, 0075, 0086, 0088
   and 0093; the roadmap's "Where search time goes".
6. `crates/gunfinger-core/src/search/lines.rs`.
7. The newest file in `docs/experiments/`.

`docs/brief-2.md` to `docs/brief-16.md` hold rules that still apply
unless this brief changes them (among them: edit no configuration file;
test fixtures are generated, never copied from the corpus or the NAS;
runs longer than 30 minutes start detached under `screen`; heavy runs one
at a time; NAS-scale harness runs overwrite
`work/reports/library-nas-dnb-peaks/`, so copy it first). This session
changes one of session 16's: the machine is idle, so timings are taken
and reported, on the idle machine only, old and new builds alternating.

## The prompt, verbatim

```text
Start session 17 of Gunfinger, an early-morning session on speed, plus
small items if time allows. It runs unattended from about 04:40 and must
finish by 10:30. The NAS is
mounted at
/Volumes/atlas. Work autonomously and
never stop to ask. When something is ambiguous, take the conservative
option, write the decision down and continue.

Context. Session 16 recorded our principle (ADR 0017): Gunfinger
is a tool you fire and forget. For such a user, speed matters in two
places: `identify` on a mix, and the first `index` of a large library.
The disk has about 36 GB free; keep scratch data under a few GB and
remove what you no longer need from work/.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it),
   docs/brief-16.md and docs/session-16-checklist.md (brief-2 to brief-15
   hold rules that still apply unless this prompt changes them),
   docs/session-notes.md (from "Session 15" to the end), docs/status.md
   (newest entries at the bottom), ADRs 0011, 0014, 0017 and 0018,
   experiments 0064, 0075, 0086, 0088 and 0093, the roadmap's "Where
   search time goes", and crates/gunfinger-core/src/search/lines.rs.
2. `git status` should be clean at b7ddfb6. Write docs/brief-17.md (this
   prompt verbatim, the state at the start, the plan with times) and
   docs/session-17-checklist.md. Point CLAUDE.md's reading list at them;
   keep brief-16 among the older briefs. Commit.
3. Check the machine is idle (no CPU-heavy processes
   you did not start; load average low). Build with --release and copy
   the binaries to work/bin/s17-start/. Run `gunfinger-eval baseline
   session-17-start`. Record the NAS peak store's state
   (scripts/analysis/store_revision.py): 33,596 records and revision
   f9f9bc706195a71b expected.

Steps:

1. Setup (20 min): the steps above.

2. Search speed (about 2.5 h). Experiment 0093's profile after its
   change: grouping each window's hits by asset, now with the
   pre-filter, 51% of the NAS search's CPU (the scatter of the counting
   sort); query analysis 30%. Profile first (0088's and 0093's method,
   `work/s15/profile.sh`, the 2003 mix at NAS scale), then take the
   larger cost that can be cut with identical results. Accept a change
   only with identical detections: `regress session-17-start` identical,
   the 2003 mix at NAS scale (`--store-only --playback both -f json`)
   byte-identical between the old and new builds, timed A B B A on the
   idle machine, with wall, CPU and peak memory. Peak memory must not
   grow by more than a few percent. An experiment file per change kept
   or rejected (next number after 0095).

3. Indexing speed (about 2 h). A first `index` of the NAS collection
   costs about 1.9 s of CPU per hour of audio, CPU-bound (experiment
   0064: a full index about 1.8 h over Ethernet). Profile where it goes
   (FFmpeg's decode and resampling against the spectrogram and peak
   picking, tags, store writes), on temporary stores only: the corpus
   library (262 tracks) and one NAS folder indexed into a scratch store
   (`--config /dev/null`, as 0064 did; never the NAS store). If Gunfinger's
   own analysis is a large share, make it faster with peak records that
   stay bit-identical (compare every record of the scratch stores, old
   build against new; any difference rejects the change). FFmpeg's
   options are part of the profile: a different resampler or rate
   changes every record, so it is out of scope. An experiment file.

4. If time remains, small items, each committed on its own:
   - Listening questions for the two strongest "unrelated" pairs not yet
     heard (docs/status.md, "2026-10-12: after session 15"): Watermelon ~
     300 Baud - Monotone (Mobiuz & Cycom remix), 41 hits, and Falcon -
     Fractles ~ Ryme Tyme - We Enter (Optical remix v3.0), 40 hits. The
     question is whether each pair shares material (a break, loop or
     sample) or the match is chance. Write them as a question file the
     listening page opens with `gunfinger-eval audition <file>` (a new producer
     only if it is the clean way; ADR 0016 and docs/audition-questions.md
     describe how), outside the default queue, with choices such as
     shared material / chance / unsure written to the answer log only.
     Check the page loads it (with a temporary answer log); never answer.
   - The roadmap's "Behaviour at scale" and "The possible tier at NAS
     scale", and docs/calibration.md if it repeats it: "chance grows past
     the possible tier from about 9,000 assets" rests on Rubba Rock, which
     was found by ear to be shared material (2026-10-12); every strongest
     "unrelated" match checked since session 10 was shared material.
     Correct the text with those facts; change no threshold.

5. A read-only review of the session's code by a subagent; fix what it
   finds.

6. Wrap-up (30 min): a "Session 17" section at the end of
   docs/session-notes.md (what got faster and by how much, measured on
   the idle machine; anything to judge by ear with full absolute
   paths, the question and the times), docs/status.md, the checklist, the
   roadmap. `regress session-17-start` identical; scripts/check.sh green;
   the NAS store's revision. Commit. Last, print what was
   done, the checks, and anything left.

Rules for this session:
- Timings only on the idle machine, old and new builds alternating; if
  another heavy process appears, say so and repeat the runs.
- The held-out test set (ed-rush-optical-essential-mix) is out of every
  step: do not open its manifest or reports, rescore nothing, and never
  run test evaluation 5. Never identify, decode or index
  EM_1999_02_07_-_Ed_Rush_Optical.mp3 at the NAS root, and run nothing on
  the candidate mixes of experiment 0065.
- The NAS (/Volumes/atlas/Music/dnb) is read, never written. The NAS peak
  store is read-only: no `index`, `prune` or `--retry-skipped` against
  it; index only into temporary stores. Check its revision at the end.
- Agents never write verdicts or listening answers:
  docs/pair-verdicts.txt and work/audition/answers.jsonl do not change.
  Manifests do not change.
- The matcher, the confidence rule, the peak records and the detections
  do not change: `regress session-17-start` identical at the end;
  scripts/check.sh green. A change to how postings are made from peaks
  changes `hash::design()`'s tag (AGENTS.md).
- Out of scope: the containment rule and the possible tier's threshold
  (waiting for our decision), development-mix choices, report edits (ADR
  0017), anything that changes output.
- Leave alone every process this session did not
  start.
- Commit after each step; check `git status` before every commit and
  never use `git add -A`. Do not push.
```

## State at the start of session 17

- 04:35. `git status` clean at `b7ddfb6` on `master`. Experiments
  0001-0095 (the next is 0096); ADRs 0001-0018. Test set: 4 of 5
  evaluations used (none in this session).
- `/Volumes/atlas` mounted
  over
  SMB,
  935
  entries
  at `/Volumes/atlas/Music/dnb`.
- The NAS peak store is on a local disk
  (`~/.local/share/gunfinger/nas-dnb-peaks`); the CLI's saved index of it
  (`~/.cache/gunfinger/indexes/`, 3.80 GB, 2026-10-11) and the harness's
  (`work/indexes/library-nas-dnb-peaks.index`, 3.80 GB) exist.
- Toolchain: rustc 1.98.0; `rust-version = "1.89"`.

## Plan, with times

Start 04:40; hard end 10:30.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1 | setup: brief, checklist, build, baseline, store revision | 0:20 | 05:00 |
| 2 | search speed: profile, change, benchmark, regress, A B B A, experiment | 2:15 | 07:15 |
| 3 | indexing speed: profile on scratch stores, change if worth it, records compared, experiment | 1:45 | 09:00 |
| 4 | small items: the two listening questions, the roadmap's text on chance at scale | 0:30 | 09:30 |
| 5 | read-only review by a subagent (started once step 3's code is committed, while step 4 runs); fixes | 0:30 | 10:00 |
| 6 | wrap-up: notes, status, checklist, roadmap, regress, checks, store revision | 0:30 | 10:30 |

No slack beyond step 4. If step 2 or 3 overruns by more than 20 minutes,
step 4 shrinks first; the review starts by 09:30 and the wrap-up by 10:00
whatever is left. An unfinished step is reverted or committed only in a
working, tested state, and named in the notes.

## Conservative decisions taken up front

- Timings: every timed run is one at a time, with nothing else of this
  session running (no build, no test, no other harness run). Before each
  timing series the load average and the top processes are recorded; a
  heavy process this session did not start means the series is repeated.
  The A B B A order shares any slow drift between both builds.
- Search: a change is kept only if `regress session-17-start` is
  identical, the four NAS reports of the A B B A series are
  byte-identical, and peak memory grows by at most 3%.
- Indexing: profiles and comparisons run only on scratch stores under
  `work/s17/` with `--config /dev/null`; the NAS folder is read-only
  input. A change is kept only if every record of the scratch stores is
  byte-identical between the old and new builds (the files as written,
  apart from nothing; if the record holds a write time, that field is
  named and compared apart).
- `regress session-17-start` runs after each kept change and once at the
  end with the final build.
- Listening questions: written outside the default queue
  (`work/audition/questions/`) unless the producer's default puts them
  there; the page is checked only with a temporary answer log and a copy
  of the verdicts file, and never answered.
