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

- [ ] 2. The NAS reports copied to `work/s23/before/`; `map-library`;
  `clusters --from-peaks` (full, every verdict); what changed;
  experiment 0120
- [ ] 3. Re-scored offline with old and new clusters (the old reproduce
  the published numbers): the development mixes' NAS scans, 0111, 0112,
  0116, 0118; experiment 0121
- [ ] 4a. The three development mixes at the new revision
- [ ] 4b. `shared` on the development half at the new revision
- [ ] 4c. `shared-mixes` s21, then the rips plan; experiment 0122
- [ ] 5. The ambiguity note: definition, wording and criteria written
  first; counts; tracklists; experiment 0123
- [ ] 6a. Questions in `work/audition/s23/` (Sonar upload, Kemal remix,
  borderline pairs)
- [ ] 6b. A proposed second panel, drawn and frozen, not searched

## Wrap-up

- [ ] 7. Read-only review by a subagent (from 11:45); fixes
- [ ] Notes "Session 23", status, checklist, roadmap
- [ ] `regress session-23-start` identical; `scripts/check.sh` green
  with both toolchains
- [ ] The NAS store's revision at the end equals the one after the
  index run
- [ ] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [ ] Commit; the final message
