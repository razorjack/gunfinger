# Session 38 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-38.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `docs/pair-verdicts.txt` digest in
  `work/s38/listening-digests-start.txt`
- [x] Start build `work/bin/s38-start/`; `regress session-32-linux` and
  `regress session-33-linux-nas` identical (1:09; 6:42)

## Steps

- [x] 1. Pre-registration (the rule, its threshold from development data,
  the criteria), committed before any variant run on test data (`393469d`)
- [x] 2. The variant (opt-in, tested, `regress` identical when off),
  measured against the criteria; the two development candidates under it;
  experiment 0179 (S1, S2 and S3 hold)
- [x] 3. A ride inside `robust`'s excerpts, 262 tracks and NAS scale,
  default and both variants; experiment 0180
- [x] 4. What an ADR would decide (`docs/research/`)
- [x] 5. (Ahead) per-second scores under the variant; experiment 0181
- [~] 6. Read-only review by a subagent; findings fixed

## Wrap-up

- [x] "Session 38" in `docs/session-notes.md`, with the handoff
- [x] `docs/status.md`, the roadmap, this checklist
- [~] Final checks: `scripts/check.sh` green; `regress session-32-linux`
  and `regress session-33-linux-nas` identical with the final build;
  `docs/pair-verdicts.txt` unchanged; `git status` clean
