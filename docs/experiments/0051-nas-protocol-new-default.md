# 0051: the NAS-scale protocol under the new default and clusters

**Question.** Item 6 of session 6: the development scan, leave-outs 3
and 11, sweeps 2026 and 2027 and calibrate at NAS scale, under the
fitted matcher (ADR 0008) and the clusters of 0049, against session 5
(0039, 0040, 0045: skip at 240, with the link rules where measured).

**Command.** `work/scripts/s6-nas-protocol.sh` (harness with
`--other-peaks-dir`, the frozen store `d428ee9585936326`, 26,969-26,972
assets in the sweeps), one step at a time; `nas_calibrate.py` and
`nas_scale_summary.py` split and summarise. Data:
`data/0051-nas-protocol-new-default.json`.

| At NAS scale | Session 5 (skip at 240, links) | Now |
|---|--:|--:|
| Development scan: identified; wrong; possible matching no track | 11/11; 6; 3 | 11/11; 0; 3 |
| Leave-outs 3 and 11: identified; wrong | 8/11, 0/11; 0 | 8/11, 0/11; 0 |
| Scan: false candidates; 30 hits or more; strongest unrelated | 2,981; 14; 32 | 2,990; 8; 32 |
| Sweep 2026: recalled; wrong; weakest identifying | 540/540; 91; 680 | 540/540; 0; 703 |
| Sweep 2027 (0040, no links): recalled; wrong; weakest | 540/540; 174; 671 | 540/540; 24; 671 |
| Calibrate: weakest; strongest false; margin | 661; 5,845; 0.11× | 671; 408; 1.64× |
| Without China Cup ~ its Prototype upload | | 341 (Coma ~ Spraycan); 1.97× |
| Without that pair as well | 220 (The Nine ~ VIP); 3.0× | 197 (The Nine ~ VIP); 3.41× |
| Unrelated chance (different artist and title) | 67-70 | 63-70 |
| Scan; sweep: wall, CPU | 273 s, 1,963 s; 1,351 s, 12,135 s | 255 s, 1,950 s; 1,296 s, 12,077 s |

The 24 wrong answers of sweep 2027 are two pairs, 3 per speed: China
Cup (two corpus rips) against the alien5ive PRO 001 upload (65-68%
coverage, borderline) and Coma against the i-witness Spraycan (58%).
The strongest "unrelated" false candidate, 112 hits, is a blend on the
mixed CD Dangerous Drums 2 (its Meatball track against the Luminous
remix next to it), not chance. The protocol took 3,341 s and 30,030 s
CPU, at most 5.6 GB.

**Conclusions.** With the fast uploads clustered, the development mix
has no wrong identification at NAS scale and sweep 2026 none either.
What stays false are two borderline pairs and The Nine's VIP, all for
the owner's ear; unrelated chance is unchanged at 63-70 hits.
