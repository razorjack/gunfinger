# 0263: records repaired at indexing: recovery and controls at NAS scale

**Question** (brief 67, registered in `faeadee`). With the five records
the check repairs (0262) are those recordings found in correct-polarity
queries, and is nothing else moved?
**Command.** `work/s67/{recovery,search,asripped,rebuilt,setaside}.sh`. T: a
reflink copy of the NAS store; R: T with the five records from `variant-store
--repair-inverted`. Per file, from its audio with the right channel negated
(`work/s67/queries.py`, mono MP3s): 12 excerpts (30 s at 20-80%, speeds
0.96-1.04) and a mix (150 s at 1.02 between two drawn corpus tracks);
`identify --store-only` on each store. Data: `data/0263-repaired-records-recovery-and-controls.json`.

| Confident (median hits of the file) | T (today) | R (repaired) |
|---|--:|--:|
| Plainly inverted four: excerpts; mix plays | 0 of 48 (0); 0 of 4 | 48 of 48 (1,037-1,417); 4 of 4 |
| Hello World: excerpts; mix play | 12 of 12 (313); 1 of 1 | 12 of 12 (1,450); 1 of 1 |
| Exploratory: the four, as ripped (uncorrected) | 48 of 48 | 0 of 48 |
| Exploratory: Hello World, as ripped | 12 of 12 (1,323) | 12 of 12 (302) |

Possible equals confident in every row. **Other rips:** none by audio
(T finds no other file on the five's excerpts; item 2 unmeasured). New
confident pairs under R: 52, the four on their own queries; none wrong
(item 3 holds). By file name, the mixes' only confident play of another
version, in both arms, is session 62's Evol Intent VIP over The Nine.
**Controls.** (a) Corpus: none repaired, 265 of 265 records identical.
(b) 46 NAS files indexed today's way equal the NAS records; checked,
the five are repaired and 41 byte-identical. (c) `regress
session-64-linux-nas` against R: 709 of 720 sweep queries identical; in
9 the confident Cause 4 Concern - Abduction's fitted speed moves by up
to 2e-4 percentage points (hits equal), in 2 sub-threshold candidates
(6-7 hits) change; `per_speed`'s mean speed errors move in the sixth
digit; Stakka counts and calibrate unchanged. Razorjack and Nocny scans,
T against R: confident detections equal (41, 42; one speed in the
eighth digit); Nocny loses two 6-hit candidates (today's Advent record,
Amex & Kano - Descent). A fresh index of T is identical; R's sweep with today's lists set aside equals the baseline in all 720
queries: one list each way crosses the cut (hash 1,337,041 from 9,910 to
9,913 postings, in; 69,523 out; 0242, 0254); for the scans, which have
no such option, the same cause is an inference.
**Conclusion.** The four plainly inverted recordings are recovered in full, Hello World
is found either way, item 4 fails as written; under R, queries in the rip's own polarity are lost.
