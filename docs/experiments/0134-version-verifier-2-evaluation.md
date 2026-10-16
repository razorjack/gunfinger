# 0134: version 2 of the version verifier on the development mixes and rendered mixes

**Question.** Session 26, step 5b, a declared second use of E1 and E2
(version 1: 5, 43; its E1 failures shaped version 2): at 0133's T?

**Pre-registration** (brief, step 3; T committed first): rejected at S1
>= T = 0.00683, unmeasured not; E1 at most 5 of 108, E2 at least 44 of 87.

**Run.** Session 24's play lists, one run each (52 s, 88 s), no errors;
one drawn play unmeasured. `changes` against session 24. `data/0134-*`.

| At T | Plays | Version 2 | Version 1 (T 0.121) | Required | |
|---|--:|--:|--:|--:|---|
| E1: true plays, development mixes | 108 | 9 | 5 | 5 at most | fails |
| E2: wrong related, rendered mixes | 87 | 45 | 43 | 44 at least | passes |
| E2: drawn true plays; positive control | 480; 15 | 18; 0 | 2 (0125: 3); 3 | (beside) | |

- Decisions changed: E1 8 newly rejected, 4 kept; E2 wrong 13 newly
  rejected (Pyro 7, Fire 5, Dozer 1), 11 kept (all Fire); drawn 18 and
  3. E2 by family: Fire 37 of 67 (43), Pyro 7 of 9 (0), Dozer 1 of 2
  (0), Sonar 0 of 9. One contradicted patch rejects a play with up to
  146 decided patches: 5 drawn plays are rejected on one patch.
- Lines: E1 median offset 7 ms, 13 over 48 ms; a stretch 2.8-129 s off
  in 4 E1 plays and 15 E2 wrong plays (repeated material, an inference).
  Now kept: Phantom Force (-145 ms, its opening window a stray), the
  alien5ive uploads (80-259 ms, 4-8 pieces), PRO 012 A's controls.
- Rejected, 9 plays of 4 tracks at blends or mix-ins, S1 0.007-0.062:
  Bulletproof - Anthrax (3 files, Vertical, 38:11-38:25), Future Cut -
  Horns 2000 (4 files, Vertical, its mix-in, 48:33-49:00), Kontempt.mp3
  (Knowledge, mix-in and out; its line follows a 170 ms nudge-in),
  Spirit - Out of Control (2003 mix, 4 patches). Why (an inference):
  another record fills the low group while bands of an anchored group
  read as contradicted (`r < 0.2`): the lowest mid bands (Anthrax), mid
  bands (Kontempt, Spirit), the lowest low bands (Horns 2000).
- Exploratory, deciding nothing: version 1's band rules on version 2's
  patches, T re-derived by the same rule (0.112): E1 0, E2 36 (Fire).

**Conclusion.** Not worth pursuing as built (E2 passes, E1 fails). The
line removes version 1's E1 failures; the group rule cuts T about
eighteenfold, and blends then reject true plays.
