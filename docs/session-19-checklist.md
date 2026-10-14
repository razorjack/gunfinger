# Session 19 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-19.md`
for scope and rules.

## Setup

- [ ] `git status` clean at `d867557`
- [ ] Brief, checklist; `CLAUDE.md` points to them
- [ ] Release build in `work/bin/s19-start/`; `baseline session-19-start`
- [ ] The NAS store at the start (38,105 records, `b285da9984422fb5`
  expected); the compiler, RealFFT and RustFFT versions; the load
- [ ] `doctor`: the CLI's saved index current for the store

## Steps

- [ ] 2a. Integer phases per query point in `OffsetSlots`, guarded, with
  today's f64 filter outside the guard
- [ ] 2b. Predicate tests against `densest_clusters`
- [ ] 2c. Counts old against new (instrumented run); split profiles; the
  20% stop point
- [ ] 2d. A B B A x3; `regress`; three NAS reports byte-identical; stakka
  at 262; a 30 s excerpt; memory; kept or deleted; experiment 0108
- [ ] 3a. The lengths of the three (four) arms with factorisations
- [ ] 3b. Timing on fixed decoded input; instrumented weights; plan count
- [ ] 3c. The weighted first-pass time per arm; the 20% stop point; what a
  retrieval study needs
- [ ] 3d. A plan cache, only if plan construction is material
- [ ] 3e. Experiment 0109
- [ ] 4. Read-only review by a subagent; fixes

## Wrap-up

- [ ] Notes "Session 19", status, roadmap ("Where search time goes"), the
  research note (proposals 1 and 2 measured)
- [ ] `regress session-19-start` identical with the final build;
  `scripts/check.sh` green
- [ ] The NAS store's revision at the end
- [ ] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [ ] Commit; the final message
