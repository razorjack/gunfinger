# 0210: the pitch-ride variant on double drops, and their tracklists

**Question.** Session 45, runs D1-D3 and T (brief 45, 21:30): with two
records at once, often at different speeds, does `--apart-in-speed 0.006`
keep a wrong file's extra segment, or change the printed tracklist?

**Method.** Session 39's drop mixes (seeds 2026 and 2028, 16 each, its
renders, copied). D1: `mixes --kind drops` at 262 tracks, start build,
default and variant (ladder `both`; also `turntable`, the CLI's). D2, D3:
`identify` from the start build and from a diagnostic build whose
`Matcher::Fitted` uses the variant (never committed; brief 21:18-22:15)
at 262 tracks (store-only copies) and at NAS scale (the NAS store's own
records less the held-out recordings' files, no NAS listing), scored by
`tracklist_score.py`; `variant_compare.py answers` and `tracklists`.

| Default -> variant | 2026 | 2028 |
|---|--:|--:|
| D1: right indexed plays (S3) | 221 -> 221 | 220 -> 220 |
| D1: wrong confident detections; added by mix and file (S2) | 0 -> 0; 0 | 0 -> 0; 0 |
| D1: detections added (all weak, at most); removed | 11 (28 hits); 0 | 9 (44 hits); 0 |
| D2, D3: CLI reports identical between builds | 16 + 16 of 32 | 16 + 16 of 32 |
| D2: entries printed; wrong confident plays | 222 -> 222; 0 | 223 -> 223; 0 |
| D3: entries printed; wrong confident plays | 230 -> 230; 0 | 230 -> 230; 2 -> 2 |

- 19 of D1's 20 added detections are of the Clockwork pair's file not
  playing while its related version plays (7-44 hits, 2-3 windows), each
  0.61-1.51% in speed from a stronger alignment of the same file; none
  lies across a ride. On `turntable` the variant adds 11 and 7 (at most
  28 hits), all below what a CLI report holds.
- T holds: no entry, outcome or level changes. D3's two wrong confident
  plays are session 39's Kemal - Animation remix, in both builds.
- The diagnostic CLI is the variant: each of its 638 segments at 262
  tracks is a detection of the turntable harness report with its matcher,
  and on the Stakka mix it keeps Star Trails' 793-877 s side (315 hits),
  which the default drops. The default's plays equal session 39's.

**Conclusion.** S2, S3 and T hold on both seeds at both scales. On
these drops the variant changes only weak detections, but those show
that a related version, without any ride, has alignments just over 0.6%
from its stronger one. Data `data/0210-drops-apart-in-speed.json`.
