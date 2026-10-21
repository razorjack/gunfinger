# 0161: long generated mixes with runs of held-out tracks

**Question.** Session 34, step 5a: with exact truth, DJ-length plays and enough
clean intervals, do 0159's minimum length and 0160's method 1 hold?

**Change.** `mixes --kind long` (`mixes.rs`): plays of 120-360 s (at most 85% of
the track at its speed), crossfades of 10-60 s (at most half the shorter play;
half of them bass swaps), speeds within ±6%; 2 indexed plays, then one run each
of 1, 2, 3 and 4 held-out tracks in a drawn order, each followed by 2 or 3
indexed plays (a first version with 1-2 was replaced unread, brief 07:44).

**Command.** `mixes --kind long --count 16` at seeds 2026-2029 and 7 (262
tracks, the sweep's held-out clusters, `--jobs 16`): 80 mixes of 62-90 minutes
(median 73), 20-24 plays, crossfades a median 35 s; 3:26-3:43 and 13.3-14.0 GiB
per seed. `gaps` and `gap_scores.py`. Data `data/0161-long-mixes-gap-scores.json`.

**Results.** 952 of 952 indexed plays confident; no wrong confident play
(6-10 possible ones per seed, at most 144 hits). Step 1: every one of the 800
held-out plays is reached at 10-30 s (799 at 45-60 s, 794 at 90 s), with 100%
of their seconds, and no listed gap at any length lacks a missed track.

| k, at 30 s | Gaps | Exact | -1 / +1 / 2+ | Range A holds | Range B holds (width) |
|--:|--:|--:|---|--:|--:|
| 1 | 80 | 69 (86%) | 0 / 11 / 0 | 80 | 80 (1.4) |
| 2 | 80 | 42 (53%) | 23 / 11 / 4 | 76 | 73 (2.2) |
| 3 | 80 | 31 (39%) | 25 / 20 / 4 | 76 | 74 (2.8) |
| 4 | 80 | 21 (26%) | 24 / 19 / 16 | 64 | 70 (3.5) |

- With the registered interval (every mix has one): M1 holds for range A
  (296/320, 92.5%; 80% at k = 4) and B (297/320; 87.5% at k = 4); M2 holds (no
  gap without a missed track); M3 fails: 24 of 320 (7.5%) are off by two or
  more, 16 of them at k = 4.
- The found entries' median is the plays' (ratio 1.02, 0.56-1.56 per mix): no
  indexed play is missed. Errors run both ways here, unlike 0160's undercount:
  plays of 120-360 s drawn uniformly vary more than a DJ's intervals (0154:
  quartiles 253-352 s for Stakka), which would explain it (not measured).

**Conclusion.** On DJ-length plays 10-30 s reaches every missed run (45-60 s
all but one) and lists nothing else. Method 1 gives a range (A or B hold 92-93%), not a
count: exact 86/53/39/26% at k = 1-4; at k = 4 one gap in five is off by two.
