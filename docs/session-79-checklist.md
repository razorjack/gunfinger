# Session 79 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-79.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Digests of `docs/pair-verdicts.txt`, the NAS clusters file and the
  answer log in `work/s79/listening-digests-start.txt`
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] Start build `work/bin/s79-start/`; byte-identical to `s78-final`,
  stores unchanged, so the start regressions are skipped

## Steps

- [x] 1. Pre-registration committed (brief 79): the fix, the parallel
  design, the scores, the fresh seeds, what the numbers would mean
- [x] 2. Implementation, tested: the saved failures kept; a repeated
  upload and an aligned unrelated drop still dropped; identical output
  with 1 and many workers; option off byte-identical
- [~] 3. Runs: cost A B C C B A (both scales); G seeds 2050 and 2053 at
  NAS scale; confirmation seeds 2091 and 2092
- [ ] 4. Experiments 0281 (the drop rule) and 0282 (the cost)
- [ ] 5. ADR 0025's open points, decisions queue item 2, roadmap item
- [ ] 6. Read-only review by a subagent; fixes
- [ ] 7. Wrap-up: notes "Session 79", status, checklist; final checks
  (`scripts/check.sh`, `regress session-78-linux` and
  `session-78-linux-nas` identical with `--strict`, digests unchanged,
  `git status` clean)
