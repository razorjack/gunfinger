# Session 87 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-87.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Digests of `docs/pair-verdicts.txt`, the NAS clusters file and the
  answer log in `work/s87/listening-digests-start.txt`
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `work/bin/s87-start` built; start regressions skipped (byte-identical to `s86-final`, store revisions unchanged)

## Steps

- [x] 1. Design in the brief (family key, where it prints, split audit)
- [x] 2. Harness: `families` (key and offline scoring), tested (`23bd445`, `bde3443`); both regressions identical with `s87-run` (`--strict`)
- [x] 3. The audit on every panel (no family across a split); the re-reading of past experiments
- [x] 4. Experiment 0297 (`b3741f1`)
- [-] 5. Any family on both sides of a split: none found, nothing to record
- [x] 6. Review by a subagent; fixes (`a0a8512`, `91bbf83`)
- [x] 7. Wrap-up: notes, status, final checks (check.sh green; both regressions identical with `--strict`; digests unchanged), commit
