# 0295: duplicate clusters of the whole NAS library, in a scratch file

**Question** (brief 86, R1, registered in `7909c1e`). Can every file of the NAS-scale index be clustered with the clusters' criterion unchanged, and do the clusters keep the existing ones, so that later sessions can use them to label rips among random negatives?
**Method.** `gunfinger-eval clusters --from-peaks --library-wide work/s86/clusters/library-wide.json --cut-sparsest` (`work/bin/s86-run/`, `58c53c1`; `clusters::library_wide`), scratch work directory `work/s86/w`. Every file of the index a query (the 262 corpus files and the NAS records other than the corpus files' copies). Candidates: the file's middle 20 s of stored peaks on the turntable ladder, the fullest 1% of lists set aside; the 20 strongest other files of 30 hits or more. Each candidate measured with the whole file against the found file alone at the fitted speed, and on the whole ladder when that covers 20-80%; same recording at 80% of the shorter file; the verdicts by ear applied, a chain between files judged different cut at its sparsest join. Compared by `scripts/analysis/library_clusters.py` (summary `data/0295-library-wide-clusters.json`; the clusters `work/s86/clusters/library-wide.json`, uncommitted).

| Run (40,688 queries, 32 jobs) | |
|---|--:|
| Wall; user CPU; peak memory | 35:41; 66,425 s; 5.5 GiB |
| Candidates measured (also on the ladder); below 20% coverage | 19,052 (1,099); 1,932 |
| Pairs kept (20% or more, either direction) | 17,120 |
| Clusters with duplicates; files in them | 4,880; 11,165 |
| With a corpus file; without one (files) | 215; 4,665 (10,529) |
| Sizes: 2; 3-5; 6-20; over 20 (largest) | 3,854; 1,004; 22; 0 (10, The Nine) |
| Joins (unordered same-recording pairs); median hits per second | 8,139; 18.3 |

| Against the clusters in use (284 clusters, 1,004 files, 811 joins) | |
|---|--:|
| (a) Old joins kept in one new cluster | 810 of 811 (99.9%) |
| (a) Old clusters kept whole | 283 of 284 (29 of them a corpus file and its NAS copy, one file in this index) |
| (b) New clusters joining two or more old clusters | 0 |
| (d) Sparse joins (under a tenth of the median hits per second) | 5 (0.06% of joins) |
| (d) Borderline pairs (40-80%, or cut, or judged different) | 421 |
| (e) Joins cut between files judged different | 6, in 3 chains |

- The old join lost is Pyro - Time Is Broken (`extra/A. Pyro -- Time Is Broken.mp3`) with a NAS upload of it (`butch-french/Pyro - Time Is Broken.m4a`; old pair 97.8%, 3,108 hits, 7.1 hits per second): neither file's excerpt measured the other (no pair at 20% kept). Why is not traced (by inference, under 30 hits or outside the 20 strongest in its excerpt).
- The excerpts rarely fill the 20 measured files: 0.47 candidates per query on average, so the cap was seldom reached.
- Cuts: a NAS upload of China Cup (`ƆƐI3C - China Cup.opus`, judged different from `extra/b-bad_company-china_cup.mp3`) joins three other China Cup rips at 99-100%, so all three links were cut; Mind Overload (the Molten Beats rip, judged different from the RAMM23 rip) joins two other uploads at 82%; We Enter (Optical Remix v.3.0 instrumental, judged different from the Optical RMX upload) joins V.2.0 at 94%. Each is a question for the ear: the coverage says the same recording and a verdict says different.
- Sparse joins to listen to (coverage, hits, hits per second): Robot Death Squad - Hot 9mm ~ Hot 9 mm (92%, 389, 1.1); Never Forget ~ DJ Lee, Karl K & Jae Kennedy - Come Home (98%, 534, 1.5); Big Brovaz - OK (Ray Keith Remix Vocal ~ Dub) (97%, 602, 1.5); Future Forces Inc - The Killer, the RH08 rip ~ an upload (83%, 456, 1.5); Joe Armon-Jones - Wrong Side Of The Rhodes ~ Dub Side Of The Town (85%, 323, 1.7). By title, at least two are different tracks (a vocal and a dub, or two titles).
- The NAS clusters file is unchanged (`c4d3da2e7e37...` before and after the run); the run wrote only `work/s86/clusters/`.
**By the registered conditions** (1) 99.9% of old joins kept (bar 95%), (2) no new cluster joins two old clusters, (3) sparse joins 0.06% (bar 1%): all hold, so the scratch clusters may label rips among random negatives, with the 5 sparse joins and the 6 cuts unheard.
**What it shows.** Candidates from a 20 s excerpt keep the existing clusters but one join, at 1.6 CPU seconds per query (a whole-file query of `find_around` takes about 40, an estimate from 0169's run), so the whole library takes 36 minutes on the Linux machine instead of an estimated 14 hours; the NAS library holds 4,665 clusters of rips (10,529 files) beyond the corpus recordings', which the clusters in use leave as unrelated files.
