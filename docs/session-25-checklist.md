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

- [x] 2. Clusters at the current revision (20:52-21:24, with
  `--cut-sparsest` after the pre-check stopped on 0120's Sonar chain);
  what changed against 0120; experiment 0128
- [x] 3a. `decode::watch` tests robust under load (test code only);
  checked with busy loops on every core (old 1 of 43 rounds failed,
  new none)
- [x] 3b. `holders` refuses a left-out file that is not indexed, using
  the copy map (refined: a copy beside its corpus file passes)
- [x] 3c. `render_samples_as_indexed` on the store's timeline; files of
  0112 and 0124-0126 affected (Do Drop In, one Miami position); E2's
  rejected drawn true plays 3 -> 2; 0112's outcomes unchanged
- [x] 4a. The second panel's development half: `pair` joins, passages,
  `docs/panels/shared-material-2-seed-2026-development.json`; committed
  before any search (21:40)
- [~] 4b. `shared` at NAS scale (started 21:41), scored under the fresh
  clusters; experiment 0129
- [~] 4c. Pre-registration (committed 21:17); `versions` as built
  (T = 0.121) on 4b's wrong confident plays and positive controls;
  experiment 0132

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
