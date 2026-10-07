# 0032: the second pass fitting the speed per stretch

**Question.** Item 4 of session 4: the second pass measures a play at one
fitted speed, and Star Trails plays 0.1-0.3% off it in many windows
(0024). Does fitting each stretch of a few windows recover the hits,
what does it cost, and does it change anything else?

**Change** (commit afc731c, opt-in `--speed-per-stretch`, second pass
only). After the fitted measurement, each stretch of 3 windows gets a
local speed and alignment from the drift of its hits (and the window
either side); if that speed is 0.05% or more from the fitted one, the
stretch is analysed again at it, 100 frames of margin either side, and
the new measurement replaces the first. Measured under skip at 240 with
the full protocol and `robust --only` for wow, the pitch ride and blends.

| Skip at 240 | without | with stretch |
|---|--:|--:|
| Sweeps 2026-2029: recall; weakest identifying | 2,160/2,160; 658 | same |
| Strongest false: overall; sweep 2026; margin | 119; 99; 5.53× | 119; 109; 5.53× |
| Development: Star Trails hits (11/11, 0 wrong) | 1,558 | 1,733 (+11%) |
| Development: Dominion; Pathogen; Kontempt | 3,347; 2,078; 4,452 | -3.2%; -1.3%; +0.7% |
| Mixes: confident, possible; strongest false | 106, 9; 42 | 106, 9; 50 |
| Mixes: plays with more hits; all hits | | 15 of 115, up to 1.66×; +1.4% |
| Robust pitch ride 2% (80 indexed): hits | 41,486 | 45,255 (+9.1%; 43 up, 7 down) |
| Robust wow 0.55 and 0.75 Hz, blends, beatmatched | | identical hits |
| Development scan, 3 rounds: wall; CPU | 29.9 s; 241 s [241-243] | 29.5 s; 241 s [241-241] |

The 15 mix plays that gain are 20-36 s long at constant speed (the mixes
have no drift): with 2-4 windows the fitted speed and alignment are
uncertain, and each stretch's own fit finds more (inference). Wow swings
±0.2% within 1.3-1.8 s, far inside a 30 s stretch, so a stretch's drift
averages it out and nothing is measured again. Leave-outs 3 and 11: 0 wrong; grid
counts unchanged. Data: `data/0032-speed-per-stretch.json`.

**Conclusions.** (1) Following the speed recovers hits where it drifts
slowly: Star Trails +11%, the pitch ride +9%. (2) Replacing a stretch's
measurement can also lose hits (Dominion -3.2%), and false candidates
gain too (42 to 50 in the mixes). (3) The extra analysis costs nothing
measurable. (4) No detection changes level in any set measured here; it
would matter only for plays near the 240-hit rule.
