# 0124: a version verifier along the fitted line, calibrated on the panel

**Question.** Session 24, steps 3-5 (research note 2026-10-16): does a
three-state band-dynamics test with a constrained gain/EQ fit contradict
a related version's confident play where hashes are silent, and at what
threshold, chosen on the panel's development half only?

**Pre-registration** (`docs/brief-24.md`, step 3, with two amendments
from the synthetic tests, all before any run). `gunfinger-eval versions`:
the reference rendered at each segment's fitted speed; 20 log bands in
three EQ groups; 2 s patches every 0.256 s, one lag within +-48 ms;
dynamics `r`; group gains from anchor bands (`r >= 0.6`) of one fold,
tested on the other; bands supported, contradicted (an un-EQ-shaped 10
dB deficit, or `r < 0.2` unmasked), killed, masked; a patch contradicted
at `c >= 2, c >= s/2`. S1 (contradicted share) decides; T is the
`ceil(n/2)`-th highest S1 of the negatives (wrong confident plays, played
left out); the positives are the positive control's right plays.

**Run.** 387 plays of 0122's report under 0120's clusters (312 negatives
from 106 queries, 75 positives; `version_plays.py calibration`),
`versions` in 67 s, 0.6 GB, no errors; `threshold`; `data/0124-*`.

| S1 | Plays | Median | 95th pct | Max | Rejected at T |
|---|--:|--:|--:|--:|--:|
| Negatives | 312 | 0.121 | 0.708 | 0.785 | 157 |
| Positives | 75 | 0.000 | 0.012 | 0.053 | 0 |

- **T = 0.1210**: half the negatives, no positive. By family: Dozer 13 of
  16, Fire 144 of 248, Pyro 0 of 32, Sonar 0 of 16 (median S1 0.007 and
  0: their passages play the same content).
- Negatives' low bands (39-246 Hz): 22.8% contradicted, 9.1% killed,
  17.2% masked; mid and high 82-89% supported (the shared drums).
  Positives: 96-99.5% supported in every group. The rejections come from
  basslines; a patch is contradicted only when the low bands' count
  reaches half its supported bands.
- S2 undefined for 230 negatives and 54 positives (every patch held by a
  sibling); S3 median 0 s for both. Neither decides.

**Conclusion.** On clean renders the verifier separates: T = 0.121
rejects half of the wrong plays and no positive control; Pyro's and
Sonar's are never contradicted. 0125 tests T on real true plays.
