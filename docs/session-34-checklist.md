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
- [x] 3. Runs: slot leave-outs and plain scans at NAS scale; the same at
  262 tracks; generated mixes at five seeds; `gaps` on all (06:49-07:39,
  `work/s34/runs.sh`); 24 single-run leave-outs (08:08-08:30)
- [x] 4. Experiments 0159 (30 s meets C1-C3) and 0160 (method 1 a range,
  not a count; registered interval needs clean intervals)
- [x] 5a. (Ahead) long generated mixes (`mixes --kind long`, 80 mixes);
  experiment 0161
- [x] 5b. (Ahead) the gap against itself, a prototype (`self-match`);
  experiment 0162
- [x] 6. Read-only review by a subagent (08:13-08:24); findings fixed

## Wrap-up

- [x] "Session 34" in `docs/session-notes.md`, with the handoff
- [x] `docs/status.md`, the roadmap's "Time without a confident match",
  the plan, this checklist
- [x] Final checks: `scripts/check.sh` green; `regress session-32-linux`
  and `regress session-33-linux-nas` identical with the final build (the
  calibrate line once the slot reports are moved out, see the notes);
  `docs/pair-verdicts.txt` unchanged; `git status` clean
