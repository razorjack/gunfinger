# 0244: a sketch before grouping, in full searches

**Question** (brief 60). Does F1024, the replays' winner (0243), meet the
registered bar (NAS scale at 32 jobs 5% faster in both pairs; no pair 1%
slower at 16 jobs or 262 tracks; peak memory at most 2% higher)?
**Method.** `work/bin/s60-start` (master) against the prototype
(`s60-sketch`, `GUNFINGER_S60_SKETCH=1024`): the Stakka scan A B B A at
NAS scale and 262 tracks, 32 then 16 jobs, the Linux machine otherwise
idle (`work/s60/ab.sh`; the Mac not measured). Then session 41's stage
timers, NAS scale, 32 jobs, A B B A. Data `data/0244-sketch-times.txt`.

| Stakka scan, search (`wall_seconds`) | Start | F1024 | Change |
|---|--:|--:|--:|
| NAS scale, 32 jobs | 23.60, 24.16 s | 30.85, 31.97 s | +30.7%, +32.4% |
| The same, user CPU | 650, 661 s | 865, 906 s | +33.1%, +37.0% |
| NAS scale, 16 jobs | 29.20, 29.33 s | 32.97, 32.93 s | +12.9%, +12.3% |
| 262 tracks, 32 jobs | 6.53, 6.50 s | 6.74, 6.53 s | +3.2%, +0.6% |
| 262 tracks, 16 jobs | 6.75, 6.87 s | 6.93, 6.85 s | +2.6%, -0.3% |
| Peak memory (max RSS, 32 jobs), NAS / 262 tracks | 4.9 GiB / 197 MiB | +0.5-0.9% / +5-7% | |

| First pass CPU, NAS scale, 32 jobs (timers) | Start | F1024 |
|---|--:|--:|
| Whole first pass | 655, 662 s | 830, 832 s |
| Taking parts | 85, 87 s | 135, 135 s |
| The sketch | | 300, 300 s |
| Grouping (flagged assets only with F1024) | 262, 265 s | 195, 196 s |
| The pre-filter | 136, 137 s | 33, 33 s |
| Query analysis; aligning, sorting, lines | 100-101; 68-69 s | 95-96; 67 s |

Every Stakka report identical to its baseline's apart from `wall_seconds`
(16 scans, 4 timed); the other registered equality checks were not run
once the speed bar failed.

**Conclusion.** Rejected on four counts: NAS scale 31-32% slower at 32
jobs and 12-13% at 16, 262 tracks up to 3.2% slower, its peak memory 5-7%
higher. The sketch costs 300 s, more than the pre-filter (103 s) and the
grouping (67-69 s) it spares: grouping still reads every hit twice to
test its flag, and by inference the 1 MiB table evicts the lists and
parts, so taking costs 50 s more. Smaller tables pass most hits (0243).
Master's search is unchanged.
