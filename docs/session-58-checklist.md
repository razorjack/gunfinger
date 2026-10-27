# Session 58 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-58.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] Digests of `docs/pair-verdicts.txt` and the NAS clusters file in
  `work/s58/listening-digests-start.txt`
- [x] Start build `work/bin/s58-start/`; `regress session-54-linux` and
  `regress session-54-linux-nas` identical

## Steps

- [x] 1. Pre-registration (signal, thresholds, query set and cost, the
  known cases as the check, what is worth the user's ear), committed
  before the candidates are listed
- [x] 2. The harness: a report of candidate files with their parts,
  sorted by strength, and a `questions` pack; unit tests on fixtures;
  `regress` identical
- [x] 3. The run at NAS scale (tmux, log), leaving out 0065's candidate
  mixes and what the ignore file leaves out
- [x] 4. Experiment 0241: candidates by kind, the `[CUT]` uploads, the
  check against the known cases
- [x] 5. The listening pack (at most 20 questions) and its command
- [x] 6. Read-only review by a separate agent; findings fixed
- [x] 7. Notes ("Session 58"), status, checklist; final checks
  (`scripts/check.sh`, both regressions with the final build, the
  verdicts file and the clusters file's digest unchanged, `git status`
  clean)
