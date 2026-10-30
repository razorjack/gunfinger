# Session 62 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-62.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] Digests of `docs/pair-verdicts.txt` and the NAS clusters file in
  `work/s62/listening-digests-start.txt`
- [x] Start build `work/bin/s62-start/` from master; `regress
  session-54-linux` and `regress session-54-linux-nas` identical

## Steps

- [x] 1. Pre-registration committed before any run: the frozen rule,
  the runs, the scores, the cost, the bar and what stops the proposal;
  the census's classes
- [x] Harness: `--front-end` for `shared` and `recall`; `mixes` at NAS
  scale; the census in `stereo-width`; tests; `regress` identical with
  the run build
- [x] 2. The fault census (thresholds checked on session 42's faults,
  then the NAS sample; 0248)
- [x] 3. Runs: panels, `recall`, generated mixes, noise conditions,
  scans, timing (`recall`'s damaged sets not run: time box, the bar
  already failed)
- [x] 4. Experiments: the census (0248); the panels and `recall` (0249);
  the mixes, noise and cost (0250)
- [x] 5. Decision note "Session 62"; the roadmap's "Stereo faults before
  the downmix"; the proposals note's §1
- [x] 6. Read-only review by a separate agent; findings fixed
- [x] 7. Notes ("Session 62"), status, checklist; final checks
  (`scripts/check.sh`, both regressions with the final build, the
  verdicts file and the clusters file's digest unchanged, `git status`
  clean)
