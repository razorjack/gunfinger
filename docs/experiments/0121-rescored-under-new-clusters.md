# 0121: the saved shared-material and development reports, new clusters

**Question.** Session 23, step 3: which published numbers change when
the saved reports (store `b285da9984422fb5`) are scored under 0120's
clusters, before any new run?

**Method.** Offline (`work/s23/rescore/run.sh`, `scripts/analysis/`);
with the old clusters each script reproduces the published numbers
exactly. `data/0121-rescored-under-new-clusters.json`.

| Saved report: old -> new clusters | Right | Wrong confident |
|---|--:|--:|
| 3 development mixes, NAS scans: tracks | 40 = | 9 -> 0 |
| The same, tracks left out (17, 12, 11): tracks | 0 -> 7 | 9 -> 0 |
| 0111, development half: queries (218) | = | 106 = |
| 0116, played left out: plays drawn; related; other | 438 -> 480 | 87 =; 43 -> 1 |
| 0116, source left out: plays drawn; related; other | 139 -> 154 | 16 =; 15 -> 0 |

- The 9 wrong: newer uploads now in their recordings (weedsurfer's
  Phoenix, Trace & Fresh's U-PUNK-001; 5 of REACTCD201 disc 2). The
  leave-out runs kept them indexed, so 7 now count as the left-out track.
  0116's other plays: its named uploads, now right, and Kemal - Animation
  (Silent Extent Remix). 0111, 0112: no development recording gains a file.
- (c) is unknown where the saved holder (60+ hits) is now in the play's
  own recording (no weaker one is saved); (c)-based thresholds undetermined.

| (c) at 60: old -> new (unknown) | True | Wrong related | Dev. mixes |
|---|--:|--:|--:|
| 0116 | 79/453 -> 17/495 (104) | 61/87 = | 28/97 -> 16/104 (19) |
| 0118 (other rips) | 51/292 -> 18/292 (33) | 61/87 = | as 0116 |
| Plays the exploratory cut rejects (0116) | 0 = | 61 = | 4 -> 1 (19) |

- Development mixes' 16 known marks: Stab 2 Def 4, Star Trails 3,
  Clockwork 3, We R The Ones 2, Phantom Force 2, Omni Trio 1, Cannibal 1.

**Conclusion.** The stale clusters explain the 9 wrong confident answers
and 0116's other plays but one. The 87 wrong related plays and their 61
marks do not depend on the clusters. Of 0116's 453 old true plays, 17-79
stay marked (62 unknown); the 42 now right add 0-42; the development
mixes have 16-35 of 104 marked (12 old marks and 7 new plays unknown).
