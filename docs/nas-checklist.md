# Portable store and NAS evaluation: checklist

`[x]` done and committed, `[~]` in progress, `[ ]` not started, `[-]`
blocked (with the reason). Scope and rules: `docs/brief-nas.md`.

## Setup

- [x] Brief, checklist and the NAS analysis plan (`docs/nas-plan.md`)
- [ ] Release build and a baseline for `regress` in this worktree

## Code

- [x] 1. Progress while `index` lists the library (also `identify`,
  `explain`, `stats`, `doctor`, `prune`)
- [x] 2. Tags in the peak store (probe, sidecar, `index` fills them in;
  survey, `prune`, `doctor`)
- [ ] 3. Names in the report (schema version, renderers)
- [ ] 4. A store-only index (`--store-only`, automatic without a library)
- [ ] 5. Tests and README
- [ ] 6. The harness against another library (`--library`,
  `--library-map`)
- [ ] 7. `gunfinger-eval map-library`
- [ ] 8. Clusters from chosen queries

## After the NAS index (docs/nas-plan.md)

- [ ] Steps 1-12, each measurement as an experiment
