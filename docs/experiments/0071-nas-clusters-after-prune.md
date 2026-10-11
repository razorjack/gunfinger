# 0071: the NAS clusters after the prune, at 33,596 records

**Question.** Item 4 of session 11: what the eight new ignore patterns,
1,252 new uploads and the test manifest's seven NAS references change
against 0066. **Command.** 0066's, with `--manifest
ed-rush-optical-essential-mix` (aa0862a, store `f9f9bc706195a71b`; the
panel's two pruned sources left out). `clusters_change.py`; data
`data/0071-nas-clusters-after-prune.json`.

| NAS clusters | 0066 | Now |
|---|--:|--:|
| Records; queries (rounds) | 32,441; 639 (347, 290, 2) | 33,596; 648 (352, 294, 2) |
| Clusters; files in them (NAS files) | 264; 867 (613) | 267; 874 (620) |
| Weakest same; strongest different (unjudged) | 0.803; 0.794 | 0.803; 0.793 |
| Borderline pairs (40-80%); without a verdict | 61; 33 | 44; 17 (2 new) |
| Sparse joins; joins cut | 0; 1 | 1; 1 (the same Sonar join) |
| Instructions; cycles (10^12) | 781; 209 | 827; 226 (+5.9%, +8.0%) |
| Wall; CPU; peak | 8,063 s; 75,059 s; 5.41 GB | 29,130 s; 122,481 s; 6.19 GB |

The machine had other load (load average about 350): the same work
took 3.6 times as long, so the wall time is not comparable.

**Changes.** 13 corpus clusters change: six lose a pruned mixed-disc
track (four of Inside The Machine's disc 2, Essential Rewindz's Warhead,
Blazin's "Skynet & Stakka - Mix"); seven gain a new upload (Funktion
(Original Mix), two Decoder remasters, The Mummy, Bleed, a Sugar Crashing
remix, and "Ostmet" under Merkaba). Of 0066's 33 unjudged borderline
pairs, 18 go with a pruned file (the Essential Rewindz pack items among
them) and 15 stay. Two are new: the test reference for Mind Overload
(RAMM23) against Molten Beats' `111-ram_trilogy-mind_overload-sour.mp3`
(79.3%, 3,321 hits, the strongest unjudged pair), and Ryme Tyme's T-Minus
against its "Unreleased Vocal Mix" (58%). The sparse join is the weak
measurement (736 hits, 1.84 hits/s) of the Phoenix join measured at
29,114 hits the other way; the median rose from 18.1 to 18.9 hits/s.
Of the test manifest's new references, Iron Lung joins Molten Beats'
`108` (80%) and System Error (Y2K) its CD copy (100%); five join nothing.
28 of the 80 panel sources join another file, none a corpus recording.

**Conclusions.** The patterns removed what they were written for. No
new join is doubtful; Mind Overload's pair is one to judge by ear.
