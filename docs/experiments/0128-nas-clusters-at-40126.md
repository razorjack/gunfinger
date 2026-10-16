# 0128: the NAS duplicate clusters rebuilt at 40,126 records

**Question.** Session 25, step 2: what changes in the NAS clusters when
session 18's `clusters --from-peaks` runs again at `fed26c76b8fe89ea`,
772 records after 0120's (`d71ba5333cff9a34`)?

**Method.** `work/bin/s25-start`, `map-library` (247 copies, as before),
session 18's queries, full search, every verdict. Without
`--cut-sparsest` it stopped in 0.85 s (the verdicts are checked against
0120's pairs, which hold the Sonar chain, before searching), so it ran
with it, 0120's decision in one run (brief, 20:52): 20:52-21:24, 725
queries, 5.7 GB. `clusters_rebuilt.py`; `data/0128-*.json`.

| NAS clusters | 0120 | Now |
|---|--:|--:|
| Clusters; pairs | 281; 2,593 | 281; 2,636 |
| Files in clusters (NAS) | 961 (699) | 963 (701) |
| Corpus groups as the 262-track clusters' 17 | same | same |

- Added, 2, from the index run after session 23 (old-skool):
  `Bad Company - The Nine.m4a` joins The Nine (the first panel's
  validation family), `Trace - Sonar.m4a` Sonar. None gone, split or joined.
- Cut, 2 (each the sparsest join of a chain between files
  judged different by ear): 0120's PRO 012 A upload ~ the Mark System Revision
  (99%, 406 hits, 1.1 per s) and, new, the old-skool `Trace - Sonar.m4a`
  ~ the revision (100%, 301 hits, 0.8 per s), which joins the corpus
  Sonar at full coverage (`pair`, 22:16: speed 1.0000, all supported):
  a listening question, not a verdict.
- Gap: unchanged, weakest same 80.3% (Known Unknown), strongest unjudged
  pair left apart 74.2% (Stalker). Borderline (40-80%, not joined): 50
  rows, 2 new: the old-skool The Nine ~ the Evol Intent VIP (49%,
  decided by verdict line 32), and the corpus `APH027 ... cool_rock_suff`
  ~ old-skool's `Mobb Deep Vs Urban Takover - Cool Rock Shook Ones`
  (41%, 922 hits, unjudged): a listening question.
- Queried as members, as in 0120: 22 and 6 validation-half files of
  the first and second panels; no panel run touched them.

**Conclusion.** The 772 records add two uploads to their recordings, one
a second link from Sonar to its judged-different revision; nothing else
changes. These clusters are used from step 4 on.
