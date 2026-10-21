# 0164: telling a found track's own repeats from a missed record

**Question.** Session 35, step 3 (pre-registered, D1-D2): a gap inside a found
track makes stretches of that track's repeats (0162). Does searching the gap
against its neighbours' records, and not counting the stretches they cover, give
0 where nothing is missed without discounting missed records?

**Change.** `self-match --neighbours`: each gap searched (the product's search
and ladder) against an index of the records of the confident plays at its
edges; `gap_tests.py` drops a stretch that their detections of 10 hits or more
cover for half its span (chosen on development data before the test).

**Command.** Development: session 34's 45 NAS listings (19:21, nearly serial)
and 45 at 262 tracks; test: 0163's shared and control mixes (384 gaps). Not
registered: the rides mixes of 0166 (267 gaps, 8:53 at 12 jobs beside a NAS
regression). Data `data/0164-neighbour-discount.json`.

| Gaps | Holding no missed track: counted 0, stretches / discounted | Discount pushes a count below the truth |
|---|---|---|
| NAS, development | 4 / 8 of 12 | 3 of 116 |
| 262 tracks, development | 2 / 6 of 10 | 3 of 57 |
| Shared and control (the test) | none listed | 0 of 384 (it lowered no count) |
| Rides, not registered | 17 / 139 of 139 | 4 of 128 |

- D1 fails on the data the rule was chosen on (8 of 12; registered at least
  10), so step 4 uses the count without the discount. The 4 left are one 43 s
  gap inside Blindside in Nocny, between a possible segment and the confident
  one: the search finds no detection of two windows there, though Blindside's
  repeats make a stretch.
- D2 holds on the test only because no neighbour detection of 10 hits lies in
  any of its gaps: there the found plays' segments reach their edges.
- Where a found track runs on into a gap (a pitch ride's far side, 0166),
  its record covers it (213-6,698 hits): every such rides gap goes to 0.
- The mistakes are gaps where a missed record made no stretch of its own and
  the found track's tail stood in for it: removing the tail exposes the miss.

**Conclusion.** The discount removes a found track's own material when the
search can find that track in the gap, as it does after a pitch ride, but not
material too weak for two windows; it fails the registered D1 and stays out of
the combined estimate.
