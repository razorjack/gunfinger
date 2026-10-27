# Session 55 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-55.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] Digests of `docs/pair-verdicts.txt` and the NAS clusters file in
  `work/s55/listening-digests-start.txt`
- [~] Start build `work/bin/s55-start/`; `regress session-54-linux` and
  `regress session-54-linux-nas` identical

## Steps

- [ ] 1. ADR 0023 (accepted 2026-10-27), committed before the code
- [ ] 2. The gap list (core, shared with the harness), the tracklist,
  table and JSON; unit, output and end-to-end tests; help texts and
  README; `tracklist_score.py` passes over gap lines (tested);
  `scripts/check.sh` green
- [ ] 3. The printed tracklists of the three development mixes at both
  scales before and after, every gap line against the manifest;
  `regress` at both scales identical; experiment 0234 (with what 45 s
  lists that 60 s would not)
- [ ] 4. The roadmap's "Time without a confident match", the printing
  note's status line, README's example
- [ ] 5. Read-only review by a separate agent; findings fixed
- [ ] 6. Notes ("Session 55", with part 2's handoff), status, checklist;
  final checks (`scripts/check.sh`, both regressions with the final
  build, the verdicts file and the clusters file's digest unchanged,
  `git status` clean)
