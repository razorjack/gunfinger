# Session 43 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-43.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `docs/pair-verdicts.txt` digest in
  `work/s43/listening-digests-start.txt` (and the clusters file's in
  `work/s43/clusters-digest-start.txt`)
- [x] Start build `work/bin/s43-start/`; `regress session-32-linux` and
  `regress session-33-linux-nas` identical

## Steps

- [x] 1. Diagnostic build: count pass, prefix sum and scatter apart;
  hits per part and per asset; cache lines the scatter writes; parts of
  512, 1,024 and 2,048; experiment 0199
- [x] 2. Pre-registration (at least three candidates, exactness
  arguments, tests, timing protocol, stop point), committed before any
  candidate is timed
- [~] 3. Candidates, each tested and timed; the combination of those
  that pass
- [ ] 4. Adoption: tests; the 11 development scans at NAS scale and at
  262 tracks; the clusters sample; both regressions; the full NAS
  clustering into a scratch copy if time allows
- [ ] 5. Experiments (0199 and up)
- [ ] 6. Read-only review by a subagent; findings fixed
- [ ] 7. Wrap-up: notes "Session 43", status, roadmap's speed bullet,
  low-bandwidth note; final checks (`scripts/check.sh`, both
  regressions, verdicts and clusters digests, `git status` clean)
