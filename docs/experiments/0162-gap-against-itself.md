# 0162: a gap matched against itself, a first prototype of method 2

**Question.** Session 34, step 5b: do the repeats inside a gap (a record playing
again what it played before) count its records better than its length does?
The plan's "The gap against itself"; a prototype, not a method yet.

**Change.** `gunfinger-eval self-match` (`self_match.rs`): each gap of 30 s or
more is decoded, its pair hashes matched with its own at speed 1.0 at lags of
8 s or more (2-frame steps; hashes occurring over 40 times in the gap left
out); a 5 s cell joins the cell a lag reaches when at least N of its hashes
recur at that lag; groups of joined cells are blocks. `self_match_scores.py`
counts blocks, blocks spanning 30 s or more, and stretches (long blocks with
overlapping spans merged), at N = 5, 10, 20, 40. Exploratory on the NAS gaps;
the best reading (stretches, N = 10) was then fixed and tested unchanged on
the long mixes of 0161 (`docs/brief-34.md`, 07:45): 320 gaps in 21 s at 16
jobs, 1.1 GiB. Data `data/0162-self-match.json`.

| Gaps (k = 1 / 2 / 3 / 4 missed) | Method 1 exact | Stretches at 10: exact | within 1 |
|---|---|---|---|
| Long mixes, 80 each (the test) | 86 / 53 / 39 / 26% | 95 / 80 / 69 / 54% | 100 / 99 / 98 / 93% |
| NAS leave-outs, 35 / 30 / 27 / 24 (explored) | 100 / 53 / 37 / 42% (single run) | 86 / 70 / 52 / 42% | 100 / 100 / 85 / 92% |
| 262 tracks, 20 / 13 / 12 / 12 | | 85 / 62 / 50 / 58% | 100% each |
| Short mixes, 99 / 28 (20-60 s plays) | 76 / 61% of all gaps | 81 / 14% | 100 / 75% |

- On the test the fixed count beats method 1 (registered interval) at every k
  and is within one for 311 of 320 gaps. Blocks as listed (not merged) are
  exact for 63/39/20/10%: one record's repeats form several groups.
- A gap with no missed track (12 at NAS scale, inside a found track): 4 give
  0 stretches at 10 hits, all 12 at 20 hits; the found track's own repeats
  give the rest. Counting records cannot tell a found track's tail from a
  missed one without comparing the blocks with the found neighbours.
- Short plays repeat little: two 20-60 s plays give 0.9 stretches on average.
- The NAS and 262-track rows count a gap once per set by its count and place,
  which keeps one gap at k = 1 that `gap_scores.py` merges.

**Conclusion.** Exact landmark repeats inside a gap count the records of
DJ-length plays better than the gap's length, unchanged from the gaps it was
explored on to the long mixes it was tested on. Generated mixes splice records
that rarely share breaks; beatmatched records sharing an Amen or Think break,
the plan's main risk, are not in them.
