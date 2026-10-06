# 0015: related recordings in the library

**Question.** Which different recordings in the library share material
(remixes, versions, reused breaks), and how strong is the evidence between
them? These are the false candidates that grow with a real library, unlike
chance (experiment 0012).

**Method.** `gunfinger-eval related`: every file searched against the
whole library at speeds 0.98-1.02 (the clusters self-match, refactored
into `clusters::self_match`), keeping the strongest alignment of 30 hits or
more per pair of duplicate clusters. Data: `data/0015-related-recordings.json`.
The refactor left `clusters` unchanged (same 17 duplicate clusters).

**Results.** 21 pairs of different recordings with 30 hits or more; 5 reach
the possible tier, none the confident rule.

| Hits | Pair | Shared part |
|-----:|------|-------------|
| 130 | Aphrodite - From the East / Clean Decibel (drum versions, APH33) | 19 s |
| 129 | Stakka remix of Clockwork / Skynet & Stakka - Clockwork | 19 s |
| 84 | Aphrodite - Fanfare (Dubstyle) / Fanfare | 18 s |
| 67 | Aphrodite - Wikki Wikki Plate / Defjammer | 109 s, same positions |
| 67 | Sinthetix - Cryogenic / Neurotoxin | 36 s |
| 30-55 | 16 more; 9 of them Aphrodite with Aphrodite, also Kemal - Star Trails / Mechanizm (43), Tribal Warfare / Point Blank (39) | 9-94 s |

12 of the 21 pairs are two Aphrodite records: one producer reusing breaks
and parts across releases.

**Conclusions.** (1) Shared material, not chance, gives the strongest false
evidence: 130 hits in 19 s between two records, against 30 for chance at
8,122 assets. (2) It concentrates in one producer's catalogue; jungle and
breakbeat hardcore, built on a few famous breaks, will have much more of it,
so ADR 0007's measurement 3 matters more than library size. (3) The list
tells the owner in advance which possible plays are expected; a future
`gunfinger related` command could show it, and the Track model could link
such recordings.
