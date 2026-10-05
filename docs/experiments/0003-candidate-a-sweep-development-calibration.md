# 0003: candidate A on the sweep and the development set; calibration

**Hypothesis.** Candidate A (pair hashes, 41-rung scaled-STFT ladder, lines
per 10 s window chained over time) finds every track of the development set
and every indexed sweep excerpt, and an absolute hits-and-windows rule
separates them from everything false.

**Change.** Confidence rule `hits >= 500 && windows >= 3` in
`gunfinger-core/src/confidence.rs`. Same-asset detections overlapping a
stronger one are dropped (one record plays once at any moment).

**Commands.**
`gunfinger-eval sweep --seed 2026`;
`gunfinger-eval scan stakka-skynet-knowledge [--leave-out 3|11 --seed 2026]`;
`gunfinger-eval calibrate`. Data: `data/0003-*.json`.

**Results.**

- Sweep (seed 2026; 209 indexed assets, 48 clusters held out; 60 indexed and
  20 held-out excerpts × 9 speeds, MP3 128k): recall 100% at every speed
  (60/60 each), 0 wrong answers. Speed error mean 0.002 to 0.004%, max
  0.014%. Wall time 2 min 40 s (renders cached).
- Development set: 11/11 identified, 0 wrong; 56:09 scanned in 30.6 s.
  Speeds +0.34% to +4.24%.
- Leave-out 3 (seed 2026: Night Lore, Side Effects, Kontempt): 8/11, 0 wrong.
  Leave-out 11 (every played track and its cluster out): 0/11, 0 wrong.
- Calibration over 559 identifying detections and 125,302 false candidates:
  weakest identifying 1,274 hits / 3 windows (sweep, SKC & Cord "Swarm" at
  -3%); strongest false 243 hits / 2 windows (the Stakka remix of Clockwork
  while the original plays, development set). Unrelated audio at most 69
  hits. Margin 5.24×; the threshold is 2.06× above the strongest false and
  2.55× below the weakest identifying.
- The sweep before calibration, under a provisional rule (40 hits), had 9
  wrong answers, all remix or chance chains of 61 to 225 hits.

**Conclusion.** The development bar is met on the first build of candidate
A; candidate B is not needed. Hits per excerpt are lowest for speeds between
rungs (±1, 3, 5%: medians about 2,000) and highest on rungs (0, ±8%: about
2,850), matching the survival curve of experiment 0001.
