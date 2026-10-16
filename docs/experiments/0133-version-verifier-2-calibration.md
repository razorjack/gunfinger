# 0133: version 2 of the version verifier, its own line from the audio, calibrated on the panel

**Question.** Session 26, steps 3-5a: with each play's line measured
from the reference's own hash hits instead of the reported line, and
contradiction limited to EQ groups that demonstrably pass the
reference, where does T fall on the first panel's development half, and
does the calibration still separate?

**Pre-registration** (`docs/brief-26.md`, step 3, committed before any
run of version 2). Per reported segment: the mix analysed at the
reported speed against the reference's record alone; per 10 s window the
densest run of hits within 2 frames, at any offset, is a knot from 10
hits; knots join a stretch within `0.02 x dt + 2 frames` of the previous
one, else a jump starts a new stretch; a lone knot among others is a
stray, dropped; each stretch's line is the fewest chords keeping every
knot within 1 frame; tested from its first to its last hit, each piece
as version 1 tests a segment. A band of a group without an anchor is
killed (under -10 dB) or masked, never contradicted. A play with no
tested patch is unmeasured: not rejected, counted apart. T by version
1's rule (the `ceil(n/2)`-th highest S1 of the 312 negatives); the 75
positives kept at T reported beside.

**Run.** (to be written after the run)
