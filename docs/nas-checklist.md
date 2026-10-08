# Portable store and NAS evaluation: checklist

`[x]` done and committed, `[~]` in progress, `[ ]` not started, `[-]`
blocked (with the reason). Scope and rules: `docs/brief-nas.md`.

## Setup

- [x] Brief, checklist and the NAS analysis plan (`docs/nas-plan.md`)
- [x] Release build and a baseline for `regress` in this worktree
  (`session-4-start` copied from the main checkout; `regress` against the
  main checkout's corpus and peaks, read only: identical after items 1-4)

## Code

- [x] 1. Progress while `index` lists the library (also `identify`,
  `explain`, `stats`, `doctor`, `prune`)
- [x] 2. Tags in the peak store (probe, sidecar, `index` fills them in;
  survey, `prune`, `doctor`)
- [x] 3. Names in the report (renderers use them). No new schema version:
  the report's rule is that fields may be added without one
- [x] 4. A store-only index (`--store-only`, automatic without a library;
  `doctor` checks the store alone)
- [x] 5. Tests and README
- [x] 6. The harness against another library (`--other-peaks-dir`;
  design in the brief)
- [x] 7. `gunfinger-eval map-library`
- [x] 8. Clusters from chosen queries (`clusters --from-peaks
  --other-peaks-dir`)

Items 6-8 are one commit (they share `main.rs`). Checked on a scratch
other library under `work/scratch-other/` (clones of 260 corpus files, an
extra copy of a held-out recording, transcoded rips of an indexed and a
held-out recording; audio moved away before the harness ran): the map
found 260 copies and the 2 missing files; the 17 corpus clusters
reappeared and both rips were found in a second round; the sweep against
both libraries had the same recall and wrong answers as the corpus sweep,
with the indexed recording's rip found and counted correct and the
held-out recording's copy and rip left out; the development scan 11/11
and leave-out 3 8/11, no wrong identifications. `regress` stays identical.

## Merge

- [x] Rebased onto session 4's end, `scripts/check.sh` green,
  fast-forwarded into master; worktree and branch removed

## After the NAS index (docs/nas-plan.md)

Session 5 (`docs/brief-5.md`) ran the analysis from the NAS peak store
alone; the NAS was not read.

- [x] Step 1: indexing as it happened; the tags pass was run by the
  owner (experiment 0035)
- [x] Step 2: tag coverage (experiment 0035)
- [x] Step 3: the content map: 247 of 262 corpus files have a NAS copy
  (experiment 0036)
- [x] Step 4: clusters at NAS scale: the 17 corpus clusters reappear,
  170 further rips, 76 borderline pairs (experiment 0037)
- [x] Step 5: related recordings (experiment 0038)
- [x] Step 6: the development set, both matchers (experiment 0039)
- [x] Step 7: the sweep: skip at 240 seeds 2026-2029 (experiment 0040);
  today's matcher seeds 2026 and 2027 (experiment 0041)
- [x] Step 8: the proxy against reality (experiment 0042)
- [x] Step 9: posting lists and famous breaks (experiment 0043)
- [x] Step 10: identifying without the NAS (experiment 0044; internal
  disk only, no other disk attached)
- [-] Step 11: references for the test set: the owner's part; not
  started (ground truth does not change in session 5)
- [x] Step 12: calibration figures and the matcher decision: figures in
  `docs/calibration.md` as measurements; no rule changed (brief 5); the
  decision is the owner's
