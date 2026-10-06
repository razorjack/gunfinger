# 0017: the fullest 1% dropped, through the full protocol

**Question.** Experiment 0013 found that emptying the fullest 1% of
posting lists removes most chance evidence and 62% of lookups. Does it
pass the protocol a default change needs?

**Command.** A scratch build whose `Index::build` ends in
`without_fullest(0.01)`; `gunfinger-eval --work work/variants/drop-0.01
regress session-2-start`, then `calibrate`. The same regress without the
drop ran right after on the same machine. Data:
`data/0017-drop-fullest-full-protocol.json`.

**Results.**

| Measurement | Without | 1% dropped |
|-------------|---------|------------|
| Sweep, 9 speeds × 60 | 100%, 0 wrong, error ≤ 0.016% | 100%, 0 wrong, error ≤ 0.016% |
| Development set; leave-out 3, 11 | 11/11; 8/11, 0/11; 0 wrong | 11/11; 8/11, 0/11; 0 wrong |
| Weakest identifying | 501 hits | 445 hits |
| Strongest false (the Clockwork remix) | 95 hits | 76 hits |
| Margin | 5.27× | 5.86× |
| Audio not in the index | 19 hits | 15 hits |
| False candidates in all reports | 8,986 | 506 |
| Detections, development mix | 4,181 | 139 |
| Regress (sweep and three scans) | 182 s | 189 s |

The development mix keeps one possible play matching no track (the remix,
73 hits); every other non-confident detection has at most 25 hits.

**Conclusions.** (1) The drop passes the protocol and widens the margin at
both ends: the rule sits 2.63 times above the strongest false candidate
(was 2.11) and 2.23 times below the weakest identifying (was 2.50). (2)
Nearly all weak candidates vanish, which makes `explain` and the possible
tier quieter. (3) No speed gain at 262 tracks; the gain is in lookups,
which dominate only in a large library (0013). (4) The default is
unchanged: it would lower every hit count by about 17% and needs a test
evaluation to confirm, which is the owner's call (notes for the owner).
