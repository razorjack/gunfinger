# 0223: exact copies at NAS scale: the lists set aside, candidates, answers

**Question** (registered in `docs/brief-50.md`, 04:48). As 0222, at NAS
scale: 40,580 files; the fullest 1% of lists start at 9,913 postings.
**Method.** `sweep --trace-candidates` (seed 2026) and its other-rips form
(423 queries of 47 sources; each source and its identical copies left
out), with *panel* (219 files of the queries' clusters, 157 indexed) at
K = 9 and *share* (4,070 files, a seeded tenth) at K = 6 and 3: K = 9
stopped at the 65,536-asset limit (brief, 05:41). 2:52-9:54 a run, at
most 7.6 GiB; data `data/0223-padding-nas.json`.

| Queries, selection, K | Cut | Lists in/out | Set aside: copied, rest | Own first pass, min; median | Confident |
|---|--:|--:|--:|--:|--:|
| sweep, level 0 | 9,913 | (10,863) | 18.7% | 1; 1 | 540/540 |
| sweep, panel 9 | 10,317 | 152/155 | 20.4%, 18.7% | 0.954; 0.998 | 540/540 |
| sweep, share 3 | 12,857 | 131/131 | 18.7%, 18.7% | 0.990; 1.000 | 540/540 |
| sweep, share 6 | 15,821 | 216/215 | 18.7%, 18.7% | 0.987; 1.000 | 540/540 |
| other rips, level 0 | 9,899 | (10,862) | 18.7% | 1; 1 | 408/423 |
| other rips, panel 9 | 10,136 | 96/95 | 20.2%, 18.7% | 0.821; 0.999 | 408/423 |
| other rips, share 6 | 15,793 | 218/217 | 18.7%, 18.7% | 0.821; 1.000 | 408/423 |

No U1, U2 or U3 in any run: no query loses its candidate or its
confident answer; the 7 wrong confident answers are the held-out
Aphrodite - Cool Rock Stuff against the Mobb Deep vs Urban Takeover
blend, at level 0 with the same hits. Wrong possible answers by file:
26 → 26-27 (sweep), 21 → 21-22 (other rips). Candidates per query
(median), by detection: 3 → 22 (panel) and 3 → 20 (other rips), each
own copy a candidate; by distinct wrong file 1 → 1. One query moves:
Siren Bass at +3% (other rips) keeps a 2-window first-pass chain of 193
hits instead of 3 windows of 235 in both selections, though only the
panel copies its rip: lists moved by other files' copies suffice. The
second pass measures the chain's span: 298 hits instead of 363 (still
confident). Every other own second-pass count is 0.974-1.012 of level 0.
**Against the registration.** Stable. The panel moves few lists
(0.9-1.4%), as declared; the share selection, against the declaration,
raises the cut by 60% but moves 1.2-2.0%: each of the fullest lists is
held by many files, and about a tenth of them are copied.
**Conclusion.** At 40,580 files the set aside follows the library's
proportions; copies shift it by a few percent of its lists, and through
the chain's span that can lower a second-pass count (Siren Bass, 18%).
