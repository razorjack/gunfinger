# Session 13 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-13.md`
for scope and rules.

## Setup

- [x] `git status` clean at `c066c2a`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [ ] Release build in `work/bin/s13-start/`; `baseline session-13-start`
- [ ] The NAS store at the start: records and revision
  (`f9f9bc706195a71b` expected)

## Steps

- [ ] 2. The fix: entries gone when read are skipped and counted; the root
  rules; `prune` with a listing that skipped entries; `doctor` and
  `index` report the count; ADR; tests; checks pass; `regress
  session-13-start` identical
- [ ] 3. The live NAS: `doctor` several times (audio files, vanished,
  duration); one `identify` of the 2003 mix without `--store-only`;
  experiment 0089
- [ ] 4. Read-only review by a subagent; fixes

## Wrap-up

- [ ] Notes "Session 13"; status; checklist; roadmap item done; README
- [ ] The NAS store's revision at the end
- [ ] Commit; the final message
