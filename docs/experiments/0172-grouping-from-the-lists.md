# 0172: grouping straight from the posting lists, without `part`

**Question.** Session 36, step 4 (the research note's item 5): taking and
grouping are 35% of the search's CPU at 16 jobs (0170), above the 15% the
step requires. Is grouping straight from the lists faster, exactly?

**Variant** (deleted; `work/s36/lines.rs.nopart`, on 0171's 4-byte grouped
hits). For each part, `front_partition` finds each list's run first; the
runs are counted by asset straight from the lists, then scattered from
them into `grouped`: no `part` buffer, each posting read twice from the
list instead of copied once and read twice. A window too wide for
relative anchors takes the 8-byte path, as in 0171.

**Method.** In 0171's A/B (`work/s36/ab-hits.sh`), against `s36-step1`.

| Build | Search, 16 jobs | User CPU | Search, 32 jobs | User CPU |
|---|--:|--:|--:|--:|
| 0169 | 45.2, 45.3 s | 673-674 s | 38.1, 38.0 s | 1,099-1,101 s |
| from the lists, 4-byte grouped hits | 48.9, 48.7 s | 727-729 s | 39.9, 39.9 s | 1,155-1,159 s |
| (0171) 4-byte grouped hits with `part` | 47.3, 47.3 s | 705 s | 38.8, 38.4 s | 1,112-1,114 s |

Reports identical to session 33's apart from `wall_seconds`.

**Conclusion.** Slower by 7.8% at 16 jobs and 5% at 32, and 3% slower than
the same grouped form with `part`, as on the Mac (0096 +6%, 0103 +7%). By
inference, the copy into `part` is cheap next to the scatter, and reading
about 600 list fronts twice costs more than reading one contiguous buffer
twice; with 0171, grouping's cost looks like the scatter's work per hit,
not the buffers' size. Not kept.
