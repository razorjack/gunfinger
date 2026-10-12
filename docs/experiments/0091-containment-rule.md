# 0091: a containment rule for confident plays, measured offline

**Question.** Step 3 of session 15: what would a containment rule remove
and cost? **Rule.** A confident play inside a stronger confident play of
another cluster is reported as shared material. **Method.**
`containment.py` on stored reports, 0090's clusters. Grid: span held
against (`play`: the stronger play's whole span; `passage`: its segments
joined across gaps of 10 s or less) x tolerance 0, 10, 30 s x hit-ratio
bound none, 0.75, 0.5, 0.25; a play's hits are its segments' sum. New
runs: the 7 shared passages of the pairs judged different on 2026-10-12
(T-Minus, Dozer, Pyro), 0076's method (`work/s15/shared.sh`), 6 again
with the played cluster left out. **Informed the grid:** 0076 (ratios
2-24%, VIP 55-60%), the 2003 NAS scan (copies 30-100%); tolerances a
priori. **Only tested it:** the rest. `data/0091-containment-rule.json`.

| Not used to design | Wrong confident | Removed | Right demoted |
|---|--:|--:|--:|
| Scans and leave-outs (262, NAS), sweeps (7,373 confident), recall (786) | 0 | 0 | 0 at all 24 points |
| Robust (262, 3,250 queries) | 0 | 0 | 0 |
| Generated mixes (262, 108 confident plays) | 0 | 0 | `play` 4-35; `passage` 0-10 s: 0; 30 s: 4-30 |
| New passages, played file indexed (7) | 6 | 5 (not Dozer's bootleg: no NAS original) | 0 |
| The same, played cluster left out (6) | 5 | 0 | 0 |

A right play in a development mix would need 110-176 s of tolerance. In
the generated mixes a returning track (gap under 90 s) makes one play
spanning the brief plays between (`passage` avoids it); 30 s swallows
brief plays at a boundary. Every grid point gives the same new-passage row.

| 0076 (design data), `passage`, 10 s | none | 0.75 | 0.5 | 0.25 |
|---|--:|--:|--:|--:|
| Runs keeping a wrong play, of 34 (26 before) | 0 | 1 | 6 | 10 |
| Right plays demoted (The Nine's other rips inside the VIP) | 10 | 10 | 2 | 0 |
| Played file dropped, other rips kept (simulated): right recording lost | 2 | 2 | 0 | 0 |
| Played cluster left out: runs keeping a wrong play, of 8 | 8 | 8 | 8 | 8 |

**Conclusions.** On data not used to design it, the rule meets 6 cases,
removes the 5 it can, costs nothing at `passage`, 0-10 s, and cannot fix
an unindexed recording. A ratio bound trades the VIP for copies: none
loses the right recording when a weaker copy lies inside the VIP; 0.5 or
less keeps the VIP. A new development mix is needed before a decision.
