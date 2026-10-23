# 0193: where the search's CPU goes with integer offsets

**Question.** After 0192, how does the first pass of the search split
its CPU, for the handoff (exploratory; decides nothing)?

**Method.** 0170's timers on master's code (`work/s41/timers.diff`,
never committed): per call and per part, summed per worker; the
per-asset loop runs as two passes (the filter for every asset of a
part, then aligning, sorting and lines), which costs about 2% of the
search (27.2-27.4 s against 26.7 s at 32 jobs). The Stakka scan at NAS
scale, 32, 16, 16, 32 jobs, otherwise idle; all four reports identical
to the start build's apart from `wall_seconds`. Data
`data/0193-timers.txt` (its "wall" is the first pass's; the numbers:
analysis, `in_windows`, take, group, filter, sort and lines, phases;
hashing and lookups are the remainder of `in_windows`).

| Share of the first pass's CPU | 16 jobs | 32 jobs |
|---|--:|--:|
| CPU in the first pass (`in_windows`) | 463-464 s | 745-750 s |
| Query analysis (`points_near`) | 19.5% (90 s) | 21.9% (163-164 s) |
| Phases (`Phases::find`) | 0.01% | 0.01% |
| Hashing and posting lookups | 0.5% | 0.5% |
| Taking parts (`take_part`, with the integer offsets) | 10.2-10.4% | 11.1% |
| Grouping a part by asset (`group_part`) | 40.4-40.5% (188 s) | 34.9-35.0% (260-262 s) |
| The pre-filter | 20.1% (93 s) | 22.1% (164-166 s) |
| Aligning, sorting, `densest_clusters`, lines | 9.2% | 9.5% |

Against 0170's timer build (session 36, 16 jobs; another build): the
pre-filter 93 s instead of 288 s of CPU (1.9 ns a hit instead of 5.8),
taking parts 47-48 s instead of 42 s, grouping 188 s instead of 190 s,
query analysis 90 s instead of 92 s. The whole scan's user CPU is
481-482 s at 16 jobs and 764-770 s at 32; the first pass is 96-97% of it.

**Conclusion.** Grouping a part is now the largest stage (40% at 16
workers, 35% at 32), then the pre-filter and query analysis (20-22%
each), taking parts (10-11%) and sorting with lines (9%). By inference
from 0170, storing the integer offsets costs `take_part` about 5 s. Grouping's
counting sort writes each hit to one of 1,024 places and reads it back;
0171 and 0172 found narrower or list-direct forms slower, and G1 (0190)
counting while taking no faster: what is left there needs another idea.
