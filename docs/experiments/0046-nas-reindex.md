# 0046: the NAS store re-indexed with the store fixes

**Question.** Item 3 of session 6: what does one `index --retry-skipped`
add with item 1's fixes (RIFF behind ID3v2, hidden folders, 1%)?

**Command.** `gunfinger --config ~/.config/gunfinger/nas-dnb.toml index
--retry-skipped` (commit c3ba1b1, release), then `prune` and `prune
--yes`: the session's only NAS reads. Data: `data/0046-nas-reindex.json`.

| Store | Before (02:45) | After (03:20) |
|---|--:|--:|
| `.peaks`, `.tags` | 27,010 | 27,042 |
| `.skip`: failed, too long, too short | 36, 1,285, 108 | 2, 1,286, 108 |
| Revision | `3d16641d2ecb6956` | `d428ee9585936326` |

28,438 audio files listed (one fewer than the first run: the partial
download in `.incomplete/` is in a hidden folder; 2 hidden folders
skipped); 387 s wall, 159 s user, 99 s system, 0.73 GB, about 5 minutes
of it listing the share. 32 records added, not 33: 26 of the 27 MP3s in
a WAV container (the 27th, a radio show in `sety/`, is now "too long")
and all 6 files damaged in their last 1.3-2.4 s. The 2 truncated files
still fail (16.2 and 21.2 s missing; tolerances 4.1 and 3.3 s). `prune`
listed exactly the stale `.incomplete` skip note and `prune --yes`
deleted it: 1,396 notes (1,429 - 32 - 1), not 1,394.

Tags: 26 of the 32 have artist and title, 20 RIFF files from their ID3
tag (`ffprobe -f mp3`). Six RIFF files have none (Drum Kru's HYB003
pair, Cause 4 Concern - Give It 2 Em, RAM Trilogy's Titan EP): their
tags hold no text FFmpeg maps, or `-f mp3` fails; not checked (the NAS
is read only by `index` and `prune`).

`map-library`: as in session 5 (247 corpus files with a copy, 238
records, 15 without). The new NAS records of four corpus files
(SKC & Cord - Swarm, SKC - Recharger (Black Sun Empire remix), Usual
Suspects - Bleach, Cause 4 Concern - Give It 2 Em) are not identical to
the corpus copies, which decode as plain MP3s. The NAS-scale index holds
262 + 27,042 - 238 = 27,066 assets.

**Conclusions.** The fixes recover 32 of 34 files (the radio show is
too long); the store is frozen at `d428ee9585936326` from here.
