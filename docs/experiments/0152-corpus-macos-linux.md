# 0152: the corpus on macOS and on Linux

**Question.** Session 32, 2026-10-18: indexed from scratch on the Linux
machine (x86-64, FFmpeg 8.1.2, Rust 1.98.0, `b97e3da`'s code), does the
corpus give the Mac's peaks, clusters and detections (arm64, FFmpeg 9.0.2;
`mac-reference/`)? `index` (265 files, 20 s), the store listings against
the Mac's (`store_compare.py`), `clusters`, `regress session-30-start` on
the new store (265) and on a copy without the 3 records the Mac's store
has skip notes for (262, `work/s32/peaks-262`).

| | Linux against the Mac |
|---|---|
| Records | 265 against 262; the 262 on both: same paths and sizes |
| Durations (decoded samples) | 262 of 262 equal |
| Peak counts; digests | 248 equal, 14 differ by 1-2 peaks; 65 equal, 197 differ |
| Clusters (262 or 265) | 17 of 17 equal; 47 of 47 pairs, same verdicts (at 262, 4 have 1 hit fewer) |
| Sweep (262) | 60/60 recalled, 0 wrong at each speed; 136 of 720 queries identical; no confident answer changes; hits of 586 confident detections change, median 0.66%, max 3.71% |
| Scan, leave-outs (262) | 11, 8, 0 identified, 0 wrong; 85 of 87, 92 of 93, 79 of 79 detections identical (1 hit; speed in the 7th digit) |
| Calibrate (262) | weakest identifying 680 -> 690, strongest false 99 -> 100 |

The 3 files the Mac skipped as damaged decode here to 443.8, 291.0 and
457.9 s, the lengths in the Mac's notes, with the same "Header missing"
error. The 1% tolerance (`53c6990`, 2026-10-08) accepts them; the Mac's
notes date from session 1's 1 s tolerance and stay until the files
change. At 265 they add only candidates of 6-8 hits.

**Cause.** A diagnostic build (uncommitted, `work/s32/diag/`) chose
rustfft's code path by an environment variable. On the same decoded
samples, AVX (the default here) and SSE give different peak digests for
143 of 265 files, AVX and scalar for 142; none matches the Mac's peaks for
more than 65 files. The Mac's rustfft takes its NEON path, so the FFT alone
explains differences at this rate. Whether FFmpeg decodes the same samples
on both is open: `decoded_digests.py` lists Linux's
(`data/0152-corpus-decoded-linux.tsv`) for the Mac to compare. The sweep's
queries are also encoded by FFmpeg, whose encoders differ by version.

**Conclusion.** Linux reproduces every score and cluster, not bit for bit.
Search is deterministic on each machine (`regress session-32-linux`
repeated: identical), so a baseline compares only on the machine that
made it, as ADR 0020 expected.
