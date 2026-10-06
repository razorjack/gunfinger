# 0013: dropping the fullest posting lists

**Question.** Hashes that occur all over the library cost the most lookups
and tell assets apart the least (ADR 0007). What do evidence and false
candidates do if their posting lists are emptied?

**Change.** `Index::without_fullest(share)` empties that share of the
non-empty posting lists, fullest first; `--drop-fullest` on `robust` and
`scan` (harness only). `robust --only control --drop-fullest
0.001|0.01|0.05` (seed 2026), 0.01 at 30 synthetic copies (0012), and the
development scan at 0.01. Data: `data/0013-drop-fullest-summary.json`.

**Results** (80 indexed and 20 held-out excerpts; "wrong" leaves out the
Clockwork excerpt, whose remix is the last column).

| Dropped | Own hits median, min | Wrong median, max | Held-out max | Remix |
|---------|---------------------:|------------------:|-------------:|------:|
| none (30 copies) | 858, 654 | 7 (10), 29 (29) | 24 (24) | 93 |
| 0.1% | 823, 648 | 7, 28 | 24 | 93 |
| 1% (30 copies) | 715 (691), 611 (579) | 0 (8), 11 (25) | 11 (15) | 65 (58) |
| 5% | 551, 399 | 0, 9 | 9 | 42 |

Every row: 80/80 confident, none wrong. Development mix, 1% dropped: 11/11,
0 wrong, own-track hits 74-89% of before, the remix's alignments 91 → 73
and 45 → 25 hits.

Lookups (`hash-cost`, experiment 0014; exact counts on the control
excerpts): the fullest 1% of lists hold 16% of postings but 62% of what a
query scans (4,835 postings per second of query → 1,853). Wall time at
262 tracks did not move (development scan, three interleaved rounds on a
loaded machine: median 25.6 s without, 32.9 s with; fastest 25.4 and
25.2 s): lookups are a small part of the search at this size.

**Conclusions.** (1) Common hashes carry most lookups and most chance
evidence: dropping the fullest 1% costs 17% of own hits, 62% of lookups and
over half of the strongest chance alignment (29 → 11). (2) Shared material
leans on them less (the remix loses 30%). (3) 1% is the knee; 5% costs a
third of own hits for little more. (4) A candidate default for a large
library, where lookups dominate; full protocol in experiment 0017.
