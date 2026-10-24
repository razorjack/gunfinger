# 0209: the pitch-ride variant on the second shared panel

**Question.** Session 45, run P (brief 45, 21:20): does `--apart-in-speed
0.006` keep brief 38's S2 and S3 on shared-material queries its threshold
has not seen? (Its traces included the first panel, 0179.)

**Method.** `shared docs/panels/shared-material-2-seed-2026-development.json`
at NAS scale, start build (`3c1ecd9`), by default and with the variant,
in `work/s45/w` on session 44's renders (no NAS audio read). 10
scenarios, 169 queries, 440 query and index pairs (everything 169,
played left out 169, source left out 102). `variant_compare.py shared`;
`work/s45/panel_beside.py` beside. The default equals session 44's run.

| Confident detections (default -> variant) | |
|---|--:|
| Role `played` (208 query and index pairs) | 545 -> 545 |
| Role `related` | 12 -> 12 |
| Role `other` | 3 -> 3 |
| Role `partner` | 0 -> 0 |
| `other` segments added, by query and file (S2) | 0 |
| Pairs losing their confident played or partner detection (S3) | 0 |
| Confident detections added; removed | 0; 0 |
| User CPU (s); wall | 5,140 -> 5,145; 18:38 -> 18:36 |

- The three `other` detections are the same in both: Phantom 2018 in
  phantom-force-5's 50 s "before" query, one per index (262 hits, 0.4-47.9
  s, speed 1.0). That scenario's related file is Rift - Meltdown, so the
  roles call Phantom 2018, phantom-force-1's related version, `other`.
- Beside: the variant adds 83 weak or possible detections in 53 pairs
  (weak: 60 `other`, 14 `related`, 7 `played`; possible: 2 `played`, 61
  hits) and drops 2 weak `related` ones. Their speed differences to the
  stronger overlapping detection of their file are 0.60-12.1% (41 under
  0.9%; at most 61 hits). Both runs used 467% CPU; by inference they
  mostly waited on reading the NAS store's records from the local disk.

**Conclusion.** S2 and S3 hold on the second panel's development half:
nothing confident changes in any query, as on the first panel (0179).
Every detection the default drops here at more than 0.6% from a stronger
one of its file is weak or possible. The validation half was not opened. Data `data/0209-second-panel-apart-in-speed.json`.
