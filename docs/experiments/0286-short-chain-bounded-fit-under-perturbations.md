# 0286: the bounded fit for short chains only, under the same perturbations

**Question** (`docs/brief-81.md`, registered). With 0252's bounded fit applied only where `chain_speed`
gives the mean of the rungs (fewer than 3 lines, or under 16 s between their centres;
`--second-pass-fit iterated-short`), do short plays move less under 0285's perturbations, at no cost?
**Method.** 0285's runs repeated with the variant (`work/bin/s81-run3`; its bases and `regress
session-78-linux` with `s81-run`, the same search code), each against the variant's own base, by
`scripts/analysis/perturbation.py`; at the bases, the variant against the default
(`perturbation.py fits`). `data/0286-short-chain-bounded-fit-under-perturbations.json`.

| Grid, 10-20 s plays (582 strongest matching detections) | Default | Short-chain fit |
|---|--:|--:|
| added: speed moved > 1e-4 pp; hits moved; by 5% or more; largest speed move | 131; 98; 23; 0.35 pp | 45; 40; 16; 0.23 pp |
| removed | 97; 54; 14; 0.40 pp | 31; 29; 11; 0.015 pp |
| cut-down; cut-up | 53; 9; 2 / 0; 0; 0 | 1; 1; 0 / 0; 0; 0 |
| Confident identity changes, all perturbations (grid, sweep, both scales) | 0 | 0 |

- The 25-40 s plays and the sweep (30 s) move as under the default (within 3 detections per
  perturbation); at NAS scale the two fits' moves are identical (47 and 83 speeds, 5 and 15 hit counts).
- The moves left are hit counts at an unchanged speed (Clockwork's 20 s plays 930 to 872 hits at
  -1.39%: by inference the chain's span or pivot moved), and a 10 s play under the fit's floor of 80
  hits: Swarm's 244 hits at +5.25% become 25 at +5.48%, where the fit does not run.
- At the bases: on the grid's 10-20 s plays the variant's hits are 1.22 times the default's (median;
  10th-90th percentile 1.00-1.46), its speed error 0.006 pp against 0.095 (median); 25-40 s plays
  unchanged. One play becomes confident (Sinthetix - Neurotoxin, 20 s: 588 hits in 2 windows to 739 in
  3); no wrong confident detection is added in the sweep, the grid or the scans.
- `regress session-78-linux` with the variant: the sweep 710 of 720 queries identical, every scan's
  answers the same; calibrate's strongest false 100 to 107 (margin 6.90 to 6.45): the Clockwork (Stakka
  remix) passage in the Stakka scan at 20:23, 94 to 107 hits at +1.20% to +1.274% (0252's B4 case).

**Against the bar.** A1 fails: neither fit changes a confident answer under any perturbation, so the
variant cannot have fewer. A2 passes (27 detections moved by 5% or more against 39). A3 fails through
calibrate's strongest false (107 against 100). A4 was not run.
**Conclusion.** For short chains the bounded fit removes most of the speed's dependence on the library
(a third as many speeds move under `added` and `removed`, 1 of 53 under `cut-down`; the largest move
0.23 and 0.015 pp against 0.35 and 0.40) and raises their hits by about a fifth. But no confident
answer depended on that dependence at 262 tracks, and the fit lifts the shared passage that sets
calibrate's margin, as in 0252. Not a candidate for an ADR proposal as registered.
