# 0216: the registered rules for marked plays on fresh seeds

**Question.** Session 48 (brief 48, 02:43 and 03:05): on test seeds no feature was tuned on, does a rule fixed in advance print the records playing under a partner that ADR 0019 leaves out, without letting shared material into the tracklist?
**Method.** Seeds 2032 and 2034 (the first from 2030 up whose sweep panel keeps both Clockwork records indexed), `mixes --kind drops --verify --ladder turntable --tagged work/s39/tags-262.json` and `--other-pairs` (renders 1:04-1:31 per 16 mixes), the CLI as session 39 and 45 ran it: 262 tracks on store-only copies less the held-out files (0:42-0:44), NAS scale on the NAS store's records less the held-out recordings' files (`--store-only --saved-index off --exclude-from`, 2:21-2:23). `pair` on 34 and 117 file pairs (0:26; 0:02). Features and rules exactly as registered; R4 and R5's bars from 0215.

| Rule (test seeds) | Unrelated playing printed, 262 \| NAS | Pair drops printed | Shared let in | NAS "other" printed |
|---|--:|--:|--:|--:|
| R1 F1 < 60 (no possible alignment between the two files) | 25/25 \| 101/101 | 0/9 \| 1/19 | 0/69 \| 0/117 | 4/31 |
| R2 F2 > 10 s or none | 25/25 \| 101/101 | 9/9 \| 19/19 | 7/69 \| 21/117 | 17/31 |
| R3 F3 >= 60 | 25/25 \| 101/101 | 8/9 \| 18/19 | 15/69 \| 22/117 | 11/31 |
| R4 F4 < 0.172 | 0/25 | 0/9 | 0/69 | |
| R5 F5 > 0.00344 | 20/25 \| 86/101 | 0/9 \| 1/19 | 2/69 \| 0/117 | 4/31 |

- Unrelated playing records: brief 14 \| 60, quiet 6 \| 23, unrelated drop inside 5 \| 18 marked plays. Separation (AUC against shared, 262 \| NAS): F1 1.00 \| 1.00, F2 0.99 \| 1.00, F3 0.96 \| 0.95, F5 0.90 \| 0.94, F4 0.16 (the wrong way, as in 0215).
- R1's margin: the unrelated records' file pairs reach at most 16 \| 17 hits, the shared plays' at least 112 \| 93 (the bar is 60). The pair drop R1 prints is the Monotone remix inside Watermelon (13 hits between the files, as in 0215).
- Every shared play here is the Clockwork family's (the original, its remix and, at NAS scale, two more rips of the original): 69 \| 117 plays of one relation. The other pairs' mixes held no shared marked play on these seeds.
- The 4 "other" plays R1 prints at NAS scale are NIL - Infectious over Ed Rush & Optical - Mystery Machine (seed 2032, mix-07, 103 s and 36 s, in both mix sets, which play the same records there; 73-77 hits; the files share 41 hits), as in 0182's unjudged list.
- R2 and R3 let in the Clockwork rips found where the remix plays elsewhere (F2 up to 155 s; F3 up to 155 hits), more at NAS scale, where three rips of the original are indexed.

**Criteria (brief 48).** R1 meets the print bar (100% of 126 unrelated playing records, against 80%) with no shared play let in at either scale; step 5 decides whether it is worth an ADR proposal. R2, R3 and R5 let shared plays in; R4 prints nothing. No rule tells a record inside its relative from shared material: R2 and R3 print the pair drops and let in 7-22 shared plays.
**What it shows.** "Do the two files share material at all" (F1) reproduces on fresh seeds: unrelated records under a partner have no alignment with its file, shared material does. **Does not show** other families: the shared class is one relation (Clockwork) in both 0215 and 0216; a related version whose pair search finds under 60 hits (the Monotone remix and Watermelon: 13) would be printed. Data `data/0216-marked-features-test.json`.
