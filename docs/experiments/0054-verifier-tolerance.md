# 0054: the peak verifier's tolerance, chosen on the development scan and sweep 2026

**Question.** Item 1 of session 7: a peak verifier after Qfp's
(Sonnleitner & Widmer 2016, §VI-C) as a harness diagnostic. Which of
three tolerances (half-widths in bins × frames: 1×1, 3×2, and 6×2, about
Qfp's box) separates identifying from false detections best?

**Design** (`gunfinger-eval --verify`, `verifier.rs`, commit fb723ed).
Per reported detection: its asset's reference peaks in the aligned span,
looked for in the query analysed at the detection's speed and playback,
on the line through the detection's ends, each 10 s window moved by the
offset of its densest run of the asset's hash hits (the verifier
searches no offset). Reference peaks found are counted, never query
peaks matched. Chance: the reference shifted −6.2, −5.0, +5.0 and +6.2 s
(no whole number of bars at 160-180 BPM). A first version that searched
each window's offset over ±48 frames reached the next bar of the shifted
reference (chance shares of 15-40%). 1,070 detections; groups by
`verifier_groups.py` (related: same name, version or artist,
`labels.py`). Data: `data/0054-verifier-tolerance.json`.

| Tolerance | Share: identifying; related; unrelated false (chance) | Support (chance) | AUC at equal hits (7-99 hits; 5 identifying, 199 false): hits; excess share; excess support |
|---|---|---|---|
| 1×1 | 0.84; 0.09; 0.04 (0.005-0.010) | 1.00, 0.92, 0.80 (0.16-0.29) | 0.52; 0.72; **0.82** |
| 3×2 | 0.85; 0.21; 0.13 (0.03-0.05) | 1.00 all (0.60-0.72) | 0.52; 0.78; 0.59 |
| 6×2 | 0.87; 0.33; 0.24 (0.06-0.07) | 1.00 all (0.72-0.83) | 0.52; 0.68; 0.54 |

Excess is the measure minus the mean over the four shifts. Wider boxes
add chance faster than found peaks; support saturates from 3×2 on. Only
5 identifying detections (7-18 hits, fragments at track edges) fall in
the false candidates' range. The Clockwork remix's shared passage (74-99
hits) verifies at 0.23-0.44, as high as an identifying rip of SKC -
Recharger (116-162 hits, 0.38-0.42): a share alone does not tell shared
material from a weak rip.

**Decision.** Frozen at 1×1 (±1 bin, ±1 frame): the best separation at
equal hits (support excess, AUC 0.82) and the lowest chance level. It is
reported unchanged on seeds 2027-2029, the leave-outs, `robust --only
combined`, `mixes --count 12` and `grid` (0056).
