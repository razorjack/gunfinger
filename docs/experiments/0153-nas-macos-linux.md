# 0153: the NAS collection on macOS and on Linux

**Question.** Session 32, 2026-10-18: indexed from scratch on the Linux
machine (the share over SMB 3.1.1, `cifs`; FFmpeg 8.1.2; 32 jobs), does the
NAS collection give the Mac's records and skip notes (`mac-reference/`),
and the development mixes the Mac's tracklists (README, "Results")?
`index` once, run by the user (no log; a second run found all up to date
and wrote nothing); `store_compare.py`; `reference_paths.py`; `stats`
twice; `identify -f json` of the three mixes, each confident play checked
against its slot's references, corpus references mapped to their NAS
copies by identical peak records (`work/s32/tracklist_check.py`).

| | Linux against the Mac |
|---|---|
| Index | 42,609 audio files in 3,168.9 s: 40,683 records, 147 too short, 1,768 too long, 11 failed, the Mac's counts |
| Paths | the same 40,683 records; 160 equal only after NFC (decomposed on the Mac, precomposed here); none on one side only, no file added |
| Durations, sizes | 40,683 of 40,683 equal |
| Peak counts; digests | 39,331 equal, 1,352 differ by 1-2 peaks (at most 0.018%); 14,901 equal, 25,782 differ (equal: m4a 40.7%, opus 35.8%, mp3 22.6%) |
| Skip notes | 1,926 with the same reason, limit, size and seconds (67 after NFC); the 11 failures: same lengths decoded and declared, same FFmpeg errors |
| Named NAS paths | 114 in the verdicts and the 3 manifests, all present byte for byte; 112 records, 2 left out by the ignore file |
| `stats` | 571.7 M peaks, 1.14 G postings, index 4,571 MB (4.01 bytes per posting); built 88.5 s, saved 11.8 s (4.6 GB peak); loaded 5.9 s (4.5 GB) |
| Mixes | 11/11, 12/12, 17/17 referenced tracks, nothing wrong; each slot has a confident play of a reference copy |
| Search | 56:09, 46:10, 1:06:47 of audio in 57.8, 45.7, 79.3 s; 5.1-5.2 GB peak |

The held-out broadcast's file got its too-long note from the header (7,091.7
s declared), not decoded. Confident plays outside the references are other
rips with the slot's title; two carry other titles, both known: INFRA011's
"Sex Drive" holds The Specialist (0061), SUICIDE004's "We R The Ones 3" is
the VIP (0100). The tracklists' entries and order match the manifests;
Nocny's unreferenced slot 17 gets nothing. Times: the README's Mac figure
(about 90 s, 4.8 GB, a 46-minute mix, 33,000 tracks, 10 workers) is not
like for like; the index read the share over the network.

**Conclusion.** Linux reproduces the Mac's NAS store in every path (after
NFC), length, skip note and failure, and the three tracklists; peak records
differ slightly, as on the corpus (0152). ADR 0020's first risk holds: the
two machines name 160 records with different bytes, so a store moved
between them with the library mounted would not find those records. The
second does not: times keep 100 ns on this mount.
