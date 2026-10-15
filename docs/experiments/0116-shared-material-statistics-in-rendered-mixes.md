# 0116: candidate statistics for shared material in rendered mixes

**Question.** Session 21 (rule in `docs/brief-21.md`, written first): in
mixes playing a development played file whole, does one threshold keep
95% of true confident plays and reject half of the related ones?

**Method.** `gunfinger-eval --other-peaks-dir <NAS store> shared-mixes
docs/panels/shared-mixes-s21.json` (`b285da9984422fb5`, 36 min, 7.0 GB):
36 mixes, 6.4-11.3 min (seeds 2101-2103 x 12 played files, whole, at
0.943-1.053, between 3 + 3 drawn corpus plays); everything indexed, the
played recording left out, the played file alone left out (4 files); the
development mixes as they are. `shared_mixes.py score`, `data/0116-*`.

**Played recording left out.** All 216 drawn plays right (438 plays,
rips apart); wrong related 87 in 9 of 12 played files (Fire 67, Pyro 9,
Sonar's revision 9, Dozer 2), 240-4,480 hits, the same 87 with
everything indexed; the positive control's rip right in 15 plays. Of 43
other confident plays, 42 are newer uploads of drawn tracks (`pair`
86-99.9%, set aside), one Kemal's Silent Extent Remix over the original.

| Candidate | Threshold | Wrong rejected (87) | Development mixes kept (97) |
|---|--:|--:|--:|
| (a1) hits per second; (a2) verifier share | 9.31; 0.498 | 81; 80 | 50; 45 |
| (b) own material beyond (reversed: true plays end at transitions) | 0.0033 | 1 | 96 |
| (c) strongest other holder | 421 hits | 39 | 86 |
| (c)+(a1); (c)+(a2); (c)+(b) | 24.6; 0.722; 0.0096 | 61; 61; 8 | 70; 70; 97 |

- Each threshold keeps 431-432 of 453 true plays. (a1), (a2), (c)+(a1)
  and (c)+(a2) are promising in rendered mixes only: drawn plays copy the
  indexed files ((a1) median 31.5); real plays are sparser (rips 6.6-20.1,
  development mixes median 9.4; wrong 4.6); the rips keep 10, 11, 0, 2 of 15.
- (c) at 60 hits marks 61 of 87 wrong (all Fire: no third version of
  Dozer, Pyro or Sonar is indexed), 79 of 453 true (62 through newer
  uploads) and 28 of 97 development plays (11). Exploratory: cuts
  keeping 95% of the 112 real plays reject 43 (a1) and 12 (a2) of 87.

**Conclusion.** The test passes only against plays copied from the
indexed files; at its thresholds the development mixes keep 46-72% of
true plays, the rips 0-11 of 15: nothing separates a related version
from a real rip. (c) marks shared passages: ambiguity to report.
