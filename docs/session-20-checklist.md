# Session 20 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-20.md`
for scope and rules.

## Setup

- [x] `git status` clean at `2836414`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [ ] Release build in `work/bin/s20-start/`; `baseline session-20-start`
- [ ] The NAS store at the start (38,105 records, `b285da9984422fb5`
  expected); the load

## Steps

- [ ] 2a. Families: the 13 `different` pairs, the two `shared` answers,
  0055's and 0076's judged pairs, with rips and other versions
- [ ] 2b. Shared passages from `pair`, with the distinctive material
  around them
- [ ] 2c. The oracle and denominator written first; split by family
  (seed 2026); the panel frozen in `docs/panels/`
- [ ] 2d. Experiment 0110 (the panel)
- [ ] 3. `shared` at NAS scale, with context before, a weak rip, a blend
  masking the context, a short break repeated; tested; the development
  half in one command; outcomes per scenario and index; experiment 0111
- [ ] 4. Proposal 5's diagnostic on the development half: where the
  evidence falls, audio, peaks and pairs against the rips' noise floor,
  one of three outcomes per family; experiment 0112
- [ ] 5. Listening questions in `work/audition/s20/`, the page checked
  with temporary files
- [ ] 6. If time remains: proposal 3's first step (`take_part` split
  profile); an experiment
- [ ] 7. Read-only review by a subagent; fixes

## Wrap-up

- [ ] Notes "Session 20", status, roadmap ("A confidence statistic for
  shared material"), the research note (proposal 5 measured)
- [ ] `regress session-20-start` identical with the final build;
  `scripts/check.sh` green
- [ ] The NAS store's revision at the end
- [ ] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [ ] Commit; the final message
