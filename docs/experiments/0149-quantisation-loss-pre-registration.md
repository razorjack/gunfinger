# 0149: what exact hash quantisation loses after fitting: pre-registration

**Question.** Session 30 (roadmap, "Evidence beyond exact pair hashes",
step 2): on the alignment the second pass fitted, how much evidence lost
under combined damage and other rips sits one quantisation step away,
and does the same tolerance raise false and related candidates as much?

**Counts per detection** (`verifier.rs`, on the verifier's per-window
alignment and its analysis of the query at the detection's speed). The
pair hash's components (`hash.rs`): anchor level (whole bins below 50,
2% steps above), bin delta, frame delta. E: query pairs (anchors in the
detection's span) matched to the record's own pairs (anchors in the
aligned reference passage) with the same hash, reference anchor within
±1 frame of its place; N: then, among unused pairs, hashes whose three
components each differ by at most one step (26 neighbours), same ±1
frame; each pair counts once, none in both. V: the verifier's peaks
found (±1 bin, ±1 frame), of P in the passage. Chance: E, N, V at the
verifier's four shifts (-6.2, -5, +5, +6.2 s). No index lookup for E, N.

**Runs** (`--work work/s30/vwork`, NAS store at 40,126 records, `nice`,
one at a time). R1 `--verify recall --queries clean,damaged`; R2
`--verify recall --other-rips-only`; R3, R4 `shared` on both shared
panels' development halves; R5 0055's plan at 262 tracks if cheap.

**Sets** (fitted detections only). T(c), T(a), T(C): per observable
searched query, the strongest own-cluster detection in (c), (a), clean.
F: R1 and R2's detections outside the own and partner clusters that
`labels.py` calls unrelated (named: wrong possible; strongest on each
held-out or unobservable query). R: R3 and R4's related-recording
detections (`related`, and `version` files); beside it R(r) (R1 and R2's
related wrong detections), P (the played recording in R3, R4), R5's.

**Measures.** M1 N/E, (E+N)/E, V/E, V/P and chance-corrected N/E per
set by hit band; M2 median E/E(C) and (E+N)/E(C) for T(c), T(a); M3
detections under 240 reaching 240 with E+N (and hits+N); M4 AUC of N/E
and chance-corrected N/E at equal E (bands 10-29, 30-59, 60-119,
120-239, pooled within bands): T(c)~F, T(a)~F, T(c)~R, P~R; V/P beside.

**Expected.** Clean N/E 0.1-0.3; T(c) 0.2-0.6, few reach 240; F near
chance (N/E up to about 26), raw AUC T~F under 0.5; R gains as P does.

**Notable, declared before any count.** T(c) median (E+N)/E ≥ 1.5 with
F and R gaining at most half as much (chance-corrected, same E band); R
gaining at least as much as T(c) or P in an E band; ≥ 10% of T(c) under
240 reaching 240, or any F, R(r) or not-confident R doing so; T(a)
median (E+N)/E(C) ≥ 0.75; any pooled chance-corrected AUC ≥ 0.75 or
≤ 0.25; median E/hits outside 0.8-1.1. Shrinks: R2, then R5.
Full text: `docs/brief-30.md`, step 2. **Result:** 0151 and up.
