# 0293: key lock over the gaps only, against both ladders over the whole query

**Question** (brief 84, registered in `0d2c4da`). Do the key-locked rungs, searched only where the
turntable ladder finds nothing confident, find what both ladders find, at the turntable's cost?
**Method.** G: `--ladder turntable --gap-key-lock` (`bd7d2a4`), 0292's stretches searched on the
41 key-locked rungs; B: `--ladder both` (the default); K: the turntable ladder alone. K1: `mixes
--playing key-locked --count 24` (seed 2026; every second play key-locked at its drawn tempo,
±6%; 109 key-locked and 116 turntable plays of indexed tracks); C3: `regress` with G; C5: `robust`
at 262 tracks (B's is 0292's L0); K2: the development scans at NAS scale alone, B G K K G B, 32
workers. Every run exit 0. Data: `data/0293-key-lock-over-the-gaps.json`; `gap_search_*.py`.

| | B | K | G |
|---|--:|--:|--:|
| K1, 262: key-locked plays confident of 109 (of them within 1%: of 15) | 99 (13) | 25 (13) | 97 (12) |
| K1, NAS | 99 (13) | 25 (13) | 94 (12) |
| K1: turntable plays confident of 116 (262; NAS) | 109; 108 | 109; 108 | 109; 108 |
| K1: wrong confident (NAS; none at 262); wrong possible (NAS) | 4; 9 | 2; 8 | 4; 11 |
| C5 robust (2,600): confident; key-lock conditions of 200; wrong | 1,938; 200; 0 | 1,741; 3; 0 | 1,938; 200; 0 |
| K2 CPU: Stakka 688.3 s, 2003 555.2, Nocny 882.0 (× B) | ×1 | ×0.563, 0.563, 0.556 | ×0.562, 0.564, 0.581 |
| K2 search time (× B) | ×1 | ×0.608, 0.598, 0.582 | ×0.603, 0.589, 0.619 |

- K1's wrong answers are all The Nine's Evol Intent VIP (shared material, 0291), the same under
  B and G (two under K): none is new against B. K2's ranges lie within 0.3%. B costs 1.78-1.80
  times K; G searches T1's stretches (6.1% of Nocny at NAS scale), so it costs about K's CPU.
- G loses 7 of B's key-locked plays at NAS scale and gains 2 (262: 5 and 3). 6 of the 7 are the
  returning play (key-locked here): 3 whose time a confident play of the same file covers (as in
  0292), 1 found more weakly over a partly covered stretch, 1 cut short by a neighbour (295 hits
  in 2 windows against 299 in 3), and 1 where the join drops K's confident detection (317 hits, 3
  windows) for a stronger possible one (426, 2 windows, 0.4% apart; the suppression orders by
  hits alone); the 7th is a short play (472 in 2 windows against 474 in 3).
- C3: no confident sweep answer changes at either scale (NAS: 610 of 720 queries change below the
  confident tier); the scans and leave-outs keep their confident plays; calibrate as B. Their
  detections of 60 hits or more equal K's: the one-hit moves come from the turntable ladder.
**Against the registration.** K-G1 holds (97 and 94 against 99: 1.8 and 4.6 points, 5 allowed;
every turntable play kept); K-W holds (K1, C3, C5); K-C holds (×0.56-0.58, at most 0.8 allowed).
G passes (evidence for decisions queue item 9). **Conclusion.** Key lock searched only over the
turntable ladder's gaps keeps both ladders' answers on the protocol, robust and key-locked mixes
within the registered margin, for 56-58% of their CPU on the development scans. It misses plays
whose time a confident play covers; the margin rests on one seed's generated mixes (4.6 points
of 5 at NAS scale), and no real key-locked mix was searched.
