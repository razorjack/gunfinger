# 0212: where the query analysis's time goes, and the `x86-64-v2` build

**Question.** How does the first pass's query analysis (20-24% of its
CPU, 0205) split by stage and playback, how often is an analysis
repeated, and what do the baseline target's rounding calls cost the
whole scan (exploratory; decides nothing)?

**Method.** Timers and counts in a diagnostic build (`work/s46/diag.diff`,
never committed; five `Instant::now` calls a frame and two a candidate
inflate the neighbourhood test). The Stakka scan at NAS scale, 16 and 32
jobs, idle; reports identical to the start build's. Master's tree built
with `-C target-cpu=x86-64-v2` (SSE4.1: `roundss`/`roundsd`, 205 in the
binary, none in the start build) against the start build, A B B A at 32
and 16 jobs. Data `data/0212-analysis-timers.txt`.

| Share of the analysis | Turntable, 32 jobs (16) | Key-locked, 32 jobs (16) |
|---|--:|--:|
| Seconds summed over workers | 133.7 (74.0) | 45.0 (25.5) |
| FFT | 44.1% (45.1%) | 16.5% (17.8%) |
| Log power, `10 log10(|X|^2 + 1e-20)` per bin | 26.5% (24.3%) | 39.6% (36.5%) |
| Four-neighbour test | 16.6% (17.2%) | 24.9% (25.6%) |
| Neighbourhood test (timers included) | 10.3% (10.8%) | 15.3% (16.3%) |
| Windowing; row copy; refinement; plans; points | 1.5; 0.6; 0.3; 0.1; 0.01% | 2.3; 0.8; 0.4; 0.05; 0.1% |

Both playbacks at 32 jobs: FFT 37%, log power 30%, four-neighbour test
19%, neighbourhood test 12%. Counts, the same at both job counts: 1,189
turntable analyses (8.74 M frames), 609 key-locked (4.45 M) reused 580
times by the next rung of their hop; neighbourhood edges add 0.3% of
frames, block edges about 1%. A frame has 17.6 candidates and 0.61
peaks (so about two `round` calls); a neighbourhood test reads 1.85 rows.

**`x86-64-v2`.** Reports byte-identical. Search 26.84, 26.61 s against
26.32, 26.41 s at 32 jobs (0.8-1.9% slower, 0.8-0.9% more CPU); 32.23,
31.89 s against 31.74, 31.97 s at 16 (-1.6%, +0.2%).

**Conclusion.** The FFT is 44% of the turntable analysis at 32 workers
(0109: 75% on one thread) and 37% of all of it; log power and the
four-neighbour test together cost more. Plans, reused analyses and
rounding in the analysis are each under 1%. The rounding calls cost nothing measurable
now; `x86-64-v2` gives no speed to weigh in a platform decision.
