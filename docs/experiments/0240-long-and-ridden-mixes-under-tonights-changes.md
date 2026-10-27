# 0240: the short, long and ridden generated mixes under tonight's changes

**Question.** Session 57, group 5 (pre-registration of 2026-10-27): what do
ADR 0021 and 0022 change in the generated mixes at 262 tracks whose
renders are kept: short and long seed 2026, rides 3741 and 3742, and seed
3801's four ride conditions (3% and 1-2%, ramps of 2-5 s and 10-20 s)?
**Method.** Build `s57-start`, `work/s57/wmixes` on reflink copies of the
renders (none rendered again), `--panels` session 38's copy, `--count 16`.
Old: `work/reports/` (sessions 37 and 38) and session 38's
`variant-apart-0.006/` where it exists. `work/s57/{mixes_score,diffs,residual_mixes}.py`.

| Old -> today | Right plays confident | Wrong confident | Wrong possible | Added (ADR 0021): confident; possible; weak |
|---|--:|--:|--:|--:|
| Short 2026 | 146 | 0 | 1 | 0; 0; 0 |
| Long 2026 | 191 | 0 | 9 | 0; 0; 14 |
| Rides 3741; 3742 | 185; 196 | 0 | 13; 8 | 155, 179; 1, 0; 9, 9 |
| 3801, 3% 2-5 s; 10-20 s | 191; 191 | 0 | 11; 8 | 138, 18; 4, 6; 6, 3 |
| 3801, 1-2% 2-5 s; 10-20 s | 191; 191 | 0 | 8; 8 -> 9 | 221, 200; 3, 3; 7, 7 |

- Every report with a session 38 variant equals it detection for
  detection; no play changes level, and no other difference appears.
- The confident additions are the other sides of rides: all 927 kept
  detections of a recording playing then are of the played recording. Of
  the 56 of a recording not playing, the strongest has 67 hits: the
  Clockwork remix possible over the original's shared passage (3801, 1-2%
  10-20 s, mix-11), the one wrong possible added; the other 55 are weak
  (at most 44). One possible detection is dropped there, overlapping
  within 0.6% a newly kept side (0179 found the same). None reaches 240.
- ADR 0022: ƆƐI3C's upload is not indexed at 262 tracks, so no credit
  (China Cup plays in four rides 3741 mixes, confident through its own
  cluster).
- Not run: mixes whose renders were deleted (plain and long 2027-2029 and
  7, long 3531-3532, rides 3521-3522, 3701's dose conditions).
**Conclusion.** Explained by ADR 0021 alone: rides keep both sides, the
only wrong addition is a possible related version (67 hits). No finding.
