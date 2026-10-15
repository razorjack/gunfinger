# 0113: `take_part` split into its searches and its copying

**Question.** Session 20, step 6, proposal 3's first step: a directory of
part boundaries would replace `WindowHits::take_part`'s binary searches
(`partition_point`, one per list and part of 4,096 assets). What share of
CPU are the searches, against copying the hits? The research note's stop
point: under 5%.

**Method.** A build never committed, from `a076ca2` in a temporary
worktree: the search and the copy moved into `s20_part_end` and
`s20_copy_part`, both out of line, as are `take_part` and `group_part`.
The 2003 mix at NAS scale (`identify --store-only --playback both -f
json`, 38,105 records, store `b285da9984422fb5`) under `sample` every
10 ms (0103's method), twice (`work/s20/profiles.sh`; stages by
`work/s20/py/stages.py`, session 19's with two markers). Both reports
byte-identical to the start build's (48.7 s, 412 s user, 4.85 GB). Load
18-24 at each start; two other processes at 0.6-1 core and
up to 0.4. Data `data/0113-take-part-split.json`.

| Share of CPU samples | Run 1 | Run 2 |
|---|--:|--:|
| Wall, user CPU | 49.6 s, 403 s | 51.0 s, 395 s |
| Binary searches (`s20_part_end`) | 489 (1.4%) | 482 (1.3%) |
| Copying hits into the part (`s20_copy_part`) | 3,246 (9.1%) | 3,341 (9.1%) |
| `take_part`'s own loop | 131 (0.4%) | 158 (0.4%) |
| Grouping a part by asset | 24.4% | 24.3% |
| Lines, with the pre-filter inlined here | 28.0% | 28.1% |
| The ladder's STFT and peak picking | 25.6% | 25.5% |

**Conclusion.** The searches are 1.3-1.4% of CPU, under the 5% stop
point: a directory of boundaries could save at most that, before its own
72 MiB (9 boundaries at 38,105 assets) and its rebuild for restricted
indexes. Proposal 3 stops. `take_part`'s 10.8% is mostly the copying:
each hit written with its anchor into the part's buffer, which
`group_part` then reads. Nothing kept; the worktree, its build and its
target directory were removed (the diff is in `work/s20/`).
