# 0254: the four ignored NAS uploads pruned, against session 54's baseline

**Question** (`docs/brief-64.md`, steps 1-4). Since 2026-10-30 the NAS `.gunfingerignore`
leaves out four uploads the user heard holding more than one recording (status,
2026-10-30): the i-witness-dnb Spider (Optiv & BTK Remix) `[CUT]`, Break - Wine
`[CUT]` and 500 Mills - Vaccine, and the 3zb-gangstep-dnb OMR - Bad Vibes. What does
pruning their peak records change at NAS scale?
**Method.** `doctor` and `prune` listed exactly the four (4 peak records, 4 tag notes);
`prune --yes`; `clusters --from-peaks --reuse-pairs`; `regress session-54-linux-nas`
(`work/bin/s64-start` = `s63-final`); `work/s64/diff.py`. Then `sweep --set-aside-like`
(new, harness only, tested) with the pre-prune saved index (only read): today's
postings, the old lists set aside. `data/0254-prune-four-ignored.json`.

| | Session 54's baseline | After the prune |
|---|--:|--:|
| NAS store records (revision) | 40,678 (`1fc6fe50293ad330`) | 40,674 (`7210fc479da58376`) |
| Sweep 2026: indexed assets; queries with identical detections | 40,575 | 40,571; 707 of 720 |
| Stakka scan; leave-outs 3, 11 | 11/11; 8/11; 0/11, 0 wrong | the same; 1 detection fewer in each |
| Calibrate: weakest identifying, strongest false, false accepted; at 30+ hits | 704, 282, 7; 95 | the same |
| Clusters; pair entries | 284; 2,876 | 284; 2,875 |

- Six sweep queries each lose one sub-threshold candidate of a pruned file (6-8 hits,
  2 windows) and gain none: Bulletproof - Scarred at -5% (Spider), Bad Company - Pulse
  at 0% (Bad Vibes), Item.Code - Magnetics at -3%, -1%, +1% and +8% (Break - Wine).
  Each Stakka scan loses the 500 Mills upload at 5:36-5:46 (7 hits, 2 windows).
- Seven sweep queries change through the lists set aside (0242's mechanism): one
  sub-threshold candidate each in Decoy at -1%, Kemal - Animation at +8% and Konflict -
  RoadBlock at +1% (6 to 8 hits), and Recharger's fitted speed at four speeds (by about
  1e-6 percentage points; hits unchanged). Under the old lists the sweep equals the baseline less the pruned
  files' candidates, `per_speed` included. In the sweep's index (held-out files left
  out) one list left the set aside (9,919 to 9,871 postings, 48 of the four files) and
  one entered (9,911 in both, as the cut fell).
- The Stakka scan's three Clockwork detections keep their hits (3,096, 3,096, 2,480);
  their speed moves in the 8th digit. In the whole index two lists left the set aside,
  9,939 and 9,940 to 9,938 postings; the first holds 11-14 postings of each Clockwork
  file (`work/s64/cut_moved.py`, `list_holders.py`); by inference, its first-pass hits
  move the chain's fit.
- The clusters lose one pair entry, Item.Code - Magnetics against Break - Wine (9 hits,
  20% coverage); no cluster changes. File `c4d3da2e7e37...`.
**Conclusion.** No scored result changes. Every difference is a weak candidate or pair
entry of a pruned file, or a list crossing the set-aside cut. `session-64-linux-nas`
replaces `session-54-linux-nas` as the NAS-scale baseline.
