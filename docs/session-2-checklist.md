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

- [ ] Robustness harness and first runs
- [ ] Key-lock: measure, then tempo-only ladder prototype
- [ ] Scale proxy with reversed copies
- [ ] Lookup-cost levers
- [ ] Related recordings from the self-match

## 3. Command line

- [ ] Colour
- [x] Position in the track
- [ ] Same-audio rows collapsed
- [ ] Timeline view
- [ ] `explain`
- [ ] `listen`
- [ ] Export formats
- [ ] Batch identification
- [ ] Completions, man page, `--quiet`, progress
- [ ] Configuration file
- [ ] `doctor`
- [ ] `prune`
- [ ] Remembered failed files
- [ ] TUI

## Wrap-up

- [ ] README, AGENTS, roadmap, calibration, status up to date
- [ ] Morning summary at the top of `docs/notes-for-owner.md`
