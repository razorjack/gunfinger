# 0063: the NAS store at 32,905 records

**Question.** Item 3 of session 9: what do `stats` and two store-only
searches cost at 32,905 records (0062), and what do the 5,897 new
records change in the two development mixes?

**Commands.** `work/bin/s9-item2/` (4ab661a), store `b8bb402f0ba23761`:
`stats --config ~/.config/gunfinger/nas-dnb.toml`; the development mix
with 0050's command, `razorjack-2003-03-29` with 0060's (both
`--store-only --playback both`). Data: `data/0063-nas-32905-records.json`.

| Measurement | 32,905 records | Before |
|---|--:|--:|
| Postings; index; bytes per posting (delta varint) | 922.0 M; 3.70 GB; 4.01 (3.15) | 759.4 M; 3.05 GB (0050); 4.01 (3.15) (0035) |
| Buckets: mean, p99, max; fullest 1% share | 440, 5,515, 123,575; 26.0% | 360, 4,538, 106,517; 26.1% (0035) |
| `stats`: wall (with the NAS listing); CPU; peak; build | 126 s; 71 s; 3.75 GB; 71.8 s | 78 s; 57 s; 3.07 GB (0035, 26,890) |
| Development mix: wall; CPU; peak; build | 315 s; 2,344 s; 4.76 GB; 64.7 s | 263 s; 1,954 s; 3.98 GB; 56.5 s (0050) |
| 2003 mix: wall; CPU; peak; build | 266 s; 1,854 s; 4.66 GB; 68.3 s | 229 s; 1,567 s; 3.9 GB (0060) |

**Development mix.** 30 plays (28 confident) as in 0050, 25 identical;
5 are named after new copies with the same hits, one the Dangerous
Drums 2 rip `va-dangerous_drums_2-udfrcd002-2000-sour/cd2/`, back under
a path the ignore file does not cover. Same 13 tracklist lines.

**2003 mix.** 31 plays (26 confident) against 23 (20); all 12 tracks,
21 tracklist lines against 16. From the new folders:
- other rips of the played recordings: Konspiracy and Escape Route
  (IND002, untagged), The Specialist (INFRA011), Phantom Force's PHUD1
  vinyl rip (6,513 hits against 2,233; the track starts at 10:48, where
  the owner hears it blend in, 10:47);
- mixed-CD tracks: DJ Marky's Audio Architecture (02, The Specialist, a
  line of its own at 2:52; 01, Kosheen, possible from its last 21 s),
  Contagious Drum & Bass Vol 1 (14, Phantom Force, a line at 11:21),
  Dieselboy's System Upgrade (09, Konspiracy VIP, possible, 281 hits);
- a confident title not played: Contagious track 13, "MC MC & Rushour -
  Music Maker (Majistrate Remix)", 1,364 hits at 10:54-14:17, all from
  its last 1:11, which already plays track 14.

**Conclusions.** Cost follows the records (postings +21.4%, CPU +18-20%).
New files in the mixes are mostly other rips; mixed CDs add duplicate
lines and a wrong title.
