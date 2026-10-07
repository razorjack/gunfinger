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

## After the NAS index (docs/nas-plan.md)

- [ ] Steps 1-12, each measurement as an experiment
