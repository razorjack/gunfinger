# 0189: where the pre-filter's time per hit goes on the Linux machine

**Question.** Session 41, step 1: what do the pre-filter's parts cost
per hit (43-46% of the search's CPU, 0170); how are hits spread?

**Method.** A diagnostic build, never committed (`work/s41/diag.diff`):
per part, after grouping, seven timed passes over its assets with 3
hits or more, in an order rotating by part, each adding one piece of
today's filter; then the real search, unchanged. The Stakka scan at NAS
scale, 32 then 16 jobs, otherwise idle; both reports identical to the
start build's apart from `wall_seconds`.

| Pass (CPU per hit in a filtered asset) | 32 jobs | 16 jobs |
|---|--:|--:|
| Loads of the grouped hits | 0.70 ns | 0.43 ns |
| + the anchor's point and the `f64` offset | 1.46 ns | 0.80 ns |
| + the slot: `/ 4`, `floor`, casts, modulo | 6.00 ns | 2.95 ns |
| Today's filter | 9.97 ns | 5.86 ns |
| Today's, cleared by one 4 KiB fill, no `touched` | 9.47 ns | 5.02 ns |
| The fill alone | 1.07 ns | 0.75 ns |
| Today's without the early exit (saturating counts; not used) | 12.04 ns | 6.63 ns |

Of today's filter at 16 (32) jobs: loads 14% (15%), slot arithmetic 37%
(46%), `touched` and its clearing 27% (16%; 5.86 - (5.02 - 0.75) = 1.59
ns at 16), counting and the test 23% (24%). On baseline x86-64
`f64::floor` is an indirect call (`callq *floor@GOTPCREL(%rip)`, no
SSE4.1); the Mac has one instruction, by inference why 0108 saved less.

**Counts** (both job counts): 49.81 G hits, 1,105,360 parts; 1.122 G
filtered assets hold 49.80 G (44 each on average; 17-32 hits 27.8%,
33-64 50.2%, 65-128 14.3%, fewer than 17 6.6%); 24.6 M pass (2.19%),
with 93 hits on average. The filter reads 49.12 G hits: its early exit
spares 1.4%; a passing asset passes late (22.0% in the last tenth of
its hits, 0.4% in the first; only these tenths of the passing index are
valid, its histogram shared two counters).

**Conclusion.** The slot arithmetic, with its `floor` call, and the
`touched` list are 62-64% of
the filter. A cheap first test cannot help: nearly every asset is read
to its end. Candidates: brief 41's pre-registration (C1-C5, G1).
