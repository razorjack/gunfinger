# 0258: the masked residual with the true time map at NAS scale, test seeds

**Question.** Brief 65: does 0257's picture (precision kept, recall lost with another rip as A's reference) hold on the test seeds?
**Method.** As 0257, on 0256's renders of seeds 2036 and 2039 (copied to `work/s65/wn`; all 32 plans equal), `work/bin/s65-run3` (11:05 and 11:25, 14.4 GB peak). 445 cases: A's reference a corpus rip in 128, a NAS file in 285, the played file in 32.

B inside A, both seeds (candidate / confirmed / confident):

| B inside A | n | P | D | M | M, A from another rip |
|---|--:|--:|--:|--:|--:|
| Quiet (-9 dB) | 32 | 21 / 14 / 3 | 23 / 18 / 2 | 32 / 27 / 17 | 29: 29 / 24 / 15 (P 20 / 13 / 2) |
| Brief (20-40 s) | 32 | 32 / 21 / 9 | 32 / 24 / 6 | 32 / 28 / 17 | 29: 29 / 25 / 14 (P 29 / 18 / 9) |
| Unrelated drop, inside | 32 | 32 / 32 / 25 | 32 / 31 / 24 | 32 / 32 / 29 | 30: 30 / 30 / 27 |
| Pair drop, inside | 17 | 17 / 17 / 9 | 17 / 17 / 9 | 17 / 17 / 17 | 17: 17 / 17 / 17 |

- Wrong confident in the whole-mix search and in P and D: 2, both seed 2036 mix-03: the Mobb Deep Vs Urban Takover - Cool Rock upload over Aphrodite - Cool Rock Stuff (389 hits; the Cool Rock pair, by ear a different title, calibration's strongest false) and Paragliders - Share Of Bitterness over Konflict - Pendulum returning (414 hits; nothing in the repository relates the two).
- M: 2 wrong confident, both the Cool Rock upload. In mix-03 it survives the mask of Cool Rock Stuff (364 hits; the reference, a NAS upload of Cool Rock Stuff, left A confident). In mix-15 the residual made it: Cool Rock Stuff plays as the second record of an unrelated drop, and with its partner (a Stakka & Skynet album track) masked the upload reaches 265 hits over 60 s, where P had it below confident. MH: 1 (mix-03's). Wrong possible: P 342, D 295, M 10.
- What the mask leaves of A (confident): played file 0 of 32; corpus rip 24 of 128; NAS file 95 of 285.
- Of the 20 quiet and brief Bs that M confirms and P does not, 14 have a plain detection.

**Against 0257.** Recall reproduces: quiet and brief confirmed 55 of 64 (P 35, D 42), confident 34 (P 12); with another rip 49 of 58 confirmed (P 31), 29 confident (P 11). Precision does not hold without exception: the residual made one confident answer that is not playing, a known related version of the record under A, by removing the other record over it. MH's one is not new (present in P).

**What it shows.** Taking A out strengthens every record under it, a relative of B as much as B: the same mechanism that recovers quiet partners makes a related version confident once its competitor is masked. A rule built on the residual needs the shared-material controls of a real relation, not only drawn pairs, and confirmation from the original mixture. Data `data/0258-residual-oracle-nas-test-seeds.json`.
