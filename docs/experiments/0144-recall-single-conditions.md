# 0144: recall at NAS scale under one hard condition at a time

**Question.** 0143's single conditions at 40,126 records: clean C, other
rips only (a), brief plays (b), combined damage (c). How often is the
own recording confident, what is wrong, why do misses fail?

**Method.** As pre-registered (0143); `recall` from `0c0905d`, one run
each, 03:57-04:28 (6-12 min, 5.1 GB); `recall_conditions.py`, data
`data/0144-recall-single-conditions.json`. (a) leaves out 63 records (58
own, 5 identical copies): 20 sources (180 queries) observable, 38 not.
(c): 88 of 702 renders play the partner unmatched, as `robust` does.

| Condition | Confident | Possible+ | Wrong confident; possible | Held out (unobservable) confident |
|---|--:|--:|--:|--:|
| C, 30 s clean | 522/522 | 522 | 0; 69 | 0/180 |
| (a) other rips only | 175/180 (97.2%) | 178 | 0; 69 | 0/180 (0/342) |
| (b) 15 s; 20 s | 228/522 (43.7%); 493/522 (94.4%) | 522; 522 | 0; 112 | 0/360 |
| (c) combined damage | 5/522 (1.0%) | 428 (82.0%) | 0; 38 | 0/180 |

- Misses. (a): 2 never a candidate (Klute's Silent Weapons, Photek
  remix, at -8%; John B's Droogs at +8%), 3 too few hits (The Sect's
  Victim, a vinyl rip, finds its YouTube upload with 202-231 hits in 3
  windows); 4 of 5 in "elsewhere" (both vinyl rips). (b): 320 too few
  windows, 3 too few hits. (c): 423 too few hits but possible, 88 too
  few and weak, 6 never a candidate.
- (b) by where the play starts in a 10 s window: 15 s from 0-5 s, 0 of
  259 confident; 5-6 s, 29 of 53; 6-10 s, 199 of 210. 20 s plays miss
  only within 1 s of a window border (29 misses of 102 there). (c)
  confident: 5 scene releases, 0-1 per speed.
- Strongest wrong detection: Fire (Splash Remix) on the panel's Fire
  (Original Mix), 185 hits in C and (a), possible; none reaches 200.

**Against 0143.** As expected ((c) below `robust`'s 5% at 262 tracks).
Notable as declared: only (a)'s misses, 4 of 5 in one kind (2 sources).
No confident wrong answer; no held-out or unobservable query confident.

**Conclusion.** One hard condition at a time costs no precision. Other
rips are named almost as well as the source; brief plays behave as on
the 262-track grid, set by the windows; combined damage leaves 82%
possible or better and 1% confident, for too few hits.
