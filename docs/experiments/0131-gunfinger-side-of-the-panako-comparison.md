# 0131: Gunfinger's side of the Panako comparison, at 262 tracks

**Question.** Session 24b, step 4: do the saved Gunfinger reports that
0130's comparison reads still match the current build, and what are
Gunfinger's numbers on those queries?

**Method.** `work/bin/s24-start/` of the main checkout (the code of
`53d4136`, unchanged on this branch) run from this worktree, `nice -n
10`, one run at a time, with `--work work/s24b`: copies of the peak
store, the clusters and the renders (receipts written for renders made
before receipts existed, so nothing was rendered again). Each report
against the saved one with `scripts/analysis/report_identity.py`;
columns with `scripts/analysis/panako_compare.py gunfinger`. Seed 2026's
sweep and the stakka scans were checked by session 24's `regress` at
18:45 the same day.

| Run (saved report's date) | Wall | Load | Against the saved report |
|---|--:|--:|---|
| scan `razorjack-2003-03-29` (10-11) | 15 s | 10-12 | identical |
| scan `nocny-trans-vertical-2002-08-23` (10-14) | 30 s | 10-15 | identical |
| sweeps 2027, 2028, 2029 (10-08) | 113, 152, 166 s | 14-37 | identical |
| 12 mixes of seed 2026 (10-08) | 42 s | 17-19 | identical; renders byte-identical |

| Panel | Gunfinger, confident only |
|---|---|
| Sweeps 2026-2029 (720 queries each) | 60/60 at every speed, 0 wrong; speed error at most 0.026 points |
| Development mixes (referenced at 262 tracks) | stakka 11/11, razorjack 7/7, nocny 9/9; 0 wrong |
| Generated mixes, seed 2026 | 106 of 115 indexed plays, 0 wrong (29 held-out plays) |
| Index (`gunfinger stats`) | 105,303 s of audio; peak store 19.7 MB; index 39.7 MB, 7.84 M postings |
| Fresh `gunfinger index` (265 files) | 50.9 s wall, 471 s CPU, 938 MB maximum resident, load 15 |

Boundaries (earliest credited start minus the manifest's): stakka -68
to +0.1 s; nocny -26 to +16 s except track 10, +91 s; razorjack's track
4 is on the NAS only, so none is measured there at 262 tracks.

**Conclusions.** Every saved report the comparison reads matches the
current build, so Panako's answers can be scored against them as they
stand. A fresh index now extracts all 265 audio files, 0 failed,
including the 3 the working store keeps as skip notes ("decoded short");
the harness's library stays the store's 262.
