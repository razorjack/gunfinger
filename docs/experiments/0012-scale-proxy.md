# 0012: scale proxy with reversed, stretched copies of the library

**Question.** How do chance alignments grow with the library (ADR 0007,
measurement 1), without new music?

**Method.** `--synthetic-copies N` (`crates/gunfinger-eval/src/synthetic.rs`)
adds N copies of every peak record: reversed in time, so nothing can truly
align with forward audio, and stretched in time and frequency by 0.80-1.20
in 4% steps, so their hashes differ. `robust --only control` (seed 2026,
speeds 0.95 and 1.03, 40 indexed and 10 held-out 30 s excerpts; 209 real
records) and `scan stakka-skynet-knowledge` (262) at N = 10, 30, 100.
Data: `data/0012-scale-proxy-summary.json`.

**Results.** Strongest wrong candidate per excerpt (98 excerpts; the
Clockwork excerpt, whose remix shares material, left out):

| Copies | Assets (robust) | Median | 90th pct. | Max | Held-out max | Confident |
|-------:|----------------:|-------:|----------:|----:|-------------:|----------:|
| 0 | 209 | 7 | 13 | 29 | 24 | 80/80 |
| 10 | 2,299 | 10 | 16 | 29 | 24 | 80/80 |
| 30 | 6,479 | 10 | 16 | 29 | 24 | 80/80 |
| 100 | 21,109 | 12 | 17 | 29 | 24 | 80/80 |

Own-track hits are unchanged (median 858, minimum 654). The maxima are
real tracks; no copy beat them. Development mix (56 min): 11/11, 0 wrong,
and the real detections are bit-identical to the run without copies. The
strongest copy reaches 25 hits (6 windows) at 10 copies (2,882 assets) and
30 hits (6 windows, a reversed drum track) at 30 (8,122); 14 and 17
detections of copies reach 20 hits, none 60. The scan at 100 copies (26,462 assets) swapped
beside the desktop applications (8.6 GB) and was stopped.

**Conclusions.** (1) Chance grows slowly: a hundredfold library moves an
excerpt's typical strongest wrong candidate from 7 to 12 hits; over a whole
mix at 30 copies the strongest chance alignment is 30 hits, half the
possible tier and under a sixth of the confident rule. (2) The proxy is a lower bound: reversed copies hold no
shared breaks, remixes or samples, which give the real false candidates
(91-95 hits). Those, not chance, set the thresholds at scale; breakbeat
hardcore is the case to measure (ADR 0007, measurement 3). (3) Memory
comes first: the in-memory index of 26,000 assets does not fit beside a
desktop session (ADR 0005). Query time: experiment 0013.
