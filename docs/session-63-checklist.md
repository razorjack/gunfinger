# Session 63 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-63.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] Digests of `docs/pair-verdicts.txt` and the NAS clusters file in
  `work/s63/listening-digests-start.txt`
- [x] Start build `work/bin/s63-start/` from master; `regress
  session-54-linux` and `regress session-54-linux-nas` identical

## Steps

- [x] 1. Pre-registration committed before any variant run: the
  reference, the split, the bounded fit, the candidates, the
  perturbation, the scores, the bar for the gain and the bar for an ADR
  proposal
- [x] 2. The reference and the split (and the bounded fit), opt-in from
  the harness (`--second-pass-fit`); tests; `regress` identical with it
  off
- [x] 3. Runs at 262 tracks and NAS scale; the perturbation; the cost
- [x] 4. Experiments: the gain on positives and negatives (0251); the
  bounded fit, the bar for the gain having passed (0252); the
  perturbation (0253; renumbered, brief 06:56)
- [x] 5. Decision note `docs/research/2026-10-30-second-pass-fit-decision.md`;
  the roadmap's matching bullets; the proposals note's §3
- [~] 6. Read-only review by a separate agent; findings fixed
- [~] 7. Notes ("Session 63"), status, checklist; final checks
  (`scripts/check.sh`, both regressions with the final build, the
  verdicts file and the clusters file's digest unchanged, `git status`
  clean)
