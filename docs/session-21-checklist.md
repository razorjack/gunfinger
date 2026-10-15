# Session 21 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-21.md`
for scope and rules.

## Setup

- [x] `git status` clean at `8537226`
- [~] Brief, checklist; `CLAUDE.md` points to them
- [ ] Release build in `work/bin/s21-start/`; `baseline session-21-start`
- [ ] The NAS store at the start (38,105 records, `b285da9984422fb5`
  expected) and the load

## Steps

- [ ] 2. Pre-registration in `docs/brief-21.md`, committed before any
  render: candidates (a), (b), (c), the combinations, the oracle, the
  denominators, the criterion
- [ ] 3. Rendered mixes: `mixes` extended minimally and tested; rendered
  and searched at NAS scale (everything indexed; the played recording
  left out; the positive control)
- [ ] 4. Measurement: every confident play scored; (a), (b), (c) and the
  combinations against the criterion; 0114's true plays apart;
  experiment 0116
- [ ] 5. The outcome (frozen if a candidate passes; the validation half
  not searched)
- [ ] 6. If time remains: the FFT lengths' first stage on the panel's
  development half; an experiment
- [ ] 7. Read-only review by a subagent (by 10:45); fixes

## Wrap-up

- [ ] Notes "Session 21", status, roadmap, the research note
- [ ] `regress session-21-start` identical with the final build;
  `scripts/check.sh` green with both toolchains
- [ ] The NAS store's revision at the end
- [ ] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [ ] Commit; the final message
