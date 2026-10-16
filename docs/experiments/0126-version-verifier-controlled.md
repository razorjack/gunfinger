# 0126: the report's controlled test of the version verifier

**Question.** Session 24, step 7 (plan in `docs/brief-24.md`, written
first): with a shared passage fixed, does the verifier respond to the
played version's own material after it when audible, attenuated (-12
dB) or cut away (another record)?

**Method.** `scripts/analysis/version_controlled.py` on `dozer-1` and
`fire-1`: the passage plus N = 0, 5, 10, 20 s of the played file's
material after it (20 queries, MP3); each identified against the
related file alone (negative) and the played recording's other rip
alone (positive) with `gunfinger identify --store-only --saved-index
off`; `versions` as built and, exploratory, with the longest line
extended over the query. `data/0126-*`.

| Related file (negative) | N | As built S1 | Extended S1 | Added patches S / C / U |
|---|--:|--:|--:|---|
| Dozer's bootleg, audible | 0; 10; 20 | 0.120 | 0.145; 0.173; 0.137 | -; 24/5/2; 65/4/2 |
| the same, cut away | 10; 20 | 0.120 | 0.192; 0.338 | 0/9/22; 0/49/22 |
| Fire's remix, audible | 0; 10; 20 | 0.078 | 0.078; 0.077; 0.130 | -; 19/3/9; 26/26/18 |
| the same, cut away | 10; 20 | 0.078 | 0.080; 0.080 | 0/0/31; 0/0/70 |

- Every query found both candidates confident (related 249-511 hits).
- As built, S1 never changes with N: the added material lies outside
  the related file's line, so the verifier cannot see it.
- Attenuated equals audible in every count: the gain fit absorbs -12 dB.
- Extended, Fire's remix is contradicted only at 20 s (26 of 70 added
  patches); 5-10 s after the passage are still mostly shared. Dozer's
  bootleg stays supported over 20 s: the material after the panel's
  passage is shared too.
- The positive (the rip): every added patch supported when audible or
  attenuated (71, 70 at 20 s); cut away, Fire's unobservable (69 of 70)
  as expected, but Dozer's contradicted (42 of 71, S1 0.185): Decoy's
  drums line up with Dozer's in some bands, so patches get anchors and
  the rest of the band set reads as missing.

**Conclusion.** Within its line the verifier sees only differences under
the shared material; beyond it, a cut to a record whose drums line up
reads as contradiction for the true rip too, so extending the line (as
(b) did in 0116) would reject true plays at transitions.
