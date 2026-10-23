# 0190: the pre-filter's candidates timed against the start build

**Question.** Session 41, step 3: do the candidates pre-registered in
brief 41 cut the Stakka scan's search at 32 workers by 3% in every pair?

**Change** (sources `work/s41/lines.rs.<name>`, builds
`work/bin/s41-<name>/`): C1 integer slots from per-point phases
(`floor(frame) mod 2^16`); C2 as C1, with each grouped hit holding its
integer offset in its posting's frame bits, so the filter reads only
the grouped hit; C3 stamps instead of the `touched` list; C4 C2 cleared
by computing the counted hits' slots again; C5 one 4 KiB fill instead
of `touched`; G1 the part's assets counted while it is taken. Core tests
green for each.

**Command.** `work/s41/ab.sh r1 s41-start s41-c1 ... s41-g1`: the
Stakka scan at NAS scale (saved index), start, C1-G1, G1-C1, start, at
32 then 16 jobs, otherwise idle. Search time is the report's
`wall_seconds`. Data `data/0190-ab-r1.tsv` (its half labels are garbled
by a shell variable; runs pair by position in `work/s41/ab.py`).

| Search time (cut against start) | 32 jobs | 16 jobs |
|---|--:|--:|
| Start | 37.60, 38.05 s | 45.62, 45.28 s |
| C1 phases | 30.76, 30.85 (18.2%, 18.9%) | 37.47, 37.17 (17.9%, 17.9%) |
| C2 offsets in the grouped hit | 29.13, 29.28 (22.5%, 23.0%) | 35.34, 35.45 (22.5%, 21.7%) |
| C3 stamps | 35.85, 35.92 (4.6%, 5.6%) | 43.19, 43.05 (5.3%, 4.9%) |
| C4 C2, cleared by re-reading | 27.14, 27.38 (27.8%, 28.0%) | 32.78, 32.60 (28.1%, 28.0%) |
| C5 one fill | 35.32, 35.35 (6.1%, 7.1%) | 42.00, 42.11 (7.9%, 7.0%) |
| G1 counting while taking | 38.59, 38.39 (-2.6%, -0.9%) | 45.65, 45.48 (-0.1%, -0.4%) |

Every report identical to the start build's apart from `wall_seconds`.
User CPU, C4: 762-763 s against 1,083-1,099 s at 32 jobs, 474-475 s
against 674-679 s at 16.

**Conclusion.** C1-C5 pass the stop point; G1 does not (the counter
loop in `take_part` costs what the second read saved) and is dropped.
By the pre-registered rule the combination is C4: it contains C2 (and
so C1), and is the fastest clearing that passed. C3 and C5 were timed
on `f64` slots; whether a fill or stamps beat re-reading on integer
slots is checked next (0191), exploratory.
