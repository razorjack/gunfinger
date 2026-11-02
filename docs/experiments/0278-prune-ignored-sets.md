# 0278: the records of DJ sets and a mixed CD's track fix pruned, against session 64's baseline

**Question** (brief 75). Three NAS ignore entries of 2026-10-31 leave out `sety/` (DJ sets), the Virus
on Kiss FM show of 3 May 2000 and the Lowdown Vol 6 track fix (status, 2026-10-31). The store held 8
peak records, 99 skip notes and 8 tag notes of 107 of those files. What does pruning them change?
**Method.** `doctor` and `prune` listed exactly those (counted separately by `work/s75/ignored_in_store.py`,
since `prune` prints 20 paths per group); `prune --yes`; `clusters --from-peaks --reuse-pairs`; `regress
session-64-linux-nas` (`work/bin/s75-start` = `s74-final`); `work/s75/diff.py`; `sweep --set-aside-like`
with the pre-prune saved index (a hard link, only read). `data/0278-prune-ignored-sets.json`.

| | Session 64's baseline | After the prune |
|---|--:|--:|
| NAS store records (revision); skip notes | 40,674 (`7210fc479da58376`); 1,926 | 40,666 (`0bd3f0c23fabc698`); 1,827 |
| Sweep 2026: indexed assets; queries with identical detections | 40,571 | 40,563; 689 of 720 |
| Stakka scan; leave-outs 3, 11 | 11/11; 8/11; 0/11, 0 wrong | the same; 1 weak detection fewer each |
| Calibrate: weakest identifying, strongest false, false accepted; at 30+ hits | 704, 282, 7; 95 | the same |
| Clusters; pair entries; file | 284; 2,875; `c4d3da2e7e37...` | the same, byte for byte |

- Of the pruned files: SKC - Recharger (Black Sun Empire Remix) at -1%, 0% and +1% each lose a
  candidate of the Lowdown Vol 6 fix's Plastic (8, 6, 7 hits), Ed Rush & Optical - Sicknote at -3%, -1%
  and +1% one of `sety/Weekenders_1_-_Nocny_Trans.mp3` (7, 7, 6); each Stakka scan loses the Plastic
  at 51:33-51:44 (6 hits). All 2 windows, sub-threshold; nothing takes their place. No cluster, pair
  entry, panel or verdict named any of the 8.
- Through the lists set aside (0242's mechanism; traced for the sweep): 21 queries refit a confident
  right detection by at most 4e-4 percentage points, hits unchanged (Abduction at all nine speeds,
  The Time The Place at four, Come Closer at -3%, The Pulse at three, Dominion at four) apart from Dominion's
  dfect-dnb upload at -5% (303 to 317 hits); Decoy at +3% and Magnetics at -5% and +5% lose a weak
  candidate (6-7 hits); in Konflict - RoadBlock at +1% a weak one gains a hit (6 to 7). Under the old lists all
  720 queries and `per_speed` equal the baseline less the pruned files' candidates. In the sweep's index
  four lists (9,911-9,919 postings) lost 3-15 postings of the 8 files and left the set aside; one of
  9,910 entered as the shortest list set aside fell to 9,910.
- In the leave-out-3 scan the three confident Clockwork detections (hits unchanged) take the fitted
  speed the full scan has had since session 64 (1.01405033 instead of 1.01405026, the corpus files).
  In the whole index the longest list kept fell from 9,938 to 9,932 postings; hash 1141698, holding
  11-14 postings of each Clockwork file, lost 6 of the pruned files (9,938 to 9,932). By inference it
  fell below the leave-out index's own cut (not run under the old lists; `scan` has no `--set-aside-like`).
**Conclusion.** No scored result changes. Every difference is of a pruned file, or comes from lists
crossing the cut (traced for the sweep, inferred for the scan). `session-75-linux-nas` replaces
`session-64-linux-nas` as the NAS-scale baseline.
