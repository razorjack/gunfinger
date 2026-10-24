# 0214: the adopted analysis, its times and where the CPU goes now

**Question.** How fast is master with C4 (0213) against the start build,
and how does the first pass split its CPU now (the split exploratory)?

**Method.** `work/bin/s46-adopt/` (C4 in master's tree) against
`s46-start`, otherwise idle: the Stakka scan at NAS scale A B B A at 32
and 16 jobs; the clusters sample A B B A; `identify` of the Stakka mix
through the CLI with the corpus configuration and `--playback both`
(262 tracks, both ladders), warm-up then A B B A. Then session 41's
stage timers on the adopted code (`work/s46/timers.diff`), 32, 16, 16,
32 jobs. Every report and JSON output identical to the start build's
(the scans apart from `wall_seconds`). Data `data/0214-adopted-times.txt`.

| Run (Linux machine, 32 workers unless stated) | Start | Adopted |
|---|--:|--:|
| Stakka scan at NAS scale, search | 26.20, 26.08 s | 24.32, 24.12 s |
| The same, user CPU | 724, 729 s | 659, 662 s |
| The same, 16 workers | 31.83, 31.84 s | 29.10, 29.06 s |
| Clusters sample of 48 (stored peaks, no analysis) | 1:20.4, 1:19.8 | 1:18.6, 1:19.4 |
| `identify`, corpus configuration, both ladders | 8.47, 8.50 s | 6.50, 6.66 s |
| Its user CPU | 179.3, 179.2 s | 118.6, 117.9 s |
| `regress session-32-linux` (one run each, unpaired) | 1:08 | 0:52 |
| `regress session-33-linux-nas` (one run each, unpaired) | 4:21 | 4:02 |

| Share of the first pass's CPU | 16 jobs (0205) | 32 jobs (0205) |
|---|--:|--:|
| CPU in the first pass | 410-412 s (446 s) | 664-668 s (707-712 s) |
| Query analysis | 13.1% (20.4%) | 14.9% (23.5-23.6%) |
| Grouping a part | 45.9% (42.6-42.7%) | 40.2% (35.6%) |
| The pre-filter | 18.3% (16.2%) | 20.7% (18.6-18.7%) |
| Taking parts | 11.6-11.7% (10.5-10.7%) | 13.1-13.2% (11.8%) |
| Aligning, sorting, lines | 10.5% (9.7%) | 10.5% (9.9%) |

**Conclusion.** The search is 7.2-7.5% shorter at 32 workers and
8.6-8.7% at 16; `identify` at 262 tracks, where by inference (0028)
analysis is most of the work, takes 22-23% less time and 34% less
CPU. Stored peaks of four files: identical (`work/s46/indexcheck.txt`). The analysis's CPU
fell from 91 s to 54 s at 16 workers and from 167 s to 99 s at 32.
Grouping (40-46%) now costs about three times the analysis.
