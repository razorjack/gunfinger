# Session 23 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-23.md`
for scope and rules.

## Setup

- [x] The diff was 17 new verdict lines and the status entry;
  committed as `d39fdca`
- [x] Brief, checklist; `CLAUDE.md` points to them (`d9b3b6a`)
- [x] The index run exited (02:23-02:25; waited for, never
  signalled): 39,354 records, revision `d71ba5333cff9a34`; a status
  line; `doctor` (27 audio files not indexed yet)
- [x] Release build in `work/bin/s23-start/` (byte-identical to
  `s22-final`); `baseline session-23-start` (the same reports as
  `session-22-start`)

## Steps

- [x] 2. The NAS reports copied to `work/s23/before/`; `map-library`;
  `clusters --from-peaks` (full, every verdict; stopped on the Sonar
  chain, its pairs clustered again with `--cut-sparsest`); what changed;
  experiment 0120
- [x] 3. Re-scored offline with old and new clusters (the old reproduce
  the published numbers): the development mixes' NAS scans, 0111, 0112,
  0116, 0118; experiment 0121
- [x] 4a. The three development mixes at the new revision (scans and
  leave-outs, `--verify`)
- [x] 4b. `shared` on the development half at the new revision, and
  its holder lookups
- [-] 4c. `shared-mixes` s21, then the rips plan: shrunk (brief,
  09:25): the 36 rendered mixes would have ended near noon; the three
  development mixes ran alone; the rips plan dropped. Experiment 0122
- [x] 5. The ambiguity note: definition, wording and criteria written
  first; counts (criterion 1 fails, 2 passes); flagged plays with their
  categories; the three tracklists (6 of 40 entries noted, all right;
  the noted lines in the notes); experiment 0123
- [x] 6a. Questions in `work/audition/s23/` (Sonar upload, Kemal remix,
  borderline pairs): `01-s23-pack.json`, 6 questions, checked on a
  temporary server, never answered
- [x] 6b. A proposed second panel, drawn and frozen, not searched
  (`docs/panels/shared-material-2-seed-2026.json`, done before step 4)

## Wrap-up

- [x] 7. Read-only review by a subagent: pass 1 at 09:05 (14 findings,
  fixed), pass 2 at 11:12 (findings fixed; the `holders` check for
  unindexed left-out names reverted and left open, see the notes)
- [ ] Notes "Session 23", status, checklist, roadmap
- [ ] `regress session-23-start` identical; `scripts/check.sh` green
  with both toolchains
- [x] The NAS store's revision at the end equals the one after the
  index run (39,354 records, `d71ba5333cff9a34`, 11:37)
- [ ] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [ ] Commit; the final message
