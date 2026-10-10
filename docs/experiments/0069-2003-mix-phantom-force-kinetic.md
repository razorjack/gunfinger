# 0069: the 2003 mix without Phantom Force's rips, and the Kinetic tease

**Question.** Item 6 of session 10: without Phantom Force's rips, do
Phantom 2018 or Fracture's edit become confident? Without the PHUD1 rip,
does the start move from 10:48 to 11:55? What lies over the Kinetic tease?

**Commands.** 0063's `identify --store-only --playback both -f json -v`
(`work/bin/s10-item2/`, store `5701f221f7b48ee1`) as is, then with
`--exclude-from` Phantom Force's cluster (the i-witness upload and the
PHUD1 rip, joined in the stopped clusters run), then the PHUD1 rip alone;
`explain --at 17:53 --asset kinetic --windows`. 255-265 s, 1,843-1,863 s
CPU, 4.6 GB each; `explain` 77 s. Data: `data/0069-*.json`.

| 10:00-16:00 | Nothing left out | Cluster left out | PHUD1 left out |
|---|---|---|---|
| PHUD1 rip | 10:49.0, 6,513 hits, +4.52% | – | – |
| i-witness upload | 11:55.8, 2,233 hits, +5.97% | – | 11:55.8, 2,233 hits |
| Phantom 2018 | possible, 322 hits from 11:31 | the same | the same |
| Fracture's edit | possible, 118 hits at 13:46 | the same | the same |
| Rift - Meltdown | possible, 98 hits, key lock | the same | the same |

With nothing left out the mix has 26 plays (23 confident) of the 12
tracks; 0063's mixed-CD lines left with the prune. Without the cluster
nothing is confident from Escape Route's end (11:48.6) to Two Faced
(14:40.8). Phantom 2018's 322 hits are three segments of 85-146 hits,
each on the same part of it (track 1:01-1:56), so none reaches 240. The
PHUD1 rip's first segment, 10:49.0-12:02.8 (823 hits in 9 windows, track
0:58-2:15), holds an intro the upload never matches (it aligns from its
2:09); the rip runs 1.4% faster than the upload.

**Kinetic.** The play's first segment, 17:23.7-18:01.8, is track 1:11-1:51
(199 hits in 5 windows: possible alone, 39 hits short). No Kinetic line
from 18:02 to 18:35; at 18:35.6 the track starts again from 0:15,
confident to 25:18.8 (8,434 hits in 41 windows); one play from 17:23.

**Conclusions.** Phantom 2018 and Fracture's edit share a passage with
Phantom Force but stay below the rule with or without its rips: without
them the track is missed with three possible notes, none wrong. The 10:48
start comes from the PHUD1 rip alone. The evidence matches the owner's
tease: 40 s from 1:11, then the track from 0:15 at 18:35.
