# 0202: ranks in grouping's count pass

**Question.** Session 43, round 2 (pre-registered in brief 43 at 08:17,
`3e2db3c`): does C4 cut the Stakka scan's search at 32 workers by 3% in
every pair?

**Change** (`work/s43/lines.rs.c4`, generator `work/s43/py/make_c4.py`,
build `work/bin/s43-c4/`; the core's 152 tests green): `group_part`'s
count pass keeps each hit's rank among its asset's hits; the scatter
writes each hit to its asset's start plus its rank, so it reads no
cursor another hit has just written (0201). Same `grouped`, hit for hit.

**Command.** `ROUNDS="1 2" work/s43/ab.sh r2 s43-start s43-c4`: start,
C4, C4, start, twice, at 32 then 16 jobs, otherwise idle. Data
`data/0202-ab-r2.tsv`.

| Search time (cut against start) | 32 jobs | 16 jobs |
|---|--:|--:|
| Start, round 1 | 26.94, 27.17 s | 32.51, 32.54 s |
| C4, round 1 | 26.95, 27.02 (-0.0%, 0.6%) | 31.42, 31.33 (3.4%, 3.7%) |
| Start, round 2 | 27.13, 27.13 s | 32.57, 32.76 s |
| C4, round 2 | 27.16, 27.17 (-0.1%, -0.1%) | 31.37, 31.36 (3.7%, 4.3%) |

Every report identical to the start build's apart from `wall_seconds`.
User CPU: 32 jobs 751-763 s for both builds; 16 jobs start 473-475 s,
C4 455 s.

**Conclusion.** C4 misses the stop point at 32 workers (no change) and
is dropped; at 16 workers it saves 3.4-4.3% in all four pairs. By
inference, with one thread a core the scatter waits on the cursor
chain that ranks remove, while with two threads a core the other
thread fills that wait, and only fewer operations per hit help, as
rooms did a little (0200). The next candidates therefore cut work per
hit in the pre-filter (brief 43, round 3).
