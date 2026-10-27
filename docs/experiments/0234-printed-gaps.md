# 0234: the gap lines the development mixes print (ADR 0023)

**Question.** Session 55: which "ID - ID" lines do the development mixes print, what plays there?

**Method.** `identify --store-only` of each mix with the start and the run build (`s55-start`,
`s55-run`) at 262 tracks and NAS scale (the NAS store's records); `show` of each report; reports
and tracklists compared (`work/s55/beforeafter.py`); each gap against the manifest (`gaplines.py`;
missed: no reference in the library at that scale). Then session 34's 45 NAS-scale scans (3 plain,
42 slot leave-outs) with today's search (ADR 0021) and harness `gaps` at 30, 45 and 60 s, each scan
counted as it is, repeats included (`leavecount.py`; 0159 counted repeats once).

| Scale, mix | Gap line | Plays there (manifest) | Kind |
|---|---|---|---|
| 262, 2003 | 0:00 (2:25) | 1 Konspiracy | missed, the mix's start |
| 262, 2003 | 6:43 (7:57) | 3 Escape Route, 4 Phantom Force | 2 missed |
| 262, 2003 | 20:04 (4:15) | 6 Kinetic | missed |
| 262, 2003 | 29:25 (1:39) | 8 Artificial Life | missed |
| 262, Nocny | 6:19 (3:04) | 2 Mutated (Version X); Phoenix's last 6 s | missed |
| 262, Nocny | 15:04 (3:30) | 4 The Descent (Phunckateck VIP) | missed |
| 262, Nocny | 21:42 (14:27) | 6 Invid, 7 Mistaken Identity, 8 Nephelim; Scarred's last 12 s | 3 missed |
| 262, Nocny | 46:29 (2:04) | 12 Stab 2 Def; Dead Fall's last 23 s | missed |
| 262, Nocny | 51:07 (6:36) | 14 We R The Ones (VIP), 15 Cannibal | 2 missed |
| 262, Nocny | 1:01:23 (3:32) | 17, "ID - ID" in the manifest (no reference) | missed |
| NAS, Nocny | 1:01:24 (3:10) | 17, the same | missed |

Stakka at both scales and the 2003 mix at NAS scale print none; no line is a found track's own time,
an intro or an outro; no gap holds a possible play; all last 1:39 or more, so 30-60 s print the same.

| Min. | NAS leave-outs: gaps | without a missed track | missed reached | plain scans' gaps |
|--:|--:|---|--:|--:|
| 30 s | 140 | 8: in Blindside, 43 s, and Kinetic, 34 s (4 scans each) | 284/284 | 1 (slot 17) |
| 45 s | 128 | 0 | 284/284 | 1 |
| 60 s | 128 | 0 | 284/284 | 1 |

No scan has a wrong identification and no gap lasts 45-60 s. Brief plays are where the lengths
differ: 0159's generated mixes (plays of 20-60 s, before ADR 0021) reached 99 of 215 at 45 s, 58 at 60 s.

**Conclusion.** Before and after, every report's plays are identical, each tracklist is the old one
plus its gap lines, and both regressions are identical. Every printed line marks a track not in the
library at that scale; the found tracks' uncovered stretches (34 and 43 s) stay under 45 s.
