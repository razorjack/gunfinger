# 0106: the index built from the store on threads

**Question.** Session 18, step 6: when the saved index is stale (the
first search after `index` changes the store), `identify` builds it
first. On the NAS store that took 71-88 s of a 112-128 s search. Can
the build be faster, with the same index?

**Profile** (`--saved-index off`, `sample` every 10 ms): one thread
loads every record twice. Counting 32 s (pairs enumerated twice: for the
total, then per hash), filling 18 s, reading 12 s, opening, parsing 3 s.

**Change.** `HashedRecord` holds a record's pair hashes and anchor
frames in `for_each_pair`'s order; `Counting::count_hashed` and
`Filling::fill_hashed` take it (`count` and `fill` hash, then call
them). `indexing::build_planned` loads and hashes 32 records at a time
on `jobs` threads (`map_in_order`) while the calling thread counts or
fills the batch before, in library order; the same records give the
same errors. A test builds 69 records from a store on 3 threads and
compares every list with `Index::build`; it fails if the batches run in
another order.

**Batch size** (index ready; peak memory): 256 records, one batch at a
time, 13.6 and 12.4 s; 128, hashed during the previous batch's filling,
9.9-10.4 s, 4.41-4.43 GB; 32 the same, 9.9-10.3 s, 4.33-4.35 GB. Kept 32.

| 2003 mix, `--saved-index off`, A B B A | Wall | Index ready | CPU (user) | Memory |
|---|--:|--:|--:|--:|
| `s18-step6` (0105) | 121.1, 119.4 s | 80.6, 78.9 s | 440.3, 438.9 s | 4.34 GB |
| this change | 51.4, 50.8 s | 10.5, 10.1 s | 437.4, 430.2 s | 4.35, 4.36 GB |

Four reports identical, and the saved-index search's report. The saved
index written by `stats --saved-index rebuild` into a scratch
`XDG_CACHE_HOME` (built in 11.0 s) is byte-identical to the file the
start build wrote at 03:48. `regress session-18-start` identical. Load
average 14 at the start (this session's runs); another process at 8.7% of a core.

**Conclusion.** Kept: the first search after the store changes takes
about 51 s, not about 120 s; the hashing runs on every core, and the
calling thread's counting and filling (about 3 s) overlap it. Data
`data/0106-index-built-on-threads.json`.
