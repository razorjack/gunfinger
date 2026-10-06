# 0008: shared drum sounds extend chains past the end of a record

**Context.** The synthetic end-to-end test (`crates/gunfinger/tests/`)
builds a mix from procedural tracks: track B plays from 38 to 82 s, then an
unindexed insert (82 to 96 s), then track C.

**Hypothesis.** A chain ends where its record stops.

**Change.** Three versions of the synthetic tracks, otherwise identical
(seeds 1 to 5, 75 s each, 1/16 chords at 170 BPM):

1. every track has a kick on every beat with a pitch drawn from 50 to 70 Hz
   on each hit: near-identical kicks on a common grid in every track;
2. no kicks;
3. each track has its own kick pitch (45 to 90 Hz) and its own one-bar
   pattern (the test's final version).

**Command.** `cargo test -p gunfinger --test end_to_end`, then
`gunfinger identify target/tmp/end-to-end/mix.wav --format json` (release).

**Results** (B's play):

| Tracks | Span | Windows | Hits |
|--------|------|--------:|-----:|
| 1. shared kicks | 38.5 to 108.4 s | 7 | 1,011 |
| 2. no kicks | 38.5 to 81.2 s | 6 | 782 |
| 3. own kicks | 38.4 to 81.4 s | 6 | 818 |

With shared kicks, B's chain continued for 26 s after B stopped, across the
insert and into C: hits of the kick pattern, common to every track, formed
lines at the offset B's chain predicted, and a chain bridges up to 2
windows. A and C were not affected in any version.

**Conclusion.** Material shared across tracks at a common tempo can extend a
detection beyond the end of its record. Real tracks rarely share identical
drum sounds, but sampled breaks (Amen, Think) in jungle and breakbeat
hardcore do. Boundaries are already approximate (roadmap, timeline polish);
trimming chain ends whose lines fall to chance level is a candidate fix, to
be checked against the full evaluation before it changes default behaviour.
Version 1 is the test case for that work.
