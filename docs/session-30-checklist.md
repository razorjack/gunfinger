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
- [x] Brief, checklist and `CLAUDE.md` committed on `s30-quant`
- [x] Session 29 finished (message at 08:34; checked: `68dae5b`, clean,
  no `work/bin/s29-*` process)
- [x] Release build of master (Homebrew's 1.98.0) in `work/bin/s30-start/`
  (byte-identical to `s29-final`); `baseline session-30-start` (0.33
  s)
- [x] `s30-quant` merged into master (`2f8f4c9`, no conflict)

## Steps

- [x] 2. Pre-registration (brief, experiment 0149), committed before any
  count
- [x] 3. The diagnostic in `gunfinger-eval` with unit tests (one step
  away counts in N, not E; no correspondence twice; nothing outside the
  passage); committed before the runs (`aecdf93`)
- [x] 4. The runs: (c) and clean, (a), both shared panels' development
  halves, 0055's scenarios (R1-R5, 08:03-09:23, exit 0); counts;
  experiment 0151

## Wrap-up

- [x] 5. Read-only review by a subagent (09:29-09:41); fixes committed
- [x] Notes "Session 30", status, checklist, roadmap bullet (first
  written at 09:28; the review's fixes and the checks added)
- [x] `regress session-30-start` identical with the final build
  (`work/bin/s30-final/`, `ff47fec`'s code = `aecdf93`'s; 156 s)
- [x] `scripts/check.sh` green with rustup's 1.98.0 first on PATH
  (`RUSTUP_TOOLCHAIN=1.98.0`) at `315529c`: 466 Rust tests, 5 Node tests,
  clippy for macOS and Linux (400 s); again after the review's test
  fixes, green (385 s)
- [x] The NAS store's revision at the end: 40,126 records,
  `fed26c76b8fe89ea` (09:29)
- [x] `docs/pair-verdicts.txt` and `work/audition/answers.jsonl`
  unchanged (start digests, 09:29)
- [ ] Commit; the final message
