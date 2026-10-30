# 0252: a bounded iterated fit in the second pass, against the reference

**Question** (`docs/brief-63.md`; run because 0251 passed the bar for the
gain). From the chain's alignment, at most two rounds of the hit-weighted
drift of the measured windows (within ±0.4%, the last round kept): does it
keep the reference's gain on short chains, without the selection?
**Method.** `--second-pass-fit iterated` on 0251's sets (both scales, both
panels' development halves, `recall`, other rips with and without one extra
rung pair, the three scans); `data/0252-second-pass-bounded-fit.json`.

| Set (NAS scale unless 262) | Right confident | Weak true, median (p90) | Wrong, median (p90) | Own speed error, median |
|---|--:|--:|--:|--:|
| Sweep 2026, 262; NAS | 540; 540 = | 1.00 (1.01); 1.00 (1.01) | 0.98 (1.26); 0.98 (1.01) | 0.003% to 0.0015% |
| Other rips; +1 rung pair | 408 =; 407 = | 1.00 (1.01) | 0.97 (1.26) | |
| `recall` clean; other rips | 522 =; 174 +1 | 1.00 (1.01) | 1.00 (1.13-1.14) | 0.003% to 0.002% |
| `recall` brief | 721 +5 -1 | 1.02 (1.42) | 1.00 (1.15) | 0.049% to 0.004% |
| `recall` brief, other rips | 171 +13 -1 | 1.01 (1.24) | 1.00 (1.15) | |

- The same 5 brief plays as the grid and the split gain, and the same one
  (Offshore Drift, +3%) loses its third window; the other rips' loss is
  Known Unknown at +3% (242 to 238). Echos 503 (386) to 602 (455) at +1.00%.
- Siren Bass at -8%: 305 to 299 (L0); with one extra rung pair 214 to 230 at
  -8.09%, not confident (the grid's 241 was at -8.25%).
- No wrong confident answer added in any sweep, `recall` set or scan; the
  scans' tracks and their credited hits within 2 hits (Into Music 3,164 to
  3,165, where the grid and the split reach 3,504).
- Calibrate (weakest identifying; strongest false): 262 tracks 690; 100 to
  689; 107 (margin 6.90 to 6.44); NAS 704; 282 to 705; 282.
- Panels: panel 1 newly confident 2 (a partner, Decoy, in one blend, at both
  indexes), no longer 5 (Playground Shades looped 240 s, 245 to 238, at two
  indexes; Pyro - Ignorance 240 to 239 at three); panel 2 none either way
  (Phantom 2018, which the grid and the split made confident, stays below).
**Against the ADR bar.** B1 passes (no wrong or related file newly
confident); B2 fails (4 confident right answers lost: 2 brief plays, one
played file at two indexes); B3 passes (19 gained against the split's 18);
B4 passes at NAS scale, fails at 262 tracks (strongest false +7%, margin -6.7%).
**Conclusion.** The bounded fit keeps the split's gain on short chains with
none of the grid's lift of related versions and a smaller speed error;
but it loses answers near the rule, and at 262 tracks the Clockwork remix's
passage gains 7%. Not worth an ADR proposal as registered.
