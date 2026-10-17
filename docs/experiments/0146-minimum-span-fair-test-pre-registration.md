# 0146: the minimum span's fair test, brief passages at every grid offset: pre-registration

**Question.** Session 29 (0142's conclusion): with each brief shared
passage at every offset of the 10 s window grid, do the variants of 0136
name more wrong files confidently than today's rule? What do they name
among session 28's brief plays at NAS scale?

**Rules.** Today's (240 hits in 3 windows), A(S) and B(S) for S = 10, 15,
20 s, each replacing only `windows >= 3` (0136), recomputed offline once
today's rule reproduces every stored level.

**Shared queries.** Both shared panels' development halves (16 + 10
scenarios): each `alone` cut (10, 20, 30 s, no longer than the passage;
46 + 29) after o = 0, 1, ..., 9 s of digital silence: 750 queries in
0142's indexes, 188 passages (cut, index), 1,880 results. Variant build
`583bfe3` (`S29_OFFSETS_ONLY`), corpus plus NAS store at 40,126 records.

**Criterion 1, fair form (F), both required.** F1: on each panel, the
variant's wrong answers ((result, file) pairs named confidently, not the
played recording) summed over passages and offsets are no more than
today's. F2: no passage is wrong under the variant at any offset while
today's rule is wrong at none of its ten. A wrong possible answer
turning confident counts. Beside, brief 27's criterion 1 as written
(C27): no (result, file) confident and wrong that is not so today.

**Measures.** Wrong answers per rule and offset; per passage, the
offsets where today's rule is wrong (its grid dependence) and each
variant's; right confident results; level changes. Check: the 3, 5 and
7 s offsets of the 20 s cuts are 0142's queries (renders, detections).

**Recall brief plays (NAS).** Session 28's (b) and (a)+(b) queries
rebuilt by the variant build, its detections first compared with session
28's; per rule: confident recall at 15 and 20 s, by where the play starts
in a window, wrong answers confident and possible, held-out and
unobservable queries confident. Offline: 0150's 7 Panako-only plays.

**Expected.** Today's rule wrong at most offsets of fire-2's, fire-6's
and pyro-2's 20 s cuts and at all of their 30 s cuts, never on a 10 s
cut; F2 may fail for B(10) on 10 s cuts. Full text: `docs/brief-29.md`.
**Result:** 0147 and up.
