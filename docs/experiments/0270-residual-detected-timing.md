# 0270: the masked residual with A's time map from its detection

**Question.** Brief 70, step 2 (pre-registered in `c3217e8`): with A's time map from the whole-mix search's confident detection (its file, line and speed; no fades, level or bass cuts) instead of the generator's, how much of the oracle's gain is left? The reference follows 0257's rule from the detected file, so where the store holds another rip it is the same file as under the true map.
**Method.** The runs of 0269 (both timings and all fixes in one run per seed and scale; same cases, same plain and denser arms). `scripts/analysis/residual_residue.py`. Primary row: detected timing with the fix frozen in 0269, *Align*.

Quiet and brief Bs (64), confirmed (possible or confident) / confident; A confident after masking (cases):

| Scale | Plain | Truth, no fix | Detected, no fix | Truth, *Align* | Detected, *Align* |
|---|--:|--:|--:|--:|--:|
| 262 tracks (309 played, 132 corpus rips) | 41 / 8 | 62 / 47; A 20 | 62 / 47; A 26 | 63 / 45; A 0 | 63 / 45; A 0 |
| NAS scale (37 played, 132 corpus, 271 NAS) | 40 / 8 | 50 / 32; A 115 | 49 / 32; A 116 | 61 / 38; A 31 | 61 / 38; A 31 |

- Wrong confident answers in M and MH: 0 under both timings and every fix, at both scales; wrong possible in M 4-8 (plain 181 and 292). The plain arm's 2 at NAS scale are 0257's Kemal - Animation remix upload.
- The lag the reference is off by (median over the excerpt's blocks): with the played file, truth 0 ms and detected 0-2 ms (262 tracks: 5 of 309 cases over 20 ms under detected timing, which leave A confident without a fix); with another rip the two timings give the same distribution (NAS files: 67 and 64 of 271 over 20 ms), since the rip's line dominates the error.
- Detected, *Align*, NAS scale, by reference: played file 11 of 11 quiet and brief Bs confirmed, other rips 50 of 53 (plain 32 of 53).
- Of the 21 quiet and brief Bs that M (detected, *Align*, NAS) confirms and plain does not, 8 have a plain detection of two windows or more.

**Against the pre-registration.** No threshold decides here. Detected timing keeps the oracle's gain: with *Align* it gives the same confirmed and confident counts as the true time map at both scales (63 / 45 and 61 / 38), and without a fix within one B.

**What it shows.** The whole-mix detection's line is accurate enough for the mask once the reference is aligned on the mixture; the 5 of 309 lines off by 26 to 448 ms at 262 tracks (two at the measure's limit of 28 frames, 0127's kind of error) leave A behind without *Align* and not with it. In these generated mixes the error that matters is the other rip's, not the detection's. **Does not show** real mixes, where a detection can span a different version or a pitch ride (no pitch rides here), or A's fades and EQ beyond what the band gains follow. Data `data/0269-residual-residue-and-detected-timing.json` (the same runs).
