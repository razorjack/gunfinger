# 0021: search memory at 26,462 assets

**Question.** At 8,122 assets each search worker added about 300 MB
(0019). What does the search hold at 26,462 assets, and does it grow with
workers or with the length of the query?

**Command.** `/usr/bin/time -l gunfinger-eval --jobs J memory
--synthetic-copies 100 --minutes M` on the two-pass build (3cb535f), both
ladders, the start of the development mix. Data:
`data/0021-search-memory-at-scale.json`.

**Results.** The built index peaks at 3,132 MB (3,048 MB of index).

| Workers | Query | Peak footprint | Above the build | User CPU |
|--------:|------:|---------------:|----------------:|---------:|
| 1 | 10 min | 4,945 MB | 1,813 MB | 1,031 s |
| 2 | 10 min | 4,770 MB | 1,638 MB | 1,011 s |
| 1 | 5 min | 4,096 MB | 964 MB | 434 s |

**Conclusions.** (1) At this size the search's memory grows with the
query, about 180 MB per minute, and not with workers. What the search
keeps for the whole query is the lines of every rung and window until
chains are built; chance lines multiply with the library. That they are
the bulk is an inference, not measured directly. (2) Extrapolated, an
hour-long mix at 26,462 assets needs about 3 GB of index and 11 GB of
lines. (3) Levers: the common-hash filter (fewer chance hits, so fewer
lines; 0013) and stricter lines at scale, which change detections; and
merging neighbouring rungs' lines as rungs finish instead of at the end,
which can keep them identical.
