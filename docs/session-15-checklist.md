# Session 15 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-15.md`
for scope and rules.

## Setup

- [x] `git status` clean at `f3b3191`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Release build in `work/bin/s15-start/`; `regress session-14-start`
  with it identical (217.3 s); `baseline session-15-start`, and
  `session-15-start-nas` from session 11's NAS reports
- [x] The NAS store at the start: 33,596 records, revision
  `f9f9bc706195a71b`, as expected

## Steps

- [x] 2. The verdicts applied: clusters at 262 tracks and at NAS scale
  from the saved pairs, without `--cut-sparsest` first (no chain; the
  option is not needed); what changed (2 joins, 22 confirmed); development
  scores unchanged at both sizes, recall 9 -> 0 wrong; experiment 0090
- [x] 3. Experiment A, the containment rule: parameters (span, tolerance,
  hit ratio), a grid of 24 points over every stored development report
  and 7 new shared passages; which reports informed the grid and which
  only tested it; experiment 0091; the summary drafted in
  `work/s15/notes-draft.md`
- [ ] 4. Experiment B, the possible tier at NAS scale: thresholds 60, 80,
  100, 130; possible plays per hour (right, related, unrelated); weak
  true plays lost; strongest unrelated chance against 0052; experiment
  0092; a summary for the session notes
- [ ] 5. Speed step (only if at least 2 h remain): profile, change, regress,
  the 2003 mix at NAS scale with the old and new builds
- [ ] 6. Read-only review of the session's code by a subagent; fixes

## Wrap-up

- [ ] Notes "Session 15", status, roadmap items touched
- [ ] `regress session-15-start` identical; `scripts/check.sh` green
- [ ] The NAS store's revision at the end
- [ ] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [ ] Commit; the final message
