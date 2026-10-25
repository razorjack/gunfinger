# 0218: the marked-play rules on the development mixes' saved reports

**Question.** Session 48, step 5: would the rules of 0216 print any of the five marked plays in the three development mixes' NAS-scale tracklists, all shared material by experiments 0102 and 0107 (0183's last paragraph)?
**Method.** Session 36's saved `identify` reports at NAS scale (`work/s36/identify/id-{nocny,razorjack,stakka}-s36-step1-32.json`; no mix searched), `gunfinger-eval --other-peaks-dir ... pair` on the five (marked file, partner file) pairs (stored peaks, 0:02), `marked_features.py known`. F4 needs the harness's verifier on generated mixes and is not measured here.

| Marked play (mix) | Seconds | F1 hits | F2 s | F3 hits (s) | R1 | R2 | R3 | R5 |
|---|--:|--:|--:|--:|--|--|--|--|
| Mampi Swift - 2000 in The Kraft - Stab 2 Def (nocny-trans-vertical) | 116 | 831 | 11.1 | 100 (25) | out | prints | prints | out |
| Digital & Spirit - Phantom 2018 in Phantom Force (razorjack) | 125 | 187 | 79.9 | 0 (0) | out | prints | out | out |
| Phantom Force (Fracture's Astrophonica Edit) in Phantom Force (razorjack) | 30 | 90 | 0.0 | 39 (11) | out | out | out | out |
| Kemal & Rob Data - Star Trails (Synergy Remix) in the Audio Blueprint upload (Stakka) | 31 | 117 | 0.0 | 10 (4) | out | out | out | out |
| Clockwork (Stakka Rmx) in Clockwork (Stakka) | 18 | 120 | 0.0 | 6 (1) | out | out | out | out |

- R1 leaves out all five: every pair of files shares at least 90 hits (bar 60), from four families, three of them outside the generated mixes' shared class (Clockwork only, 0216).
- R2 and R3 would print Mampi Swift's 2000 (it lies 11.1 s off the nearest alignment, with 100 hits outside the shared spans) and R2 also Phantom 2018 (80 s off). Both are shared material by 0102 and 0107, so R2 and R3 fail here as on the generated mixes.

**Criteria (brief 48).** R1 met the print bar with no shared play let in on the test seeds at both scales (0216) and lets none of the five in here: by the registered criterion it is a feature worth an ADR proposal. R5 also leaves all five out, which keeps its "promising, more data first" reading of 0216. The proposal is ours to make or not (brief 48 does not change the tracklist's rules).
**What it shows.** "The two files share no possible alignment" is a library-level test that leaves known shared material out in the real mixes too, where the families differ from the generated mixes'. **Does not show** real double drops of unrelated records (none with ground truth in the development mixes), nor related versions whose files share under 60 hits by the pair search (the Monotone remix and Watermelon: 13 hits, 0215-0216), which R1 would print.
