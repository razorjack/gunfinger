# 0138: the minimum span and supported time, counted on the variant build

**Question.** Session 27, step 4, source V (0136): variants A(S) and
B(S) on every panel, from one run of each with the variant build.

**Method.** Worktree `s27-span` (never merged): reports carry
`supported_seconds` and first and last hits; work dir `work/s27/vwork/`
(clones; NAS store at 40,126 records). Identical to master's reports
(`span_rule.py identity`): grid, sweeps, mixes 2026, six scans at 262,
shared 2; master's shared 1, NAS scans and recall are of older stores.
Today's rule reproduced every level of V's reports (0 problems; shared
and `robust` detections taken as fitted). Data `data/0138-v-*.json`.

| Panel | Today | A(10) | A(15) | A(20) | B(10) | B(15) | B(20) |
|---|--:|--:|--:|--:|--:|--:|--:|
| Grid: confident of 960; flips | 693; 30 | 803; 2 | 644; 2 | 482; 1 | 947; 13 | 800; 0 | 640; 0 |
| Grid at 10, 15, 20 s (of 160) | 0, 67, 146 | 3, 160, 160 | 0, 4, 160 | 0, 0, 2 | 147, 160, 160 | 0, 160, 160 | 0, 0, 160 |
| Mixes (136 plays): confident, possible | 125, 11 | 135, 1 | 131, 5 | 114, 22 | 135, 1 | 133, 3 | 120, 16 |
| `robust`: confident of 2,600 | 1,941 | 1,991 | 1,944 | 1,939 | 1,991 | 1,991 | 1,918 |
| Shared 1, played out: wrong of 218 | 106 | 109 | 109 | 106 | 109 | 109 | 109 |
| Shared 1: new wrong detections | | 8 | 6 | 0 | 8 | 6 | 6 |
| Shared 1 and 2: right confident (everything) | 56, 96 | 67, 106 | 67, 104 | 56, 96 | 67, 106 | 67, 106 | 67, 104 |
| Criterion 1; criterion 2 | | no; yes | no; no | yes; no | no; yes | no; yes | no; no |

- Unchanged by every rule: sweeps (2,160 of 2,160), recall (522 of 522
  indexed), scans at 262 (7) and NAS scale (6; 40 of 40, every-track
  leave-outs 0), shared 2's 5 wrong. Elsewhere 0 wrong under every rule.
- Shared 1's new wrong answers are 0137's: the 20 s `alone` queries of
  fire-2, fire-6 and pyro-2 (2 windows, spans 19.6-19.8 s, supported
  20 s, so B(20) admits them) in both indexes, and fire-2's 240 s loop
  (342 hits, span 10.3 s, supported 11 s) under A(10) and B(10).
- Shortest confident evidence on the grid: span 14.3 s today, 11.8 s
  (A(10)), 9.3 s and 10 supported seconds (B(10), 13 flips at 10 s).
  `robust`: 2 s needle skip 31 to 80 of 80 under A(10), B(10), B(15);
  B(20) lowers it to 12 and the 2% pitch ride from 26 to 22. Mixes:
  A(10), B(10) promote 10 plays of 20-26 s; A(20) demotes 11 of 23-28 s.

**Conclusion.** No variant meets both criteria. Every variant but A(20)
makes three shared-material queries newly wrong with the played
recording left out; A(20) adds none but names fewer grid queries.
