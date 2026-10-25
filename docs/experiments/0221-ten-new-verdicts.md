# 0221: coverage, supported time and `near_10` on the ten verdicts of 2026-10-25

**Question.** Brief 49, step 4: on judged pairs no measure was fitted on
(session 40's pack, judged by ear on 2026-10-25), do session 40's
measures at their thresholds fitted on the 60 earlier judged pairs (a)
put each pair on its verdict's side?

**Data.** `pairs-s49-new.json`, `drift_offsets.py scores` (values in
`data/0220-drift-offsets-scores.json`). The pack chose these pairs as the joins with the least `near_10` and the
apart pairs with the most, plus two control joins: they lie near the
threshold by construction, not at random.

| Pair (verdict) | Coverage | Supported | `near_10` |
|---|--:|--:|--:|
| Dead Air ~ the RH 66 upload (same) | 0.988 | 0.953 | 0.814 |
| Pyro - Time Is Broken, two rips (same) | 0.978 | 0.789 | 0.758 |
| Alien Girl ~ the PRO014 rip (same) | 0.741 | 0.905 | 0.678 |
| Falcon - The Stand, Speed Up ~ CITRUS 003 (same) | 0.966 | **0.732** | 0.715 |
| Mudslide ~ Futuretech upload (same, control) | 0.996 | 0.998 | 0.998 |
| Psyke ~ the 1210 005 rip (same, control) | 0.993 | 0.987 | 0.987 |
| Drumtrip ~ Propaganda (different) | 0.232 | 0.222 | 0.222 |
| Tower Bass, two mixes (different) | 0.331 | 0.362 | 0.164 |
| We R The Ones VIP ~ Klute remix (different) | 0.542 | 0.496 | 0.301 |
| Wikki Wikki Plate ~ Defjammer (different) | 0.210 | 0.285 | 0.162 |
| Threshold fitted on (a) | 0.579 | 0.741 | 0.552 |

**Result.** All three order the ten without a fault (AUC 1.000). At its
threshold, coverage and `near_10` put every pair on its verdict's side;
supported time puts Falcon, the drifting pair of 0219, 0.009 below its
own. Pooled with (a) (28 same, 42 different): leave-one-out errors 9,
9 and 6 (coverage, supported, `near_10`); AUC 0.915, 0.943, 0.957.

**Margins.** `near_10`'s nearest pairs: Alien Girl (0.678, same) and We
R The Ones VIP (0.301, different). Coverage's: the VIP at 0.542, 0.04
below its threshold.

**Conclusions.** A first test on unseen judged pairs agrees with session
40: `near_10` and coverage make no error, supported time one, on a
drifting copy. Ten pairs chosen near the threshold cannot tell the
measures apart; the pooled leave-one-out errors still favour `near_10`.
