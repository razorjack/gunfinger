# 0185: the printed tracklist with three more shared-material pairs, at NAS scale

**Question.** 0182's pair scenarios rest on Clockwork alone. With the panel's three other families whose tags agree (brief 39, 01:15: Pyro - Time Is Broken ~ Ignorance, Ryme Tyme - T-Minus ~ its Unreleased Vocal Mix, Ed Rush & Optical - Watermelon ~ 300 Baud - Monotone (Mobiuz & Cycom Remix)), what does the tracklist print for a record and its relative apart and together?
**Method.** `mixes --kind drops --other-pairs --tagged work/s39/tags-other-pairs.json`, seeds 2026 and 2028, 16 mixes each (renders 1:37-1:39). Each seed holds one family's original out, so each plays two families (2026: T-Minus, Watermelon; 2028: Pyro, Watermelon), both directions. The CLI at NAS scale as in 0182 (3:25-3:35 per seed), scored as 0182 with a record indexed when the NAS index holds a file of its recording; all 32 tracklists match their rebuilt entries. Code: `check.sh` green, both regressions identical (1:10; 5:59).

| Pair plays (NAS scale) | n | Today confident | Printed | Under the partner's entry | Left out (possible, marked) |
|---|--:|--:|--:|--:|--:|
| Alone (the panel's played and related records) | 64 | 64 | 64 | 0 | 0 |
| Drop, first record | 32 | 32 | 29 | 3 (Pyro) | 0 |
| Drop, second through | 13 | 13 | 13 | 0 | 0 |
| Drop, second inside: T-Minus inside its vocal mix | 2 | 0 | 0 | 0 | 2 |
| the vocal mix inside T-Minus | 4 | 4 | 4 | 0 | 0 |
| Monotone remix inside Watermelon | 5 | 2 | 2 | 0 | 3 |
| Watermelon inside the Monotone remix | 4 | 4 | 4 | 0 | 0 |
| Pyro, either inside the other | 4 | 4 | 0 | 4 | 0 |

- Pyro's two records share their whole length (the panel: 910 hits over 188 s at speed 1.0). Together they line up, so every Pyro drop is one entry under `titles disagree:` (7 entries, all flagged); apart, each is found confidently while the other plays (8 + 8 + 7 wrong confident plays on its files), but those plays line up with the right one and join its entry, so no extra line is printed.
- T-Minus and its vocal mix: apart, each is printed as a confident extra entry while the other plays (4 + 4; related versions, as ADR 0019 expects).
- The other scenarios repeat 0182's NAS rows within 1-2 plays (unrelated partner inside: 25 printed, 6 left out, 1 not found; brief: 6, 19, 7; quiet: 4, 9, 19).
- New: Digital & Spirit - Crash, confident on two files (272 and 252 hits in one 54 s segment of 6 windows; a third file 229 hits, possible) over the first 56 s of the T-Minus vocal mix playing alone before a drop (seed 2026, mix-00, 15:44). Nothing in the repository relates the two; by P1's letter a confident extra naming a recording not playing. Also Kemal - Animation's remix twice again (0182).
- Marked plays: 78; 57 of a record playing (the cost), 1 of a related version, 20 neither.

**Criteria (brief 39, as 0182).** P1 met by Crash unless it shares material with the vocal mix (a listening question). P3: no merge across families; 7 within one (Pyro, flagged). P2 for pair drops, both sides pooled: 12 of 64 (19%: 5 left out, 7 under the partner's entry), so P2 is met here; the second inside alone 9 of 19. P4-P6 as 0182 (one split; files missing 6 of 343 entries; no unflagged confident title).

**What it shows.** Whether a record inside its relative is printed depends on the pair: the vocal mix and Watermelon are printed every time, T-Minus inside its vocal mix and the Monotone remix inside Watermelon are left out in 5 of 7, and Pyro's near-identical records become one flagged entry. 0182's Clockwork result (the original inside its remix never confident, 6 of 8 left out) is one pair's; Pyro's 7 under the partner's entry are named there under `titles disagree:`. Data `data/0185-tracklist-other-pairs.json`.
