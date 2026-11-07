# 0287: the grid's brief plays under library perturbations at NAS scale

**Question** (`docs/brief-81.md`, decided at 20:11 before the runs). Do 0285's short-play moves hold
at NAS scale, where a change of the library is a small share of it, and does any confident answer move?
**Method.** `grid` seed 2026 at NAS scale (40,563 assets indexed), default matcher, `work/bin/s81-run3`,
against its base: `removed-nas` (`--other-sample 40000`, 420 records fewer) and `cut-up-nas`
(`--set-aside-share 0.0101`, 107 more lists of 10,865 in the whole index); 0285's measures and
scripts; `data/0287-short-plays-under-perturbations-at-nas-scale.json`. Its 16 brief plays (of 8
files) and their 960 queries are those of the 262-track grid.

| Grid, strongest matching detections | 10-20 s plays (582) | 25-40 s plays (600) |
|---|--:|--:|
| removed-nas: speed moved > 1e-4 pp; hits moved; by 5% or more; largest speed move | 22; 7; 1; 0.096 pp | 115; 20; 0; 0.005 pp |
| cut-up-nas | 29; 10; 4; 0.33 pp | 138; 25; 0; 0.005 pp |

- No confident identity change in either (960 queries each), no strongest matching detection changed its
  windows, and every detection survived; the 25-40 s plays moved by at most 4 hits.
- The moves of 5% or more are rung swaps or moved pivots of short chains: under `removed-nas`
  Sinthetix - Neurotoxin's 20 s play goes from 677 hits at -0.800% to 814 at -0.704%; under
  `cut-up-nas` Origin Unknown - Elevator's 15 s play from 422 at +5.200% to 557 at +5.157%,
  Neurotoxin's 15 s play from 664 to 558 hits at the same speed, and Clockwork's 10 s play (both
  corpus rips) from 428 at -1.311% to 317 at -1.200%.
- At the base, as at 262 tracks, every confident 10-20 s play has 300 hits or more and no other has 3
  windows and 200 hits; the 15 s and 20 s plays' weakest strongest detections hold 355 and 493 hits at
  both scales, so the moves seen (at most about a third) stay above B(15)'s 200 hits on these plays.
**Conclusion.** At NAS scale, 1% fewer records or 1% more lists set aside move far fewer short plays
than 0285's 262-track changes (1-4 of 582 by 5% or more), but the moves that remain are as large: a
short chain's speed still swaps rungs (up to 0.33 pp) and its hits move by 16-32%. The longer plays
stay within 4 hits. No answer moves, for the window count's reason in 0285. The 16 brief plays come from
8 files; recall's brief plays, near 200 hits under B(15), are the measurement still missing.
