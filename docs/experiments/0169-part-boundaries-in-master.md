# 0169: part boundaries from the front, parts of 1,024 assets, in master

**Question.** Session 36, step 1: adopt 0167's change (the research
note's items 1 and 2) with tests and the part size measured at 3600 MT/s.

**Change** (`search/lines.rs`). `front_partition` finds a part's end in
each posting list by doubling from the front, then `partition_point`
between the last two probes; `ASSETS_PER_PART` 4,096 to 1,024. Same
slices, so the same hits in the same order. Two tests compare it with
`partition_point` (2,000 random sorted lists; empty lists; ends before the
first asset, at the last, past it; runs of one asset).

**Part size** (`work/s36/ab-parts.sh`): the Stakka scan at NAS scale
(40,683 assets), interleaved, two rounds, otherwise idle.

| Build | Search, 16 jobs | User CPU | Search, 32 jobs | User CPU |
|---|--:|--:|--:|--:|
| `s35-final` (4,096, binary search) | 47.2, 47.2 s | 703 s | 49.0, 49.2 s | 1,441-1,446 s |
| 1,024 | 45.4, 45.3 s | 674-675 s | 37.9, 38.0 s | 1,095-1,099 s |
| 2,048 | 45.6, 45.5 s | 675-677 s | 38.3, 38.8 s | 1,111-1,119 s |
| hit budget | 44.1, 44.1 s | 655 s | 37.0, 37.1 s | 1,072-1,073 s |

The hit budget sizes each window's parts (the next power of two of its
hits times 16 bytes over 1.5 MiB; 256-4,096 assets; `work/s36/budget.diff`):
2.7% faster than 1,024 at 16 jobs, 2.2% at 32, below the design report's
3% stop point, so 1,024 is kept. Reports identical to session 33's apart
from `wall_seconds`; peak memory 4.82-4.93 GiB (4.87-5.10 GiB before).

**Validation** (`work/s36/validate.sh`, `full.sh`): `scripts/check.sh`
green (rerun once after ADR 0020's "Text file busy" test race); both
regressions identical, every report they wrote (sweeps at seed 2026
included) identical to its baseline's apart from `wall_seconds`; the 11
development reports at NAS scale (0157's scans and leave-outs) identical to
session 33's; the clusters sample identical to 0168's (1:58 at 32 jobs,
0168's old code 165 s); the full NAS clusters run (session 33's run 2,
32 jobs) byte-identical to session 33's file, in 23:02 (40,613 s user,
7.2 GiB) against 52:30 at 2133 MT/s with the old code.

**Conclusion.** Kept: at 3600 MT/s the scan's search takes 3.9% less time
at 16 jobs, 23% less at 32; 32 jobs now beat 16. `work/s36/ab-parts.tsv`.
