# 0059: recall from other rips alone, against speed relative to the rip

**Question.** Item 3 of session 7 (roadmap, "The edge of the ladder"):
when only other rips of a recording can answer, how does recall depend
on the play's speed relative to the nearest remaining rip, and what do
three extra rungs at each end change?

**Command.** `gunfinger-eval --other-peaks-dir <NAS store> sweep --seed S
--other-rips` (commit 1c289ee), seeds 2026-2029, default matcher, then
with `--extra-rungs 3`. Each panel's indexed excerpts whose cluster has a
file that is not an identical copy (33-40 of 80), at the 9 sweep speeds:
1,305 queries; sources and identical copies (library map) left out,
26,925-26,931 assets indexed; expected answer: the cluster.
Relative speed: the excerpt's speed combined with each remaining file's
speed from the clusters' pairs, nearest to 0. `relative_speed.py`;
`data/0059-relative-speed.json`.

| Relative speed | Queries | Recalled | With `--extra-rungs 3` |
|---|--:|--:|--:|
| 0-4% | 746 | 718 | 718 |
| 4-8% | 424 | 404 | 404 |
| 8-9% | 100 | 91 | 91 |
| 9.2-10% | 4 | 0 | 3 |
| 10.2-12.1% | 31 | 0 | 0 |
| All | 1,305 | 1,213 | 1,216 |

- Wrong: 24, all from listening-list pairs: China Cup's Prototype upload
  (16; 18 with extra rungs, which reach it at -8%), Synthesis VIP's
  mixed-CD "Synthesis (Remix)" (8).
- The 57 misses within 9% are found below the rule by the remaining
  copy: another vinyl rip of Spirit - Out of Control (124-182 hits, 18
  queries), Phoenix's i-witness upload (25-47, 9), DSCI4 uploads of
  Cryogenic and Lobotomy (170-213, 16), uploads of Blindside (213-239, 7),
  Funktion (229-239, 5), Z-Plane and Recharger (143, 190).
- 881 of 1,213 answers are uploads. Of 233 NAS clusters, 55 span over
  2% in speed, 7 over 4%, 1 over 6% (Falcon - The Stand, 11.5%).

**Conclusions.** Recall holds to 9% of relative speed, gone from 10.2%;
three extra rungs add 9.2-9.5% only. Inside the edge every miss is a
weak copy found below the rule. Nothing changed.
