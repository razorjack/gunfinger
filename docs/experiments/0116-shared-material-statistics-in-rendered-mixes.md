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

**Played recording left out.** 203 of 216 drawn plays confident (13 of
20-28 s possible only; 438 plays with rips); 87 wrong related in 9 of 12
played files (Fire 67, Pyro 9, Sonar's revision 9, Dozer 2; 240-4,480 hits;
the same with everything indexed); the positive control right in 15 plays.
43 other confident plays: 42 newer uploads (`pair` 86-99.9%), Kemal's remix.

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
- (c) at 60 hits marks 61 of 87 wrong (all Fire: no third version of Dozer,
  Pyro or Sonar is indexed), 79 of 453 true (62 through newer uploads), 28
  of 97 development plays (11 through uploads). Exploratory, cuts keeping
  95% of the 112 real plays: (a1) 4.55, (a2) 0.321, (c)+(a2) 0.350 reject
  43, 12, 30; (c)+(a1) 6.338 rejects the 61 (densest 6.01), keeps 92 of 97.

**Conclusion.** The rule passes in rendered mixes (on copies here, through
other rips in 0118) and fails against real true plays: the development
mixes keep 46-72%, the positive control 0-11 of 15. The exploratory
(c)+(a1) cut is frozen in the brief for a test on other real mixes.
