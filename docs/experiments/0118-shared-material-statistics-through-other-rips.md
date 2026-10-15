# 0118: the same statistics with true plays through other rips

**Question.** Session 21, follow-up to 0116 (rule in `docs/brief-21.md`,
written first): found only through another rip of each drawn track, do
true plays still separate from the related versions' by 0116's criterion?

**Method.** 0116's 36 mixes and renders, `shared-mixes` with
`leave_out_drawn` (`docs/panels/shared-mixes-s21-rips.json`): every index
also leaves out the mix's six drawn files (`b285da9984422fb5`, 33 min,
5.2 GB). A rip: the drawn track's cluster, or `pair` covering 80% (no new
file needed checking). `shared_mixes.py score --same
work/s21/holder-coverage.json --second-from`, `data/0118-*`.

**Plays, played recording left out.** 163 of 216 drawn plays confident
through another rip (277 plays), 19 possible only, 34 with neither (no
indexed rip, or a weak one); with the positive control's 15, 292 true
plays. Wrong related: the same 87 as 0116; wrong other: Kemal's remix.

| Candidate | Threshold | Wrong rejected (87) | Development mixes kept (97) | Positive control kept (15) |
|---|--:|--:|--:|--:|
| (a1) hits per second | 8.49 | 78 | 57 | 13 |
| (a2) verifier share | 0.446 | 76 | 65 | 12 |
| (b) own material beyond | 0.0038 | 1 | 96 | |
| (c) strongest other holder | 380 hits | 39 | 86 | |
| (c)+(a1); (c)+(a2) | 13.2; 0.578 | 61; 61 | 74; 73 | 9; 9 |
| (c)+(b) | 0.0163 | 13 | 97 | |

- Each keeps 278-279 of 292; (a1), (a2), (c)+(a1), (c)+(a2) are again
  promising in rendered mixes only. (a1) medians: true plays here 17.4
  (5.1-58.0), 0116's copies 31.5, development mixes 9.4, wrong 4.6. 59 of
  the 277 lie within 1% of the drawn file's own density (likely the same
  encode in another folder); without them (a1) at 7.49 rejects 76, keeps 65.
- 0116's frozen thresholds keep 255-265 of 292 and reject the same 61-81.
  (c) at 60 hits marks 61 wrong and 51 of 292 true (32 through uploads, the
  15 positive-control plays, 4 through other files: `pair` 7% and 32%, 2
  not measured). Exploratory: (c)+(a1) at 6.276 (0116) keeps all 292.

**Conclusion.** Another rip removes about two thirds of the density gap;
the rest is most likely the mixing chain. No pre-registered threshold
keeps 95% of the development mixes' true plays and rejects half the wrong.
