# 0192: integer offsets in the grouped hits, in master

**Question.** Session 41, steps 4-5: with C4 in master (`7269498`), are
all reports identical, and how long do the runs take now?

**Change.** C4 of 0190, tidied: `Hit::taken` stores a hit's integer
offset in its posting's frame bits, `Hit::frame` gives the frame back,
`Hit::slot` folds the offset; `Phases` once per block and rung; the
filter clears by re-reading; a block with a point 2^31 frames out or
more sorts every asset. Tests as pre-registered, plus the round trip.

**Validation** (`work/bin/s41-adopt/`): `scripts/check.sh` green;
`regress session-32-linux` and `session-33-linux-nas` identical (each
report too, apart from `wall_seconds`); the 11 development scans at NAS
scale and at corpus scale (262 tracks) and the clusters sample
byte-identical to the start build's; the full NAS clustering (session
33's run 2 command, scratch copies) byte-identical to the start build's
own run (the file in use was written by earlier builds; its clusters
are the same, not its pair entries).

| Run (Linux machine, otherwise idle) | Start | Adopted |
|---|--:|--:|
| Stakka scan's search, 32 workers | 37.64, 37.92 s | 26.66, 26.69 s (-29.2%, -29.6%) |
| User CPU of that scan | 1,088, 1,096 s | 747, 748 s (-31.3%, -31.7%) |
| The search, 16 workers | 45.63, 45.41 s | 32.51, 32.53 s (-28.8%, -28.4%) |
| User CPU, 16 workers | 679, 674 s | 473, 473 s |
| Clusters sample of 48, 32 workers | 1:56, 1:58 | 1:22, 1:21 (-31%) |
| Its user CPU | 2,504 s | 1,642 s (-34%) |
| `identify` of the Stakka mix, NAS configuration, 32 workers | 25.1, 25.2 s | 19.8, 19.8 s (-21%) |
| Its user CPU | 595, 597 s | 427, 427 s (-28%) |
| `regress session-33-linux-nas` (one run each) | 6:06 | 4:12 |
| Full NAS clustering (one run each) | 23:02 | 14:36 (-37%) |

A B B A each (`work/s41/timing.sh`; `identify` after a warm-up run);
data `data/0192-*.tsv`. Every report identical to the start build's.

**Conclusion.** Adopted. The pre-filter's integer offsets take about
29% off the search's wall time at both worker counts and 21% off a
user's `identify` at NAS scale (which also loads the index and decodes
the mix). What the search's CPU goes to now: 0193.
