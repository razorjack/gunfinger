# 0067: the NAS baseline at 32,441 records with the owner's verdicts

**Question.** Item 4 of session 10: the NAS protocol with 0066's
clusters against 0051 and 0053 (27,042 records), and the 2003 mix.
`work/s10/item4.sh` (d77df95, store `5701f221f7b48ee1`), one run at a
time, other-rips sweeps moved out. Data: `data/0067-*.json`.

| At NAS scale | 0051, 0053 | Now |
|---|--:|--:|
| Development scan: identified; wrong; possible, no track | 11/11; 0; 3 | 11/11; 0; 3 |
| Leave-outs 3 and 11: identified; wrong | 8/11, 0/11; 0 | 8/11, 0/11; 0 |
| Scan: false candidates; 30 hits or more | 2,990; 8 | 3,533; 8 |
| Sweeps 2026-2029: recalled; wrong | 2,160; 0, 24, 8, 8 | 2,160; 4, 0, 8, 12 |
| Calibrate: weakest; strongest false; margin | 661; 511; 1.29× | 661; 382; 1.73× |
| Without same-name pairs (The Nine's VIP) | 220; 3.00× | 220; 3.00× |
| Unrelated chance, outside mixed-disc blends | 63-70 | 63-70 |
| Scan: wall; CPU; peak | 255 s; 1,950 s | 307-315 s; 2,281-2,328 s; 4.8 GB |
| Sweep: wall; CPU; peak | 1,296 s; 12,077 s; 5.5 GB | 1,535-1,584 s; 14,012-14,535 s; 6.3-6.7 GB |

**Changes.** (0053's 197 for the VIP was its held-out figure, 197 now.)
The verdicts remove all 40 wrong answers of 0053: China Cup (24) and
Coma (8) join their uploads; Synthesis VIP's mixed-CD track (8) is
pruned and ignored. All 24 new ones come from a new folder, Essential
Rewindz's mixed disc 2: Funktion (16, up to 332 hits) and The Nine (8, up
to 382) against mixed tracks that cover 58-60% and 78% of themselves with
the track, borderline, so different recordings. That disc's blends are
also the strongest "unrelated" candidates (The Nine in Killa Bees,
142-177). The store's growth (+20% records) costs 19-20% more CPU and
18% more false candidates; chance and the weakest identifying stay.

**The 2003 mix.** NAS scale: 12/12, 0 wrong, the three possible notes of
0069; leave-out 3 (Phantom Force, Balderdash, Skektics) 9/12, 0 wrong,
nothing confident over the three. 262 tracks: 7/7 referenced, 0 wrong,
nothing possible over the 5 absent tracks; leave-out 3 (The Specialist,
Yvon Is On, Skektics) 4/7, 0 wrong. Calibrate with it: 1.73× (NAS) and
6.65× (262 tracks), as without it.

**Conclusions.** Recall holds at 32,441 records; the verdicts clear
every earlier wrong answer; the rest is one unlisted mixed disc, for the
owner's ignore file or ear (pack items 4 and 8).
