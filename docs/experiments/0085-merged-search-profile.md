# 0085: where the merged search spends its CPU at NAS scale

**Question.** Item 2 of session 12: after the pre-release audit's
speed-ups (a window's hits sorted by asset first above 50,000 hits,
0080/0083; key-locked peaks shared per rounded hop, 0082), where does a
NAS `identify` of the 2003 mix spend its CPU? Compared with 0075.

**Method.** 0075's method (`work/s11/item8b.sh`) with
`work/bin/s12-start/` (`7ef893f`, byte-identical to 0084's merged
build): `gunfinger --config nas-dnb.toml identify razorjack-2003-03-29.mp3
--store-only --playback both -f json -v`, the saved index warm (store
`f9f9bc706195a71b`, 33,596 records), `sample` every 10 ms over the whole
run, idle machine. Stacks attributed by 0075's script with the sort split
by the closure on the stack (`work/s12/py/sample_stages.py`).
`work/s12/item2.sh`; data `data/0085-merged-search-profile.json`.

| | 0075 (`1afd8cc`) | Now (`7ef893f`) |
|---|--:|--:|
| Wall; CPU; peak memory | 221.4 s; 1,847 s; 4.79 GB | 177.7 s; 1,548 s; 4.78 GB |
| CPU samples | 166,639 | 134,528 |
| Sorting hits: one comparison sort | 82.1% | - |
| Sorting hits: grouping by asset (`sort_unstable_by_key`) | - | 49.9% |
| Sorting hits: each asset's offsets | - | 28.2% |
| Lines from sorted hits (clusters, `chunk_by`) | 4.5% | 7.5% |
| Analysing the query on each rung (STFT, peaks) | 9.5% | 9.7% |
| Hashing and posting scans | 2.5% | 3.2% |
| Second pass | 1.2% | 1.2% |
| Loading the saved index; record headers | 0.2% | 0.1% |

In samples, sorting fell from 136,869 to 105,108 (-23%) and the query
analysis from 15,836 to 13,094 (-17%, the shared key-locked peaks). Every
window of this search holds more than 50,000 hits (the benchmark of 0086
saw 0.6-2.0 million in each of 120 windows, nearly all 33,596 assets in
each), so the comparison sort below 50,000 never runs.

**Conclusions.** The sort still dominates: 78.1% of the CPU, two thirds
of it grouping hits by asset with a comparison sort of 1-2 million hits a
window. Asset numbers are dense and below 33,596, so a counting sort can
group them in two linear passes (item 3, experiment 0086). Sorting each
asset's 30-60 offsets stays; it is 28% and needs a comparison.
