# 0225: what the drum-loop pairs share, and how much of it is set aside

**Question** (step 5 of `docs/brief-50.md`, a case study). Of the hashes
two different records built on one drum loop both hold, how many lie in
the fullest lists the first pass sets aside, now and under copies?
**Method.** `fullest --shared-by` (new, tested): for each pair, the
distinct hashes both files hold and those set aside, on the whole index
(no file left out). Pairs: the seven verdict lines of the Aphrodite
family and the Klute VIP judged `different` (Wikki Wikki Plate ~
Defjammer, two Tower Bass mixes, Stalker, Style From The Dark Side,
the VIP against two rips of the Klute Remix). Padding as 0222 and 0223.
Control, added after the first readings (exploratory): 20 seeded random
pairs of NAS files at level 0. Data `data/0225-drum-loop-pairs.json`.

| Index | Pairs | Shared hashes | Set aside: median (range) | The files' postings set aside |
|---|---|--:|--:|--:|
| 262, level 0 | Wikki ~ Defjammer | 2,227 | 27.2% | 21.4%, 17.8% |
| 262, panel 9; share 9 | Wikki ~ Defjammer | 2,227 | 24.4%; 24.1% | 15.2-18.2% |
| NAS, level 0 | the 7 related | 1,793-3,850 | 28.8% (23.6-35.1%) | 17.5-25.3% |
| NAS, panel 9; share 6 | the 7 related | the same | 28.8%; 28.9% (each within 0.2 points) | the same |
| NAS, level 0 | 20 random pairs | 334-2,010 | 42.4% (26.5-49.9%) | 11.0-26.1% |

Wikki Wikki Plate ~ Defjammer has the largest share set aside of the
related pairs (35.1% of 2,227 at NAS scale; each file 25%, the most of
the 12 files); the two Tower Bass pairs 30.8% and 31.6%. At NAS scale
Wikki Wikki Plate is itself among the share selection's copied files
and Tower Bass (Millennium Mix) among the panel's, and their pairs move
by at most 0.2 points; at 262 tracks neither file is copied.
**What it shows.** What related records built on one loop share sits in
the lists set aside more often than their own material (1.3 to 1.7
times the files' mean share) but less often than what unrelated files
share by chance (median 42%): the common hashes chance gives two files
are the fullest ones, while a shared loop also brings rarer hashes. So
the filter keeps about 70% of what these pairs share for the first
pass, and copies do not change that at NAS scale. At 262 tracks copies of
other files push some of their lists out of the set (27% to 24%).
**Does not show** how many hits these pairs make at search time, or
whether a famous break used by many records behaves the same: these are
two-file families. The pairs remain shared material for ADR 0019.
