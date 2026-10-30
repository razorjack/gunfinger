# 0251: what the second pass loses to the chain's speed, by a local search

**Question** (`docs/brief-63.md`, pre-registered). At the best of 41 speeds
within ±0.4% of the chain's (the grid, offset refitted), or chosen on alternate
windows and counted on the others (the split), how many answers change?
**Method.** `--second-pass-fit grid|cross` (candidates of 80 hits or more),
today's rule, against the default's reports (baselines, session 62's);
`scripts/analysis/second_pass_fit.py`, `data/0251-second-pass-fit-reference.json`.

| Set (NAS scale unless 262) | Right confident: default | grid | split | Weak true, median (p90) hit ratio: grid; split | Wrong, median (p90): grid; split |
|---|--:|--:|--:|--:|--:|
| Sweep 2026, 262 | 540 | = | = | 1.01 (1.07); 0.99 (1.03) | 1.02 (1.30); 0.94 (1.02) |
| Sweep 2026 | 540 | = | = | 1.01 (1.07); 0.99 (1.04) | 1.02 (1.24); 1.00 (1.04) |
| Other rips; +1 rung pair | 408; 407 | =; +1 | -1; -1 | 1.01 (1.07); 0.99 (1.04) | 1.08 (1.30); 0.97 (1.24) |
| `recall` clean; other rips | 522; 174 | =; +2 | =; = | 1.01 (1.07); 1.00 (1.04) | 1.02 (1.17); 0.99 (1.16) |
| `recall` brief (15 s) | 721 | +5 -1 | +5 -1 | 1.05 (1.42); 1.02 (1.42) | 1.03 (1.15); 1.00 (1.10) |
| `recall` brief, other rips | 171 | +22 | +13 -2 | 1.04 (1.26); 1.00 (1.25) | 1.03 (1.17); 1.00 (1.09) |

- No wrong confident answer added in any sweep, `recall` set or scan. Calibrate
  (weakest identifying; strongest false: default, grid, split): 262 tracks
  690; 100, 110, 100. NAS 704; 282, 288, 279.
- Scans (Stakka at both scales, the 2003 and 2002 mixes): every track
  identified; credited hits -4% to +11% (Into Music 3,164 to 3,504). Under the
  grid, 4 weaker confident segments of the 2003 mix lie under a widened one.
- Panels (development halves), newly confident detections: panel 1 grid 21
  (9 other, 6 related, 4 played, 2 partner), none lost; split 8 (4 played, 4
  related: Sonar's), 15 lost (8 other, 7 related). Panel 2, both fits: 9
  related (Phantom 2018, 223 to 244), 3 other, 3 played; split loses 2 played.
- Speed error on the own file, median: 30 s excerpts 0.003% (chain), 0.010-
  0.012% (fits); brief plays 0.049%, 0.014% (grid), 0.012% (split). Largest
  grid-to-default ratio 2.13, below 3: the floor of 80 holds.
- Echos (911; 910): 503 at +0.933% (386 at +1.056%) to 603 (457) at +0.99%
  (grid), 597 (455) split. Siren Bass at -8%: 305 to 339 at -8.26% (grid), 306
  split; with one extra rung pair 214 to 241 (grid, confident), 237 (split).
**Against the bar.** (a) passes (split: `recall` and sweeps +18 -4, panels'
played +7 -2); (b) fails (medians 0.99-1.02). The bounded fit runs (0252).
**Conclusion.** The chain's speed costs hits where the chain is short (15 s
plays: the mean of rungs), not on 30 s excerpts. The split keeps most of that
gain, so it is not selection. Most changed answers cross 240 hits (2 gain and
1 loses a third window); related versions near 240 gain as true plays do.
