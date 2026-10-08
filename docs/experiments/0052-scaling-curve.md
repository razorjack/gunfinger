# 0052: chance and cost against index size, on real records

**Question.** Item 8: the development scan and sweep 2026 under the
default matcher with seeded, nested subsets of the NAS records (1,000,
3,000, 9,000), between 262 tracks (0047) and the NAS (0051). How do
chance and cost grow, and what possible tier follows at each size?

**Command.** `work/scripts/s6-scaling.sh` (`--other-sample N`, new in
the harness; `regress session-6-candidate` identical with it built),
one step at a time; `scripts/analysis/scaling_curve.py`. Data:
`data/0052-scaling-curve.json`.

| Assets in the sweep | 209 | 1,205 | 3,202 | 9,196 | 26,969 |
|---|--:|--:|--:|--:|--:|
| Sweep: false candidates; 30+; 60+ | 362; 9; 9 | 1,295; 18; 14 | 3,112; 26; 14 | 8,204; 32; 15 | 22,569; 60; 24 |
| Strongest unrelated, sweep | 21 | 27 | 34 | 64 | 64 |
| Strongest unrelated, development scan | 23 | 20 | 20 | 21 | 32 |
| Strongest on audio not in the index | 20 | 72 | 72 | 72 | 72 |
| Sweep: wall; CPU | 180 s; 1,555 s | 204 s; 1,930 s | 287 s; 2,689 s | 533 s; 4,981 s | 1,296 s; 12,077 s |
| Scan: wall; CPU; peak | 31 s; 238 s; 0.3 GB | 37 s; 303 s; 0.4 GB | 52 s; 427 s; 0.7 GB | 100 s; 802 s; 1.5 GB | 255 s; 1,950 s; 4.0 GB |

Recall is 540/540 with 0 wrong at every size; the scan 11/11, 0 wrong.
The false candidates of 60 hits or more are related records (versions,
shared breaks, a mixed CD's blends), plus the unrelated pair below. The
72 on audio not in the index is a held-out VIP (Item Code - Protection,
IBS Faction VIP) against an upload of the original. CPU grows linearly:
0.39 s per asset for the sweep, 0.064 s for the scan, after a fixed
1,500 and 240 s.

The strongest unrelated pair, 64 hits from 9,196 assets, is one record
(Clockwork against Simon Static - Rubba Rock); the next are 34 or less.
Chance at the top is a few individual coincidences, not a smooth curve:
it stays at 20-34 to 3,200 assets and reaches 63-70 at 27,000 (0051).

**Proposal (not applied).** Twice the strongest unrelated chance gives a
possible tier of 60 up to about 3,000 assets and about 130-140 from
9,000. A rule by size: 60 while the strongest unrelated chance measured
at the library's size stays below 30, otherwise twice it, measured again
when the library doubles. At 140 the development mix would lose its two
shared-material notes (79 and 94 hits).
