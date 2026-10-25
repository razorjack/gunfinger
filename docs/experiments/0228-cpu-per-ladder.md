# 0228: search cost per ladder on the Stakka scan

**Question** (`docs/brief-51.md`, 07:43, C4 and criterion C). What do 1-5
extra rungs cost today, after sessions 36-46 made the search cheaper?
**Method.** `scan stakka-skynet-knowledge --extra-rungs k` (the
default matcher, 32 workers) at 262 tracks and NAS scale (the saved
index), in the order L0 L1 L2 L3 L5 L5 L3 L2 L1 L0 at each scale, alone
on the otherwise idle Linux machine (queue 3c, 09:20-09:26; no other
process above 2% CPU at 09:20). CPU: user + system of `/usr/bin/time`
(at NAS scale including the saved index's load, the same for every
ladder); search: the report's `wall_seconds` (decode and search). Each
cell the median of two runs [range]; `data/0228-cpu-per-ladder.json`.

| Ladder (rungs) | 262: CPU s | search s | NAS: CPU s | search s | Rungs |
|---|--:|--:|--:|--:|--:|
| L0 (82) | 119.4 [119.1-119.8] | 6.5 | 667.2 [663.9-670.5] | 24.0 [23.9-24.1] | 1 |
| L1 (86) | 124.6 (×1.043) | 6.6 (×1.025) | 697.9 (×1.046) | 25.1 (×1.047) | ×1.049 |
| L2 (90) | 132.1 (×1.106) | 6.9 (×1.061) | 729.8 (×1.094) | 26.0 (×1.083) | ×1.098 |
| L3 (94) | 138.4 (×1.159) | 7.0 (×1.088) | 761.4 (×1.141) | 27.0 (×1.126) | ×1.146 |
| L5 (102) | 149.9 (×1.255) | 7.5 (×1.150) | 819.7 (×1.229) | 28.8 (×1.203) | ×1.244 |

Ranges within each pair: at most 1.0% of the median. CPU follows the
number of rungs at both scales (the index's load at NAS scale is small
beside the search); the wall time grows a little less.
**The targeted estimate.** The default NAS-scale scan leaves 2.0% of the
mix (67 of 3,370 s) without a confident detection (`ladder_edge.py
uncovered`; the same at 262 tracks), so T_k ≈ L0 + 0.02 × (L_k - L0):
×1.001 (T1), ×1.002, ×1.003, ×1.005 (T5) in CPU.
**Against the registration.** C (NAS-scale CPU and search time each
within +15%) holds for L1, L2 and L3 and fails for L5 (+22.9%, +20.3%).
T_k's estimate is within +5% for every k. 0033's +14% for L3 holds
today (+14.1% at NAS scale): the search is cheaper, and the extra rungs
cost the same share of it.
**Conclusion.** Each extra rung pair costs 4.3-5.3% more search CPU at
both scales; a targeted search would cost almost nothing on a mix whose
time is nearly all confidently covered.
