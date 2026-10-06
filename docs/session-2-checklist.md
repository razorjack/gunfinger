# Session 2 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started. See `docs/brief-2.md` for scope and rules.

## Setup

- [x] Brief, checklist and notes file; `CLAUDE.md` points to them

## 1. Test infrastructure

- [x] Determinism check (experiment 0007: deterministic, no fix needed)
- [x] Synthetic fixtures and end-to-end test (experiment 0008: shared kicks extend chains)
- [x] Regression command with saved baselines (`baseline`, `regress`; baseline `session-2-start`)
- [x] Property tests (plays, timecode, hashing)

## 2. Experiments on transformed copies

- [x] Robustness harness and first runs (experiment 0009)
- [~] Key-lock: measure, then tempo-only ladder prototype. Measured in
  0009. Prototype: `speed::Rung` (Turntable/KeyLocked), `key_lock_ladder()`,
  `points_at_tempo` (hop scaled only), `gunfinger-eval robust --ladder
  turntable|key-lock|both` (uncommitted in core and eval). Results so far:
  key-lock ladder alone finds 40/40 key-locked excerpts at ±2, ±5, +8%
  (median 656-973 hits, held-out max 12, wrong possible only the Clockwork
  remix); turntable conditions need the turntable ladder. Dev scan with the
  refactor: 4181 detections identical. Running: `robust --ladder both`
  (log `work/logs/robust-both.log`, report
  `work/reports/robust-seed-2026-both.json`). Then: experiment 0010, cost on
  an idle machine, decide default vs `--key-lock` flag, commit.
- [ ] Scale proxy with reversed copies
- [ ] Lookup-cost levers
- [ ] Related recordings from the self-match

## 3. Command line

- [x] Colour
- [x] Position in the track
- [x] Same-audio rows collapsed
- [x] Timeline view
- [x] `explain`
- [x] `listen`
- [x] Export formats (csv, cue, tracklist; `show` renders saved reports)
- [x] Batch identification
- [~] Completions, man page, `--quiet` (all done), progress
- [x] Configuration file
- [x] `doctor`
- [x] `prune`
- [x] Remembered failed files (and too-long files)
- [ ] TUI

## Wrap-up

- [ ] README, AGENTS, roadmap, calibration, status up to date
- [ ] Morning summary at the top of `docs/notes-for-owner.md`
