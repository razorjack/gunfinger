# 0294: hash rarity against raw hits on the shared panels and `recall`

**Question** (brief 85, registered in `2066319` before any run). At equal raw hits, does weighting each hit by its
hash's rarity, `w = 1 / ln(e + L)` with `L` the hash's postings in the index searched (set-aside lists included),
separate a record's own material (played) from shared material (related versions, the family's other versions)?
**Method.** `gunfinger-eval --rarity` (`rarity.rs`, `38dce7d`): every second-pass detection of 30 hits or more counted
again at its alignment, `R` hits and `W` their weights; `m = W / R`; `k` the share outside the set-aside lists (0277's
measure, the stated sensitivity form). `shared` on both panels' development halves (3 indexes) and `recall`'s development
panel (clean, damaged, brief, brief-damaged), at NAS scale and at 262 tracks + 109 NAS files (+109). Roles as 0151.
`scripts/analysis/rarity.py`; data `data/0294-rarity-against-hits.json`. Validation halves not named; breaks not rerun.

| NAS scale (families with data) | Panel 1 (5) | Panel 2 (2) | +109: panel 1; 2 |
|---|--:|--:|--:|
| Detections played; related | 624; 3,986 | 665; 964 | 518, 1,448; 537, 884 |
| S1, AUC of W; of R; gain | 0.932; 0.930; +0.002 | 0.961; 0.962; -0.001 | +0.004; -0.002 |
| S2, AUC of m at equal R (pooled over bands); of k | 0.697; 0.362 | 0.453; 0.453 | 0.455, 0.39; 0.526, 0.554 |
| Median m played; related | 0.129; 0.127 | 0.124; 0.124 | 0.308, 0.311; 0.289, 0.255 |
| `break` queries, S2 of m (played; related) | 0.125 (29; 34) | 0.98 (52; 61) | 0.115; 0.844 |
| S5, recount within 5% of the hits: played; related | 90.1%; 91.0% | 80.6%; 80.9% | 92.1, 94.2; 85.1, 79.1 |

- S3, families (NAS; within-band pairs): dozer 1.0 (636), pyro 1.0 (316), sonar 0.939 (726), fire 0.69 (27,591),
  watermelon 0.25 (288), phantom-force 0.639 (19,583), star-trails 0.692 (104 pairs, 2 played); playground-shades and
  we-r-the-ones have no band holding both: 6 of 7 measurable families above 0.5 (+109: the same 6, watermelon 0.0).
- S4, `recall` (NAS): 4,436 right detections against both panels' related ones, S2 of m 0.395 (clean 0.652, brief 0.702,
  damaged 0.239, brief-damaged 0.261; +109 0.52). The damaged queries' hits lie in fuller lists: median `L` 3,956-3,997 for
  the damaged sets, 2,882-2,956 clean. No wrong confident recall detection was measured.
- The rule's counterpart (NAS): `W >= 29.33` accepts the same 930 related detections as `R >= 240` and 1,168 played
  (against 1,150) and 2,285 of recall's right (against 2,247); +109: 1,003 against 971 and 1,605 against 1,581.
- Exploratory, not registered: within one search and index, the strongest played detection has a higher `m` than the
  strongest related one in 191 of 236 searches (panel 1) and 176 of 218 (panel 2), but at unequal hits.
- The recount's reach of 48 frames takes in runs a beat away in looped passages; the second pass's 24 frames on +109's
  panel 2 (brief 85, 05:01) gave 89.4% and 81.8% within 5% and S2 0.523 (from 0.526): the instrument, not the result.

**By the registration.** D1 fails: the overall gain is +0.002 and -0.001, not +0.02, and panel 2's S2 is 0.453. D2
fails on its AUC (0.395) and holds on the counterpart (38 more right). D3 holds (6 of 9 families; two have no data and
star-trails rests on 2 played detections). The validity line fails: 85.2% of played detections within 5% (90% needed),
so no design note would follow even from a pass. **Conclusion.** No. A hit's mean weight varies more between recordings
(per family, the related files' hashes are commoner in 6 of 7) and with damage than between own and shared material, so
at equal hits it does not separate them over a panel, and it ranks damaged true plays below shared material. The overall
AUCs are those of raw hits. The weight closes as a separator of shared material in this form.
