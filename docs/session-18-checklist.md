# Session 18 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-18.md`
for scope and rules.

## Setup

- [x] `git status` clean at `300e008`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [ ] Release build in `work/bin/s18-start/`; `baseline session-18-start`
- [ ] The NAS store at the start: 33,723 records, revision
  `c835c5531c325274` expected
- [ ] Digests of `docs/pair-verdicts.txt` and `work/audition/answers.jsonl`

## Steps

- [ ] 2a. `clusters --from-peaks` with the new manifest; `pair` on the
  `# unclear:` and `# related:` files
- [ ] 2b. Scans at 262 tracks and NAS scale, NAS leave-outs (3 seeds),
  `identify` at NAS scale (tracklist and JSON); scores and causes; track
  17's slot apart
- [ ] 2c. The fire-and-forget view, entry by entry
- [ ] 2d. Experiment 0100
- [ ] 3. The containment rule on the new reports and every stored
  report; recommendation; experiment
- [ ] 4. The possible tier: the listening answers in the roadmap and
  `docs/calibration.md`; possible plays per hour at 60-130; true plays
  only possible
- [ ] 5. Listening questions in `work/audition/s18/`; the page checked
  with temporary files
- [ ] 6. Search speed (if time remains)
- [ ] 7. Read-only review by a subagent; fixes

## Wrap-up

- [ ] Notes "Session 18", status, roadmap ("More development mixes", the
  possible tier), ADR 0004's "Later"
- [ ] `regress session-18-start` identical with the final build;
  `scripts/check.sh` green
- [ ] The NAS store's revision at the end
- [ ] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [ ] Commit; the final message
