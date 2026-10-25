# Session 47 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-47.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] Digests of `docs/pair-verdicts.txt`, `work/audition/answers.jsonl`
  and the clusters file in `work/s47/listening-digests-start.txt`
- [~] Start build `work/bin/s47-start/`; `regress session-32-linux` and
  `regress session-33-linux-nas` identical

## Steps

- [x] 1. The far pieces of session 40's pack, and which were `same`
  (brief, "Decisions", 02:10)
- [x] 2. Design in the brief, before code
- [ ] 3. The `O` key and the two lane fields, with Node and Rust tests;
  a check in a scratch server on a pack pair with a far piece, in each
  state
- [ ] 4. `listening_pack.py render` resolves `--corpus` and `--nas`,
  with a test
- [ ] 5. ADR 0016, `docs/audition-questions.md`, the page's help
- [ ] 6. Read-only review by a subagent; findings fixed
- [ ] 7. Wrap-up: notes "Session 47", status, checklist; final checks
  (`scripts/check.sh`, both regressions, verdicts, answer log and
  clusters digests, `git status` clean)
