# 0160: counting a gap's tracks from its length, on detection edges

**Question.** Session 34, step 2, method 1 at 30 s (0159): on detection edges,
how often does the gap over the found entries' median give the missed count
(0154: exact 97/75/48/42% at k = 1-4; criteria M1-M3)?

**Method.** 0159's reports, `gap_scores.py` (each gap once per set and set of
missed tracks). The registered interval skips intervals holding a listed gap;
with up to seven runs per scan, 81 of 115 NAS gaps get none. The single-run
interval (a deviation, brief 07:35) uses the plain scan's intervals, as if the
run were the only one missing, with the gap's own edges. Data
`data/0159-gap-scores.json`; the check: `single_run_check.py`.

| NAS scale, k | Gaps | Single run: exact | -1 / +1 / 2+ | Range A holds | Range B holds | Registered: estimated; A holds |
|--:|--:|--:|---|--:|--:|---|
| 0 | 12 | 0 | 0 / 12 / 0 | 0 | 0 | 8; 0 |
| 1 | 34 | 34 (100%) | 0 / 0 / 0 | 34 | 34 | 2; 2 |
| 2 | 30 | 16 (53%) | 13 / 1 / 0 | 30 | 28 | 6; 6 |
| 3 | 27 | 10 (37%) | 12 / 5 / 0 | 27 | 22 | 14; 14 |
| 4 | 24 | 10 (42%) | 8 / 4 / 2 | 22 | 20 | 12; 11 |

- Single run: M1 holds (range A 113/115, 98%; at least 91% at each k), M2
  (12 of 145 gaps, 8.3%, hold no missed track) and M3 (2 of 115 off by two)
  hold. Range B holds 104/115 (81% at k = 3), narrower (2.2 tracks against
  2.6): the registered choice. Exact falls below 90% from k = 2.
- Registered: M1 fails (33/115 hold, 81 unestimated; A holds 33/34 estimated).
  0154's interval (only the gap's own left out): exact 34/34, 4/26, 7/25, 6/22.
- 24 scans leaving out one run each (k = 1-4 from slots 3, 6): the registered
  interval estimates all 24, as the single run in 22; exact 6, 5, 3, 4 of 6.
- 35 of 45 errors are undercounts; a gap is a median 0.76, 0.85, 0.86, 0.92 of
  k intervals at k = 1-4, probably as neighbours' labels reach into the missed
  tracks (not measured). The plain scans' found median is near the manifest's
  (307/316, 232/233, 251/235 s): every kept track is found here.
- 262 tracks, single run: range A holds 19/19, 13/13, 10/12, 9/11 at k = 1-4.
  Generated (20-60 s, registered): exact 75/89, 17/26 estimated at k = 1-2;
  possible-only plays make no entry, so the found median is 1.14 times the plays'.

**Conclusion.** With one run missing, method 1 meets M1-M3 as a range (B
narrowest), not as a count: exact 100/53/37/42% against 0154's 97/75/48/42%,
mostly low. In a mix missing a third of its tracks most gaps get no estimate.
