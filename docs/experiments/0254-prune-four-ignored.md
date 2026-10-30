# 0254: the four ignored NAS uploads pruned, against session 54's baseline

**Question** (brief 64). The NAS `.gunfingerignore` now leaves out four uploads heard to
hold more than one recording (status, 2026-10-30): the i-witness-dnb Spider (Optiv & BTK
Remix) `[CUT]`, Break - Wine `[CUT]`, 500 Mills - Vaccine and the 3zb-gangstep-dnb OMR -
Bad Vibes. What does pruning them change? **Method.** `doctor` and `prune` listed exactly the four (4 peak records, 4 tag notes);
`prune --yes`; `clusters --from-peaks --reuse-pairs`; `regress session-54-linux-nas`
(`work/bin/s64-start` = `s63-final`); `work/s64/diff.py`. Then `sweep --set-aside-like`
(new, harness only, tested): the pruned store's postings searched with the lists that
the pre-prune saved index (only read) sets aside. `data/0254-prune-four-ignored.json`.

| | Session 54's baseline | After the prune |
|---|--:|--:|
| NAS store records (revision) | 40,678 (`1fc6fe50293ad330`) | 40,674 (`7210fc479da58376`) |
| Sweep 2026: indexed assets; queries with identical detections | 40,575 | 40,571; 707 of 720 |
| Stakka scan; leave-outs 3, 11 | 11/11; 8/11; 0/11, 0 wrong | the same; 1 detection fewer in each |
| Calibrate: weakest identifying, strongest false, false accepted; at 30+ hits | 704, 282, 7; 95 | the same |
| Clusters; pair entries; file | 284; 2,876; `ce293fe6165e...` | 284; 2,875; `c4d3da2e7e37...` |

- Six sweep queries each lose one sub-threshold candidate of a pruned file (6-8 hits, 2
  windows) and gain none: Bulletproof - Scarred at -5% (Spider), Bad Company - Pulse at
  0% (Bad Vibes), Item.Code - Magnetics at -3%, -1%, +1% and +8% (Break - Wine). Each
  Stakka scan loses the 500 Mills upload at 5:36-5:46 (7 hits). The clusters lose one
  pair entry, Item.Code - Magnetics against Break - Wine (9 hits, 20% coverage).
- Seven sweep queries change through the lists set aside (0242's mechanism): in Decoy at
  -1% and Konflict - RoadBlock at +1% one weak candidate gains or loses a hit (6 to 7, 7
  to 6); in Kemal - Animation at +8% one weak candidate moves from +4.8% to +4.0% and two
  new ones appear (6 and 8 hits); Recharger's fitted speed moves at four speeds by about
  1e-6 percentage points, hits unchanged. Under the old lists all 720 queries and
  `per_speed` equal the baseline less the pruned files' candidates. In the sweep's index
  a list of 9,919 postings lost 48 of the four files and left the set aside; the
  shortest list set aside fell from 9,912 to 9,911 postings, so one of 9,911 entered.
- The Stakka scan's three Clockwork detections keep their hits (3,096, 3,096, 2,480);
  their speeds move by less than 1e-5 percentage points. In the whole index two lists
  left the set aside (9,939 and 9,940 to 9,938 postings), and the first of them holds
  11-14 postings of each Clockwork file (`work/s64/cut_moved.py`, `list_holders.py`). By
  inference those postings, now in the first pass, move the chain's fit (not run).
**Conclusion.** No scored result changes. Every difference is of a pruned file, or comes
from lists crossing the cut (traced for the sweep, inferred for the scan).
`session-64-linux-nas` replaces `session-54-linux-nas` as the NAS-scale baseline.
