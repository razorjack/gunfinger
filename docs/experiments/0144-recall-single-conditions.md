# 0144: recall at NAS scale under one hard condition at a time

**Question.** 0143's single conditions on the recall panel's development
half at 40,126 records: the clean control C, other rips only (a), brief
plays (b) and combined damage (c). How often is the own recording
confident, what is wrong, and why do misses fail?

**Method.** As pre-registered (0143, `docs/brief-28.md`); `recall` from
`0c0905d` (`work/bin/s28-recall/`), one run each, 03:57-04:28, 6-12 min
each, 5.1 GB. `scripts/analysis/recall_conditions.py`; data
`data/0144-recall-single-conditions.json`. (a) leaves out 63 records (58
own records, 5 identical copies): 20 sources (180 queries) observable,
38 unobservable. (c): 88 of 702 renders found no partner tempo within
10% and play it unmatched, as `robust` does.

| Condition | Confident | Possible or better | Wrong confident; possible | Held out (unobservable) confident |
|---|--:|--:|--:|--:|
| C, 30 s clean | 522/522 | 522 | 0; 69 | 0/180 |
| (a) other rips only | 175/180 (97.2%) | 178 | 0; 69 | 0/180 (0/342) |
| (b) 15 s | 228/522 (43.7%) | 522 | 0; 112 (both lengths) | 0/360 (both) |
| (b) 20 s | 493/522 (94.4%) | 522 | | |
| (c) combined damage | 5/522 (1.0%) | 428 (82.0%) | 0; 38 | 0/180 |

- Misses, by reason. (a): 2 never a candidate (Klute's Silent Weapons,
  Photek remix, at -8%; John B's Droogs at +8%), 3 too few hits (The
  Sect's Victim, vinyl rip, finds the YouTube upload with 202-231 hits
  in 3 windows at -8, -5, 0%); 4 of 5 in "elsewhere" (both vinyl rips).
  (b): 320 too few windows, 3 too few hits (all possible). (c): 423 too
  few hits but possible, 88 too few hits and weak, 6 never a candidate.
- (b) by where the play starts on the 10 s grid: 15 s plays starting
  0-5 s into a window, 0 of 259 confident; 5-6 s, 29 of 53; 6-10 s, 199
  of 210. 20 s plays miss only when they start within 1 s of a window
  border (29 of 102 there). (c) confident by kind: scene releases 5,
  YouTube and elsewhere 0; by speed 0-1 of 58 at each.
- The strongest wrong detection anywhere: Fire (Splash Remix) on the
  panel's Fire (Original Mix), 185 hits in C and (a), possible. No
  wrong possible detection reaches 200 hits.

**Against 0143.** Expected: C 522/522, (a) 90% or more, (b) about 42%
and 91%, (c) about 5% confident and 81% possible. Notable as declared:
only (a)'s misses, 4 of 5 in one kind (from 2 sources). No confident
wrong answer and no held-out or unobservable query confident anywhere.

**Conclusion.** One hard condition at a time costs no precision. Other
rips are named almost as well as the source itself; brief plays behave
as on the 262-track grid, set by where the windows fall; combined damage
leaves almost every query possible and almost none confident (1%,
against 5% at 262 tracks), for too few hits, not too few windows.
