# 0282: the fourth drop rule of `identify --under`, within one passage

**Question.** If lining up drops a play found under an entry only when
the two plays are one passage (overlap, or at most 10 s apart; the tag
test keeps 90 s), are the two partners 0274 lost kept, with no wrong
line, no lost entry and no repeated entry (brief 79, F1-F3)?

**Commands.** `work/s79/g.sh`: `identify` at NAS scale, `--exclude-from`
as 0274, `--save-dir`, `show`: default, *before* (`s79-start --under`),
*after* (`s79-run --under`), on 0274's renders (seeds 2050, 2053) and
fresh seeds 2091, 2092 (`work/s79/gen.sh`, 16 mixes each).
`work/s79/score.sh` (`tracklist_score.py`), `compare.py`. D: 0281's runs.

**Result.** Plays and gaps are identical across each mix's three runs;
no default line is lost.

| Input | Released by the fix | Entries gained, before; after | Confident finds, before; after | Possible matching no track |
|---|---|--:|--:|--:|
| 2050, 2053 | 5 plays: Blackout (3 uploads), Splinter (2) | 38; 39 | 56; 58 | 3; 3 |
| 2091 | 3: Firewire (2, confident), Blindside (possible) | 21; 22 | 34; 36 | 0; 0 |
| 2092 | 1: Spectre (confident) | 22; 23 | 36; 37 | 1; 1 |
| D, both scales | none (JSON identical) | 0; 0 | 0; 0 | - |

- Every gained entry and confident find is right. F1 met: 2050 `mix-14`
  prints `9:49 Chris SU & SKC - Blackout (...) (under 6.)`; Splinter is
  in 2053 `mix-00`'s `under` as possible. F2 met. F3: no repeated entry
  gained (0274's Afterburner repeat is there before and after).
- The released plays on 2050 and 2053 are exactly session 73's recorded
  fourth-rule fates of Blackout and Splinter; the rule's other drops
  there (8 and 18 of 11 and 20) stay dropped.
- Partners printed on 2050 and 2053 (of 32 each; default, before,
  after): quiet 2, 18, 19; brief 6, 18, 18; unrelated inside 20, 29, 29.
  On 2091 and 2092: quiet 0, 16, 17; brief 5, 20, 20; unrelated 16, 28, 29.
- Wall time for 16 mixes (not A B B A): 2050 2:29, 6:39, 4:05; 2053
  2:25, 6:19, 3:56; 2091 2:29, 6:30, 4:00; 2092 2:30, 6:37, 4:06.

**Conclusion.** The fix keeps both lost partners and, on the fresh seeds,
releases 4 more plays, all right partners (2 more entries), with no wrong
line, lost entry or repeat: 64 drops mixes on four seeds, all drawn from
the 262-track corpus.
