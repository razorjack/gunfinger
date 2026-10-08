# 0008: The fitted matcher by default

## Status

Accepted (2026-10-08), the owner's decision. Supersedes the default matcher
of ADR 0001 (one pass over the speed ladder, 200 hits in 3 windows), which
stays available as `--single-pass`. ADR 0006 (plays, the possible tier of
60 hits) is unchanged.

## Context

Sessions 3 and 4 measured an opt-in package against the default matcher
(experiments 0024, 0026, 0027, 0030):

- **Skip the fullest lists for candidates.** The 1% of posting lists with
  the most postings hold 18.7% of the postings of a large collection and
  find most chance candidates (experiment 0043). Leaving them out of the
  search for candidates removes 94% of false candidates and 57-63% of the
  search CPU at 8,122 and 26,462 assets.
- **The second pass at the fitted speed.** Each candidate is analysed again
  with one STFT at its fitted speed and its hits counted in every posting
  list, so skipping costs no evidence. Its own rule, 240 hits in 3
  windows, was set from calibration data alone (experiment 0026).
- **Link rules.** Chains join only lines from rungs at most a step apart,
  and cross an empty window only on lines of 10 hits or more, so chance
  lines no longer lengthen a play or make it confident (experiment 0030).

At NAS scale (26,914 assets, session 5) the package found everything
today's matcher found with 3.3 times less wall time, 3.1 times less CPU,
half the memory and 4.3% of the false candidates (experiments 0039-0045).

## Decision

`identify`, `explain` and the harness search with `Matcher::Fitted`
(`gunfinger-core/src/search.rs`): `Index::skipping_fullest(0.01)` for the
first pass, `Links { nearby_rungs: true, strong_across_gaps: true }`, and
the second pass (`Options::second_pass`). Its detections carry
`Pass::Fitted` evidence and meet `FITTED_RULE`: confident at 240 hits in
3 windows, possible at 60. `--single-pass` (CLI and harness) searches with
`Matcher::SinglePass`, the matcher before this ADR; the harness keeps its
reports in `work/reports/variant-single-pass/`. Reports record the
matcher in `search.matching` and the rule in `search.confidence`, so
`identify --save-dir` searches again when either changes.

## Consequences

- At 262 tracks (experiment 0047): recall unchanged over four sweep seeds,
  the development set 11/11 with 0 wrong, leave-outs 0 wrong; true plays
  gain hits (0.84-1.16 times today's); the development scan's detections
  matching no track fall from 6,096 to 69.
- Plays confident only through a chance window become possible: mix 10's
  Dominion in the generated mixes and the grid's 10 s plays (experiment
  0030). A speed change of more than 0.6% between windows splits a play
  into segments (Star Trails at 13:20).
- The second pass measures up to 10 s beyond a candidate's chain. After a
  needle skip where the same drums continue, the part after the skip can
  take in the seconds before it, and the part before the skip, which then
  overlaps it, is dropped (the end-to-end test's synthetic mix). The play
  is still found; its start moves to the stronger part.
- Sick Note in the held-out test mix reached 209 hits under the single
  pass; with the fullest lists skipped it would most likely become
  possible (inference, experiment 0026). The next test-set evaluation
  should measure this matcher.
- Calibration figures before this ADR describe `--single-pass`; the
  register keeps both (`docs/calibration.md`).
