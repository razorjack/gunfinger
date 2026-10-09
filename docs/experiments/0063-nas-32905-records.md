# 0063: the NAS store at 32,905 records

**Question.** Item 3 of session 9: with 16 frame bits (0062), what do
`stats` and two store-only searches cost at 32,905 records, and what do
the 5,897 new records (scene releases, label packs, uploads) change in
the two development mixes?

**Commands.** Binary `work/bin/s9-item2/` (4ab661a), store revision
`b8bb402f0ba23761`: `stats --config ~/.config/gunfinger/nas-dnb.toml`;
the development mix with 0050's command and `razorjack-2003-03-29` with
0060's (both `--store-only --playback both`). Data:
`data/0063-nas-32905-records.json`.

| Measurement | 32,905 records | Before |
|---|--:|--:|
| Postings; index; bytes per posting (delta varint) | 922.0 M; 3.70 GB; 4.01 (3.15) | 759.4 M; 3.05 GB (0050); 4.01 (3.15) (0035) |
| Buckets: mean, p99, max; fullest 1% share | 440, 5,515, 123,575; 26.0% | 360, 4,538, 106,517; 26.1% (0035) |
| `stats`: wall; CPU; peak; index build | 126 s; 71 s; 3.75 GB; 71.8 s | 78 s; 57 s; 3.07 GB (0035, 26,890) |
| Development mix: wall; CPU; peak; build | 315 s; 2,344 s; 4.76 GB; 64.7 s | 263 s; 1,954 s; 3.98 GB; 56.5 s (0050) |
| 2003 mix: wall; CPU; peak; build | 266 s; 1,854 s; 4.66 GB; 68.3 s | 229 s; 1,567 s; 3.9 GB (0060) |

Postings and CPU grow with the records (+21.4%, +20% and +18%); the
`stats` wall time includes listing the NAS.

**Development mix.** 30 plays, 28 confident, as in 0050; 25 identical,
5 now named after a new copy with the same hits (identical peak records
in the scene folders, or, for Night Lore, the Dangerous Drums 2 rip
`va-dangerous_drums_2-udfrcd002-2000-sour/cd2/`, back under a path the
owner's ignore file does not cover). The tracklist has the same 13
lines; only line 1's spelling changed.

**2003 mix.** 31 plays (26 confident) against 23 (20). All 12 tracks are
found; 21 tracklist lines against 16. From the new folders:
- other rips of the played recordings: Konspiracy and Escape Route
  (IND002, untagged), The Specialist (INFRA011 copy), Phantom Force's
  PHUD1 vinyl rip (6,513 hits against 2,233 before; it starts the track
  at 10:48, where the owner hears it blend in, 10:47);
- tracks of mixed CDs: DJ Marky's Audio Architecture (02, The Specialist,
  a line of its own at 2:52; 01, Kosheen, possible from its last 21 s),
  Contagious Drum & Bass Vol 1 (14, Phantom Force, a line at 11:21) and
  Dieselboy's System Upgrade (09, Konspiracy VIP, possible, 281 hits);
- one confident detection of a recording not played: "MC MC & Rushour -
  Music Maker (Majistrate Remix)", Contagious track 13, 1,364 hits,
  10:54-14:17, all from its last 1:11, which already plays track 14.

**Conclusions.** The new records cost what their number predicts. Most
new files in the mixes are other rips of the same recordings; the mixed
CDs among them add duplicate lines and one confident wrong title,
the case ADR 0009's ignore file exists for.
