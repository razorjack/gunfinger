# 0276: a passage query, which files hold a break: retrieval and noise

**Question** (brief 74, pre-registered in `56a2ec0`, cases in `46e441c` before any run). Can a 20 s passage of one file, searched against the whole index, list the files that hold it, strongest first, with little unrelated material above them?
**Method.** `gunfinger-eval break-search` (`work/bin/s74-run/`, `716fbad`): the passage's stored peaks on the turntable ladder (±8%, 41 rungs), one pass, every list and filtered (0277); each file once at its strongest detection of 20 hits or more, ranked by hits; own cluster apart; the reach of the 20 strongest others (the whole query against that file alone). Cases (`data/0276-cases.json`): A the panel's 22 shared-material directions, B the 7 drum-loop pairs of 0225, C six spans of Special Branch, D 32 `parts` candidates heard as one recording whose edge part is another title, N1 40 random passages, N2 40 intros. `scripts/analysis/break_search.py`; data `data/0276-break-search.json`.

| NAS scale (40,696 files), all lists | Scored | Found | First | First 5 | Others at 20 hits, median (max) |
|---|--:|--:|--:|--:|--:|
| A, panel shared material | 20 | 18 | 11 | 16 | 3 (11) |
| B, drum-loop verdicts | 7 | 5 | 4 | 5 | 1 (2) |
| C, Special Branch (Amen variants) | 6 | 6 | 6 | 6 | 124 (232) |
| D, parts edges heard as one recording | 31 | 30 | 4 | 24 | 18 (403) |
| Positives pooled | 64 | 59 (92%) | 25 (39%) | 51 (80%) | |
| N1 random; N2 intros | 40; 40 | | | | 0 (207); 0 (11) |

- Not scored: T-Minus's vocal mix has no record (pruned), the Ski Mask's expected file is ignored. Missed: both Watermelon directions (the panel's passage is key-locked; the ladder searched is the turntable one), drum loops 5 and 7 (130 and 85 hits over 2 minutes in `pair`), Omni Trio - Nu Birth.
- At 262 tracks (85 queries): 4 of 5 positives found, all first; random passages and intros list at most 2 other files.
- The registered null fails as a null: *T*, the 95th percentile of N1's strongest other file, is 726 hits against a median 72 for the found expected files, because 13 of 40 random passages (8 of 40 intros) list another rip of their own recording outside the NAS clusters (which hold only the corpus recordings' rips), reaching 60 s or more of the query.
- Famous breaks: a passage of Knowledge & Wisdom - Kill A Sound lists 403 files at 20 hits (1.0% of the index), Bigger Star - Raid 390, J Majik - Your Sound (Remix) 348, Krome & Time - License 347, Special Branch 305-325 s 232; random passages from John B - Double-J and Renegade - Terrorist list 207 and 120. In those lists the expected file is one holder among hundreds (License's at rank 28), at relative speeds across the ladder (0.93-1.05 in License's first 30), 40-110 hits each.
- Exploratory, not registered: with files whose reach is 60 s or more (rips and versions by audio) set apart like the own cluster, 39 of 64 positives rank first and 53 in the first 5; the strongest file reaching less than 60 s on a random passage has a median 0 hits, 90th percentile 29, maximum 111 (Double-J's break).
**By the registered bar** (the better mode, all lists): (1) found 92% and first five 80% hold, first 39% fails (50%); (2) the median random passage lists 0 other files, but *T* exceeds the expected files' median, so (2) fails; (3) holds (0277). Not worth a design note and an ADR as registered.
**What it shows.** The query finds the judged shared material and Special Branch's partners, and it lists famous breaks' holders by the hundred at similar strength; what ranks above an expected file is mostly other rips, other versions of the query's tune and other holders of the same break, not chance. Hits do not tell these apart; reach tells rips and versions from passage-only holders. Whether the passage-only holders all share the break is a question for the ear.
