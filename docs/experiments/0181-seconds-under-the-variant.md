# 0181: per-second scores under `--apart-in-speed 0.006`

**Question.** Session 38, step 5: with the variant passing its criteria
(0179), how much of the time a track plays does it label, second by
second (Sonnleitner et al. 2016, §5), in the rides mixes and the
development scans? And does `--overlap-by-chains` recover the same sides
on 0179's seed?

**Method.** `seconds` on 0179's saved reports (no search): seed 3801's
four rides conditions at 262 tracks and the three development scans at
NAS scale, default and variant. `--overlap-by-chains` on seed 3801 after
0179 was written (a comparison, not a criterion; 2:37-2:39 each).
Outputs in `work/s38/seconds/`; data `data/0181-seconds-under-the-variant.json`.

| Report (3 windows) | Correct, default -> variant | Start error: median; p90 of its size |
|---|--:|--:|
| Rides 3% over 2-5 s | 73.7% -> 96.2% | +14.3 -> +10.0 s; 109.2 -> 19.9 s |
| Rides 3% over 10-20 s | 92.8% -> 95.6% | +9.8 -> +9.7 s; 23.3 -> 19.9 s |
| Rides 1-2% over 2-5 s | 63.7% -> 96.8% | +22.5 -> +9.6 s; 110.2 -> 20.5 s |
| Rides 1-2% over 10-20 s | 65.6% -> 96.0% | +18.2 -> +9.8 s; 112.8 -> 21.0 s |
| Stakka scan | 91.8% -> 93.5% | |
| 2003 mix scan | 87.9% -> 88.5% | |
| Nocny scan | 91.7% -> 91.7% | |

- Mislabelled seconds unchanged everywhere (0 in the mixes; 210, 309 and
  240 in the scans, at the manifests' approximate borders); specificity
  100% in the mixes and Nocny's 135 s not indexed, unchanged.
- `--overlap-by-chains` on seed 3801: the same 257 sides kept, wrong 0,
  right 191 in each condition. User CPU of the searches on cached
  renders: variant 2,216-2,229 s, chains 2,226-2,238 s.

**Conclusion.** On the rides mixes the variant brings correct seconds
from 64-93% to 96-97% and the 90th percentile of track start errors
from about 110 s to about 20 s; in the development scans it adds about 57 s
(Stakka, Star Trails) and about 17 s (2003 mix, Into Music). The two overlap
rules keep the same sides on this seed.
