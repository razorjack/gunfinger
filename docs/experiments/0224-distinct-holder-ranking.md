# 0224: ranking the fullest lists by distinct files or recordings

**Question** (registered in `docs/brief-50.md`, 04:48). Set aside 1% of
the lists ranked by distinct files (R-files) or recordings (R-clusters:
a cluster, an unclustered file, a copy as its original) holding each,
not by postings: what does it cost, is it steadier under copies?
**Method.** `sweep --trace-candidates --rank-fullest-by files|clusters`
(`Index::skipping_most_held`), at level 0 and with copies, compared
with the default's traced level-0 run (0222, 0223); 0:53-3:13 a run at
262 tracks, 5:02-10:22 at NAS scale. Data
`data/0224-distinct-holder-ranking.json`.

| Scale, padding | Ranking | Set aside | Lists unlike the default's | Own first pass, min; median | Confident |
|---|---|--:|--:|--:|--:|
| 262, level 0 | postings (default) | 16.0% | 0 | 1; 1 | 540/540 |
| 262, level 0 | files; clusters | 14.4%; 14.2% | 1,518; 1,586 | 0.978; 1.026 / 0.983; 1.027 | 540/540 |
| 262, panel 9 | postings; files; clusters | 17.4%; 14.9%; 14.2% | 1,519; 1,699; 1,586 | 0.768 / 0.972 / 0.983 | 540/540 |
| 262, panel 99 | postings; files; clusters | 18.4%; 15.1%; 14.2% | 1,897; 1,766; 1,586 | 0.717 / 0.972 / 0.983 | 540/540 |
| NAS, level 0 | files; clusters | 18.4%; 18.4% | 1,193; 1,195 | 0.980; 1.011 / 0.979; 1.011 | 540/540 |
| NAS, panel 9 | postings; files; clusters | 18.8%; 18.5%; 18.5% | 155; 1,187; 1,195 | 0.954 / 0.981 / 0.979 | 540/540 |
| NAS, share 6 | postings; files; clusters | 18.7%; 18.4%; 18.4% | 215; 1,203; 1,195 | 0.987 / 0.983 / 0.979 | 540/540 |

"Unlike": default level-0 lists no longer set aside; minima are
ratios of the default's level-0 first-pass hits. Every run: no lost
candidate or answer, the second pass's own counts 0.991-1.012 of level
0, no new wrong confident answer, wrong possible answers by file 7-9
(9) at 262 tracks, 24-25 (26) at NAS scale. With ties at the cut kept,
the rankings set aside fewer lists at 262 tracks (5,407-5,710 against
5,713-5,722) and fewer postings, so part of their extra first-pass hits
is postings scanned. R-clusters sets aside the same lists with and
without copies (by construction); R-files' lists move under copies
(1,518 → 1,766 unlike the default's) but keep a padded query's first
pass within 3% of level 0, where postings ranking loses up to 28%.
**Against the registration.** (1) fails: the default shows no U1-U3.
(2) holds, read as no new wrong confident answer (the 7 at NAS scale
are the held-out Cool Rock Stuff, as with the default); (3) holds
(18.4% at NAS scale). Not worth an ADR on this evidence.
**Conclusion.** Counting holders changes 11-28% of the lists set aside
and keeps the first pass within 3% under copies, at no measured cost;
only R-files is open to the product as it stands.
