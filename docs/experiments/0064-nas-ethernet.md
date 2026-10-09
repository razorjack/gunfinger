# 0064: the NAS over Ethernet

**Question.** Item 4 of session 9: `doctor` and `index` over Ethernet
(1000baseT, SMB) against Wi-Fi (0035); repeatability; the failed files.

**Doctor** (NAS configuration): 43.9 s against 2:21 over Wi-Fi, after
two earlier listings (the first about 32 s). 34,470 audio files: 32,905
records, 1,565 passed over (13 failed, 129 too short, 1,423 too long);
none unindexed, out of date or gone.

**Throughput.** Three new folders by seed (`work/s9/pick_folders.py`),
each its own library in a scratch store, `--config /dev/null`
(`work/bin/s9-item2/`). Data: `data/0064-nas-ethernet.json`.

| Library | Files | Decoded | Wall | Mbit/s | s per file | s per hour of audio |
|---|--:|--:|--:|--:|--:|--:|
| `__full_scene/1999/1999-07` | 35 | 235 MB, 3.5 h | 6.2 s | 305 | 0.18 | 1.75 |
| the same again, from the SMB cache | 35 | | 5.8 s | 326 | 0.16 | 1.64 |
| `__full_scene/=Industry Recordings=` | 35 | 331 MB, 3.6 h | 6.7 s | 395 | 0.19 | 1.85 |
| `__youtube_archivists/dnbfreak0` | 306 | 2,028 MB, 34.3 h | 66.1 s | 245 | 0.22 | 1.93 |
| All three, first runs | 376 | 2,594 MB, 41.4 h | 79.0 s | 263 | 0.21 | 1.91 |
| 0035: Wi-Fi; local disk (corpus) | | | | 84; 423 | 0.64; 0.20 | 6.2; 1.8 |

`en7` received the files' bytes in the last two runs (347 and 2,126
MB), 0.2 MB in the cached one. Workers kept 8.3-8.8 of 10 cores busy:
indexing is CPU-bound, at the local-disk rate per hour of audio; Mbit/s
is below 0035's local figure because these files average 132-202 kbit/s
against 211. The owner's run of the 5,897 new records agrees: at most
1.88 s per hour of audio (48.7 GB, 614 h written in 1,153 s).

**Repeatability.** All 374 scratch records equal the NAS store's (length,
peaks, peak digest, size; 4 indexed over Wi-Fi), as do both skip notes.

**Failed files.** All 13 (TI028's Cobra twice) decode 5-71 s short of
the length ffprobe declares, 12 with "Header missing"; four hold zeroed
blocks the size of the missing audio (incomplete transfers), the rest
bytes that are not MP3 frames. All are damaged.

**Conclusions.** A full re-index (3,433 h) over Ethernet: about 1.8 h,
CPU-bound (5.9 h at the Wi-Fi rate). Peaks repeat bit for bit.
