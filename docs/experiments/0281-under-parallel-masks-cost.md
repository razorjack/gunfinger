# 0281: what `identify --under` costs with the masks on the workers

**Question.** With each confident entry's decode and mask on its own
worker (brief 79), is the option within twice the default's time at NAS
scale (J3, session 73)?

**Commands.** `work/s79/cost.sh`, the Linux machine otherwise idle, 32
workers, a warm-up per scale: each development mix in the order A B C C
B A (A default, B `--under` with `s79-run`, C `--under` with
`s79-start`, the option before this session), `/usr/bin/time -v`;
`work/s79/cost.py`. Means of two runs (within 3.3%); memory the larger.

**Result.** Wall time (times the default), the step's own time
(`--verbose`), CPU, peak memory (GiB); entries searched under.

| Mix, scale | A | B, after | C, before | Step B; C | CPU A; B; C | Memory A; B; C |
|---|--:|--:|--:|--:|--:|--:|
| Stakka, NAS (11) | 17.7 s | 33.1 s (1.87) | 50.4 s (2.85) | 15.4; 32.3 s | 371; 747; 756 s | 4.8; 7.2; 5.0 |
| razorjack, NAS (12) | 15.4 s | 28.4 s (1.85) | 45.1 s (2.93) | 13.0; 29.4 s | 303; 589; 593 s | 4.7; 6.7; 4.9 |
| Nocny, NAS (17) | 20.8 s | 41.9 s (2.01) | 61.2 s (2.94) | 21.1; 40.0 s | 475; 995; 1002 s | 4.7; 7.6; 4.9 |
| Stakka, 262 (11) | 5.9 s | 12.8 s (2.18) | 28.8 s (4.91) | 6.9; 23.0 s | 96; 225; 209 s | 0.18; 2.8; 0.52 |
| razorjack, 262 (7) | 4.3 s | 8.4 s (1.94) | 16.4 s (3.79) | 4.1; 12.2 s | 79; 142; 136 s | 0.17; 1.5; 0.43 |
| Nocny, 262 (9) | 6.0 s | 10.7 s (1.78) | 19.3 s (3.21) | 4.7; 13.0 s | 116; 190; 183 s | 0.22; 1.7; 0.49 |

- Added at NAS scale: 13-21 s per mix, 1.1-1.4 s per entry, 16-19 s per
  hour of mix (before: 30-40 s, 2.4-3.0 s, 35-39 s; 0275 measured
  2.79-2.92 times on the store before session 75's prune).
- Output: B and C give byte-identical JSON on all six mixes, and the
  Stakka mix at 262 tracks with `--jobs 1` gives the same bytes as with
  32 (1:45 wall, the step 60.3 s). The tracklists are those of 0273.
- CPU B = C at NAS scale (3-8% more at 262). Memory, B above A: NAS 2.0-2.8,
  262 1.3-2.7 GiB (above C: 1.8-2.7): a mask per worker, about 0.2 GiB.

**Conclusion.** The step takes half its former time, and the option
1.85-2.01 times the default's at NAS scale (about 16-19 s more per hour
of mix). J3 is met on Stakka and razorjack and fails on Nocny by 0.3 s
(41.86 s against twice 20.79 s): as registered, J3 is not met. The
price is about 2.4 GiB more peak memory; a cap on masks in flight would
bound it (not built). What remains of the step is the searches in entry
order and the slowest mask (an inference; no phase timings were taken).
