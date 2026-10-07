# 0040: the sweep at NAS scale, skip at 240

**Question.** Item 6 of session 5 (NAS plan step 7), first half: the
saved sweep panels 2026-2029 at NAS scale under skip at 240, then
`calibrate`, against 0026 (weakest 658, strongest false 119, 5.53x).

**Command.** `work/s5/E --second-pass --skip-fullest 0.01 sweep --seed N`
(26,821-26,829 assets), `calibrate`; `nas_calibrate.py` splits the false
candidates (confident ones by hand from tags and paths, not listened
to). Data: `data/0040-nas-sweeps-skip-at-240.json`.

| Seed | Recalled | Wrong answers | Weakest identifying | Wall; CPU; peak |
|---|--:|--:|--:|--:|
| 2026 | 540/540 | 91 | 680 | 1,358 s; 12,153 s; 5.75 GB |
| 2027 | 540/540 | 174 | 671 | 1,366 s; 12,206 s; 5.71 GB |
| 2028 | 540/540 | 126 | 661 | 1,312 s; 11,808 s; 5.45 GB |
| 2029 | 540/540 | 114 | 732 | 1,355 s; 12,174 s; 5.62 GB |

None at -8% (an upload 3% fast would need -11%). `calibrate` with 0039's
scans: 2,179 identifying, none below 240; 523 of 256,649 false accepted;
weakest 661 (Illuminati - Melange, -3%), strongest false 5,845, 0.11x.

| The 523 confident false detections | Count | Strongest |
|---|--:|--:|
| Another upload or rip with the same artist and title (47 pairs) | 507 | 5,845 |
| Synthesis VIP ~ "Synthesis (Remix)" on the mixed CD Dangerous Drums 2 | 8 | 511 |
| Bad Company - Coma (`extra/02 Coma.mp3`) ~ Spraycan (i-witness) | 8 | 341 |

Without the same-name pairs: strongest false 511, margin 1.29x; without
those two pairs as well, 220 (The Nine ~ Evol Intent VIP), 3.0x. Audio
not in the index, rips aside: 197 (The Nine ~ its VIP). Unrelated chance
(different artist and title): 68 (Clockwork ~ Simon Static - Rubba
Rock), 67 (Sentient ~ Shere Khan - Criminal Record; Mystery Machine ~
NIL - Infectious), 70 against the untagged root file `1.mp3`.

**Conclusions.** (1) Recall holds: 2,160 of 2,160, weakest 661. (2) The
margin under the rules is gone: uploads the clusters miss (many sped
up, 0039) count as false. (3) Two pairs, probably mislabelled or shared
recordings, pass 240, for the owner. (4) Unrelated chance reaches 67-70
hits, above the possible tier of 60.
