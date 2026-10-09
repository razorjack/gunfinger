# Session 10 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-10.md`
for scope and rules.

## Setup

- [x] The owner's reviewed notes and status: `scripts/check.sh` green,
  committed (`9116ec3`)
- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Release build of `9116ec3` (`work/bin/s10-start/`); `baseline
  session-10-start` (session 9's reports at 262 tracks)
- [x] The NAS store at the start: 32,441 `.peaks`, 32,441 `.tags`, 1,558
  `.skip` (13 failed, 122 too short, 1,423 too long), `library.txt`;
  revision `5701f221f7b48ee1`, as expected; census
  `work/s10/census-start.json` (3,399.5 h, tags on 32,035 records); no
  file newer than brief 9
- [x] `doctor --config ~/.config/gunfinger/nas-dnb.toml` (35.2 s wall):
  31 patterns, 504 audio files left out, 33,999 audio files = 32,441
  current records + 1,558 passed over; no records of ignored or gone
  files

## Items

- [ ] 2a. Verdict links only between files the run searches, never to
  ignored files; other verdicts reported with path and line; tests;
  regress identical
- [ ] 2b. `second-library/<path>` references: valid with
  `--other-peaks-dir`, absent at 262 tracks; `validate`, `scan`,
  scoring at both sizes; tests; the owner's manifest edit; validate at
  both sizes; regress identical
- [ ] 2c. The NAS recall panel: drawn by seed, development and
  validation halves by family, saved in `docs/panels/`; the development
  half's excerpts rendered from the NAS
- [ ] 2d. Manifest NAS references and the panel's development sources
  as extra `clusters` queries; tests; regress identical
- [ ] 3. `map-library`; the NAS clusters run (wall, CPU, memory); the
  experiment (against session 6, borderline pairs, sparse joins, panel
  sources); meanwhile the track-range message and the listening pack;
  after it, the pack for new pairs
- [ ] 4. The NAS baseline: development scan, leave-outs 3 and 11, sweeps
  2026-2029, `calibrate`; razorjack-2003-03-29 at both sizes with
  leave-out 3; against 0051, 0053, 0063; calibration register
- [ ] 5. The panel's development half: recall per speed and kind, wrong
  answers, held-out answers, why each miss failed
- [ ] 6. The 2003 mix: Phantom Force left out; the PHUD1 rip left out;
  the Kinetic tease

## Wrap-up

- [ ] Experiments; notes ("Session 10", the listening pack first);
  status; calibration register; roadmap
- [ ] The NAS store's revision at the end: `5701f221f7b48ee1`
- [ ] Commit; the message to the owner
