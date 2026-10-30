# Session 62 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-62.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] Digests of `docs/pair-verdicts.txt` and the NAS clusters file in
  `work/s62/listening-digests-start.txt`
- [~] Start build `work/bin/s62-start/` from master; `regress
  session-54-linux` and `regress session-54-linux-nas` identical

## Steps

- [x] 1. Pre-registration committed before any run: the frozen rule,
  the runs, the scores, the cost, the bar and what stops the proposal;
  the census's classes
- [ ] Harness: `--front-end` for `shared` and `recall`; the census in
  `stereo-width`; tests; `regress` identical with the run build
- [ ] 2. The fault census (thresholds checked on session 42's faults,
  then the NAS sample)
- [ ] 3. Runs: panels, `recall`, generated mixes, noise conditions,
  scans, timing
- [ ] 4. Experiments: the census; the panels and `recall`; the mixes,
  noise and cost
- [ ] 5. Decision note "Session 62"; the roadmap's "Stereo faults before
  the downmix"; the proposals note's §1
- [ ] 6. Read-only review by a separate agent; findings fixed
- [ ] 7. Notes ("Session 62"), status, checklist; final checks
  (`scripts/check.sh`, both regressions with the final build, the
  verdicts file and the clusters file's digest unchanged, `git status`
  clean)
