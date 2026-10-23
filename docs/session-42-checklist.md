# Session 42 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-42.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `docs/pair-verdicts.txt` digest in
  `work/s42/listening-digests-start.txt` (and the clusters file's in
  `work/s42/clusters-digest-start.txt`)
- [x] Start build `work/bin/s42-start/`; `regress session-32-linux` and
  `regress session-33-linux-nas` identical

## Steps

- [x] 1. Pre-registration (faults and levels, how they are applied,
  near-mono sources, channel choices, scores, what would count),
  committed before any fault is searched
- [x] 2. Harness: stereo faults as opt-in `robust` conditions, a
  harness-only `--query-channels`; tests; `regress` identical
- [x] 3. Runs: every fault and channel choice at 262 tracks, then at
  NAS scale
- [x] 4. Experiments 0194-0196 (cost of each fault to the downmix;
  channel choices; near-mono sources)
- [x] 5. Ahead: (a) a fault over a stretch of generated mixes; (b) a
  fault in the indexed reference (store-only scratch copy)
- [ ] 6. Read-only review by a subagent; findings fixed
- [ ] 7. Wrap-up: notes "Session 42", status, roadmap bullet; final
  checks (`scripts/check.sh`, both regressions, verdicts and clusters
  digests, `git status` clean)
