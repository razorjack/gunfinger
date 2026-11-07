# Session 80 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-80.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Digests of `docs/pair-verdicts.txt`, the NAS clusters file and the
  answer log in `work/s80/listening-digests-start.txt`
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] Start build `work/bin/s80-start/`; byte-identical to `s79-final`,
  stores unchanged, so the start regressions are skipped

## Steps

- [x] 1. Pre-registration committed (brief 80): the matrix, the scores,
  what counts as a precision problem
- [x] 2. Harness: `mixes --kind drops --case`, tested, opt-in, `regress`
  identical
- [x] 3. Runs at NAS scale under tmux; renders deleted apart from the
  three listed for listening
- [x] 4. Experiments 0283 (the cases by family) and 0284 (the mistaken
  foreground and the related versions)
- [x] 5. ADR 0025's open points, decisions queue item 2, roadmap item
- [x] 6. Read-only review by a subagent; fixes
- [x] 7. Wrap-up: notes "Session 80", status, checklist; final checks
  (`scripts/check.sh`, `regress session-78-linux` and
  `session-78-linux-nas` identical with `--strict`, digests unchanged,
  `git status` clean)
