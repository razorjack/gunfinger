# 0149: what exact hash quantisation loses after fitting: pre-registration

**Question.** Roadmap, "Evidence beyond exact pair hashes", step 2: how
much evidence lost under damage and other rips sits one quantisation
step away; does the tolerance raise false and related candidates as much?

**Counts per detection** (`verifier.rs`: its per-window alignment and its
query analysis at the detection's speed). Hash components (`hash.rs`):
anchor level (whole bins below 50, 2% steps above), bin delta, frame
delta. E: query pairs (anchors in the detection's span) matched to the
record's own pairs (anchors in the aligned reference passage) with the
same hash, reference anchor within ±1 frame of its place. N: then, among
unused pairs, hashes whose three components each differ by at most one
step (26 neighbours), same ±1 frame. Each pair counts once, none in
both; no index lookup. V: the verifier's peaks found (±1 bin, ±1 frame)
of P in the passage. Chance: E, N, V at the shifts -6.2, -5, +5, +6.2 s.

**Runs** (40,126 records, one at a time): R1 `--verify recall --queries
clean,damaged`; R2 the same, `--other-rips-only`, clean; R3, R4 `shared`
on both shared panels' development halves; R5 0055's plan if cheap.
**Sets** (fitted detections). T(c), T(a), T(C): per observable searched
query, the strongest own-cluster detection in (c), (a), clean. F: R1, R2
detections outside the own and partner clusters that `labels.py` calls
unrelated (named: wrong possible; strongest on held-out or unobservable
queries). R: R3, R4 related-recording detections (`related`, `version`);
beside it R(r) (R1, R2 related wrong), P (R3, R4 played), R5's.

**Measures.** M1 N/E, (E+N)/E, V/E, V/P, chance-corrected N/E by set and
hit band; M2 median E/E(C), (E+N)/E(C) for T(c), T(a); M3 those under 240
reaching 240 with E+N (and hits+N); M4 AUC of N/E and chance-corrected
N/E within E bands 10-29, 30-59, 60-119, 120-239 and pooled: T(c)~F,
T(a)~F, T(c)~R, P~R; V/P beside. Expected: brief 30.

**Notable, declared before any count.** T(c) median (E+N)/E ≥ 1.5 with
F, R gaining at most half as much (chance-corrected, same E band); R
gaining at least as much as T(c) or P in an E band; ≥ 10% of T(c) under
240 reaching 240, or any F, R(r) or not-confident R; T(a) median
(E+N)/E(C) ≥ 0.75; a pooled chance-corrected AUC ≥ 0.75 or ≤ 0.25;
median E/hits outside 0.8-1.1. Shrinks: R2, then R5. Full text: brief
30, step 2. **Result:** 0151 and up.
