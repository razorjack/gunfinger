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
- [x] Key-lock: measured (0009), key-locked rungs (0010); `identify
  --playback turntable|key-lock|both`, default unchanged. Cost on an idle
  machine still to be timed.
- [~] Scale proxy with reversed copies. Code (uncommitted):
  `crates/gunfinger-eval/src/synthetic.rs` (time-reversed copies stretched
  in time and frequency, up to 121), `--synthetic-copies N` on `robust` and
  `scan` (scan report `scale-scan-<set>-copies-N.json`, never read by
  calibrate). Queued by `work/scripts/queue-after-both.sh` (waits for the
  combined-ladder run): related, clusters check, then robust control and
  dev scan at 10, 30, 100 copies; logs `work/logs/scale-*`.
- [ ] Lookup-cost levers
- [~] Related recordings from the self-match. Code (uncommitted):
  `clusters::self_match` (refactor of `find`; check that
  `work/reports/duplicate-clusters.json` duplicates equal
  `duplicate-clusters-before-refactor.json`), `related.rs`, command
  `gunfinger-eval related` -> `work/reports/related-recordings.json`.

## 3. Command line

- [x] Colour
- [x] Position in the track
- [x] Same-audio rows collapsed
- [x] Timeline view
- [x] `explain`
- [x] `listen`
- [x] Export formats (csv, cue, tracklist; `show` renders saved reports)
- [x] Batch identification
- [x] Completions, man page, `--quiet`, progress line
- [x] Configuration file
- [x] `doctor`
- [x] `prune`
- [x] Remembered failed files (and too-long files)
- [x] TUI: `review`, a line-based interactive loop over a report (numbered
  plays, listen by number or time). A full-screen TUI was not built: it
  would add a terminal UI dependency for little beyond `review`.

## Wrap-up

- [ ] README, AGENTS, roadmap, calibration, status up to date
- [ ] Morning summary at the top of `docs/notes-for-owner.md`
