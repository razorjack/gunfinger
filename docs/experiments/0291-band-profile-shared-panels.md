# 0291: the band profile under the default matcher: the shared panels and the false candidates

**Question** (brief 83, registered in `8bdcb2e`; arms as in 0290). Does either band profile strengthen shared
material: on the shared panels' development halves (B2) and among calibrate's false candidates (B4)? **Method.**
`shared` on `shared-material-seed-2026-development.json` (16 scenarios of 6 families, 605 results) and
`shared-material-2-seed-2026-development.json` (10 of 3, 440), each index holding the corpus and 109 NAS files
(0290), not the NAS store; Scripts `panels.py`, `falsepairs.py`. A query is flagged when it holds a confident
detection of role related or other. The validation halves were not named. Data:
`data/0291-band-profile-shared-panels.json`.

| Panel (index) | T | a (4-24) | b (4-16) |
|---|--:|--:|--:|
| 1, flagged queries (everything; played out; source out) | 110; 109; 47 | 97; 97; 38 | 103; 103; 42 |
| 1, related with the played file out: confident; possible | 186; 128 | 142; 195 | 173; 159 |
| Strongest related, played file out: panel 1; panel 2 (hits) | 601; 297 | 536; 281 | 864; 380 |
| 2, flagged queries (everything; played out; source out) | 8; 7; 4 | 4; 3; 3 | 16; 16; 13 |
| 2, related with the played file out: confident; possible | 7; 295 | 4; 306 | 13; 300 |
| 2, Phantom 2018 confident with role other (results; queries) | 5; 3 | 0; 0 | 15; 5 |

On panel 1 flags occur in 12 scenarios of 4 families; under a they fall in 6 and rise in none, under b they fall
only in Sonar 1 and 2. Panel 2's flags are all in the Phantom Force family: Digital & Spirit's Phantom 2018 (a
remake of the played track), and in phantom-force-2 the Phantom Force upload. In phantom-force-1 Phantom 2018's
confident detections sit near the rule (240-378 hits under T, 243-294 a, 256-380 b); a flags 3 queries there that T
does not and over the panel drops 12; b flags 45 against T's 19 (in phantom-force-5, whose related file is
Meltdown, Phantom 2018 has role other).

**False candidates of 30 hits or more** (calibrate, per query recording and file; T, a, b): the Clockwork remix
100, 94, 111; Aphrodite - Fanfare on its Dubstyle 61, 92, 98; China Cup on The Nine -, 51, 52 (shared drums, by
ear); two tracks of Aphrodite's APH33 EP (Clean Decibel on From the East) 38, 83, 83; Star Trails on Kemal -
Mechanizm 31, 44, 50; Slipstream on its Gridlok remix -, 34, 33; b adds China Cup on two more rips of The Nine (50)
and SKC - Recharger on its 2007 version (31), and raises two Aphrodite pairs T lists (38, 35). Each pair is a
version, a remix or two tracks of one artist.

**Bars.** B2 holds for a (fewer flagged queries on every panel and index, no new (query, file) of role other) and
fails for b (panel 2 on all three indexes, and 10 new results of Phantom 2018 as other). B4 is not shown for
either: its figures hold, but "unrelated" was read only after the calibrate runs (brief 83, 00:00), and the APH33
pair (two titles, unheard) reaches 83 hits. **Conclusion.** On the 5 families with a flag variant a does not
strengthen shared material at the confident level; it moves related detections from confident to possible. Variant
b, the one closer to 0290's damage bar, strengthens it (panel 1's strongest related, played file out, 601 → 864
hits; 562 with everything indexed). With 0290, neither variant passes.
