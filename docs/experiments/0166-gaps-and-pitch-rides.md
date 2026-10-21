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

- The search loses time first. All 382 ridden indexed plays have a confident
  play, but 192 have a side of 30 s or more of their ride with no detection
  of 3 windows and 200 hits; their own parts are covered for a median 0.71-0.74
  per seed, the control's 1.00. On one rides mix (`logs/ride-cli-check.log`)
  the CLI finds Figurehead and Sin on one side of their rides only, though
  each other side alone is confident (2,130 and 3,082 hits). So 139 of 267
  listed gaps at 30 s hold no missed track (99 at 60 s); the control's none.
- R1, registered, all gaps: the count drops. Exact at k = 1-4: rides 9/3/4/2
  of 32, control 29/25/26/19 (18 and 99 of 128); with the neighbouring rungs
  31 and 96, with every rung 49 and 96. Most rides gaps (95) hold a lost side.

| Not registered: gaps of missed records only | At 1.0 | 0.996, 1.004 too | Every rung to ±3.2% |
|---|---|---|---|
| Rides, 33: exact; within 1 | 8; 24 | 18; 29 | 25; 33 |
| Long control, 127: exact; within 1 | 98; 124 | 95; 124 | 95; 124 |

**Conclusion.** In half of the plays the search drops one side of a sustained
1-3% pitch change: a gap inside a found track, counted as more records. In 33
gaps of missed records only, every rung to ±3.2% brought method 2 near the
control (25 of 33 against 98 of 127 exact), a small sample (`ride_gaps.py`).
