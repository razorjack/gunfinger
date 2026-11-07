# Session 78 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-78.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Digests of `docs/pair-verdicts.txt`, the NAS clusters file and the
  answer log in `work/s78/listening-digests-start.txt`
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] Start build `work/bin/s78-start/`; byte-identical to `s77-final`,
  stores unchanged, so the start regressions are skipped

## Steps

- [x] 1. Design in the brief: the render recipe, the receipt, mismatches,
  reuse across directories; the watchdog's coverage, heartbeat and
  timeout; the regress coverage and leave-outs
- [ ] 2. Implementation, tested: receipts refuse a changed source, fault
  and renderer; the watchdog on a stalled and on a silently progressing
  child; `regress` names the new reports and reports an old baseline as
  incomplete
- [ ] 3. Runs: `regress session-54-linux` and `regress
  session-75-linux-nas` identical for the reports they hold; refused
  renders counted and timed; baselines `session-78-linux` and
  `session-78-linux-nas`; the new scans' scores against 0232
- [ ] 4. Experiment, only if a figure moved or refused renders that
  earlier experiments used
- [ ] 5. Read-only review by a subagent; fixes
- [ ] 6. Wrap-up: notes "Session 78", status, roadmap queue item 1,
  checklist; final checks (`scripts/check.sh`, both new baselines
  identical also with `--strict`, digests unchanged, `git status` clean)
