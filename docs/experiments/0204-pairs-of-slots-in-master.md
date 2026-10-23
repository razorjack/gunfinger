# 0204: pairs of slots in master

**Question.** Session 43, steps 4-5: with C5 (0203) in master
(`fd86305`), are all reports identical, and how long do the runs take?

**Change.** C5 of 0203, formatted and with one clippy fix in its new
test: `OffsetSlots` counts each pair of neighbouring slots in a byte,
a hit adds to its two pairs with one 16-bit word, and the test of both
is an addition and a mask. Tests as pre-registered.

**Validation** (`work/bin/s43-adopt/`, `work/s43/validate.sh`):
`scripts/check.sh` green (3:54); `regress session-32-linux` (1:13) and
`session-33-linux-nas` (4:05) identical, each report too, apart from
`wall_seconds`; the 11 development scans at NAS scale and at corpus
scale (262 tracks) and the clusters sample byte-identical to the start
build's; the full NAS clustering (session 33's run 2 command, scratch
copies) byte-identical to the start build's own run (SHA-256
`be08df5b1948...`, as session 41's runs).

| Run (Linux machine, otherwise idle) | Start | Adopted |
|---|--:|--:|
| Stakka scan's search, 32 workers | 27.18, 27.18 s | 26.02, 26.01 s (-4.2%, -4.3%) |
| User CPU of that scan | 762, 764 s | 727, 727 s (-4.6%, -4.8%) |
| The search, 16 workers | 32.59, 32.63 s | 31.60, 31.65 s (-3.1%, -3.0%) |
| User CPU, 16 workers | 475, 475 s | 459, 460 s |
| Clusters sample of 48, 32 workers | 1:23.0, 1:23.4 | 1:20.1, 1:19.4 (-4%) |
| Its user CPU | 1,677, 1,672 s | 1,593, 1,591 s (-5%) |
| `identify` of the Stakka mix, NAS configuration, 32 workers | 20.0, 20.0 s | 19.7, 19.7 s (-1.6%) |
| Its user CPU | 427, 427 s | 416, 416 s (-2.7%) |
| Full NAS clustering (one run each) | 15:03 | 14:12 (-5.7%) |
| Its user CPU | 26,314 s | 24,790 s (-5.8%) |

A B B A each (`work/s43/timing.sh`; `identify` after a warm-up run);
data `data/0204-*.tsv`. Every report and output identical to the start's.

**Conclusion.** Adopted. Counting pairs of slots takes 4.2-4.3% off the
search's wall time at 32 workers and 3.0-3.1% at 16, and 1.6% off a
user's `identify` at NAS scale, which also lists the library, loads
the index and decodes the mix. Where the search's CPU goes now: 0205.
