# 0048: why fast uploads cover 0-55% of their corpus rips

**Question.** Item 5a of session 6. At NAS scale the development scan's 6
wrong identifications are uploads of the tracks played, 3-4.7% fast
(experiment 0039). Session 5's clusters found three of the pairs at
44-55% coverage and three not at all. Is that the ladder, a cut or edit,
or key lock?

**Method.** `gunfinger-eval --other-peaks-dir <NAS store> pair
work/logs/s6-dev-pairs.tsv`: each corpus rip searched against an index of
its upload alone, by stored peaks on the clustering ladder of session 5
(0.98-1.02), the turntable ladder (0.92-1.08), the key-locked ladder, one
rung at the fitted speed, and by decoded audio on both ladders, with and
without the second pass. 277 s, one core (loading the NAS store's list).
Data: `data/0048-fast-uploads.json`.

| Upload (corpus rip) | Fitted speed | ±2% ladder: strongest | Turntable ladder | At fitted speed | Audio, both ladders |
|---|--:|--:|--:|--:|--:|
| knight lore, alien5ive (Night Lore) | 0.9704 | 736 hits, 44% | 4,517 | 4,231 | 5,148 |
| Nightlore, i-witness (Night Lore) | 0.9668 | 37, 3% | 4,788 | 4,911 | 5,706 |
| star trails (Star Trails) | 0.9710 | 816, 55% | 4,142 | 4,112 | 4,777 |
| pathogen (Pathogen) | 0.9618 | 19, 2% | 4,766 | 4,283 | 5,453 |
| kontempt (Kontempt) | 0.9556 | 6, 0% | 5,447 | 5,242 | 6,199 |
| logistics (Logistics) | 0.9713 | 848, 44% | 6,153 | 6,108 | 6,689 |

On the turntable ladder each pair is one alignment over the whole of
both files (38-46 windows, from the first seconds to the last); the
upload's offset is within 1.5 s of the rip's start. Key lock finds
nothing over 24 hits. The ±2% figures reproduce session 5's clusters
file exactly.

**Conclusions.** The ladder alone explains the low coverage. The uploads
play 3.0-4.6% faster than the rips (speed 0.956-0.971 from rip to
upload). On the ±2% ladder the nearest rung, 0.98, is 0.9-2.6% off: at
0.9-1.0% half of the track still aligns, in pieces at one offset (no cut
or edit); from 1.4% almost nothing. On the ±8% ladder, as
planned for item 5b, every pair covers the whole file. Coverage must be
measured in the shorter file's own seconds: the query's span over the
shorter file gives 100-104% here.
