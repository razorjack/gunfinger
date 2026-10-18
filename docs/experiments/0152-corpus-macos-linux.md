# 0152: the corpus on macOS and on Linux

**Question.** Session 32, 2026-10-18: indexed from scratch on the Linux
machine (x86-64, FFmpeg 8.1.2, Rust 1.98.0, `b97e3da`'s code), does the
corpus give the Mac's peaks, clusters and detections (arm64, FFmpeg 9.0.2)?
`index` (265 files, 20 s), `store_compare.py`, `clusters`
(`clusters_compare.py`), `regress session-30-start` (`sweep_compare.py`,
`scan_compare.py`) on the new store (265) and on a copy without the 3
records the Mac's store has skip notes for (262, `work/s32/peaks-262`).

| | the Linux machine against the Mac |
|---|---|
| Records | 265 against 262; the 262 on both: same paths and sizes |
| Durations | 262 of 262 equal to the millisecond |
| Peak counts; digests | 248 equal, 14 differ by 1-2 peaks; 65 equal, 197 differ |
| Clusters (262 or 265) | 17 of 17 equal; 47 of 47 pairs, same verdicts (at 262, 4 have 1 hit fewer) |
| Sweep (262) | 60/60 recalled, 0 wrong at each speed; 136 of 720 queries identical, 83 gain or lose candidates; no confident answer changes; hits of 586 confident detections change, median 0.66%, max 3.71% |
| Scan, leave-outs (262) | 11, 8, 0 identified, 0 wrong; 85 of 87, 92 of 93, 79 of 79 detections identical (1 hit; speed in the 7th digit) |
| Calibrate (262) | weakest identifying 680 -> 690, strongest false 99 -> 100 |

The 3 files the Mac skipped as damaged decode on Linux to 443.8, 291.0 and
457.9 s, the lengths in the Mac's notes, with the same "Header missing"
error. The 1% tolerance (`53c6990`, 2026-10-08) accepts them; the Mac's
notes date from session 1's 1 s tolerance and stay until the files change.
At 265: 4 more candidates of 6-8 hits and a few 6-hit ones swapped in the
scans, no score changed; 31 of 47 cluster pairs move by -14 to +9 hits.

**Cause, in part.** A diagnostic build (uncommitted, `work/s32/diag/`)
chose rustfft's code path by an environment variable. On the same decoded
samples, AVX (the default on the Linux machine) and SSE give different peak
digests for 143 of 265 files, AVX and scalar for 142; none matches the
Mac's peaks for more than 65 files. So the FFT path alone changes about
half the digests, and the Mac's takes NEON. Whether it is the whole cause
is open: the decoded samples and the maths library (`log10`, `cos`) were
not compared (`decoded_digests.py`, `data/0152-corpus-decoded-linux.tsv`
for the Mac). The sweep's queries are also encoded by libmp3lame.

**Conclusion.** Linux reproduces every score and cluster, not bit for bit.
Search is deterministic on each machine (`regress session-32-linux`
repeated identically): a baseline compares only where it was made.
