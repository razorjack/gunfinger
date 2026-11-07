# 0288: two records for a file with an inverted channel: recovery and controls

**Question** (brief 82, registered in `98db422`). With the repaired record
indexed beside the record as ripped (B), are both kinds of query found?
T: today's index; R: B without the five files' first records (0263's
store). **Change:** harness only (`e7cd915`; build `s82-run2`).
**Command.** `work/s82/{setup-stores,qnas,q262}.sh`; `queries.py` (mono
MP3s from byte copies of the NAS files), `score.py`, `samefirst.py`,
`moves.py`. Data: `data/0288-two-records-recovery-and-controls.json`.
**The flag (A5).** Over the scratch library (F 5, P 1, W 12) the check
repairs exactly F's five; P and W get none. The 18 records made today's
way equal the NAS store's byte for byte, the 5 second records 0263's.

| Confident at the file, NAS scale (median hits) | T | B | R |
|---|--:|--:|--:|
| Plainly inverted four: correct polarity, 48 excerpts; 4 mixes | 0; 0 | 48 (1,225); 4 | 48; 4 |
| The same, as ripped | 48 (1,154); 4 | 48 (1,154); 4 | 0; 0 |
| Hello World, correct: 12; 1 (as ripped the same counts) | 12 (316); 1 | 12 (1,454); 1 | 12; 1 |
| P (heard inverted, not flagged): its 2 inverted windows, as ripped; negated | 2; 0 | 2; 0 | 2; 0 |
| P, its 5 other windows: as ripped; negated | 5; 0 | 5; 0 | 5; 0 |
| W (12 wide, unflagged): 48 as ripped; 48 negated | 47; 0 | 47; 0 | 47; 0 |

At 262 tracks (the corpus and the 18 files) every row is the same,
medians within 3 hits. Under B the four are found through the first
record as ripped and the second with correct polarity, never both; Hello
World through both, the file keeping the stronger (1,454 hits, no sum). **Wrong answers
(A2, A3).** The same 9 confident plays of other files in each arm at NAS
scale: other uploads of two W files with the same titles (Conrad Black -
Programme Four 4, Mampi Swift - Origin 4; the clusters cover corpus
recordings only) and the known Evol Intent VIP of The Nine over a mix's
corpus track (0263). Possible plays of other files: 13 in each arm (2 at
262 tracks). No second record is named on another file's query, and no
other file is confident on F's queries (other rips: none).
**What else moves.** First-record plays are identical between T and B in
239 of 240 queries at NAS scale (one speed by 1e-6 percentage points);
at 262 tracks 111 plays in 98 queries move (hits by up to 3.9%, speeds
2e-3 points), no level changes (0289 attributes them). **Conclusion.**
A1, A2, A3 and A5 hold: both kinds of query find the four plainly
inverted recordings in full and nothing new is named. P's inverted
windows with correct polarity are lost in every arm: the flag missed P.
