# 0043: posting lists and famous breaks at NAS scale

**Question.** Item 8 of session 5 (NAS plan step 9): which records hold
the fullest 1% of posting lists, and are they the false candidates, with
and without skip at 240?

**Command.** `work/s5/E fullest` (new, `crates/gunfinger-eval/src/
fullest.rs`: per record, its postings in the lists `skipping_fullest
(0.01)` sets aside; 55 s, 3.14 GB), joined with the reports of 0039-0041
(`scripts/analysis/fullest_join.py`; sweeps 2026-2027 today's, 2026-2029
skip). Data: `data/0043-nas-posting-lists-and-breaks.json`.

| Posting lists, 26,914 assets | Figure |
|---|--:|
| `stats` (0035): hash slots; mean; p99; max | 2,097,152; 360; 4,538; 106,517 |
| `stats`: the fullest 1% of slots (empty ones counted) hold | 26.1% |
| Non-empty lists; set aside by skip at 240 (6,747 postings or more) | 1,063,552; 10,635 |
| Postings in the set-aside lists | 141.5 M of 755.9 M (18.7%) |
| A record's share there: median; p90; p99; max | 18.6%; 22.8%; 26.5%; 34.0% |
| The 1% and 10% of records with most set-aside postings hold | 1.9%; 15.1% of them |
| Rank correlation, share against false-candidate appearances: today's; skip | 0.72; 0.24 |
| The top 1% (269 records): share of appearances, today's; skip | 2.6%; 1.7% |
| Records ever a false candidate (3 scans; sweeps: today's 2, skip 4) | 26,852; 22,979 |
| Own excerpts of the heaviest tenth (108, median share 25%): median hits today's; skip; below 240 | 826; 1,028; 0 |

Highest shares: Axis Of Evil - Ignition Sequence 34.0%, Lemon D - Ghost
Stories 33.9%, Wave & Sebass - Turtle in the Map 32.1%, MC MC & Rushour -
Music Maker 32.0% (two uploads), Bad Company - Seizure 31.5% (two). Most
frequent false candidates under today's matcher: Fierce & Usual Suspects
- Sawn Off (573 times; 44 under skip), Music Maker (540; 16), Amex -
Highhead (528; 22). Under skip at 240: dBridge - Violent Plains (174),
dRamatic - Caroline (160), shares 14-20%. An excerpt's hits under skip
over today's do not depend on its record's share (rank correlation -0.05).

**Conclusions.** (1) The fullest lists are spread over the collection:
no small set of break-heavy records holds them. (2) Under today's matcher
records with more postings in those lists are false candidates more
often; skip at 240 removes most of that (0.72 to 0.24) and keeps 4.3% of
the false candidates of the same two draws (0041). (3) Skipping hides no
record when it is played: the heaviest tenth keep their hits.
