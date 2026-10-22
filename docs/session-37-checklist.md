# Session 37 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-37.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `docs/pair-verdicts.txt` digest in
  `work/s37/listening-digests-start.txt`
- [x] Start build `work/bin/s37-start/`; `regress session-32-linux` and
  `regress session-33-linux-nas` identical (1:09; 6:05)

## Steps

- [x] 1. Where the side is lost: the stage and rule, counts over all lost
  sides, the generator's ride checked; experiment 0175 (the second
  `strongest_per_moment`, after the second pass's margin)
- [x] 2. Dose-response on new seeds (size, ramp, direction, drift);
  development scans' split plays as candidates; experiment 0176
- [x] 3. Existing options (`--speed-per-stretch`, `--extra-rungs`,
  `--trim-ends`, `--single-pass`, others); experiment 0177
- [x] 4. A candidate variant (if step 1 names a rule), criteria committed
  before its test seeds (`8899de4`); experiment 0178 (V1 and V3 hold; V2 fails by its letter on the shared panel)
- [-] 5. (Ahead) a ride inside the sweep's 30 s excerpts (not done:
  steps 1-4 took the time; brief 07:21)
- [x] 6. Read-only review by a subagent; findings fixed

## Wrap-up

- [x] "Session 37" in `docs/session-notes.md`, with the handoff
- [x] `docs/status.md`, the roadmap, this checklist
- [x] Final checks (08:00-08:11, `work/bin/s37-final/` from `c684370`): `scripts/check.sh` green; `regress session-32-linux`
  and `regress session-33-linux-nas` identical with the final build;
  `docs/pair-verdicts.txt` unchanged; `git status` clean
