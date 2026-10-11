# 0076: shared material at NAS scale, played whole

**Question.** Item 8c of session 11 (roadmap, "A confidence statistic for
shared material"): for the pairs judged different by ear and the
other version families, how close do their shared passages come to the
confidence rule (240 hits in 3 windows) as plays? Measurement only.

**Method.** From 0071's clusters (`shared_pairs.py`): the judged-different
pairs and the unjoined pairs of 100 hits or more whose title tags share
their first words, outside the five versions found through the test
tracklist; one pair per pair of clusters, 17 in all (The Nine and its VIP
and bootleg, Sonar and its revision, seven "Fire" versions, Tsunami,
Killa Bees, Star Trails). The plan was written at 03:43, before test
evaluation 4. For each direction, `identify` searched the played file's
aligned span (11-361 s) at NAS scale with the saved index (34 runs), and
the first 8 again with the played file's cluster left out (the index
rebuilt). `work/s11/item8c.sh`, `shared_results.py`; data
`data/0076-shared-material-nas.json`.

| Related file's play over the passage | Directions | Hits; share of the played file's own play |
|---|--:|---|
| Confident | 18 of 34 | The Nine's VIP 5,194-5,204 (21.7× the rule; 55-60%); "Fire" versions 283-596 (1.2-2.5×; 6-10%); Sonar's revision 384-542 (1.6-2.3×; 2-3%); Tsunami 299-343; Killa Bees 248-280 (10-13%) |
| Possible | 15 | 103-309 hits; 4-24% |
| Nothing | 1 | an 11 s passage |

Left out of the index, the played file's cluster changes none of the 8
related plays (the same hits and windows). The 30 s excerpts of the
sweeps and the recall panel never pass the rule for these pairs (at most
220 hits, the VIP; "Fire" 109-185), likely because 30 s covers little of
a shared passage: the same pairs gain hits with the passage's length. In
my 2003 mix the three Phantom Force variants reach 1.5-4.9% of
the original's play, all possible.

**Conclusions.** Played whole, shared passages pass the rule often, and
hits and windows cannot tell the versions apart. The played file's own
play is 4-60 times stronger (1.7 times for the VIP), but in the 2003
mix, copies of one recording over one passage range from 0.30 to 1.00 of
the strongest copy's hits (uploads against rips), so a ratio of
overlapping plays would separate the remixes, not a VIP that copies the
original. A statistic to test on recordings not used here (roadmap).
