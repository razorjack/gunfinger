# 0106: the index built from the store on threads

**Question.** Session 18, step 6: with a stale saved index (the first
search after `index` changes the store), `identify` builds the index
first: 71-89 s on the NAS store. Faster, with the same index?

**Profile** (`--saved-index off`, `sample` every 10 ms): one thread
loads every record twice. Counting 32 s (pairs enumerated twice: for the
total, then per hash), filling 18 s, reading 12 s, opening, parsing 3 s.

**Change.** `HashedRecord` holds a record's pair hashes and anchor
frames in `for_each_pair`'s order; `Counting::count_hashed` and
`Filling::fill_hashed` take it (`count` and `fill` hash, then call
them). `indexing::build_planned` loads and hashes 32 records at a time
on `jobs` threads (`map_in_order`) while the calling thread counts or
fills the batch before, in library order; the same records give the
same errors. Tests: 69 files on 3 threads, one without a record, every
list as `Index::build`'s (fails if batches run out of order); a record
too long in the second batch stops the build with its error.

**Batch size** (index ready; peak memory): 256 records, one batch at a
time, 13.6, 12.4 s; 128, hashed during the previous batch's filling,
9.9-10.4 s, 4.41-4.43 GB; 32 the same, 9.9-10.3 s, 4.33-4.35 GB. Kept 32.

| 2003 mix, A B B A | Wall | Index ready | CPU (user) | Memory |
|---|--:|--:|--:|--:|
| `off`: `s18-step6` (0105) | 121.1, 119.4 s | 80.6, 78.9 s | 440.3, 438.9 s | 4.34 GB |
| `off`: this change | 51.4, 50.8 s | 10.5, 10.1 s | 437.4, 430.2 s | 4.35, 4.36 GB |
| `rebuild`: `s18-step6` | 129.7, 115.9 s | 88.9, 74.5 s | 440.7, 440.3 s | 4.30, 4.34 GB |
| `rebuild`: `s18-final` | 55.3, 52.7 s | 13.6, 12.7 s | 434.6, 432.0 s | 4.33, 4.34 GB |

`--saved-index off` builds in memory; `rebuild`, as a stale index, also
saves it (1.8-2.3 s), here into a scratch `XDG_CACHE_HOME`. `s18-final`
has the review's fixes. Every report identical; the four saved indexes
match the start build's file byte for byte; `regress` identical. Load
14 (this session's runs), another process 8.7% of a core at the `off` series.

**Conclusion.** Kept: the first search after the store changes takes
53-55 s, not 116-130 s; the hashing runs on every core, and the calling
thread's counting and filling (about 3 s) overlap it. Data `data/0106-*`.
