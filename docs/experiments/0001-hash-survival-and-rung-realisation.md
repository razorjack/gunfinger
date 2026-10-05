# 0001: hash survival against speed error, and how to realise a rung

**Hypothesis.** Pair hashes with absolute anchor frequency survive a residual
speed error of a few tenths of a percent; peak-coordinate transforms are as
good as resampling and much cheaper.

**Change.** `gunfinger-eval survival`: 30 s lossless excerpts of 6 library
tracks rendered by FFmpeg at true speeds 0.92 to 1.08, searched at rungs
`speed × (1 + residual)`. Survival = query hashes found in the reference within
±2 frames of the true alignment. Three realisations: resample audio, STFT with
window and hop scaled by the rung (S), peak coordinates transformed (T).

**Command.** `target/release/gunfinger-eval survival <6 assets>`; logs in
`work/logs/survival-*.log`.

**Results** (mean of 6 tracks, %; anchors hybrid-log, 2% steps above bin 50):

| residual | resample | T @0.92 | T @1.05 | S @0.92 | S @1.05 |
|---------:|---------:|--------:|--------:|--------:|--------:|
| 0.0%     | 72.9     | 39.3    | 46.3    | 78.8    | 67.8    |
| ±0.1%    | 51.8     | 37.8    | 44.2    | 52.8    | 47.5    |
| ±0.2%    | 45.7     | 35.4    | 41.3    | 44.7    | 44.7    |
| ±0.3%    | 42.4     | 32.4    | 37.9    | 43.6    | 40.7    |
| ±0.5%    | 34.4     | 26.2    | 31.0    | 34.8    | 33.7    |

1. Integer frame times made survival depend on the grid phase: 99% when the
   excerpt started on a frame boundary, about 30% otherwise. Refining peak
   time by parabolic interpolation (as for frequency) fixed most of it.
2. Survival still depends on the grid phase (speed 1.0, start offset 0 to 7/8
   frame): 99, 85, 76, 71, 70, 72, 77, 85%. A residual speed error also
   shifts the frequency grid, hence the drop from 0 to ±0.1%.
3. Anchor quantisation: linear 1-bin anchors fall to 26% at ±0.2% and 3% at
   ±1%. Hybrid log anchors (linear below the knee, relative steps above):
   1% steps 42% at ±0.2%, 2% steps 46%, 3% steps 48%; linear 2-bin 38%.
4. T is worse than S everywhere (39 vs 79% at 0.92): peaks picked on the
   query's own grid differ from those picked on the reference's grid.

**Conclusion.** Realise a rung with an STFT whose window and hop are scaled by
the rung (one decode, one STFT per rung, same quality as resampling). Anchor
frequency in hybrid-log steps of 2% above bin 50 (167 levels, 8 bits). Ladder
step 0.4% (residual at most ±0.2%, survival ≥ 45% of hashes on clean audio),
41 rungs over 0.92 to 1.08. The transform realisation is dropped.
