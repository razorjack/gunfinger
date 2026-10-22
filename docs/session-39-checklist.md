# Session 39 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-39.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `docs/pair-verdicts.txt` digest in
  `work/s39/listening-digests-start.txt`
- [~] Start build `work/bin/s39-start/`; `regress session-32-linux` and
  `regress session-33-linux-nas` identical

## Steps

- [ ] 1. Pre-registration (scenarios, truth, scores, what counts as a
  problem; the tag check and how many recordings qualify), committed
  before any count
- [ ] 2. The generator: double drops and the scenario kind, tested,
  `regress` identical
- [ ] 3. The runs: render, CLI `identify -f json` and `show` at 262
  tracks (store-only copy without the held-out records); NAS scale if
  time allows; the scoring script
- [ ] 4. Experiments (0182 and up)
- [ ] 5. (Ahead) the same scenarios at the detection level (`mixes`'s
  scores, `seconds`)
- [ ] 6. Read-only review by a subagent; findings fixed

## Wrap-up

- [ ] "Session 39" in `docs/session-notes.md`, with the handoff
- [ ] `docs/status.md`, the roadmap, this checklist
- [ ] Final checks: `scripts/check.sh` green; `regress session-32-linux`
  and `regress session-33-linux-nas` identical with the final build;
  `docs/pair-verdicts.txt` unchanged; `git status` clean
