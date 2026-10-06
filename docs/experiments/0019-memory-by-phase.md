# 0019: memory by phase; the index built in two passes

**Question.** What used the memory when the scan at 26,462 assets swapped
(0012)? Inference: the build holds every `PeakRecord`, a `Point` per peak
and the postings at once. If so, build in two passes over the store.

**Command.** `/usr/bin/time -l gunfinger-eval --jobs J memory
--synthetic-copies N [--until loaded|built] --minutes 10` (new; searches
10 minutes of the development mix, both ladders) before (82dcc3c) and
after (3cb535f); kernel peak footprint (`ps` RSS undercounts on this
machine). Data: `data/0019-memory-by-phase.json`.

**Build, peak footprint.** Inferred = records + points + index.

| Assets | Before: loaded | Before: built | Inferred | After: built |
|-------:|---------------:|--------------:|---------:|-------------:|
| 262 | 67 MB | 181 MB | 166 MB | 117 MB |
| 2,882 | 748 MB | 1,815 MB | 1,715 MB | 429 MB |
| 8,122 | 2,096 MB | 5,047 MB | 4,783 MB | 1,042 MB |
| 26,462 | | about 16 GB (extrapolated) | | 3,132 MB |

The inference holds to within 6%. The two-pass build holds the index and
one record; its CPU at 8,122 assets is 19.6 s against 16.9 s.

**Search, peak footprint after the change, 10 minutes of query.**

| Assets | 1 worker | 4 workers | 10 workers |
|-------:|---------:|----------:|-----------:|
| 262 | 144 MB | 167 MB | 178 MB |
| 8,122 | 1,918 MB | 2,859 MB | 4,680 MB |

At 8,122 assets each worker adds about 300 MB (a rung's window of hits at
24 bytes each, and its lines) to 1.6 GB of index and lines.
`identify`, 262 tracks, 10 workers: 249 MB before, 131 MB after for 10
minutes; 395 and 369 MB for the whole mix (249 and 257 s of user CPU);
JSON byte-identical. `regress session-3-start`: identical.

**Conclusions.** The build no longer limits memory; the search does, and
grows with workers and library size. Levers: fewer workers, the
common-hash filter (62% fewer postings scanned, 0013), compact hits.
