# The second pass's speed estimate: what a better fit gains (2026-10-30)

**Status: measured (session 63, experiments 0251-0253); not adopted.**
The second pass, its default and the product are unchanged. The fits
are opt-in in the core's options and set only by the harness
(`--second-pass-fit`); this note gives our recommendation for whether
to take one further. It answers proposal 3 of
[the proposals note](2026-10-29-matching-and-archive-proposals.md).

## What was measured

The second pass measures a candidate at the chain's speed (`chain_speed`:
a fit through the lines' centres when the chain has 3 lines or more over
enough span, otherwise the hit-weighted mean of their rungs). Three
alternatives, each for candidates of 80 hits or more at the chain's
speed, on the same span and pivot, under today's rule:

- **The grid** (the reference): the best of the chain's measurement and
  41 speeds within ±0.4% of it in 0.02% steps, the offset refitted at
  each. It maximises over 42 alignments.
- **The split**: the same 42 alignments, chosen on alternate 10 s windows
  and counted on the others, for each half. It has the grid's freedom
  without choosing on the hits it counts.
- **The bounded fit**: at most two rounds of the hit-weighted drift of the
  measured windows' deviations (the fit `--speed-per-stretch` uses for a
  stretch), each within ±0.4% of the chain's speed, keeping the last
  round whatever its count.

## The gain

The chain's speed costs hits only where the chain is short. On 30 s
excerpts (the sweeps, `recall`'s clean excerpts, their other rips) the
chain's fit is already within 0.003% of the played speed (median), no
confident answer changes under any fit at either scale, and the median
hit ratio of weak true plays is 0.99-1.01. On `recall`'s 15 s brief
plays, whose chains are too short for a fit, the speed is the mean of
the lines' rungs: 0.049% off (median), 0.012-0.014% under the grid or
the split and 0.004% under the bounded fit. There the three fits find
the same answers:

| Positives, confident right (NAS scale) | Default | Grid | Split | Bounded fit |
|---|--:|--:|--:|--:|
| `recall` brief plays | 721 | +5 -1 | +5 -1 | +5 -1 |
| `recall` brief plays, other rips only | 171 | +22 | +13 -2 | +13 -1 |
| `recall` clean, other rips only | 174 | +2 | = | +1 |
| `sweep --other-rips` (with one extra rung pair) | 408 (407) | = (+1) | -1 (-1) | = (=) |
| Every other sweep and `recall` set; scans' tracks | | = | = | = |

Most changed answers cross 240 hits in 3 windows; on the brief plays 2
gain and 1 loses a third window. The named cases: Echos (query 911)
goes from 503 hits at +0.933% to 597-603 at +0.99-1.00% under each fit
(the truth is +1%). Siren Bass's upload at -8% is confident by the
default at today's ladder (305 hits at -8.09%); with one extra rung pair
(0226's loss, 214 hits at -8.18%) the grid reaches 241 hits at -8.25%,
the split 237 and the bounded fit 230 at -8.09%: the count at a better
speed sits at the rule, not clearly above it.

## The selection effect on negatives

TODO

## Discovery's perturbation

TODO

## Cost

TODO

## Recommendation

TODO
