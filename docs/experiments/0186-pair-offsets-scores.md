# 0186: offset-aware measures on every judged pair, against coverage and supported time

**Question.** Brief 40, step 1 (pre-registered at `dd7ed06`): do the
near-offset share, the longest stretch without a near hit or the far
offset jumps order the pairs judged by ear better than coverage and
supported time? The hypothesis came from China Cup ~ ƆƐI3C (different)
and Dilusion ~ dfect-dnb (same), left out of every score here.

**Command.** `gunfinger-eval --other-peaks-dir <NAS store> pair
work/s40/populations/s40-{a,b,c,d}.txt` (`work/bin/s40-run`; 31-52 s
each), `pair_offsets.py measures`, `scores`. (a) 60 judged pairs, (b) 100
joins drawn, 98 distinct (counted once, the (b) AUCs move by 0.001 at
most), (c) 51 panel pairs, (d) 60 presumed different. Data: `data/0186-*.json`.

| Measure | AUC (a) | (b) ~ (a) diff. | (b) ~ (c) | (b) ~ (d) | Threshold | Errors | LOO | (b) below | (c) above | (d) above |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| `coverage` | 0.896 | 0.989 | 1.000 | 1.000 | 0.579 | 8 | 9 | 0 | 1 | 0 |
| `supported` | 0.935 | 0.993 | 0.999 | 1.000 | 0.741 | 6 | 7 | 0 | 5 | 0 |
| `hits_per_second` | 0.833 | 0.929 | 0.975 | 0.999 | 8.38 | 12 | 16 | 10 | 4 | 0 |
| `near_2`, `_5`, `_10` | 0.952 | 0.999 | 1.000 | 1.000 | 0.552 | 4 | 6 | 0 | 1 | 0 |
| `near_20` | 0.955 | 0.999 | 1.000 | 1.000 | 0.705 | 5 | 8 | 0 | 1 | 0 |
| `longest_gap` | 0.892 | 0.994 | 0.999 | 1.000 | 50.5 s | 5 | 7 | 2 | 0 | 0 |
| `far_jumps` | 0.795 | 0.928 | 0.953 | 0.906 | 0.5 | 10 | 10 | 5 | 4 | 9 |

**By the registered bar** `near_10` is worth a roadmap entry: 6
leave-one-out errors against 9 (coverage) and 7 (supported time), and
no (b) join below its threshold; the margin is one pair of 60. T = 2,
5 and 10 s differ on four judged pairs, none crossing the threshold.
`longest_gap` ties supported time and puts two (b) joins below 50.5 s;
`far_jumps` and `hits_per_second` do worse than both.

**The source pairs.** At `near_10`'s threshold (0.552) China Cup (0.765)
and Dilusion (0.906) both fall on the same side: the measure that set
the two apart does not at the threshold the judged pairs fit.
`longest_gap` places both right (81 s and 10 s against 50.5 s). With
them in, the judged AUCs move by at most 0.016.

**Conclusions.** The near-offset share orders the judged pairs slightly
better than coverage or supported time (AUC 0.95 against 0.90 and 0.94),
by one or two pairs; these three separate the presumed populations.
