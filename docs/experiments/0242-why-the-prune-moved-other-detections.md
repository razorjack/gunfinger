# 0242: why pruning five files moved detections of files it did not touch

**Question** (brief 59). After session 52's prune, recall's brief query 911
(Echos at +1%) went from 455 hits at +1.147% to 503 at +0.933% (0238), a few
others by a hit or a refit; old binaries on today's store agree. Why?
**Method.** The pre-prune saved index (2026-10-21, 40,705 assets) survives as
hard links (`work/s36/w/indexes/`), read in memory only. `recall
--set-aside-like FILE` (new, tested) loads it by its header's provenance,
leaves out the run's held-out files and searches today's postings with the
lists it sets aside; `--trace-candidates` keeps each first-pass candidate's
chain of lines. Clean and brief queries, `--overlap-by-spans` (the old
reports' rule); `scripts/analysis/recall_by_position.py`, `chain_lines.py`,
`saved_index_cut.py`; reports in `work/s59/wrecall/`.

| Recall's index (held-out files left out) | Old lists set aside | Today's lists |
|---|--:|--:|
| Lists set aside; shortest | 10,863; 9,931 postings | 10,864; 9,930 |
| Clean: queries unlike the old report (of 702) | 0 | 3 (0238's moves) |
| Brief: queries unlike the old report (of 1,404) | 0 | 2 (910, 911) |
| Query 911, first pass | 370 hits at +1.147% | 369 at +0.933% |
| Query 911, second pass | 455 hits | 503 hits |

Pruned files' 19 and 27 detections set aside; the old index less the five is today's.
- One list moved: hash 1,286,800, 9,930 postings in both indexes and none
  of a pruned file. By inference (lists only shrink, the count of lists is
  the same), another list of 9,930 at the cut lost a pruned posting, the
  cut fell from 9,930 to 9,929 and the tie partner crossed it. The whole
  index's cut does not move (1,086,463 lists, 10,865 set aside, longest
  kept 9,938, before and after): scans and `regress` are untouched.
- Query 911's window 1 (10-20 s): the old chain kept the +1.2% rung's line
  (198 hits), today's the +0.8% rung's view of the same alignment (197);
  by inference the moved list cost the first a hit and `distinct`, which
  keeps the strongest view, kept the other at a tie. The chain is short, so
  its speed is the hit-weighted mean of its rungs: (123x1.2 + 198x1.2 +
  49x0.8)/370 = +1.147%; (123x1.2 + 197x0.8 + 49x0.8)/369 = +0.933%. The
  second pass measures there (truth +1%): 455 and 503 hits. Query 910's
  +1.2% line loses a hit: 276 -> 275, +1.0565% -> +1.056%, 386 hits both.
**Conclusion.** The lists set aside are a share of the index each run
searches (ADR 0014), so a list at a tie moved. Expected, no defect: a play
between two rungs makes the chain's speed hinge on one hit; both right.
