# 0105: peak records parsed from memory

**Question.** Session 18, step 6: 0104 made reading peak records slower
without touching their code. Why, and can their speed stop depending on
it?

**Cause.** `read_peaks` reads a record a byte at a time (varint frame
steps, then 1-, 2- and 1-byte fields) through a `BufReader<File>`. That
is fast only while the compiler inlines `BufReader::read_exact`'s
buffered path into `PeakStore::load`. With 0104 it did not (the release
profile's 16 codegen units, no LTO): 9 calls out of line, each with a
`memcpy`. Any change in `gunfinger-core` may flip that again.

**Change** (`PeakStore::load`). After the header, the rest of the record
is read into memory at once (`read_to_end`) and the peaks are parsed
from the slice, whose `read_exact` is small and always inlined. Records,
errors and messages unchanged. The store holds 187 bytes per second of
audio: about 190 kB in memory for a 17-minute track.

**Measured** (alternating builds, one run at a time):

| | `stats` (every record once) | Index built in memory, 2003 mix |
|---|--:|--:|
| `s18-start` | 24.5-26.5 s, 12.8 s user | ready in 71.3, 87.9 s; 441.1, 444.6 s user |
| 0104 alone | 26.8-27.3 s, 17.4 s user | 90.3, 93.6 s; 452.5, 453.0 s user |
| this change alone | 20.6, 21.2 s, 11.0 s user | not run |
| both (`s18-step6`) | 20.0-20.2 s, 11.6-11.7 s user | 78.8, 83.1 s; 440.2, 440.9 s user |

The index build is one thread reading 33,723 records twice, and its
wall time drifts by 16 s between two runs of one build; its CPU is the
steadier measure. Every report identical; the saved-index search as in
0104; `regress session-18-start` identical. `PeakStore::load` in the
kept build calls no `read_exact` of the `BufReader` (disassembly).

**Conclusion.** Kept: it removes 0104's slowdown and makes `stats` about
19% faster than at the start. The in-memory build is single-threaded and
reads every record twice (`indexing::build_planned`); it is the next
cost to look at when the saved index is stale. Data `data/0105-*.json`.
