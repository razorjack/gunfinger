# Session 34 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-34.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `docs/pair-verdicts.txt` digest in
  `work/s34/listening-digests-start.txt`
- [x] Start build `work/bin/s34-start/`; `regress session-32-linux`
  identical (68.5 s); `regress session-33-linux-nas` identical (11:23)

## Steps

- [x] 1. Pre-registration (`docs/research/2026-10-21-gap-measures.md`)
  and the leave-out design (14 scans per set, the fewest possible),
  committed before any count
- [x] 2. Harness: `scan --leave-out-slots`; `gaps` (listing, truth,
  method 1) with unit tests; `check.sh` green; both regressions identical
  with the run build `work/bin/s34-runs/`
- [~] 3. Runs: slot leave-outs and plain scans at NAS scale; the same at
  262 tracks; generated mixes at several seeds; `gaps` on all (queue
  `work/s34/runs.sh` from 06:35)
- [ ] 4. Experiments: step 1 (the minimum length) and step 2 (method 1 on
  detection edges)
- [~] 5a. (Ahead) long-play generated mixes with runs of held-out tracks
  (`mixes --kind long`, committed; queued after step 3)
- [~] 5b. (Ahead) the gap against itself, a prototype (`self-match`,
  committed; runs after step 3)
- [ ] 6. Read-only review by a subagent; findings fixed

## Wrap-up

- [ ] "Session 34" in `docs/session-notes.md`, with the handoff
- [ ] `docs/status.md`, the roadmap's "Time without a confident match",
  the plan, this checklist
- [ ] Final checks: `scripts/check.sh` green; `regress session-32-linux`
  and `regress session-33-linux-nas` identical with the final build;
  `docs/pair-verdicts.txt` unchanged; `git status` clean
