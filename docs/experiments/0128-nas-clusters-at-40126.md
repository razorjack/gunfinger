# 0128: the NAS duplicate clusters rebuilt at 40,126 records

**Question.** Session 25, step 2: what changes in the NAS clusters when
session 18's `clusters --from-peaks` runs again at `fed26c76b8fe89ea`,
772 records after 0120's (`d71ba5333cff9a34`)?

**Method.** `work/bin/s25-start` (the s24-final bytes), `map-library`
(247 of 262 corpus files with a copy, as before), then session 18's
queries, full search, every verdict. Without `--cut-sparsest` the run
stopped in 0.85 s: `clusters` checks the verdicts against the last run's
pairs before searching, and 0120's hold the Sonar chain. So the search
ran with `--cut-sparsest` (0120's decision in one run; brief, 20:52):
20:52-21:24, 370 + 352 + 3 queries, 5.7 GB. `clusters_rebuilt.py`;
`data/0128-nas-clusters-at-40126.json`.

| NAS clusters | 0120 | Now |
|---|--:|--:|
| Clusters; pairs | 281; 2,593 | 281; 2,636 |
| Files in clusters (NAS) | 961 (699) | 963 (701) |
| Corpus groups as the 262-track clusters' 17 | same | same |

- Added, 2, both stored by the index run after session 23
  (old-skool-jungle-dnb): `Bad Company - The Nine.m4a` joins The Nine
  (the first panel's validation family) and `Trace - Sonar.m4a` joins
  Sonar. Gone: none. Nothing split or joined.
- Cut, 2 (each the sparsest join of a chain between files
  judged different by ear): 0120's PRO 012 A upload ~ the Mark System Revision
  (99%, 406 hits, 1.1 per s) and, new, the old-skool `Trace - Sonar.m4a`
  ~ the revision (100%, 301 hits, 0.8 per s), which joins the corpus
  Sonar at full coverage. A listening question, not a verdict.
- Gap: unchanged, weakest same 80.3% (Known Unknown), strongest unjudged
  pair left apart 74.2% (Stalker). Borderline (40-80%, not joined): 50
  rows, 2 new: the old-skool The Nine ~ the Evol Intent VIP (49%,
  decided by verdict line 32), and the corpus `APH027 ... cool_rock_suff`
  ~ old-skool's `Mobb Deep Vs Urban Takover - Cool Rock Shook Ones`
  (41%, 922 hits, unjudged): a listening question.
- Queried as members: 22 of the first panel's 54 validation-half files
  (the same 22 as 0120) and 6 of the second panel's 19 (Stab 2 Def and
  Tsunami Remix rips, 6 in 0120 too); no panel run touched either half.

**Conclusion.** The 772 records add two uploads to their recordings, one
of them a second link between Sonar and its judged-different revision;
nothing else changes. These clusters are used from step 4 on.
