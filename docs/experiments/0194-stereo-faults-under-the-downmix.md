# 0194: what each stereo fault costs today's downmix

**Question.** What does a fault of one channel cost FFmpeg's downmix
(`-ac 1`, each channel at 1/sqrt(2))? Brief 42's pre-registration.

**Change.** Harness only (tested; both regressions identical): nine
`robust` conditions, run only when named in `--only`: the excerpts in
stereo, the right channel inverted, silent, under pink noise (10, 0, -10
dB below its RMS) or late by 9, 44 or 221 samples at 44.1 kHz (0.2, 1, 5
ms), against `stereo-control`; stereo MP3s at 128 kbit/s.

**Command.** `robust --seed 2026 --only stereo-control,...` at 262 tracks
(212 assets; 1:36) and NAS scale (40,580; 3:50), `robust_stereo.py`.
Data: `data/0194-stereo-faults-under-the-downmix.json`.

| Fault (80 indexed queries; both scales alike) | Confident | Possible+ | Mono or nearly mono / stereo | Median hits vs control |
|---|--:|--:|--:|--:|
| None (`stereo-control`) | 80 | 80 | 32/32, 48/48 | 1.00 |
| Polarity inverted | 0 | 6 | 0/32, 0/48 | 0.00 |
| Silent | 78 | 80 | 32/32, 46/48 | 0.66 |
| Noise, 10 dB | 80 | 80 | 32/32, 48/48 | 0.62 |
| Noise, 0 dB | 29 | 80 | 6/32, 23/48 | 0.18 |
| Noise, -10 dB | 0 | 0 | 0/32, 0/48 | 0.00-0.01 |
| Delay 0.2 ms | 80 | 80 | 32/32, 48/48 | 0.45 |
| Delay 1 ms | 70 | 80 | 32/32, 38/48 | 0.30 |
| Delay 5 ms | 37 | 80 | 16/32, 21/48 | 0.20 |

`stereo-control` keeps 81-83% of the mono `control`'s hits. Wrong
confident: none at 262 tracks; at NAS scale only the held-out Cool Rock
Stuff excerpt found as the Mobb Deep / Urban Takover upload (session
40), in `stereo-control`, silent and noise at 10 dB, as in the mono
`control`. Raised by a fault: one possible answer at NAS scale (silent:
a Cause 4 Concern excerpt as a remix by Cause 4 Concern, 68 hits).

**Conclusion.** An inverted channel loses every excerpt: the downmix
keeps only the side, 10-20 dB below the mid in most sources (0196) and,
by inference, unlike the indexed downmix. Noise at -10 dB (all) and 0
dB (64%), and delays of 5 ms (54%) and 1 ms (13%) also cross the bar of
8 of 80; a silent channel (2), noise at 10 dB and 0.2 ms do not. No
fault raised a wrong confident answer, at either scale.
