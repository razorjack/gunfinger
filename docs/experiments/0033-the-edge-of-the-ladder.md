# 0033: the edge of the ladder, ±8.2% to ±9%, and 3 extra rungs

**Question.** Item 7 of session 4: the ladders end at ±8% (rungs 0.4%
apart); recall was 100% at ±8%, 0% at ±10%. Where does it fall, and what
do a few more rungs cost?

**Method** (commit 804c65c). `robust --only` the 10 speeds from ±8.2% to
±9% (40 indexed and 10 held-out excerpts each, seed 2026's panel, both
ladders), with today's ladders and with `--extra-rungs 3` (to ±9.2%, 94
rungs instead of 82), under today's matcher and skip at 240. Cost: the
development scan, three interleaved rounds each, nothing else running.

| Confident of 40 | -9% | -8.8% | -8.6% | -8.4% | -8.2% | +8.2% | +8.4% | +8.6% | +8.8% | +9% |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| Today's matcher | 0 | 12 | 39 | 40 | 40 | 40 | 40 | 40 | 35 | 0 |
| Skip at 240 | 7 | 35 | 40 | 40 | 40 | 40 | 40 | 40 | 38 | 11 |
| Either, 3 extra rungs | 40 | 40 | 40 | 40 | 40 | 40 | 40 | 40 | 40 | 40 |

No wrong answer; at ±9% without extra rungs 34-39 of 40 are possible.
With them the strongest wrong is 98 and 105 hits, as on the ladder (93, 100).

| Development scan, 3 rounds | Wall | CPU | Detections |
|---|--:|--:|--:|
| Today's matcher | 32.3 s [31.8-32.8] | 268 s [267-268] | 6,114 |
| with 3 extra rungs | 36.9 s [36.4-36.9] | 306 s [306-307] | 6,527 |
| Skip at 240 | 29.1 s [29.1-29.2] | 243 s [242-244] | 232 |
| with 3 extra rungs | 33.3 s [33.0-34.0] | 278 s [277-278] | 244 |

Protocol with the extra rungs: sweeps 2026-2029 identical in recall,
weakest identifying and strongest false under both matchers (margins
4.15× and 5.53×); development 11/11, leave-outs 0 wrong; mixes identical
play for play; grid identical (today's matcher; not run under skip).
False candidates 70,506 to 78,547 (today's) and 4,503 to 4,862 (skip);
at 30 hits or more 55 to 58 and 41 to 42.

**Conclusions.** (1) Recall holds 0.4-0.6% past the last rung and is gone
1% past it; skip's second pass reaches further. (2) Three rungs at each
end recover every edge speed for 14% more search CPU at 262 assets (in
proportion to rungs at scale, by inference) and change nothing else
measured. Data: `data/0033-the-edge-of-the-ladder.json`.
