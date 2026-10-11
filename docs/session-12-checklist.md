# Session 12 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-12.md`
for scope and rules.

## Setup

- [x] `git status` clean at `7ef893f`
- [x] Release build in `work/bin/s12-start/` (byte-identical to
  `work/bin/merged/`); `baseline session-12-start`
- [x] `regress session-12-start` with the start build: identical (720/720,
  87, 93, 79), the known calibrate line; 212.7 s
- [x] The NAS store at the start: 33,596 `.peaks`, 33,596 `.tags`, 1,582
  `.skip`; revision `f9f9bc706195a71b`, as expected; census
  `work/s12/census-start.json` (3,523.4 h)
- [x] The saved NAS index 3.8 GB
- [~] Brief, checklist; `CLAUDE.md` points to them

## Items

- [ ] 2. Profile the merged search (the 2003 mix at NAS scale, `sample`
  every 10 ms); experiment against 0075
- [ ] 3. A faster grouping of hits (or the stage item 2 shows):
  measurement, code, tests; `scripts/check.sh`, `regress
  session-12-start`, the 2003 mix at NAS scale identical and timed
- [ ] 4. The harness uses the saved index: ADR, code, tests (stale,
  damaged, changed store, leave-out); the development scan at NAS scale
  rebuilt and saved, identical and timed; regress; experiment
- [ ] 5. `doctor` reports the saved index and other files in the index
  folder; tests

## Wrap-up

- [ ] Experiments; notes "Session 12"; status; checklist; roadmap; README
- [ ] The NAS store's revision at the end
- [ ] Commit; the final message
