# 0140: the shared panels' changed outcomes, by query length

**Question.** Session 27, reading 0138: where on the shared panels do
the variants change outcomes, and are the new wrong answers new titles?

**Method.** Offline, from V's shared reports and 0138's counts
(`scripts/analysis/span_shared_lengths.py`). Today's rule counts 10 s
windows from the query's start, so a query file of 20 s or less holds
at most 2 windows and no detection in it can be confident, whatever its
hits. Data: `data/0140-shared-{1,2}-lengths.json`.

| | Shared 1 | Shared 2 |
|---|--:|--:|
| Query results in files of at most 20 s (of all) | 86 (605) | 56 (440) |
| ... holding a detection of 240 hits or more | 29 | 17 |
| Confident today in those files | 0 | 0 |
| A(10): changed outcomes; all in those files | 29 | 17 |
| ... right confident; wrong confident | 23; 6 | 17; 0 |

- Every outcome changed by A(10), A(15), B(10) and B(15) on either
  panel is in a file of at most 20 s (37 of A(10)'s 46 in 20 s `alone`
  files, the rest `alone`, `weak` and `before` files of 14-19 s). B(20)
  also demotes one 60 s loop on shared 1 to possible only.
- The 6 wrong ones are the 20 s `alone` queries of fire-2, fire-6 and
  pyro-2 in two indexes. In each, the wrong file is one today's rule
  already names confidently in 12-13 other queries of the same scenario
  and index (the 30 s `alone`, the loops, `before`, `beyond`). The new
  wrong detection in fire-2's 240 s loop is a third segment of a file
  with two confident, wrong segments in that query.
- The panel's 20 s files start at query time 0, so today's 2 windows
  are fixed by the file's length. In a mix the same 20 s sits at any
  offset; on the grid, 146 of 160 20 s plays (16 brief plays at 10
  window offsets) are confident today (0138). That today's rule would
  name these versions confidently at most offsets is an inference (642
  and 703 hits over 19.8 s); not tested (handoff).

**Conclusion.** Criterion 1 fails as written, and the result stands.
On the panels' files of 20 s or less today's rule cannot be confident,
so a variant's changes there are measured against a level the file's
length fixes, not against the same evidence placed in a mix.
