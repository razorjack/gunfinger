# Session 40 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-40.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed (`3a037dd`)
- [x] `docs/pair-verdicts.txt` digest in
  `work/s40/listening-digests-start.txt` (and the clusters file's in
  `work/s40/clusters-digest-start.txt`)
- [x] Start build `work/bin/s40-start/`; `regress session-32-linux` and
  `regress session-33-linux-nas` identical (1:10; 6:15)

## Steps

- [x] 1. Pre-registration (populations with sizes and seeds, measures
  defined exactly, scores, what counts as worth a roadmap entry),
  committed before any measure is computed beyond the two source pairs
- [x] 2. `pair` reports supported seconds; `pair_offsets.py` with unit
  tests; `listening_pack.py select --min-hits`, tested; `regress`
  identical
- [x] 3. The runs: `pair` on each population at NAS scale (31-50 s each)
- [x] 4. Experiments 0186 (scores) and 0187 (the pairs the measures get wrong; near-identical against damaged) (measures against the scores; the pairs every
  measure gets wrong; near-identical versions against damaged copies)
- [x] 5a. (Ahead) the false confident detections left at NAS scale (0188; sweeps 2027 and 2028 at NAS scale)
- [x] 5b. (Ahead) a listening pack of the strongest disagreements in
  `work/s40/pack/`
- [ ] 6. Read-only review by a subagent; findings fixed

## Wrap-up

- [ ] "Session 40" in `docs/session-notes.md`, with the handoff
- [ ] `docs/status.md`, the roadmap, this checklist
- [ ] Final checks: `scripts/check.sh` green; both regressions identical
  with the final build; `docs/pair-verdicts.txt` and the clusters file
  unchanged; `git status` clean
