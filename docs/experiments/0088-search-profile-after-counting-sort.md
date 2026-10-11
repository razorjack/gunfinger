# 0088: where the NAS search spends its CPU after the counting sort

**Question.** After experiment 0086's counting sort, which stage of a NAS
`identify` comes next? Measurement only, for the roadmap.

**Method.** As 0085, with the session's last code (`work/bin/s12-item5/`,
`0a625f3`): `identify` of the 2003 mix with `--config nas-dnb.toml
--store-only --playback both -f json -v`, the saved index warm (store
`f9f9bc706195a71b`), `sample` every 10 ms over the whole run, idle
machine (`work/s12/after-item5.sh`). `work/s12/py/sample_stages.py` gains
markers for `WindowHits::lines`, whose own frame holds the inlined
grouping, clustering and line building. Data
`data/0088-search-profile-after-counting-sort.json`.

| | 0085 (`7ef893f`) | Now |
|---|--:|--:|
| Wall; CPU; peak memory | 177.7 s; 1,548 s; 4.78 GB | 92.1 s; 780 s; 4.80 GB |
| CPU samples | 134,528 | 66,689 |
| Grouping by asset | 67,133 (49.9%) | 21,594 (32.4%), with |
| Lines from sorted hits | 10,140 (7.5%) | the lines |
| Sorting each asset's offsets | 37,975 (28.2%) | 26,219 (39.3%) |
| Analysing the query on each rung | 13,094 (9.7%) | 12,595 (18.9%) |
| Hashing and posting scans | 4,322 (3.2%) | 4,305 (6.5%) |
| Second pass | 1,556 (1.2%) | 1,559 (2.3%) |
| Loading the saved index | 129 | 243 (3.8 s, the file cache colder) |

The report is byte-identical to the start build's.

**Conclusions.** Grouping and building lines fell from 77,273 samples to
21,594, and sorting each asset's offsets from 37,975 to 26,219 (16-byte
elements, assets with fewer than 3 hits skipped). Sorting offsets is now
the largest stage (39%), then grouping and lines (32%), then the query
analysis (19%), which is unchanged. The next steps with identical
detections: sorting the 30-60 offsets of each asset with fewer
comparisons (a radix pass over their bits keeps the order exact), and a
grouping that writes to fewer places at once (two passes over asset
ranges) to cut cache misses in the scatter. Neither is tried here.
