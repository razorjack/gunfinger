# 0134: version 2 of the version verifier on the development mixes and rendered mixes

**Question.** Session 26, step 5b: at 0133's T, does version 2 keep
E1's true plays and reject E2's wrong related plays? A declared second
use: version 1's counts (5, 43) were known and its E1 failures shaped it.

**Pre-registration** (`docs/brief-26.md`, step 3; T committed first).
Rejected when S1 >= T = 0.00683; unmeasured plays are not rejected.
Both required: E1 at most 5 of 108 rejected; E2 at least 44 of 87.

**Run.** Session 24's play lists, one run each (52 s, 88 s), no errors;
one drawn play unmeasured. `data/0134-*`.

| At T | Plays | Version 2 | Version 1 (T 0.121) | Required | |
|---|--:|--:|--:|--:|---|
| E1: true plays, development mixes | 108 | 9 | 5 | 5 at most | fails |
| E2: wrong related, rendered mixes | 87 | 45 | 43 | 44 at least | passes |
| E2: drawn true plays; positive control | 480; 15 | 18; 0 | 2; 3 | (beside) | |

- E2 by family: Fire 37 of 67 (version 1: 43), Pyro 7 of 9 (0), Dozer
  1 of 2 (0), Sonar 0 of 9 (0). E1's lines: median offset 7 ms, 13 over
  48 ms. Now kept: Phantom Force (-145 ms; its opening window a stray),
  the alien5ive uploads (80-259 ms, 4-8 pieces), PRO 012 A's controls.
- Rejected, 9 plays of 4 tracks, S1 0.007-0.062, all far under version
  1's T: Bulletproof - Anthrax (3 files, Vertical, 27-40 patches at
  38:11-38:25), Future Cut - Horns 2000 (4 files, Vertical, 6-11 at its
  mix-in, 48:33-48:46), Kontempt.mp3 (Knowledge, 15 at the mix-in and
  out; its line follows a 170 ms nudge-in), Spirit - Out of Control
  (2003 mix, 4 patches, S1 0.0072).
- Why (an inference from the timelines): another record covers the low
  group and reaches across the crossover into a group with an anchor
  (the lowest mid bands; for Horns 2000 the lowest low bands under an
  anchored band 7): `r < 0.2` contradicts them, and one patch in 150 rejects.
- Exploratory, after the counts, deciding nothing: version 1's band
  rules on version 2's lines, T by the same rule (0.112): E1 0 of 108,
  E2 36 of 87 (Fire only), drawn 2 of 480.

**Conclusion.** Not worth pursuing as built (E2 passes, E1 fails). The
line removes version 1's E1 failures; the group rule lowers every S1, T
falls twentyfold, and blends leaking across a crossover reject true plays.
