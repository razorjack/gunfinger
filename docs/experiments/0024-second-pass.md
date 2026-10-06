# 0024: the second pass at the fitted speed

**Question.** Analysing each candidate's span again at its fitted speed
(`search_twice`, opt-in) recovers the hashes lost between rungs. What does
it do to evidence, the null and recall, still under the frozen rule?

**Command.** `gunfinger-eval --second-pass` with `regress`, `calibrate`,
`mixes --count 12`, `grid` and `robust` (seed 2026, both ladders), against
the same without it; candidates with 10 hits or more are refined. Data:
`data/0024-second-pass.json`.

**Evidence and null.**

| Measure | Ladder | Second pass |
|---------|-------:|------------:|
| Sweep own hits, median at 0.2% from a rung (-5% … +5%) | 725-766 | 1,037-1,095 |
| Sweep own hits, median on a rung (-8%, 0, +8%) | 1,050-1,101 | the same |
| Weakest identifying; strongest false (the Clockwork remix) | 501; 97 | 680; 97 |
| Margin; strongest on audio not in the index | 5.16×; 19 | 7.01×; 20 |

Development set 11/11 either way, own hits +0 to +17% except Star Trails
(-15%), whose windows play 0.1-0.3% above its fitted speed. Mixes: 106
confident, 9 possible (107 and 8 before), strongest false 41 (46). Grid:
no confident 10 s play (8 before, all from chance lines 30 s away); 15 s
67 of 160 (76), 20 s 146 (148); a 10 s span would still give 160 and 160.

**Robust**, confident of 40 at -5% / +3% (ladder → second pass): needle
skip 0/1 → 18/15; broadcast 32/33 → 37/37; clipping 32/33 → 35/36;
beatmatched -6 dB 33/34 → 37/36, 0 dB 12/13 → 16/14; blend 0 dB 13/14 →
17/17; echo, noise 10 dB and blend -6 dB gain 0-2. No row loses; combined
stays 3/4. Wrong confident 0 everywhere; strongest wrong 108 (104).

**Cost.** Development mix: 310 candidates, 23,000 s analysed once beside
276,000 s of rungs (8% more); `regress` user CPU 2,425 s (2,400-2,453).

**Conclusions.** (1) The rung no longer shapes the evidence: excerpts
between rungs reach the hits of excerpts on one; false candidates do not
gain. (2) Under damage more plays reach the frozen rule; nothing wrong
does. (3) One speed per play loses where the speed wanders (Star
Trails). (4) The window grid is the rule's problem, not fixed here.
