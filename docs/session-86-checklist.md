# Session 86 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-86.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Digests of `docs/pair-verdicts.txt`, the NAS clusters file and the
  answer log in `work/s86/listening-digests-start.txt`
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `work/bin/s86-start` built; start regressions skipped (byte-identical to `s85-final`, store revisions unchanged)

## Steps

- [x] 1. Registration committed (in `docs/brief-86.md`, `7909c1e`; 0296's cases `80b57e7`)
- [x] 2. Harness: `clusters --library-wide` (`58c53c1`), `break-search --passage-ladder` (`5ab4e2a`), `questions breaks` (`c4a01f0`); tested; both regressions identical with `s86-run2` (`--strict`)
- [x] 3. Runs: the clustering (35:41, exit 0), break search on both ladders (exit 0)
- [x] 4. Experiments 0295 (clustering) and 0296 (break search as retrieval, the null)
- [x] 5. The listening pack (`work/s86/pack/07-breaks-s86.json`, 32 questions; not answered)
- [x] 6. Break-search note, roadmap items, decisions queue item 4
- [x] 7. Review by a subagent; fixes (`8e59ef8`, `8afcb94`, `5d29677`)
- [x] 8. Wrap-up: notes, status, final checks (check.sh green; both regressions identical with `--strict`; digests unchanged), commit
