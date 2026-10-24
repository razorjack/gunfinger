# 0211: the pitch-ride variant on beatmatched long mixes

**Question.** Session 45, step 5, run B (brief 45, 22:30): on session 35's
beatmatched mixes, where every record plays at the tempo of the one
before and crossfades hold two records at once, does `--apart-in-speed
0.006` keep S2 and S3 of brief 38?

**Method.** `mixes --kind beatmatched`, session 35's test seeds 3511-3513
(16 long mixes each), rendered again from the seeds (corpus files only),
at 262 tracks with the start build, by default and with the variant on
the same renders, in `work/s45/w5`. `variant_compare.py answers`;
`work/s45/drops_beside.py` beside. The default's detections equal
session 35's reports of the same seeds in all 48 mixes.

| Seed (default -> variant) | 3511 | 3512 | 3513 |
|---|--:|--:|--:|
| Indexed plays with a confident detection (S3) | 186 -> 186 | 195 -> 195 | 188 -> 188 |
| Wrong confident detections; added by mix and file (S2) | 0 -> 0; 0 | 0 -> 0; 0 | 0 -> 0; 0 |
| Detections added; removed | 12; 0 | 5; 0 | 4; 0 |

- All 21 added detections are weak (at most 27 hits) and of files not
  playing then (13 with Aphrodite in the file name). 19 lie 0.60-2.49%
  in speed from a stronger overlapping detection of the same file (12
  under 0.9%); 2 overlap none. None is of a played record; these mixes
  hold no ride.
- Times: default with rendering 3:24-3:30 (13.4-14.6 GiB), variant on
  the cached renders 2:31-2:39 (2.6-2.7 GiB).

**Conclusion.** S2 and S3 hold on all three seeds: nothing confident
changes. As on the panel and the drops (0209, 0210), what the variant
keeps is weak, at speed differences from 0.6% up. Data
`data/0211-beatmatched-apart-in-speed.json`.
