# 0148: the minimum span's fair test: shared passages at every offset of the window grid

**Question.** 0146: with every shared passage of 30 s or less after 0-9 s
of silence, does a variant name more wrong files confidently (F1), or one
in a passage today's rule never gets wrong (F2)?

**Method.** As pre-registered (`b61c6b5`), 07:22-07:40, exit 0; 1,120 +
760 results; `span_offsets.py`, 0 reproduction problems. The 3, 5, 7 s
offsets equal 0142's (78 renders byte for byte, 195 results). Data
`data/0148-minimum-span-shared-passages-every-offset.json`.

| First panel (second: 0 wrong under every rule) | Today | A(10) | A(15) | A(20) | B(10) | B(15) | B(20) |
|---|--:|--:|--:|--:|--:|--:|--:|
| Wrong answers, 10 offsets (offset 0) | 168 (11) | 177 (17) | 174 (17) | 110 (11) | 201 (17) | 174 (17) | 166 (17) |
| F1; F2 (passages failing) | | no; yes | no; yes | yes; yes | no; no (4) | no; yes | yes; yes |
| Brief 27's criterion 1 (new wrong) | | no (9) | no (6) | yes | no (33) | no (6) | no (6) |
| Right confident, first; second panel | 388; 287 | 419; 310 | 409; 300 | 200; 150 | 576; 393 | 409; 310 | 399; 289 |

- Grid dependence (today): the other Fire versions and Time Is Broken
  are confident in fire-2's and fire-6's 20 s cuts at 9 of 10 offsets,
  pyro-2's at 8, and every 30 s cut of fire-2, fire-6, pyro-1, pyro-2 at
  10 of 10; pyro-1's 20 s cut (Ignorance) at 2 (1 and 5 s). Never on a
  10 s cut. All 168 are in these four scenarios.
- New wrong answers. A(15), B(15), B(20): only offset 0 of fire-2's,
  fire-6's and pyro-2's 20 s cuts (2 indexes each; 697, 795, 285 hits,
  spans 19.6-19.8 s), where a 20 s passage holds exactly 2 windows; at
  offsets 1-9 A(15) and B(15) are wrong exactly where today's rule is.
  A(10): the same 6 and pyro-1 after 3 s (Ignorance, 250 hits, span
  12.5 s, 3 indexes). B(10): those 9 and 24 on 10 s cuts (fire-2 at 5
  offsets, fire-6 at 7, 298-358 hits, 10 supported s), which today's rule
  can never make confident (F2 fails).
- A(20) meets F by dropping wrong answers (58 to possible) and half the
  right ones; B(20) drops pyro-1's 20 s cut (1, 5 s) and pyro-2's at 3 s.

**Against 0146.** As expected: today's rule wrong at most offsets of the
20 s cuts and every offset of the 30 s cuts, never on 10 s; F2 fails for
B(10); A(15), B(15), A(10) close to today's (+6, +6, +9). Not predicted:
pyro-1's 30 s cut wrong at all 10 offsets. **Conclusion.** No variant meets F and criterion 2 together: A(20)
and B(20) meet F but not criterion 2 (0138); A(10), A(15), B(15) fail F1
by 9, 6, 6 wrong answers of 168, all but A(10)'s 3 at offset 0.
