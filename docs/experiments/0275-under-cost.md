# 0275: what `identify --under` costs, and budgets

**Question.** What does the step add to time and memory, where does the
time go, and would a budget do (brief 73; J3: at most twice the default)?

**Commands.** `work/s73/cost.sh`: each development mix, default (a) and
`--under` (b), a b b a after a warm-up, the Linux machine otherwise idle
(32 workers), `/usr/bin/time -v`; `under_score.py cost`. The timing
variant (`work/bin/s73-timing/`, never merged; JSON equal to the
product's on all six development runs): `under_score.py fates`, `budget`.

**Result.** Means of two runs each; both runs of a kind gave the same
JSON, and the plays of a and b are identical.

| Mix, scale | Default; `--under` | Times | Added per entry; per hour of mix | CPU | Peak memory |
|---|--:|--:|--:|--:|--:|
| Stakka, 262 | 5.8; 29.2 s | 5.05 | 2.1 s; 25 s | 96; 212 s | 0.19; 0.53 GB |
| razorjack, 262 | 4.3; 16.6 s | 3.83 | 1.8 s; 16 s | 79; 138 s | 0.17; 0.44 GB |
| Nocny, 262 | 6.0; 19.1 s | 3.19 | 1.5 s; 12 s | 115; 183 s | 0.22; 0.49 GB |
| Stakka, NAS | 17.8; 49.7 s | 2.79 | 2.9 s; 34 s | 372; 737 s | 4.8; 5.0 GB |
| razorjack, NAS | 15.5; 45.0 s | 2.90 | 2.5 s; 38 s | 301; 577 s | 4.7; 4.9 GB |
| Nocny, NAS | 20.9; 61.1 s | 2.92 | 2.4 s; 36 s | 472; 969 s | 4.7; 4.9 GB |

- Entries searched under: 11, 7, 9 at 262 tracks; 11, 12, 17 at NAS
  scale (per confident play at NAS scale: 1.0-1.1 s).
- Where the time goes (variant): the mask 40-70% (one thread: STFTs and
  the lag measure's eight phases), the residual's search 14-42% (more at
  NAS scale), the decode and render 12-19%, the shared-material check
  under 1%. Generated drops at NAS scale (438 entries): mask 52%, decode
  23%, search 24%; `identify` of 16 mixes 2:29 becomes 6:44 (seed 2050).
- Budgets on the drops mixes (38 right entries gained, step 488 s): span
  of at least 60 s keeps 38 for 98% of the time; 120 s keeps 33 for 82%;
  180 s keeps 4 for 32%; the 8 longest entries per mix keep 22 for 68%.
  No length or cap reaches J3 (the step must lose about half its time)
  while keeping 90% of the finds.

**Conclusion.** J3 fails: at NAS scale the option takes 2.8-2.9 times
the default's time (about 35 s more per hour of mix), with 0.2 GB more
memory, and no budget meets J3. The mask, on one thread, is the largest
part (an inference that a faster step is the way, not measured).
