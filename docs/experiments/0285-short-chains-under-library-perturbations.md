# 0285: how short chains' fitted speeds and hits move when the library changes

**Question** (`docs/brief-81.md`, registered). Under unrelated files added or removed, duplicate padding
and a moved set-aside cut, how far do detections move, by play length, and do confident answers change?
**Method.** Default matcher, `work/bin/s81-run`/`s81-run3`; each run against its base, query by query
(`scripts/analysis/perturbation.py`; strongest detection per corpus file; NAS files and copies apart).
262 tracks: `added` (+400 seeded NAS records, `--other-sample 400`), `removed` (that index less 100,
`--other-sample 300`), `cut-down`/`cut-up` (`--set-aside-share 0.0099`/`0.0101`; whole index: 62 fewer,
78 more lists of 6,029), `copies` (2 exact copies of 50 drawn files, sweep only). NAS scale, sweep only:
`removed-nas` (420 fewer records, `--other-sample 40000`), `cut-up-nas` (107 more lists of 10,865).
`grid` seed 2026 (960 plays, 10-40 s), `sweep` seed 2026 (720 of 30 s); `data/0285-short-chains-under-library-perturbations.json`.

| Grid, strongest matching detections | 10-20 s plays (582) | 25-40 s plays (600) |
|---|--:|--:|
| added: speed moved > 1e-4 pp; hits moved; by 5% or more; largest speed move | 131; 98; 23; 0.35 pp | 529; 183; 4; 0.008 pp |
| removed | 97; 54; 14; 0.40 pp | 363; 94; 2; 0.006 pp |
| cut-down | 53; 9; 2; 0.25 pp | 135; 17; 0; 0.008 pp |
| cut-up | 0; 0; 0 | 0; 0; 0 |

| Sweep, detections of 10 hits or more (664 at 262, 667 NAS) | Survived | Speed moved; hits moved; by 5% or more | Of those 5%: under 120 hits |
|---|--:|--:|--:|
| added; removed | 658; 657 of 658 | 566; 186; 11 / 442; 99; 3 | 10; 3 |
| cut-down; cut-up | all | 121; 14; 1 / 0; 0; 0 | 0 |
| copies | 654 | 527; 172; 11 | 11 |
| removed-nas; cut-up-nas | all | 47; 5; 0 / 83; 15; 0 (largest 0.002, 0.004 pp) | |

- Confident identity changes: none in the 11 comparisons (grid 4 x 960, sweep 7 x 720 queries); no
  strongest matching detection of the grid changed its windows; every lost detection had under 120 hits.
- The short plays' large moves are rung swaps of the mean-of-rungs speed (0242's mechanism). Under
  `added`, Item.Code - Kosmic Whoa's 20 s plays go from 1,046 hits at -3.744% to 682 at -3.963% and from
  1,021 at -3.762% to 705 at -3.600%; Swarm's 10 s play from 228 at +5.129% to 25 at +5.480%.
- In the grid's 10-20 s plays the window count decides the level: all 209 confident plays have 300 hits
  or more, and no other play has 3 windows and 200 hits. `cut-up` moved nothing in either command's
  index, by inference because no list length crossed there (ties at the cut).
**Conclusion.** A 10-20 s play's true detection moves by 5% or more 6-7 times as often as a 25-40 s
play's (`added`, `removed`), and its speed by up to a rung (0.25-0.40 pp) where the longer plays' stay
under 0.01 pp. At NAS scale, 1% fewer records or 1% more lists set aside moved 30 s excerpts by at most
3 hits. No confident answer changed, because at 262 tracks the short plays near the rule fail on
windows, not hits; a rule that makes 15-20 s plays confident on hits and span (the minimum span) would
put these moves at its threshold.
