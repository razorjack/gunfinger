# 0129: the second shared-material panel's development half at NAS scale

**Question.** Session 25, step 4b: on new families (Phantom Force, Star
Trails, We R The Ones), how often does a related version become a
confident wrong title with the played recording left out, against the
first panel's 106 of 218 (0111, 0122)?

**Method.** `docs/panels/shared-material-2-seed-2026-development.json`
(step 4a: 10 directions, 2 of them controls; passages 15-62 s, 108-195
hits by `pair`, against 24-188 s and 41-910 on the first panel). One
`shared` run at 40,126 records (21:41-22:05, 24 min, 5.4 GB), the same
query kinds and three indexes; `shared_outcomes.py --clusters` under
0128's clusters, which also score 0122's run of the first panel
unchanged; `shared_side_by_side.py`; `data/0129-*.json`.

| Played recording left out | Panel 1 | Panel 2 |
|---|--:|--:|
| Directions with a passage; queries | 14; 218 | 8; 135 |
| Wrong confident | 106 (12 directions) | 5 (1 direction) |
| Possible only; nothing | 44; 68 | 105; 25 |
| Wrong by kind: looped; beyond; blend; weak | 34; 33; 12; 15 | 2; 3; 0; 0 |
| Everything indexed: wrong beside the played file | 106 | 5 |
| Positive control: rip confident (with a wrong title) | 65 of 89 (44) | 65 of 85 (5) |

- The 5: Phantom Force played, Phantom 2018 confident (269-286 hits, 6-7
  windows) in looped 120 and 240 s and beyond 10, 20, 30 s. Phantom
  2018 played, Phantom Force reaches 185 hits, possible only.
- Every other related version stays possible: Fracture's edit 152-167
  hits, the Synergy Remix and Star Trails 113-132, the Klute Remix and
  the VIP 144-178. No unjudged file passes 13 hits; an unrelated
  `Taxman - It Comes at Night` reaches 100 (possible) on Phantom Force.
- Controls (Meltdown ~ Phantom Force, 14 hits): the related file at most
  6 hits. Phantom Force's control passage (91.2-129.6 s) overlaps its
  passage with Phantom 2018 (100.3-149.2 s), so Phantom 2018 reaches
  60-227 hits in those controls and 261, confident, in `before 20 s`.

**Conclusion.** The first panel's mechanism repeats (a looped or extended
passage lifts a related version past 240 hits), but on these families
it is rare: 5 of 135 queries, one direction; the rest stop at possible,
most likely as their passages are shorter, with fewer hits (inferred).
