# 0283: `identify --under` on harder generated cases, by construction

**Question.** Does the option print a wrong confident partner under
broadcast processing, key lock under or over, a ride of the louder
record, three records at once, or no partner at all (brief 80, P1-P3)?

**Commands.** `work/s80/gen.sh` (`mixes --kind drops --case <case>`, 12
mixes, one unused seed per case; controls `--kind long`, 4 mixes),
`broadcast.sh` (plain's renders through `acompressor`, `alimiter`, MP3
96 kbit/s), `id.sh` (`identify` at NAS scale, default and `--under`,
`--exclude-from` the sweep's held-out recordings; key lock with
`--playback both`), `score.sh` (`tracklist_score.py`), `analyse.py`.

**Result.** Partners: quiet, brief, inside (and three's third) records.

| Case (seed) | Recordings | Partners: default; option | Gained (right) | Confident under (wrong) | Possible under (matching nothing) |
|---|--:|--:|--:|--:|--:|
| plain (8002) | 88 | 13; 26 of 36 | 13 (13) | 22 (0) | 28 (5) |
| broadcast (8002) | 88 | 8; 16 of 36 | 8 (8) | 15 (0) | 34 (0) |
| key-locked-under (8003) | 82 | 8; 25 of 36 | 17 (17) | 32 (0) | 23 (0) |
| key-locked-over (8004) | 93 | 9; 28 of 36 | 20 (20) | 31 (0) | 31 (2) |
| ride-over (8006) | 94 | 11; 28 of 36 | 17 (17) | 28 (0) | 23 (0) |
| three (8007) | 105 | 16; 27 of 48 | 11 (11) | 17 (0) | 18 (0) |
| controls (8011) | 71 | none | 0 | 0 | 0 |

- P1 not hit: 86 entries gained, all right; 145 confident finds, none
  wrong (one gained entry repeats a nearby track through another upload).
  P2 not hit: at most 5 of 28 possible finds (18%) match nothing playing.
  P3 not hit: no default line lost, no gap line changed.
- three: the quiet record under two loud ones is never printed (0 of 12,
  3 possible); the third loud record goes from 4 to 8 of 12.
- Default's own wrong plays: key-locked-under 2 (Cool Rock; a held-out
  Konflict - Pendulum named as a remix sharing its material, 0038), controls
  3 (a held-out China Cup through an upload of its title, outside its cluster).
- One panel family (Clockwork) per drops case; 82-105 recordings each, of
  the 208 tagged corpus files, overlapping between cases and earlier seeds.

**Conclusion.** No wrong confident partner on these constructions.
Broadcast processing cuts what both print by about 40%; key lock and
rides cost no precision. Generated audio only.
