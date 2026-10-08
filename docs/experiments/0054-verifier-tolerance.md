# 0054: the peak verifier's tolerance, chosen on the development scan and sweep 2026

**Question.** Item 1 of session 7: a peak verifier after Qfp's
(Sonnleitner & Widmer 2016, §VI-C) as a harness diagnostic. Which of
three tolerances (half-widths in bins × frames: 1×1, 3×2, 6×2; Qfp's box
is about 6×2 here) separates identifying from false detections best, on
the development scan and sweep 2026 only?

**Design** (`gunfinger-eval --verify`, `verifier.rs`, commit fb723ed).
For each reported detection: its asset's reference peaks in the aligned
span, looked for in the query analysed at the detection's speed and
playback, on the line through the detection's ends, each 10 s window
moved by the offset of its densest run of the asset's hash hits (as the
second pass follows a drifting speed; the verifier searches no offset).
Counted: reference peaks found, never query peaks matched. Chance: the
same with the reference shifted −6.2, −5.0, +5.0 and +6.2 s (no whole
number of bars at 160-180 BPM). A first version that searched each
window's offset over ±48 frames gave chance shares of 15-40%: the search
reached the next bar of the shifted reference. Default matcher; 1,070
detections. Groups by `verifier_groups.py`: identifying; false from
related recordings (same name, version or artist, `labels.py`); false
from unrelated ones. Data: `data/0054-verifier-tolerance.json`.

| Tolerance | Share: identifying; related; unrelated (chance) | Support (chance) | AUC at equal hits (7-99 hits; 5 identifying, 199 false): hits; excess share; excess support |
|---|---|---|---|
| 1×1 | 0.84; 0.09; 0.04 (0.005-0.010) | 1.00, 0.92, 0.80 (0.16-0.29) | 0.52; 0.72; **0.82** |
| 3×2 | 0.85; 0.21; 0.13 (0.03-0.05) | 1.00 all (0.60-0.72) | 0.52; 0.78; 0.59 |
| 6×2 | 0.87; 0.33; 0.24 (0.06-0.07) | 1.00 all (0.72-0.83) | 0.52; 0.68; 0.54 |

Excess = measured minus the mean of the four shifts. Wider boxes add
chance faster than found peaks, and support saturates at 1.00 for every
group from 3×2 on.

What the measure cannot do: the Clockwork remix's shared passage
(74-99 hits) verifies at share 0.23-0.44 at 1×1, as high as the
identifying SKC - Recharger rip (116-162 hits, 0.38-0.42): in that
passage the two records are the same audio. Five identifying detections
fall in the range of false ones (7-18 hits, fragments at track edges),
too few to judge separation at equal hits.

**Decision.** Frozen at 1×1 (±1 bin, ±1 frame): the highest separation
at equal hits (support excess, AUC 0.82), and the lowest chance level.
It is reported without change on seeds 2027-2029, the leave-outs,
`robust --only combined`, `mixes --count 12` and `grid` (0055).
