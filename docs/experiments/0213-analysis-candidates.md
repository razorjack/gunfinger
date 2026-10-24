# 0213: cheaper query analysis, the candidates

**Question.** Which exact changes to the analysis cut the Stakka scan's
search at NAS scale by 3% or more at 32 workers in every pair (brief
46's pre-registration, 23:29, and its addenda, 23:36 and 23:45)?

**Candidates** (`work/s46/c*.diff`), all with the same peaks to the bit:
C1, `round` without the library call (refinement, magnitudes, the STFT
frame of a peak); C2, the peak test on powers, decibels taken for the
17.6 candidates a frame and the peaks, the neighbourhood folded with
`max`; C2b, C2 with the old comparison fold and a second scan only for
bins that pass; C2c, C2b with decibels only for bins that pass every
test on powers (about 0.6 a frame); C3, the four-neighbour test as one
loop over equal slices, then the passing bins gathered; C4, C2c with C3.
C2's argument: `decibels` never decreases as power grows, a test over
every non-negative `f32` on the machine running the tests.

**Method.** `ab.sh`, otherwise idle: round 1 start, C1, C2, C3, C3, C2,
C1, start; round 2 start, C2b, C2b, start; round 3 start, C2c, C4, C4,
C2c, start, twice; each at 32 then 16 jobs. Every report identical to
the start build's apart from `wall_seconds`. Data
`data/0213-candidate-rounds.tsv`.

| Candidate | Cut at 32 workers | At 16 | CPU at 32 |
|---|--:|--:|--:|
| C1 rounding | -1.1%, -0.2% | 0.0%, 0.1% | -0.6%, 0.1% |
| C2 powers, `max` fold | -0.9%, -0.5% | 2.4%, 1.9% | -1.3%, -0.2% |
| C2b comparison fold | 2.6%, 2.1% | 3.8%, 3.9% | 2.3%, 2.6% |
| C2c decibels for survivors | 2.4-3.2% (4 pairs) | 4.4-5.1% | 2.7-3.4% |
| C3 four-neighbour loop | 1.7%, 2.5% | 2.9%, 3.0% | 1.1%, 2.4% |
| C4 = C2c with C3 | 7.2-7.8% (4 pairs) | 8.4-8.7% | 8.5-9.3% |

Search at 32 workers, round 3: start 25.98-26.12 s, C4 24.03-24.20 s;
at 16, 31.77-32.03 s and 29.09-29.28 s.

**Conclusion.** Only C4 passes; it is adopted. Together the two halves
save more than the sum of their cuts alone (4.1-5.7%). By inference, after C2c the four-neighbour loop is the larger share of
the picking, and C3 makes it cheaper. C2's `max` fold cost more than
the logarithms it saved at 32 workers (C2 against C2b). Rounding
costs nothing measurable, as 0212 predicted.
