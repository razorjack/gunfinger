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
- [~] 2. Harness: `clusters --library-wide` (`58c53c1`), `break-search --passage-ladder` (`5ab4e2a`), `questions breaks` (`c4a01f0`); tested; both regressions identical
- [~] 3. Runs: the clustering (started 06:17), break search
- [ ] 4. Experiments 0295 (clustering) and 0296 (break search as retrieval, the null)
- [ ] 5. The listening pack (not answered)
- [ ] 6. Break-search note, roadmap items, decisions queue item 4
- [ ] 7. Review by a subagent; fixes
- [ ] 8. Wrap-up: notes, status, final checks, commit
