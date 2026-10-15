# 0112: proposal 5's identifiability diagnostic, development half

**Question.** Session 20, step 4: where 0111 makes the related recording
wrong confident, does the passage tell it from the played recording, and
does that survive into peaks and pairs? Rule in `docs/brief-20.md`.

**Change.** `gunfinger-eval identifiability` (6 tests) searches each
passage against each file of the related recording (R, the plan's, first)
and each rip of the played recording, alone: *audio* (spectral
correlation along the line, silence left out), *peaks* (verifier share),
*pairs* (hits per passage second); chance 0.02-0.24 and 0.00-0.10, none
for pairs (the rule asked). 3.0 min; `data/0112-identifiability.json`.

| Direction | R: audio / peaks / pairs | Floor: the poorest rip | Outcome |
|---|---|---|---|
| dozer-1, 2 | 0.58 / 0.33-0.34 / 5.0-5.1 | own 0.88 / 0.79 / 24.7; pooled | 3 |
| fire-1 to 6 | 0.35-0.36 / 0.34-0.35 / 4.3-5.0 | own 0.72-0.75 / 0.61-0.64 / 13-14 (1, 3, 5); pooled | 3 |
| pyro-1; pyro-2 | 0.49; 0.72 / 0.59; 0.60 / 2.3; 13.2 | own 0.66 / 0.38 / 6.0; pooled, the same | 3 (pairs only); 1 |
| sonar-1; sonar-2 | 0.57; 0.60 / 0.25; 0.33 / 1.8; 2.8 | own 0.82 / 0.61 / 11.1; pooled | 3; separated today |
| watermelon-1, 2 | 0.09-0.16 / 0.24-0.25 / 0.6-1.2 | own 1.00; pooled | separated today |

- Outcome 3 in 10 of 14 directions, clearly in the five with rips of
  their own. The pooled floor is one poor upload of Time Is Broken:
  dozer-2's and fire-2, 4, 6's peaks sit 0.025-0.05 below it. Sonar-2's
  R is never confident; the PRO 012 A upload that is (334 hits) is
  outcome 3 by pairs only (0.63 / 0.39 / 3.75). Pyro-1's R aligns in
  blocks: per aligned second 10.8 hits against the rip's 9.8, outcome 2.
  Pyro-2 holds only against that upload (next rip 0.723, R 0.716).
- Where it falls (played left out): Fire's R line ends within 4 s of the
  passage's end while the played file runs 30 s on; R runs 5-29 s past
  it for Dozer, Sonar-2 and Pyro-2. In loops each repeat is a new line:
  2-16 R detections, strongest 244-600, summed 451-5,399.
- Density under damage (weak rip, blend; hits per second of the span,
  verifier share; controls out): played file 25-53 and 0.64-0.93, rips
  6.5-24 and 0.35-0.82, R 1.5-12.4 and 0.24-0.59. Pyro overlaps (R
  3.3-12.4, rip 6.5-17.7); Dozer's R (4.0-6.2) nears the lowest rip.

**Conclusions.** Outcome 3 dominates where the played recording has rips
of its own: the information survives into peaks, and the rule ignores
how sparse R's evidence is. No rule adopted; 0114 and 0115 test that.
