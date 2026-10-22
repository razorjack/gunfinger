# 0170: where a NAS-scale scan's CPU goes on the Linux machine

**Question.** Session 36, step 2: with 0169's change, how does the search
split its CPU between its stages? This replaces the `perf` profile the
research note asks for and decides steps 3-5.

**Method.** A diagnostic build of 0169's code, never committed
(`work/s36/timers.diff`): each worker sums `Instant` intervals per call
and per part (no clock per hit) and adds them to a total when its thread
ends. The per-asset loop runs as two passes in this build, the pre-filter
for every asset of a part, then aligning, sorting and lines for those it
passed, so that each can be timed. The Stakka scan at NAS scale, 16, 32,
32, 16 jobs (`work/s36/timers.sh`), otherwise idle; every report identical
to session 33's apart from `wall_seconds`, and the scan's search time
45.3-45.5 s and 37.9 s, as 0169's build (no measurable cost).

| Share of the search's CPU | 16 jobs | 32 jobs |
|---|--:|--:|
| CPU in the search (first and second pass) | 668-670 s | 1,101-1,103 s |
| Query analysis (`points_near`: STFT, peaks) | 13.7% | 14.7-14.8% |
| Hashing and posting lookups | 0.3% | 0.3% |
| Taking parts (`take_part`) | 6.3% | 7.2% |
| Grouping a part by asset (`group_part`) | 28.4% | 23.7% |
| The pre-filter (`may_form_line`) | 43.1% | 46.1% |
| Aligning, sorting, `densest_clusters`, lines | 6.4% | 6.1% |
| The second pass (`refine`) | 1.8% | 1.8% |

Counts (both job counts): 1,105,360 parts, 49.8 G hits, 1.12 G assets
with 3 hits or more filtered, 24.6 M sorted, 3.48 M lines kept after
merging; chains 0.1 s; the first pass 41.6-41.7 s of wall time at 16 jobs
and 34.4-34.5 s at 32.

**Conclusion.** The pre-filter is the largest stage: 288 s of CPU for 49.8 G
hits at 16 jobs, about 5.8 ns of CPU a hit, reading each grouped hit and
its anchor's point. Taking and grouping are 35% at 16 jobs and 31% at 32,
above the 15% that step 4 requires, so grouping straight from the lists is
measured. Narrower grouped hits (step 3) reduce what both grouping and the
pre-filter read. Query analysis is 14-15%, less than the Mac's 25-30%
(0103): by inference, the per-hit stages cost more here. The second pass, lookups, sorting and
lines leave little to gain.
