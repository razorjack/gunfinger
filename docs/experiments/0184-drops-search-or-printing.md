# 0184: double drops at the detection level: what the search loses, what printing loses

**Question.** Session 39, step 5: on 0182's mixes, which losses come from the search and which from grouping and printing?
**Method.** `gunfinger-eval --panels work/s39/panels --ladder turntable mixes --kind drops --seed 2026|2028 --count 16 --other-root /mnt/atlas/Music/dnb --tagged work/s39/tags-262.json` on the cached renders (0:59-1:00 each; the first runs used the harness's default ladder `both`, and give the same row for every scenario), `tracklist_score.py detections` (`mixes`'s own level per play), and `seconds` on the two reports. 262 tracks only. Each truth play's level agrees with the CLI's report in all 512 indexed plays (confident, possible or below), so the CLI's search and the harness's are the same here.

| Indexed plays, both seeds | n | Search: confident | Possible | Weak | None | Tracklist (0182): printed | Left out |
|---|--:|--:|--:|--:|--:|--:|--:|
| Alone, and every first of two | 352 | 352 | 0 | 0 | 0 | 351 (one merged) | 0 |
| Seconds through (unrelated; pair) | 45 | 45 | 0 | 0 | 0 | 45 | 0 |
| Unrelated drop, second inside | 32 | 25 | 6 | 1 | 0 | 25 | 6 |
| Pair drop, second inside | 19 | 11 | 8 | 0 | 0 | 13 (2 possible) | 6 |
| Quiet (-9 dB) inside | 32 | 4 | 10 | 5 | 13 | 4 | 10 |
| Brief (20-40 s) inside | 32 | 4 | 19 | 7 | 2 | 4 | 19 |

- Wrong confident detections: 0. `seconds` (3 windows): correct 99.0% and 98.6% of 28,324 and 29,136 indexed seconds, mislabelled 150 and 292 s, specificity 100% over 2,325 and 2,248 s without an indexed record; start error median +2.4 and +3.1 s.
- Of the 43 records inside a partner that the search finds as possible, the tracklist prints 2 (the original inside its remix, its possible play not lying inside one passage of the remix's play, so not marked) and leaves out 41; every one stays in the table and JSON.
- The search loses the quiet record most (18 of 32 weak or not found) and the brief one less (9 of 32); printing adds no loss there beyond leaving out what is possible.

**What it shows.** For records at full level, the search finds every partner of a double drop that plays on afterwards, and 36 of 51 that stay inside their partner confidently, 14 as possible and 1 as weak; printing then removes 12 of those 14 by ADR 0019's rule. Grouping costs one merge in 512 plays (0182). Below full level or under 40 s, the search is the main loss. Data: the reports `work/reports/ladder-turntable/mixes-drops-seed-{2026,2028}.json` (not copied; 0182's data holds the levels).
