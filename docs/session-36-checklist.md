# Session 36 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-36.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `docs/pair-verdicts.txt` digest in
  `work/s36/listening-digests-start.txt`
- [ ] Start build `work/bin/s36-start/`; `regress session-32-linux` and
  `regress session-33-linux-nas` identical

## Steps

- [ ] 1. `front_partition` and its tests; the part size from A/B at 16
  and 32 workers; the validation (check, regressions, development scans
  and leave-outs, clusters sample, full clusters run); experiment 0169;
  committed only if every comparison is identical
- [ ] 2. Stage timers in a diagnostic build; experiment
- [ ] 3. Narrower hits: grouped as 4 bytes, raw as 6 bytes; experiments
- [ ] 4. Grouping without `part` (only if taking and grouping are above
  about 15%)
- [ ] 5. Repeated hashes: count first
- [ ] 6. (Ahead) `identify` A/B on the development mixes; the worker
  count recommendation
- [ ] 7. Read-only review by a subagent; findings fixed

## Wrap-up

- [ ] "Session 36" in `docs/session-notes.md`, with the handoff
- [ ] `docs/status.md`, the roadmap's "Where search time goes", the
  research note, this checklist
- [ ] Final checks: `scripts/check.sh` green; `regress session-32-linux`
  and `regress session-33-linux-nas` identical with the final build;
  `docs/pair-verdicts.txt` unchanged; `git status` clean
