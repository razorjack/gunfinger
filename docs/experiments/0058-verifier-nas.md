# 0058: the frozen peak verifier at NAS scale

**Question.** Item 1 of session 7 at NAS scale (26,972 assets): what do
the listening list's four pairs verify at, next to identifying
detections of the same file, of other rips and of other uploads?

**Command.** `gunfinger-eval --other-peaks-dir <NAS store> --verify`
(commit fe8daf9), default matcher: `scan stakka-skynet-knowledge`, `sweep
--seed 2027` (540/540, 24 wrong, as in 0051). Groups as in 0056; the four
pairs apart, unlabelled. 40,295 detections; `data/0058-verifier-nas.json`.

| Sweep 2027 detections | n | Hits | Share of reference peaks found |
|---|--:|---|---|
| Identifying: the source or an identical copy | 639 | 671-, median 1,060 | 0.72-0.92, median 0.84 |
| Identifying: another rip | 213 | 111-, median 564 | 0.18-0.91, median 0.70, 5th pct 0.44 |
| Identifying: another upload (YouTube archive) | 387 | 56-, median 475 | 0.14-0.87, median 0.65, 5th pct 0.47 |
| China Cup ~ its Prototype upload (16 confident) | 16 | 305-408 | 0.59-0.61 |
| Coma ~ Spraycan | 8 | 285-341 | 0.56-0.58 |
| The Nine ~ Evol Intent VIP | 18 | 84-197 | 0.36-0.54 |
| Synthesis VIP ~ "Synthesis (Remix)" | 8 | 94-115 | 0.34-0.41 |
| Related false (strongest: Clockwork remix) | 718 | to 94 | median 0.06, to 0.41 |

Chance shares 0.007-0.081 (highest for The Nine's VIP); China Cup's 17th
detection has 36 hits (0.13). Development scan: whole plays 0.34-0.67.
Over the 6-134 hits both groups reach (14 identifying, 38,843 false),
AUC of hits 0.864, of excess share 0.950; within 60-120 hits 0.93 and
0.12.

- Those identifying detections are the untagged B side of UDFR014
  (`extra/b-unknown-udfr014-sour.mp3`) against the mixed CD's track 07,
  "Luminous (Remix)", which the clusters join to it (111-117 hits,
  0.18-0.24). Its strongest false detection is track 06, Kraken -
  Meatball (93-112 hits, 0.32-0.37): probably the CD's mix into track 07.
- Falcon - The Stand also finds an untagged `second-library/1.mp3`
  (65-70 hits, 0.29-0.31), labelled unrelated for want of a name.

**Conclusions.** China Cup's and Coma's pairs verify like another upload
of one recording; The Nine's VIP and Synthesis lower, where other rips'
tail and shared passages meet. That can order listening; it decides
nothing. At equal hits the share does not separate weak true detections.
