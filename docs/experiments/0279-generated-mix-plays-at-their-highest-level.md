# 0279: generated-mix plays at their highest level, the saved reports rescored

**Question** (brief 77, registered in `3c93455`). The generated-mix scorer took a play's
level from its matching detection with the most hits (`mixes::score_play`), so a play
with a confident detection scored possible when a stronger one spanned fewer windows
(the external review of 2026-11-05, item 1). With the level the highest of the matching
detections (ADR 0013's rule for plays), which figures of committed experiments move?
**Change.** Harness only (`0bcfa44`): the level is the highest; `best` (most hits) still places
the play, `at_level` names the one giving the level; `counterexamples`' override is gone.
**Command.** `work/s77/rescore.sh`: `gunfinger-eval rescore` (today's clusters and
same-title judgments; `--other-peaks-dir` for NAS-scale reports) over every saved
`mixes` report under `work/` (202) and the three `grid` reports of session 57, each
written beside its saved one as `<stem>-rescored.json`. `work/s77/report-map.tsv` maps
197 reports to 38 citing experiments; `scripts/analysis/query_side_bar.py` for 0250.
Data: `data/0279-rescored-generated-mixes.json`.

| Reports | Plays | Level moved | Drifted otherwise |
|---|--:|--:|--:|
| 205, of 38 experiments (and 8 cited by none) | 54,477 | 6, all possible to confident | 0 |
| 0250 (NAS scale, stereo-fault mixes of seed 2028), 4 of its 12 | 1,152 in all 12 | 5 | 0 |
| The other 37: 0159-0166, 0175-0179, 0181, 0182, 0184, 0185, 0198, 0210, 0211, 0215-0217, 0235, 0237, 0239, 0240, 0245, 0246, 0255-0258, 0269-0271, 0274 | | 0 | 0 |

- Bad Company - Oxygen (0:00-0:24 of mix 02; 521 hits in 2 windows, 487 in 3) in the control
  and inverted mixes under both front ends, and Balderdash (5:26-5:53 of mix 06; 329 in 2, 250 in 3) in the inverted mix
  under power-sum queries; the sixth is Oxygen again in an uncited identity check.
- 0250, plays confident: first halves 91/99 to 92/99 (control and inverted, both front
  ends); the variant's inverted second halves 93/100 to 94/100. No play lost, no wrong
  file; brief 62's bar items pass and fail as before. 0250 gained an amendment.
- No drift: under today's clusters, judgments and thresholds every rescored play keeps
  its saved strongest detection, boundaries and position error (within 1e-9 s, since
  serde_json reads some floats one unit off, 0265), and every mix its wrong answers.
- Not rescorable without a search: the residual pilots' arms (0255-0258, 0269-0271),
  which keep each play's strongest detection only: 463 of 60,337 arm plays are below
  confident with 240 hits or more in under 3 windows, an upper bound on those that could
  move. Their cases (the confident indexed plays) are unchanged. 0264-0266 used the
  override, which gave the registered definition (0266); `shared-mixes`, `passages`
  and `identify` reports have their own scorers.

**Conclusion.** The defect was real and moved little: 6 of 54,477 saved plays move, all in
0250, by one play per figure, and no decision's reading changes.
