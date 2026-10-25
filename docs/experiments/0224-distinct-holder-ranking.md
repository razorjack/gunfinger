# 0224: ranking the fullest lists by distinct files or recordings

**Question** (registered in `docs/brief-50.md`, 04:48). Set aside the
same 1% of lists ranked by how many distinct files (R-files) or
recordings (R-clusters: a cluster, an unclustered file, a copy as its
original) hold each, instead of by postings: what does it cost, and is
it steadier under copies?
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
candidate or answer, the second pass's own counts within 1% (smallest
ratio 0.991), no new wrong confident answer, wrong possible answers by
file 7-9 (level 0: 9) at 262 tracks and 24-25 (26) at NAS scale.
R-clusters sets aside the same lists with and without copies at both
scales (by construction). R-files keeps a padded query's first pass
within 3% of level 0, where postings ranking loses up to 28%.
**Against the registration.** Condition (1) fails: the default shows no
U1-U3, so there is nothing to remove. Conditions (2) and (3) hold: no
cost at level 0 and 18.4% of the postings set aside at NAS scale (at
least 15%). Not worth an ADR on this evidence.
**Conclusion.** Counting holders instead of postings changes about 11%
(NAS) to 28% (262) of the lists set aside, gives the first pass slightly
more hits and makes it insensitive to copies, at no cost here. Only
R-files is open to the product as it stands; it is a candidate if a
collection ever shows the failure this session looked for.
