# 0201: what a scattered write costs, and repeated assets

**Question.** Session 43, after 0200 (exploratory): why does grouping's
scatter cost about 4 ns a hit at 32 workers (3.2 ns at 16, 0199), four
times its count pass, whatever the part size?

**Method.** (1) The diagnostic build with one more count
(`work/s43/lines.rs.diag2`, never committed): hits whose asset is the
hit's before it in `part`. The Stakka scan at NAS scale, 16 jobs; report
identical to the start build's. (2) A single-threaded microbenchmark of
the count pass and scatter alone (`work/s43/bench/scatter.rs`, rustc
1.98.0 `-O`, one core, idle machine, best of 5), on synthetic parts of
607 runs of 74 hits over 1,024 assets, each run ordered by asset, with
0-50% of hits repeating the asset before them. Data
`data/0201-repeats-diag.txt`, `data/0201-scatter-bench.txt`.

**Repeats** (49.8 G hits): 40.3% of hits have the asset of the hit
before them; 55% lie in runs of one asset of 2 or more (3.8 hits a run
on average; by inference, tracks holding a hash at several frames).

| ns a hit, count pass + scatter | No repeats | 20% | 40% | 50% |
|---|--:|--:|--:|--:|
| Today's (count, prefix sum, scatter) | 1.66 | 2.43 | 2.36 | 2.35 |
| Count pass and prefix sum alone | 0.40 | 0.44 | 0.47 | 0.49 |
| Assets' places in another order (`* 643 mod 1024`) | 1.61 | 2.62 | 2.58 | 2.56 |
| Runs of one asset copied whole (`chunk_by`) | 3.97 | 5.26 | 6.61 | 6.87 |
| A cursor held while the asset repeats (a branch) | 1.61 | 3.28 | 4.89 | 5.09 |
| Ranks: the count pass keeps each hit's rank, the scatter writes to start + rank | 1.97 | 2.00 | 1.98 | 1.98 |

Random order without repeats: today's 1.45 ns. Run to run, the same
case varies by up to 0.4 ns (1.55-1.93 for today's without repeats).

**Conclusion.** Repeats cost the scatter about 0.7 ns a hit: by
inference, a hit of the same asset as the one before reads the cursor
the previous hit has just written, and the scatter's next write waits
on it, while the count pass's same chain costs little. Branches on runs
mispredict (their lengths vary) and lose. Ranks take the cursor out of
the scatter: every hit goes to the same place as today, and at 40%
repeats count and scatter cost 16% less here. Pre-registered as C4
(brief 43, round 2).
