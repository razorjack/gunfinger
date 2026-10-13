# Session 17 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-17.md`
for scope and rules.

## Setup

- [x] `git status` clean at `b7ddfb6`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [ ] The machine idle (no other CPU-heavy process; load low)
- [ ] Release build in `work/bin/s17-start/`; `baseline session-17-start`
- [ ] The NAS store at the start: 33,596 records, revision
  `f9f9bc706195a71b` expected

## Steps

- [ ] 2. Search speed: profile (0093's method), the larger cost cut with
  identical results; `regress` identical, the 2003 mix at NAS scale
  byte-identical, A B B A with wall, CPU and peak memory; an experiment
  per change
- [ ] 3. Indexing speed: profile on scratch stores (corpus library, one
  NAS folder); a change only with bit-identical records; an experiment
- [ ] 4a. Listening questions for Watermelon ~ 300 Baud - Monotone and
  Falcon - Fractles ~ Ryme Tyme - We Enter (shared material / chance /
  unsure), outside the default queue; the page checked with a temporary
  answer log
- [ ] 4b. The roadmap's "Behaviour at scale" and "The possible tier at
  NAS scale" (and `docs/calibration.md`) corrected: the strongest
  "unrelated" matches checked since session 10 were shared material
- [ ] 5. Read-only review of the session's code by a subagent; fixes

## Wrap-up

- [ ] Notes "Session 17", status, roadmap
- [ ] `regress session-17-start` identical with the final build;
  `scripts/check.sh` green
- [ ] The NAS store's revision at the end
- [ ] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [ ] Commit; the final message
