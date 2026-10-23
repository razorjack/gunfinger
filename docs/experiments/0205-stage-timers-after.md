# 0205: where the search's CPU goes with pairs of slots

**Question.** After 0204, how does the first pass of the search split
its CPU, for the handoff (exploratory; decides nothing)?

**Method.** 0193's timers (session 41's `timers.diff`) on master's code
(`work/s43/timers.diff`, never committed), the per-asset loop as two
passes (the filter for every asset of a part, then aligning, sorting
and lines), which costs about 0.1-0.2 s of the search at 32 jobs (26.1-26.2 s
against 26.0 s). The Stakka scan at NAS scale, 32, 16, 16, 32 jobs,
otherwise idle; all four reports identical to the start build's apart
from `wall_seconds`. Data `data/0205-timers.txt` (fields as 0193's).

| Share of the first pass's CPU | 16 jobs | 32 jobs |
|---|--:|--:|
| CPU in the first pass (`in_windows`) | 446 s | 707-712 s |
| Query analysis (`points_near`) | 20.4% (91 s) | 23.5-23.6% (167-168 s) |
| Hashing, posting lookups, phases | 0.5% | 0.5% |
| Taking parts (`take_part`) | 10.5-10.7% (47-48 s) | 11.8% (83-84 s) |
| Grouping a part by asset (`group_part`) | 42.6-42.7% (190 s) | 35.6-35.7% (252-254 s) |
| The pre-filter | 16.2% (72 s) | 18.6-18.7% (132-133 s) |
| Aligning, sorting, `densest_clusters`, lines | 9.7% (43 s) | 9.9% (70 s) |

Against 0193 (master before 0204, the same method): the pre-filter 72 s
instead of 93 s of CPU at 16 jobs (1.45 ns a hit instead of 1.9) and
132-133 s instead of 164-166 s at 32; the other stages within 3 s of
0193's at 16 jobs, and at 32 grouping 8 s less, query analysis 4 s more. The scan's user CPU is 464 s at 16 jobs and 727-733 s at 32.

**Conclusion.** Grouping a part is still the largest stage (43% at 16
workers, 36% at 32), then query analysis (20-24%), the pre-filter
(16-19%), taking parts (11-12%) and sorting with lines (10%). Of
grouping, 0199 puts about four fifths in the scatter, whose cost per
hit at 32 workers neither rooms, staged lines nor ranks reduced
(0200-0202); ranks would cut 3-4% at 16 workers (0202).
