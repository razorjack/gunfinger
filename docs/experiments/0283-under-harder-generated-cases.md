# 0283: `identify --under` on harder generated cases, by construction

**Question.** Does the option print a wrong confident partner when the
mix is processed like a broadcast, a record is key-locked under or over
another, the louder record's pitch rides, three records play at once, or
no record plays under another (brief 80, P1-P3)?

**Commands.** `work/s80/gen.sh` (`mixes --kind drops --case <case>`, 12
mixes per case, one unused seed each; controls `--kind long`, 4 mixes),
`work/s80/broadcast.sh` (plain's renders through `acompressor`,
`alimiter` and MP3 at 96 kbit/s), `work/s80/id.sh` (`identify` at NAS
scale, default and `--under`, `--exclude-from` the sweep's held-out
recordings; key lock `--playback both`), `work/s80/score.sh`
(`tracklist_score.py`), `work/s80/analyse.py`, `work/s80/possible.py`.

**Result.** Partners: the quiet, brief and inside records (three: also
the third), printed by default; with the option.

| Case (seed) | Recordings | Partners: default; option | Gained (right) | Confident under (wrong) | Possible under (matching nothing) |
|---|--:|--:|--:|--:|--:|
| plain (8002) | 88 | 13; 26 of 36 | 13 (13) | 22 (0) | 28 (5) |
| broadcast (8002) | 88 | 8; 16 of 36 | 8 (8) | 15 (0) | 34 (0) |
| key-locked-under (8003) | 82 | 8; 25 of 36 | 17 (17) | 32 (0) | 23 (0) |
| key-locked-over (8004) | 93 | 9; 28 of 36 | 20 (20) | 31 (0) | 31 (2) |
| ride-over (8006) | 94 | 11; 28 of 36 | 17 (17) | 28 (0) | 23 (0) |
| three (8007) | 105 | 16; 27 of 48 | 11 (11) | 17 (0) | 18 (0) |
| controls (8011, long) | 71 | no partner | 0 | 0 | 0 |

- 86 entries gained, all right; 145 confident finds under others, none
  wrong. P1 not hit. One gained entry (key-locked-over) repeats a track
  the default prints nearby through another upload, as in 0274.
- P2 not hit: at most 5 of 28 (18%) possible finds match nothing
  playing; 4 of plain's 5 lie in one double drop.
- P3 not hit: no line of the default is lost and no gap line changes.
- three: the quiet record under two loud ones is never printed (0 of 12,
  3 possible); the third loud record goes from 4 to 8 of 12.
- The default's own wrong confident plays: key-locked-under 2 (the NAS
  Cool Rock detection, and a held-out Konflict - Pendulum named as a
  Konflict remix of another track that shares material, 0038's kind);
  controls 3 (a held-out China Cup named through an upload we judged
  to carry the same title, which the clusters do not join).
- Every drops case plays one panel family (Clockwork) and draws its
  other recordings from the 208 tagged corpus files: 82-105 distinct
  recordings per case, overlapping between cases and with earlier
  sessions' seeds.

**Conclusion.** On these constructions the option's precision holds:
no wrong confident partner. Broadcast processing cuts what both print by
about 40%; key lock and rides cost no precision. Generated audio only.
