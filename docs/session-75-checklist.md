# Session 75 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-75.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] Digests of `docs/pair-verdicts.txt` and the NAS clusters file in
  `work/s75/listening-digests-start.txt`
- [~] Start build `work/bin/s75-start/`; `regress session-54-linux` and
  `regress session-64-linux-nas` identical before the prune

## Steps

- [x] 1. `doctor` and `prune` (no `--yes`) list exactly the 107 files (8
  peak records, 99 skip notes, 8 tag notes); where the 8 recorded files
  appear today
- [ ] 2. `prune --yes`; `doctor` (40,666 records, none ignored); revision
  before and after; `clusters --from-peaks --reuse-pairs`
- [ ] 3. The standard evaluation at NAS scale on the pruned store; every
  difference traced to the 8 files or to set-aside lists moving
- [ ] 4. `baseline session-75-linux-nas`; `regress session-54-linux`
  identical; experiment 0278
- [ ] 5. The places that name the NAS baseline, the record count and the
  clusters digest; the roadmap's "Library ignore file"
- [ ] 6. Read-only review by a subagent; findings fixed
- [ ] 7. Wrap-up: notes "Session 75", status, checklist; final checks
