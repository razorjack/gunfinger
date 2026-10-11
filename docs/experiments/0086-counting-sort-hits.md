# 0086: grouping a window's hits by asset with a counting sort

**Hypothesis.** Asset numbers are dense and below the index's asset
count, so a counting sort groups a window's hits in two linear passes
where 0080's comparison sort by asset took 50% of the NAS search's CPU
(0085). Lines depend only on each asset's offsets in sorted order (a
cluster never splits equal offsets), so detections stay identical.

**Benchmark** (`work/s12/perf`; logs `work/logs/s12-perf-*.log`): the
merged code against a counting sort into 16-byte (offset, query frame)
hits, one histogram reused across windows; medians of three alternating
runs, lines compared exactly. Real windows: the 2003 mix on rungs 1.00,
0.96 and key-locked 1.02, against the NAS index and the corpus.

| Buffers | Counting / merged |
|---|--:|
| NAS, 120 windows of 0.59-2.0 M hits (5.557 s merged) | 0.42 |
| Corpus, 417 windows of 4.6-21 k hits (0.199 s merged) | 0.38 |
| Synthetic, 33,596 assets: 1 M, 50 k, 10 k hits | 0.39, 0.42, 0.93 |
| Synthetic, 33,596 assets: 5 k, 1 k hits | 1.5, 6.9 |
| Synthetic, 262 assets: 1 k to 200 k hits | 0.38-0.66 |

Whole 24-byte hits split with `chunk_by`: 0.58 and 0.52 on real
windows. Counting loses only below about a third of the asset count in
hits, by at most 0.1 ms a window; no real window came near: no threshold.
**Change**: `search::lines::WindowHits`, hits found as 8 bytes (posting,
anchor position), offsets computed by the same expressions as before.

**NAS acceptance** (`identify` of the 2003 mix, `--store-only --playback
both -f json`, saved index warm, idle machine, store `f9f9bc706195a71b`):

| Build | Wall | CPU | Peak memory |
|---|--:|--:|--:|
| `s12-start` (3 runs) | 170.9, 171.3, 166.6 s | 1,542, 1,551, 1,544 s | 4.78 GB |
| counting, 24-byte found hits | 93.0, 92.8 s | 794, 793 s | 5.18 GB |
| counting, 8-byte found hits | 92.1, 85.7 s | 774, 773 s | 4.80 GB |

Reports byte-identical to `s12-start`'s; `regress session-12-start`
identical (203.7 s against 212.7 s); checks pass. **Kept**: half the wall
time and CPU at NAS scale, the same memory. Data `data/0086-*.json`.
