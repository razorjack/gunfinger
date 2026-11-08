# Session 88 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-88.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Digests of `docs/pair-verdicts.txt`, the NAS clusters file and the
  answer log in `work/s88/listening-digests-start.txt`
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `work/bin/s88-start` built; start regressions skipped (byte-identical to `s87-final`, store revisions unchanged)

## Steps

- [x] 1. Pre-registration committed before the scored runs (`df1ce0e`)
- [x] 2. The variant, opt-in, tested (`fa681d2`); `regress --strict` identical with it off at both scales
- [x] 3. The runs at NAS scale and at 262 tracks
- [x] 4. Experiment 0298 (`d60d618`)
- [x] 5. Roadmap (item and active queue item 11), decisions queue item 11 (`d60d618`)
- [x] 6. Review by a subagent; fixes (`3c9c788`)
- [x] 7. Wrap-up: notes, status, final checks (check.sh green; both regressions identical with `--strict`; digests unchanged), commit
