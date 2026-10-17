# 0145: recall at NAS scale under hard conditions combined

**Question.** 0143's combinations at 40,126 records: (a)+(b)+(c), (a)+(c),
(b)+(c), (a)+(b). How often is the own recording (under (a), another rip)
confident, what is wrong, why do misses fail?

**Method.** As 0144, 04:28-05:11; (b)+(c) cuts brief plays from damaged
excerpts with damaged context. `recall_conditions.py`; `recall_subsets.py`
for (a)'s 20 observable sources. `data/0145-recall-combined-conditions.json`.

| Condition, 15 s; 20 s | Confident | Possible+ | Wrong confident; possible | Held out (unobservable) confident |
|---|--:|--:|--:|--:|
| (a)+(b)+(c) | 0/180; 0/180 | 34; 73 | 0; 27 | 0/360 (0/684) |
| (a)+(c), 30 s | 0/180 | 125 (69.4%) | 0; 38 | 0/180 (0/342) |
| (b)+(c) | 0/522; 0/522 | 184; 306 | 0; 27 | 0/360 |
| (a)+(b) | 37/180 (20.6%); 141/180 (78.3%) | 177; 178 | 0; 112 | 0/360 (0/684) |

The 20 sources, 180 queries each: confident (possible+) [median hits of
the strongest own-cluster detection].

| Answered by | 30 s | 15 s | 20 s |
|---|--:|--:|--:|
| own record, clean | 180 [1,085] | 75 [472] | 173 [647] |
| own record, damaged | 5 (146) [114] | 0 (80) [56] | 0 (116) [77] |
| other rips, clean | 175 [458] | 37 [233] | 141 [299] |
| other rips, damaged | 0 (125) [82] | 0 (34) [38] | 0 (73) [54] |

- Why. Damage leaves about a tenth of the hits, other rips about half.
  Damaged misses: too few hits or never a candidate ((a)+(b)+(c): 217
  weak, 107 possible, 36 never). (a)+(b), 15 s: 91 too few hits, 49 too
  few windows, 3 never; 20 s: 29, 8, 2.
- Wrong possible: only 0144's pairs (other Fire versions on Fire (Original
  Mix), Tsunami on its remix); strongest 146 hits ((a)+(b), 20 s).

**Against 0143.** As expected for (a)+(b)+(c) (no confident query); none
of the declared notable results.

**Conclusion.** Combined, the conditions cost no precision. With damage
nothing is confident and 19-69% stay possible; without it, other rips'
brief plays are confident in 21% (15 s) and 78% (20 s), short of hits.
