# 0260: the harmonic view on the frozen families, with 0259's thresholds

**Question.** Brief 66, reading rule (2): with 0259's thresholds
unchanged, does the harmonic view (H) keep the frozen families' wrong
rate at or below the denser arm's (D) and at most 10%, while retaining
more hit-matched true plays than D?

**Run.** Once, after 0259's commit: `shared --harmonic`, NAS scale, 14
scenarios of seven families in four processes (7:44-16:36, 7.2-8.7 GiB),
and at 262 tracks (7:42) beside. NAS: 3,085 cases; 944 hit-matched
positives (negatives' hits reach 1,288, against 600 in development);
1,156 negatives; 808 others; a detection counts once per index it is
found under. `data/0260-harmonic-view-frozen.json`.

| Measure, NAS scale | Kept at T0 (of 944) | Negatives kept at T0 (of 1,156) | Kept, negatives at T10 | Killed at T0 (of 49) | AUC; 20-59; 60-239; 240+ |
|---|--:|--:|--:|--:|---|
| H, harmonic | 483 (51.2%) | 6 (0.5%) | 560, 163 | 43 | 0.667; 0.602; 0.184; 0.896 |
| D, denser | 474 (50.2%) | 4 (0.3%) | 599, 98 | 37 | 0.724; 0.673; 0.414; 0.903 |
| P, percussive (beside) | 439 | 0 | 603, 71 | 31 | 0.752; 0.693; 0.522; 0.903 |
| Hits (neither) | 338 | 68 | 554, 106 | 24 | 0.738; 0.697; 0.625; 0.691 |
| Verifier (beside) | 563 | 13 | 629, 241 | 45 | 0.739; 0.468; 0.384; 0.954 |

- **The Cool Rock pair** (105 negative cases, 80 confident, 31-1,288
  hits; median excess H 0.341, D 0.408): kept at T0 by H 6 (two
  detections, 141 and 256 hits, of the 30 s alone query, under three
  indexes), by D 4 (310 and 770 hits, under two), P 0, the verifier 9,
  hits 68. Its 75 hit-matched true plays (50 of them the blend partner):
  H keeps 12, D 6. AUC H 0.242, D 0.436, hits 0.170.
- By family, H's AUC above D's only for Watermelon (0.720, 0.653); Clockwork
  0.915, 0.940; Konflict 0.633, 0.739; Stalker 0.594, 0.647; Star Trails
  0.672, 0.771 (Playground Shades has no negative).
- 262 tracks (beside): 217 negatives (the Clockwork remix, 33-134 hits)
  and 25 hit-matched positives, all rejected at T0 by every measure; AUC
  H 0.114, D 0.120: the hit-matched positives are blend partners.

**Reading rule (2):** not met: H's wrong-answer rate (0.5%) is above D's
(0.3%), though both are under 10% and H keeps 9 more true plays.
**Conclusion.** On fresh families the harmonic view does no better than
denser peaks; the percussive part alone has the highest AUC (0.752)
and no negative above its threshold.
