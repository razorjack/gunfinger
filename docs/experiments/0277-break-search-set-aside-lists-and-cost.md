# 0277: a passage query, the lists the filter sets aside and the cost

**Question** (brief 74). Does a passage query need the fullest 1% of posting lists, which the first pass sets aside (ADR 0014), and what does a query cost?
**Method.** 0276's runs: each query searched with every list (*all*) and with the matcher's lists set aside (*filtered*), on one index, one thread per query, 32 queries at a time (`work/bin/s74-run/`). For the cost alone, ten NAS-scale queries (the six Special Branch spans, four random passages) again with `--jobs 1`, nothing else running. Data `data/0276-break-search.json`, `data/0277-break-search-cost.json`.

| NAS scale | All lists | Filtered |
|---|--:|--:|
| Positives found (of 64); first; first 5 | 59; 25; 51 | 56; 24; 49 |
| Found expected files' hits, median | 72 | 50 |
| Other files at 20 hits, median (max): C Special Branch; D `parts` edges | 124 (232); 18 (403) | 25 (58); 9 (139) |
| Random passages: other files at 20 hits, max | 207 | 61 |
| Passage hashes in the lists set aside, median: A; C; D; N1; N2 | (none set aside) | 16.0%; 20.7%; 17.5%; 17.5%; 15.1% |
| Hits kept by the filter, median: expected file A; B; C; D | | 82%; 74%; 49%; 64% |
| Hits kept by the filter, median: the query's own file | | 75-81% by group |
| Thread seconds per query, 32 at once: median (max) | 7.96 (21.4) | 1.55 (2.83) |
| Ten queries alone: median (max) | 5.53 (9.37) | 0.91 (1.23) |

- Found only with every list: Genotype - Toxic, Heist - Sentinels Warning Revisited, Parallax - Watercolors (D). With the filter, fewer break holders compete: Krome & Time - License's expected file moves from rank 28 to 4, Kid Jay - Soundbwoy's from 17 to 44.
- A famous break's passage does not hold more set-aside hashes than a random passage (17-21% against 15-18%), but its holders' hits lie there more often: the shared break is made of the common hashes the filter removes.
- Other costs: the reach of the 20 strongest files, a median 0.03 s (at most 0.8 s); loading the saved NAS index 2.6 s; 147 queries in both modes 55 s wall and 6.2 GiB peak (one index; the filtered mode sets lists aside in place). The ten queries alone: 1:15 wall, 4.5 GiB. At 262 tracks: medians 0.063 s (all) and 0.021 s (filtered) per query, 85 queries in 1.2 s.
- Every list costs a median 5.1 times the filtered search's thread seconds. Thread seconds measured with 32 queries at once are 1.8-1.9 times those of a query alone (memory bandwidth shared on the Linux machine, a Ryzen 9 5950X with DDR4-3600); the listings are identical either way.
**By the registered bar**, (3) holds in both modes: a median 8 thread seconds at most (bar 60).
**What it shows.** For a passage query the filter is a loss: it hides more than half of a famous break's evidence and most of its holders, and saves seconds that a single query can afford. A command would search every list; spread over 32 threads a query would take well under a second after loading the index (an inference: each query here ran on one thread).
