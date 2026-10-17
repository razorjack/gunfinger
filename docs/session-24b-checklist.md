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
- [-] 4a-d. Panako's index and queries: not run; no JDK 11 by 22:00
  (the launching agent's instruction, 19:24); `work/panako/run.sh` ready
- [x] 4. Gunfinger's side: saved reports identical with the current
  build; columns and cost (0131)
- [x] 5. Experiment files: 0130 (build, pre-registration, amendments),
  0131 (Gunfinger's side); the comparison's waits on Panako

## Wrap-up

- [x] 6. Read-only review by a subagent (21:30-21:45); fixes: the
  pre-registration amended before any count (brief A1-A10), the scorer
  and the run script, times and loads
- [x] Notes "Session 24b", status entry, checklist, one roadmap bullet
- [-] `scripts/check.sh`: not run; no Rust code changed (it checks only
  Rust); the Python scripts were run
- [x] Nothing of Panako's outside `work/` (no `~/.panako`, no
  `~/.gradle`, the Java preferences file unchanged)
- [x] Commit on `s24b-panako`; the final message with what
  session 25 needs for the merge

## The run of 2026-10-17

- [x] `git merge --ff-only master`: `3a1420a` -> `6c7728c`
- [x] Instruction, state and order in the brief
- [x] Build with Temurin 11 (17 s, Panako's wrapper unchanged)
- [x] `install`, `smoke` (the self-check): JGaborator loads on arm64;
  LMDB fails (`UnsatisfiedLinkError: dlopen(liblmdb.dylib ...)`) in
  `store`, `query` and `stats`, before decoding
- [-] `index-all`, the sweeps, the development and generated mixes,
  `stakka-out`, `dev-query`: not run; the Panako part stopped (native
  LMDB missing on arm64, the instruction); run after 01:22, below
- [x] State outside `work/`: empty Java Preferences nodes removed,
  `be.panako.util.plist` deleted; the prefs plist's content as before
- [x] Experiment 0150; 0130 points to it
- [x] Notes ("The run of 2026-10-17" in "Session 24b"), status entry and
  roadmap bullet updated in place
- [x] Read-only review by a subagent (00:54-01:00); fixes: 0130's
  pointer, the LMDB versions, the README's words quoted, the Node
  project's LMDB described, the cleanup's JDK recorded, predictions
  marked, older notes text pointed to the run, the wait script's path
- [x] Commit; the message with what the merge into master needs

## The run of 2026-10-17, after our approval of LMDB (01:22)

- [x] The instruction recorded verbatim in the brief
- [x] LMDB 0.9.29 from OpenLDAP's tag (`8ad7be25`, the same on the
  GitHub mirror), `libraries/liblmdb` only, into `work/lmdb/`; `file`
  arm64, `mdb_version` exported; nothing installed
- [x] `smoke` passes; setup checks recorded (exploratory)
- [x] 4a `index-all` (stopped at 30 min by the tool's limit, resumed;
  cost from the per-file times)
- [x] 4b sweep seed 2026 (S* = 13), then seeds 2027-2029
- [x] 4c the three development mixes with `monitor`; whole-file
  `query`: heap failure at 256 MB and a hung JVM (stopped), amendment
  A11 before the rerun at 2 GB: no match on any
- [x] 4d seed 2026's generated mixes; I-stakka-out
- [-] 4d `robust`: not run (its conditions were never named, step 3)
- [x] Every log checked for `Exception`/`Error` and its exit line
- [x] Java Preferences node removed again, `be.panako.util.plist`
  deleted; the prefs plist's content as before; no `~/.panako`,
  `~/.gradle`, `~/.m2`; no Panako process left
- [x] Experiment 0150 rewritten with the results; 0130's pointer
- [x] Notes subsection, status entry and roadmap bullet updated in place
- [x] Read-only review by a subagent (05:13-05:23); fixes (brief,
  "05:13-05:25")
- [ ] Commit; the message with the numbers and what the merge needs
