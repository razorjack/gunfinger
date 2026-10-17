# Session 29 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-29.md`
for scope and rules.

## Setup

- [x] Reading (First, item 1)
- [x] The NAS store at the start: 40,126 records, `fed26c76b8fe89ea`;
  listening files recorded (`work/s29/listening-digests-start.txt`)
- [x] Brief and checklist drafted under `work/s29/` while session 28
  finished
- [x] Session 28 finished (message at 07:06; checked at 07:10: `df2f038`,
  `git status` clean, no `work/bin/s28-*` process)
- [x] Brief and checklist in `docs/`; `CLAUDE.md` points to them;
  committed
- [x] Release build (Homebrew's 1.98.0) in `work/bin/s29-start/`
  (byte-identical to `s28-final`); `baseline session-29-start` (0.21
  s)

## Steps

- [x] 2. Worktree `s29-span` from master with `s27-span` merged in
  (`15f3240`); silence lead-ins at 0-9 s with four unit tests
  (`583bfe3`); own target and work directories; today's rule reproduces
  master's grid exactly (06:30)
- [x] 3a. Pre-registration (brief, experiment 0146); committed before
  any count
- [~] 3b. The runs on both shared panels' development halves, every
  brief passage at every declared offset (`screen`, exit lines checked)
- [ ] 3c. Counts; experiment 0147; committed
- [~] 4. Recall (b) and (a)+(b) under every rule, today's levels
  reproduced first; 0150's 7 Panako-only plays offline; experiment
  0148; committed
- [x] 5. `seconds`'s unit-test gaps (manifest truth, generated-mix truth,
  margins beside an unindexed track): six tests, no bug (`12d3243`)

## Wrap-up

- [ ] 6. Read-only review by a subagent (from about 09:45); fixes
  committed
- [ ] Notes "Session 29", status, checklist, roadmap bullet
- [ ] `regress session-29-start` identical with the final build
- [ ] `scripts/check.sh` green with rustup's 1.98.0 first on PATH
  (`RUSTUP_TOOLCHAIN=1.98.0`), including the Node tests
- [ ] The NAS store's revision at the end
- [ ] `docs/pair-verdicts.txt` and `work/audition/answers.jsonl`
  unchanged (start digests)
- [ ] Commit; the final message
