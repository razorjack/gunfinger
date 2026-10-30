# 0250: power-sum queries at NAS scale: mixes, faults, noise and cost

**Question.** Brief 62's runs 3-6: do 0245's fault gains hold at NAS
scale with no new wrong file, and what do noise and a search cost?

**Change.** Harness only (`fcf385d`): `mixes` takes `--other-peaks-dir`.

**Command.** `work/s62/mixes.sh` (seeds 2026-2028, 8 mixes each,
`stereo-control`, `stereo-inverted`), `rest.sh` (`robust --seed 2026
--only control,` the nine `stereo-*`; the scans), `cost.sh`; NAS scale,
downmix and `power-sum-queries` by `s62-run`; `query_side_bar.py`.
Data: `data/0250-query-side-mixes-faults-cost.json`.

| NAS scale | Downmix | Power-sum queries |
|---|--:|--:|
| Mixes, control: first half, across, second half | 91/99, 23/24, 95/100 | the same plays |
| Mixes inverted from the middle: the same | 91/99, 16/24, 1/100 | 91/99, 23/24, 93/100 |
| `control` (mono); `stereo-control` (of 80) | 80; 80 | 80; 80 |
| Inverted; silent | 0; 78 | 80; 78 |
| Delay 0.2, 1, 5 ms | 80, 70, 37 | 80, 80, 80 |
| Noise 10, 0, -10 dB: confident (possible or better) | 80, 29 (80), 0 | 80, 19 (75), 0 |
| Scans: Stakka, 2003 mix, Nocny (tracks identified) | 11, 12, 17 | 11, 12, 17 |
| Stakka scan, A B B A, idle (5:13-5:16): search `wall_seconds` | 24.29, 24.64 s | 27.34, 27.43 s |
| The same: user CPU; peak memory | 673, 682 s; 4.91-4.93 GiB | 761, 763 s; 4.99-5.03 GiB |

Wrong confident: in the mixes only the i-witness upload of Bad Company -
The Nine (Evol Intent VIP) over plays of The Nine, in two mixes of seed
2026, under both front ends; in `robust` only draw 41's held-out Cool
Rock Stuff excerpt as the Mobb Deep / Urban Takover upload (7 queries
to 16, exempt by the bar: the variant keeps that excerpt in the fault
rows); in the scans none. No (query, file) pair is new under the variant.

**Conclusion.** At NAS scale the fault gains of 0245 hold in full
(inverted, 1 and 5 ms: 80 of 80; 93 of 100 plays in the inverted halves),
mono and clean stereo controls and the scans keep every identification,
and no wrong file is new. Noise at 0 dB in one channel stays the cost (29
to 19 confident, 80 to 75 possible). A search costs 12% more wall time
and 12.5% more CPU (0247: 12%, 13%); each pair's reports are identical,
the downmix's equal to `session-54-linux-nas`'s. Bar items 1, 3, 5, 6
pass here; 0249 fails item 2.
