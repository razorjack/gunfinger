# 0020: generated mixes with exact truth; the window grid

**Question.** How do brief plays, blends and boundaries fare when the truth
is exact, and how much does the 3-window rule depend on where the 10 s
windows fall?

**Command.** `gunfinger-eval mixes --count 12` and `grid` (seed 2026, both
ladders, the sweep's index of 209 assets; held-out clusters as in the
sweep). Mixes: plays of 20-60 s at 0.94-1.06, 40% bass swaps, 30%
crossfades, 30% cuts, a returning track, held-out tracks. Grid: 8 tracks ×
2 positions, plays of 10-40 s between held-out tracks, slid 0-9 s against
the grid, same samples throughout. Data:
`data/0020-generated-mixes-and-window-grid.json`.

**Mixes.** 115 indexed plays: 107 confident, 8 possible, none missed; no
wrong answer (strongest false 46 hits). All 8 possible plays last 20-26 s,
span 2 windows, and 7 of them have 265-490 hits. Boundaries: start late by
median 2.7 s, end early by 2.7 s (fades). Speed error median 0.003%.

**Grid**, confident of 160 queries (briefs whose verdict changes with the
offset alone, of 16). Possible or better: 149 at 10 s, all others.

| Play | 3 windows | span ≥ 10 s | span ≥ 15 s | span ≥ 20 s |
|-----:|----------:|------------:|------------:|------------:|
| 10 s | 8 (1) | 10 (2) | 9 (1) | 9 (1) |
| 15 s | 76 (16) | 160 (0) | 12 (3) | 12 (3) |
| 20 s | 148 (12) | 160 (0) | 160 (0) | 9 (1) |
| 25-40 s | 480 (0) | 480 (0) | 480 (0) | 480 (0) |

No wrong answer; strongest false 38 hits. Cut boundaries are within 0.5 s.
In 61 queries (55 of one brief) the chain took a 3-hit chance line up to
30 s outside the play, adding a window and span (all 10 s confident
results come from this) and moving the fitted speed by up to 2.4%.

**Conclusions.** (1) Whether a 15-20 s play is confident depends on the
grid; a 10 s minimum span (first to last hit) removes that, at no cost
here: the sweep and development scans have no detection with 200 hits in
fewer than 3 windows. It stays an offline comparison until the full
protocol runs. (2) Chains link chance lines across gaps; both rules count
them, and the fitted speed suffers.
