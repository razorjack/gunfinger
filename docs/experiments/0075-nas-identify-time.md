# 0075: where a NAS `identify` spends its time with the saved index

**Question.** Item 8b of session 11 (roadmap, "Where search time goes"):
once the index loads from disk, where does `identify` of the 2003 mix at
NAS scale spend its time? Measurement only.

**Method.** `gunfinger identify` of the 2003 mix (46:10) with `--config
nas-dnb.toml --store-only --playback both -f json -v`, the saved index
warm (1afd8cc, store `f9f9bc706195a71b`, 33,596 records), one run under
`sample` every 10 ms (attached about half a second after the start), the
stacks attributed by `sample_stages.py` (0028's method, with stages for
the saved index). `work/s11/item8b.sh`; data
`data/0075-nas-identify-time.json`.

**Timings.** 221.4 s wall, 1,847 s CPU (8.3 cores), 4.79 GB: index
ready in 2.9 s (file 2.2 s), decoding 2.6 s, search 213.6 s, the rest
(plan, report) about 2 s. The search is 96.5% of the wall time.

| CPU samples (166,639) | Now: fitted matcher, 33,596 assets | 0028: skip at 240; single pass, 26,462 |
|---|--:|--:|
| Sorting each window's hits | 82.1% | 77%; 87% |
| Analysing the query on each rung (STFT, peaks) | 9.5% | 12%; 3.6% |
| Clustering offsets | 4.5% | 4.4%; 4.8% |
| Hashing and posting scans | 2.5% | 2.4%; 2.3% |
| Second pass (measuring candidates again) | 1.2% | - |
| Loading the saved index; reading record headers | 0.1%; 0.1% | - |

The leaf frames under the sort are `slice::sort_unstable` on the hits
(quicksort, its pivot and small-sort helpers). Waiting (idle workers):
17,387 samples beside the CPU ones.

**Conclusions.** With the index loaded in 2-3 s, a NAS `identify` is the
search, and the search is sorting hits: 82% of its CPU, more than at
26,462 assets as the index grew. The roadmap's next step stands:
grouping each window's hits by asset before sorting each asset's few
hundred, which gives identical lines and so identical detections.
`regress` would have to show it.
