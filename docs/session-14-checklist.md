# Session 14 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-14.md`
for scope and rules.

## Setup

- [x] `git status` clean at `633ebdd`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [ ] Release build in `work/bin/s14-start/`; `baseline session-14-start`
- [ ] The NAS store at the start: 33,596 records, revision
  `f9f9bc706195a71b` expected

## Steps

- [ ] 2. Design: ADR 0016, `docs/audition-questions.md`
- [ ] 3. Schema and its check, placement arithmetic, answer log, the
  pair-verdicts effect and undo, the pack and folder producers; tests
- [ ] 4. HTTP server (token, Host), audio windows, envelopes and their
  cache, the page (dark theme, queue, lanes, keys, help)
- [ ] 5. Page checks with real NAS items (pack 1, 2, a control join, two
  folders), temporary answer and verdict files; screenshots in `work/s14/`
- [ ] 6. The detections producer (2003 development mix only), or its
  documentation
- [ ] 7. Read-only review by a subagent; fixes

## Wrap-up

- [ ] AGENTS.md section, README, roadmap items, notes "Session 14", status
- [ ] `regress session-14-start` identical; `scripts/check.sh` green
- [ ] The NAS store's revision at the end
- [ ] `git diff docs/pair-verdicts.txt` empty before every commit
- [ ] Commit; the final message
