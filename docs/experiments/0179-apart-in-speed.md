# 0179: keep an overlapping side only at another speed (opt-in)

**Question.** Session 38, step 2: does judging overlap after the second
pass only between detections within 0.6% in fitted speed keep both sides
of a ride (0175) without 0178's wrong added segment on the shared panel?

**Change** (`393469d`, opt-in `--apart-in-speed 0.006`,
`SecondOverlap::SpansAtSpeed`; off by default, `regress` identical when
off). The second suppression drops a detection only when it overlaps a
stronger one of the same asset and their fitted speeds differ by at most
0.6%. Threshold and criteria S1-S3 committed first (brief 08:26), chosen
from session 37's development traces: rides' dropped sides differ by the
ride (1%: 0.90% at least), the shared panel's by 0-0.40%.

| Run (default -> variant) | Result |
|---|---|
| S1, rides seed 3801, sides lost (3% 2-5 s; 3% 10-20 s; 1-2% 2-5 s; 1-2% 10-20 s) | 94; 10; 44; 109 -> 0 (257 of 257, 100% of 17,487 s); partial count 380 -> 4 |
| Seed 3801: wrong confident; right plays | 0 -> 0; 191 -> 191 in each condition, none lost |
| Sweep 2026, 262 tracks; NAS scale | identical; 4 of 720 gain a weak detection, wrong 7 -> 7, right 540 -> 540 |
| Stakka and 2 leave-outs, both scales | wrong 0 -> 0, identified same; Star Trails' 13:13 side added in the scan and leave-out 3 (306-315 hits) |
| 8 other development reports, NAS scale | wrong 0 -> 0, right same; Into Music's 25:57-26:56 segment added (1,392 hits); weak only otherwise |
| Generated mixes 2026: short; long | identical; 14 weak added, wrong 0 -> 0, right 191 -> 191 |
| First shared panel, development half, NAS scale (622 queries; a real default run) | confident detections by role identical in every query (played 516, other 475, related 430, partner 8); 185 weak added |
| Calibrate, false candidates at 30 hits+ (262 tracks; NAS) | 18 -> 18; 95 -> 95 (0178: 18 -> 19) |
| User CPU: `regress` 262 tracks; NAS scale; shared | +1.3%; +0.8%; -3.1% (defaults: 1,750-1,778 s; 10,220-10,287 s, the variant 0.6% above; `check.sh` ran beside the default shared run) |

- Not a superset of the default's list: one possible detection dropped
  in seed 3801, a weak Star Trails one in Stakka; no play changes.
- Phantom Force's 0.005% segment (2003 mix), which 0178 added, stays
  dropped. Candidates nobody has listened to: Star Trails' slower side,
  Into Music's middle at 1.064 (between 1.040 and 1.053).
- Data `data/0179-apart-in-speed.json`; `variant_compare.py`,
  `ride_variant.py`.

**Conclusion.** S1, S2 and S3 hold: every side the default loses on new
seeds is kept, and no run gains a wrong confident detection or loses a
right one; on the shared panel, where 0178 added the Rogue Unit's
segments, nothing confident changes. The shared panel was also the
development data for the threshold (brief 08:26). Not adopted: an ADR.
