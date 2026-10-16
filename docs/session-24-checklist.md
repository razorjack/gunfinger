# Session 24 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-24.md`
for scope and rules.

## Setup

- [x] `git status` clean at `53d4136`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Release build in `work/bin/s24-start/` (byte-identical to
  `s23-final`); `baseline session-24-start` (copies reports, 0.36 s);
  `regress session-24-start` as the speed check: identical, 153.45 s
  (session 22: 153 s)
- [x] The NAS store at the start: 40,126 records, `fed26c76b8fe89ea`,
  not the prompt's 39,354 and `d71ba5333cff9a34` (772 records written
  after session 23's end check, the newest at 17:53); listening files as at
  session 23's end

## Steps

- [ ] 2. The research note `docs/research/2026-10-16-version-verification.md`
- [ ] 3. Design and pre-registration in the brief and experiment 0124,
  committed before any run on calibration or evaluation data
- [ ] 4. The verifier in `gunfinger-eval`, synthetic tests, smoke runs
  on a positive control and one of Fire's wrong plays
- [ ] 5. Calibration on the panel's development half; thresholds
  written and committed before step 6
- [ ] 6. Evaluation: E1 (108 true plays), E2 (87 wrong related plays,
  drawn true plays); rejected true plays with causes; timelines of
  examples; experiment file(s)
- [ ] 7. If time remains: the controlled test on two calibration
  families; otherwise its plan for session 25

## Wrap-up

- [ ] 8. Read-only review by a subagent from 03:30 at the latest; fixes
- [ ] Notes "Session 24" with the handoff for session 25, status,
  checklist, roadmap
- [ ] `regress session-24-start` identical with the final build
- [ ] `scripts/check.sh` green with rustup's 1.98.0 first on PATH
  (`RUSTUP_TOOLCHAIN=1.98.0`)
- [ ] The NAS store's revision at the end equals the start's
- [ ] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [ ] Commit; the final message
