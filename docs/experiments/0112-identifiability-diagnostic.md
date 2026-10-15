# 0112: proposal 5's identifiability diagnostic, development half

**Question.** Session 20, step 4: where 0111 makes R (the related file)
wrong confident, does the passage tell R from the played recording, and
does that survive into peaks and pairs? Rule in `docs/brief-20.md`.

**Change.** `gunfinger-eval identifiability` (4 tests) searches each
passage against R alone and each rip of the played recording alone:
*audio* (spectral correlation along the line), *peaks* (verifier share),
*pairs* (hits per passage second); chance (shifted alignments): audio
0.02-0.24, peaks 0.00-0.08. 2.4 min; data `data/0112-identifiability.json`.

| Direction | R: audio / peaks / pairs | Floor: the poorest rip | Outcome |
|---|---|---|---|
| dozer-1, 2 | 0.58 / 0.33-0.34 / 5.0-5.1 | own 0.88 / 0.79 / 24.7; pooled | 3 |
| fire-1 to 6 | 0.35-0.36 / 0.34-0.35 / 4.3-5.0 | own 0.72-0.75 / 0.61-0.64 / 13-14 (1, 3, 5); pooled | 3 |
| pyro-1 | 0.49 / 0.59 / 2.3 | own 0.66 / 0.38 / 6.0 | 3 (pairs) |
| pyro-2 | 0.72 / 0.60 / 13.2 | pooled 0.66 / 0.38 / 6.0 | 1 |
| sonar-1, 2 | 0.57-0.60 / 0.25-0.33 / 1.8-2.8 | own 0.82 / 0.61 / 11.1; pooled | 3 |
| watermelon-1, 2 | 0.09-0.16 / 0.24-0.25 / 0.6-1.2 | own 1.00; pooled | separated today |

- Outcome 3 in 11 of 14 directions: R's audio, and except in Pyro-1 its
  peak share, sit below the poorest rip; the rule still counts R's sparse
  hits over a long enough passage. Pyro-2 is outcome 1 only against a
  poor upload (0.66; the next rip 0.723, R 0.716). Controls: R 0-6 hits.
- Where the evidence falls (played left out): Fire's R line ends at the
  passage end (-0.1 to +3.7 s) while the played file runs 30 s on;
  weaker R lines (34-126 hits) match material before and after it. R
  runs 5-29 s past the passage for Dozer, Sonar-2 and Pyro-2, and covers
  parts of Pyro-1's and Sonar-1's long passages. In loops each repeat is
  a new line: 2-16 R detections, strongest 244-600, summed 451-5,399.
- Density under the same damage (weak rip, blend), hits per second of
  span: played file 25-53, rips 6.5-24, R 1.5-12.4; verifier share
  0.59-0.93, 0.35-0.82, 0.24-0.59. Pyro overlaps (R 8.5-12.4, rip
  6.5-17.7); Dozer's R (4.0-6.2) is close to the lowest rip (6.5).

**Conclusions.** Mostly outcome 3: the information survives into peaks,
and the confident rule ignores how sparse R's evidence is. That justifies
studying a statistic (no rule adopted), first on true plays in real mixes
(blends dilute density) and the sweeps' rips, then the validation half.
