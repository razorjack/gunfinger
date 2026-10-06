# Session 3 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` blocked (with the reason). See `docs/brief-3.md`
for scope and rules.

## Setup

- [x] Brief, checklist; `CLAUDE.md` points to them
- [ ] Release build; baseline `session-3-start`

## 1. Memory by phase

- [ ] Peak memory while loading, building and searching, at several worker
  counts, on the scale proxy
- [ ] Two-pass index build reading one peak record at a time, if the
  inference holds
- [ ] Peak records dropped after the build in `identify` and `explain`
- [ ] `regress session-3-start`: identical detections

## 2. Reports and explain

- [ ] Atomic report writes
- [ ] Search settings in each report (profile and hash design, confidence
  rule, library revision); `--save-dir` searches again when any differ
- [ ] Evidence per window in `explain`

## 3. Generated mixes and the window grid

- [ ] Seeded renderer of library-track mixes with exact truth (speeds,
  crossfades, bass-swap EQ, plays of 20-60 s, cuts, a returning track,
  held-out tracks)
- [ ] Window-grid sweep: brief plays slid in 1 s steps, length and source
  position varied
- [ ] Minimum aligned span in seconds (opt-in), compared with 3 windows
- [ ] Boundary error against the truth

## 4. Where real mixes lose evidence

- [ ] Per-play attribution on the development mix: solo against blended,
  frequency band, distance from the nearest rung, speed variation within
  a window
- [ ] Robust conditions: wow (0.55 and 0.75 Hz), broadcast compression,
  beatmatched blends, combined damage

## 5. Second pass at the fitted speed

- [ ] Opt-in second pass: one STFT at the fitted speed over each
  candidate's span, hits against that asset alone
- [ ] Its own null and calibration (items 3 and 4)

## 6. Default-change package (opt-in)

- [ ] Second pass plus the common-hash filter; threshold from calibration
  data alone
- [ ] Every robust condition with the filter on and off, both ladders,
  brief excerpts, combined damage
- [ ] Confident recall, possible recall and false candidates reported
  separately
- [ ] Full protocol, including the padded index
- [ ] Case for adoption in the notes

## 7. Before tracks are added

- [ ] Fixed query panel with more sweep seeds
- [ ] Duplicate clustering from stored peaks, checked against `clusters`
  on controlled cases

## Wrap-up

- [ ] README, roadmap, calibration, status up to date
- [ ] Summary at the top of `docs/notes-for-owner.md`
