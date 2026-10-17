# Session 30 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-30.md`
for scope and rules.

## Setup

- [x] Reading (First, item 1)
- [x] The NAS store at the start: 40,126 records, `fed26c76b8fe89ea`;
  listening files recorded (`work/s30/listening-digests-start.txt`)
- [x] Worktree `s30-quant` from master's `36dc022`, its own target
  directory
- [~] Brief, checklist and `CLAUDE.md` committed on `s30-quant`
- [ ] Session 29 finished (message, or my own check from 08:50)
- [ ] Release build of master (Homebrew's 1.98.0) in `work/bin/s30-start/`;
  `baseline session-30-start` (wall time)
- [ ] `s30-quant` merged into master

## Steps

- [ ] 2. Pre-registration (brief, experiment 0149), committed before any
  count
- [ ] 3. The diagnostic in `gunfinger-eval` with unit tests (one step
  away counts in N, not E; no correspondence twice; nothing outside the
  passage); committed before the runs
- [ ] 4. The runs: (c) and clean, (a), both shared panels' development
  halves (0055's scenarios if cheap); counts; experiment file

## Wrap-up

- [ ] 5. Read-only review by a subagent; fixes committed
- [ ] Notes "Session 30", status, checklist, roadmap bullet
- [ ] `regress session-30-start` identical with the final build
- [ ] `scripts/check.sh` green with rustup's 1.98.0 first on PATH
  (`RUSTUP_TOOLCHAIN=1.98.0`)
- [ ] The NAS store's revision at the end
- [ ] `docs/pair-verdicts.txt` and `work/audition/answers.jsonl`
  unchanged (start digests)
- [ ] Commit; the final message
