# Session 22 brief: part 1 of the FFT lengths' retrieval study

Written by the agent for itself on 2026-10-15, so the plan survives
context compaction. Work autonomously and never stop to ask. When
something is ambiguous, take the conservative option, write the decision
down and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-22-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 19" and "Session 21").
4. `docs/status.md` (newest entries at the bottom).
5. `docs/research/2026-10-14-matching-proposals.md` (proposal 2 and
   "Protections that apply to every proposal"); the roadmap's "Where
   search time goes"; ADRs 0004, 0006, 0008, 0014, 0017; experiments
   0020, 0033, 0047, 0059, 0109, 0117 and this session's.
6. `work/s19/py/fft_variant.py`, `work/s19/fft-arms.json`,
   `work/s21/fft-factor2.diff` (how session 21 built the variant).

`docs/brief-2.md` to `docs/brief-21.md` hold rules that still apply
unless this brief changes them (among them: edit no configuration file;
test fixtures are generated, never copied from the corpus or the NAS;
runs longer than 30 minutes start detached under `screen`; heavy runs one
at a time; NAS-scale harness runs overwrite
`work/reports/library-nas-dnb-peaks/`, so copy it first). No timing is
measured in this session, so no load series is needed.

## The prompt, verbatim

```text
Start session 22 of Gunfinger, a short morning session: part 1 of the FFT lengths' retrieval study, on the panels that exist. It runs unattended from about 09:15 and must finish by 11:59. The NAS is mounted at /Volumes/atlas, and no timing is measured in this session. Work autonomously and never stop to ask. When something is ambiguous, take the conservative option, write the decision down and continue.

Context.
- Session 19 (experiment 0109): turntable rungs analyse the mix with windows of round(1024 / speed) samples, some of which are slow FFT lengths. Choosing, within 2 samples (0.21%), the length whose FFT has small prime factors ("factor2") makes the first pass's turntable analysis 36.5% faster and `identify` 5-11% faster end to end. It changes peaks, so detections can change; session 19 described the retrieval study an adoption would need (docs/session-notes.md, "Session 19", "Decision: is a retrieval session for the FFT lengths worth running?"): a first-pass-only variant named in reports; the diagnostic sweep (speeds between rungs, sample offsets, damage, other rips, blends, short plays) with the first pass's lines traced before confidence and key-lock-only runs as controls; the three development mixes at 262 tracks and NAS scale with leave-outs; the generated mixes with a boundary tolerance fixed in advance.
- Session 21 (experiment 0117) ran the factor2 first-pass variant, built in a worktree never merged, on the shared-material panel's development half at NAS scale: no wrong confident query added, no right file lost, two titles changed at the 240-hit rule. That was the study's first stage.
- This session runs the stages that need no new harness: everything the existing panels measure. Adopting the lengths stays our decision; the session measures and adopts nothing. The parts that need new harness features (speeds between rungs, sample offsets, traced first-pass lines) are part 2, for a later session; name what they need.
- State at the start: git clean at 86b1b8d (session 21's last commit; check it); the NAS peak store holds 38,105 records, revision b285da9984422fb5.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it), docs/brief-21.md and docs/session-21-checklist.md (brief-2 to brief-20 hold rules that still apply unless this prompt changes them), docs/session-notes.md ("Session 19" to the end), docs/status.md (newest entries at the bottom), the research note's proposal 2, the roadmap's "Where search time goes", ADRs 0004, 0006, 0008, 0014 and 0017, experiments 0020, 0033, 0047, 0059, 0109 and 0117, and how session 21 built the variant (its brief, checklist and 0117).
2. `git status` should be clean. Write docs/brief-22.md (this prompt verbatim, the state at the start, the plan with times) and docs/session-22-checklist.md. Point CLAUDE.md's reading list at them; keep brief-21 among the older briefs. Commit.
3. Build with --release and copy the binaries to work/bin/s22-start/. Run `gunfinger-eval baseline session-22-start`. Record the NAS peak store's state.

Steps:

1. Setup (15 min): the steps above.

2. The variant (about 20 min). Rebuild session 21's factor2 first-pass-only variant in a worktree that is never merged (the second pass keeps today's lengths). Write down, before any run, what each comparison counts and the boundary tolerance for the generated mixes. The variant's harness reports must not overwrite or mix with work/reports/ (calibrate and regress read every report there): give it its own work directory, or copy and restore, and say how.

3. Runs, the variant against today's build, one heavy run at a time, in this order of priority (about 1 h 45 min, until about 11:05):
   a. `regress session-22-start` with the variant: the sweep and the three stakka-skynet-knowledge scans at 262 tracks, detection by detection.
   b. The three development mixes at NAS scale (`identify --store-only` or the harness's scans), today's build against the variant: right confident plays lost, wrong confident plays added, possible plays added or lost, boundaries moved.
   c. The generated mixes, seeds 2026 and 7 at 262 tracks: right and wrong plays, boundaries against the tolerance.
   d. The other sweep seeds (2027-2029) at 262 tracks against the diagnostic bar (at least 95% recall within ±5%, zero wrong).
   e. If time remains: `grid` (short plays at every window offset, experiment 0047), `robust` (damage, seed 2026), and the development mixes' leave-outs at NAS scale.
   Each comparison lists every changed detection with its cause where it can be named.

4. An experiment file (next number after 0118) with the counts per panel and every change of a confident result; and part 2's needs.

5. A read-only review by a subagent, starting by 11:15; fix what it finds.

6. Wrap-up (from 11:35): a "Session 22" section at the end of docs/session-notes.md (what the variant changes on each panel, whether any criterion failed, what part 2 needs, and that adoption is our decision); update docs/status.md, the checklist, the roadmap ("Where search time goes") and the research note (proposal 2's status line). `regress session-22-start` identical with the final build of the main tree; scripts/check.sh green (with rustup's toolchain first on PATH so the Linux clippy pass runs); the NAS store's revision. Remove the worktree. Commit. Last, print what was done, the checks, and anything left.

Rules for this session:
- Hard end 11:59. Step 3 shrinks from its end (e, then d) if time runs short. The review starts by 11:15 and the wrap-up by 11:35 whatever is left. An unfinished step is reverted, or committed only in a working, tested state, and named in the notes.
- The product on master does not change: the variant lives only in the worktree; `regress session-22-start` identical with the main tree's final build; scripts/check.sh green.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or reports, rescore nothing, and never run test evaluation 5. Never identify, decode or index EM_1999_02_07_-_Ed_Rush_Optical.mp3 at the NAS root. Run nothing on the candidate mixes of experiment 0065, including the NAS's sety/vertical.mp3. The recall panel's and the shared-material panel's validation halves stay unsearched.
- The five test-derived versions are never used to tune anything: Terminal 2, the Orders remixes, the Marky & Bungle Funktion, the Clear Skyz Break remix and 1996 VIP, and the Black Barrel Beachball.
- The NAS (/Volumes/atlas/Music/dnb) is read, never written. The NAS peak store is read-only: no `index`, `prune` or `--retry-skipped` against it. Check its revision at the end.
- Agents never write verdicts or listening answers; manifests do not change; the duplicate clusters are not rebuilt.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines, written before the next experiment starts; evaluation JSON in docs holds file names only; cross-platform code (ADR 0020).
- Out of scope (ADR 0017): report edits, review workflows, and features that fix tags or the library.
- corpus/ is private and never committed.
- Leave alone every process this session did not start.
- Runs longer than 30 minutes start detached under `screen`; heavy runs one at a time. NAS-scale harness runs overwrite work/reports/library-nas-dnb-peaks/, so copy what you need first. Keep scratch data under a few GB.
- Commit after each step. Check `git status` before every commit and never use `git add -A`. Do not push. No AI attribution in commits.
```

## State at the start of session 22

- 09:06. `git status` clean at `86b1b8d` on `master`. Experiments
  0001-0118 (the next is 0119); ADRs 0001-0020. Test set: 4 of 5
  evaluations used (none in this session).
-
- Files may be added to the NAS
  during the
  session; they do not change the NAS peak store.
- A worktree `work/probe-tree` (detached at `6eaa393`) exists from an
  earlier session; not this session's, left alone.
- The NAS peak store, the build and the baseline: setup step 3 below.

## Plan, with times

Start 09:06; hard end 11:59.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1 | setup: brief, checklist, build, baseline, store revision | 0:20 | 09:30 |
| 2 | the variant in a worktree; the comparisons and the tolerance written first; the work directories | 0:20 | 09:50 |
| 3a | `regress session-22-start` with the variant (262 tracks) | 0:10 | 10:00 |
| 3b | the three development mixes at NAS scale, both builds | 0:25 | 10:25 |
| 3c | generated mixes, seeds 2026 and 7, both builds | 0:15 | 10:40 |
| 3d | sweeps 2027-2029, both builds | 0:15 | 10:55 |
| 3e | if time remains: `grid`, `robust` seed 2026, the development mixes' leave-outs at NAS scale | 0:10 | 11:05 |
| 4 | experiment 0119; part 2's needs | 0:10 | 11:15 |
| 5 | read-only review by a subagent, from 11:15 at the latest; fixes | 0:20 | 11:35 |
| 6 | wrap-up: notes, status, checklist, roadmap, research note, `regress`, checks, store revision, worktree removed, commit | 0:24 | 11:59 |

Step 3 shrinks from its end (e, then d) if time runs short. The review
starts by 11:15 and the wrap-up by 11:35 whatever is left.

## Setup (done at 09:10)

- Release build: `cargo build --release` was current; binaries in
  `work/bin/s22-start/` (`gunfinger` and `gunfinger-eval` byte-identical
  to `work/bin/s21-final/`). `baseline session-22-start` saved from the
  reports of session 21's final `regress` (08:47-08:48), made by that
  identical build.
- The NAS peak store: 38,105 records, revision `b285da9984422fb5`
  (`work/s22/store-revision-start.txt`), as expected.
- Listening files: `docs/pair-verdicts.txt` sha256 `d9af100a…9586`,
  `work/audition/answers.jsonl` sha256 `dff04586…09be`, as at session 21's
  start; the development sets' manifest modification times recorded
  (`work/s22/listening-digests-start.txt`).

## Step 2: the variant and the comparisons (committed at 09:10, before any run of the variant)

### The variant

Session 21's patch (`work/s21/fft-factor2.diff`), applied with `git
apply` to a detached worktree at `9aecef4` (`work/s22-fft-tree`, never
merged; the core and the CLI are unchanged since `49f4c77`, where session
21 applied it): each first-pass turntable rung analyses the query with
session 19's factor2 length (`work/s19/fft-arms.json`: within 2 samples of
`round(1024 / s)`, the smallest largest prime factor of the complex FFT;
29 of 41 lengths change). The second pass (`refine`, through
`Rung::points`), the key-locked rungs, the stored peaks and the postings
keep today's lengths. One addition: `speed::design()` appends
`first-pass-turntable-fft=factor2-s19`, so a CLI report's
`search.matching` names the variant and `identify --save-dir` never
reuses a report of the other build. The diff:
`work/s22/fft-factor2.diff`; binaries `work/bin/s22-fft-factor2/`.

### Where the reports go

Two work directories, one per build, so nothing is written to
`work/reports/`: `--work work/s22/today` for today's build
(`work/bin/s22-start/`) and `--work work/s22/variant` for the variant.
Each starts with APFS clones (`cp -c`, no extra disk) of what the runs
read: `reports/duplicate-clusters.json`,
`reports/library-nas-dnb-peaks/{library-map,duplicate-clusters}.json`,
`indexes/library-nas-dnb-peaks.index` (the harness's saved NAS index,
current at `b285da9984422fb5`; postings are unchanged by the variant, so
both builds load it), the renders `sweep/`, `mixes/`, `grid/` and
`robust/`, and, for the variant, `baselines/session-22-start/`. Renders
are reused only when their plan matches (`render_once`, the sweep's
renders), so both builds search the same audio. The test set's reports
are never cloned or opened. `work/reports/` is written only by the
final `regress session-22-start` of the main tree. `--peaks-dir` stays
`work/peaks` (the corpus store, current).

### What each comparison counts (today's build against the variant)

A *detection* changes when any field differs (hits, windows, start, end,
track position, speed). For every panel: the number of identical and
changed detections, and the variant's hits over today's for detections
of the same file at the same moment (median, range). Every change of a
confident result is listed with its cause where one can be named: hits
crossing 240 or windows crossing 3 (the rule's edge, ADR 0008), a chain
joined or split, a segment moving between plays.

- **(a) `regress` at 262 tracks** (sweep seed 2026; the development scan
  and its leave-outs 3 and 11): `regress`'s own lines, and query by
  query: confident right answers lost, confident wrong answers added,
  possible-only answers gained or lost; per scan, referenced tracks
  identified, wrong confident detections, plays (by file) confident or
  possible added and lost, and boundaries moved; the calibrate line
  (weakest identifying, strongest false).
- **(b) The three development mixes at NAS scale**
  (`stakka-skynet-knowledge`, `razorjack-2003-03-29`,
  `nocny-trans-vertical-2002-08-23`; `gunfinger-eval --other-peaks-dir
  <NAS store> scan <set>`): plays by file, by the harness's scoring
  (credited within the manifest's window, rips through the clusters):
  right confident plays lost; wrong confident plays added (and removed);
  possible plays added or lost; play boundaries moved. The razorjack
  manifest's starts are Gunfinger's own boundaries, so boundaries are
  compared between builds only, never scored against it.
- **(c) Generated mixes**, seed 2026 (`--count 12`, as experiments 0020
  and 0047) and seed 7 (`--count 2`, as experiment 0107 and the existing
  report): each planned play's level (confident, possible, none) and
  credited file, `wrong_confident` and `wrong_possible`, and the
  boundary errors against the exact truth (`start_late_seconds`,
  `end_early_seconds`).
- **(d) Sweeps 2027-2029**: the diagnostic bar for each build (at least
  95% recall within ±5%, zero wrong answers), and query by query as in
  (a).
- **(e) If time remains**: `grid` seed 2026 (confident, possible, none
  per query; wrong confident), `robust` seed 2026, all conditions
  (confident, possible and wrong per condition and query), and the
  development mixes at NAS scale with every referenced track left out
  (`--leave-out 11`, `12` and `17`, as experiment 0115): wrong confident
  plays added or removed.

### The boundary tolerance (fixed now)

A play's start or end *moves* when the variant's differs from today's by
more than **1.0 s** (a tenth of a confidence window; peak changes alone
shift a first or last hit by a few hops, 12-13 ms each). Every move is
listed. In the generated mixes, where the truth is exact, a moved
boundary *got worse* when its error against the truth grew by more than
1.0 s; the criterion is that none got worse, and each one that did is named
with both errors. A play whose level changes is counted under its level,
not as a boundary.

### The criteria (from the study's design; adopting is our decision)

No wrong confident play or query added on any panel; no right confident
or possible file lost; the sweeps keep the diagnostic bar; no boundary in
the generated mixes worse by more than 1.0 s. A criterion that fails is
reported with every change behind it; nothing is adopted either way.

## Runs added after the plan (committed at 09:47, before any of them ran)

Steps (a)-(d) and the grid finished by 09:46, well ahead of the plan.
The prompt asks for "everything the existing panels measure" that needs
no new harness, and session 19's study names more of it than steps
(a)-(e) list. Queued after (e), one at a time, in this order, and
stopped at 11:05 wherever they are (`work/s22/queue2.sh`, `queue3.sh`):

1. **The key-lock-only control**: `--ladder key-lock sweep --seed 2026`
   with both builds. The variant changes no key-locked rung, so every
   detection should be identical; any difference is a fault in the
   variant, not a result.
2. **Other rips at NAS scale** (experiment 0059): `--other-peaks-dir
   <NAS store> sweep --seed 2026 --other-rips`, both builds; then seeds
   2027-2029 (last in the queue). Counted as the sweeps in (a).
3. **The other two development mixes at 262 tracks**
   (`razorjack-2003-03-29`, `nocny-trans-vertical-2002-08-23`): the
   scan, `--leave-out 3` and every referenced track left out (12, 17),
   both builds. Counted as the scans in (a).
4. **The recall panel's development half** at NAS scale (`recall --seed
   2026`, which searches only the development half), both builds: each
   source's answer (right confident, possible, nothing, wrong) query by
   query. The validation half stays unsearched.

The criteria and the tolerance are those of step 2.
