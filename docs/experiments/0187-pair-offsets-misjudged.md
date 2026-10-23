# 0187: the judged pairs the measures get wrong; near-identical versions against damaged copies

**Question.** Brief 40, step 4: which judged pairs does every measure get
wrong, and does any measure tell a near-identical version (China Cup's
kind) from a damaged or edited copy (Dilusion's)? Exploratory: 14 pairs
chosen after 0186 from its errors and the comments by ear.

**Every measure wrong** (in-sample and leave-one-out): Fractles ~ the
CITRUS 003 upload (same; the ladder fits 0.9688 for 0-75 s, 0.9657 for
190-339 s, and one rung keeps 68 s near the main offset). Wrong under
6-7 of 9: Mind Overload's RAMM 23 rip ~ the Molten Beats track (offsets
-22, -67, -9 s), We Enter ~ its full instrumental and Sonar's Mark
System Revision ~ the PRO012 rip (one offset each, end to end).

| Pair (verdict) | Coverage | Supported | `near_10` | Gap (s) | Far jumps | Hits/s |
|---|--:|--:|--:|--:|--:|--:|
| Dilusion, a skip (same) | 0.73 | 0.91 | 0.91 | 10 | 0 | 4.3 |
| Stratus - Waves (same) | 0.74 | 0.75 | 0.75 | 21 | 0 | 11.3 |
| Falcon - The Stand (same) | 0.64 | 0.84 | 0.80 | 33 | 3 | 3.0 |
| Dykast - Punishment (same) | 0.44 | 0.86 | 0.80 | 61 | 2 | 14.2 |
| Phoenix, drifting upload (same) | 0.87 | 0.63 | 0.62 | 50 | 1 | 2.4 |
| Coma ~ Spraycan, drift, 4 s shift (same) | 0.58 | 0.76 | 0.55 | 160 | 3 | 10.2 |
| Alien Girl, 13 s shift (same) | 0.73 | 0.81 | 0.55 | 91 | 4 | 5.7 |
| Fractles, drifting upload (same) | 0.26 | 0.48 | 0.12 | 488 | 8 | 4.0 |
| We Enter, instrumental (different) | 0.95 | 0.87 | 0.83 | 51 | 1 | 6.3 |
| China Cup ~ ƆƐI3C, hi-hats (different) | 0.78 | 0.90 | 0.77 | 81 | 3 | 8.3 |
| Mind Overload, Molten Beats (different) | 0.79 | 0.99 | 0.74 | 55 | 2 | 11.9 |
| Sonar revision ~ PRO012 rip (different) | 0.99 | 0.66 | 0.66 | 7 | 0 | 1.3 |
| Sonar PRO012 upload ~ revision (different) | 0.99 | 0.61 | 0.55 | 58 | 2 | 2.1 |
| Stalker ~ album version (different) | 0.74 | 0.73 | 0.15 | 287 | 2 | 7.6 |

**Why.** Same recordings leave the main offset through speed drift
(Phoenix, Fractles, Coma; one rung follows one speed) or edits moving
the rest by more than T (Alien Girl); a version differing in one layer
keeps it. Every search's spans, which follow drift, do worse (AUC 0.94).

**Conclusions.** Within these 14 (8 same, 6 different), chosen from the
measures' errors, the AUCs are 0.40-0.56 (coverage 0.06): near chance by
that choice, so they show where the measures fail, not how often.
Dilusion (0.91) lies above China Cup (0.77), both above 0.552.
