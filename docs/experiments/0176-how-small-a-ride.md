# 0176: how small a pitch ride loses a side

**Question.** Session 37, step 2: from what change of speed, over what
ramp, is a side lost (0175)? Do the development mixes show it?

**Method.** `mixes --kind rides --seed 3701 --count 16` with
`--ride-change` and `--ride-seconds` (new; every condition plans the same
194 ridden indexed plays, speeds, crossfades and ride starts; directions
drawn either way); a 0% ride is the control (the same render path).
`ride_dose.py`, `ride_stages.py --partial`; the development scans at NAS
scale with 0175's trace build (reports identical to session 33's),
`ride_candidates.py`. Data `data/0176-how-small-a-ride.json`.

| Ride (ramp) | Sides of 30 s+: no detection | 30 s+ uncovered | Own time uncovered |
|---|--:|--:|--:|
| 0% (control); 0.25% (2-5 s; 10-20 s) | 0; 0; 0 | 0; 0; 0 | 5-7 s of 31,933 s |
| 0.5% (2-5 s; 10-20 s); drift 0.5% over 60 s | 0; 0; 0 | 2; 0; 0 | 230; 7; 7 s |
| 1% (2-5 s; 10-20 s) | 1; 42 | 133; 70 | 29.4%; 15.1% |
| 2% (2-5 s; 10-20 s) | 110; 102 | 150; 102 | 32.9%; 24.8% |
| 3% (2-5 s; 10-20 s) | 91; 14 | 101; 14 | 22.7%; 4.7% |

- Of 326-347 sides per condition. No play loses every confident detection;
  no wrong confident one. Up and down lose alike (2% over 2-5 s: 59
  of 180, 51 of 167). Of the 572 sides with 30 s+ uncovered, 552 are
  dropped at the second suppression, 7 at the first, 5 below the rule; 8
  keep a shorter confident detection.
- At 1% over 2-5 s the kept side's widened end reaches ~10 s into the lost
  side. 3% over 10-20 s loses little: by inference, each side's alignment
  stops finding hits early in the ramp, so the widened ends rarely meet.
- Development mixes at NAS scale (candidates; no listening): Star Trails'
  side at 13:13-14:37, 1.3-1.5% slower than the kept 9:45-13:24 (0030's
  +4.5% to +3.1%), is confident (306-315 hits) and dropped; it is missing
  at 262 tracks too, and the manifest has Star Trails until 15:31. Into
  Music (2003 mix) plays at 1.040, 1.064 (25:57-26:56, 1,392 hits,
  dropped) and 1.053, leaving a 15-41 s hole. None in Nocny.

**Conclusion.** 0.25%, or 0.5% over 10 s or more, costs nothing; 0022's
within-play wander (±0.2%) is below it. From 1%, 21-43% of the sides lose
30 s or more (15-33% of the own time at 1-2%); a development mix holds
such a ride (Star Trails' dropped side is 84 s long).
