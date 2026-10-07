# 0042: the scale proxy against the NAS collection

**Question.** Item 7 of session 5 (NAS plan step 8): how well did 100
reversed, stretched copies of the corpus (26,462 assets; 0012, 0019,
0027-0029) predict cost and chance at the size of the real collection?

**Command.** `/usr/bin/time -l work/s5/E [--second-pass --skip-fullest
0.01] memory` (the whole 56-minute development mix, 10 workers, no
`--minutes`), one at a time on an idle machine (`timed5.sh`); against
0029's development scans after the block merge (3 rounds, idle). CPU is
user plus system. Data: `data/0042-nas-proxy-against-reality.json`.

| Development mix | Proxy, 26,462 (0029) | NAS `memory`, 26,914 | NAS `scan` (0039) | NAS over proxy |
|---|--:|--:|--:|--:|
| Postings; index | 759.9 M; 2,986 MB | 755.9 M; 3,032 MB | same index | 0.99; 1.02 |
| Index build | 52.7 s | 41.9 s; 48.8 s | | 0.80-0.93 |
| Today's: wall; CPU; peak | 718.6 s; 5,841 s; 7.68 GB | 848.3 s; 6,106 s; 8.42 GB | 864.6 s; 6,097 s; 8.37 GB | 1.18; 1.05; 1.10 |
| Skip at 240: wall; CPU; peak | 229.0 s; 1,719 s; 3.79 GB | 258.8 s; 1,958 s; 4.01 GB | 260.6 s; 1,963 s; 4.01 GB | 1.13; 1.14; 1.06 |
| Strongest chance in the mix | 31; 25 (a reversed copy) | | 40; 48 (unrelated tracks) | 1.3; 1.9 |
| Strongest chance per excerpt | 29 (0012: robust, real tracks) | | 66-78; 67-70 (0040, 0041) | 2.3-2.7 |
| Wrong identifications in the mix | 0 | | 6 (other uploads) | |

The store-only `identify` (0044) at 26,890 assets: 856.0 s, 6,137 s,
8.35 GB, within 1% of `memory`. Today's matcher uses 7.2 cores' worth
of the 10 workers on the NAS index against 8.1 on the proxy. Skip at
240 needs 14% more CPU on real music than on the copies. Neither cause
is measured.

**Conclusions.** (1) The proxy sized the index exactly and predicted
memory within 10% and today's CPU within 5%; plan with 1.2 times its
wall time. (2) It cannot predict chance from real music: unrelated
tracks reach 1.3-2.7 times the copies' hits, and other rips and uploads
of the same recordings, which a reversed copy cannot be, cause every
wrong identification at NAS scale. (3) Skip at 240 keeps its lead at
NAS scale: 3.3 times less wall time, 3.1 times less CPU, 48% of the
memory.
