# Session 52 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-52.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] Digests of `docs/pair-verdicts.txt` and the NAS clusters file in
  `work/s52/listening-digests-start.txt`
- [x] Start build `work/bin/s52-start/`; `regress session-32-linux` and
  `regress session-33-linux-nas` identical before the prune

## Steps

- [x] 1. `doctor` and `prune` (no `--yes`) list exactly the five files;
  where the five files appear today
- [x] 2. `prune --yes`; `doctor` (40,678 records, none ignored); revision
  before and after; `clusters --from-peaks --reuse-pairs`
- [x] 3. The standard evaluation at NAS scale on the pruned store; every
  difference traced to the five files; panel entries that name a pruned
  file
- [x] 4. `baseline session-52-linux-nas`; `regress session-32-linux`
  identical; experiment 0231
- [ ] 5. The places that name the NAS baseline, the record count and the
  clusters digest
- [ ] 6. Read-only review by a subagent; findings fixed
- [ ] 7. Wrap-up: notes "Session 52", status, checklist; final checks
