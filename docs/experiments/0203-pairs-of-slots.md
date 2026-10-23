# 0203: the pre-filter counting pairs of slots in one word

**Question.** Session 43, round 3 (pre-registered in brief 43 at 08:29,
`1336fde`): do C5 and C6 cut the Stakka scan's search at 32 workers by
3% in every pair?

**Change** (`work/s43/lines.rs.c5`, `.c6`, generator
`work/s43/py/make_c5.py`; builds `work/bin/s43-c5/`, `s43-c6/`; the
core's tests green). C5: `OffsetSlots` counts each pair of neighbouring
slots in a byte (4,097 bytes, the pair across the wrap twice); a hit
adds one to its two pairs as one 16-bit word and the test of both is an
addition and a mask, instead of a slot's count plus two loads of its
neighbours. Same answer for every asset (brief 43's argument; tested
against the old filter on 300,000 sets). C6: C5 with C4's ranks (0202).

**Command.** `work/s43/ab.sh r3 s43-start s43-c5 s43-c6`: start, C5,
C6, C6, C5, start, at 32 then 16 jobs, otherwise idle. Data
`data/0203-ab-r3.tsv`.

| Search time (cut against start) | 32 jobs | 16 jobs |
|---|--:|--:|
| Start | 26.85, 27.09 s | 32.54, 32.61 s |
| C5 pairs of slots | 25.74, 25.91 (4.1%, 4.4%) | 31.55, 31.54 (3.1%, 3.3%) |
| C6 C5 with ranks | 26.35, 26.34 (1.9%, 2.8%) | 30.52, 30.38 (6.2%, 6.8%) |

Every report identical to the start build's apart from `wall_seconds`.
User CPU at 32 jobs: start 752-761 s, C5 719-723 s, C6 736-737 s; at 16
jobs start 474-475 s, C5 458 s, C6 440-442 s.

**Conclusion.** C5 passes the stop point (4.1-4.4% at 32 workers) and
is adopted (0204). C6 misses it at 32 workers, though it is the fastest
at 16: as in 0202, ranks add work that the second thread of a core does
not hide. With fewer loads per hit, the filter's share of the search
falls (0205).
