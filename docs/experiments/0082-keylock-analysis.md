# 0082: Share analysis between key-locked rungs with equal hops

**Hypothesis.** The default 41 key-locked rungs use only 21 rounded hops.
Their raw peaks are identical for equal hops; only time scaling differs.

**Method.** `work/perf/keylock.rs`, `rustc -O` against the release core;
`work/perf/keylock > work/logs/perf-keylock.log`. Eight alternating pairs
on seed-2026 draw 0 at native speed. Compare every point of every rung.

**Result.** Identical points. Median ladder extraction 0.934457 s with
repeated extraction versus 0.519674 s grouped by hop (0.556 times wall time).
Other work was active; this measures extraction, not the complete search.

**Implementation.** Adjacent key-locked rungs with one hop share a worker
job and one block's peak buffer. Reuse only when the cached frame range
contains the requested range; filter to that rung's original frame range
before applying its original time scale. No global cache, locking or
persistent growth; progress still counts individual rungs. Peak queries
keep their existing coordinate transform.

**Verification.** Whole-query and blocked searches agree, including
adjacent equal-hop rungs and both sample and peak queries. The full standard
detection regression is identical (experiment 0077).
