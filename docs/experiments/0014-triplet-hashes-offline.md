# 0014: longer hashes (triplets), estimated offline

**Question.** Would longer, more specific hashes cut lookup cost at scale
(ADR 0007) without losing the evidence the confident rule needs?

**Method.** `gunfinger-eval hash-cost` (no search): a triplet hashes an
anchor with two points of its target zone (both pair hashes, 64 bits);
`Pairs(n)` and `Triplets(n)` use the first n targets. Library: postings
and lookups per hash. Query: the robustness excerpts (seed 2026, 40 × 2
speeds, experiment 0009) under 10 conditions, counting hashes that meet
their reference within ±2 frames, on the rung and 0.2% off it (between
rungs, the ladder's worst case). Data: `data/0014-hash-cost-seed-2026.json`.

**Results** (per second of audio; relative to the current `Pairs(2)`).

| Hashing | Postings | Lookups | True on rung | True between rungs |
|---------|---------:|--------:|-------------:|-------------------:|
| Pairs(2), current | 74.4 | 4,835 | 44.8 | 28.6 |
| Pairs(1), Pairs(3) | ×0.5, ×1.5 | ×0.45, ×1.6 | ×0.47, ×1.54 | ×0.46, ×1.56 |
| Triplets(2) | ×0.5 | ×1/79 | ×0.33 | ×0.24 |
| Triplets(3) | ×1.49 | ×1/25 | ×1.04 | ×0.80 |

True hashes per 1,000 postings looked up (a chance proxy: chance hits
grow with postings scanned), between rungs: pairs 5.9 under every fan-out,
triplets 115. Under damage triplets keep about half of what pairs keep:
`Triplets(3)` against `Pairs(2)` in absolute true hashes: telephone 0.74,
MP3 32k 0.63, blend -6 dB 0.52, clipping 0.51, noise 10 dB 0.49, echo
0.43, blend 0 dB 0.41, noise 0 dB 0.22. Per posting looked up they still
beat pairs 5-12 times in every condition.

**Conclusions.** (1) Triplets are the lookup-cost lever: 25 times fewer
postings scanned for the same clean evidence (`Triplets(3)`), or 79 times
fewer at half the memory (`Triplets(2)`). (2) The price is robustness:
three peaks must survive, so heavy damage halves the evidence, and the
0.2% residual costs more (a finer ladder would recover some at twice the
lookups, still far below pairs). (3) Counts drop and chance drops faster,
so the hit thresholds would need recalibrating from scratch; remix false
candidates share real audio and would not shrink. (4) Worth a search
variant only if lookups, not memory, limit the library (memory came first
in 0012). `targets` and `pair_hash` (core, behaviour-neutral) stay.
