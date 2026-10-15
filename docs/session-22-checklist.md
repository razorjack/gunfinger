# Session 22 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-22.md`
for scope and rules.

## Setup

- [x] `git status` clean at `86b1b8d`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Release build in `work/bin/s22-start/` (byte-identical to
  `s21-final`); `baseline session-22-start`
- [x] The NAS store at the start: 38,105 records, `b285da9984422fb5`, as
  expected; listening files as at session 21's start

## Steps

- [x] 2. The variant (factor2, first pass only) in a worktree never
  merged; the comparisons and the boundary tolerance written in the
  brief before any run; the variant's own work directory
- [x] 3a. `regress session-22-start` with the variant: 720/720 sweep
  answers, 11/8/0 identified, 0 wrong, no confidence change; 4
  boundaries moved
- [x] 3b. The three development mixes at NAS scale, both builds: the
  same tracks identified and wrong detections; one possible play added;
  two confident segments change inside confident plays
- [x] 3c. Generated mixes, seeds 2026 and 7, both builds: levels,
  credits and 0 wrong unchanged; 2 starts inside fade-ins worse by
  1.33 s and 1.41 s (the boundary criterion fails on them)
- [x] 3d. Sweeps 2027-2029, both builds: every answer unchanged, the
  bar passed by both (60/60 at every speed, 0 wrong)
- [x] 3e. `grid` (confident and wrong unchanged; two 10 s plays lose
  their possible answer), `robust` seed 2026 (0 wrong; 20 right
  confident answers lost, 24 gained), the development mixes' leave-outs
  at NAS scale (unchanged); added after the plan, written first: the
  key-lock-only control (byte-identical), the other two development
  mixes at 262 tracks with leave-outs (unchanged), the recall panel's
  development half (unchanged) and other rips at NAS scale for seeds
  2026-2029 (one confident answer gained, one possible lost and one
  gained); all runs done by 10:48
- [x] 4. Experiment 0119 (40 lines, data `0119-*`); part 2's needs
- [~] 5. Read-only review by a subagent, from 10:51; fixes

## Wrap-up

- [x] Notes "Session 22", status, roadmap, the research note (drafts,
  before the review)
- [x] `regress session-22-start` identical with the main tree's final
  build (byte-identical to the start build); `scripts/check.sh` green
  with both toolchains (396 tests)
- [x] The NAS store's revision at the end (`b285da9984422fb5`, unchanged)
- [ ] The worktree removed
- [ ] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [ ] Commit; the final message
