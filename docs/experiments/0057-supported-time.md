# 0057: coverage against supported time for joins and borderline pairs

**Question.** Item 5 of session 7 (roadmap, "Clustering precision" and
"A minimum span instead of 3 windows"): how much of the shorter file
holds aligned hits, compared with the coverage the clusters use?

**Command.** `gunfinger-eval --other-peaks-dir <NAS store> pair
work/s7/supported-pairs.txt` (commit e9562ea; 1,273 s CPU, one thread):
the 4 sparse joins `clusters` lists for the owner's ear, the 28
borderline pairs (0049) and 8 joins drawn at random (seed 2026) as
controls. At the fitted speed `pair` lists each alignment at its own
offset; supported time is the share of the shorter file's seconds
holding a hit in an alignment's densest run per 10 s window (as the
second pass forms lines), summed over the alignments. Coverage is the
clusters' (one alignment's first to last hit). `supported_time.py`; data
`data/0057-supported-time.json`.

| Pairs | Clusters' coverage | Alignments at the fitted speed | Supported time |
|---|--:|--:|--:|
| 8 control joins | 0.984-0.998 | 1 | 0.980-1.000 |
| Phoenix: i-witness upload ~ TECH012 rip, corpus rip ~ upload (joined) | 0.992, 0.840 | 4, 5 | 0.715, 0.628 |
| Sonar ~ "Mark System Revision", dfect upload ~ revision (joined) | 0.891, 0.911 | 4, 6 | 0.391, 0.429 |
| China Cup (4 rips) ~ Prototype upload | 0.651-0.676 | 2 | 0.960-0.976 |
| Future Cut - The Specialist (3 files) ~ INFRA 012 upload | 0.765-0.766 | 2 | 0.970-0.991 |
| The Nine (6 files) ~ Evol Intent VIP | 0.484-0.503 | 4-5 | 0.856-0.955 |
| Alien Girl; Stratus - Waves (2); Falcon - The Stand | 0.641-0.762 | 2-6 | 0.747-0.836 |
| Coma ~ Spraycan | 0.585 | 4 | 0.758 (0.073 twice) |
| Synthesis VIP, i-witness upload ~ "Synthesis (Remix)" | 0.594, 0.528 | 12, 8 | 1.360, 1.027 (0.225, 0.208 twice) |
| Fractles; Pyro; Dozer ~ bootleg | 0.444-0.570 | 5-9 | 0.338-0.510 |
| Global Report ~ Luminous (3); Aphrodite ~ Critikal; Night Gasp ~ Dark Soldier | 0.417-0.572 | 0-6 | 0-0.304 |

"Twice": the share of the shorter file two alignments both cover
(repeated sections); the sum counts it twice.

**Conclusions.** Coverage and supported time order the pairs
differently. The Sonar joins rest on one ladder line with 0.7 hits per
second; at the fitted speed only 0.39-0.43 of the file holds hits. Pairs
the clusters keep apart are held almost whole at two offsets (China
Cup, The Specialist: an edit or a moved section) or several (The Nine's
VIP). Neither measure tells same from different recordings; both go to
the owner with the listening list. Nothing changed in the clusters.
