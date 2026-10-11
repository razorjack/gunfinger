# 0080: Sorting hits by asset before sorting offsets

**Hypothesis.** Separate integer asset sorting and per-asset offset sorting
reduce comparisons on large hit buffers without changing line evidence.

**Method.** `rustc -O work/perf/sort.rs -o work/perf/sort`;
`work/perf/sort > work/logs/perf-sort.log`. Eight alternating pairs on
seeded synthetic hits; both outputs checked for identical ordered asset and
offset keys. Cloning is outside the timed region. Other work was active.

| Assets | Hits | Combined median s | Split median s |
|--:|--:|--:|--:|
| 262 | 50,000 | 0.002080 | 0.002329 |
| 32,000 | 1,000,000 | 0.192199 | 0.082720 |

**Conclusion.** Split sorting helps the large case (0.43 times the wall
time) but loses on the small case. Apply it only to buffers above 50,000
hits; keep the existing comparator below that. Neither representation nor
floating-point offset changes. Real-query measurements are in experiment
0083; full regression is identical (0077). These are not end-to-end timings.
