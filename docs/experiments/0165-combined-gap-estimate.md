# 0165: method 2's count limited to method 1's range

**Question.** Session 35, step 4 (pre-registered, K1 and the printing
criteria): does limiting the gap's self-match count (method 2) to method 1's
range B count better than either method alone?

**Rule** (fixed before the test seeds): the stretches at 10 hits, raised to the
low end of range B (the gap over the upper and lower quartiles of the clean
intervals) or lowered to its high end; 0 stays 0; a gap without an estimate
keeps the count. Without 0164's discount, which failed D1.

**Data.** 0163's shared and control test mixes pooled (96 gaps per k), 0166's
rides and long mixes beside, and the development gaps; `gap_tests.py --range B`.
Data `data/0165-combined-estimate.json`.

| Gaps, k = 1-4 | Method 1: exact; off 2+ | Method 2 | Combined | Combined exact by k |
|---|---|---|---|---|
| Shared and control, 384 (the test) | 58%; 5.5% | 59%; 7.6% | 64%; 2.9% | 99 / 57 / 58 / 43% |
| Shared, 192 | 58%; 4.7% | 43%; 12.5% | 55%; 4.2% | 98 / 44 / 48 / 31% |
| Long control of 0166, 128 | 62%; 6.2% | 77%; 2.3% | 78%; 0 | 91 / 84 / 78 / 59% |
| Long, development, 320 | 51%; 7.5% | 74%; 2.8% | 80%; 0 | 98 / 86 / 75 / 60% |
| NAS, development, 116 | 14%*; 0.9% | 65%; 5.2% | 71%; 4.3% | 89 / 73 / 59 / 54% |

\* 82 of 116 NAS gaps have no registered estimate (0160); the combined count
keeps method 2's there.

- K1 holds: on the test the combined estimate is exact more often than either
  method (64% against 59% and 58%) and off by two least often (2.9%).
- As a count it fails (57% at k = 2; registered 90% at each k). The range n-1
  to n+1 holds the truth for 100, 99, 100 and 89.6% at k = 1-4 (86 of 96; 87
  needed) and 2.9% are off by two (at most 5%): one gap short at k = 4, where
  8 of the 10 misses are in the shared mixes.
- Limiting helps most where method 2 fails: on the shared mixes exact rises
  from 43% to 55% and off by two falls from 12.5% to 4.2%.
- The rides mixes (0166) defeat all three (12-15% exact): their gaps hold
  found tracks' lost time.

**Conclusion.** The combined estimate is the best count measured, but not a
count to print. As a range of one either side it falls one gap short at k = 4
on the test; the shortfall comes from records sharing material.
