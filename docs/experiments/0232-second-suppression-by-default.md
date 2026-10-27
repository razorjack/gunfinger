# 0232: the second suppression at another speed by default (ADR 0021)

**Question** (brief 53, steps 3-4): with `SpansAtSpeed(0.006)` as the
default, what changes in the standard evaluation, the development scans
and the printed tracklists? **Method.** `regress session-32-linux` and
`session-52-linux-nas` with `s53-run` (= `s53-final`); the 2003 and
Nocny scans at NAS scale and `identify --store-only` on the three
development mixes at both scales, start and final builds.
`overlap_changes.py`; `data/0232-second-suppression-by-default.json`.

| Before -> after | 262 tracks | NAS scale |
|---|--:|--:|
| Sweep 2026: queries whose detections changed | 0 of 720 | 4 (one weak each, 6-15 hits) |
| Stakka scan; leave-outs 3, 11: identified, wrong | 11, 8, 0; 0, unchanged | 11, 8, 0; 0, unchanged |
| Stakka scan: confident added; weak added, removed | 1; 3, 1 | 3; 3, 3 |
| 2003 mix; Nocny: identified, wrong; confident added; weak added | | 12/12, 17/17, 0; 1, 0; 12, 5 |
| Calibrate, unchanged: weakest identifying, strongest false, false accepted; false candidates at 30 hits+ | 690, 100, 0; 18 | 704, 282, 7; 95 |
| CLI on the 3 mixes: printed tracklists and `show`; JSON (`lines-chains-v2`) | identical; 1 play gains a segment | identical; 3 plays gain one |

- Confident added: Star Trails' 793.4-877.3 s side, 315 hits, 1.34%
  slower than the kept 585.1-803.7 s side (at NAS scale on all three
  files of its recording, 306-315 hits, 1.34-1.47%), in the scan and
  leave-out 3; Into Music's 1,556.9-1,616.4 s segment in the 2003 mix,
  1,392 hits, 1.05% from the stronger 1,604.9-1,772.1 s one. Each play
  gains the segment; no play is added, lost or changes level.
- Every weak detection added (6-20 hits; 2 in leave-out 11 at NAS scale)
  overlaps a stronger one of its file 0.61-5.8% apart. Removed: weak
  Star Trails detections at 870-894 s (9-18 hits; 1 per report at 262
  tracks, 3 at NAS scale) within 0.22-0.52% of the newly kept side. No
  confident or possible detection was lost.
- User CPU, Stakka scan at NAS scale, idle: start build 693 and 701 s,
  final 659 and 661 s (A B B A); final alone, `--overlap-by-spans` 661
  and 664 s, default 663 and 664 s (+0.2%). The rule costs nothing
  measurable; the 5% between the builds is not the rule's (by inference
  code layout). The opt-in's report equals session 52's baseline.

**Conclusion.** Every difference is a segment more than 0.6% in speed
from the stronger overlapping detection of its file, or a weak one such
a segment now drops. New baselines `session-53-linux`, `-linux-nas`.
