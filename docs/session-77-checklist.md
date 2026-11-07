# Session 77 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-77.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Digests of `docs/pair-verdicts.txt` and the NAS clusters file in
  `work/s77/listening-digests-start.txt`
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] Start build `work/bin/s77-start/`; byte-identical to `s75-final`,
  stores unchanged, so the start regressions are skipped

## Steps

- [x] 1. Pre-registration in the brief (the rule, the reports rescored,
  what counts as a changed conclusion), committed before the fix
- [x] 2. The scoring fix at the common scorer, tested; the counterexample
  runner's workaround; every consumer of `level`; an offline `rescore`
- [x] 3. Rescoring the listed reports; a table per experiment
- [x] 4. `regress`: query identity and counts, scan set and leave-outs,
  "incomparable", provenance from `baseline`, `--strict`, unit tests
- [x] 5. Final regressions (also `--strict`); provenance for both
  baselines
- [x] 6. Experiment 0279
- [x] 7. Read-only review by a subagent; fixes
- [x] 8. Wrap-up: notes "Session 77", status, roadmap queue item 1,
  decisions queue, checklist; final checks
