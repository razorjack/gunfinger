# 0114: the unused evidence of 0112 against true plays in the mixes

**Question.** Session 20, step 4's follow-up (rule in `docs/brief-20.md`,
written first): are true plays in real mixes denser than 0112's related
files, so that a threshold could reject 0111's wrong confident plays?

**Method.** `gunfinger-eval --other-peaks-dir <NAS store> --verify scan`
on the three development sets (store `b285da9984422fb5`; 2.0, 2.1 and
2.3 min): every confident detection credited to a referenced track (102,
40 tracks) against 0111's 106 wrong confident detections of the main
case. Per detection: hits per second of its span, and the verifier's
aligned share. `shared_true_plays.py`; data `data/0114-true-plays.json`.

| Per detection | True (102) | Wrong (106) | Cut keeping 95% of true | Wrong below |
|---|---|---|--:|--:|
| Hits per second of the span | 1.5-31.9 | 1.5-39.9 | 3.36 | 8 (8%) |
| Verifier's aligned share | 0.24-0.77 | 0.24-0.87 | 0.30 | 12 (11%) |

- Both fail proposal 5's review (keep 95%, reject at least half). The
  66 related-file plays alone: 8 and 12 below the cuts. The 40 plays of
  other versions (mostly Fire's remix and VIP, copies of each other) sit
  at 4.4-39.9 hits per second, among the dense true plays.
- With each track's strongest detection only (40), the cuts are 4.92 and
  0.357 and reject 40 and 51 of 106, but applied to every detection they
  keep only 89 and 90 of the 102 true ones.
- Medians: the Vertical broadcast 6.7 hits a second and 0.43, the 2003
  mix 11.9 and 0.55, Stakka's 10.3 and 0.52 (Anthrax: 4 minutes at 1.5).
- Scoring at this revision: 9 confident detections count as wrong, all
  NAS files outside session 18's clusters (33,723 records), each the
  title playing then or its edge: the-hifi-junglist's Drum&BassArena (React REACTCD201)
  disc 2 tracks 3, 4, 5, 7 and 8 in the 2003 mix (7 detections), Phoenix
  (weedsurfer) and Mutated (Version X) (U-PUNK-001 A) in the Vertical
  mix. Most likely the same recordings; left out of both sides above.

**Conclusions.** The separation in 0112 is relative to a clean copy of
the played record. Without one, a related file's evidence is as sparse
as a true play's in a real mix, and no cut on either measure meets the
review: development data do not support a sparseness statistic. Left:
where the line stops while the music runs on (0112), and a library map
of shared passages to report ambiguity (our decision, ADR 0017).
