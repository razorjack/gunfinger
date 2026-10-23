# Session 44 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-44.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `docs/pair-verdicts.txt` digest in
  `work/s44/listening-digests-start.txt` (and the clusters file's in
  `work/s44/clusters-digest-start.txt`)
- [~] Start build `work/bin/s44-start/`; `regress session-32-linux` and
  `regress session-33-linux-nas` identical

## Steps

- [ ] 1. Pre-registration (populations, the split and how each kind is
  decided, the displaced share that opens the fan-out step, the
  fan-outs, the scores and the criterion), committed before any count
- [ ] 2. The harness, tested: the split in `verifier/pairs.rs`, the
  query fan-outs; `regress` identical
- [ ] 3. The runs at 262 tracks and at NAS scale
- [ ] 4. Experiments: the split; the fan-outs only if the registered
  share is reached, otherwise what the lost pairs mean for step 3
- [ ] 5. Read-only review by a subagent; findings fixed
- [ ] 6. Wrap-up: notes "Session 44", status, checklist, roadmap's
  bullet; final checks (`scripts/check.sh`, both regressions, verdicts
  and clusters digests, `git status` clean)
