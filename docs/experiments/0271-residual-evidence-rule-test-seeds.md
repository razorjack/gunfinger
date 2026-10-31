# 0271: the evidence rule, *Align* and detected timing on the test seeds

**Question.** Brief 70, step 3 (pre-registered in `c3217e8`; the bar frozen in `0b45014`): on two seeds nothing was tuned on, does the masked residual with detected timing and *Align* keep its gain without wrong answers, and does the rule that a masked detection counts only with evidence in the plain excerpt (at least `H` = 20 hits at its alignment, the excerpt searched against its file alone) keep the gain and the wrong answers out?
**Method.** Seeds 2040 and 2041 (both keep the Clockwork records indexed; never rendered before), rendered for the run: `gunfinger-eval --work work/s70/t-<scale>-<seed> --panels work/s65/panels --ladder turntable [--other-peaks-dir ...] mixes --kind drops --seed S --count 16 --other-root /mnt/atlas/Music/dnb --tagged work/s39/tags-262.json --residual --residual-timing truth,detected --residual-fix off,align` (four side by side; 262 tracks 22 min, 7.1 GB; NAS scale 46-47 min, 14.8 GB). 448 cases per scale: at NAS scale the reference is a NAS file in 275, a corpus rip in 145, the played file in 28.

Quiet and brief Bs (64), confirmed / confident; wrong confident in M (MH):

| Scale, arm | Plain | Truth, no fix | Detected, no fix | Detected, *Align* | Detected, *Align*, rule `H` 20 |
|---|--:|--:|--:|--:|--:|
| 262 tracks | 41 / 17 | 63 / 53; 0 (0) | 62 / 53; 0 (0) | 64 / 53; 0 (0) | 52 / 47; 0 (0) |
| NAS scale | 41 / 15 | 55 / 39; 2 (1) | 55 / 39; 2 (1) | 61 / 46; 0 (0) | 51 / 42; 0 (0) |

- A confident after masking, NAS scale: corpus rips 26 of 145 without a fix, 0 with *Align*; NAS files 98-99 of 275, 32-33 with *Align*; the played file 0 of 28. At 262 tracks: corpus rips 26 of 145, 0 with *Align*.
- The Cool Rock pair (by ear a different title): the plain search names the Mobb Deep Vs Urban Takover - Cool Rock upload confidently twice in seed 2040 (mix-07, mix-15; 385 and 637 hits), as calibration's strongest false. Without a fix M keeps both (323 and 546 hits; MH the second): A's residue in mix-07, a passage the plain excerpt already holds in mix-15. With *Align* both leave M (mix-07 keeps another detection of the upload at 167 s as possible, 135 hits, plain evidence 75, possible also without a fix). The rule cannot remove either without *Align*: their plain evidence is 385 and 638 hits.
- Other related versions: Kemal - Animation (Silent Extent Remix) confident in plain once (seed 2040, mix-14), not in M; Paragliders - Share Of Bitterness (Konflict Remix) possible in M once (117 hits, plain evidence 29). Clockwork's files: no wrong answer in M (detected, *Align*) at either scale.
- Wrong possible, NAS scale: plain 349, M 6 (rule 2); MH 1 (rule 0).
- Of the 20 quiet and brief Bs that M (detected, *Align*, NAS) confirms and plain does not, the rule keeps 10; at 262 tracks 11 of 23.

**Against the pre-registration.** The rule was run once as frozen. It adds no protection measured here: under *Align* M already gives no wrong confident answer, and the two it would have had to stop (without a fix) have strong plain evidence. It costs 10-12 confirmed Bs (61 to 51, 64 to 52) and 4-6 confident ones.

**What it shows.** On fresh seeds, detected timing with *Align* keeps the oracle's gain (NAS scale: confirmed 61 against plain 41, confident 46 against 15) with no wrong confident answer; *Align* also removes the Cool Rock errors that the unaligned mask keeps. Evidence in the original mixture, as built, keeps the related versions it was meant for (they are present in the mixture) and drops half of the true partners found only in the residual. **Does not show** real mixes, broadcast processing, or relations other than these few pairs. Data `data/0271-residual-evidence-rule-test-seeds.json`.
