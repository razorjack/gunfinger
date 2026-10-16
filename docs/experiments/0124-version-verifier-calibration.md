# 0124: a version verifier along the fitted line, calibrated on the panel

**Question.** Session 24, steps 3-5 (`docs/research/2026-10-16-version-verification.md`):
does a three-state band-dynamics test with a constrained gain/EQ fit
contradict a related version's confident play where hashes are silent,
and at what threshold, chosen on the panel's development half only?

**Pre-registration** (`docs/brief-24.md`, step 3, committed before any
run). `gunfinger-eval versions`: per segment, the reference rendered at
the fitted speed onto the mix clock; 20 log bands (39 Hz-3.9 kHz) in
three EQ groups; 2 s patches every 0.256 s, one lag within +-48 ms;
band dynamics `r` (48 ms differences); group gains fitted on anchor
bands (`r >= 0.6`) of one fold of 0.256 s blocks and tested on the
other; band states killed, supported (`r >= 0.5`), contradicted (10 dB
deficit, or `r < 0.2` unmasked), masked (+6 dB); patch contradicted at
`c >= 2` and `c >= s/2`, supported at `s >= 3`, unobservable without an
anchor. Summaries: S1 contradicted share (decides), S2 the same in
distinctive regions, S3 distinctive support. T: the `ceil(n/2)`-th
highest S1 of the calibration negatives (wrong confident plays, played
left out, 0122); positives (the positive control's right plays) kept
at T reported.
