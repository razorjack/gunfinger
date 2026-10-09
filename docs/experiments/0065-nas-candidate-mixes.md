# 0065: candidate mixes and mixed CDs on the NAS

**Question.** Item 5 of session 9, without `identify`: what are the 1,423
files passed over as too long, and which new folders look like mixed CDs?

**Method.** Lengths and tags with `ffprobe` (`work/s9/probe_too_long.py`,
42 s over Ethernet), kinds from paths and titles. Boundaries of every
folder with at least 4 records from the peak records alone
(`scripts/analysis/folder_edges.py`, 94 s): a boundary is joined when
the last second of a file and the first second of the next both hold
more than half the file's mean peak density. Lists: `work/s9/
candidate-mixes.tsv`, `work/s9/mixed-cd-candidates.tsv`. Data:
`data/0065-nas-candidates.json`.

**Too long.** 1,423 files, 1,264 h; 15:00 to 4:29:54, median 45.9 min
(62 under 20 min, 223 to 30, 393 to 45, 275 to 60, 324 to 90, 91 to
120, 54 longer); 137 new, all in `__full_scene/`. One RIFF-wrapped MP3
in `sety/` has no length from plain ffprobe.

| Kind | Files | Median | Hours |
|---|--:|--:|--:|
| YouTube, game videos (alien5ive) | 561 | 34 min | 343 |
| YouTube, titles naming a mix, set or show | 366 (155 naming 1990-2003) | 68 min | 452 |
| YouTube, other (albums, unclear) | 247 | 62 min | 261 |
| Scene folders 1999-2000: live sets, radio shows, mixed CDs | 136 | 45 min | 104 |
| `sety/`: Polish radio shows and sets | 96 | 48 min | 86 |
| Root and single-mix folders | 17 | 57 min | 18 |

**Mixed CDs.** Known mixed discs score 0.92-1.00 joined boundaries
(Dangerous Drums 2's new copy: CD2 1.00, CD1 0.88; the folders named
"mixed by"; Contagious, DJ Marky, Dieselboy from 0063); vinyl rips and
Drum & Bass Assassins 0.00-0.11. 33 of 468 folders score 0.75 or more,
30 of them new; releases that hold a mixed disc beside unmixed ones
score in between (Blazin 0.35, Inside The Machine 0.48, Molten Beats
0.52). Gapless albums would score high too, so the list is for the ear.

**Conclusions.** Most long files are mixes, but 561 are game videos. The
1999-2000 scene sets and the single-file mixed CDs date from the
owner's main era. The boundary measure finds candidates the names miss
(Contagious, confirmed in 0063; Bootleggers 3 and others, unconfirmed).
