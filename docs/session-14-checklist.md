# Session 14 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-14.md`
for scope and rules.

## Setup

- [x] `git status` clean at `633ebdd`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Release build in `work/bin/s14-start/` (byte-identical to
  `work/bin/s13-review/`); `regress session-13-start` with it identical
  (210.6 s); `baseline session-14-start`
- [x] The NAS store at the start: 33,596 records, revision
  `f9f9bc706195a71b`, as expected

## Steps

- [x] 2. Design: ADR 0016, `docs/audition-questions.md`
- [x] 3. Schema and its check, placement arithmetic, answer log, the
  pair-verdicts effect and undo, the pack and folder producers; tests
- [x] 4. HTTP server (token, Host), audio windows, envelopes and their
  cache, the page (dark theme, queue, lanes, keys, help)
- [x] 5. Page checks with real NAS items (pack 1, 2, a control join, two
  folders), temporary answer and verdict files; screenshots in `work/s14/`
- [x] 6. The detections producer (2003 development mix only): 27
  questions in `work/audition/detections/2003-s12.json`, outside the
  default folder; checked in the page on play 4
- [ ] 7. Read-only review by a subagent; fixes

## Wrap-up

- [ ] AGENTS.md section, README, roadmap items, notes "Session 14", status
- [ ] `regress session-14-start` identical; `scripts/check.sh` green
- [ ] The NAS store's revision at the end
- [ ] `git diff docs/pair-verdicts.txt` empty before every commit
- [ ] Commit; the final message
