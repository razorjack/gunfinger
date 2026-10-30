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

The chain's speed costs hits mostly where the chain is short. On 30 s
excerpts (the sweeps, `recall`'s clean excerpts, their other rips) the
chain's fit is already within 0.003% of the played speed (median), few
confident answers change, all against other rips (grid +3, split -2,
bounded fit +1, over the sets below), and the median hit ratio of weak
true plays is 0.99-1.01. On `recall`'s 15 s brief
plays, whose chains are too short for a fit, the speed is the mean of
the lines' rungs: 0.049% off (median), 0.012-0.014% under the grid or
the split and 0.004% under the bounded fit. There the three fits gain
the same 5 own-file answers and lose the same one; against other rips
the grid gains 22, the split and the bounded fit 13 each (11 of them the
same queries):

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

Maximising over 42 alignments raises wrong versions as it raises true
plays; the split and the bounded fit, which do not choose on the hits
they count, mostly do not:

| Negatives (NAS scale unless 262) | Grid | Split | Bounded fit |
|---|--:|--:|--:|
| Wrong candidates of 80 hits or more, median hit ratio | 1.02-1.08 | 0.94-1.00 | 0.97-1.00 |
| The same, 90th percentile | 1.15-1.30 | 1.02-1.24 | 1.01-1.26 |
| Calibrate's strongest false, 262 tracks (default 100) | 110 | 100 | 107 |
| Calibrate's strongest false, NAS scale (default 282) | 288 | 279 | 282 |
| Panel 1, `related` and `other` newly confident; no longer | 15; 0 | 4; 15 | 0; 3 |
| Panel 2, the same | 12; 0 | 12; 0 | 0; 0 |
| Wrong confident answers added (sweeps, `recall`, scans) | 0 | 0 | 0 |

By inference, the split's additions are not selection: in panel 2 a related version
(Phantom 2018, which shares the played passage) goes from 223 to 244
hits under the grid and the split alike, at 9 query and index pairs, as
a true play does when its alignment improves. No fit can tell such a
passage from the record it was taken from; it can only move it across
the rule. The bounded fit leaves it below. At 262 tracks the Clockwork
remix's passage that the original shares (calibrate's strongest false)
goes from 100 to 107 hits under the bounded fit, and the margin from
6.90× to 6.44×.

## Discovery's perturbation

Searched with the lists the pre-prune index set aside (0242's
perturbation, `recall --set-aside-like`), the default moves Echos's
query 911 from 503 hits at +0.933% to 455 at +1.147%; the grid gives
603 and 602, the bounded fit 602 and 602 (+0.998% both). No confident
answer changes under any fit, and the bounded fit moves 5 queries'
detections against the default's 6 (0253). By inference from Echos, the
one query whose count moves, a better fit removes a short chain's
dependence on which rung's line a window kept; the
lists set aside still move weaker candidates, as ADR 0014 expects.

## Cost

The Stakka scan at NAS scale, alone on the Linux machine (32 workers):
search 24.17 s for the default (twice), 24.55-24.56 s for the bounded
fit (+1.6%; user CPU 677-678 s against 666-668 s), and 37.33 s for the
grid and for the split (+54%; 1,014 s), which analyse each candidate of
80 hits or more 41 more times. The bounded fit passes the registered
cost bound (B5, at most 3%).

## Recommendation

Keep the chain's speed. None of the three fits is worth an ADR proposal
as registered (0252: the bounded fit fails B2 and B4; the grid and the
split were never candidates, being a reference and a diagnostic).

What the measurements say, for a later decision that is ours:

- The loss is mostly on short chains, where `chain_speed` falls back to
  the hit-weighted mean of the rungs. There the fits come 3.6 (grid) to
  11 (bounded fit) times closer to the played speed and gain confident
  answers, mostly against other rips. On chains long enough for the fit
  through line centres, little is gained (1-3 answers per fit, against
  other rips); the grid and the split add noise to the speed there, and
  the bounded fit halves its error.
- By inference the gain is mostly real, not selection: the split keeps
  18 of the grid's 27 gained answers, though about a third of its median
  hit gain. So it raises shared material as well: a related version that
  shares the played passage crosses 240 under the split as a true play
  does. Only a fit that does not move such passages much (the bounded
  fit, in these panels) leaves the threshold's measured meaning roughly
  intact, and even it lifts the Clockwork remix's shared passage by 7%.
- The answers that move are near the rule in both directions: the
  bounded fit gains 19 and loses 4 (2 of them one played file at two
  indexes). A change here is a change to which weak plays are
  confident, not a recovery without cost.

The fits stay in the tree as the opt-in harness diagnostic the session
was asked to build (`--second-pass-fit`, `search/refine/fit.rs`), because
the grid and the split are the reference against which a short-chain fit
would be measured. They are to be deleted when that measurement is
decided, or if it is not taken up.

If this is taken further, the candidate is the bounded fit applied only
where `chain_speed` falls back to the mean of rungs (chains under 3
lines or under the minimum fit span), measured with the same panels and
calibrate at both scales; it belongs beside the minimum-span question
(the roadmap's "A minimum span instead of 3 windows"), since 15-20 s
plays are where both act.

