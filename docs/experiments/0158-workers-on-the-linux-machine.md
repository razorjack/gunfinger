# 0158: workers on the Linux machine's 16 cores

**Question.** Session 33: clustering and long scans took longer here than on the
Mac (0155, 0157) at the default 32 jobs, one per hardware thread of the AMD
Ryzen 9 5950X (16 cores). Are they faster with fewer workers?

**Method.** `work/bin/s33-start`, scratch work directory `work/s33/jw` (the
saved NAS index hard-linked, the clusters and map copied), power profile
`throughput-performance`, nothing else running. `scan stakka-skynet-knowledge
--leave-out 3` at 16, 24, 32 and again 16 jobs; then `clusters --from-peaks
--sample 48` (48 corpus queries, one round, the index built first) at 16 and 32
jobs. Scripts `work/s33/jobs.sh`, `work/s33/jobs-clusters.sh`.

| Run | Jobs | Wall | User CPU | Peak |
|---|--:|--:|--:|--:|
| Leave-out 3 scan, 56:09 of audio | 16 | 52.4 s*, 52.8 s | 717 s, 722 s | 4.9 GiB |
| | 24 | 56.1 s | 1,156 s | 5.0 GiB |
| | 32 | 76.3 s (76.7 s in step 4) | 2,144 s | 5.1 GiB |
| Clusters sample, 48 queries | 16 | 146 s | 1,886 s | 5.3 GiB |
| | 32 | 208 s | 5,193 s | 6.0 GiB |

- \* The second 16-job run overwrote the first one's logs; its figures are
  from the line the script printed. The clusters sample ran once each.
- The detections do not depend on the workers: the scan reports differ only in
  `wall_seconds`, and the two cluster samples are byte-identical.
- 16 workers take less time than 32: 31% less for the scan, 30% less for the
  clusters sample, with a third of the user time (which counts both threads
  of a core). At 32 the second thread of each core slows these runs, and each
  worker's hit buffers add memory (ADR 0020). Why two threads per core slow the search is not measured (an
  inference: they compete for the core's caches and the memory bandwidth
  while reading the 4.6 GB index at random).
- At 16 jobs a NAS leave-out scan of the Stakka mix takes 52 s (the Mac, ADR
  0014: 107 s at 33,596 records with session 12's matcher). If the sample's
  ratio holds, run 2 of 0155 would take about 37 min at 16 jobs (an
  estimate), near the Mac's 32 min (0128).

**Conclusion.** On the Linux machine, this scan and this clusters sample take
about 30% less time with `--jobs 16` than with the default 32, with identical
results; a full clustering run at 16 jobs is not measured. The
default stays (one worker per hardware thread); changing it is our decision.
