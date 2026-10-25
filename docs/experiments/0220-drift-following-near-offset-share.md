# 0220: a near-offset share that follows drift, against the registered scores

**Question.** Brief 49, step 2 (registered at `3b159f6`, choice fixed at
`6d20c8e` before the test pairs were measured): does the share of the
shorter file holding hits of alignments on rungs within W% of the fitted
speed whose lines stay within T s of the main one (`drift_W_T`) order
the judged pairs better than `near_10` and supported time?

**Command.** `pair --fine-rungs` (`work/bin/s49-run`) on session 40's
populations and the ten verdicts of 2026-10-25; `drift_offsets.py`
(`work/s49/step4.sh`). (b) and (d) leave out the three test pairs they
held. Data: `data/0220-drift-offsets-*.json`.

**Choice** (development only): `drift_0.8_20`, the fewest leave-one-out
errors (3); one (b) join below its threshold (Ram Trilogy - Beastman).

| Measure | AUC (a) | Thr. | LOO (a) | (b) below | (c) above | Ten: AUC | Ten: errors | Pooled AUC | Pooled LOO |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| `coverage` | 0.896 | 0.579 | 9 | 0 | 1 | 1.000 | 0 | 0.915 | 9 |
| `supported` | 0.935 | 0.741 | 7 | 0 | 5 | 1.000 | 1 | 0.943 | 9 |
| `near_10` (= `drift_0_10`) | 0.952 | 0.552 | 6 | 0 | 1 | 1.000 | 0 | 0.957 | 6 |
| `drift_0.8_20` | 0.989 | 0.843 | 3 | 1 | 0 | 1.000 | 2 | 0.988 | 4 |

No measure puts a (d) pair above. Supported time's error on the ten is
Falcon (0.732, seen in step 1). `drift_0.8_20`'s two are judged same:
Pyro - Time Is Broken's rips (0.827) and Alien Girl ~ the PRO014 rip
(0.801). Its leave-one-out errors on (a): Fractles (0.652), Mind
Overload ~ the RAMM 23 rip (0.832) and We Enter ~ its instrumental (0.905).

**By the registered bar** `drift_0.8_20` is not better: pooled it has
fewer leave-one-out errors (4 against 6 and 9), but two errors on the ten
(against 0 and 1) and a (b) join below its threshold.

**What the fine rungs change** (exploratory): drifting same pairs rise
(Fractles 0.12 to 0.65, Phoenix 0.62 to 0.87, Coma ~ Spraycan 0.55 to
0.97, Falcon 0.72 to 0.89), and so do versions sharing most of a track
(the three Sonar revision pairs, 0.49-0.66 to 0.66-0.77), which lifts the
threshold above two same pairs of the ten. Found after the test, not
confirmed: the runner-up `drift_0.8_10` and five tied with it (4
leave-one-out errors, no (b) join below) make no error on the ten.

**Conclusions.** Hits gathered over rungs near the fitted speed order the
judged pairs better than every earlier measure (AUC 0.99 against 0.95),
but the threshold fitted on 60 pairs does not carry to the ten.
