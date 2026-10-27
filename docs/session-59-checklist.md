# Session 59 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-59.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] Digests of `docs/pair-verdicts.txt` and the NAS clusters file in
  `work/s59/listening-digests-start.txt`
- [x] Start build `work/bin/s59-start/`; `regress session-54-linux` and
  `regress session-54-linux-nas` identical

## Steps

- [x] 1. The trace: query 911 (and one or two hit-count changes) before
  and after the prune, in memory; the lists that move into or out of
  the set aside; the candidates or lines that change; the cause named
  with evidence
- [x] 2. The judgment: expected under ADR 0014 or a defect; ADR 0014 and
  the roadmap's "The filter and famous breaks" (or, for a defect, its
  smallest reproduction, the product unchanged)
- [x] 3. Unit tests of the same-title branches of `robust` and `grid`;
  `regress` identical
- [x] 4. Experiment 0242: the cause, the evidence, the judgment
- [x] 5. Read-only review by a separate agent; findings fixed
- [x] 6. Notes ("Session 59"), status, checklist; final checks
  (`scripts/check.sh`, both regressions with the final build, the
  verdicts file and the clusters file's digest unchanged, `git status`
  clean)
