# Session 15 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-15.md`
for scope and rules.

## Setup

- [ ] `git status` clean at `f3b3191`
- [ ] Brief, checklist; `CLAUDE.md` points to them
- [ ] Release build in `work/bin/s15-start/`; `regress session-14-start`
  with it; `baseline session-15-start`
- [ ] The NAS store at the start: 33,596 records, revision
  `f9f9bc706195a71b` expected

## Steps

- [ ] 2. The verdicts applied: clusters at 262 tracks and at NAS scale
  from the saved pairs, without `--cut-sparsest` first; what changed;
  development scores at both sizes against `session-15-start`;
  experiment 0090
- [ ] 3. Experiment A, the containment rule: parameters (tolerance, hit
  ratio), a grid over every stored development report; which reports
  informed the grid and which only tested it; experiment 0091; a summary
  for the session notes
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
