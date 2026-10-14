# 0100: the third development set, DJ Vertical on Nocny Trans, 2002-08-23

**Question.** Session 18, step 2: what is found in the third
development set (a 66:48 broadcast via YouTube, tracklist written without
Gunfinger), which plays versions whose other versions are indexed?

**Library first.** `clusters --from-peaks` with the three manifests (370
queries, 1,554 s): 268 -> 271 clusters, new for Stab 2 Def, Mutated, the
Konflict Nephelim, the VIP with SUICIDE004's "We R The Ones 3" (100%),
alien5ive's "FUZE 29 cannibal" with two "Sonic - Change" uploads (100%;
butch-french's Cannibal is apart); stored scores unchanged. `pair`, 40
pairs: the TI029 C rips are one recording, matching neither the VIP (6
hits) nor the 1999 original (7). Against the played version: 1999's
Descent 32, Invid 25, Gridlok 13-38, Klute Remix 166-263, others 0-27.

| `scan` (ladder both) | Identified | Wrong | Possible, no track | Wall |
|---|--:|--:|--:|--:|
| 262 tracks | 9/9 | 0 | 0 | 47 s |
| NAS (index rebuilt) | 17/17 | 0 | 4 | 173 s |
| NAS, `--leave-out 3`, seeds 2026-2028 | 14/17 each | 0 | 4 | 71-74 s |

**Causes.** No miss, no wrong play (weakest: Anthrax 364-386 hits).
Blindside's tease is a possible segment (38:00-38:48)
of the play confident from 39:31. The possible plays share material with
the confident play over them: Mampi Swift - 2000 (282; 827 with Stab 2 Def
in `pair`), the Klute Remix in two files (196, 161), the untagged `1.mp3`
(161; part of Cannibal 6% faster). Left-out slots stay empty (other
versions 6-19 hits; the Klute Remix 96-116 with or without the VIP).
Track 17 (1:01:25-1:04:40): nothing at least possible in any run.

**`identify`** (NAS store, the configuration, turntable; 135 s,
saved index rebuilt; `--playback both` the same): 21 entries. Each track
has one confident entry holding a right file, 16 named by the right
title; entry 15: "Klute - We R The Ones 3 (titles disagree: Klute - We R
The Ones VIP)". Only the four possible entries, marked as sharing
material, show other titles alone; the Klute Remix is listed twice.

**Conclusions.** 17/17, 0 wrong at NAS scale, with leave-outs too; the
indexed versions share too little audio to come near the rule, so there
is nothing for the containment rule (0101). Data `data/0100-*.json`.
