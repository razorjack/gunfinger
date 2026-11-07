# 0290: the band profile under the default matcher: the protocol, recall, grid and robust

**Question** (brief 83, registered in `8bdcb2e`). Under today's default matcher, do 0034's
band profiles (a peak dominates ±round(0.094 k) bins at bin k, within 4-24 for a, 4-16 for b)
keep the protocol's answers at 262 tracks and gain evidence under damage? **Method.** Builds of
`50433c0` with `spread` set in `Profile::CURRENT` (`work/s83/variant-{a,b}.patch`, never merged)
on their own stores; T is `s83-start` on `work/peaks`. `recall` adds 109 NAS files as the
other library (byte copies of the files the development halves name; brief 83). `arm.sh`,
`protocol.py`, `robustgrid.py`, 56 runs, every exit 0; T's sweep 2026 equals `session-78-linux`.
Data: `data/0290-band-profile-protocol-and-recall.json`.

| 262 tracks | T | a (4-24) | b (4-16) |
|---|--:|--:|--:|
| Peaks; share at 2-4 kHz; postings; index MB | 3.97 M; 48%; 7.93 M; 40.1 | 3.64 M; 26%; 7.23 M; 37.3 | 4.21 M; 34%; 8.39 M; 42.0 |
| Sweeps 2026-2029; their true hits | 2,160/2,160, 0 wrong; 2.58 M | the same; 2.82 M | the same; 3.08 M |
| Stakka, leave-outs 3 and 11, 2003, Nocny | 11, 8, 0, 7/7, 9/9; 0 wrong | the same | the same |
| True hits: Stakka; 2003; Nocny | 69,885; 21,906; 13,407 | 92,224; 27,746; 15,601 | 96,973; 30,081; 16,679 |
| `loss`, Stakka: kept; rung hits; postings looked up | 13.0%; 38,770; 19.6 M | 19.0%; 52,069; 12.7 M | 17.1%; 54,775; 16.8 M |
| Calibrate: weakest; strongest false; margin | 675; 100; 6.75× | 652; 94; 6.94× | 750; 111; 6.76× |
| Audio not in the index; false candidates ≥ 30 | 21; 49 | 51; 50 | 52; 71 |
| Recall clean of 522; brief of 1,044 | 522; 722 | 522; 732 | 522; 735 |
| Damaged of 522: found; median true hits; at 60+ | 3; 97; 430 | 9; 97.5; 407 | 22; 110.5; 438 |
| Brief-damaged of 1,044: the same | 0; 57; 485 | 0; 54; 449 | 0; 61.5; 550 |
| Grid, confident at 15 and 20 s of 160; strongest false | 65, 144; 69 | 66, 146; 55 | 66, 146; 67 |
| Robust (2,600): confident; possible or better | 1,938; 2,242 | 1,932; 2,253 | 1,969; 2,271 |
| Blend 0 dB; beatmatched 0 dB; blend -6 dB; broadcast | 31; 28; 72; 66 | 23; 21; 71; 65 | 31; 28; 74; 72 |

No arm has a wrong confident answer in any of these evaluations (recall's held-out queries
included). Damaged recall rests on few recordings: T finds 1 source, a 1 other, b 4.
The strongest audio not in the index is China Cup on The Nine under a and b (shared drums,
judged by ear in session 4); calibrate's pairs of 30 hits or more are read in 0291.

**Bars.** B1 and B3 hold for a and b. B4 holds on its figures (margins 6.94× and 6.76× against
6.75×; 51 and 52 hits under 60), subject to 0291's reading of the pairs. B5 fails for both:
pooled damaged and brief-damaged recall gains +0.38 points (a) and +1.21 (b), under the
registered +2; robust falls under a (1,932, with equal-level blends 31 → 23 and 28 → 21, as
in 0034) and rises under b (1,969). **Conclusion.** Under the default matcher both profiles
keep every answer and raise the evidence of real mixes by 16-39%, but neither shows the
registered gain under combined damage: a peak placement change that helps clean and mixed
audio does not recover the peaks that recall's combined damage removes (0207). b is the
stronger variant on every damage figure here.
