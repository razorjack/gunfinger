# 0011: summing a play's segments (offline null)

**Question.** The roadmap item "evidence summed across a play" would let a
play that falls apart into chains (Sin, faded out for a station insert: two
chains of 84 hits) reach the confident rule. How strong do sums get for
audio that is not the track?

**Method.** Offline, on existing reports (no new search, no test-set use):
`scripts/analysis/summed_null.py` groups each asset's detections as plays do
(gap ≤ 90 s, consecutive speeds within 0.2% or 0.5%), keeps segments with at
least 0, 20, 30 or 60 hits, and sums the groups credited to no track.
Reports: the dev scan and both leave-outs (session-2-start baselines) and
the dev scan at 10× and 30× synthetic copies (experiment 0012). Data:
`data/0011-summed-play-null.json`.

**Results** (strongest false multi-segment sum; segments, windows).

| Segments kept | Speeds within 0.2% | Within 0.5% |
|---------------|--------------------|-------------|
| all | 119 (2, 5) Stakka remix; other 51; synthetic 76 (5, 17) | 236 (7, 18) remix; other 125 (Kemal - Mechanizm, 7 segments); synthetic 109 (12, 27) |
| ≥ 20 hits | 119 remix; other 42 | 199 (4, 11) remix; other 42 |
| ≥ 30 hits | 80 remix; nothing else | 171 (3, 8) remix; nothing else |
| ≥ 60 hits | none | none |

"Other" is another real track, "synthetic" a copy (10× and 30× only).
From 20 hits up the rows are the same in all five reports: the copies add
thousands of weak groups when every segment is summed (1,446 and 2,708 at
0.2%, against 192 without copies) but none with a segment of 20 hits.

**Conclusions.** (1) Summing every segment is unsafe: a long play of noise
reaches 109-125 hits from 7-12 chance segments, past the possible tier.
(2) Summing segments of at least 60 hits creates no false group here, but
gains nothing measurable either: every dev track is found, and Sin's two
chains sum to 168, still under 200. (3) The related remix is the only false
group from 30 hits up, and it would come close to 200 at 0.5%. No change to
detection. If a future missed track shows two or more possible segments
whose sum passes 200, the rule "sum of ≥ 60-hit segments within 0.2%" is
the one to test; it needs the full protocol.
