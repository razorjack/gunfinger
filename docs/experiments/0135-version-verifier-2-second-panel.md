# 0135: version 2 of the version verifier on the second panel's development half (beside, deciding nothing)

**Question.** Session 26, step 5c: on the side set the pre-registration
names (`docs/brief-26.md`, step 3), what does version 2 do at 0133's T?
Version 1 rejected 4 of the 5 wrong plays and kept 77 of 78 positives
there (0132); with fewer than 10 wrong plays nothing can be read from it.

**Run.** Session 25's play list (`work/s25/plays/panel-2.json`, 83
plays), one run, 14 s, no errors; `evaluate` at T = 0.00683, `changes`
against 0132. `data/0135-*`.

| At T = 0.00683 | Plays | Version 2 rejected | Unmeasured | Version 1 (0132) |
|---|--:|--:|--:|--:|
| Wrong confident plays (Phantom 2018 over Phantom Force) | 5 | 0 | 4 | 4 rejected |
| Positive control | 78 | 0 | 0 | 1 rejected |

- The wrong plays: in each 10 s window Phantom 2018's densest run lies
  at another offset (+40 s to -51 s from the reported line), so every
  knot is a stray in the `beyond` queries and the 120 s loop: no line,
  nothing tested. The 240 s loop keeps one stretch, at a repetition 37 s
  from the reported line, with S1 0. Phantom 2018 repeats the shared
  material, and the densest run picks a different repetition per window
  (an inference; the knots' offsets are in the run's output).
- The positives: median offset 0.8 ms, 5 over 48 ms. CERT1853's Klute
  Remix rip in the 240 s loop, version 1's one false rejection (S1
  0.975, its line 313-327 ms off by 0132's check), is measured 324 ms
  off in two stretches and kept, S1 0.
- Version 1's band rules on version 2's lines would reject 7 positives
  at T (S1 0.008-0.035); the group rule keeps them.

**Conclusion.** Nothing decided here. The measured line repairs the one
false rejection, as on E1; on this family's wrong plays it finds no line
at all, because the related file repeats the passage. Choosing the
densest run per window independently splits repetitive material into
strays; a line that prefers continuity with the previous knot would
not, which is for a later design, not a change to this one.
