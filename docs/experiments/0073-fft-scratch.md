# 0073: Reusing FFT scratch buffers

**Hypothesis.** RealFFT allocates scratch on every `process` call; allocating
once per spectrogram reduces cost without changing spectra or peaks.

**Method.** `work/perf/fft.rs`, built with `rustc -O` against release RealFFT,
compares the current loop with `process_with_scratch` on seed-2026 draw 0
at native speed. Twelve alternating pairs, three 30-second analyses per
sample; every spectrum compared exactly before timing. Other work was active.
Command: `work/perf/fft > work/logs/perf-fft.log`.

**Measured medians (seconds per three analyses).**

| FFT size | Per-frame allocation | Reused scratch | Ratio |
|--:|--:|--:|--:|
| 1024 | 0.032283 | 0.016849 | 0.522 |
| 985 | 0.347125 | 0.369398 | 1.064 |
| 1113 | 0.371784 | 0.224277 | 0.603 |

**Conclusion.** Spectra identical. Keep scratch reuse, which removes a
per-frame allocation and improved two of three sizes; the contended wall
timings are not an end-to-end speed claim. Keep plan/window construction
local for now; cross-call caching needs a separate memory and timing case.
Full pre-release regression: all detections identical (experiment 0072).
