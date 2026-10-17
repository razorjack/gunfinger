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
- [x] 3b. The runs on both shared panels' development halves, every
  brief passage at offsets 0-9 s (`screen`, 07:22-07:40, exit 0; 0142's
  queries reproduced)
- [x] 3c. Counts; experiment 0148
- [x] 4. Recall (b) and (a)+(b) under every rule, today's detections
  reproduced first (0 differences); 0150's 7 Panako-only plays offline;
  experiment 0147
- [x] 5. `seconds`'s unit-test gaps (manifest truth, generated-mix truth,
  margins beside an unindexed track): six tests, no bug (`12d3243`)

## Wrap-up

- [x] 6. Read-only review by a subagent (07:45-08:13): every count
  right; the presentation fixes made 08:13-08:20 and committed
- [x] Notes "Session 29", status, checklist, roadmap bullet (first
  written at 07:44; the review and the checks added)
- [x] `regress session-29-start` identical with the final build
  (`work/bin/s29-final/`, byte-identical to the start's; 157 s)
- [x] `scripts/check.sh` green with rustup's 1.98.0 first on PATH
  (`RUSTUP_TOOLCHAIN=1.98.0`) at `36dc022`: 458 Rust tests, 5 Node tests
- [x] The NAS store's revision at the end: 40,126 records,
  `fed26c76b8fe89ea`, as at the start
- [x] `docs/pair-verdicts.txt` and `work/audition/answers.jsonl`
  unchanged (start digests)
- [x] Commit; the final message
