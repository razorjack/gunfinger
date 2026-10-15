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

- [~] 2. The variant (factor2, first pass only) in a worktree never
  merged; the comparisons and the boundary tolerance written in the
  brief before any run; the variant's own work directory
- [ ] 3a. `regress session-22-start` with the variant
- [ ] 3b. The three development mixes at NAS scale, both builds
- [ ] 3c. Generated mixes, seeds 2026 and 7, both builds
- [ ] 3d. Sweeps 2027-2029, both builds
- [ ] 3e. If time remains: `grid`, `robust` seed 2026, the development
  mixes' leave-outs at NAS scale
- [ ] 4. Experiment 0119; part 2's needs
- [ ] 5. Read-only review by a subagent (by 11:15); fixes

## Wrap-up

- [ ] Notes "Session 22", status, roadmap, the research note
- [ ] `regress session-22-start` identical with the main tree's final
  build; `scripts/check.sh` green with both toolchains
- [ ] The NAS store's revision at the end
- [ ] The worktree removed
- [ ] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [ ] Commit; the final message
