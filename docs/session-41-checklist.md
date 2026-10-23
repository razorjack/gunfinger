# Session 41 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-41.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `docs/pair-verdicts.txt` digest in
  `work/s41/listening-digests-start.txt` (and the clusters file's in
  `work/s41/clusters-digest-start.txt`)
- [x] Start build `work/bin/s41-start/`; `regress session-32-linux` and
  `regress session-33-linux-nas` identical; the start build's reference
  reports (11 development scans at NAS scale, the clusters sample)

## Steps

- [x] 1. Diagnostic build: where the pre-filter's time per hit goes;
  hits per filtered asset; where a passing asset passes (experiment 0189)
- [x] 2. Pre-registration (candidates with exactness arguments and tests,
  timing protocol, stop point), committed before any candidate is timed
- [x] 3. Candidates, each tested and timed; their combination; grouping
  if time allows
- [x] 4. Adoption: tests; the 11 development scans at NAS scale; the
  262-track scans; the clusters sample; both regressions; the full NAS
  clustering into a scratch copy if time allows
- [~] 5. Experiments (0189 and up)
- [ ] 6. Read-only review by a subagent; findings fixed

## Wrap-up

- [ ] "Session 41" in `docs/session-notes.md`, with the handoff
- [ ] `docs/status.md`, the roadmap's speed bullet, the low-bandwidth
  note, this checklist
- [ ] Final checks: `scripts/check.sh` green; both regressions identical
  with the final build; `docs/pair-verdicts.txt` and the clusters file
  unchanged; `git status` clean
