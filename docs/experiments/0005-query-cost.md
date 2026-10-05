# 0005: where query time goes

**Hypothesis.** At 262 tracks a query's time is spent on the 41 ladder STFTs,
not on index lookups.

**Change.** None to the code. `identify` on the development mix twice per
configuration: once against the full index, once with every asset but one
excluded (`--exclude-from`), so lookups, lines and chains cost almost
nothing. 10 jobs, Apple M1 Pro (10 cores), commit 5559c99.

**Command.**
`target/release/gunfinger identify <mix> --library corpus/library [--exclude-from work/exclude-all-but-one.txt] --format json`;
logs in `work/logs/cost-*.log`.

**Results** (56:09 of audio; both runs of each configuration agreed within
0.1 s):

| index | postings | decoding | total |
|-------|---------:|---------:|------:|
| 1 asset | 26,081 | 3.7 s | 19.5 s |
| 262 assets | 7,836,148 | 3.7 s | 22.1 s |

- Decoding: 3.7 s. The ladder (41 scaled STFTs, peak picking, pair
  hashing): about 15.8 s. Lookups, lines and chains against 262 tracks:
  about 2.6 s, 12% of the total.
- Building the in-memory index from the peak store: 0.4 s.

**Conclusion.** At this library size the ladder dominates (71% of query
time) and lookups are cheap. Lookup work grows with posting-list length,
which grows linearly with the collection; at 25,000 tracks (95 times the
postings) lookups would dominate if they scale linearly. That is the next
measurement to make, with a synthetic or real larger library.
