# 0036: the content map of the corpus on the NAS

**Question.** Item 2 of session 5 (NAS plan step 3): which NAS file is
each corpus file, by content? The evaluation at NAS scale replaces one
copy per corpus file with the corpus file, so the index holds the NAS
library's content.

**Command.** `gunfinger-eval --other-peaks-dir
~/.local/share/gunfinger/nas-dnb-peaks map-library`: identical peak
records (duration from the headers, then the peaks), no audio. 5.1 s,
79 MB. Data: `data/0036-nas-content-map.json` (the map, file names only).

**Result.** 247 of the 262 corpus files have an identical record on the
NAS, all of the same size. 31 have two: the NAS keeps the rip in two
release folders (28 pairs; the corpus itself holds the two Aphrodite
Fanfare rips and SKC's Recharger remix twice). 15 have none:

| Corpus files without a NAS copy | Why, from the store |
|---|---|
| Ed Rush & Optical - Compound, Dozer, Fixation, Glass Eye, Lithosphere, Mystery Machine, Point Blank, Wormhole; Fortran - Splinter; Optical - Millennium, Slip Thru (11, all at the corpus library's root) | no record and no skip note under these names: the NAS dnb library holds no copy. Other rips by name: `dfect-dnb/Optical - Millennium.m4a`, `dfect-dnb/Optical - Slip Thru.m4a`; remixes and bootlegs of Compound, Dozer, Fixation, Slip Thru |
| SKC & Cord - Swarm; SKC - Recharger (Black Sun Empire remix); Usual Suspects - Bleach; Cause 4 Concern - Give It 2 Em (4) | the NAS copy failed to open: "Invalid data found when processing input", the failure of the 27 MP3s in a WAV container (inference; the corpus copies decode) |

The 15 stay in the NAS-scale index under their corpus names, so it holds
26,890 + 15 = 26,905 assets (82.1% of 32,768).

**Conclusions.** (1) The map works from the store alone. (2) The owner's
"every corpus file is a copy of a NAS file" holds for 247: the
11 root files are not in the NAS dnb library, and 4 NAS copies did not
decode. Dozer, Fixation and Slip Thru are test-set references, so a
NAS-scale test evaluation keeps them as corpus files (as here) or needs
them on the NAS; no development reference is affected. (3) The 31 pairs
are byte-identical copies (0035); the clusters must hold both.
