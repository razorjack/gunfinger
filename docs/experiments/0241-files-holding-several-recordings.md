# 0241: library files that may hold more than one recording

**Question.** Which NAS files have a stretch at their start or end that
lines up inside another recording, the rest being a different one
(pre-registration `docs/research/2026-10-27-files-holding-several-recordings.md`)?
**Method.** `gunfinger-eval parts` with the pre-registered rules, tiers
and 0065 leave-outs (`work/s58/main-run.sh`, reports
`work/s58/out/tier-*.json`, tally `work/s58/tally.py`): the 46 `[CUT]`
records whole; the first and last 60 s of all 6,293 other `i-witness-dnb`
and 6,088 release-folder records, and of 3,200 of the 28,236 other
uploads (a path-order prefix) before the 90-minute budget ran out.
15,627 files, 32 threads, 6.9 GB. Data: `data/0241-parts-candidates.json`.

**Result.** 205 candidates, 186 worth the ear; 17 with edge parts at
both ends, 133 with a second part elsewhere.

| Kind (by folder and parts) | Candidates | Worth the ear | Edge file shares a title word (post hoc) |
|---|--:|--:|--:|
| Cut from a mix | 12 | 12 | 5 |
| A mixed CD's track | 117 | 106 | 67 |
| An upload opening with another record | 37 | 31 | 29 |
| Other (an upload ending with another record) | 39 | 37 | 27 |

**`[CUT]` uploads.** 54 audio files in the channel: the ignored Space
Cake file, 46 with a record, 7 with a skip note. 3 of the 46 are
candidates, all worth the ear: Spider (Optiv & BTK Remix) opens with 28 s
of Nero - Satisfy (155 hits), Break - Wine ends with 41 s of All Thieves
- Stars (152), Planet Dust (DC Breaks Remix) lines up with the original
over 0:11-0:47 (100; shared material more likely).

**Known cases** (saved pairs, `work/s58/known-cases.txt`): the Space Cake
cut and the track-fix Drumtrip (with a second part) are worth the ear;
the T-Minus upload, Kraken and track-fix Propaganda are not (no saved
pair at the edge, or 10 s short). Controls: no Listen Up track; 7 tracks
of unmixed discs, 5 of Vintage Dread 2000's first disc.

**Conclusions.** The rules find transitions (two cuts, two mixes indexed
as tracks, Vintage Dread's first disc), but most strong candidates are a
version of the same tune starting inside the original: 128 of 205 by the
crude name split, 14 of the pack's 20. Hits do not tell them apart.
