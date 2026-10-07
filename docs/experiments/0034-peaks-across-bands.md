# 0034: peaks spread across frequency bands

**Question.** Item 5 of session 4: 74% of hashes sit at 1-4 kHz, where
the mix keeps least (0022). What does each band cost; do lower peaks pay?

**Variants** (commit d533142, `Profile::spread`): a peak dominates ±(9.4%
of its bin) bins instead of ±12 (the same at 1 kHz), within 4-24 bins (a)
or 4-16 (b): more peaks below 1 kHz, fewer above. Each in its own store
(`work/variants/band-{a,b}/`); full protocol under both matchers, no
grid (`s4-band-*.sh`). `loss`: the development mix's 11 plays, true rung.

| True hits per 1,000 postings (postings, M) | <125 Hz | -250 | -500 | -1k | -2k | -4k | All |
|---|--:|--:|--:|--:|--:|--:|--:|
| Today | 3.19 (1.41) | 18.05 (0.12) | 13.09 (0.34) | 8.16 (1.11) | 2.64 (3.98) | 0.65 (12.48) | 2.00 (19.44) |
| a | 4.26 (1.61) | 5.21 (1.63) | 4.45 (2.81) | 4.24 (2.85) | 4.91 (1.67) | 1.98 (1.98) | 4.15 (12.53) |
| b | 4.28 (1.61) | 5.22 (1.63) | 4.46 (2.81) | 4.24 (2.85) | 3.91 (2.25) | 1.08 (5.51) | 3.29 (16.66) |

| 262 tracks | Today | a (4-24 bins) | b (4-16 bins) |
|---|--:|--:|--:|
| Peaks; share at 2-4 kHz; postings; index | 3.93 M; 48%; 7.84 M; 39.7 MB | 3.60 M; 26%; 7.14 M; 37.0 MB | 4.16 M; 34%; 8.29 M; 41.6 MB |
| Development mix: hashes kept; true hits | 13.0%; 39,555 | 19.0%; 53,046 | 17.1%; 55,672 |
| Robust, today's matcher: confident; possible+ | 1,959; 2,258 | 1,955; 2,278 | 1,991; 2,293 |
| Robust, skip at 240: confident; possible+ | 1,995; 2,273 | 1,993; 2,281 | not run |
| Today's matcher, 4 seeds: weakest; strongest false; margin | 403; 97; 4.15× | 511; 103; 4.96× | 552; 128; 4.31× |
| False candidates; 30 hits or more; audio not in the index | 70,506; 55; 28 | 28,037; 94; 43 | 41,647; 115; 45 |
| Skip at 240, 4 seeds: weakest; strongest false; margin | 658; 119; 5.53× | 658; 113; 5.82× | 762; 135; 5.64× |
| False candidates; 30 hits or more; audio not in the index | 4,503; 41; 24 | 3,940; 65; 50 | 7,078; 87; 51 |
| Mixes: confident (today's; skip); strongest false (today's) | 107; 106; 46 | 107; 106; 62 | 107; 106; 65 |

Every cell: sweeps 2,160/2,160, development 11/11, 0 wrong. False
candidates of 30 hits or more are nearly all remixes and records of one
artist; the 43-51 hits are a held-out China Cup finding Bad Company's The
Nine. Robust: a loses on equal-level blends; b gains on blends, broadcast.

**Conclusions.** (1) Anchors at 2-4 kHz are 64% of the postings looked up
and 21% of the true hits. (2) Spreading the peaks raises the real mix's
evidence by a third (a: +34% with 36% fewer postings looked up) and the
margins, most under today's matcher. (3) Shared material between related
records grows too: the strongest audio not in the index nears the
possible tier's 60 hits. Data: `data/0034-peaks-across-bands.json`.
