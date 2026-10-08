# 0053: four sweep seeds at NAS scale under the new default

**Question.** Item 10 of session 6 (only if time remained): sweeps 2028
and 2029 at NAS scale under the fitted matcher and the clusters of 0049,
then `calibrate` over the four seeds with 0051's scans, against 0040
(skip at 240 without the link rules, session 5's clusters).

**Command.** `work/scripts/s6-item10.sh` (`--other-peaks-dir`, store
revision `d428ee9585936326`), one step at a time; `nas_calibrate.py`
splits the false candidates. Data: `data/0053-nas-four-seeds.json`.

| Seed | Recalled | Wrong answers (0040) | Wall; CPU; peak |
|---|--:|--:|--:|
| 2026 (0051) | 540/540 | 0 (91) | 1,296 s; 12,077 s |
| 2027 (0051) | 540/540 | 24 (174) | 1,306 s; 12,162 s |
| 2028 | 540/540 | 8 (126) | 1,272 s; 11,775 s; 5.5 GB |
| 2029 | 540/540 | 8 (114) | 1,316 s; 12,133 s; 5.4 GB |

| `calibrate`, four seeds and the scans | 0040 | Now |
|---|--:|--:|
| Identifying; below the rule | 2,179; 0 | 2,179; 0 |
| False accepted by the rule | 523 | 40 |
| Weakest identifying; strongest false; margin | 661; 5,845; 0.11× | 661; 511; 1.29× |
| Without the pairs for the owner | 220; 3.0× | 197; 3.36× |
| Unrelated chance | 67-70 | 63-70 |

The 40 confident false answers are three pairs, each at 8 speeds of one
excerpt: China Cup ~ the alien5ive Prototype upload (24, seeds 2027 and
2028; 65-68% coverage), Coma ~ Spraycan (8; 58%) and Synthesis VIP ~
"Synthesis (Remix)" on the mixed CD Dangerous Drums 2 (8; 59%). Without
them the strongest false candidate is The Nine against its Evol Intent
VIP (197 hits, possible). Unrelated chance leaves out a mixed-CD blend
(112 hits) and Star Trails' Synergy remix during Star Trails (79).

**Conclusions.** Recall holds over 2,160 queries. The margin rests on
the owner's verdicts on three pairs: if each is one recording, it is
3.36×; if not, different recordings share up to 511 hits, past the rule
of 240. Unrelated chance is a tenth of the weakest identifying evidence.
