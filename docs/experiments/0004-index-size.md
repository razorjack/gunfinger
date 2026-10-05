# 0004: index size: density, posting width, delta coding

**Hypothesis.** The baseline (182 postings/s) is denser than needed; fewer
pairs per anchor or sparser peaks keep the results. 4-byte postings lose
nothing. Delta coding shrinks posting lists further.

**Change.** Variants of peak neighbourhood (±N frames × ±N bins) and fan-out
F, each through stats, sweep (seed 2026), development scan, leave-out 3 and 11,
and `calibrate` (`work/scripts/variant.sh`, results in `work/variants/`). Rule:
the leanest variant whose results (recall, wrong answers, margins) are no
worse than the best. Margins: sweep = weakest identifying / strongest false
over sweep and development; dev = the same on the development set and its
leave-outs only.

**Results** (every variant: 0 wrong anywhere; recall 100% and 11/11 once the
hits threshold is recalibrated; B/p = bytes per posting with offsets table):

| N, F | peaks/s | postings/s | B/p | 25k GB | sweep margin | dev margin |
|------|--------:|-----------:|----:|-------:|-------------:|-----------:|
| 12, 5 | 37.3 | 181.7 | 4.44 | 7.31 | 5.24 | 16.9 |
| 12, 3 | 37.3 | 111.3 | 4.72 | 4.48 | 4.93 | 17.4 |
| **12, 2** | 37.3 | **74.4** | 5.07 | **3.00** | **5.27** | **20.2** |
| 12, 1 | 37.3 | 37.2 | 6.14 | 1.51 | 4.87 | 21.1 |
| 14, 2 | 28.7 | 57.1 | 5.39 | 2.31 | 3.69 | 15.9 |
| 16, 3 | 22.5 | 65.0 | 5.23 | 2.62 | 4.13 | 13.8 |
| 20, 3 | 15.2 | 39.3 | 6.03 | 1.59 | 3.47 | 12.0 |

- Fewer pairs per anchor costs nothing down to 2; sparser peaks lose margin
  at once (wider neighbourhoods drop peaks that survive the speed ladder).
- Posting width (at 12, 2): 8-byte postings (u32 asset, u32 frame) gave the
  identical 4,181 detections at 9.07 B/p against 5.07 B/p. 4 bytes with 16
  asset bits and frames at 32 ms (room for 65,536 assets): sweep margin 4.54
  (-14%), dev margin 20.1, recall unchanged.
- Delta + varint coding of each posting list (asset gap, then frame gap or
  full frame), measured by `gunfinger stats` on the 12, 2 index: 4.19 B/p
  with offsets, 3.12 B/p for the postings alone, against 4.00.
- Buckets at 12, 2: mean 3.74, p99 55, max 1,796; the fullest 1% hold 30.7%
  of postings.

**Conclusion.** Fan-out 2 with the ±12 neighbourhood: 74.4 postings/s, 59%
fewer than the baseline with better margins. Threshold recalibrated to 200
hits (2.11× the strongest false, 95; 2.50× below the weakest identifying,
501). 4-byte postings with 17 frame bits stay; the 16/16 split is the fallback
beyond 32,768 assets. Projection at 25,000 tracks of 402 s: 7.5e8 postings,
3.0 GB fixed-width, about 2.3 GB delta-coded. See ADR 0005.
