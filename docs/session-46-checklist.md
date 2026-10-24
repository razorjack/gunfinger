# Session 46 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-46.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `docs/pair-verdicts.txt` digest in
  `work/s46/listening-digests-start.txt` (and the clusters file's in
  `work/s46/clusters-digest-start.txt`)
- [~] Start build `work/bin/s46-start/`; `regress session-32-linux` and
  `regress session-33-linux-nas` identical

## Steps

- [~] 1. Diagnostic build: the analysis's time by stage and playback,
  repeated analyses; the `x86-64-v2` build beside the start build;
  experiment 0212
- [ ] 2. Pre-registration (at least three candidates, exactness
  arguments and tests, timing protocol, stop point), committed before
  any candidate is timed
- [ ] 3. Candidates, each tested and timed; the combination of those
  that pass
- [ ] 4. Adoption in master: tests, 11 development scans at both
  scales, clusters sample, both regressions; stored peaks of a few
  corpus files in a scratch store if `index` code changed
- [ ] 5. Experiments: 0212 (breakdown and `x86-64-v2`), each candidate,
  the adopted combination's times
- [ ] 6. Read-only review by a subagent; findings fixed
- [ ] 7. Wrap-up: notes "Session 46", status, checklist, roadmap's speed
  bullet, low-bandwidth note; final checks (`scripts/check.sh`, both
  regressions, verdicts and clusters digests, `git status` clean)
