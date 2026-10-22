# Session 38 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-38.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `docs/pair-verdicts.txt` digest in
  `work/s38/listening-digests-start.txt`
- [~] Start build `work/bin/s38-start/`; `regress session-32-linux` and
  `regress session-33-linux-nas` identical

## Steps

- [ ] 1. Pre-registration (the rule, its threshold from development data,
  the criteria), committed before any variant run on test data
- [ ] 2. The variant (opt-in, tested, `regress` identical when off),
  measured against the criteria; the two development candidates under it;
  experiment 0179
- [ ] 3. A ride inside `robust`'s excerpts, 262 tracks and NAS scale,
  default and both variants; experiment 0180
- [ ] 4. What an ADR would decide (`docs/research/`)
- [ ] 5. (Ahead) per-second scores under the variant, if it passes
- [ ] 6. Read-only review by a subagent; findings fixed

## Wrap-up

- [ ] "Session 38" in `docs/session-notes.md`, with the handoff
- [ ] `docs/status.md`, the roadmap, this checklist
- [ ] Final checks: `scripts/check.sh` green; `regress session-32-linux`
  and `regress session-33-linux-nas` identical with the final build;
  `docs/pair-verdicts.txt` unchanged; `git status` clean
