# 0182: the printed tracklist on mixes of double drops

**Question.** Session 39 (roadmap "The printed tracklist"): what does the tracklist `identify` prints get right in double drops and the other scenarios of the pre-registration (brief 39, `937ceba`), and how often does a played record found only as possible inside its partner leave it (ADR 0019's cost)?
**Method.** `mixes --kind drops --tagged work/s39/tags-262.json`, seeds 2026 and 2028, 16 mixes each (every scenario once per mix; records from the 198 recordings whose tags agree). The CLI: 262 tracks with store-only copies less the held-out files (0:55-0:56 per 16 mixes); NAS scale with the NAS configuration and `--exclude-from` the NAS files of held-out recordings (3:33-3:38); `--save-dir`, then `show`. `tracklist_score.py` rebuilds the entries from each report: all 64 tracklists match line for line.

Truth plays of indexed records, 262 tracks | NAS scale:

| Plays | n | Today confident | Printed (possible) | Left out (marked) | Not found |
|---|--:|--:|--:|--:|--:|
| Alone: original, remix, returning, rips | 192 | 192 \| 192 | 192 \| 192 | 0 | 0 |
| First of two (drops, quiet, brief) | 160 | 160 \| 160 | 159 (1 under another entry) \| 160 | 0 | 0 |
| Second through (unrelated; pair) | 32; 13 | all | all | 0 | 0 |
| Unrelated drop, second inside | 32 | 25 \| 23 | 25 \| 23 | 6 \| 8 | 1 \| 1 |
| Pair drop, second inside | 19 | 11 \| 11 | 13 (2) \| 13 (2) | 6 \| 6 | 0 |
| Quiet (-9 dB, 30-90 s) inside | 32 | 4 \| 4 | 4 \| 4 | 10 \| 11 | 18 \| 17 |
| Brief (20-40 s) inside | 32 | 4 \| 4 | 4 \| 4 | 19 \| 19 | 9 \| 9 |

- Every play left out is in the table (`show -f human`: "shares material with") and the JSON. Marked plays: 105 \| 181; of a record playing then 41 \| 71 (the cost), of a related version playing 64 \| 101 (the Clockwork pair's shared passage: the mark's purpose), other 0 \| 9.
- The pair drop by direction: the remix inside the original is confident 11 of 11; the original inside the remix 0 of 8 (6 left out, 2 possible).
- Entries, 262: 445 printed, 442 right, 3 extra possible related (the pair), no confident extra. One merge: Swarm's two rips joined Compound's entry ("titles disagree: SKC & Cord - Swarm"): the plays overlap for exactly half of Swarm's and the tracks would have started 2.2 s apart, so `same_recording` holds. No split; no matching file missing (133 entries of recordings with two indexed files or more); the one entry with two names is flagged.
- NAS: 460 printed, 443 right; 2 confident extras, both Kemal - Animation (Silent Extent Remix) over the original Animation (verdict `different`, a related version since 0038); 6 possible related, 9 possible matching no panel family (by their names 7 are remixes or versions of the record playing; Techno Funk over Alien Girl and NIL - Infectious over Mystery Machine are not judged). No merge. 2 splits, both from NAS files tagged otherwise (an upload tagged artist "Jo", title "S - Afterburner"; "Clockwork (Remastered)" aligned 141 s off). Files missing in 6 of 406 entries (4 of them with a play of the file elsewhere over the entry). Of 241 entries with two names or more, 189 flagged; the rest differ in the artist only (48; ADR 0017 compares titles) or by a possible play's title (4).

**Criteria (brief 39).** P1 not met at 262 tracks; met by its letter at NAS scale through the Animation remix, which is in no panel family (the scorer's family), though by the `different` verdict and 0038 a related version of the original playing. P2 by its letter (both sides of a scenario pooled) not met: unrelated drops 7 of 128 (5.5%) \| 8 (6.3%), pair drops 6 of 64 (9.4%); the second record inside its partner alone loses 19% \| 25% and 32%. P3: 1 of 64 unrelated drops (1.6%). P4: 0 \| 2 of 512 truth plays. P5 (a file with a play over the entry, missing from it): 0 \| 4 of 406 (1.0%). P6: no confident other title unflagged; 4 possible ones (0.9%).

**What it shows.** Grouping and printing lose nothing the search finds confidently, apart from one coincidental merge: the tracklist is today's confident score plus possible lines. What leaves it is a record under a stronger one found only as possible: ADR 0019's mark is right on shared material 64 times and wrong on a played record 41 times here. A drop through to the second record costs nothing. **Does not show** how often real double drops line up the two tracks' starts; DJs align drops, so a merge by `same_recording` may be commoner there than by chance (an inference). At 262 tracks the pair scenarios rest on one pair. Data `data/0182-tracklist-on-double-drops.json`.
