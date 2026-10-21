# Session 35 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-35.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `docs/pair-verdicts.txt` digest in
  `work/s35/listening-digests-start.txt`
- [~] Start build `work/bin/s35-start/`; `regress session-32-linux` and
  `regress session-33-linux-nas` identical

## Steps

- [ ] 1. Pre-registration (`docs/research/2026-10-21-gap-tests.md`),
  committed before any test seed is rendered
- [ ] 2. Shared material, beatmatched: `mixes --kind shared` beside the
  control `--kind beatmatched`; experiment 0163
- [ ] 3. A gap inside a found track: `self-match --neighbours`;
  experiment 0164
- [ ] 4. The combined estimate; experiment 0165
- [ ] 5. Pitch rides: `mixes --kind rides`, `self-match --ratios`;
  experiment 0166
- [ ] 6. The proposal `docs/research/2026-10-21-printing-gaps.md`
- [ ] 7. (Ahead) the prototype branch `s35-gaps-output`
- [ ] 8. Read-only review by a subagent; findings fixed

## Wrap-up

- [ ] "Session 35" in `docs/session-notes.md`, with the handoff
- [ ] `docs/status.md`, the roadmap's "Time without a confident match",
  the plan, this checklist
- [ ] Final checks: `scripts/check.sh` green; `regress session-32-linux`
  and `regress session-33-linux-nas` identical with the final build;
  `docs/pair-verdicts.txt` unchanged; `git status` clean
