# 0039: the development set at NAS scale

**Question.** Item 5 of session 5 (NAS plan step 6): the development scan
and leave-outs 3 and 11 against the NAS collection, under today's matcher
and skip at 240, against the 262-track figures.

**Command.** `work/s5/E [--second-pass --skip-fullest 0.01] scan
stakka-skynet-knowledge [--leave-out 3|11]` (`E` is `gunfinger-eval
--other-peaks-dir <NAS store>`): 26,905 assets (corpus and NAS less the
247 copies); clusters of 0037. Timed one at a time (`timed5.sh`, 86%
idle before). Data: `data/0039-nas-development-set.json`.

| Development scan | Today's, NAS | Skip at 240, NAS | 262 tracks (today's; skip) |
|---|--:|--:|--:|
| Identified; wrong; possible plays matching no track | 11/11; 6; 3 | 11/11; 6; 3 | 11/11; 0; 1 |
| Leave-out 3; leave-out 11 (wrong) | 8/11 (6); 0/11 (6) | 8/11 (6); 0/11 (6) | 8/11 (0); 0/11 (0) |
| Weakest identifying | 1,839 | 1,557 | 1,839; 1,558 |
| Strongest false: all; without the 6; unrelated | 5,785; 94; 40 | 5,845; 119; 48 | 94; 94; 24 / 119; 119; 21 |
| Detections matching no track; with 30 hits or more | 413,989; 16 | 8,704; 16 | 6,096; 2 / 214; 2 |
| Wall; CPU; peak footprint | 864.6 s; 6,097 s; 8.37 GB | 260.6 s; 1,963 s; 4.01 GB | 33.6 s; 264 s; 399 MB / 31.1 s; 238 s; 293 MB |

Leave-outs take within 1% of these times. The 6 wrong detections are the
same in all six scans: YouTube uploads of the tracks being played (Night
Lore twice, Star Trails, Pathogen, Kontempt, Logistics), at the same mix
times, 2.9-4.7% faster than the corpus rips by the ratio of fitted
speeds. The clusters (0037) search rips within 0.98-1.02, so these never
join their recordings; their pairs with the corpus rips align at 0.970-
0.972 over 44-55% (borderline). The 3 possible plays matching no track
are known shared material: the Stakka remix of Clockwork twice (as at
262 tracks) and Star Trails (Synergy Remix) at 10:27, 75-79 hits.
Strongest unrelated: Heretik - Biodome at 39:12 (40 hits, today's) and
Genetix - Crunch at 6:11 (48, skip). Nothing else is confident in the
left-out slots.

**Conclusions.** (1) Every referenced track is found at NAS scale with
the same weakest evidence; under the rules 6 detections are wrong, all
other rips of the played tracks, for the owner to confirm. Without them
the margins of this mix are unchanged. (2) Unrelated chance rises from
21-24 to 40-48 hits, below the possible tier. (3) Skip at 240 takes
3.3 times less wall time, 3.1 times less CPU and half the memory.
