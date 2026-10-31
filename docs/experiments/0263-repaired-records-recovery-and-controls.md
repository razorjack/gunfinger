# 0263: records repaired at indexing: recovery and controls at NAS scale

**Question.** Brief 67's recovery test and controls (registered, `faeadee`):
with the five records the check repairs (0262: Filmscape, Advent, The
Weapon, Axis plainly inverted; Hello World uncertain), are those
recordings found in correct-polarity queries, and is nothing else moved?

**Command.** `work/s67/{recovery,search,asripped,rebuilt,setaside}.sh`.
T: a reflink copy of the NAS store; R: T with the five records from
`variant-store --repair-inverted`. Per file, from its audio with the right
channel negated (`work/s67/queries.py`, mono MP3s): 12 excerpts (30 s at
20-80%, speeds 0.96-1.04) and a mix (150 s at 1.02 between two drawn
corpus tracks); `identify --store-only` on each store.
Data: `data/0263-repaired-records-recovery-and-controls.json`.

| Confident (median hits of the file) | T (today) | R (repaired) |
|---|--:|--:|
| Plainly inverted four: excerpts; mix plays | 0 of 48 (0); 0 of 4 | 48 of 48 (1,037-1,417); 4 of 4 |
| Hello World: excerpts; mix play | 12 of 12 (313); 1 of 1 | 12 of 12 (1,450); 1 of 1 |
| Exploratory: the four, as ripped (uncorrected) | 48 of 48 | 0 of 48 |
| Exploratory: Hello World, as ripped | 12 of 12 (1,323) | 12 of 12 (302) |

**Other rips.** None by audio: under T no other file is confident on the
five's excerpts (item 2 unmeasured). New confident pairs under R: only
the five on their own queries; no new wrong one (item 3 holds).
**Controls.** (a) Corpus: none repaired, 265 of 265 records identical.
(b) 46 NAS files indexed today's way equal the NAS records; checked,
the five are repaired and 41 byte-identical. (c) `regress
session-64-linux-nas` against R: 709 of 720 sweep queries identical, the
rest in sub-threshold candidates (6-8 hits) and fitted speeds in the
eighth digit; Stakka counts and calibrate unchanged. Razorjack and Nocny
scans, T against R: confident detections equal (41, 42); Nocny loses a
non-confident candidate of today's Advent record. An index built afresh
from T is identical, and R's sweep with today's lists set aside equals
the baseline in all 720 queries: one list each way crosses the cut
(hash 1,337,041 from 9,910 to 9,913 postings, in; 69,523 out; 0242, 0254).
**Conclusion.** Repair recovers the four plainly inverted recordings in
full (0 to 48 of 48, mixes 0 to 4 of 4); Hello World is found either way.
Item 4 fails as written: no confident answer moves, but the cut moves
sub-threshold candidates. A query in the rip's own polarity loses.
