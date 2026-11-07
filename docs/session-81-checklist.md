# Session 81 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-81.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Digests of `docs/pair-verdicts.txt`, the NAS clusters file and the
  answer log in `work/s81/listening-digests-start.txt`
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `work/bin/s81-start` built; start regressions skipped (byte-identical
  to `s80-final`, store revisions unchanged)

## Steps

- [x] 1. Registration committed (in `docs/brief-81.md`)
- [x] 2. Variants (`--second-pass-fit iterated-short`, `--set-aside-share`,
  `grid --other-sample`), tested; `regress session-78-linux --strict`
  identical with them off
- [x] 3. Runs (262 tracks; NAS-scale sweep; the NAS-scale grid added)
- [x] 4. Experiments 0285, 0286, 0287
- [x] 5. Note, roadmap, decisions queue item 1
- [x] 6. Review by a subagent; fixes (`ae03fb8`, `8f177b0`)
- [x] 7. Wrap-up: notes, status, final checks (check.sh green after `94d1daf`; both regressions strict-identical), commit
