# 0108: integer phases per query point for the pre-filter (proposal 1)

**Question.** Proposal 1: with `a = floor(q) mod 2^14` per query point and
slots `((r - a) mod 2^14) >> 2`, are the pre-filter and its sort 20%
cheaper (the stop point), exactly?

**Change** (deleted; `work/s19/lines.rs.p1`): `Phases::of` per rung and
block, guarded (`q` finite, `|q| < 2^32`, else `f64` slots); 10 tests: 7
against `densest_clusters` (negative offsets, bucket edges, the wrap,
equal offsets, orders, `q` near 2^32, float edges, 2 - 2^-52, 200,000
random sets), 1 against a comparison sort, 2 of the phases alone.

**Counts** (instrumented builds; the 2003 mix at NAS scale, `--store-only
--playback both`, 38,105 records): 864,119,839 assets filtered, 36.76
billion hits examined; 17,867,626 sorted (start), 17,870,712 (phases);
4,200,915 lines in both. Reports byte-identical.

**Split profiles** (0103's method, `work/s19/profiles.sh`, `stages.py`) in
place of a replay, which does not exist. Stage: the pre-filter, phases,
and the sort and clustering it leads to. Phases: 0-1 samples.

| Series, order, build | CPU samples | Pre-filter | Stage | Analysis | Stage share |
|---|--:|--:|--:|--:|--:|
| contended, 1 start | 42,974 | 10,915 | 12,707 | 10,538 | 29.6% |
| contended, 2 phases | 47,296 | 10,469 | 12,571 | 12,208 | 26.6% |
| contended, 3 phases | 44,710 | 9,980 | 11,926 | 11,645 | 26.7% |
| contended, 4 start | 48,262 | 12,086 | 14,127 | 12,140 | 29.3% |
| quieter, 1 start | 36,527 | 9,435 | 11,029 | 9,180 | 30.2% |
| quieter, 2 phases | 35,580 | 8,335 | 9,972 | 9,344 | 28.0% |
| quieter, 3 phases | 35,350 | 8,119 | 9,764 | 9,361 | 27.6% |
| quieter, 4 start | 42,358 | 10,928 | 12,703 | 10,389 | 30.0% |

Contended: load 24.5-75.6 (other processes, 0109's compile
in run 2); quieter: about one core of other load. **Result.** Against the
unchanged analysis the stage fell 13.3% (contended) and 12.9% (quieter);
in samples 8.7% and 16.8% (noisy: the start runs differ by 16%); as a
share of CPU 9.5% and 7.5%. The slot arithmetic is less than the note
estimated (about 3,600 samples); the rest is, by inference, the counting
and the hit loads. **Conclusion.** Below 20% in both series: no A B B A
timing; code deleted. Data `data/0108-*.json`.
