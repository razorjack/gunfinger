# 0291: the band profile under the default matcher: the shared panels and the false candidates

**Question** (brief 83, registered in `8bdcb2e`; arms as in 0290). Does either band profile
make shared material between different recordings stronger: on the shared panels'
development halves (B2) and among calibrate's false candidates (B4)? **Method.** `shared` on
`shared-material-seed-2026-development.json` (12 scenarios of 4 families, 605 results) and
`shared-material-2-seed-2026-development.json` (10 of 3, 440), each index holding the corpus
and 109 NAS files (0290), not the NAS store; `panels.py`, `falsepairs.py`. A query is flagged
when it holds a confident detection of role related or other. The validation halves were not named. Data: `data/0291-band-profile-shared-panels.json`.

| Panel (index) | T | a (4-24) | b (4-16) |
|---|--:|--:|--:|
| 1, flagged queries (everything; played out; source out) | 110; 109; 47 | 97; 97; 38 | 103; 103; 42 |
| 1, related with the played file out: confident; possible | 186; 128 | 142; 195 | 173; 159 |
| 1, strongest related, played file out (hits) | 601 | 536 | 864 |
| 2, flagged queries (everything; played out; source out) | 8; 7; 4 | 4; 3; 3 | 16; 16; 13 |
| 2, related with the played file out: confident; possible | 7; 295 | 4; 306 | 13; 300 |
| 2, strongest related, played file out (hits) | 297 | 281 | 380 |
| 2, files of role other confident beyond T's | | none | Phantom 2018, 10 queries |

On panel 1 the flagged queries fall under a in 6 of 12 scenarios and rise in none; under b
they fall only in Sonar 1 and 2. Panel 2's flags are all in the Phantom Force family: Digital &
Spirit's Phantom 2018 (a remake), and in phantom-force-2 the Phantom Force upload. In
phantom-force-1 Phantom 2018 sits at the rule in every arm (243-280 hits); a flags 3 queries T
does not and drops 12; b flags 45 queries against T's 19, in 3 scenarios, and in phantom-force-5 (whose related file is Meltdown) names Phantom 2018 as an other file 10 times.

**False candidates of 30 hits or more** (calibrate over the 4 sweeps and the Stakka scans, per
query recording and file; T, a, b): the Clockwork remix 100, 94, 111; Aphrodite - Fanfare on
its Dubstyle 61, 92, 98; China Cup on The Nine -, 51, 52 (shared drums, by ear); two tracks of
Aphrodite's APH33 EP (Clean Decibel on From the East) 38, 83, 83; Star Trails on Kemal -
Mechanizm 31, 44, 50; Slipstream on its Gridlok remix -, 34, 33; b adds China Cup on two more
rips of The Nine (50) and SKC - Recharger on its 2007 version (31). Each pair is a version, a
remix or two tracks of one artist; the APH33 pair has not been heard.

**Bars.** B2 holds for a (fewer flagged queries on every panel and index, no new file of role
other) and fails for b (panel 2 on all three indexes, and a new other file). B4 holds for both
on its figures, with the APH33 pair unconfirmed by ear. **Conclusion.** On these 7 families
variant a does not strengthen shared material at the confident level; it moves related
detections from confident to possible. Variant b, the one that gains under damage (0290), strengthens it (strongest related 601 → 864 hits on panel 1). With 0290's B5, neither variant
passes the registration.
