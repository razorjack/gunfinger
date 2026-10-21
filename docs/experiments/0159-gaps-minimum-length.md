# 0159: gaps without a confident segment, and their minimum length

**Question.** Session 34, step 1 of the plan: which minimum length lists missed
tracks' time and leaves blends out, by C1-C3 of `docs/research/2026-10-21-gap-measures.md`?

**Method.** Build `work/bin/s34-runs/`, `--jobs 16`. Each development mix plain
and in 14 `scan --leave-out-slots` runs leaving every run of 1-4 slots with kept
neighbours out once (`leave_out_design.py`), at NAS scale and 262 tracks;
`mixes --count 16`, seeds 2026-2029 and 7 (80 mixes, 262 tracks). `gaps` on
each report, `gap_scores.py` counts. Data `data/0159-gap-scores.json`.

| Min. | NAS: reached; seconds | NAS no missed: plain; leave-outs | 262 leave-outs: no missed | Generated: reached; no missed |
|--:|---|---|---|---|
| 10 s | 271/271; 95.1% | 3; 29/161 (18%) | 25/146 | 213/215; 24/197 (12%) |
| 20 s | 271/271; 95.1% | 2; 25/157 (16%) | 23/144 | 196/215; 3/159 (2%) |
| 30 s | 271/271; 95.1% | 1; 11/143 (7.7%) | 9/130 | 172/215; 2/134 (1.5%) |
| 45 s | 271/271; 94.8% | 1; 3/131 (2.3%) | 3/124 | 99/215; 0/63 |
| 60 s | 271/271; 94.8% | 0; 0/128 | 0/121 | 58/215; 0/26 |
| 90 s | 270/271; 94.7% | 0; 0/127 | 0/121 | 13/215; 0/4 |

- C1 (NAS reach of 95%) holds everywhere, C2 (plain at most 3, leave-outs 10%)
  from 30 s, C3 (generated 10%) from 20 s: **the smallest length meeting all
  three is 30 s.** At 262 tracks all 184 missed tracks are reached at every
  length (97.5% of their seconds). No scan has a wrong identification; every
  kept track is found in every leave-out. The 5% of missed seconds outside
  gaps is manifest time a neighbour's labels cover (approximate starts, 0139).
- A gap with no missed track at NAS scale is time inside a found track that no
  segment covers, with no transition second and no possible play: at 30 s,
  three distinct ones of 34-59 s (Star Trails' end 13:24-14:23; 18:02-18:36 in
  Kinetic; Nocny's 38:48-39:31 after Blindside's tease); at 10-20 s, also 15 s
  in my 2003 mix (26:03) and Nocny's last 24 s. None lasts 60 s.
- Generated (plays of 20-60 s), at 30 s: held-out plays of 40 s or more are
  reached (113/114), shorter ones half the time (59/101), as crossfades of up
  to half the play leave under 30 s. Both gaps without a missed track are at
  a mix's start, where the first plays were found only as possible.

**Conclusion.** By the registered criteria the minimum length is 30 s: every
missed DJ-length track is reached; 1 gap in 13 of the leave-out scans holds none,
always inside a found track. 60 s lists none of those and still reaches every
missed track here; brief plays need less. The product's choice is the ADR's.
