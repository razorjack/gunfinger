# Session 27 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-27.md`
for scope and rules.

## Setup

- [x] `git status` clean at `21b0220`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Release build in `work/bin/s27-start/` (byte-identical to
  `s26-final`); `baseline session-27-start` (copies reports, 0.21 s);
  `regress session-27-start` as the speed check: identical,
  154.5 s
- [x] The NAS store at the start: 40,126 records, `fed26c76b8fe89ea`;
  listening files as at session 26's end

## Steps

- [ ] 2. Pre-registration: variants A (minimum span) and B (supported
  time), which rule they replace, segment and play level, the possible
  tier; the panels; the criteria; which panel is counted from which
  data; experiment 0136's first part; committed before any count
- [ ] 3a. The variant build in a worktree, never merged, with its own
  work directory; identity checks against master's reports; its runs
  queued under `screen`
- [ ] 3. Per-second scoring in the harness: generated mixes (exact
  truth), the development mixes with margins stated in advance; unit
  tests; today's matcher as the baseline; committed
- [ ] 4. The counts: every panel under today's rule and each variant;
  changed levels with causes; the shortest identification; experiment
  files

## Wrap-up

- [ ] 5. Read-only review by a subagent (by 04:15); fixes committed
- [ ] Notes "Session 27" with the handoff, status, checklist, roadmap
- [ ] `regress session-27-start` identical with the final build
- [ ] `scripts/check.sh` green with rustup's 1.98.0 first on PATH
  (`RUSTUP_TOOLCHAIN=1.98.0`)
- [ ] The NAS store's revision at the end
- [ ] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [ ] Commit; the final message
