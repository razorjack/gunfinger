# 0208: pairing the query with more targets (fan-outs 3, 5, 8)

**Method.** 0206's fan-out step, opened by 0207, on the same runs and
sets: G(F), the exact pairs the query's targets 3 to F add to E (record
and index at fan-out 2, each reference pair once); chance G(F) median 0
everywhere. `lost_pairs.py --fan-outs`; `data/0208-query-fan-outs.json`.

| Median G(F)/E | F 3 | F 5 | F 8 | Median E |
|--|--:|--:|--:|--:|
| T(C) clean | 0.11 | 0.14 | 0.14 | 1,019 |
| T(c) damaged | 0.18 | 0.30 | 0.32 | 96 |
| T(a) other rips | 0.14 | 0.19 | 0.19 | 458 |
| P played, shared | 0.12 | 0.16 | 0.16 | 783 |
| R related, shared | 0.16 | 0.23 | 0.24 | 120 |
| R(r) related wrong | 0.14 | 0.22 | 0.24 | 49 |
| F unrelated | 0.00 | 0.09 | 0.10 | 9 |

- By E band (F 5; P, R): 10-29 0.04, 0.15; 30-59 0.11, 0.21; 60-119
  0.17, 0.22; 120-239 0.19, 0.24; 240+ 0.16, 0.25: R gains more than P
  in every band at every F; pooled AUC of G/E, P~R 0.27, 0.24, 0.25 (F 3,
  5, 8), T(c)~R(r) 0.70, 0.82, 0.82, T(c)~F 0.79, 0.86, 0.86.
- Under 240 hits reaching 240 with hits + G(F) over 3 windows (F 3, 5,
  8): T(c) 12, 21, 23 of 507; T(a) 3 of 3 (The Sect's Victim, all F); R
  233, 384, 401 of 4,428 (the Fire Rogue Unit VIP, Phantom 2018, the
  Dozer bootleg among them); F and R(r) none; at 262 tracks T(0) 2, 6, 7
  of 76, W(0) none. Per shared search, played left out, a related file
  confident: 114 of 419 today, 124, 129, 129.
- CPU (estimate, not timed): the query's pairs, and so the second
  pass's lookups, grow 1.49, 2.44 and 3.54-3.66 times; the second pass
  is at most 3-4% of a scan's CPU (0205): at most about 2%, 5%, 10% more.
  F 8 adds little over 5 (0207: 91% of displaced pairs at ranks 3-5).

**Against the criterion.** (a) T(c)'s median G/E reaches 0.25 at F 5
(0.30), but 4.1% of T(c) under 240 reach 240, not 10%: not met at any F.
(b) not met: R gains more than P in every band, AUC 0.24-0.27, not
>= 0.75. (c) met: no unrelated candidate reaches 240. Step 2's
prediction holds and more: at equal E related versions gain more than the
played recording. **Conclusion.** Not worth an experiment on the rules: a
wider query fan-out recovers pairs for shared material at least as
readily as for the played recording.
