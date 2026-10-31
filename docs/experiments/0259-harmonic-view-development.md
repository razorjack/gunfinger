# 0259: the harmonic view on the development families, NAS scale

**Question.** Brief 66 (pre-registration committed at `0da37f3`): at the
alignment the search finds, does a candidate's harmonic part (FitzGerald
2010, §2; medians over 25 frames and 17 bins, soft mask) match the query's
better for true plays than for related versions sharing their drums, at
similar hits, and better than denser peaks of the whole spectrum (D)?

**Run.** `shared --harmonic` (`7604a41`, `work/bin/s66-run`), NAS scale,
the development part's 26 scenarios in four parallel processes (24:06-
34:28, 9.0-10.3 GB each); `harmonic_views.py develop`. 9,678 cases (20
hits or more): 1,763 positives (870 within the negatives' 20-600 hits:
301 blend, 152 looped, 115 damaged, 46 killed...), 2,389 negatives (540
confident), 5,526 others (605 confident). No view abstained, no error.
Data: `data/0259-harmonic-view-development.json`.

| Measure (excess share) | T0 | Kept at T0 (of 870) | T10 | Kept at T10 | Killed at T0 (of 46) | AUC | AUC 20-59; 60-239; 240+ hits |
|---|--:|--:|--:|--:|--:|--:|---|
| H, harmonic | 0.524 | 216 (24.8%) | 0.375 | 313 | 32 | 0.579 | 0.257; 0.403; 0.913 |
| D, denser, no separation | 0.605 | 185 (21.3%) | 0.454 | 321 | 26 | 0.684 | 0.468; 0.591; 0.882 |
| P, percussive (beside) | 0.616 | 123 | 0.437 | 329 | 10 | 0.715 | 0.489; 0.649; 0.864 |
| Hits (neither) | 600 | 0 | 394 | 217 | 0 | 0.591 | 0.547; 0.445; 0.601 |
| Verifier, 12 x 12 (beside) | 0.589 | 235 | 0.484 | 358 | 26 | 0.648 | 0.319; 0.581; 0.914 |

- Medians, positives / negatives, H; D: 20-59 hits 0.084/0.134;
  0.225/0.228; 60-239 0.155/0.211; 0.339/0.284; 240+ 0.544/0.211;
  0.595/0.259. Blend partners (347 hit-matched positives) H 0.113, D
  0.292; the played recording's (523) H 0.477, D 0.549.
- By family at T0, H against D: Phantom Force 137 and 111 of 348, Sonar
  20 and 12, Dozer 19 and 19, Fire 29 and 31, Pyro 6 and 8, Wikki Wikki
  2 and 3, We R The Ones 2 and 0, Tower Bass 1 and 1. Pooled AUC higher
  for H than D in no family (We R The Ones 0.87 and 0.91).
- Near-identical marking: the other rips' 5th percentile of D is 0.029,
  so 2,332 of 2,389 negatives are marked; the marker separates nothing.
  Without the looped and break kinds (beside): AUC H 0.570, D 0.733.

**Reading rule (1)** (H at least 10 points above D at T0, AUC at least
0.05 above D's): not met, +3.6 points and -0.105. Rule (3), the bass
kill: met (32 against 26). **Thresholds frozen** for 0260 (in the data).

**Conclusion.** Under 240 hits the harmonic view scores true plays
below the related versions: a blend partner's sustained material is
masked by the record over it, while drums survive. At 240 hits and more
it separates (0.913), about as the verifier does (0.914).
