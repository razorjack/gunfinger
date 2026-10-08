# Session 7 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-7.md`
for scope and rules.

## Setup

- [x] The reviewed work from after session 6 committed after
  `scripts/check.sh` (cc81599)
- [x] Brief, checklist; `CLAUDE.md` points to them
- [ ] Release build; `baseline session-7-start`
- [ ] The NAS store's revision recorded at the start
  (`d428ee9585936326` expected)
- [-] Mixotic download: all three Google Drive links answer with a
  sign-in page; nothing arrived (`docs/brief-7.md`)
- [ ] Test-set evaluation 3: one scan of `ed-rush-optical-essential-mix`
  under the default matcher, ledger entry, nothing changed

## Items

- [ ] 1. Peak verifier (harness diagnostic): reference peaks in the
  aligned span found in the query within a tolerance; found, share,
  support over 1 s slices; chance level from shifted alignments
  - [ ] tolerance chosen from at most three settings on the development
    scan and sweep 2026, then frozen
  - [ ] seeds 2027-2029, leave-outs 3 and 11, `robust --only combined`,
    `mixes --count 12`, `grid`
  - [ ] three groups: identifying (weakest), unrelated false, related
    false; separation at equal hits
  - [ ] NAS scale: development scan and sweep 2027; the listening
    list's four pairs apart and unlabelled
  - [ ] proposal in the notes; no level changed
- [ ] 2. Shared-material scenarios (Clockwork ~ its remix at 20:22;
  China Cup ~ The Nine): shared passages from `pair`; a. passage alone
  10/20/30 s; b. looped 60/120/240 s; c. into distinctive material
  10/20/30 s; d. played recording and rips left out, and the positive
  control; hits, windows, level, verifier measures
- [ ] 3. Relative speed at NAS scale: panels' indexed excerpts with a
  non-identical rip, source and identical copies left out; recall
  against relative speed, wrong answers; again with `--extra-rungs 3`;
  NAS clusters with members more than 4% apart
- [ ] 4. Wrap-up: experiments, session 7 summary in the notes, roadmap,
  calibration if moved, status, checklist, NAS store revision, gate,
  commit
- [ ] 5. (If time remains) supported time for joins
