# 0109: FFT lengths of the turntable rungs (proposal 2, the diagnostic)

**Question.** Proposal 2: would windows within 1-2 samples of `round(1024
/ s)` make the first pass's turntable analysis 20% faster (stop point)?

**Method.** `work/perf/s19-fft` (rustc 1.98.0, RealFFT 3.5.0, RustFFT
6.4.1) times 60 s of the 2003 mix at each length and hop on one thread, 5
rounds: plan and window, windowing, FFT, log power, STFT, peak picking;
weighted by each rung's frames in the mix's NAS `identify` (instrumented).
Arms: today; nearest-even; the smallest largest prime factor of the
complex FFT within 1 and 2 samples (factor1, factor2); the fastest
measured within 1 and 2 (best1, best2: a ceiling fitted to its data).

**Lengths.** Today's complex FFT has a prime factor above 31 on 25 rungs,
above 100 on 16; nearest-even changes 17 lengths (24 and 13 left),
factor1 21 (12, 0), factor2 29 (6, 0). One `identify`: 984 first-pass
turntable analyses (7.18 M frames), 504 key-locked (3.66 M), 374 in
refinement; its 1,862 plans take 0.052 s of 128 s: no plan cache tried.

| Arm | Turntable rungs | With key-locked rungs | E2E user CPU, both / turntable |
|---|--:|--:|--:|
| today | 73.2 s | 86.6 s | 411 s / 250 s |
| nearest-even | -21.8% | -18.4% | |
| factor1 | -25.5% | -21.6% | |
| factor2 | -36.5% | -30.8% | -6.9% / -12.1% |
| best1 | -35.4% | -29.9% | |
| best2 | -44.0% | -37.2% | -8.7% / -14.4% |

FFT: 54.7 of today's 73.2 s; 985 (5 x 197) 22.8 us a frame, 1024 3.7.
Prime rank predicts time poorly: 949 (13 x 73) 14.7 us, 948 8.9; 1114 (2
x 557) 9.4, 1113 (3 x 7 x 53) 18.7. **End to end** (E2E: factor2 or best2
lengths in the first pass, timing only; `work/s19/fft-abba.sh`; contended,
load 22-157): `--playback both` A B C C B A 3 times, wall medians 48.8 s
(today), 46.2 (factor2, -5.4%, ranges overlap), 45.1 (best2, -7.6%); the
configured turntable playback twice: 30.9, 28.0 (-9.2%), 27.4 (-11.1%).
Memory unchanged; reports change (hits, boundaries, a possible play).

**Conclusion.** factor2 and best2 pass the stop point on both ladders,
factor1 narrowly, nearest-even not with key-locked rungs. Lengths
unchanged: a retrieval study is our decision. Data `data/0109-*`.
