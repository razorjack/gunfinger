# Session 53 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-53.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] Digests of `docs/pair-verdicts.txt` and the NAS clusters file in
  `work/s53/listening-digests-start.txt`
- [~] Start build `work/bin/s53-start/`; `regress session-32-linux` and
  `regress session-52-linux-nas` identical

## Steps

- [ ] 1. ADR 0021 (accepted 2026-10-27), ADR 0008 marked; committed
  before the code change
- [ ] 2. The default matcher uses `SpansAtSpeed(0.006)`; the old rule an
  opt-in harness variant; `--apart-in-speed` decided; tests (the
  predicate at the boundary, a ride's second side kept); help, README
  and docs; `scripts/check.sh` green
- [ ] 3. `regress` at both scales with the changed build; every
  difference explained; the CLI's `identify` on the three development
  mixes at 262 tracks and NAS scale, before and after, compared line by
  line
- [ ] 4. Baselines `session-53-linux` and `session-53-linux-nas`;
  experiment 0232 with CPU A B B A
- [ ] 5. `docs/calibration.md`, the roadmap's "Speed that wanders within
  a play", the decision note's status line
- [ ] 6. Read-only review by a subagent; findings fixed
- [ ] 7. Wrap-up: notes "Session 53", status, checklist; final checks
