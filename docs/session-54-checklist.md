# Session 54 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-54.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] Digests of `docs/pair-verdicts.txt` and the NAS clusters file in
  `work/s54/listening-digests-start.txt`
- [x] Start build `work/bin/s54-start/`; `regress session-53-linux` and
  `regress session-53-linux-nas` identical

## Steps

- [x] 1. The design, in the brief before code
- [x] 2. ADR 0022 (accepted 2026-10-27), committed before the code
- [x] 3. `docs/same-titles.txt` (one entry), its parser and tests; the
  scoring in every judging place; the separate count in human output
  and JSON; a test crediting a same-title detection and not a
  different-title one; `scripts/check.sh` green
- [x] 4. `regress` at both scales with the changed build; every
  difference explained; calibrate's strongest false and false accepted
  at both scales
- [x] 5. Baselines `session-54-linux` and `session-54-linux-nas`;
  experiment 0233 (wrong answers before and after at both scales, seeds
  2026-2028; calibrate before and after)
- [x] 6. `docs/calibration.md` and the roadmap's bullets on China Cup's
  wrong answers
- [x] 7. Read-only review by a subagent; findings fixed
- [~] 8. Wrap-up: notes "Session 54", status, checklist; final checks
