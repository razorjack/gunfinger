# 0073: the NAS baseline at 33,596 records, after the prune

**Question.** Item 5 of session 11: the NAS protocol of 0067 with 0071's
clusters. Prediction: Essential Rewindz's 24 wrong sweep answers are
gone. `work/s11/item5.sh` (1afd8cc, store `f9f9bc706195a71b`), one run
at a time; `nas_calibrate.py` and `sweep_diff.py`. Data:
`data/0073-nas-baseline-33596.json`.

| At NAS scale | 0067 (32,441) | Now (33,596) |
|---|--:|--:|
| Development scan: identified; wrong; possible, no track | 11/11; 0; 3 | 11/11; 0; 3 |
| Leave-outs 3 and 11: identified; wrong | 8/11, 0/11; 0 | 8/11, 0/11; 0 |
| Scan: detections; false at 30 hits or more | 3,568; 8 | 3,681; 8 |
| Sweeps 2026-2029: recalled; wrong | 2,160; 4, 0, 8, 12 | 2,160; 0, 0, 0, 0 |
| Calibrate: weakest; strongest false; margin | 661; 382; 1.73× | 661; 220; 3.00× |
| Strongest on audio not in the index | 197; 3.36× | 197; 3.36× |
| Unrelated chance in the sweeps | 63-70 (blends 142-177) | 63-70 |
| Scan: CPU; peak | 2,281-2,328 s; 4.8 GB | 2,337-2,368 s; 4.93 GB |
| Sweep: wall; CPU; peak | 1,535-1,584 s; 14,012-14,535 s; 6.3-6.7 GB | 1,666-1,724 s; 14,619-15,129 s; 6.8-6.9 GB |

**Changes.** As predicted, the 24 wrong answers are gone: Funktion (16)
and The Nine (8) against Essential Rewindz's mixed disc 2, whose files
the patterns leave out and the prune removed. The strongest false
candidate is now The Nine's Evol Intent VIP (220 hits, judged
different), so the margin with and without same-name pairs is the same
3.00×; no false candidate reaches the rule. The strongest "unrelated"
candidate of the split (79 hits) is Star Trails' Synergy remix, scored in
the development scan against Decoy's slot; in the sweeps, unrelated
chance stays at 63-70 hits. +3.6% records cost 3-4% more CPU. The scans'
wall times (424-974 s) ran under other load and
are not compared; the sweeps had most of the machine.

**The 2003 mix.** NAS scale: 12/12, 0 wrong, 3 possible plays matching
no track; leave-out 3, 9/12, 0 wrong (300 s each). 262 tracks: 7/7 and
4/7 with leave-out 3, 0 wrong. Calibrate with it: 3.00× (NAS) and 6.65×
(262 tracks).

**Conclusions.** Recall holds and nothing false passes the rule; the
closest is a VIP of the played track, 1.09× below it (shared material).
