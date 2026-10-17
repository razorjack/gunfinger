# 0151: what exact hash quantisation loses after fitting

**Method.** 0149 as registered: code `aecdf93` (= master `2f8f4c9`), R1-R5
08:03-09:23, exit 0; R1, R2 detections equal session 28's (beside the new
field). `quantisation.py`; `data/0151-quantisation-loss.json`.

| Set (fitted), medians | n | E | N | N/E | (E+N)/E | V/P | E/hits |
|---|--:|--:|--:|--:|--:|--:|--:|
| T(C) clean, own record | 522 | 1,022 | 303 | 0.29 | 1.29 | 0.84 | 1.00 |
| T(c) combined damage | 515 | 95 | 149 | 1.48 | 2.48 | 0.37 | 0.98 |
| T(a) other rips only | 178 | 458 | 336 | 0.69 | 1.70 | 0.71 | 1.00 |
| F unrelated (R1, R2)* | 2,832 | 8 | 6 | 0.83 | 1.83 | 0.06 | 0.82 |
| R(r) related wrong (R1, R2)* | 383 | 49 | 87 | 1.54 | 2.54 | 0.24 | 0.93 |
| P played, shared panels | 1,066 | 954 | 427 | 0.52 | 1.52 | 0.73 | 1.00 |
| R related, shared, not confident | 4,274 | 107 | 112 | 1.11 | 2.11 | 0.39 | 0.98 |

*R1's clean and R2's queries are the same excerpts: most entries twice.
Confident R (910): N/E 1.08. Chance medians: E 0-2.5, N 0-3.5, mostly
under 1. 0055 (262 tracks): N/E 1.39 related, 0.19 played. M2 (derived
shares beside): (c) E/E(C) 0.096, (E+N)/E(C) 0.243, N/(E(C)-E) 0.16,
N/N(C) 0.49; (a) 0.475, 0.851, 0.74, 1.10.
M3, under 240 reaching 240 with E+N (in 3 windows): T(c) 260 (255) of 510;
T(a) 3 of 3; F none (max 224, Congo Natty's "Peace"; next 110); R(r) 37 of
383, 21 distinct (Fire's Splash, VIP, Rogue Unit remixes for Fire, Eddy
Woo's Tsunami for its remix, 16 clean, again in (a); 5 Splash in (c)); R
1,817 (1,690) of 4,266; 0055's related 170 (83) of 569.
Per search (exploratory), played left out, a related file confident:
112 of 419 today, 238 with E+N >= 240 in 3 windows (panels 106 to 136, 6
to 102); the played file, everything indexed: 312 to 315.
M4, pooled AUC of N/E (chance-corrected): T(c)~F 0.84 (0.85), T(c)~R 0.78
(0.77), P~R 0.50 (0.47); of V/P 0.95, 0.40, 0.35; T(a)~F undefined (no
shared E band). Within (c), exploratory, small n: T(c)~F 0.55, ~R(r) 0.35.
**Against 0149.** Notable: R gains as much as P at E 10-119 (P n 8, 14,
9; at 120-239, n 54, P more); 51% of T(c) reach 240, R(r), R too; T(a)
0.85; pooled AUCs >= 0.75 (from the condition, the (c) counts suggest).
Not met: recall without risk. Expected wrongly: T(c) N/E 0.2-0.6, few at
240, F near 26, AUC T~F < 0.5, R like T at equal E. E/hits in range.
**Conclusion.** Near pairs equal a sixth of what damage removes (other
rips: three quarters) and would name half the damaged misses; related
versions gain about as much as the played recording at equal E.
