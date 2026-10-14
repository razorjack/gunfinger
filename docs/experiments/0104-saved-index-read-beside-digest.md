# 0104: the saved index's postings read on a second thread

**Question.** Session 18, step 6: loading the NAS store's saved index
(949 million postings, 3.8 GB) takes 3.7-4.0 s of one thread before the
2003 mix's search, beside decoding the mix (0098). Can it be shorter?

**Cause.** One loop reads 4 MB, then digests (64-bit FNV-1a per word),
checks (asset in range, list order) and copies each posting. The digest
is a chain of multiplications only one thread can follow.

**Change** (`index::saved::read_postings`). A scoped thread reads each
chunk, makes the postings and checks them (`Checked`: the order carried
from chunk to chunk); the caller digests each chunk while the thread
reads the next, and appends it; three chunks in flight. File format, digest and errors unchanged. A
test spans three chunks and fails if the order check restarts per chunk.

**Measured.** `stats -v`: loaded in 2.8-3.0 s, not 3.7-4.0 s; 0.55 s
more CPU by the end of the load. A side effect: the compiler stopped
inlining `BufReader<File>::read_exact` into `PeakStore::load` and
`read_header` (9 and 3 calls out of line), which read records a byte
at a time. `stats`, which reads every record: 17.4 s user, not 12.8 s.
The index built in memory (`--saved-index off`): 452.5, 453.0 s user
against 441.1, 444.6 s. 0105 removes that dependency.

| 2003 mix, saved index, A B B A twice | Wall | CPU (user) | Memory |
|---|--:|--:|--:|
| `s18-start` | 45.4, 44.1, 43.8, 44.3 s | 378.3, 377.7, 377.2, 375.9 s | 4.31-4.34 GB |
| this change alone | 43.7, 43.2, 42.8, 43.1 s | 385.9, 381.3, 378.2, 377.1 s | 4.34-4.35 GB |
| `s18-start` | 45.1, 45.3, 44.8, 44.2 s | 378.7, 371.2, 376.4, 376.4 s | 4.31-4.34 GB |
| with 0105 (`s18-step6`) | 44.5, 43.4, 43.1, 43.0 s | 382.6, 376.5, 376.9, 377.8 s | 4.35 GB |

Every report identical to the start build's; `regress session-18-start`
identical with `s18-step6`. Load average 2.6-3.1 when a series started
(12.6 after a build); other processes at 0-0.5% of a core.

**Conclusion.** Kept with 0105: 1.4 s (3.1%) less wall time per NAS
search, CPU within the runs' spread, memory +0.4%. The load now takes
about as long as decoding the mix beside it (2.6 s in 0098), so a faster
load alone would gain little more. Data `data/0104-*.json`.
