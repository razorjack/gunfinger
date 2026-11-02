# Session 72 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-72.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] Digests of `docs/pair-verdicts.txt` and the NAS clusters file in
  `work/s72/listening-digests-start.txt`
- [x] Start build `work/bin/s72-start/`; `regress session-54-linux` and
  `regress session-64-linux-nas` identical

## Steps

- [x] 1. Design in `docs/brief-72.md`, committed before code
- [x] 2. Implementation: core `residual` and `render`, CLI `--under`,
  printing, tests; harness on the core's code; option off identical
  (outputs, both regressions, the harness's residual report)
- [x] 3. First look: one development mix at 262 tracks and NAS scale,
  one generated drops mix; the added time
- [x] 4. ADR 0025 (Proposed); proposals note §4, roadmap, README and help
- [ ] 5. Read-only review by a subagent; findings fixed
- [ ] 6. Wrap-up: notes "Session 72", status, checklist; final checks
