# 0156: recall under the hardest conditions on the Linux machine

**Question.** Session 33, step 5: at 40,683 NAS records, under 0155's clusters,
do session 28's recall conditions (0143-0145, the Mac at 40,126) give the Mac's
counts? `work/bin/s33-start`, `--panels work/s33/panels` (its one NFD source in
NFC, so all 58 indexed and 20 held-out sources are searched, as on the Mac),
eight invocations in session 28's order, one at a time, 03:57-05:27.
`recall_conditions.py`, `recall_compare.py`; `data/0156-*.json`.

| Condition (15 s; 20 s) | Confident, Mac -> here | Possible+, Mac -> here | Wall here |
|---|--:|--:|--:|
| C, clean 30 s | 522 -> 522 | 522 -> 522 | 7:35 |
| (a) other rips only | 175 -> 175 | 178 -> 178 | 7:16 |
| (b) brief | 228; 493 -> 228; 493 | 522; 522 -> same | 13:11 |
| (c) combined damage | 5 -> 4 | 428 -> 426 | 9:19 |
| (a)+(b)+(c) | 0; 0 -> 0; 0 | 34; 73 -> 36; 71 | 15:30 |
| (a)+(c) | 0 -> 0 | 125 -> 127 | 8:43 |
| (b)+(c) | 0; 0 -> 0; 0 | 184; 306 -> 181; 304 | 15:25 |
| (a)+(b) | 37; 141 -> 35; 141 | 177; 178 -> 178; 178 | 13:09 |

- Precision as on the Mac: no wrong confident answer in any condition, no
  held-out (360 brief, 180 otherwise) or unobservable query confident, no wrong
  possible detection of 200 hits; the strongest is the Fire Splash Remix on the
  Fire Original Mix everywhere (184 hits clean, Mac 185; 146 brief, equal).
- C and (a) agree in every cell; (b) in its totals, with single cells by speed,
  kind and grid phase one query apart. (a) leaves out 63 records (58 own, 5
  copies), 20 sources observable, and misses the Mac's five queries (Silent
  Weapons at -8% and Droogs at +8% never a candidate; Victim 3 times at 201-230
  hits, Mac 202-231). (a)+(b) cells differ by up to 2. With damage, cells
  differ by up to 2 ((c), (a)+(c)) and 6-8 ((b)+(c), (a)+(b)+(c)), totals by up
  to 5; partner tempos agree (88 of 702 unmatched on both, here from the log).
- Every query is rendered here (MP3 by libmp3lame, damage by the filters and
  AAC encoder of FFmpeg 8.1.3; the Mac's 9.0.2), peaks differ (0153) and the
  index holds 557 more records, so the data cannot separate the causes. The
  Mac's data files hold breakdowns, not queries; only 0144's named misses
  compare query by query.
- 4.9-5.3 GiB at peak. The Mac's runs (6-12 min) shared its CPU with Panako.

**Conclusion.** Session 28's recall results reproduce: the same precision, equal
counts for C, (a) and (b), and the rest within a few queries.
