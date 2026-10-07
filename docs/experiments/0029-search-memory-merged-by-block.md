# 0029: search memory at scale, lines merged a block at a time

**Question.** Item 2 of session 4: what does the search hold at scale;
does merging lines during the search lower the peak, detections identical?

**What it held** (instrumented session-start code, `memory
--synthetic-copies 100 --minutes 10`, 10 workers): index 3.0 GB; each
worker's hits for one window up to 201 MB; every rung's lines 12.1
million (579 MB), 7.3 million after `distinct`. Peak 15.7 GB, but 5.0 GB
with macOS's cache of freed large blocks off (`MallocLargeCache=0`).

**Change** (commit d4d8fb1). A rung-by-rung merge cannot be exact:
`distinct` keeps lines greedily, strongest first, so a line's fate
depends on every rung's lines of its window. Now the query is searched a
block of 12 windows at a time (a job per block and rung), and a block's
lines are merged once every rung has searched it. Peaks of a stretch of
STFT frames are bit-identical to the whole query's (`extract_peaks_in`),
so each window's hits are the same, in the same order. Each worker keeps
one hit buffer. About 1% more analysis at block edges.

**Identity.** `regress session-4-start`: 720 of 720 sweep queries and all
detections of the development scan and leave-outs 3 and 11 identical;
the candidate's scan and the 12 scale reports below: no difference.
Data: `data/0029-search-memory-merged-by-block.json`.

| Development scan (56 min) | Peak before | after | Wall before | after [range] | CPU before | after |
|---|--:|--:|--:|--:|--:|--:|
| 8,122 assets, today's | 5.30 GB | 2.90 GB | 261.5 s | 229.4 [229.1-229.8] s | 1,926 s | 1,944 s |
| 8,122, skip at 240 | 2.12 GB | 1.36 GB | 99.9 s | 90.8 [90.8-90.9] s | 700 s | 706 s |
| 26,462, today's | 9.80 GB | 7.68 GB | 765.6 s | 718.6 [717.4-720.0] s | 5,814 s | 5,841 s |
| 26,462, skip at 240 | 4.43 GB | 3.79 GB | 240.3 s | 229.0 [228.9-229.1] s | 1,710 s | 1,719 s |

Before = 0028 (3 rounds). With `MallocLargeCache=0` its binary peaks at
6.83 GB at 26,462 assets (one round). Distinct lines of the full mix:
12.0 and 0.68 million (today's, skip) at 8,122; 29.3 and 1.59 at 26,462.

**Conclusions.** (1) Peaks fall 36-45% at 8,122 assets and 14-22% at
26,462, wall time 5-12%, detections identical. (2) What remains at 26,462
assets: the index (3.0 GB), the merged lines (1.4 GB at 48 bytes), the
workers' hit buffers (about 2 GB) and the final sort's scratch.
