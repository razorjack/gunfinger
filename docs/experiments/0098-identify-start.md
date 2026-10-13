# 0098: the start of `identify`: header reads on threads, decoding beside the index load

**Question.** Step 2 of session 17: after 0096 and 0097, the 2003 mix at
NAS scale (`--store-only`) waits 5.5 s before the saved index starts
loading and decodes the mix (2.6 s) only after the load (3.5 s). Both
waits are outside the search.

**Cause.** With `--store-only`, `PeakStore::current_sources` reads the
header of each of the 33,596 records to list them, and `plan_index` reads
them again to check each is current and in the length range, one at a time.

**Change.** Both read the headers on `jobs` threads (`parallel::
map_in_order`, results in the same order); the CLI passes `--jobs`
through `catalog::Source`, `doctor` its setting, the harness its jobs
where it builds an index (every core where it only lists records).
`identify` decodes the first recording on a scoped thread while
`Catalog::open` loads the index (`open_while_decoding`; without a thread,
afterwards).

**Measured** (`work/s17/startup.sh`): seconds to the index loaded, 1:00:

| Store pages | Sequential | On threads |
|---|--:|--:|
| warm | 5.0-9.1 | 4.7-5.5 |
| evicted (`vmtouch -e`, read-only) | 9.0-9.2 | 4.6-4.7 |

Decoding during the load: a 10:00 excerpt ends 14.0-14.2 s, not 14.7 s.

**Acceptance**, the whole mix, `s17-start` against `s17-step2` (0096, 0097
and this), A B B A, idle machine (another process about 0.2 cores):

| Build | Wall | CPU (user; sys) | Peak memory |
|---|--:|--:|--:|
| `s17-start` | 62.1, 61.0 s | 487, 482 s; 6.6, 6.3 s | 4.79, 4.80 GB |
| `s17-step2` | 44.3, 44.1 s | 382, 377 s; 11.3, 13.9 s | 4.33, 4.34 GB |

Four reports identical; `regress` identical. **Kept**: 28% less wall
time, 22% less CPU, 10% less memory (with parts of 8,192 by mistake:
44.1-44.5 s against 57.6-58.2 s). A first search of the day reads the
headers cold, where the threads save about 4 s. Data `data/0098-*.json`.
