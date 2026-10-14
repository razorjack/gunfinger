# 0108: integer phases per query point for the pre-filter (proposal 1)

**Question.** Session 19, proposal 1 of the external review's note: with
one phase per query point (`a = floor(q) mod 2^14`) and each hit's slot
`((r - a) mod 2^14) >> 2`, are the pre-filter and the sort it leads to at
least 20% cheaper (the note's stop point), with identical lines?

**Change** (not kept; `work/s19/lines.rs.p1`): `Phases::of` per rung and
block, guarded (`q` finite, `|q| < 2^32`, else today's `f64` slots); 11
predicate tests against `densest_clusters` (negative offsets, bucket
edges, the wrap, equal offsets, every order of four cases, `q` near 2^32,
float edges, 2 - 2^-52, 200,000 random sets) fail on a 3-frame error.

**Counts** (separate instrumented builds; the 2003 mix at NAS scale,
`--store-only --playback both -f json`, 38,105 records, `b285da99`):
22,714 window searches, 864,119,839 assets with 3 hits, 36.76 billion
hits examined by the filter in both; 17,867,626 assets sorted (start)
and 17,870,712 (phases, +0.017%); 3,408,799 with lines and 4,200,915
lines in both. Reports byte-identical.

**Split profiles** (0103's method, `work/s19/profiles.sh`,
`work/s19/py/stages.py`), standing in for the replay, which does not exist:

| Order, build | CPU samples | Pre-filter | Phases | Sort, clusters | Stage | Analysis |
|---|--:|--:|--:|--:|--:|--:|
| 1 start | 42,974 | 10,915 | - | 1,792 | 12,707 | 10,538 |
| 2 phases | 47,296 | 10,469 | 1 | 2,101 | 12,571 | 12,208 |
| 3 phases | 44,710 | 9,980 | 0 | 1,946 | 11,926 | 11,645 |
| 4 start | 48,262 | 12,086 | - | 2,041 | 14,127 | 12,140 |

Contended: load 24.5-75.6 at each start (other processes;
0109's diagnostic compiled during run 2). **Result.** The stage is 8.7%
cheaper by samples (13,417 to 12,249 on average), 9.5% as a share of CPU
(29.4% to 26.6%), 13.3% against the unchanged analysis stage; the
pre-filter alone 11-16%. The per-hit float work is less than the note's
estimate (about 3,600 samples); the counting, its stores and the hit
loads remain.

**Conclusion.** Below the 20% stop point by every measure: stopped before
A B B A timing; code deleted. Data `data/0108-*.json`.
