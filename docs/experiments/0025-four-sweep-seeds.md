# 0025: the sweep with four seeds on saved panels

**Question.** Calibration rested on one sweep draw (seed 2026: 80
excerpts, 48 held-out clusters). How do the margins look over more
draws, with the panels saved so the same draws can be repeated as the
library grows?

**Command.** `gunfinger-eval sweep --seed N` for N = 2027, 2028, 2029
(default matcher, both ladders), each drawing and saving
`docs/panels/sweep-seed-N.json`; then `calibrate`, which reads every
sweep seed and the development scans. Data:
`data/0025-four-sweep-seeds.json`.

**Results.** Each seed: 540 of 540 indexed excerpts recalled at the nine
speeds, 0 wrong, speed error at most 0.024%. Together the four draws
excerpt 208 distinct assets.

| Calibrate | Seed 2026 | Seeds 2026-2029 |
|-----------|----------:|----------------:|
| Identifying detections; false candidates | 559; 15,603 | 2,179; 70,506 |
| Weakest identifying | 501 | 403 (Fibre Optix - Sin at -3%) |
| Strongest false | 97 (the Clockwork remix) | 97 (the same) |
| Margin | 5.16× | 4.15× |
| 200 below the weakest identifying | 2.50× | 2.02× |
| Strongest on audio not in the index | 19 | 28 |
| 60 above that | 3.16× | 2.14× |

The five weakest identifying detections are Fibre Optix - Sin at speeds
between rungs (403-450 hits) and one Gridlok excerpt at -5% (436). The
28-hit candidate aligns a held-out excerpt of a VIP
(`a-unknown-udfr014-(synthesis_vip)`) with Muffler - Bleak; whether they
share material is not known.

**Conclusions.** (1) Nothing crosses a threshold: 0 missed, 0 false
accepted. (2) The margins are narrower than one draw showed: the
confident threshold now sits just over twice below the weakest
identifying, and the possible threshold just over twice above audio not
in the index. (3) The weakest detections are between rungs, which the
second pass lifts (0024). (4) Panels keep these draws fixed.
