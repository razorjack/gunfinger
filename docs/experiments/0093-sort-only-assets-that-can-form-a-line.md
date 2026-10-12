# 0093: sorting only the assets whose hits can form a line

**Question.** Step 5 of session 15: sorting each asset's offsets was 39%
of the NAS search's CPU (0088). Can most sorts be skipped with identical
detections? **Profile first** (0088's method, `work/s15/profile.sh`, the
2003 mix at NAS scale, `s15-start`): sorting 40.1% of CPU samples,
grouping and lines 31.8%, query analysis 18.9%. Nearly every one of the
33,596 assets has 3 hits in a window, and few can form a line.

**Pre-filter.** Before sorting an asset, count its offsets in 4-frame
buckets folded into 4,096 one-byte slots; sort only if 3 hits lie in one
slot or two neighbouring ones, stopping at the first such hit. Exact: a
line's offsets differ by at most 2 frames as computed, so by less than 4,
and their buckets by at most one; folding only adds counts. A test holds a
case where rounding makes a 2-frame bucket miss a line.

**Benchmark** (`work/s15/perf`, the per-asset stage only, medians of 3
alternating runs, lines compared exactly; logs `work/logs/s15-perf-*`):

| Real windows of the 2003 mix (rungs 1.00, 0.96, key-locked 1.02) | Sorted | With lines | Filtered / current |
|---|--:|--:|--:|
| NAS, 120 windows of 0.59-2.0 M hits (1.763 s current) | 1.8% | 0.3% | 0.216 (0.18-0.38) |
| 262 tracks, 279 windows of 4.6-21 k hits (0.042 s) | 3.5% | 0.5% | 0.220 (0.13-0.32) |

**Change**: `search::lines::may_form_line` before the sort in
`WindowHits::lines`; 4 KB per worker. **NAS acceptance** (`identify` of
the 2003 mix, `--store-only --playback both -f json`, saved index warm,
store `f9f9bc706195a71b`, A B B A; another process used up to a core):

| Build | Wall | CPU | Peak memory |
|---|--:|--:|--:|
| `s15-start` | 96.9, 93.5 s | 776, 769 s | 4.78, 4.80 GB |
| pre-filter | 65.0, 65.7 s | 482, 485 s | 4.80, 4.78 GB |

Four reports byte-identical. `regress session-15-start` identical (212.0
s, 1,771 s CPU; 217.3 s at the start). Profile after: sorting 33,129 ->
1,681 samples (3.3%); grouping and lines, now with the filter, 26,224 ->
25,912 (51.3%); analysis 30.1%; CPU samples -39%. **Kept**: 31% less
wall time and 37% less CPU at NAS scale, the same memory. Next: the
grouping's scatter. Data `data/0093-*.json`.
