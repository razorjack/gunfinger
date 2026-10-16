# Session 24b checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See
`docs/brief-24b.md` for scope and rules.

## Setup

- [x] `git status` clean at `81e16e0` on `s24b-panako`
- [x] Reading: CLAUDE.md, AGENTS.md, brief, ADRs 0001, 0004, 0006,
  experiments 0001, 0014, 0047, the roadmap's "Longer hashes", README's
  bibliography, the harness's sweep, render and scoring code
- [x] Brief `docs/brief-24b.md`, this checklist

## Steps

- [x] 2. Panako cloned (`f1248f7`, version 2.1, AGPL-3.0) under
  `work/panako/`; **build blocked**: its wrapper's Gradle 7.2 cannot
  run on Java 21. What it needs is in the brief; the Panako part stops
  (our rule)
- [x] 3. Pre-registration in the brief and experiment 0130, for a later run
- [~] 4a-d. Panako's index and queries: waiting for Java 11 until 22:00
  (the launching agent's instruction, 19:24); `work/panako/run.sh` ready
- [x] 4. Gunfinger's side: saved reports identical with the current
  build; columns and cost (0131)
- [~] 5. Experiment files: 0130 (build, pre-registration), 0131
  (Gunfinger's side) done; the comparison's waits on Panako

## Wrap-up

- [ ] 6. Read-only review by a subagent; fixes
- [ ] Notes "Session 24b", status entry, checklist, one roadmap bullet
- [ ] `scripts/check.sh` only if code changed
- [ ] Nothing of Panako's outside `work/` (none was written)
- [ ] Commit on `s24b-panako`; the final message with what
  session 25 needs for the merge
