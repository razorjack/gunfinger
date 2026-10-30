# 0245: power-sum queries against today's records (phase A)

**Question.** Brief 61's phase A: do query peaks picked from `|X_L|^2 +
|X_R|^2` match today's records, and what do they win back of 0194's?

**Change.** Harness only (`db502a3`; regressions identical with it off):
`--front-end power-sum-queries` (`spectrogram::Signal::PowerSum`).

**Command.** `work/s61/phase-a.sh` (02:08-02:34), downmix and the variant
by the same build: `robust --seed 2026` (38 default conditions; 9 stereo,
also at NAS scale), `mixes --seed 2026|2027|2028 --count 8
--stereo-fault stereo-control|stereo-inverted`, the three development
scans at both scales; `front_end_summary.py`. Data:
`data/0245-power-sum-queries-against-todays-records.json`.

| Confident of 80 indexed (NAS where it differs) | Downmix | Power sum | Median hits, power sum / downmix |
|---|--:|--:|--:|
| 38 default conditions (mono renders) | each row | the same | 1.00 |
| `stereo-control` | 80 | 80 | 0.95 |
| Inverted; silent | 0; 78 | 80; 78 | -; 1.00 |
| Noise 10, 0, -10 dB | 80, 29, 0 | 80, 18 (19), 0 | 0.95, 0.79, - |
| Delay 0.2, 1, 5 ms | 80, 70, 37 | 80, 80, 80 | 2.8 (1 ms) |
| Mixes: plays in the first half, across, second half (control) | 91/99, 23/24, 95/100 | the same | |
| The same, inverted from the middle | 91/99, 16/24, 1/100 | 91/99, 23/24, 93/100 | |

Development scans, both scales: all referenced tracks identified under
both (Stakka 11, razorjack 7 and 12, Nocny Trans 9 and 17), no wrong or
possible one, the same confident files; hits 0.97, 0.91, 1.00 as many. Wrong confident: none at 262 tracks or in any
mix. At NAS scale only session 40's known pair, the held-out Cool Rock
Stuff excerpt (draw 41) as the Mobb Deep / Urban Takover upload: in
`control` and `stereo-control` under both, and under the variant also in
every fault row it now keeps the excerpt whole (16 queries against 7);
no other file.

**Conclusion.** Query-side compatibility holds: mono renders keep every
row's counts, clean stereo 95% of the hits and every detection, and
inverted channels and delays up to 5 ms are won back in full, in excerpts
and over half a mix. Noise in one channel costs more (0 dB: 18-19
against 29; by inference the downmix adds the channels' shared signal in
amplitude, 3 dB more than their powers). No new wrong file.
