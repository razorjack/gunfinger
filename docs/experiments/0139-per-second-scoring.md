# 0139: per-second scoring, today's matcher and the variants

**Question.** Session 27, step 3: how does today's matcher label mixes
second by second (Sonnleitner et al. 2016, §5), and the variants of 0136?

**Method.** `gunfinger-eval seconds` on the variant build's reports (V,
identical to master's where both exist; generated mixes seeds 2026 and
7, 14 mixes), B with V's scorer; session 21's 36 rendered mixes (A only)
with master's. Development mixes: margin 30 s fixed in advance (15, 60 s
beside), start errors for Knowledge and Vertical. `seconds_summary.py`;
data `data/0139-seconds.json`.

| Today's rule | Generated | Dev 262 | Dev NAS | Rendered, played out | Rendered, all |
|---|--:|--:|--:|--:|--:|
| Indexed s; correct; possible | 4,629; 94.2%; 2.9% | 7,526; 86.8%; 0 | 10,013; 90.7%; 0 | 8,405; 83.0%; 2.4% | 19,815; 69.8%; 1.0% |
| Unlabelled; in a margin | 2.7%; - | 1.9%; 6.7% | 1.2%; 0.6% | 13.7%; - | 0.4%; - |
| Mislabelled seconds | 8 | 346 | 759 | 73 | 5,699 |
| Not indexed s; silent | 1,044; 100% | 2,622; 94.9% | 135; 100% | 11,410; 50.7% | 0 |
| Start, end error: median (p90 of absolute) | +2.3 (5.4), -2.6 (5.5) | -12.8 (64.7), -6.7 (39.3) | -10.0 (65.8), +0.5 (66.9) | +1.5 (5.0), -1.4 (4.6) | +1.8 (5.6), -2.1 (6.4) |

- Leave-outs: 97.5% of unindexed seconds silent at 262 (four reports),
  all 10,148 s at NAS scale with every track left out. Generated mixes:
  the 8 s are labels running past their track's end. Development mixes:
  every mislabelled second is a confident label of a track of the mix (0
  wrong identifications) outside its manifest slot and margin; Knowledge's
  labels start a median 47 s (NAS 56 s) before the manifest's starts.
  Margin 15 / 60 s: 517 / 104 s mislabelled (262). Rendered mixes: a
  related version is labelled over the played file; with that file left
  out, half of its seconds stay silent.
- Variants, generated mixes: correct 97.1% (A(10), B(10)), 96.2% (A(15)),
  96.7% (B(15)), 90.7% (A(20)), 92.7% (B(20)); mislabelled 8 s and
  specificity 100% under all. Development mixes: no second changes under
  any variant at either scale. Rendered, played out: A(10) 85.2%, A(20)
  80.2% correct; mislabelled and silent seconds unchanged.

**Conclusions.** (1) The first per-second baseline: 94% of indexed time
in generated mixes is labelled correctly, edges 2-3 s inside the truth.
(2) In the development mixes, the manifests' borders set most of the
error, and the margin decides how it reads. (3) Shared material is the
largest source of mislabelled time; no variant changes it.
