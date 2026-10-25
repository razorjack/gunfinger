# 0222: exact copies at 262 tracks: the lists set aside, candidates, answers

**Question** (registered in `docs/brief-50.md`, 04:48). Do exact copies
of some records move a track's own hashes into the fullest 1% of lists
the first pass sets aside, so that it stops being a candidate?
**Method.** `sweep --trace-candidates` (seed 2026; 540 indexed and 180
held-out queries; 212 indexed files) with `--exact-copies K --copied`:
*panel* (the 67 files of the queries' own clusters) and *share* (a
seeded 26 of 265 files), K = 1, 3, 9 and 99 (stress, panel only);
`filter_padding.py compare` against the traced level-0 run (answers
equal to the default report's); 0:52-1:03 a run (K = 99: 2:48, 1.5 GB);
data `data/0222-padding-262.json`.

| Selection, K | Cut (postings) | Lists in/out | Set aside: copied, rest | Own first-pass hits, min; median | Confident |
|---|--:|--:|--:|--:|--:|
| level 0 | 101 | (5,713 lists) | 16.0% | 1; 1 | 540/540 |
| panel 1 | 134 | 470/514 | 17.1%, 15.3% | 0.903; 0.989 | 540/540 |
| panel 3 | 201 | 1,004/1,028 | 17.9%, 14.3% | 0.864; 0.980 | 540/540 |
| panel 9 | 410 | 1,528/1,519 | 18.4%, 13.1% | 0.768; 0.969 | 540/540 |
| panel 99 | 3,620 | 1,906/1,897 | 18.5%, 12.0% | 0.717; 0.967 | 540/540 |
| share 1 | 111 | 295/297 | 16.9%, 15.9% | 0.969; 1.001 | 540/540 |
| share 3 | 132 | 838/829 | 19.2%, 15.2% | 0.956; 1.007 | 540/540 |
| share 9 | 202 | 1,710/1,729 | 21.7%, 13.7% | 0.899; 1.021 | 540/540 |

Own first-pass hits: the best own-cluster candidate's, as a ratio of
level 0. At every level: no query loses its candidate (U2) or its
confident answer (U1), no wrong confident answer and no held-out query
confident (U3); the second pass's own hits are unchanged (median ratio
1.000); no ratio is below 0.5. Lowest: Subwave - Digital Symphony (617
first-pass hits at level 0, 474 at panel 9, 412-451 at 99). Wrong
possible answers by distinct file: 9 at level 0, 7-9 padded (by
detection up to 80 at panel 9, each copy counted). Uncopied queries of
the share selection gain first-pass hits (median 1.023 at K = 9).
**Against the registration.** Stable at every realistic level, and the
stress level does not break it. Expected as declared, except the cut,
which rises with K (4.1 times at K = 9) while about 5,700 lists stay set
aside, so 27% of the set changes at panel 9 and 33% at 99.
**Conclusion.** At 262 tracks, nine copies of the queries' 67 files move
a quarter of the set aside towards them and cost a query's first pass up
to 23% of its hits; candidates and answers do not move.
