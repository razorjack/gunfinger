# Session 15 brief: the verdicts by ear applied, two measurements for decisions

Written by the agent for itself on 2026-10-12, so the plan survives context
compaction. Work autonomously and never stop to ask. When something is
ambiguous, take the conservative option, write the decision down and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-15-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 11" to the end).
4. `docs/status.md` (newest entries at the bottom).
5. `docs/pair-verdicts.txt`, ADRs 0006, 0008, 0013 and 0014, experiments
   0052, 0055, 0070, 0076 and 0088, and the roadmap items "The possible
   tier at NAS scale" and "A confidence statistic for shared material".
6. The newest file in `docs/experiments/`.

`docs/brief-2.md` to `docs/brief-14.md` hold rules that still apply
unless this brief changes them (among them: edit no configuration file;
test fixtures are generated, never copied from the corpus or the NAS;
runs longer than 30 minutes start detached under `screen`; heavy runs one
at a time, each with wall time, CPU time and peak memory; NAS-scale
harness runs overwrite `work/reports/library-nas-dnb-peaks/`, so copy it
first and put the earlier reports back after an acceptance run).

## The prompt, verbatim

```text
Start session 15 of Gunfinger, an overnight session: apply the
new verdicts, then measure two candidate rules for our decisions
(experiments A and B below), then, if time is left, one speed step. It
runs unattended from about 02:30 and must finish by 10:30. The NAS is mounted at /Volumes/atlas. Files may
be added to the NAS during the session; that is expected.
Work autonomously and never stop to ask. When something is ambiguous,
take the conservative option, write the decision down and continue.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it),
   docs/brief-14.md and docs/session-14-checklist.md (brief-2 to brief-13
   hold rules that still apply unless this prompt changes them),
   docs/session-notes.md (from "Session 11" to the end), docs/status.md
   (newest entries at the bottom, including "2026-10-12: the
   first listening with the tool"), docs/pair-verdicts.txt, ADRs 0006,
   0008, 0013 and 0014, experiments 0052, 0055, 0070, 0076 and 0088, and
   the roadmap items "The possible tier at NAS scale" and "A confidence
   statistic for shared material".
2. `git status` should be clean at f3b3191. Write docs/brief-15.md (this
   prompt verbatim, the state at the start, the plan with times) and
   docs/session-15-checklist.md. Point CLAUDE.md's reading list at them;
   keep brief-14 among the older briefs. Commit.
3. Build with --release and copy the binaries to work/bin/s15-start/. Run
   `gunfinger-eval baseline session-15-start`. Record the NAS peak store's
   state (scripts/analysis/store_revision.py): 33,596 records and revision
   f9f9bc706195a71b expected.

Context. The whole listening pack and seven folders were judged by ear with
the listening page last night: 24 new lines in docs/pair-verdicts.txt.
The meaning of a verdict (our note, 2026-10-12): two rips of the same
vinyl by different people, however different they sound, are `same`;
files that differ in even a few bars, such as two edits, are `different`.
Item 1's Sonar join is now judged (the PRO012 rip is the original Sonar,
`different` from the Mark System Revision), which was the only reason
clusters runs needed `--cut-sparsest`. Evaluation 4 at NAS scale showed
the main open problem: a related version sharing a passage, Evol Intent's
VIP of The Nine (489 hits, 2:30-5:07) confident inside The Nine's own
play (868 hits, 2:24-6:26). Experiment 0076 showed the confidence rule
alone cannot keep a version off a shared passage. Two decisions wait on
measurements: a containment rule, and the possible tier at NAS scale.

Steps:

1. Setup (20 min): the steps above.

2. The verdicts applied (about 1 h).
   - Rebuild the duplicate clusters at both library sizes from the saved
     pairs (`clusters --from-peaks --reuse-pairs`, as `pair-review`
     prints it, with and without `--other-peaks-dir`). First without
     `--cut-sparsest`: if no chain links two files judged `different`,
     the option is no longer needed; say so, and keep it if a chain
     remains (name the chain to judge by ear).
   - Report what changed: clusters joined, split or confirmed by the new
     verdicts, and any verdict the clusters cannot honour.
   - Recompute the development scores at both sizes under the new
     clusters (a new NAS baseline of the development sets if the scores
     depend on the clusters), against session-15-start. Never the test
     set.
   - A short experiment file (next number after 0089).

3. Experiment A: the containment rule (about 2 h). Measurement only; no
   rule changes in the product or the harness defaults.
   - The candidate rule: a confident play whose span lies inside a
     stronger play of a file from another cluster (more hits, over the
     same passage of the mix) is reported as shared material, not as an
     identification. Make the containment tolerance (seconds outside the
     stronger play's span) and an optional hit-ratio bound (the
     contained play's hits over the stronger play's: experiment 0076
     found 2-24% for remixes and 55-60% for The Nine's VIP) parameters,
     and report a small grid of them, not one tuned point.
   - Apply it, offline where possible, to every stored development
     report: the development scans at 262 tracks and at NAS scale and
     their leave-outs, the sweeps, recall, robust, the 2003 mix at both
     sizes, and experiment 0076's shared passages played whole. Count,
     per report kind: wrong confident answers it removes, right ones it
     would demote (a correct identification contained in a stronger
     play of another cluster, for example an unjoined copy of the same
     recording), and what it cannot fix (with the played file left out,
     no stronger play exists).
   - The rule must be judged on data it was not designed on. Write down
     which reports informed the parameter grid and which only tested it,
     and say plainly that a new development mix is still needed before
     any decision.
   - An experiment file, and a short summary.

4. Experiment B: the possible tier at NAS scale (about 1.5 h).
   Measurement only.
   - From stored NAS-scale development reports (and new runs only on the
     development sets, if a report is missing), at thresholds of 60, 80,
     100 and 130 hits: possible plays per hour of mix, and how many are
     the right recording, a related version over the played track
     (shared material), or unrelated (chance), using the manifests and
     the verdicts by ear. Also the weak true plays each threshold would
     lose, and the strongest unrelated chance at NAS scale against
     experiment 0052.
   - An experiment file, and a short summary.

5. If at least 2 h remain: the next speed step (experiment 0088: sorting
   each asset's offsets is 39% of the search's CPU). Profile first;
   change only what keeps detections identical; check like experiments
   0084 and 0086 (`regress session-15-start` identical, then the 2003 mix
   at NAS scale with the old and new builds, segments byte-identical,
   timings). If the time is short, skip it and say so.

6. A read-only review of the session's code by a subagent (if any code
   changed); fix what it finds.

7. Wrap-up (30 min): a "Session 15" section at the end of
   docs/session-notes.md with, for each of the two decisions (the
   containment rule, the possible tier), the numbers, what they show and
   do not show, and a recommendation; what the verdicts changed; anything
   left to judge by ear (with full absolute paths, the question and
   the times). docs/status.md, the checklist, the roadmap items touched.
   `regress session-15-start` identical; scripts/check.sh green; the NAS
   store's revision. Commit. Last, print what was done,
   the checks, the decisions waiting and anything left.

Rules for this session:
- The held-out test set (ed-rush-optical-essential-mix) is out of every
  step: do not open its manifest or any of its reports
  (work/reports/evaluation-*-scan-ed-rush-optical-essential-mix.json and
  any NAS-scale copy), do not rescore evaluation 4, and do not use it to
  choose any parameter. Test evaluation 5 is not run. The five
  test-derived versions (Terminal 2, the Orders remixes, the Marky &
  Bungle Funktion, the Clear Skyz Break remix and 1996 VIP, the Black
  Barrel Beachball) are never used to tune any rule. Never identify,
  decode or index EM_1999_02_07_-_Ed_Rush_Optical.mp3 at the NAS root,
  and run nothing on the candidate mixes of experiment 0065.
- The NAS (/Volumes/atlas/Music/dnb) is read, never written. The NAS peak
  store is read-only: no `index`, `prune` or `--retry-skipped` against
  it (ignore patterns for two mixed CDs will be added and the store pruned in
  the morning; until then the store stays as it is). Check its revision
  at the end. Harness runs may rebuild the saved index in work/indexes/
  by design.
- Agents never write verdicts or listening answers:
  docs/pair-verdicts.txt and work/audition/answers.jsonl do not change.
  Manifests do not change.
- The matcher, the confidence rule and the product's detections do not
  change; experiments A and B are measurements, with any new code in the
  harness or scripts/analysis, opt-in. `regress session-15-start` must be
  identical at the end; scripts/check.sh green.
- Out of scope: the Rust version and the answer-log lock, report edits,
  the edits experiment (edits are `different` by our
  rule), development-mix choices.
- Commit after each step; check `git status` before every commit and
  never use `git add -A`. Do not push. Each experiment file is written
  before the next experiment starts (at most 40 lines).
```

## State at the start of session 15

- 02:27. `git status` clean at `f3b3191` on `master`. Experiments
  0001-0089; ADRs 0001-0016. Test set: 4 of 5 evaluations used (none in
  this session).
- `/Volumes/atlas/Music/dnb` mounted
  (935 entries at the
  root).
- Saved indexes: `identify`'s
  `~/.cache/gunfinger/indexes/nas-dnb-peaks-23935cfc56936b67.index`
  (3,795,332,786 bytes, 2026-10-11 03:25) and the harness's
  `work/indexes/library-nas-dnb-peaks.index` (3,798,396,993 bytes,
  2026-10-11 15:28).
- `docs/pair-verdicts.txt` holds 35 verdicts: 11 before session 14 and the
  24 of 2026-10-12 (the whole pack, and the corpus Sonar `same`
  as the PRO012 rip). The clusters in use
  (`work/reports/duplicate-clusters.json`,
  `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`) predate
  them.
- Test-set reports present on disk and never to be opened:
  `work/reports/evaluation-{1,2,3,4}-scan-ed-rush-optical-essential-mix.json`
  and `work/reports/scan-ed-rush-optical-essential-mix.json`. Analysis
  scripts name development reports explicitly; no glob over `scan-*`.

## Plan, with times

Start 02:30; hard end 10:30.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1 | setup: brief, checklist, build, `regress session-14-start`, baseline, store revision | 0:25 | 02:55 |
| 2 | the verdicts applied: clusters at both sizes, what changed, development scores, experiment 0090 | 1:00 | 03:55 |
| 3 | experiment A, the containment rule: offline analysis of stored reports, a grid, experiment 0091 | 2:00 | 05:55 |
| 4 | experiment B, the possible tier at NAS scale, experiment 0092 | 1:30 | 07:25 |
| 5 | speed step, only if at least 2 h remain before the wrap-up (start by 07:50) | 2:00 | 09:25 |
| 6 | read-only review of the session's code by a subagent; fixes | 0:25 | 09:50 |
| 7 | wrap-up: notes, status, checklist, roadmap, regress, checks, store revision, commit | 0:40 | 10:30 |

The speed step is the first to give way: it starts only if step 4 ends by
about 07:50 (2 h for it, then the review and the wrap-up). If it starts
and is not done by 09:30, it is stopped and reverted, and its finding
written down.

## Conservative decisions taken up front

- The clusters are rebuilt from the saved pairs only (`--reuse-pairs`);
  no clusters search runs. The reports in use are copied to
  `work/s15/before/` first, so every comparison has the old clusters.
- Experiments A and B read stored reports wherever they exist; their code
  lives in `scripts/analysis/` (Python, reading JSON), so the product and
  the harness defaults do not change. New searches run only on the
  development sets and the 0076 passages, one at a time.
- The scores of runs made before the new clusters are recomputed
  offline from their detections where the scoring allows it; a harness
  run is repeated only where the score cannot be recomputed from a
  stored report.
- Nothing reads a test-set report or manifest: scripts take explicit
  development report paths and refuse a path naming
  `ed-rush-optical-essential-mix`.
