# 0233: same-title credits (ADR 0022) in the sweeps and calibrate

**Question** (brief 54, step 5). Under ADR 0022's scoring and the one
judgment of `docs/same-titles.txt` (China Cup ~ ƆƐI3C's upload), what
happens to the wrong confident answers and calibrate? Harness scoring
only (`91d5e80`). **Runs.** `work/bin/s54-start` (= `s53-final`) and
`s54-run`: `regress session-53-linux` and `-nas` (seed 2026, the Stakka
scan, leave-outs 3 and 11); `sweep --seed 2027`, `2028` at both scales
in scratch work directories, and `calibrate` there over seeds
2026-2028 and the Stakka scans. `data/0233-same-title-credits.json`.

| Scale, seed | Recalled | Wrong confident, before -> after | Same-title credits |
|---|--:|--:|--:|
| 262 tracks, 2026-2028 | 540 each | 0 -> 0 each | 0 |
| NAS, 2026 | 540 | 7 -> 7 (Cool Rock Stuff) | 0 |
| NAS, 2027 (China Cup indexed) | 540 | 11 -> 0 | 11 |
| NAS, 2028 (China Cup held out) | 540 | 7 -> 0 | 7 |

- Seed 2027: `Bad Company - China Cup.mp3` 4 and `extra/02. Bad Company
  - China Cup.mp3` 7 confident detections of the upload; seed 2028 the
  held-out `extra/b-bad_company-china_cup.mp3` 7 (241-273 hits; 0188's
  counts). 16 and 8 detections gain `same_title`, 5 and 1 below the
  rule; nothing else differs. `regress` is identical at both scales,
  reports and calibrate: seed 2026 draws no China Cup excerpt and no
  development mix plays China Cup.

| Calibrate, seeds 2026-2028 and scans | 262 tracks | NAS, before -> after |
|---|--:|--:|
| Weakest identifying | 675 | 675 -> 675 |
| Strongest false | 100 | 282 -> 282 (Cool Rock Stuff) |
| False candidates; at 30 hits or more | 1,496; 19 | 130,663 -> 130,639; 308 -> 284 |
| False accepted by the rule | 0 | 25 -> 7 |
| Same-title credits | 0 | 0 -> 18 |

**Conclusion.** Every change is a China Cup detection of ƆƐI3C's upload
moving from wrong (or a false candidate) to a same-title credit. At NAS
scale over three seeds the wrong confident answers fall from 25 to 7,
all Cool Rock Stuff ~ the Mobb Deep / Urban Takover upload (a different
mix; its title is a question for us). No threshold moves: the strongest
false (282) and weakest identifying (675) stay. Baselines `session-54-linux(-nas)`.
