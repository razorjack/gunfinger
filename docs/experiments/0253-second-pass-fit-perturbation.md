# 0253: does a better fit make the answer independent of discovery?

**Question** (`docs/brief-63.md`). In 0242, one posting list crossing the
cut of the lists set aside moved Echos's chain to another rung and its
second-pass count from 503 to 455. Under the grid (0251) and the bounded
fit (0252), do the same queries still move?
**Method.** `recall --queries clean,brief --set-aside-like` the pre-prune
saved index (hard link in `work/s36/w/indexes/`, read only) under the
default, the grid and the bounded fit, against each one's run on today's
lists (`second_pass_fit.py moved`); `data/0253-second-pass-fit-perturbation.json`.

| Under the old lists, against today's | Default | Grid | Bounded fit |
|---|--:|--:|--:|
| Clean (702): queries whose detections differ; right answer moved | 3; 0 | 3; 0 | 2; 0 |
| Brief (1,404): the same | 3; 2 | 3; 2 | 3; 1 |
| Confident answers changed (both sets) | 0 | 0 | 0 |
| Echos query 911: hits; fitted speed | 503 to 455; +0.933% to +1.147% | 603 to 602; +0.994% to +1.005% | 602 to 602; +0.998% to +0.998% |
| Echos query 910 | 386 to 386; +1.056% to +1.0565% | 457 to 457 | 455 to 455 |

- The brief queries that move are Echos's, 901, 910 and 911, under every
  fit; 901 (-8%) moves a weaker detection only. Under the bounded fit only
  911's speed moves (by 0.0003%), and its count does not.
- In the clean queries that move, the best right detection's count and speed
  stay the same under every fit; weaker candidates move.
- The runs took 23:51-24:45 each, three at once on the Linux machine.

**Against the ADR bar.** B6 passes for the bounded fit: no more moved
queries than the default (5 against 6), and fewer right answers moved
(1 against 2).

**Conclusion.** The chain's speed is what makes a short chain's count
depend on which rung kept a window: both fits remove that dependence for
Echos (48 hits apart under the default, 0-1 under the fits). The lists set
aside still move weak candidates, as ADR 0014 expects. This does not
outweigh 0252's failures (B2, B4 at 262 tracks).
