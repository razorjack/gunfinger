# 0001: Own fingerprinter; speed-ladder pair hashes (candidate A)

## Status

Accepted.

## Context

Gunfinger must find library tracks inside vinyl DJ mixes, where the
turntable resamples the record: pitch and tempo move together, within about
±8%, and the DJ adjusts the pitch while beatmatching. Unknown audio must stay
unknown. The brief fixes that Gunfinger owns its fingerprinter (no
third-party fingerprinting crate, no copied code) and names two candidates:
A, exact pair hashes searched on a ladder of assumed speeds; B, hashes
invariant to resampling (Six & Leman 2014; Six 2021). A was to be built
first, and B only if A failed.

## Decision

Candidate A, implemented from Wang (2003):

- **Peaks** (ADR 0003): 2-D local maxima of the log-power STFT (8 kHz, Hann
  1024, hop 128), refined between frames and bins by parabolic
  interpolation.
- **Hashes**: each peak is paired with the next 2 peaks (fan-out 2) within 63
  frames and ±63 bins. Hash = anchor frequency (8 bits) | bin difference (7) |
  frame difference (6) = 21 bits. The anchor is quantised in whole bins below
  bin 50 and in 2% relative steps above, so that a residual speed error
  breaks high and low anchors equally (experiment 0001).
- **Speed ladder**: 41 rungs, 0.92 to 1.08 in steps of 0.4%. A rung is an
  STFT whose window and hop are divided by the assumed speed, so the query is
  analysed on the reference's time-frequency grid. Peaks picked on the
  query's own grid and then transformed lost about half the surviving hashes;
  resampling the audio was no better than the scaled STFT and costs a decode
  per rung (experiment 0001).
- **Lines**: per rung and per 10 s window of query time, hits of one asset
  whose offsets (reference frame − speed × query frame) agree within 2 frames
  form a line (Wang's offset histogram, §2.3, with the rung's speed as
  slope). Lines of neighbouring rungs that predict the same reference
  time merge.
- **Chains**: lines of one asset in successive windows (gaps up to 2 windows)
  that predict the same reference time within 4 frames + 0.4% of the elapsed
  time are chained by dynamic programming. A chain is a detection; its slope,
  fitted through the lines, gives the speed (sweep error ≤ 0.016%). Slow
  pitch adjustments change the rung from window to window without breaking the
  chain. Weaker detections overlapping a stronger one of the same asset are
  dropped.
- **Confidence** (`gunfinger-core/src/confidence.rs`): at least 200 hits and
  3 windows on one chain. Absolute, sustained, aligned evidence; calibrated
  against the measured null (experiments 0003, 0004), as Wang (2003,
  §2.3.1) sets a threshold from the strongest wrong track's score.
  Sources and section numbers: `docs/references.md`.

## Consequences

- Development bar met on the first build: sweep recall 100% at all speeds
  with zero wrong answers, development set 11/11 with zero wrong; candidate B
  was not built.
- Query cost is one decode, 41 STFTs and the lookups: 22 to 26 s for the
  56-minute development mix on 10 cores, of which the ladder takes about
  15.8 s and lookups, lines and chains about 2.6 s at 262 tracks (experiment
  0005). Lookup cost grows with the library; it has not been measured beyond
  262 tracks.
- Key-locked (time-stretched without pitch change) sets are out of reach of
  this design; see the roadmap.
- A remix or VIP that shares a long, unchanged section with the played
  original can produce a strong false line; the strongest measured was 95
  hits over 2 windows, below the rule.
