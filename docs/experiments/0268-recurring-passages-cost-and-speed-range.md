# 0268: recurring passages across a batch of mixes, the cost and the relative-speed range

**Question.** Brief 69's pre-registration asks for an explicit relative-speed range and its cost: what does searching mixes against each other cost at the product's ladder (±8%, 41 rungs) and at ±18% (91 rungs, 0.82-1.18, which reaches 1.08 / 0.92 = 1.174), in gaps and whole mode, and what does the wider range buy?
**Method.** 0267's ten batches (`work/s69/batches/`, `work/s69/logs/batch-*.err`, `/usr/bin/time -v`; 32 workers on the Linux machine, a Ryzen 9 5950X). Per batch: render 12 mixes, search each against the 262-track index on one thread, decode it, then for each mode and range build the pieces' index in memory and search every piece against it, one piece per worker. "Thread seconds" sums each piece's search time on its thread, a measure of the search's CPU. Data in `data/0267-recurring-passages.json`.

| Per batch, mean of ten (batches of 2.6-2.85 h of mixes; pieces overlap by 60 s) | Pieces; audio | Postings | Index build | Search: elapsed; thread s |
|---|--:|--:|--:|--:|
| Gaps, ±8% | 50.3; 0.96 h | 0.24 M (seed 6901) | 0.07 s | 3.9 s; 79 |
| Gaps, ±18% | the same | the same | 0.07 s | 9.1 s; 185 |
| Whole, ±8% | 23.6; 2.89 h | 0.77 M (seed 6901) | 0.18 s | 10.7 s; 176 |
| Whole, ±18% | the same | the same | 0.18 s | 24.6 s; 409 |
| Rendering, the 262-track searches and decoding (not timed apart; each mix decoded twice) | | | | 28.0 s elapsed |
| The whole run: elapsed; CPU (user and system); peak memory | | | | 1:15-1:22; 1,146-1,325 s; 2.33-2.56 GiB |

- The ±18% range costs 2.33 times the ±8% range's thread seconds in either mode, a little more than its 2.22 times the rungs; registered as affordable at up to 3 times, so it is.
- What it buys (0267, ten batches): positives at 8-12% relative speed found 73 of 107 in gaps mode (20 at ±8%) and 91 (23) in whole mode; beyond 12%, 62 and 98 of 112 (none at ±8%). Positives within 8% are found equally at both ranges (57 and 77 of 93). The wider range adds no wrong link of a new kind: "other" stays 0, the shared pairs linked rise from 19 to 20 of 50 (one Pyro pair more in each mode), and "another passage" links rise from 10 to 15 (gaps) and 31 to 37 (whole).
- Whole mode searches 3 times the audio of gaps mode, as queries and as index, for 2.2 times the thread seconds: the cost grows less than the audio searched; it is not broken down further.
- Per batch the four pairwise searches together take about 48 s of the 75-82 s elapsed, rendering and the library searches 28 s. For an archive the pairwise search should grow with the product of the batch's length and the length indexed: every mix is a query against every other. That growth is not measured here; a batch of 100 h of mixes would be about 35 times longer and its index about 35 times larger than this batch's whole mode.
- Memory: 2.33-2.56 GiB per batch, not broken down; the twelve mixes' samples take about 0.3 GB at 8 kHz, and the pieces' index of whole mode holds 0.77 M postings (about 3 MB).

**Conclusion.** An explicit ±18% range is necessary for opposite deck settings and costs about 2.3 times the product's ladder, in line with its rungs. At a batch of 12 mixes the four pairwise searches take under a minute; the search's growth with an archive's size is the open cost question.
