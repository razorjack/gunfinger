# 0235: the estimate of a gap's tracks, printed (ADR 0024)

**Question.** Session 56: what do the development mixes' gap lines print, does the product's code
reproduce 0165, and what does the estimate cost `identify`?

**Method.** `identify --store-only` of each mix, start and run build, 262 tracks and NAS scale
(`work/s56/cli.sh`); each gap against the manifest (`gapsvs.sh`). 0163's six test seeds rendered
again in a scratch directory; `self-match`, which now writes the product's `tracks`, on session 35's
and today's reports at 30 and 45 s (`seeds.sh`, `reproduce.py`). Times: three alternating runs.

| Scale, mix | Gap line | Missed tracks there | Count |
|---|---|---|---|
| 262, 2003 | 0:00 (2:25; about 1-2), 20:04 (4:15; 1-2), 29:25 (1:39; 1-2) | 1 Konspiracy; 6 Kinetic; 8 Artificial Life | exact |
| 262, 2003 | 6:43 (7:57; about 1-3 tracks) | 3 Escape Route, 4 Phantom Force | exact |
| 262, Nocny | 6:19 (3:04; about 1-2), 46:29 (2:04; 1-2) | 2 Mutated; 12 Stab 2 Def | exact |
| 262, Nocny | 15:04 (3:30; about 1-3), 1:01:23 (3:32; 1-3) | 4 The Descent VIP; 17, "ID - ID" (no reference) | 2 for 1 |
| 262, Nocny | 21:42 (14:27; about 2-4 tracks) | 6 Invid, 7 Mistaken Identity, 8 Nephelim | exact |
| 262, Nocny | 51:07 (6:36; about 1-3 tracks) | 14 We R The Ones, 15 Cannibal | exact |
| NAS, Nocny | 1:01:24 (3:10; about 1-2 tracks) | 17, the same | exact |

Every range holds. Nocny at 262 tracks has one clean interval, so its counts are method 2's alone
(`repeats`). Plays, gaps and other lines are unchanged; Stakka and the 2003 mix at NAS scale print no
gap. The CLI (a slice of the decoded samples) and `self-match` (`-ss`) agree on all 10 gaps.

| Test seeds of 0163, 384 gaps | Exact, k = 1-4 | Range holds | Off by two |
|---|---|---|---|
| 0165 (Python, session 35) | 99 / 57 / 58 / 43% | 100 / 99 / 100 / 89.6% | 2.9% |
| The product's code, session 35's or today's reports, 30 or 45 s | the same | the same | 2.9% |
| Shared material only (192) | 98 / 44 / 48 / 31% | 100 / 100 / 100 / 83.3% | 4.2% |

The renders give session 35's blocks, and the estimate equals `gap_tests.py`'s for every gap.

**Cost** (NAS scale, the Linux machine, 32 workers): the gap matching takes under 0.05 s per
mix (0-1 gaps) and 0.7 s for one 67-minute gap (Nocny with nothing found; 200 MB peak against
158). The search, unchanged, ran 4-6% slower in the first build and as fast without the call
(compiled differently, inferred); with the loop in the core, `identify` takes 0.3-0.5 s more
(12.8 -> 13.1 s for the 2003 mix, 18.1 -> 18.6 s for Nocny). `regress` identical at both scales.

**Conclusion.** The product reproduces 0165 exactly. On the development mixes every printed range
holds and 9 of 11 counts are exact; the two misses count one track as two, inside the range.
