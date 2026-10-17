# 0137: the minimum span (variant A) counted from master's saved reports

**Question.** Session 27, step 4, source M (0136): variant A(S), 240
hits over S s or more from the first to the last aligned hit.

**Method.** `scripts/analysis/span_rule.py` (grid, mixes, scans,
shared). Today's rule first reproduced every stored level and score of
each report (0 problems; shared detections taken as fitted, brief
00:47). `mixes-seed-7.json` (2026-10-06) is the single pass, not today's
matcher, so seed 7 is V's (0138). Data: `data/0137-m-*.json`.

| Panel | Today (3 windows) | A(10) | A(15) | A(20) |
|---|--:|--:|--:|--:|
| Grid: confident of 960; flips | 693; 30 | 803; 2 | 644; 2 | 482; 1 |
| Grid at 10, 15, 20 s (of 160) | 0, 67, 146 | 3, 160, 160 | 0, 4, 160 | 0, 0, 2 |
| Grid: shortest length, any / all offsets | 15 / 25 s | 10 / 15 s | 15 / 20 s | 20 / 25 s |
| Mixes 2026: confident, possible of 115 | 106, 9 | 114, 1 | 111, 4 | 96, 19 |
| Scans at 262 (6 reports): levels changed | | 0 | 0 | 0 |
| Shared 1, played left out: wrong of 218 | 106 | 109 | 109 | 106 |
| Shared 1: new wrong confident detections | | 8 | 6 | 0 |
| Shared 2, played left out: wrong of 135 | 5 | 5 | 5 | 5 |

- No wrong confident answer anywhere else (grid's strongest false 34
  hits; mixes 0). The grid's shortest confident span: 14.3 s today,
  11.8 s under A(10).
- Shared 1's new wrong answers under A(10): the 20 s `alone` queries of
  fire-2 and fire-6 (Fire's Rogue Unit VIP and Remix, 642 and 703 hits
  in 2 windows, span 19.8 s) and pyro-2 (Time Is Broken, 270 hits),
  each in the played-left-out and the everything index, and a third
  segment of the VIP Remix in fire-2's 240 s loop (342 hits, 10.3 s). Under
  A(15) the loop's stays possible. The same rule makes the played file
  confident in 23 of these brief queries (16 everything, 7 source left
  out), and 17 and 15 on shared 2.
- Mixes: A(10) promotes 8 plays of 20-26 s (mix 10's Dominion among
  them), A(20) demotes 10 of 23-28 s.

**Conclusion.** Criterion 1 fails for A(10) and A(15) on the first
shared panel (3 queries newly wrong with the played recording left
out); A(20) adds no wrong answer but fails criterion 2 (fewer confident
grid queries). No variant A meets both criteria. B needs V (0138).
