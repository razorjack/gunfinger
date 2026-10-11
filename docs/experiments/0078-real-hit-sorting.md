# 0078: Check split sorting on actual query hits

**Hypothesis.** Experiment 0075's sorting gain also applies to the ordering
and offset distribution of real postings.

**Method.** `work/perf/sort_actual.rs`, `rustc -O` against the release core;
`work/perf/sort_actual > work/logs/perf-sort-actual.log`. Build the copied
262-record index, skip its fullest 1%, collect the first 10 seconds of hits
from seed-2026 draw 0 at native speed. Repeat the posting lists over 64
copies with distinct asset ids for a scale proxy. Eight alternating pairs;
ordered asset/offset keys compare exactly, cloning outside the timed region.

| Assets | Hits | Combined median s | Split median s |
|--:|--:|--:|--:|
| 262 | 19,532 | 0.000646 | 0.000466 |
| 16,768 | 1,250,048 | 0.399365 | 0.330908 |

**Conclusion.** The large real-derived buffer improves by 17%; keep split
sorting above 50,000 hits. The smaller result differs from synthetic hits,
so retain the simple comparator on small inputs. This is a copied-posting
proxy, not a NAS measurement. A regression test also compares the complete
lines from a large mixed-asset buffer with the existing per-asset path.
