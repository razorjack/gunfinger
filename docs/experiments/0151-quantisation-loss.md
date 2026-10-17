# 0151: what exact hash quantisation loses after fitting

**Question.** 0149 (pre-registered): on the verifier's alignment, how many
pairs match only one quantisation step away (N) beside the exact ones (E)?
**Method.** As 0149: code `aecdf93` (= master `2f8f4c9`), R1-R5 at
08:03-09:23, exit 0; R1, R2 detections equal session 28's (apart from the
verifier's field). `quantisation.py`; `data/0151-quantisation-loss.json`.

| Set (fitted), medians | n | E | N | N/E | (E+N)/E | V/P | E/hits |
|---|--:|--:|--:|--:|--:|--:|--:|
| T(C) clean, own record | 522 | 1,022 | 303 | 0.29 | 1.29 | 0.84 | 1.00 |
| T(c) combined damage | 515 | 95 | 149 | 1.48 | 2.48 | 0.37 | 0.98 |
| T(a) other rips only | 178 | 458 | 336 | 0.69 | 1.70 | 0.71 | 1.00 |
| F unrelated (R1, R2) | 2,832 | 8 | 6 | 0.83 | 1.83 | 0.06 | 0.82 |
| R(r) related wrong (R1, R2) | 383 | 49 | 87 | 1.54 | 2.54 | 0.24 | 0.93 |
| P played, shared panels | 1,066 | 954 | 427 | 0.52 | 1.52 | 0.73 | 1.00 |
| R related, shared panels | 5,184 | 119 | 135 | 1.10 | 2.10 | 0.38 | 0.98 |

Chance (four shifts): E 0-0.25, N 0-2. 0055 (262 tracks): N/E 1.39 for
the related recordings, 0.19 for the played. M2: under (c) E/E(C) 0.096,
(E+N)/E(C) 0.243; 16% of the exact pairs damage removes reappear one step
away; N halves (0.49). (a): 0.475 and 0.851.
M3, under 240 reaching 240 with E+N: T(c) 260 of 510 (255 in 3 windows);
T(a) 3 of 3; F none (max 224, Congo Natty's "Peace"; else 110); R(r) 37
of 383 (Fire's Splash, VIP and Rogue Unit remixes on Fire's clean queries,
16 and the same 16 under (a), 5 under (c); Eddy Woo's Tsunami on its
held-out remix); R 1,817 of 4,266; 0055's related 170 of 569.
Per search (exploratory), played left out: a related file confident in
112 of 419 searches today, 238 with E+N >= 240 in 3 windows (panel 1: 106
to 136; panel 2: 6 to 102); the played file, everything: 312 to 315.
M4, pooled AUC of N/E (chance-corrected): T(c)~F 0.84 (0.85), T(c)~R 0.78
(0.77), P~R 0.50 (0.47); of V/P 0.95, 0.40, 0.35. Within (c) alone
(exploratory): T(c)~F 0.55, T(c)~R(r) 0.35: damage raises N/E for all.

**Against 0149.** Notable: R gains at least as much as P (E 10-119);
51% of T(c) reach 240, and R(r) and R do too; T(a) 0.85 >= 0.75; pooled
AUCs >= 0.75, set by the condition, not the identity. Not met: recall
without risk (R, and F from E 30, gain over half of T(c)'s). E/hits in
range. Wrong expectations: T(c) N/E 1.48 (said 0.2-0.6); F 0.83, not 26.

**Conclusion.** About a sixth of what damage removes sits one step away;
counting it would name half of the damaged misses, but related versions
gain as much as true ones and, on clean shared material, no less than
the played recording at equal E. No rule proposed.
