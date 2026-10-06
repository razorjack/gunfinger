# 0016: both ladders through the full protocol

**Question.** Experiment 0010 offered key-locked rungs as an option. Does
searching turntable and key-locked rungs together pass the full protocol
needed to become the default, and what does it cost?

**Command.** `gunfinger-eval --ladder both regress session-2-start`
(sweep, development scan, both leave-outs, against the turntable-only
baseline), then `--ladder both calibrate`. Data:
`data/0016-key-lock-full-protocol.json`.

**Results.**

| Measurement | Turntable (baseline) | Both ladders |
|-------------|----------------------|--------------|
| Sweep, 9 speeds × 60 | 100%, 0 wrong, error ≤ 0.016% | 100%, 0 wrong, error ≤ 0.016% |
| Development set | 11/11, 0 wrong | 11/11, 0 wrong |
| Leave-out 3, 11 | 8/11, 0/11, 0 wrong | 8/11, 0/11, 0 wrong |
| Weakest identifying | 501 hits | 501 hits |
| Strongest false (remix) | 95 hits, margin 5.27× | 97 hits, margin 5.16× |
| Audio not in the index | 19 hits | 19 hits |
| Strongest false under 30 hits | 28 hits | 28 hits |
| Detections, development mix | 4,181 | 6,114 |

Every credited detection is unchanged except Bios-Fear (4,542 → 4,545
hits, one more window) and Side Effects (2,237 hits over 31 windows →
2,241 over 29, starting 23 s later). Of the new weak candidates none
reaches the possible tier; the strongest is the Clockwork remix (51).

Cost: the development scan took 34.0 s with both ladders against 25.6 s
(medians of three interleaved rounds on a loaded machine; fastest 32.2 and
25.4 s), about 1.3 times, less than the 82 rungs instead of 41 suggest;
the timings are noisy and the reason was not measured.

**Conclusions.** Both ladders pass the protocol with unchanged thresholds:
the default could switch without recalibration. It stays on the turntable
ladder because the owner's sets are vinyl and the search takes about a
third longer; `playback = "both"` in the configuration file makes it the
default per library.
