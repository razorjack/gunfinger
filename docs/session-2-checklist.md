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
  --playback turntable|key-lock|both`, default unchanged; cost about 1.3×
  (0016).
- [x] Scale proxy with reversed copies (experiment 0012): chance grows
  slowly up to 21,109 assets; the scan at 100 copies swapped and was
  stopped.
- [x] Summed play evidence, offline null (experiment 0011): no change.
- [x] Lookup-cost levers: dropping the fullest lists (0013, full protocol
  0017; harness option, default unchanged) and triplet hashes estimated
  offline (0014, `gunfinger-eval hash-cost`).
- [x] Related recordings (0015); the `clusters` refactor gives the same
  duplicate clusters.
- [x] Key lock: full protocol with both ladders (0016); `playback` key in
  the configuration file.

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
