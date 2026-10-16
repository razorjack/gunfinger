# Session 25 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-25.md`
for scope and rules.

## Setup

- [x] `git status` clean at `6f204d5`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Release build in `work/bin/s25-start/` (byte-identical to
  `s24-final`); `baseline session-25-start` (copies reports, 0.00 s);
  `regress session-25-start` as the speed check: identical,
  153.8 s
- [x] The NAS store at the start: 40,126 records, `fed26c76b8fe89ea`;
  listening files as at session 24's end
- [x] `work/reports/library-nas-dnb-peaks/` cloned to `work/s25/before/`

## Steps

- [~] 2. Clusters at the current revision (`screen` `s25-clusters`,
  started 20:49); what changed against 0120; experiment 0128
- [ ] 3a. `decode::watch` tests robust under load (test code only);
  checked with busy loops on every core
- [ ] 3b. `holders` refuses or reports a left-out file that is not
  indexed, using the copy map
- [ ] 3c. `render_samples` on the store's timeline; files of 0112 and
  0124-0127 affected, counts that change
- [ ] 4a. The second panel's development half: `pair` joins, passages,
  `docs/panels/shared-material-2-seed-2026-development.json`; committed
  before any search
- [ ] 4b. `shared` at NAS scale, scored under the fresh clusters;
  experiment 0129
- [ ] 4c. Pre-registration; `versions` as built (T = 0.121) on 4b's
  wrong confident plays and positive controls; experiment 0132

## Wrap-up

- [ ] 5. Read-only review by a subagent from 02:15 at the latest; fixes
- [ ] Notes "Session 25" with the handoff for session 26, status,
  checklist, roadmap
- [ ] `regress session-25-start` identical with the final build
- [ ] `scripts/check.sh` green with rustup's 1.98.0 first on PATH
  (`RUSTUP_TOOLCHAIN=1.98.0`)
- [ ] The NAS store's revision at the end
- [ ] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [ ] Commit; the final message
