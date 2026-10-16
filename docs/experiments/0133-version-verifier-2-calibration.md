# 0133: version 2 of the version verifier, its own line from the audio, calibrated on the panel

**Question.** Session 26, steps 3-5a: with each play's line measured
from the reference's own hash hits, and contradiction limited to EQ
groups that demonstrably pass the reference, where does T fall on the
first panel's development half, and does the calibration separate?

**Pre-registration** (`docs/brief-26.md`, step 3, committed first).
Per segment, the mix against the reference's record alone: per 10 s
window the densest run within 2 frames, at any offset, is a knot (10+
hits); a step over `0.02 x dt + 2 frames` starts a new stretch; a lone
knot among others is a stray; chords keep knots within 1 frame; pieces
tested as version 1 tests a segment, between the reference's hits only.
No contradicted band in a group without an anchor. T by version 1's rule.

**Run.** Session 24's 387 calibration plays, 74 s, no errors; `data/0133-*`.

| | Negatives (312) | Positives (75) |
|---|--:|--:|
| Line: median offset from the report (abs.); over 48 ms | 3.0 ms; 11 | 0.2 ms; 0 |
| Plays with 2+ stretches; strays | 188; 198 | 13; 20 |
| S1 median; 95th pct; max | 0.007; 0.041; 0.056 | 0; 0; 0.028 |
| S1 by version 1's band rules on this line: median; max | 0.112; 0.770 | 0; 0.032 |
| At version 1's T (0.121), version 1's band rules on this line | 131 rejected | 0 |
| **At T = 0.00683 (version 2)** | **156 rejected** | **72 kept (96%)** |

- **T = 0.006825938566552901** (the 156th highest S1) reaches half and
  separates, barely: 3 positives rejected (Dozer 1, Fire 2). 2
  negatives unmeasured.
- The group rule removes most contradiction from the wrong plays too:
  18,372 of their patches are contradicted by version 1's band rules
  and not by version 2's, 99.4% of them with no low anchor.
- By family at T (version 1 at 0.121): Dozer 14 of 16 (13), Fire 127
  of 248 (144), Pyro 15 of 32 (0), Sonar 0 of 16 (0). With the line
  alone (version 1's band rules at 0.121) Dozer falls to 0 of 16, Fire
  to 131: Dozer's version 1 rejections came with its reported lines.

**Conclusion.** The measured line follows the reference (positives
within 44 ms); the group rule makes S1 small for every play, so T lands
at 0.7%, about one contradicted patch in 150. 0134 tests E1 at it.
