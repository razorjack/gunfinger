# 0115: the development mixes with every referenced track left out

**Question.** Session 20, after the review (rule in `docs/brief-20.md`,
written before the run): 0114's wrong plays come from clean renders. Do
the mixes themselves give wrong confident plays to test a sparseness cut
like-for-like?

**Method.** `gunfinger-eval --other-peaks-dir <NAS store> --verify scan
<set> --leave-out <all>` on the three development sets (12, 11 and 17
tracks with their clusters, 33, 50 and 48 files left out; store
`b285da9984422fb5`; 1.9-2.1 min each). Every confident detection is
wrong, except files covering 80% or more of a referenced track's
credited file (the clustering rule), which are the same recordings.
Data `data/0115-mixes-without-their-tracks.json`.

| Set | Confident | Of them the same recordings | Wrong | Strongest other file |
|---|--:|--:|--:|---|
| 2003 mix | 7 | 7 | 0 | Phantom 2018, 146 hits, possible |
| Stakka & Skynet | 0 | 0 | 0 | Clockwork (Stakka Rmx), 94, possible |
| Vertical broadcast | 2 | 2 | 0 | Mampi Swift - 2000, 197, possible |

- The 9 confident detections are 0114's: the newer uploads of the
  playing tracks (98.8-99.9% coverage of the credited file).
- No other file is confident. The related versions the mixes hold
  (Phantom Force's versions, Clockwork's remix, Mampi Swift - 2000 over
  Stab 2 Def, Klute's remix, `1.mp3`) stay possible at 79-197 hits.
- Fewer than 20 wrong plays: as the rule says, no cut is evaluated.

**Conclusions.** In the three development mixes, with all 40 played
recordings absent, no related version reaches the confident rule; the
panel's constructed queries (a passage played clean for up to 240 s) are
the harder case. A like-for-like test of a sparseness statistic needs
real mixes in which a related version plays over a long shared passage;
the development mixes have none.
