# Session 64 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-64.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] Digests of `docs/pair-verdicts.txt` and the NAS clusters file in
  `work/s64/listening-digests-start.txt`
- [x] Start build `work/bin/s64-start/`; `regress session-54-linux` and
  `regress session-54-linux-nas` identical before the prune

## Steps

- [x] 1. `doctor` and `prune` (no `--yes`) list exactly the four files;
  where the four files appear today
- [x] 2. `prune --yes`; `doctor` (40,674 records, none ignored); revision
  before and after; `clusters --from-peaks --reuse-pairs`
- [x] 3. The standard evaluation at NAS scale on the pruned store; every
  difference traced to the four files or to set-aside lists moving
- [x] 4. `baseline session-64-linux-nas`; `regress session-54-linux`
  identical; experiment 0254
- [x] 5. The evening's answers: session 54's and 58's pointers, the
  roadmap, `docs/calibration.md`, the stereo decision note; the places
  that name the NAS baseline, the record count and the clusters digest
- [x] 6. Read-only review by a subagent; findings fixed
- [x] 7. Wrap-up: notes "Session 64", status, checklist; final checks
