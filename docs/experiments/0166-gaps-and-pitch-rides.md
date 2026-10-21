# 0166: pitch rides within a play

**Question.** Session 35, step 5 (pre-registered, R1): method 2 matches a gap
against itself at speed 1.0 only. When a record's speed changes within its
play, does the count drop, and do repeats at neighbouring speed ratios win it
back, at what cost?

**Change.** `mixes --kind rides`: long mixes whose every play rides the pitch
once by 1-3%, up or down, over 5-15 s, 30-70% into it (fixed before its seeds
ran, brief 09:41). `self-match --ratios` adds repeats played r times faster.

**Command.** Seeds 3521-3522 (rides) and 3531-3532 (long control), 16 mixes
each; 3:50-4:11 and 14 GiB per seed. `self-match` on their 395 gaps at 1.0
(21 s), with the neighbouring rungs 0.996 and 1.004 (35 s, 1.7 times) and every
rung from 0.968 to 1.032 (1:52, 5.4 times); `ride_coverage.py`. Data
`data/0166-pitch-rides.json`.

- The search, not method 2, is hit first. All 382 ridden indexed plays have a
  confident play, but 192 have a side of 30 s or more before or after the
  ride with no detection of 3 windows and 200 hits (control: 0 of 395). The
  CLI on one rides mix agrees: Figurehead and Sin are each found on one side
  of the ride only, though each side searched alone is confident (2,130 and
  3,082 hits). So 139 of the rides mixes' 267 listed gaps at 30 s hold no
  missed track (up to 263 s; 99 at 60 s); the control lists none.
- R1: the count drops. Pooled over k = 1-4, method 2 is exact for 18 of 128
  rides gaps against 99 of 128 in the control; most rides gaps also hold a
  found track's lost side, whose repeats add stretches (95 of 128).

| Gaps holding only missed tracks, k = 1-4 | At 1.0 | 1.0 and 0.996, 1.004 | Every rung to ±3.2% |
|---|---|---|---|
| Rides, 33: exact; within 1 | 8; 24 | 18; 29 | 25; 33 |
| Long control, 127: exact; within 1 | 98; 124 | 95; 124 | 95; 124 |

Every rung to ±3.2% brings these rides gaps to the control's level (76%
against 77% exact) and costs the control 3 gaps.

**Conclusion.** In about half of the plays, the product's search drops one side
of a sustained 1-3% pitch change: a gap inside a found track, counted as more
records. In gaps of missed records only, every rung to ±3.2% recovers method 2.
