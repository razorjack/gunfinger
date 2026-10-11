# 0072: the saved index at NAS scale

**Question.** Item 3a's acceptance (ADR 0011): does a saved index give
the same detections as one built from the store, and what does a build,
a first run, and a load from a cold and a warm file cache cost?

**Method.** The 2003 mix (46:10), `gunfinger identify --config
nas-dnb.toml --store-only --playback both -f json -v` (1afd8cc, store
`f9f9bc706195a71b`, 33,596 records): `--saved-index off`; `use` with no
file (it builds and saves); `use` again (warm); then `explain --at 2:30
--around 30`; `use` again after item 5's 2:41 h of runs (peaks of 6.9 GB
on 32 GB, swap in use), the nearest to a cold cache without `purge`,
which needs root: disk0 moved 5.8 GB during it, more than the file. One
at a time, nothing else of this session running, but under other load
(load average 130-210): the timings are under load, and the build took
longer than 0063's 65-72 s. `run-timed.sh` (`time -l`).

| Run | Index ready | Search | Wall | CPU | Peak |
|---|--:|--:|--:|--:|--:|
| In memory (`off`) | 118.1 s | 305.4 s | 429.4 s | 1,934 s | 4.78 GB |
| First (`use`, no file): build; save | 163.2 s; 2.0 s | 372.2 s | 546.7 s | 1,904 s | 4.78 GB |
| Warm load | 3.0 s (file 2.2 s) | 388.0 s | 397.8 s | 1,807 s | 4.78 GB |
| Cold load (likely) | 2.8 s (file 2.1 s) | 203.3 s | 211.5 s | 1,870 s | 4.78 GB |
| `explain`, warm | | | 14 s | | |

**Result.** The four reports are byte-identical JSON (27 plays). The
file holds 3,795,332,786 bytes for 945,836,813 postings: 3,783.3 MB of
postings, 8.4 MB of offsets, 3.6 MB of header and asset table (about 107
bytes per asset), 1.47 times the 2.59 GB store it comes from. Peak memory
is unchanged: a load fills the vectors a build fills. The search times
(203-388 s) vary with the machine's load, not with the index. Both loads
took 2.1-2.2 s (1.8 GB/s); if the second cache was cold, the disk was
not the limit.

**Conclusions.** A search at NAS scale no longer pays for the index:
2-3 s instead of 118-167 s here (65-72 s on a quiet machine). On the 2003
mix the search is now 98% of the wall time; `explain` takes 14 s.
