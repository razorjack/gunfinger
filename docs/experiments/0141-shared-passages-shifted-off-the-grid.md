# 0141: the shared panels' short passages shifted off the window grid

**Question.** Session 27, pre-registered at 02:30 (`docs/brief-27.md`):
with each short passage placed off the window grid, does today's rule
give the answers the variants give on the panels' 20 s files (0140)?

**Method.** Variant worktree with `S27_SHIFTED_ONLY` (`25e26fc` on
`s27-span`, never merged), `--work work/s27/vshift`, both shared panels'
development halves: up to 20 s of each passage after 3, 5 and 7 s of the
played file's own material (33 and 30 queries, the 11 and 10 scenarios
whose passage starts 3 s or more in), the same indexes, 02:30-02:38.
`span_shifted.py` pairs each `alone` file of 20 s or less with its
shifted queries in the same index; `span_rule.py shared` counts the
shifted queries under every rule (0 problems). Data `data/0141-*.json`.

| Outcome on the short file equals today's rule, shifted | Shared 1 | Shared 2 |
|---|--:|--:|
| Today's rule; A(20) | 29 of 78 | 29 of 78 |
| A(10); B(10) | 74; 74 | 70; 70 |
| A(15); B(15); B(20) | 75; 75; 74 | 71; 70; 68 |

- 0140's six wrong answers (fire-2, fire-6, pyro-2, two indexes each):
  today's rule names the same wrong file confidently in all 3 shifted
  queries of each, 18 of 18. With the played recording left out, 9 of
  shared 1's 27 shifted results are wrong confident today; shared 2's 0.
- Where A(10) on the short file and today's rule shifted differ: 5
  pairs are 14-15 s passages that after 3 or 5 s still end inside the
  second window (2 windows, possible today); 4 gain from the 3-7 s of
  the played file's own material ahead (`source left out`, 268-426
  hits, right confident today); 3 differ in a detection near 60 hits.
- On the shifted queries no variant adds a wrong confident detection;
  A(10) to B(15) make 4 and 2 more right confident (the 14-15 s
  passages). Not covered: 12 of A(10)'s 29 changes on shared 1, 2 of 17
  on shared 2 (passages starting within 3 s, `before`, `weak` files).

**Conclusion.** Prediction held. Moved 3, 5 or 7 s off the grid, the
same 20 s of shared material gets from today's rule what the variants
give it at the grid's origin: the same three wrong titles and most of
the gains. At these offsets, the variants' failures of criterion 1 are
answers today's rule already gives. Criterion 1 still fails as written.
