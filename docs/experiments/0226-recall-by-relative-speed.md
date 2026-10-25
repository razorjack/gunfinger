# 0226: recall by relative speed for ladders with 0-5 extra rungs

**Question** (`docs/brief-51.md`, 07:50). How far does each ladder reach
against the full index and against only the other copies the NAS library
holds, and what does it add in wrong answers? **Method.** Seed 2026's sweep panel, default matcher, `--extra-rungs k`
for L0 (82 rungs), L1, L2, L3, L5 (86-102). C1 `sweep --speeds edge`
(±8.4 to ±12%; 60 indexed, 20 held-out excerpts) at both scales; C2
`sweep --other-rips`, sweep and edge speeds pooled (1,269 queries).
`ladder_edge.py recall`, relative speed to the nearest copy left in;
confident (/ possible) per bin; `data/0226-recall-by-relative-speed.json`.

| \|Relative\| | C1 262 L0 | L1 | L2 | L3 | L5 | C2 NAS L0 | L1 | L2 | L3 | L5 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 8-8.4% | 8/8 | 8 | 8 | 8 | 8 | 81/83 | 80 | 80 | 80 | 80 |
| 8.4-8.8 | 117/120 | 120 | 120 | 120 | 120 | 72/81 | 78 | 78 | 78 | 78 |
| 8.8-9.2 | 100 | 120 | 120 | 120 | 120 | 28/79 | 75 | 76 | 77 | 77 |
| 9.2-9.6 | 0 | 104 | 119 | 120 | 120 | 1/79 | 27 | 73 | 77 | 77 |
| 9.6-10.0 | 0 | 0 | 103 | 120 | 120 | 0/78 | 0 | 27 | 70 | 76 |
| 10.0-10.4 | 0 | 0 | 0 | 101 | 120 | 0/78 | 0 | 0 | 25 | 76 |
| 10.4-10.8 | 0 | 0 | 0 | 0 | 120 | 0/77 | 0 | 0 | 0 | 69 |
| 10.8-11.2 | 0 | 0 | 0 | 0 | 97 | 0/78 | 0 | 0 | 0 | 27 |
| 11.2-12.4 | 0 | 0 | 0 | 0 | 0 | 0/124 | 0 | 0 | 0 | 0 |

Reach R_k (C1 262, ≥ 57/60 at both signs): 8.4, 8.8, 9.2, 9.6, 10.4%;
recall is gone 0.8-1.2% past the last rung; C1 at NAS scale steps alike.
**G2** (C2, [8.4, R_k + 0.4)): L0 against L_k 100/160 → 153 (L1), 101/239
→ 227 (L2), 101/317 → 302 (L3), 101/472 → 453 (L5): +33 to +75 points.
Within 8.4% every L_k recalls 551 of 562, L0 552: Siren Bass's upload at
-8% (relative -8.05%) is fitted at -8.18%, not -8.09%, once the -8.4%
rung exists; 214 second-pass hits, not 305 (rule 240). T_k keeps it.
**Wrong answers.** No new wrong confident answer: none at 262 tracks or
in C2; C1 at NAS scale has the same 9 under every ladder (held-out
queries, strongest 278 hits). Wrong possible answers grow: 1, 4, 4, 7, 10
(C1 262); 3, 9, 15, 20, 34 (C1 NAS); 24, 29, 34, 38, 47 (C2; strongest
100 hits throughout); T_k 1-8, 5-27, 24-39, its recall within one query
of L_k's. **Against the registration.** G1 holds for every k; G2's gain
holds, its second clause fails by one query for every L_k, holds for T_k.
**Conclusion.** Each extra rung pair moves the edge 0.4% at both scales
and against real copies, at a cost in possible plays; one rip near -8%
loses its confident answer.
