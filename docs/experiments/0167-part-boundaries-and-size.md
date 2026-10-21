# 0167: part boundaries from the front, and smaller parts, on the Linux machine

**Question.** 2026-10-21, between sessions: does finding each part's boundary
from the front of a list, with parts of 1,024 assets instead of 4,096, take
away the 32-worker penalty of 0158 without changing detections?
`docs/research/2026-10-21-low-bandwidth-search.md`, items 1 and 2.

**Change** (not in master; `work/perf-design/verify/lines.diff`). In
`search/lines.rs`, `take_part` finds the end of a part with a doubling search
from the front of the list, then `partition_point` between the last two
probes, instead of `partition_point` over the whole remainder;
`ASSETS_PER_PART` 1,024. Same slices, same hits, same order.

**Method.** The Stakka mix at NAS scale (40,683 assets), `scan
stakka-skynet-knowledge`, saved index, `throughput-performance`, machine
otherwise idle. A design agent measured eight variants (five runs of the
current code); the check below is a clean copy of `f7f2e1a` with the change,
against `work/bin/s35-final`, run once each in the order shown.

| Build | Jobs | Search | Wall | User CPU | Peak |
|---|--:|--:|--:|--:|--:|
| Current | 32 | 76.7 s | 80.1 s | 2,224 s | 5.07 GiB |
| Changed | 32 | 38.5 s | 42.0 s | 1,096 s | 4.92 GiB |
| Changed | 16 | 45.7 s | 49.2 s | 677 s | 4.84 GiB |
| Current | 16 | 49.3 s | 52.7 s | 735 s | 4.93 GiB |

- The four reports are identical apart from `wall_seconds`.
- The agent's runs agree (current 48.9-50.1 s at 16 workers, 75.9-82.3 s at
  32; changed 46.1-46.4 s and 38.6-38.7 s). Parts of 2,048: 46.2-46.5 s and
  40.0-40.3 s with the front search, 56.9-58.6 s at 16 with the binary search.
- `clusters --from-peaks --sample 48` at 32 jobs, the index built first:
  210.9 s, 5,158 s user (current) against 120.9 s, 2,515 s user (changed);
  the sample reports byte-identical (the agent's runs, not repeated).
- Why, by inference: smaller parts keep each worker's hit buffers in its L3
  share; the front search stops reading lines that later parts fetch again.

**Conclusion.** The change halves the search at 32 workers (2.0 times) and
saves 7% at 16, with identical reports; 32 workers become faster than 16, so
the default worker count need not change. To adopt: a session with tests,
the development scans and leave-outs, the clusters sample and `regress`.
