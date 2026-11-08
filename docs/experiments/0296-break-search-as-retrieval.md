# 0296: break search as retrieval with many relevant holders, and a null with rips clustered

**Question** (brief 86, R2, registered in `7909c1e`; cases `80b57e7`, `data/0296-cases.json`). When every holder in the query's duplicate cluster, an expected file's cluster or a judged relation counts as relevant, how precise is a passage query's list, and does a null of random passages hold once 0295's library-wide clusters join their rips?
**Method.** `gunfinger-eval break-search` (`work/bin/s86-run2/`, `c4a01f0`) at NAS scale (40,688 files), every list and filtered, on the turntable ladder and with `--passage-ladder both`, 0295's clusters in use (scratch directory `work/s86/wb`). Cases: session 74's without panel 1's validation half (A 12, B 7, C 6, D 32, N1 40, N2 40) and E, panel 2's 10 development directions. Each listed holder (own and others merged by hits, the query file left out) is relevant, unrelated by judgment (none exists: the answer log has no such answer) or unjudged. `scripts/analysis/break_retrieval.py`; summary `data/0296-break-retrieval.json`.

| NAS scale, positives A-E (66 scored, 56 recordings) | Turntable, all | Both, all | Both, filtered |
|---|--:|--:|--:|
| Expected holder found; in the first 20 | 61; 60 | 63; 62 | 60; 60 |
| First among non-relevant files; in the first 5 | 40; 55 | 45; 58 | 43; 55 |
| Precision at 5, 10, 20 over judged holders | 1.0, 1.0, 1.0 | 1.0, 1.0, 1.0 | 1.0, 1.0, 1.0 |
| Judged share of the first 5, 10, 20 | 0.589, 0.437, 0.318 | 0.593, 0.432, 0.308 | 0.628, 0.477, 0.363 |
| Found expected files' hits, median | 72 | 72 | 49 |
| Thread seconds per query (147, 32 at once), median (max) | 8.4 (21.6) | 17.6 (44.2) | 3.4 (6.1) |

| Both ladders, all lists, by group | A | B | C | D | E |
|---|--:|--:|--:|--:|--:|
| Found of scored | 12/12 | 6/7 | 6/6 | 31/31 | 8/10 |
| Judged share of the first 10 | 0.588 | 0.938 | 0.571 | 0.293 | 0.788 |
| Listed holders, median | 3.5 | 2 | 134.5 | 21 | 3 |

| The null: strongest holder outside the query's cluster | N1 (40) | N2 (40) |
|---|--:|--:|
| 95th percentile, both ladders, all lists: new clusters; clusters in use | 46; 729 | 40; 606 |
| Turntable, all lists; both, filtered (new clusters) | 45; 26 | 34; 28 |
| Median; maximum (both, all) | 0; 111 | 0; 230 |
| Passages listing a file outside the new cluster that reaches 60 s of the query | 0 | 2 |

- The key-locked rungs find both Watermelon directions (A), which the turntable ladder misses (0276), and raise first places from 40 to 45 with no expected file lost; they double the cost.
- Missed with both ladders: drum loop 7 (no expected holder at 20 hits) and E's phantom-force-5 and -6 (Phantom Force ~ Rift feat. Souldrop - Meltdown both ways; the related file lists no holder at 20 hits, by inference a weak shared passage). The filter loses three more (Watermelon both ways, a `parts` edge of Heist - Sentinels Warning).
- In one D case the expected file is in the query's new cluster: 0295 joins DJ Slip - Man Of Steal with Breaker 1 (a question for the ear: one recording under two titles, or one holding the other).
- No holder is unrelated by judgment, so precision over judged holders is 1.0 by construction; what it rests on is the judged share. Famous breaks keep most holders unjudged: Special Branch 250-270 s lists 224 holders, Krome & Time - License 395, Bigger Star - Raid 436 (both, all).
- The null's 95th percentile falls from 729 to 46 hits once the rips are clustered: 0276's null failed through unclustered rips, as 0276 inferred. N1's strongest outside file is John B - Double-J's passage (111 hits), by inference a break.
**By the registered bar** (both ladders, all lists): (1) expected-holder recall 63 of 66 (95%, bar 90%) holds; (2) precision at 10 over judged holders 1.0, but the judged share of the first 10 is 0.432, under the 0.5 that makes it decidable, so the pack's answers decide it; (3) *T* 46 below the found expected files' median 72 holds; (4) a median 17.6 thread seconds (bar 60) holds. Three of four hold and (2) is undecided: not yet worth a design note as registered.
**What it shows.** With rips clustered the null is valid, and the judged material is found and listed near the top, but most holders of a famous break have never been heard; precision is open until the pack is answered.
