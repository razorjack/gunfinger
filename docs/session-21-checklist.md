# Session 21 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-21.md`
for scope and rules.

## Setup

- [x] `git status` clean at `8537226`
- [x] Brief, checklist; `CLAUDE.md` points to them (d8f6d6d)
- [x] Release build in `work/bin/s21-start/` (`gunfinger` byte-identical
  to `s20-final`); `baseline session-21-start`
- [x] The NAS store at the start: 38,105 records, `b285da9984422fb5`, as
  expected;
  listening files as at session 20's start

## Steps

- [x] 2. Pre-registration in `docs/brief-21.md`, committed before any
  render: candidates (a), (b), (c), the combinations, the oracle, the
  denominators, the criterion
- [x] 3. Rendered mixes: `gunfinger-eval shared-mixes` (`shared_mixes.rs`,
  `mixes::draw_around`, `Verifier::beyond`; 8 new tests); 36 mixes
  rendered and searched at NAS scale (everything indexed; the played
  recording left out; the positive control) and the 3 development mixes,
  06:02-06:38 (7.0 GB)
- [x] 4. Measurement: `shared_mixes.py score`; 453 true and 87 wrong
  related confident plays; (a1), (a2), (c)+(a1), (c)+(a2) promising in
  rendered mixes only (the development mixes keep 45-72% at their
  thresholds), (b), (c), (c)+(b) not; 42 other confident plays set aside
  by `pair` (newer uploads), one remix over its original; experiment 0116
- [x] 5. The outcome: the four frozen in `docs/brief-21.md` with the
  caveat; no candidate separates a related version from a real rip;
  the validation half not searched
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
