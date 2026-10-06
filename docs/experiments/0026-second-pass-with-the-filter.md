# 0026: the second pass with the common-hash filter

**Question.** Item 6 of session 3: the second pass (0024) plus the
fullest 1% of posting lists set aside, opt-in. What threshold does
calibration data alone give, and does the full protocol pass?

**Variants.** *Drop* empties those lists (`--drop-fullest 0.01`, 0017).
*Skip* leaves them out of the search for candidates only; the second pass
still counts them (`--skip-fullest 0.01`). Both with `--second-pass`, both
ladders: `regress`, `sweep --seed` 2026-2029, `calibrate`, `mixes --count
12`, `grid`. Data: `data/0026-second-pass-with-the-filter.json`.

| Four sweep seeds and the development scans | Default | Second pass | Drop | Skip |
|---|--:|--:|--:|--:|
| Weakest identifying | 403 | 660 | 520 | 658 |
| Strongest false (the Clockwork remix) | 97 | 97 | 91 | 119 |
| False candidates; at 30 hits or more | 70,506; 55 | 70,448; 56 | 4,503; 12 | 4,503; 41 |
| Strongest on audio not in the index | 28 | 24 | 24 | 24 |
| Development: own hits against default; detections | 1; 6,114 | 0.85-1.17; 6,043 | 0.72-0.96; 232 | 0.85-1.17; 232 |
| Generated mixes: confident, possible of 115; strongest false | 107, 8; 46 | 106, 9; 41 | 104, 11; 27 | 106, 9; 42 |
| Grid, confident of 160: 10 s, 15 s, 20 s | 8, 76, 148 | 0, 67, 146 | 0, 66, 146 | 0, 67, 146 |

Every matcher: sweeps 540 of 540 per seed, development 11/11, no wrong
answer anywhere.

**Threshold.** Twice the strongest false and half the weakest identifying
bound it: drop 182-260, second pass 194-330, skip 238-329. The frozen 200
fails skip's lower bound. Skip's 119 is the passage the remix shares with
Clockwork (the owner's verdict): with the fullest lists skipped, the first
pass finds it as one 60 s chain instead of two, and the second pass counts
all of it. `FITTED_RULE` is now 240 hits in 3 windows, possible 60, for
every second-pass variant: 2.02-2.64× the strongest false, 2.17-2.75×
below the weakest identifying; recalibrated, 0 missed, 0 false accepted.
At 240 the mixes and grid are unchanged.

**Conclusions.** (1) Skip keeps the second pass's evidence; drop keeps
72-96% of the default's. (2) Both cut false candidates by 94% and the
development scan's detections by 96%; at 30 hits or more drop leaves 12
and skip 41, because skip counts the common hashes again. (3) The shared
passage sets skip's threshold: hits measure it as they measure the original.
