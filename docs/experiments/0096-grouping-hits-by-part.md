# 0096: grouping a window's hits a part of the assets at a time

**Question.** Step 2 of session 17: grouping each window's hits by asset
was 51% of the NAS search's CPU (0093). Can it cost less with identical
lines? **Profile** (0093's method, `work/s17/profile.sh`, the 2003 mix at
NAS scale, `s17-start`): 62.6 s, 483 s CPU, 4.77 GB; grouping and lines
51.0%, query analysis 30.1%, hashing and posting scans 10.4%. With the
stages kept out of line: the counting sort's scatter 28.4%, the pre-filter
13.8%, its counting pass 8.1%. Synthetic windows on 10 threads: the
scatter costs 3.6-5.2 ns a hit, bound by memory traffic shared across
cores (2.1 ns on one thread).

**Variants**, A B B A on the idle machine (`work/s17/abba.sh`; another
process used 0.2-1 core), user CPU, reports byte-identical:

| Variant | Against | CPU |
|---|---|--:|
| 8-byte hits grouped, offsets computed per asset, counted while found | start, 15:00 | -9% |
| two passes, parts of 128 assets, a second buffer | start, 15:00 | +4%, rejected |
| the pre-filter clears the slots it touched instead of recomputing them | above, 15:00 | -2% |
| hits taken from the posting lists a part of 1,024 assets at a time | `s17-a6`, whole mix | +1.5%, memory -8% |
| the same, counting while taking | `s17-a6`, whole mix | +6%, rejected |
| **parts of 4,096 assets** | `s17-a6`, whole mix | **-3%, memory -7%** |
| parts of 8,192 assets | 4,096 | +1.7%, rejected |

**Change** (`search::lines`): a window keeps the posting lists its query
hashes hit, each ordered by asset as the index holds it (`Filling`). Each
part of 4,096 assets is taken off the front of every list, list by list,
then grouped by asset in cache, so each asset's hits keep the order they
were found in and the lines are the same. The 16 MB buffer of found hits
per worker is gone. The pre-filter (`OffsetSlots`) keeps the slots it
touched. `s17-a6` is 0097's build.

**Conclusion.** Kept. With the first three rows and 0097, the 2003 mix at
NAS scale took 51.7-51.9 s and 388 s CPU instead of 58.9-62.1 s and 480-483
s (`s17-a6`); parts of 4,096 then 48.0-50.6 s, 376-377 s and 4.31 GB
instead of 50.1-51.9 s, 388 s and 4.66 GB. What is left (profile of the
parts of 1,024): the pre-filter about 22%, grouping a part 20%, taking
parts from the lists (reading the postings) 16%, query analysis 26%.
Acceptance: 0098. Data `data/0096-grouping-hits-by-part.json`.
