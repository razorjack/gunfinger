# 0180: a pitch ride inside the sweep's 30 s excerpts

**Question.** Session 38, step 3 (session 37's step 5): does a sustained
ride cost recall in short excerpts, not only time in long plays (0176),
and do the two opt-in overlap rules (0178, 0179) change that?

**Change.** `robust` gains four conditions run only when named in
`--only` (tested; its default run is unchanged): the speed rises by 1%,
2% or 3% over 3 s centred 15 s into the excerpt (each side spans two
10 s windows), or by 2% at 24 s (the side before spans three); up from
0.95 and 1.03. Design written before the runs (brief 08:26).

**Command.** `robust --seed 2026 --only` the five rides, at 262 tracks
(212 assets) and NAS scale (40,580), by default, `--overlap-by-chains`
and `--apart-in-speed 0.006`; 35-37 s and 3:12-3:14 per run.
`robust_rides.py`; data `data/0180-a-ride-inside-an-excerpt.json`.

| Indexed excerpts confident, of 80 (262 tracks; NAS scale) | Default | Chains | Apart 0.006 |
|---|--:|--:|--:|
| Control | 80; 80 | 80; 80 | 80; 80 |
| 1% at 15 s | 80; 80 | 80; 80 | 80; 80 |
| 2% at 15 s | 43; 48 | 43; 49 | 43; 49 |
| 3% at 15 s | 3; 5 | 6; 8 | 6; 8 |
| 2% at 24 s | 80; 80 | 80; 80 | 80; 80 |
| 2% over the whole excerpt (`pitch-ride-2pct`, beside) | 24; 46 | 24; 46 | 24; 46 |

- Every excerpt with a ride inside stays at least possible (79-80 of
  80); every miss has its best detection in fewer than 3 windows. Wrong
  confident: none at 262 tracks; at NAS scale 2 in the control and 2 at
  24 s, the same under every matcher. Median hits against the control:
  0.36-0.50 at 15 s, 0.54-0.75 at 24 s.

**Conclusion.** Yes, from 2%: a ride in the middle of a 30 s excerpt
splits it into two sides of two windows each, and the confidence rule's
3 windows, not the overlap rule, decides; 2% centred loses 40-46% of the
excerpts and 3% 90-96%, while 1% (by inference the chain still links
across the ramp on a neighbouring rung) and a ride late in the excerpt
cost nothing. Either variant wins back 1-3 excerpts. A short play with
a ride in its middle is a case for the window condition (the roadmap's
"A minimum span instead of 3 windows"), not for overlap.
