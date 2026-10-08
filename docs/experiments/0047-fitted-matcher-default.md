# 0047: the fitted matcher as the default, at 262 tracks

**Question.** Item 4, the owner's decision: skip at 240 with the link
rules (`--second-pass --skip-fullest 0.01 --nearby-rungs --strong-gaps`)
as the default. Does it reproduce the candidate exactly, and does every
change against the single pass match experiments 0026 and 0030?

**Change.** `search::Matcher::{Fitted, SinglePass}`, `--single-pass` in
the CLI and harness (ADR 0008). Baseline `session-6-candidate` saved with
the item 1 binary. Protocol: `work/scripts/s6-protocol.sh`; data in
`data/0047-fitted-matcher-default.json`.

| Check | Result |
|---|---|
| Plain `regress session-6-candidate` | identical: 720/720 sweep queries; 87, 93, 79 scan detections |
| `regress session-6-start` (single pass before) | 701 sweep queries differ; recall 60/60 per speed and 0 wrong before and after; development 11/11, leave-outs 8/11 and 0/11, 0 wrong |
| True plays' strongest hits, after over before | 0.84-1.16; Star Trails 1,839 → 1,540 (split at the DJ's speed change, 0030), Logistics 5,779 → 6,425 |
| Development detections matching no track; 30 hits or more | 6,096 → 69; 2 → 3; strongest 94 (the Clockwork remix) both |
| Sweeps 2026-2029 | 540/540 each, 0 wrong; 180 s wall, 1,555 s CPU per seed |
| `calibrate` (four seeds, scans) | weakest 658 (Illuminati - Melange, -3%), strongest false 99 (Clockwork ~ its Stakka remix), margin 6.65× (single pass 4.15×); audio not in the index 22 (28) |
| Mixes (12, seed 2026) | 106 confident, 9 possible, 0 wrong (single pass 107, 8); only mix 10's Dominion (21.5 s) confident → possible, 525 hits in 2 windows |
| Grid, confident at 10, 15, 20 s | 0, 67, 146 of 160 (single pass 8, 76, 148); 0 wrong, strongest false 34 |

Every difference is of a kind 0026 and 0030 predict: more hits on true
plays (0026: 0.85-1.17), 99% fewer weak false candidates, mix 10's
Dominion and the grid's 10 s plays confident only through a chance
window before, Star Trails split where its speed changes by more than
0.6% between windows. The 46 false candidates of 30 hits or more are the
Clockwork remix, Aphrodite's Fanfare against its Dubstyle and one pair of
Aphrodite records (38 hits), as in 0030.

The end-to-end test's synthetic needle skip (4 s on, same kick every
bar) shows a consequence not seen in the corpus: the second pass measures
up to 10 s before a chain, takes in the kicks before the skip, and drops
the overlapped part before it. The play stays confident from 1:45, not
1:36. The test checks both matchers.

**Conclusions.** The default reproduces the candidate bit for bit, and
every change against the single pass is predicted. Sick Note (209 hits in
the test mix under the single pass) most likely becomes possible.
