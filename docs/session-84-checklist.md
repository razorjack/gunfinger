# Session 84 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-84.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Digests of `docs/pair-verdicts.txt`, the NAS clusters file and the
  answer log in `work/s84/listening-digests-start.txt`
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `work/bin/s84-start` built; start regressions skipped (byte-identical
  to `s83-final`, store revisions unchanged)

## Steps

- [x] 1. Registration committed (in `docs/brief-84.md`, `0d2c4da`)
- [x] 2. The option (`--gap-rungs`, `--gap-key-lock`, `mixes --playing`), tested; off: both regressions identical (`--strict`)
- [x] 3. Runs at NAS scale (262 tracks where cheap); every scored run exit 0
- [x] 4. Experiments 0292 (targeted extra rungs), 0293 (key lock over the gaps)
- [x] 5. Ladder-edge note, roadmap items and active queue (the citation), decisions queue item 9
- [x] 6. Review by a subagent; fixes
- [x] 7. Wrap-up: notes, status, final checks (check.sh green; both regressions identical with `--strict`; digests unchanged), commit
