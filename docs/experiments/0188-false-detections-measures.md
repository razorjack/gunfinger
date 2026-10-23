# 0188: the false confident detections left at NAS scale, by the pair measures

**Question.** Brief 40, step 5a: which recordings do the false confident
detections at NAS scale pair, what are the pair measures (0186) of each
pair, and how many are near-identical versions by them? The roadmap
names China Cup's upload as most of them.

**Reports.** `regress session-33-linux-nas` and the 11 saved
development scans at NAS scale: 7 false confident, all in sweep 2026
(calibrate's "false accepted"). Seed 2026 has no China Cup query, so
sweeps of seeds 2027 (China Cup indexed) and 2028 (held out) ran at NAS
scale (`work/bin/s40-run`; 8:34, 8:19; 60/60 at every speed); then
`pair` on each pair behind a false candidate of 30 hits or more: 84
rows of seed and pair (73 distinct pairs), 294 detections. Data: `data/0188-*.json`.

| Seed | Query (held out?) | Found | Confident | `near_10` | Gap (s) | Coverage |
|---|---|---|--:|--:|--:|--:|
| 2026 | Aphrodite - Cool Rock Stuff, APH027 B1 (yes) | Mobb Deep Vs Urban Takover - Cool Rock Shook Ones | 7 | 0.37 | 209 | 0.41 |
| 2027 | `Bad Company - China Cup.mp3` (no) | ƆƐI3C - China Cup | 4 | 0.77 | 80 | 0.71 |
| 2027 | `extra/02. Bad Company - China Cup.mp3` (no) | ƆƐI3C - China Cup | 7 | 0.77 | 80 | 0.71 |
| 2028 | `extra/b-bad_company-china_cup.mp3` (yes) | ƆƐI3C - China Cup | 7 | 0.77 | 81 | 0.78 |

18 of the 25 false confident detections over the three sweeps are China
Cup's three corpus rips found as ƆƐI3C's upload, judged different by
ear ("almost identical, but the hi-hats differ for a few bars"). The
other 7 are Cool Rock, judged a different mix sharing material.

**By the measures.** At 0186's threshold (`near_10` 0.552) only China
Cup's three pairs fall on the same side, of all 73; the next are
The Nine ~ its Evol Intent VIP (0.49, 114-219 hits, never confident),
Pyro ~ Ignorance (0.42) and Cool Rock (0.37): the measure places the
18 with the same recordings and the 7 with different ones. Since it
does not tell China Cup's kind from the same recording (0187), that
says only that the file is held near one offset as same recordings are.

**Conclusions.** The roadmap's statement holds at seeds 2027 and 2028
for ƆƐI3C's upload (the Prototype upload of 0049 was joined by a
verdict on 2026-10-09). Whether these 18 count as wrong is ours: the
detections name the right track in a version that differs for a few
bars. Without them NAS-scale false confident detections are one pair.
