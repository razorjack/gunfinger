# 0247: what the power sum costs, and how dense its peaks are

**Question.** Brief 61's cost and density scores: peaks and postings of
the 262-track store under each front end, the time to build it, and the
full search cost of power-sum queries at NAS scale.

**Change.** None beyond 0245's and 0246's harness.

**Command.** Stores: `variant-store` at 32 jobs (0246's runs, the corpus
library on local disk). Search: `work/s61/cost.sh`, the Stakka scan at
NAS scale with the saved index, downmix and `power-sum-queries` A B B A
at 32 jobs on the otherwise idle Linux machine (Ryzen 9 5950X), 02:48-02:50;
the scan's time includes decoding. Data: `data/0247-cost-and-density.json`.

| 265 corpus files, 29.6 h of audio | Downmix | Power sum |
|---|--:|--:|
| Peaks (per second of audio) | 3,972,772 (37.30) | 3,968,260 (37.26) |
| Postings | 7,927,686 | 7,918,939 |
| Record bytes | 19,906,261 | 19,888,735 |
| Build: wall, user CPU, peak memory | 15.8 s, 295.5 s, 0.72 GiB | 15.7 s, 298.7 s, 2.20 GiB |

| Stakka scan, NAS scale, 32 jobs | Downmix | Power-sum queries |
|---|--:|--:|
| Search (`wall_seconds`), pairs A and B | 24.60, 24.83 s | 27.60, 27.74 s |
| Elapsed; user CPU | 27.9, 28.2 s; 680, 686 s | 30.9, 31.0 s; 768, 770 s |
| Peak memory | 4.93 GiB | 5.02-5.05 GiB |

Each pair's reports are identical; the downmix's equals the
`session-54-linux-nas` baseline's. At 262 tracks the same scan took
6.48 s and 9.40 s (0245, one run each, not A B B A), and `robust`'s
median search of one excerpt on one thread (decoding left out) 0.99 s
and 1.71 s, at NAS scale 6.10 s and 7.09 s.

**Conclusion.** Density is unchanged (0.1% fewer peaks and postings).
Building a record costs no more time, since FFmpeg's decoding dominates,
but 3 times the memory at 32 jobs (two channels decoded and held). A
search costs 12% more wall time and 13% more CPU at NAS scale, and about
45% more at 262 tracks, where analysing the query is a larger share.
