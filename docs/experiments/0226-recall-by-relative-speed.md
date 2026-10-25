# 0226: recall by relative speed for ladders with 0-5 extra rungs

**Question** (`docs/brief-51.md`, 07:43). How far does each ladder reach,
against the full index and the NAS library's other copies only?
**Method.** Seed 2026's panel, default matcher; L0 (82 rungs) and
`--extra-rungs` 1, 2, 3, 5 (86-102). C1 `sweep --speeds edge` (±8.4 to
±12%; 60 indexed, 20 held-out) at both scales; C2 `sweep --other-rips`,
both speed sets (1,269 queries). Relative speed to the nearest copy
left in (`ladder_edge.py`; R_k by played speed). Cells: confident of the
bin's queries; possible recall in `data/0226-recall-by-relative-speed.json`.

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
recall is gone 0.8-1.2% past the last rung (NAS scale: R_2 9.6%). About
90 C2 queries lie within 0.05% of 8.4% or 9.2% (a copy at the source's
speed), so pair-speed errors pick their bin; G2 is unchanged by it.
**G2** (C2, (8.4, R_k + 0.4]): L0 100/160 → L1 153, 101/239 → L2 227,
101/317 → L3 302, 101/472 → L5 453: +33 to +75 points. Within 8.4% every
L_k recalls 551 of 562, L0 552: Siren Bass's upload at -8% (relative
-8.05%) is fitted at -8.18% under every L_k, at -8.09% under L0, with
214 second-pass hits, not 305 (rule 240); T_k keeps L0's answer.
**Wrong answers.** No new wrong confident answer (C1 at NAS scale: the
same 9 held-out ones under every ladder, 278 hits). Wrong possible: 1,
4, 4, 7, 10 (C1 262); 3, 9, 15, 20, 34 (C1 NAS); 24, 29, 34, 38, 47 (C2,
strongest 100 hits); T_k 1-8, 5-27, 24-39, recall within one of L_k's.
**Against the registration.** G1 holds for every k; G2's gain holds,
its second clause fails by one query for every L_k and holds for T_k.
**Conclusion.** Each rung pair moves the edge 0.4%, at a cost in
possible plays; one rip near -8% loses its confident answer.
