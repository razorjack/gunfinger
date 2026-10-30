# Session 61 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-61.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] Digests of `docs/pair-verdicts.txt` and the NAS clusters file in
  `work/s61/listening-digests-start.txt`
- [x] Start build `work/bin/s61-start/` from master; `regress
  session-54-linux` and `regress session-54-linux-nas` identical

## Steps

- [x] 1. Pre-registration committed before any variant run: the variant
  and its normalisation, its identity, the conditions, phases A and B,
  the scores, the bar for an ADR proposal
- [ ] 2. The variant, tested (one inverted channel leaves its peaks
  unchanged; a mono file gives the expected peaks; the default path
  untouched); `regress` identical with it off
- [ ] 3. Runs: phase A at 262 tracks and NAS scale; phase B at 262 tracks
- [ ] 4. Experiments: phase A; phase B; cost and density
- [ ] 5. Decision note `docs/research/2026-10-30-stereo-power-decision.md`;
  the roadmap's "Stereo faults before the downmix" and the proposals
  note's §1
- [ ] 6. Read-only review by a separate agent; findings fixed
- [ ] 7. Notes ("Session 61"), status, checklist; final checks
  (`scripts/check.sh`, both regressions with the final build, the
  verdicts file and the clusters file's digest unchanged, `git status`
  clean)
