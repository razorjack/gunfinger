# 0120: the NAS duplicate clusters rebuilt at 39,354 records

**Question.** Session 23, step 2 (`docs/brief-23.md`): what changes in the
NAS clusters when session 18's `clusters --from-peaks` runs again on the
store indexed tonight (`d71ba5333cff9a34`), with 17 new verdicts?

**Method.** `work/bin/s23-start` (the s22-final bytes), session 18's
queries, full search 02:32-05:26 (370 + 350 + 3 queries, 5.4 GB). It
exited 1 on 0066's Sonar chain, now through the-hifi-junglist's `PRO 012
A` upload (original 100%, 2,858 hits; the revision, judged different, 99%
at 406 hits, 1.1 per s). Its 2,593 pairs were clustered again with
`--reuse-pairs --cut-sparsest`, cutting upload ~ revision (`pair`: 56.8%);
decision in the brief. `clusters_rebuilt.py` against session 18's
clusters; members dated by record. `data/0120-nas-clusters-rebuilt.json`.

| NAS clusters | Session 18 (2026-10-14) | Now |
|---|--:|--:|
| Clusters; pairs | 271; 2,147 | 281; 2,593 |
| Files in clusters (NAS) | 886 (632) | 961 (699) |
| Corpus groups as the 262-track clusters' 17 | same | same |

- Added, 75: 59 uploads stored between the runs (the-hifi-junglist's
  23 Aphrodite 12"s, 11 Wormhole tracks, 6 of REACTCD201; 19 others), 6
  stored during the session (easy-t-dnb 5, weedsurfer 1), 8 corpus files alone before
  (7 Wormhole tracks, Fortran - Splinter: each with its 2023 upload), 2
  older NAS files joined by a new upload (Perfect Unknown - Epic;
  Dieselboy - The Descent (Phunckateck VIP)).
- Gone: none (`1.mp3` was in no cluster). Nothing split or joined.
- Gap unchanged: weakest same 80.3% (Known Unknown, Rollers Edit ~
  Drumtrip), strongest unjudged different 76.6% (Sex Drive ~ alien5ive's
  INFRA 012 upload); above both, the cut Sonar join (98.7%).
- Borderline (40-80%, not joined): 49 rows, 9 new (3 decided by verdicts).
  6 new rows without a verdict make 3 questions for step 6a: Aphrodite's
  12" against V2's 1999 album (Stalker 74%, Dark Dark Side ~ Style From
  The Dark Side '99 63%, Tower Bass (Millennium Mix) ~ Lacerba's 48%).
- Queried as members: 22 of the shared panel's 54 validation-half files
  (21 in session 18's run too); no recall-panel validation source.

**Conclusion.** The rebuild adds the newer uploads to their recordings
and changes no old cluster. The rebuilt clusters are used from step 3 on.
