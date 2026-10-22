# Session 36 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-36.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `docs/pair-verdicts.txt` digest in
  `work/s36/listening-digests-start.txt`
- [x] Start build `work/bin/s36-start/`; `regress session-32-linux` and
  `regress session-33-linux-nas` identical (1:12; 8:16)

## Steps

- [x] 1. `front_partition` and its tests; the part size from A/B at 16
  and 32 workers; the validation (check, regressions, development scans
  and leave-outs, clusters sample, full clusters run); experiment 0169;
  committed only if every comparison is identical (all identical; 0169)
- [x] 2. Stage timers in a diagnostic build; experiment 0170
- [x] 3. Narrower hits: grouped as 4 bytes, raw as 6 bytes; experiment
  0171 (both slower, not kept)
- [x] 4. Grouping without `part` (taking and grouping 35%); experiment
  0172 (slower, not kept)
- [x] 5. Repeated hashes: counted, experiment 0173 (below the stop point,
  not implemented)
- [x] 6. (Ahead) `identify` A/B on the development mixes; the worker
  count recommendation (experiment 0174)
- [~] 7. Read-only review by a subagent; findings fixed

## Wrap-up

- [x] "Session 36" in `docs/session-notes.md`, with the handoff
- [x] `docs/status.md`, the roadmap's "Where search time goes", the
  research note, this checklist
- [ ] Final checks: `scripts/check.sh` green; `regress session-32-linux`
  and `regress session-33-linux-nas` identical with the final build;
  `docs/pair-verdicts.txt` unchanged; `git status` clean
