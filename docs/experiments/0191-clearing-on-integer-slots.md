# 0191: clearing the counts on integer slots (exploratory)

**Question.** 0190 timed the fill (C5) and the stamps (C3) on `f64`
slots only. On C2's integer offsets, does either clear the counts 3%
faster than C4's re-reading in every pair at 32 workers (the rule set
in brief 41 at 04:20, before this round)? The round also repeats C4
against the start build.

**Change** (`work/s41/lines.rs.c2fill`, `lines.rs.c2stamp`; core tests
green): C2 with one 4 KiB fill after each asset; C2 with 8-bit stamps
in `u16` slots.

**Command.** `work/s41/ab.sh r2 s41-start s41-c4 s41-c2fill
s41-c2stamp`, as 0190 (the half labels now right). Data
`data/0191-ab-r2.tsv`.

| Search time (cut against start) | 32 jobs | 16 jobs |
|---|--:|--:|
| Start | 37.57, 37.94 s | 45.15, 45.38 s |
| C4 re-reading | 27.04, 27.19 (28.0%, 28.3%) | 32.58, 32.83 (27.8%, 27.7%) |
| C2 with one fill | 26.86, 26.94 (28.5%, 29.0%) | 32.88, 32.92 (27.2%, 27.4%) |
| C2 with stamps | 27.97, 27.98 (25.6%, 26.3%) | 33.48, 33.45 (25.8%, 26.3%) |

Every report identical to the start build's apart from `wall_seconds`.
User CPU at 32 jobs: start 1,087-1,099 s, C4 759-763 s, fill 753-756 s,
stamps 787-788 s.

**Conclusion.** The fill is 0.7-0.9% faster than C4 at 32 workers and
0.3-0.9% slower at 16; stamps are slower than both. Neither meets the
3% rule, so C4 is adopted as pre-registered. C4 repeats 0190's cut
(28.0-28.3% at 32 workers, 27.7-27.8% at 16).
