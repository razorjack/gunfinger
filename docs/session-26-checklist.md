# Session 26 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-26.md`
for scope and rules.

## Setup

- [x] `git status` clean at `0aa1dea`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Release build in `work/bin/s26-start/` (byte-identical to
  `s25-final`); `baseline session-26-start` (copies reports, 0.24 s);
  `regress session-26-start` as the speed check: identical,
  153.9 s
- [x] The NAS store at the start: 40,126 records, `fed26c76b8fe89ea`;
  listening files as at session 25's end

## Steps

- [x] 2. Merge `s24b-panako` with a merge commit (`2eee100`); notes, status and
  roadmap kept whole and in time order; experiments 0130 and 0131 keep
  their numbers; `docs/brief-24b.md` in CLAUDE.md's list; `regress
  session-26-start` identical after the merge (155.2 s); worktree, its
  `work/` and the branch left in place
- [x] 3. Version 2's design and pre-registration (the measured line, the
  kill rule in blends, the split, T's rule, the criteria, the second
  use); experiment 0133's first part; committed before any run
- [x] 4. Implementation in `gunfinger-eval versions` with unit tests (a
  drifting copy, a line off by 150 ms, a jump, a stray, no line, a blend
  with a bass cut); smoke run on two calibration items; committed
- [x] 5a. Calibration: T = 0.00683 by version 1's rule (156 of 312
  negatives rejected, 72 of 75 positives kept); experiment 0133;
  committed before any evaluation run
- [x] 5b. E1 and E2 against the criteria (E1 9 of 108: fails; E2 45 of
  87: passes); every rejected true play with why; experiment 0134
- [ ] 5c. Side sets, deciding nothing: the second panel's development
  half, E2's true drawn plays and positive control

## Wrap-up

- [ ] 6. Read-only review by a subagent (from 04:45 at the latest);
  fixes committed
- [ ] Notes "Session 26" with the handoff, status, checklist, roadmap
- [ ] `regress session-26-start` identical with the final build
- [ ] `scripts/check.sh` green with rustup's 1.98.0 first on PATH
  (`RUSTUP_TOOLCHAIN=1.98.0`)
- [ ] The NAS store's revision at the end
- [ ] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [ ] Commit; the final message
