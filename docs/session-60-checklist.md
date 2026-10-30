# Session 60 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-60.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [~] Brief, checklist and `CLAUDE.md` committed
- [x] Digests of `docs/pair-verdicts.txt` and the NAS clusters file in
  `work/s60/listening-digests-start.txt`
- [ ] Start build `work/bin/s60-start/` from master; `regress
  session-54-linux` and `regress session-54-linux-nas` identical

## Steps

- [ ] 1. Pre-registration committed before any timing: layouts and
  sizes, the proof obligation, the replay data, the equality checks, the
  timing protocol, the sketch's cost, dense buffers, scratch bytes and
  peak memory, the bar for adoption
- [ ] 2. The prototype with tests (never rejects an asset with a line:
  random and adversarial buffers, wraparound, saturation) and a replay
  harness against today's path; no SIMD, no `unsafe`
- [ ] 3. Replays (pass rate and line equality per layout), then full
  searches for the replay winner
- [ ] 4. Experiments: the replays; the full-search timings
- [ ] 5. Adoption under the registered bar, or the negative result
- [ ] 6. Read-only review by a separate agent; findings fixed
- [ ] 7. Notes ("Session 60"), status, checklist; final checks
  (`scripts/check.sh`, both regressions with the final build, the
  verdicts file and the clusters file's digest unchanged, `git status`
  clean)
