# Session 57 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-57.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] Digests of `docs/pair-verdicts.txt` and the NAS clusters file in
  `work/s57/listening-digests-start.txt`
- [x] Start build `work/bin/s57-start/`; `regress session-54-linux` and
  `regress session-54-linux-nas` identical

## Steps

- [x] 1. Pre-registration (runs, old reports, what counts as explained
  and as a finding), committed before the runs
- [x] 2. The runs, in scratch work directories, under tmux with logs
  - [x] the shared panels' development halves (both panels, NAS scale)
  - [x] the drops mixes of sessions 39 and 48 (262 tracks, NAS scale)
  - [x] `recall`
  - [x] the other development scans at NAS scale
  - [x] the sweeps of seeds 2027-2029 at NAS scale
  - [x] `robust`
  - [x] `grid`
  - [x] the long and ridden generated mixes
- [x] 3. Experiments from 0236, one per group (shared panels; drops;
  recall and scans; sweeps, robust and grid; long and ridden mixes),
  with ADR 0021's residual risk counted for the shared panels and drops
- [x] 4. Findings, if any: described, reproduced by the smallest
  command, first in the handoff (no product change)
- [x] 5. Roadmap bullets whose numbers changed; `docs/calibration.md`
- [ ] 6. Read-only review by a separate agent; findings fixed
- [ ] 7. Notes ("Session 57"), status, checklist; final checks
  (`scripts/check.sh`, both regressions with the final build, the
  verdicts file and the clusters file's digest unchanged, `git status`
  clean)
