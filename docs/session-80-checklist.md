# Session 80 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-80.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Digests of `docs/pair-verdicts.txt`, the NAS clusters file and the
  answer log in `work/s80/listening-digests-start.txt`
- [x] Brief, checklist and `CLAUDE.md` committed
- [ ] Start build `work/bin/s80-start/`; start regressions per
  `docs/session-rules.md`

## Steps

- [x] 1. Pre-registration committed (brief 80): the matrix, the scores,
  what counts as a precision problem
- [ ] 2. Harness: `mixes --kind drops --case`, tested, opt-in, `regress`
  identical
- [ ] 3. Runs at NAS scale under tmux; renders deleted when done
- [ ] 4. Experiments 0283 (the cases by family) and 0284 (the mistaken
  foreground and the related versions)
- [ ] 5. ADR 0025's open points, decisions queue item 2, roadmap item
- [ ] 6. Read-only review by a subagent; fixes
- [ ] 7. Wrap-up: notes "Session 80", status, checklist; final checks
  (`scripts/check.sh`, `regress session-78-linux` and
  `session-78-linux-nas` identical with `--strict`, digests unchanged,
  `git status` clean)
