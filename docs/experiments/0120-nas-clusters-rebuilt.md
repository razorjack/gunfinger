# 0120: the NAS duplicate clusters rebuilt at 39,354 records

**Question.** Session 23, step 2: what changes in the NAS clusters when
session 18's `clusters --from-peaks` runs again at `d71ba5333cff9a34`?

**Method.** `work/bin/s23-start` (the s22-final bytes), session 18's
queries, full search 02:32-05:26 (370 + 350 + 3 queries, 5.4 GB). It
exited 1 on 0066's Sonar chain, now through the-hifi-junglist's `PRO 012
A` upload (original 100%, 2,858 hits; the revision, judged different, 99%
at 406 hits, 1.1 per s). Its 2,593 pairs were clustered again with
`--reuse-pairs --cut-sparsest`, cutting upload ~ revision (`pair`: 56.8%);
decision in the brief. `clusters_rebuilt.py`; `data/0120-*`.

| NAS clusters | Session 18 (2026-10-14) | Now |
|---|--:|--:|
| Clusters; pairs | 271; 2,147 | 281; 2,593 |
| Files in clusters (NAS) | 886 (632) | 961 (699) |
| Corpus groups as the 262-track clusters' 17 | same | same |

- Added, 75: 59 uploads stored between the runs (the-hifi-junglist's
  23 Aphrodite 12"s, 11 Wormhole tracks, 6 of REACTCD201; 19 others), 6
  stored during the session (easy-t-dnb 5, weedsurfer 1), 8 corpus files alone before
  (7 Wormhole tracks, Fortran - Splinter: each with its VRS 001 upload), 2
  older NAS files joined by a new upload (Perfect Unknown - Epic;
  Dieselboy - The Descent (Phunckateck VIP)). Gone: none (`1.mp3` was in
  no cluster). Nothing split or joined.
- Gap: weakest same 80.3% in both (Known Unknown); strongest unjudged
  pair left apart 56.4% before (Fire ~ Nasty Jungle), 74.2% now (Stalker,
  below), besides the cut Sonar join (98.7%).
- Borderline (40-80%, not joined): 49 rows, 9 new (3 decided by verdicts).
  6 new rows without a verdict make 3 questions for step 6a, Aphrodite
  12"s against other releases: V2's 1999 album (Stalker 74%, Dark Dark
  Side ~ Style From The Dark Side '99 63%), Lacerba's 1997 Tower Bass ~
  the Millennium Mix (48%).
- Queried as members: 22 of the shared panel's 54 validation-half files
  (21 in session 18's run too); no recall-panel validation source.

**Conclusion.** The rebuild adds the newer uploads to their recordings,
changes no old cluster and narrows the gap (74.2% against 80.3%). The
rebuilt clusters are used from step 3 on.
