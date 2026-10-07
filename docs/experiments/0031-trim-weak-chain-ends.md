# 0031: trimming weak chain ends against exact boundaries

**Question.** Item 6 of session 4: a detection's boundaries run from its
chain's first line to its last, so a weak chance line at either end
stretches them (0020). Does leaving weak end lines out of the boundaries
fix the overshoot, and what true coverage does it cost?

**Change** (commit afc731c, opt-in `--trim-ends`). The boundaries leave
out lines at either end of a chain with fewer hits than a quarter of the
chain's median line, keeping at least 2 windows; the evidence (hits,
windows, confidence) still counts every line. The second pass measures
the whole chain's span. Full protocol (`s4-protocol.sh`) under today's
matcher and skip at 240.

**Scoring.** The generated mixes (seed 2026, 12 mixes, 115 indexed plays
of 20-60 s, 4,510 s) record each play's exact start and end. Missed
coverage: seconds of a shown play outside its detections. Overshoot:
seconds they reach beyond it (`boundaries.py`, `boundary_delta.py`).

| Mixes | Today's | with trim | Skip at 240 | with trim |
|---|--:|--:|--:|--:|
| Overshoot; plays over 2 s | 86.0 s; 5 | 1.3 s; 0 | 7.6 s; 1 | 7.1 s; 1 |
| Missed coverage | 606.3 s (13.4%) | 825.2 s (18.3%) | 616.4 s (13.7%) | 837.7 s (18.6%) |
| Plays losing coverage; median, largest | | 72; 3.1, 8.2 s | | 71; 3.0, 7.8 s |
| Grid mean span at 20, 30, 40 s | 21.1, 31.1, 41.1 s | 19.4, 28.7, 38.5 s | 19.7, 29.7, 39.7 s | 19.2, 29.1, 39.1 s |

Detection levels are unchanged everywhere: sweeps 2026-2029 identical
(regress), development 11/11 and leave-outs 0 wrong, margins 4.15× and
5.53×, mixes 107/8 and 106/9, grid confident counts identical. For
comparison, 0030's link rules under today's matcher take 78.4 s of
overshoot off 5 plays at 22.2 s of coverage lost on 7 plays.
Data: `data/0031-trim-weak-chain-ends.json`.

**Conclusions.** (1) Trimming removes the overshoot under today's matcher
but cuts the true edges of 72 of 115 plays: 2.6 s of true play lost per
second of overshoot removed. A play's first and last windows are often
partial or under a crossfade, so they are weak for the same reason a
chance line is (inference). (2) Under skip at 240 there is almost no
overshoot left to remove. (3) The link rules (0030) fix the same
overshoot at a tenth of the cost; trimming is not worth adopting.
