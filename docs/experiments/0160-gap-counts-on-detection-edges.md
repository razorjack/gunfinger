# 0160: counting a gap's tracks from its length, on detection edges

**Question.** Session 34, step 2, method 1 at 30 s (0159): how often does the
gap over the found entries' median interval give its missed tracks' count, on
detection edges, against 0154 (exact 97/75/48/42% at k = 1-4) and M1-M3?

**Method.** 0159's reports, `gap_scores.py`, each gap once per set and set of
missed tracks. The registered interval skips intervals holding a listed gap;
the scans leave out up to seven runs, so 81 of 115 NAS gaps get no estimate.
The single-run interval (a deviation, `docs/brief-34.md` 07:35) uses the plain
scan's intervals, as if the run were the only one missing; the gap keeps its
own edges. Data `data/0159-gap-scores.json`.

| NAS scale, k | Gaps | Single run: exact | -1 / +1 / 2+ | Range A holds | Range B holds | Registered: estimated; A holds |
|--:|--:|--:|---|--:|--:|---|
| 0 | 12 | 0 | 0 / 12 / 0 | 0 | 0 | 8; 0 |
| 1 | 34 | 34 (100%) | 0 / 0 / 0 | 34 | 34 | 2; 2 |
| 2 | 30 | 16 (53%) | 13 / 1 / 0 | 30 | 28 | 6; 6 |
| 3 | 27 | 10 (37%) | 12 / 5 / 0 | 27 | 22 | 14; 14 |
| 4 | 24 | 10 (42%) | 8 / 4 / 2 | 22 | 20 | 12; 11 |

- Single run: M1 holds (range A 113/115, 98%; at least 92% at each k), M2
  holds (12 of 145 gaps, 8.3%, hold no missed track), M3 holds (2 of 115 off
  by two). Range B (quartiles) holds 104/115 (90.4%; 81% at k = 3), passes too
  and is narrower (2.2 tracks wide on average against 2.6), so by the
  registered rule B is preferred. Exact counts fall short of 90% from k = 2.
- Registered: M1 fails (33/115 hold, 81 unestimated); estimated, A holds 33/34.
- Errors are undercounts: a gap is a median 0.76, 0.85, 0.86, 0.92 of k
  intervals at k = 1-4, as neighbours' labels reach into the missed tracks.
  The found median is near the manifest's (307/316, 232/233, 251/235 s):
  every kept track is found, so no short play drops out here.
- 262 tracks, single run: range A holds 19/19, 13/13, 10/12, 9/11 at k = 1-4.
- Generated (20-60 s plays, registered): exact 59/68 and 16/25 at k = 1-2.
  Possible-only plays make no entry, so the found median is 1.14 times the
  plays' (0.64-2.21): where short plays are missed, it is too long.

**Conclusion.** With an interval from a mix where the run is the only gap,
method 1 meets M1-M3 as a range (B narrowest), not as a count: exact
100/53/37/42% against 0154's 97/75/48/42%, biased low by the gap's edges. A mix
missing a third of its tracks has too few clean intervals for any estimate.
