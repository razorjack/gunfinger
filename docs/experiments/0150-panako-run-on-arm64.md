# 0150: Panako at 262 tracks on arm64, beside Gunfinger

**Question.** Session 24b, 2026-10-17: on 0130's queries (amendments A1-A11), how do
Panako 2.1's answers (`f1248f7`, strategy `PANAKO`) and costs compare with Gunfinger's (0131)?

**Method** (`docs/brief-24b.md`, "The run of 2026-10-17"; `work/panako/run.sh`;
`panako_compare.py`). Temurin 11.0.32.1; lmdbjava has no arm64 LMDB, so LMDB
0.9.29 was built from OpenLDAP's tag (commit `8ad7be25`, the same on the GitHub
mirror), `libraries/liblmdb` only, passed as `-Dlmdbjava.native.lib`. Runs alone,
`nice -n 10`, load 3-84. S* = 13: 1 + the highest wrong score on seed 2026's sweep.

| Panel | Panako, defaults | Panako, S* | Gunfinger |
|---|---|---|---|
| Sweeps 2026-2029, recalled of 540 indexed | 339, 309, 291, 341 | same | 540 each |
| wrong answers | 1, 3, 2, 7 | 0 | 0 |
| speed error, max (points) | 0.22, 0.28, 0.32, 0.66 | same | 0.026 |
| Mixes stakka, razorjack, nocny (`monitor`) | 11/11, 7/7, 8/9; 1 wrong | 0 wrong | 11/11, 7/7, 9/9; 0 wrong |
| Generated mixes, 115 indexed plays | 105, 0 wrong | 104, 0 wrong | 106, 0 wrong |
| Stakka mix, its references left out | no detection | none | 0 wrong |

Sweeps: 1,280 of 2,160 (59%; 63% within +-5%), 22-53 of 60 per speed; Gunfinger
answers every query Panako answers rightly. The 13 wrong answers (scores 6-12, 4
from held-out queries) name recordings unrelated by title. Nocny's miss: Anthrax. First
starts minus the manifest's: stakka -59 to +145 s, nocny +1 to +156 s (Gunfinger
-68 to +0.1 s; -26 to +16 s, +91 s once). Generated mixes: 8 plays only Gunfinger
finds; 7 only Panako, all possible for Gunfinger. Whole-file `query`: stakka ran out
of heap at the startup script's 256 MB and the JVM hung; at 2 GB (A11), no match on any.

| Cost | Panako (one thread) | Gunfinger |
|---|---|---|
| Index of the library | 1,728 s of per-file fingerprinting for 99,987 s of audio, about 29 min wall; LMDB 237.5 MB + cache 107.6 MB | 50.9 s wall, 471 s CPU, 938 MB; index 39.7 MB + peak store 19.7 MB |
| Sweep of 720 renders | 242-321 s wall, 276-335 s user, 432-445 MB | 113-166 s wall, 839-898 s user, 153-162 MB |
| Development mix | 92-141 s wall, 100-149 s user, 442-457 MB | 15-30 s wall, 110-161 s user, 220-264 MB |
| 12 generated mixes | 133 s wall, 156 s user, 393 MB | 42 s wall, 182 s user, 344 MB |

**Conclusion.** At 262 tracks Panako finds 59% of the sweep queries (Gunfinger all),
26 of 27 development-mix tracks (27), 105 generated-mix plays (106); 14 wrong answers,
none at S* (104 plays there). Per second of audio its index takes about 4 times
Gunfinger's CPU, and 5.8 times the disk; queries 1.5-6 times the wall, similar or less CPU.
