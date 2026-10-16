# 0125: the version verifier on the development mixes and rendered mixes

**Question.** Session 24, step 6: at 0124's T, does the verifier keep the
true plays of the three real development mixes and reject the related
versions' wrong plays in the rendered mixes?

**Pre-registration** (`docs/brief-24.md`, step 3; T committed before
these runs). A play is rejected when S1 >= T = 0.1210. Both required:
E1, at most 5 of the 108 true plays of the development mixes rejected
(session 23's step 4c plays, 0120's clusters); E2, at least 44 of the 87
wrong related plays of session 21's rendered mixes (played left out).

**Run.** `versions` (E1 96 s, E2 134 s), `evaluate`. A first run failed
to render 8 references (WAV behind an ID3 tag); fixed in
`render_samples`, all rerun, calibration identical (brief). `data/0125-*`.

| At T = 0.121 | Plays | Rejected | Required | |
|---|--:|--:|--:|---|
| E1: true plays, three development mixes | 108 | 5 | 5 at most | passes, at the limit |
| E2: wrong related, rendered mixes | 87 | 43 | 44 at least | fails by one |
| E2: drawn true plays; positive control | 480; 15 | 3; 3 | (beside) | |

- E2 by family: Fire 43 of 67, Dozer 0 of 2, Pyro 0 of 9, Sonar 0 of 9,
  as in 0124. The 44th wrong play has S1 0.120, just under T.
- Rejected E1 plays (S1 1.0, 0.59, 0.49, 0.48, 0.26; next 0.089): Phantom
  Force (2003 mix); both Kontempt rips, Auto Ignition, Pathogen (Stakka
  & Skynet). 36-98% of their patches have no anchor and about half have
  the lag pinned at +-48 ms (kept true plays: median 0): their lines lie
  98-193 ms off (0127, exploratory). No bass kill, blend or cut stands
  out (an inference from the timelines).
- Positive control: the 3 rejections are the PRO 012 A Sonar upload
  against the corpus original (S1 0.49-0.57, 40% pinned), the pair of
  open listening questions 4-5. Drawn: two Astro-Sine rips (S1 0.12-0.16)
  and Do Drop In (every lag pinned).

**Conclusion.** Not worth pursuing as built, by the rule written first:
E1 holds (5 of 108), E2 misses by one (43 of 87); each margin is one
play. In E2 it rejects only Fire's wrong plays (low bands), none of
Dozer's, Pyro's or Sonar's; its true-play rejections coincide with lines
98-193 ms off (0127), not with DJ processing.
