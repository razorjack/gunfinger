# 0163: matching a gap against itself when its records share material

**Question.** Session 35, step 2 (pre-registered in
`docs/research/2026-10-21-gap-tests.md`, S1-S2): does method 2 (stretches at 10
hits, 0162's fixed reading) still count the records of a gap when consecutive
missed records share a passage and are beatmatched, as a DJ would play them?

**Change.** `mixes --kind shared` and its control `--kind beatmatched`
(`mixes.rs`): long mixes, every play at the tempo of the one before (`tempo.rs`);
in the shared kind each held-out run of 2 holds one pair of the shared-material
panel (12 pairs, 9 families; passages of 19-280 s), a run of 3 one pair, a run of
4 two, each record played over its side of the passage. `shared_merges.py`: a
couple (two consecutive held-out plays in one gap) is merged when one stretch
overlaps each play's own part by 15 s or more.

**Command.** Seeds 3501-3503 (shared) and 3511-3513 (control), 16 mixes each, 262
tracks, `--jobs 16`: 3:42-4:00 and 14 GiB per seed; `gaps`, `self-match
--neighbours` (6:57 for the 384 gaps), `gap_tests.py --mixes`. All 1,149 indexed
plays confident, no wrong confident play. Data `data/0163-shared-material.json`.

| k = 1 / 2 / 3 / 4, 48 gaps each | Method 2 exact | within 1 | Method 1 exact |
|---|---|---|---|
| Shared | 47 / 11 / 15 / 10 | 48 / 48 / 44 / 28 | 46 / 31 / 19 / 15 |
| Control (beatmatched) | 47 / 36 / 35 / 25 | 48 / 47 / 47 / 45 | 46 / 23 / 23 / 21 |

- S1 fails: pooled over k = 2-4, method 2 is exact for 25% of the shared
  mixes' gaps against 67% in the control (registered: at most 10 points
  below), within one for 83% against 97% (at most 5 below).
- S2 fails: 130 of 192 panel pairs (68%) merge (at most 20%); other couples
  merge 11 of 96 times here, 42 of 288 in the control.
- By how much: a gap holding a merged couple is counted a mean 1.08 records
  low (98 of 111 low); gaps without one, -0.15 (none low). In the control, a
  merged couple (unrelated records) also costs one: 36 of 38 low.
- Method 1, which uses only the gap's length, is exact for 45% and 47% at k =
  2-4, so on the shared mixes it beats method 2. Beatmatching alone changed
  little: the control's 74% at k = 1-4 equals 0162's 74% (not beatmatched).

**Conclusion.** Records sharing a passage, played back to back at one tempo,
usually make one stretch, and method 2 counts one record for two. In a mix of
remixes, VIPs and records on one break, method 2 alone undercounts (see 0165).
