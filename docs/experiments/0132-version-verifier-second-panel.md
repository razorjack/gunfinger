# 0132: the version verifier as built on the second panel's development half

**Question.** Session 25, step 4c (pre-registered in `docs/brief-25.md`
at 21:17, before 0129's run): on families it has never seen, does
session 24's verifier reject the wrong confident plays at its frozen T =
0.1210 and keep the positive control?

**Method.** 0129's run; plays as 0124 formed them (`version_plays.py
calibration ... panel-2`, 0128's clusters); siblings from `holders`
(133 lookups). `versions` with this session's build (its code as session
24's; the renderer of step 3c) and, beside it, `work/bin/s24-final`.
1.5 min in all; `data/0132-*.json`.

| At T = 0.1210 | Plays | Rejected | Pre-registered reading |
|---|--:|--:|---|
| Negatives (wrong confident, played left out) | 5 | 4 | too small to read (under 10) |
| Positives (the played recording's rip, source left out) | 78 | 1 | 98.7% kept |

- The 5 negatives are one related file in one direction: Phantom 2018
  over Phantom Force (S1 0.149 in the three `beyond` queries, 0.172 and
  0.106 in the two loops). Star Trails and We R The Ones give none.
- Their contradicted bands sit in the low and mid groups (7% and 9% of
  informative band-patches, the high group 1%); the positives' 0.3-2.3%.
  Every negative patch is distinctive (no sibling holds Phantom 2018's
  span), so S2 equals S1; S3 26-106 s.
- The rejected positive, CERT1853's Klute Remix rip in `we-r-the-ones-2
  looped 240 s` (S1 0.975; 118 contradicted, 385 unobservable patches,
  the lag pinned at +48 ms): 0127's check, exploratory, puts its audio
  313-327 ms from the reported line at all 6 excerpts; the same file in
  `looped 120 s` is within 2 ms and kept. Its two segments open 9 s
  before a loop point, so the opening window spans a jump (an inference).
- The two builds: 9 plays' S1 differ by at most 0.006 (the decoders'
  start after a seek); no decision changes.

**Conclusion.** Too small to read, by the rule written first: the new
families yield 5 wrong plays from one direction. On them the verifier
rejects 4, and it keeps 77 of 78 positives; its one false rejection is
again a reported line about 320 ms off, not a version difference.
