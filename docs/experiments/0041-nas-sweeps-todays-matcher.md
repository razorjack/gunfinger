# 0041: the sweep at NAS scale, today's matcher

**Question.** Item 6 of session 5 (NAS plan step 7), second half: sweep
seeds 2026 and 2027 at NAS scale under today's matcher, then
`calibrate`, against 0025 (seeds 2026-2029 at 262 tracks: weakest 403,
strongest false 97, 4.15x) and the skip-at-240 runs of 0040.

**Command.** `work/s5/E sweep --seed N` (26,829 and 26,828 assets),
`calibrate` with 0039's scans; `nas_calibrate.py` as in 0040 (one more
confident pair labelled by hand). Data:
`data/0041-nas-sweeps-todays-matcher.json`.

| Seed | Recalled | Wrong answers (skip at 240) | Weakest identifying | False candidates per excerpt | Wall; CPU; peak |
|---|--:|--:|--:|--:|--:|
| 2026 | 540/540 | 88 (91) | 501 | 1,646 (skip 63) | 3,972 s; 36,395 s; 8.86 GB |
| 2027 | 540/540 | 179 (174) | 403 | 2,178 (skip 101) | 4,160 s; 38,214 s; 7.26 GB |

None wrong at -8%. The weakest equal the 262-track ones (Fibre Optix -
Sin at -3%, 403). `calibrate`: 1,099 identifying, none below 200; 285 of
2,753,240 false accepted; strongest false 5,785 (Logistics upload), 0.07x.

| The 285 confident false detections | Count | Strongest |
|---|--:|--:|
| Another upload or rip with the same artist and title (37 pairs) | 276 | 5,785 |
| Bad Company - Coma (`extra/02 Coma.mp3`) ~ Spraycan (i-witness) | 8 | 342 |
| Bad Company - The Nine (`extra/A- The_Nine.mp3`) ~ The Nine (Evol Intent VIP) | 1 | 200 |

Without the same-name pairs: strongest false 342, 1.18x; without the two
pairs as well, 126 (Ant Miles - China Town ~ Sea Of Chaos), 3.20x;
Synthesis VIP is in neither draw. Audio not in the index, rips aside: 200
(The Nine). Unrelated chance, by hand: 78 (Clockwork ~ Simon Static -
Rubba Rock), 72 (The Stand ~ the untagged `1.mp3`), 66 (Sentient ~
Shere Khan - Criminal Record); skip at 240: 67-70.

**Conclusions.** (1) Recall and the weakest evidence hold at NAS scale.
(2) As under skip at 240, the margin under the rules is gone: uploads the
clusters miss count as false. (3) The Nine ~ its Evol Intent VIP reaches
the rule of 200 exactly; skip at 240 keeps it out (220). (4) Today's
matcher keeps 18-26 times more false candidates per excerpt and takes
2.9-3.1 times the wall time and CPU of skip at 240.
