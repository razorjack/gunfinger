# Session 13 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-13.md`
for scope and rules.

## Setup

- [x] `git status` clean at `c066c2a`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Release build in `work/bin/s13-start/` (byte-identical to
  `work/bin/s12-review/`); `regress session-12-start` with it identical
  (206.5 s); `baseline session-13-start`
- [x] The NAS store at the start: 33,596 records, 1,582 skip notes,
  revision `f9f9bc706195a71b`, as expected

## Steps

- [x] 2. The fix: entries gone when read are skipped and counted; the root
  rules; `prune` refuses a listing that skipped entries; `doctor` and
  `index` report the count; the harness refuses such a listing; ADR 0015
  (ADR 0012's status points to it); tests; checks pass (266 tests);
  `regress session-13-start` identical; README
- [ ] 3. The live NAS: `doctor` several times (audio files, vanished,
  duration); one `identify` of the 2003 mix without `--store-only`;
  experiment 0089
- [ ] 4. Read-only review by a subagent; fixes

## Wrap-up

- [ ] Notes "Session 13"; status; checklist; roadmap item done; README
- [ ] The NAS store's revision at the end
- [ ] Commit; the final message
