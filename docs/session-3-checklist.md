# Session 3 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` blocked (with the reason). See `docs/brief-3.md`
for scope and rules.

## Setup

- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Release build; baseline `session-3-start` (the rerun at the start
  reproduced `key-lock-default` exactly)

## 1. Memory by phase

- [x] Peak memory while loading, building and searching, at several worker
  counts, on the scale proxy (experiments 0019, 0021)
- [x] Two-pass index build reading one peak record at a time, if the
  inference holds (commit 3cb535f)
- [x] Peak records dropped after the build in `identify` and `explain`
  (none are kept; `stats` reads them again)
- [x] `regress session-3-start`: identical detections

## 2. Reports and explain

- [x] Atomic report writes (commit a724809)
- [x] Search settings in each report (profile and hash design, confidence
  rule, library revision); `--save-dir` searches again when any differ
  (commit a724809; also the recording's path and the excerpt)
- [x] Evidence per window in `explain` (commit 84fae7f)

## 3. Generated mixes and the window grid

- [x] Seeded renderer of library-track mixes with exact truth (speeds,
  crossfades, bass-swap EQ, plays of 20-60 s, cuts, a returning track,
  held-out tracks)
- [x] Window-grid sweep: brief plays slid in 1 s steps, length and source
  position varied
- [x] Minimum aligned span in seconds (offline comparison on the
  harness's detections), compared with 3 windows (experiment 0020)
- [x] Boundary error against the truth

## 4. Where real mixes lose evidence

- [x] Per-play attribution on the development mix: solo against blended,
  frequency band, distance from the nearest rung, speed variation within
  a window
- [x] Robust conditions: wow (0.55 and 0.75 Hz), broadcast compression,
  beatmatched blends, combined damage (experiment 0022)

## 5. Second pass at the fitted speed

- [x] Opt-in second pass: one STFT at the fitted speed over each
  candidate's span, hits against that asset alone (commit 633da10)
- [x] Its own null and calibration (items 3 and 4; experiment 0024)

## 6. Default-change package (opt-in)

- [x] Second pass plus the common-hash filter; threshold from calibration
  data alone (dropped and skipped lists; `FITTED_RULE` 240; experiment 0026)
- [x] Every robust condition with the filter on and off, both ladders,
  brief excerpts, combined damage (experiments 0026, 0027)
- [x] Confident recall, possible recall and false candidates reported
  separately (experiment 0027)
- [x] Full protocol, including the padded index (experiments 0026, 0027)
- [x] Case for adoption in the notes

## 7. Before tracks are added

- [x] Fixed query panel with more sweep seeds (panels in `docs/panels/`;
  seeds 2026-2029, experiment 0025)
- [x] Duplicate clustering from stored peaks, checked against `clusters`
  on controlled cases (experiment 0023)

## Wrap-up

- [x] README, roadmap, calibration, status up to date
- [x] Summary at the top of `docs/notes-for-owner.md`
