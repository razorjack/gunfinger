# 0049: clusters on the ±8% ladder join the fast uploads

**Question.** Item 5b-d: `clusters` on the matcher's speed range (0048).
Do the 17 corpus clusters reappear, what is the gap, what is explained?

**Design.** Turntable ladder 0.92-1.08, the fullest 1% of lists skipped
for candidates; candidates of 30 hits or 20% of the shorter file measured
again alone at their fitted speed; the alignment covering more counts,
in the shorter file's own seconds; 80% rule, 40% borderline; verdict
file empty. A first NAS run used the refit alone and lost two rips of
session 5 (Phoenix 0.87 → 0.25, Fallout 0.97 → 0.63): one rung cannot
follow a drifting rip. Now the refit joins 10 pairs the ladder misses
(mixed-CD tracks) and covers more in 432 of 1,361. Data in `data/`.

| | 262 tracks | → now | NAS (0037) | → now |
|---|--:|--:|--:|--:|
| Corpus clusters reappear | 17 | 17 | 17 | 17 |
| Files in clusters; further rips | | | 686; 170 | 732; 216 |
| Weakest same recording | 0.984 | 0.984 | 0.825 | 0.840 (Phoenix upload) |
| Strongest different | 0.385 | 0.210 | 0.737 | 0.766 |
| Borderline pairs (40-80%) | 0 | 0 | 76 | 28 |
| Wall; CPU; peak | | 535 s; 4,951 s | 4,328 s; 38,056 s | 5,106 s; 46,736 s; 4.5 GB |

The 46 new NAS members are uploads (pairs at 0.928-1.078, 220 beyond
±2%); none dropped; two spell the title differently (the same tracks).
For the owner: "DJ Trace - Sonar (Mark System Revision)" joins Sonar at
0.91 with 246 hits. The strongest different pair is the INFRA011 rip
tagged "Sex Drive" (holding The Specialist, 0037) ~ the INFRA012 upload.

Session 5's confident false detections, re-scored with these clusters
(`scripts/analysis/rescore_clusters.py`): development scan 6 of 6
explained under both matchers; sweeps under skip at 240 (0040) 465 of
505, single pass (0041) 242 of 267. The 65 left are four pairs: China
Cup ~ the Prototype PRO 001 upload (40; 65-68%), Coma ~ Spraycan (16;
58%), Synthesis VIP ~ "Synthesis (Remix)" on a mixed CD (8; 59%), The
Nine ~ Evol Intent VIP (1; 50%).

**Conclusions.** Fast uploads join, no corpus cluster moves; 92% of
session 5's false confident detections were rips. The gap is narrow
(0.766 against 0.840); the owner's verdicts settle the borderline pairs.
