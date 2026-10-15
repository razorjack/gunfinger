# 0118: the same statistics with true plays through other rips

**Question.** Session 21, follow-up to 0116 (rule in `docs/brief-21.md`,
written first): found only through another rip of each drawn track, do
true plays still separate from the related versions' by 0116's criterion?

**Method.** 0116's 36 mixes and renders, `shared-mixes` with
`leave_out_drawn` (`docs/panels/shared-mixes-s21-rips.json`): every
index also leaves out the mix's six drawn files (store
`b285da9984422fb5`, 33 min, 5.2 GB). A rip is a file of the drawn
track's cluster or one `pair` shows covering 80% of it (no new file
needed checking). `shared_mixes.py score --same --second-from`; data
`data/0118-shared-mixes-rips.json`.

**Plays, played recording left out.** 197 of 216 drawn tracks found
through another rip (277 plays); with the positive control's 15, 292
true plays. The 19 drawn tracks without an indexed rip gave no confident
play. Wrong related: the same 87 as 0116; wrong other: Kemal's remix.

| Candidate | Threshold | Wrong rejected (87) | Development mixes kept (97) | Rips kept (15) |
|---|--:|--:|--:|--:|
| (a1) hits per second | 8.49 | 78 | 57 | 13 |
| (a2) verifier share | 0.446 | 76 | 65 | 12 |
| (b) own material beyond | 0.0038 | 1 | 96 | |
| (c) strongest other holder | 380 hits | 39 | 86 | |
| (c)+(a1); (c)+(a2) | 13.2; 0.578 | 61; 61 | 74; 73 | 9; 9 |
| (c)+(b) | 0.0163 | 13 | 97 | |

- Each keeps 278-279 of 292; (a1), (a2), (c)+(a1), (c)+(a2) are again
  promising in rendered mixes only. (a1) medians: rips 17.4 (5.1-58.0; the
  densest likely the same encode in another folder), 0116's copies 31.5,
  development mixes 9.4, wrong related 4.6.
- 0116's frozen thresholds keep 255-265 of these 292 and reject the same
  61-81 of 87. (c) at 60 hits marks 51 of 292 true (32 through uploads
  of the same recording, the 15 rips, 4 through other versions), 61 wrong.

**Conclusion.** Another rip removes about two thirds of the density gap;
the rest is most likely the mixing chain: rendered rips stay above the
related versions, real mixes' true plays overlap them. No candidate keeps
95% of the development mixes' true plays and rejects half of the wrong.
