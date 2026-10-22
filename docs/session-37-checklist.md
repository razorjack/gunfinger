# Session 37 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-37.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `docs/pair-verdicts.txt` digest in
  `work/s37/listening-digests-start.txt`
- [ ] Start build `work/bin/s37-start/`; `regress session-32-linux` and
  `regress session-33-linux-nas` identical

## Steps

- [ ] 1. Where the side is lost: the stage and rule, counts over all lost
  sides, the generator's ride checked; experiment 0175
- [ ] 2. Dose-response on new seeds (size, ramp, direction, drift);
  development scans' split plays as candidates; experiment 0176
- [ ] 3. Existing options (`--speed-per-stretch`, `--extra-rungs`,
  `--trim-ends`, `--single-pass`, others); experiment 0177
- [ ] 4. A candidate variant (if step 1 names a rule), criteria committed
  before its test seeds; experiment
- [ ] 5. (Ahead) a ride inside the sweep's 30 s excerpts
- [ ] 6. Read-only review by a subagent; findings fixed

## Wrap-up

- [ ] "Session 37" in `docs/session-notes.md`, with the handoff
- [ ] `docs/status.md`, the roadmap, this checklist
- [ ] Final checks: `scripts/check.sh` green; `regress session-32-linux`
  and `regress session-33-linux-nas` identical with the final build;
  `docs/pair-verdicts.txt` unchanged; `git status` clean
