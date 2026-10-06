# 0022: where the development mix loses evidence; new robust conditions

**Question.** Of the reference hashes a play could match, how many does
the development mix keep, and what loses the rest: the mix itself, blends,
the rung, or speed varying within a play? How do wow, broadcast
processing and beatmatched blends compare?

**Command.** `gunfinger-eval loss` (new): the 11 identified turntable
plays, per 10 s window, hits on the play's line against the reference
hashes whose anchor lies in the stretch heard; mix on the nearest rung, at
the fitted speed and at the best speed within ±0.3%; a clean render of the
same stretch at the fitted speed as control. `robust --only` the new
conditions (seed 2026, both ladders). Data:
`data/0022-evidence-loss-and-new-conditions.json`.

**Loss** (share of reference hashes kept; 390 windows, 107 blended).

| Windows | Mix: rung | fitted | local | Clean: rung | fitted |
|---------|----------:|-------:|------:|------------:|-------:|
| all | 13.0% | 13.7% | 14.5% | 47.6% | 63.0% |
| solo / blended | 16.0 / 4.3% | 17.0 / 4.3% | 17.8 / 4.9% | 46.6 / 50.4% | 63.0 / 63.1% |
| rung 0.1-0.2% off | 13.2% | 14.2% | 15.0% | 42.6% | 64.9% |

By anchor band (47% of reference hashes at 2-4 kHz, 27% at 1-2 kHz): mix
36% below 250 Hz, 21% at 0.5-1, 13% at 1-2, 6% at 2-4 kHz; clean 82-54%.

**Robust** (confident of 40 at -5% / +3%; possible+ in brackets). Wow
0.55 and 0.75 Hz: 40/40 both, retention 89-102%. Broadcast: 32/33 (39/40),
retention 34-38%. Beatmatched partner at -6 dB: 33/34 (40/40); at 0 dB:
12/13 (30/32); unmatched blends were 34/36 and 13/14. Combined (wow,
beatmatched -6 dB, broadcast, AAC 64k): 3/4 (32/32), retention 12%. No
wrong answer; strongest wrong 104 hits (wow, -5%).

**Conclusions.** (1) The mix itself (vinyl, mastering, encoding) costs
most: solo windows keep a third of what a clean render keeps on the rung,
least in the top two octaves, where most reference hashes sit. (2) A
blend costs another three quarters. (3) The rung costs a third of clean
hashes at 0.1-0.2% off but 7% in the mix; the fitted speed recovers 5%,
local speed 6% more: a second pass helps clean audio far more than this
mix. (4) Beatmatching costs no more than an unmatched blend.
