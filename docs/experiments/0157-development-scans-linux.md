# 0157: the development scans, leave-outs and per-second scores on Linux

**Question.** Session 33, step 6: at 40,683 NAS records, under 0155's clusters,
do the three development mixes with session 10/18's leave-outs (0067, 0100) and
every track left out (0122, 0139) give the Mac's scores, and `seconds` 0139's?
`work/bin/s33-start`, `scan` with `--other-peaks-dir` and the saved index
(ladder both), one run at a time; `seconds --margin 15|30|60 --boundaries` on
the 3 scans and 3 every-track leave-outs; `devscore.py --nas`,
`seconds_summary.py`, `seconds_compare.py`; `data/0157-*.json`.

| Mix | Scan | Leave-out 3 (seeds) | All left out | Possible, no track | Wall; search |
|---|--:|--:|--:|--:|--:|
| Stakka & Skynet, 56:09 | 11/11 | 8/11 (2026) | 0/11 | 3 | 76 s; 72 s |
| My 2003 mix, 46:10 | 12/12 | 9/12 (2026) | 0/12 | 3 | 59 s; 55 s |
| Nocny Trans, 1:06:47 | 17/17 | 14/17 (2026-2028) | 0/17 | 3 | 106-108 s; 101-103 s |

- No wrong identification in any of the 11 reports; every score as on the Mac
  (0067, 0100, 0122), and `devscore.py --nas` agrees with each stored score.
  The possible plays matching no track are the Mac's known ones (Nocny: Mampi
  Swift's 2000 and the Klute Remix in two files, 0100's four without the pruned
  `1.mp3`, as in 0122). Peak memory 5.0-5.1 GiB.
- `seconds` against 0139 (the Mac at 40,126, session 27's variant build, which
  is identical to master there): the three leave-outs equal, all
  10,148 s silent; Stakka's and my mix's seconds equal; Nocny one second apart:
  its track 16's label ends 1.4 s later (end error -0.52 s, Mac -1.95 s), so
  one second in a margin becomes correct. At a 30 s margin 9,082 of 10,013
  indexed seconds are correct (90.7%, as 0139), 759 mislabelled (equal);
  margins 15 and 60: 86.9% and 94.5%. Every other track boundary is equal to
  0.01 s (0139 has none for my mix; here start error median +0.7 s).
- Times for the next session (about 120 leave-out scans): a NAS-scale
  `scan --leave-out` loads the saved index in under 3 s and takes 59-108 s
  wall by the mix's length, at 32 jobs. The Mac, at 33,723 records, needed
  64-66 s of search and 71-74 s for Nocny's leave-outs (0100, session 18,
  under other load): here 1.5 times as long at 21% more records, as is
  clustering (0155); the sweep and 262-track runs are faster here. 0158
  measures 16 workers against 32.

**Conclusion.** The development scans and leave-outs reproduce the Mac's scores
exactly, and per-second scores within one second of 0139.
