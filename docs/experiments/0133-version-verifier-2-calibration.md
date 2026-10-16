# 0133: version 2 of the version verifier, its own line from the audio, calibrated on the panel

**Question.** Session 26, steps 3-5a: with each play's line measured
from the audio and contradiction limited to EQ groups that pass the
reference, where does T fall on the first panel's development half?

**Pre-registration** (`docs/brief-26.md`, step 3, committed first):
knots from 10 s windows' densest runs at any offset, stretches split at
jumps, lone knots dropped as strays, chords within 1 frame; no
contradicted band in a group without an anchor; T by version 1's rule.

**Run.** Session 24's 387 calibration plays, 74 s, no errors; `data/0133-*`.

| | Negatives (312) | Positives (75) |
|---|--:|--:|
| Line: median offset from the report (abs.); over 48 ms | 3.0 ms; 11 | 0.2 ms; 0 |
| Plays with 2+ stretches; stray knots; a stretch 2.7-97 s off | 188; 198; 30 | 13; 20; 0 |
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
  of 248 (144), Pyro 15 of 32 (0), Sonar 0 of 16 (0). Version 1's band
  rules on version 2's patches at 0.121: Dozer 0 of 16, Fire 131. Dozer's
  knots lie within 3.5 ms of the reported line, but version 2 tests 3,762
  of version 1's 5,039 Dozer patches, 216 contradicted by version 1's
  rules against 739. Where the related file differs, a window's densest
  run lies elsewhere, its knot is a stray, and that stretch goes
  untested (an inference): a selection against the differing material.

**Conclusion.** The line follows the positives within 44 ms, but skips
or relocates differing material; the group rule makes S1 small for every
play, so T lands at 0.7% (one patch in 150 rejects). 0134 tests E1.
