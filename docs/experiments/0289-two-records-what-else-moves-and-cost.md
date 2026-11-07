# 0289: two records for a flagged file: what else moves, and the cost

**Question** (brief 82, registered in `98db422`). With 0288's five second
records (B), what moves in the sweeps and on the shared panels against
today's index (T), is it the set-aside cut, and what does it cost? **Command.**
`work/s82/{qnas,q262,qattr,qpanels-off,qtime}.sh` (builds `s82-start`
for the panels' T, `s82-run2`, `s82-run3` for `--set-aside-like` and the
times); `sweepcmp.py`, `moves.py`, `panelcmp.py`. Data:
`data/0289-two-records-what-else-moves-and-cost.json`.

| B against T | Identical | What differs | With T's set-aside lists |
|---|--:|---|--:|
| NAS sweep (against `session-78-linux-nas`) | 718 of 720 | 2 candidates of 6-7 hits; `per_speed` equal | 720 of 720 |
| NAS, 0288's 240 queries | 239 | 1 speed by 1e-6 percentage points | 240 |
| 262 tracks and the 18 files: sweep | 375 of 720 | hits and speeds; no confident change | 718 |
| 262 tracks: 0288's queries | 142 of 240 | 111 plays (hits up to 3.9%), no level change | 240 |
| Panel 1 development half, NAS (622 results) | 594 | 28: 1 hit or a speed by up to 1.3e-4 points | not run |
| Panel 2 development half, NAS (440 results) | 440 | | not run |

**A2-A4 per set.** On both panels every confident and possible count by
role is the same under B and T (panel 1: 516 played, 430 related, 475
other confident; panel 2: 545, 12, 3); in the sweeps no detection
crosses 240 or 60 hits, and B's own sweeps hold no second record. A
second record is detected only weakly: Keltech - Axis's, 45 times in a
panel 1 scenario's loops (at most 11 hits). With T's lists set aside (`--set-aside-like`, the one check of
`docs/session-rules.md`) every NAS difference goes: one list crosses the
cut each way for the plan's index, one for the sweep's. At 262 tracks,
where the 5 records are 1.5% of the postings, 71 lists cross for the
plan's index and 114 for the sweep's; they account for every move but
2 sweep candidates (6 and 7 hits) of Plasmic - The Weapon's second
record. For `shared`, which has no such option, the cause is an
inference. **Cost (A6).** The second records hold 123,160 postings,
0.011% of the NAS index (1,140,812,725), in 5 more assets. The index
with them was built in memory once in 122 s and saved (4.6 GB; loaded in
2.6-3.8 s, as today's); peak memory 4.9 GB in both arms. Search time of
0288's 240 queries (32 jobs, alone, A B B A): T 96.5 and 95.8 s, B 96.3
and 96.3 s (the Linux machine), no difference beyond noise.
**Conclusion.** A2, A3, A4 and A6 hold; with 0288's A1 and A5 the
registered bar holds: two records per flagged file are worth an ADR
proposal, which is the user's decision (decisions queue item 3).
