# 0227: wrong answers and calibrate per ladder, at 262 tracks and NAS scale

**Question** (`docs/brief-51.md`, 07:43, C3, W). What do 1-5 extra rungs
change in the standard protocol? **Method.** `regress session-32-linux`
and `regress session-33-linux-nas` with `--extra-rungs k` (k = 0, 1, 2,
3, 5) in scratch work directories (the sweep of seed 2026, the Stakka
scan, leave-outs 3 and 11), then `calibrate` per ladder; queue 3b,
08:53-09:18, exit 0; `data/0227-standard-protocol-per-ladder.txt`.

| Ladder | Sweep queries changed (262; NAS) | Scan detections (NAS) | False candidates (262; NAS) | ≥ 30 hits (NAS) | Confident false (NAS) |
|---|--:|--:|--:|--:|--:|
| L0 | 0; 0 | 4,253 | 406; 31,500 | 95 | 7 |
| L1 | 21; 298 | +18 | 423; 32,462 | 97 | 7 |
| L2 | 26; 354 | +57 | 429; 33,135 | 98 | 8 |
| L3 | 31; 395 | +82 | 437; 33,822 | 98 | 8 |
| L5 | 39; 447 | +114 | 451; 34,811 | 98 | 8 |

At every ladder and both scales: the development scan 11/11, leave-outs
8/11 and 0/11, no wrong or possible track, the same unmatched possible
plays (1; 3), and at 262 tracks every scan detection identical; at NAS
scale the added detections are below the possible tier's plays (2
changed, 1-4 removed). Calibrate is the same under every ladder: 262
tracks weakest identifying 690, strongest false 100 (the Stakka remix of
Clockwork), margin 6.90×, audio not in the index 19; NAS scale 704, 282,
2.50×, the strongest on held-out audio 282. Every sweep query stays
identified (559 identifying, none below the rule).
**The new wrong answer.** The default's 7 confident false answers at NAS
scale are one held-out excerpt (Aphrodite - Cool Rock Stuff) against a
YouTube bootleg that holds it ("Mobb Deep Vs Urban Takover - Cool Rock
Shook Ones"), at -8% to +3%: the bootleg runs about 4.3% slow, so the
excerpt meets it about 4.5% above its played speed. At +5% that is
+9.7%: L2, L3 and L5 reach it (268-278 hits), L0 and L1 do not; T_k
takes L_k's answer there (L0 has nothing confident). By ear the bootleg
is a different mix of the same track (`docs/pair-verdicts.txt`).
**Against the registration.** W fails for L2, L3, L5 and T2, T3, T5
(the same pair at one more speed); it holds for L1, and for T1 by
inference from L1's runs (T_k's scans were not run).
**Conclusion.** Wider ladders add false candidates below the rule
(+3-11%) and reach one more speed of a pair judged different; calibrate
and the development scans' plays do not move.
