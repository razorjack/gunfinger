# Session 6 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-6.md`
for scope and rules.

## Setup

- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Release build; `regress session-5-start` identical; baseline
  `session-6-start` (binaries in `work/bin/s6-start/`)
- [x] The NAS store's file counts, digest and revision (`3d16641d2ecb6956`)
  recorded at the start (`docs/brief-6.md`)

## Items

- [x] 1. Store fixes: (a) MP3 in RIFF behind ID3v2, decoded and probed
  with the tag skipped, tags kept; (b) hidden folders skipped and
  counted; (c) truncation tolerance of the larger of 1 s and 1%; unit
  tests; `regress session-6-start` identical; README and doctor wording
  (commit c3ba1b1; three corpus files would now pass the tolerance, not
  indexed)
- 2. (Reserved number; nothing here.)
- [x] 3. Re-index the NAS store once (`index --retry-skipped`, `prune`
  listing, `prune --yes` only for the stale `.incomplete` note); counts,
  digest and revision before and after; `map-library`; NAS-scale asset
  count (experiment 0046: 32 new records, 1,396 skip notes, revision
  `d428ee9585936326`, 27,066 assets)
- [x] 4. The new default matcher (skip at 240 with the link rules):
  baseline `session-6-candidate`; the change with today's matcher behind
  one flag; `regress` both baselines; full protocol at 262 (sweeps
  2026-2029, scan, leave-outs 3 and 11, calibrate, `mixes --count 12`,
  `grid`); register, README, e2e tests, ADR 0008; Sick Note in the notes
  (experiment 0047: candidate reproduced exactly, every change predicted;
  `--single-pass` keeps the old matcher)
- [x] 5. Clusters that join fast uploads: (a) why the 6 development pairs
  cover 44-55%; (b) `clusters` at ±8% with refined speed, the new gap;
  (c) the owner's verdict file, created empty; (d) the 262 run (17
  clusters), the NAS run (timed); what the new clusters explain
  (experiments 0048, 0049: the ±2% ladder; 17 clusters at both sizes,
  216 further rips, 28 borderline pairs, 465 of 505 session 5 false
  confident sweep detections explained; `docs/pair-verdicts.txt` empty;
  `pair` command; the harness's `--other-sample` for item 8 in the same
  commit)
- [x] 6. The NAS-scale protocol under the new default and clusters:
  scan, leave-outs 3 and 11, sweeps 2026 and 2027, calibrate; against
  0039, 0040, 0045; the listening list in the notes (experiment 0051:
  development 11/11 with 0 wrong, sweeps 0 and 24 wrong, margin 1.64×,
  3.41× without the two borderline pairs)
- [x] 7. Grouped tracklists: plays over the same stretch at any speed as
  one entry, `(also: ...)`, cue sheet; unit tests; JSON unchanged; one
  timed `identify --store-only` (13 lines expected); README example and
  roadmap (experiment 0050: 13 lines, 263 s and 4 GB under the default;
  a possible play names the entry it shares material with)
- [x] 8. A scaling curve on real records: seeded subsets of the other
  library (1,000, 3,000, 9,000); scan and sweep 2026 at each; a proposed
  possible tier as a function of size (experiment 0052: unrelated chance
  21-34 hits to 3,200 assets, 64 from 9,200; tier 60 below 30 hits of
  chance, twice the chance above; not applied)
- [x] 9. Wrap-up: session 6 summary in the notes; calibration, roadmap,
  status, NAS plan and checklist; the NAS store checked against item 3's
  record (27,042 / 27,042 / 1,396, digest `bf35ea64...e9ae4f5`, revision
  `d428ee9585936326`: unchanged); `scripts/check.sh` green; commit

## Only if time remains

- [x] 10. NAS-scale sweeps 2028 and 2029 under the new default;
  calibrate over four seeds (experiment 0053: 540/540 each, 8 and 8
  wrong; 40 false confident over four seeds from three pairs; 1.29×,
  3.36× without them)
- [-] 11. Today's matcher at NAS scale with the new clusters: scan and
  sweep seed 2026. Not done: a single-pass sweep takes 66-69 minutes at
  this size (session 5) and did not fit after item 10
