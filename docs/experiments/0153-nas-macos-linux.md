# 0153: the NAS collection on macOS and on Linux

**Question.** Session 32, 2026-10-18: indexed from scratch on the Linux
machine (the share over SMB 3.1.1, `cifs`; FFmpeg 8.1.2; the default 32
jobs), does the NAS collection give the Mac's records and skip notes, and
the development mixes the README's scores (the Mac, 32,441-33,723 files)?
`listing_lookup.py` before the index; `index` once, run by the user (no
log; a second run wrote nothing); `store_compare.py`; `reference_paths.py`;
`stats` twice; `identify -f json` of the three mixes, each confident play
checked against its slot's references (`tracklist_references.py`).

| | the Linux machine against the Mac |
|---|---|
| Index | 42,609 audio files in 3,168.9 s: 40,683 records, 147 too short, 1,768 too long, 11 failed, the Mac's counts |
| Paths | the same 40,683 records; 160 equal only after NFC (decomposed on the Mac, precomposed on Linux); none on one side only |
| Durations, sizes | 40,683 of 40,683 equal (durations to the millisecond) |
| Peak counts; digests | 39,331 equal, 1,352 differ by 1-2 peaks (at most 0.018%); 14,901 equal, 25,782 differ (equal: m4a 40.7%, opus 35.8%, mp3 22.6%) |
| Skip notes | 1,926 with the same reason, limit, size and seconds (67 after NFC); the 11 failures: same lengths decoded and declared, same FFmpeg errors |
| Named NAS paths | 114 in the verdicts and the 3 manifests, all present byte for byte; 112 records, 2 left out by the ignore file |
| `stats` | 571.7 M peaks, 1.14 G postings, index 4,571 MB (4.01 bytes per posting); built 88.5 s, saved 11.8 s (4.4 GiB peak); loaded 5.9 s (4.3 GiB) |
| Mixes | 11/11, 12/12, 17/17 referenced tracks, nothing wrong; each slot has a confident play of a reference copy |
| Search | 56:09, 46:10, 1:06:47 of audio in 57.8, 45.7, 79.3 s; 4.8-4.9 GiB peak |

The held-out broadcast's file got its too-long note from the header (7,091.7
s declared), not decoded. Confident plays outside the references are other
rips with the slot's title; two carry other titles, both known: INFRA011's
"Sex Drive" holds The Specialist (0061), SUICIDE004's "We R The Ones 3" is
the VIP (0100). Entries and order match the manifests; Nocny's unreferenced
slot 17 gets nothing. Not compared play by play with the Mac (ADR 0020's
check): the Mac's scores are at fewer files and credit rips through its NAS
clusters. The README's Mac time (about 90 s for 46 minutes, 33,000 tracks,
10 workers) is not like for like; the index read the share over the network.

**Conclusion.** Linux reproduces the Mac's NAS store in every path (after
NFC), length, skip note and failure, and the README's three scores; peak
records differ slightly, as on the corpus (0152). ADR 0020's first risk
holds: 160 records have different bytes in their paths, so a store moved
between the machines with the library mounted would not find them. The
second was not tested for a moved store: seconds agree and the mount gives
100 ns, but the Mac's nanoseconds were not compared.
