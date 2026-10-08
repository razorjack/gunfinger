# 0056: the frozen peak verifier at 262 tracks

**Question.** Item 1 of session 7: with the tolerance frozen at 1×1
(0054), does a verifier measure separate identifying from false
detections better than hits do, at equal hits?

**Command.** `gunfinger-eval --verify` (commit fe8daf9), default matcher:
sweeps 2027-2029, `scan stakka-skynet-knowledge --leave-out 3` and `11`,
`robust --only combined`, `mixes --count 12`, `grid`. Groups by
`verifier_groups.py`: a false detection is related when its file shares
a name, version or artist with any audio in the query (the grid's
held-out neighbours, a blend's partner, overlapping plays), with Kemal ~
Konflict given. 7,243 detections; data `data/0056-verifier-frozen-262.json`.

| Hits | Identifying: n; excess share | Related false | Unrelated false | AUC within the bin: hits; excess |
|---|---|---|---|---|
| < 20 | 30; 0.016-0.197 | 654; -0.004-0.270 | 3,128; -0.007-0.228 | 0.86; 0.90 |
| 20-60 | 48; 0.098-0.308 | 116; 0.087-0.298 | 3; 0.065-0.087 | 0.65 and 0.15; 0.44 and 0.83 (20-40, 40-60) |
| 60-240 | 118; 0.203-0.755 | 8; 0.187-0.406 | none | 0.70; 0.62 (60-120) |
| ≥ 240 | 3,138; 0.366-0.932 | none | none | |

Excess share is the share of reference peaks found minus its mean over
the four shifted references (chance medians 0.004-0.010). Over the hit
range both groups reach (6-100 hits; 112 identifying, 3,886 false), AUC
of hits 0.950, of excess share 0.953; against related false 0.865 and
0.827; against unrelated false 0.970 and 0.984. Excess support, which
0054 favoured on 5 identifying detections, falls to 0.624 (0.448 against
related false): it does not replicate.

- The weakest identifying detections at 40-66 hits are blend partners in
  `robust --only combined` (excess 0.159-0.237): as low as shared
  material, the Clockwork remix (0.27-0.41 at 86-100 hits) and Aphrodite
  - Fanfare against its Dubstyle version (0.11-0.22 at 58-62).
- The strongest unrelated excesses (0.17-0.23) have 7-9 hits: Ed Rush &
  Optical - Compound next to grid brief 10, Kemal - Mechanizm in the
  development mix. Above 20 hits unrelated false reach 0.087.

**Conclusions.** At equal hits the verifier's share separates no better
than hits: slightly better against unrelated chance, worse against
related records, where a weak true play in a blend verifies like a
shared passage. It does not justify a rule; no level changed.
