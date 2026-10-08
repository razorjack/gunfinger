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
- [ ] 5. Clusters that join fast uploads: (a) why the 6 development pairs
  cover 44-55%; (b) `clusters` at ±8% with refined speed, the new gap;
  (c) the owner's verdict file, created empty; (d) the 262 run (17
  clusters), the NAS run (timed); what the new clusters explain
- [ ] 6. The NAS-scale protocol under the new default and clusters:
  scan, leave-outs 3 and 11, sweeps 2026 and 2027, calibrate; against
  0039, 0040, 0045; the listening list in the notes
- [ ] 7. Grouped tracklists: plays over the same stretch at any speed as
  one entry, `(also: ...)`, cue sheet; unit tests; JSON unchanged; one
  timed `identify --store-only` (13 lines expected); README example and
  roadmap
- [ ] 8. A scaling curve on real records: seeded subsets of the other
  library (1,000, 3,000, 9,000); scan and sweep 2026 at each; a proposed
  possible tier as a function of size
- [ ] 9. Wrap-up: session 6 summary in the notes; calibration, roadmap,
  status, NAS plan and checklist; the NAS store checked against item 3's
  record; `scripts/check.sh` green; commit

## Only if time remains

- [ ] 10. NAS-scale sweeps 2028 and 2029 under the new default;
  calibrate over four seeds
- [ ] 11. Today's matcher at NAS scale with the new clusters: scan and
  sweep seed 2026
