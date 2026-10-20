# 0154: counting the tracks in a gap from its length

**Question.** 2026-10-20, between sessions: when the tracks before and after a run
of k missed tracks are found, how often does the gap's length, over the median
of the mix's other start-to-start intervals, give k? A first look at step 2 of
`docs/research/2026-10-20-unidentified-gaps.md`, before any search.

**Method.** `scripts/analysis/gap_count.py` on the start times of the three
development manifests. For each run of k = 1-4 consecutive tracks with a track
before and after it, the gap runs from the run's first start to the next
track's start; the estimate is the gap over the median of the set's other
intervals (at least 3), rounded, at least 1. Runs overlap, so gaps are not
independent.

| Set | Tracks | Start-to-start intervals | Median | Quartiles |
|---|--:|---|--:|---|
| stakka-skynet-knowledge | 11 | 153-374 s | 316 s | 253-352 s |
| razorjack-2003-03-29 | 12 | 111-421 s | 233 s | 163-275 s |
| nocny-trans-vertical-2002-08-23 | 18 | 105-385 s | 235 s | 158-292 s |

| Missed | Gaps | Exact | Off by one | Estimate minus truth |
|--:|--:|--:|--:|---|
| 1 | 35 | 34 (97%) | 1 | +1: 1 |
| 2 | 32 | 24 (75%) | 8 | -1: 5, +1: 3 |
| 3 | 29 | 14 (48%) | 15 | -1: 8, +1: 7 |
| 4 | 26 | 11 (42%) | 15 | -1: 9, +1: 6 |

- The first row is flattered by the floor of 1: a real gap can hold no new
  track (a long blend, an MC, a treated passage of a found record).
- The starts are approximate: nocny's by ear, "more or less"; razorjack's are
  Gunfinger's boundaries except track 4's. A real gap ends at detection edges,
  a median 2.7 s inside crossfades (experiment 0020), small beside 105-421 s.
- Here every interval is known. In a real mix only the found tracks' are; if
  short plays are missed more often, their median is too long and the count
  too low (not measured).

**Conclusion.** On these three mixes the gap's length gives the count within
one in every case, exactly for 75% of two-track gaps and 42-48% at three and
four. It supports "about n tracks" with a range, not a count. The next measure
uses detection edges from runs with consecutive tracks left out.
