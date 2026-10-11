# 0079: Restrict refinement to the known asset

**Hypothesis.** Refinement knows its candidate asset, so ordered posting
lists can avoid scanning unrelated assets without changing hit order.

**Method.** `rustc -O work/perf/postings.rs -o work/perf/postings`;
`work/perf/postings > work/logs/perf-postings.log`. Eight alternating
pairs, 100,000 lookups each, packed postings in asset order, three postings
per asset and both present and absent candidates. Results compared exactly.

| List length | Linear median s | Binary median s |
|--:|--:|--:|
| 4 | 0.000593 | 0.000406 |
| 16 | 0.000725 | 0.000944 |
| 64 | 0.001124 | 0.001695 |
| 512 | 0.006496 | 0.003224 |
| 4096 | 0.202311 | 0.004322 |

**Conclusion.** Binary restriction wins on long lists, the scale problem.
Keep a linear scan for lists up to 64 postings and restrict longer lists
by asset with two partition searches. No layout change: postings remain
4 bytes, and both reference frames and asset ids remain lossless.
Full pre-release regression: all detections identical (experiment 0077).
