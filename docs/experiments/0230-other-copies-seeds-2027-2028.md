# 0230: other copies only, sweep panels of seeds 2027 and 2028

**Question** (`docs/brief-51.md`, 07:43, C2's "if time" and 09:28). Do
0226's other-copies results hold on two more panels: the gain beyond
8.4%, no loss within it, no new wrong confident answer?
**Method.** As 0226's C2 (`sweep --seed S --other-rips`, the sweep's 9
speeds and the edge speeds pooled), seeds 2027 (1,458 queries) and 2028
(1,296), ladders L0, L1, L3, L5 (L1 added after 0226-0229, so
exploratory), queue 3d, 09:28-10:08, every run exit 0;
`data/0230-other-copies-seeds-2027-2028.json`.

| Seed, ladder | [8.4%, R_k + 0.4%) | Within 8.4% | Wrong confident (new) | Wrong possible |
|---|--:|--:|--:|--:|
| 2027 L0 | | 646/708 | 15 | 34 |
| L1 (to 9.2%) | 88 → 146 of 164 | 646 | 15 (0) | 43 |
| L3 (to 10.0%) | 89 → 290 of 318 | 646 | 16 (1) | 51 |
| L5 (to 10.8%) | 89 → 434 of 468 | 647 | 20 (5) | 62 |
| 2028 L0 | | 542/590 | 0 | 24 |
| L1 | 101 → 144 of 161 | 542 | 0 | 28 |
| L3 | 101 → 281 of 312 | 542 | 0 | 36 |
| L5 | 101 → 414 of 458 | 543 | 0 | 44 |

- Gains: +35 and +27 points (L1), +63 and +58 (L3), +74 and +68 (L5);
  L1 at seed 2028 falls short of G2's 30 points.
- Within 8.4% no wider ladder loses a query on either seed (L5 gains
  one): seed 2026's Siren Bass query is the only loss in 2,031 queries.
- Seed 2027's wrong confident answers are all China Cup against its
  Prototype upload on YouTube, a different version by ear (verdicts);
  L0 has 15 of them; L3 adds one more (a corpus rip played at -11.2%),
  L5 four more (two corpus files at +8.0% and +8.4%). Strongest wrong
  261 hits (L5 273). T1: no new wrong; T3 and T5: the same 1 as L3.
- T_k's recall is within 3 queries of L_k's; its wrong possible answers
  are 14-30% fewer.
**Conclusion.** The gain beyond 8.4% and the safety within it hold on
two more panels; one rung pair's gain is smaller there (27-35 points),
and three or more pairs reach a second judged-different version at more
speeds. T1 still adds no wrong confident answer.
