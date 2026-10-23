# 0200: the grouping candidates timed against the start build

**Question.** Session 43, step 3: do the candidates pre-registered in
brief 43 (07:59, `c11f284`) cut the Stakka scan's search at 32 workers
by 3% in every pair?

**Change** (sources `work/s43/lines.rs.<name>`, generators in
`work/s43/py/`, builds `work/bin/s43-<name>/`; the core's 152 tests
green for each): C1 rooms, each hit taken straight into a room of 120
hits for its asset, the rest in an overflow grouped by a counting sort
(no `part`, no count pass, no scatter); C2 C1 with parts of 512 assets;
C3 the scatter through a staged cache line of 8 hits per asset.

**Command.** `work/s43/ab.sh r1 s43-start s43-c1 s43-c2 s43-c3`: the
Stakka scan at NAS scale (saved index), start, C1-C3, C3-C1, start, at
32 then 16 jobs, otherwise idle. Search time is the report's
`wall_seconds`. Data `data/0200-ab-r1.tsv`.

| Search time (cut against start) | 32 jobs | 16 jobs |
|---|--:|--:|
| Start | 26.75, 27.19 s | 32.55, 32.59 s |
| C1 rooms | 26.23, 26.49 (1.9%, 2.6%) | 33.47, 33.63 (-2.8%, -3.2%) |
| C2 rooms, parts of 512 | 26.60, 26.77 (0.5%, 1.6%) | 33.83, 33.87 (-3.9%, -3.9%) |
| C3 staged lines | 31.28, 31.33 (-17.0%, -15.2%) | 38.96, 38.96 (-19.7%, -19.6%) |

Every report identical to the start build's apart from `wall_seconds`.
User CPU at 32 jobs: start 750-763 s, C1 733-741 s, C2 745-749 s, C3
890-892 s; at 16 jobs start 474-475 s, C1 488-489 s.

**Conclusion.** None passes the stop point; all three are dropped. C1
removes the count pass (53 s of CPU at 32 jobs, 0199), the `part`
buffer and two of three reads per hit, yet saves only 10-22 s of CPU
at 32 jobs and costs 14 s at 16: by inference, writing each hit to its
asset's place costs about the same whether it is done from `part` or
from the lists, so that write, not the passes around it, is what
grouping costs. Staging lines adds a copy and a branch per hit and
loses. Next: a microbenchmark of the scatter alone (0201).
