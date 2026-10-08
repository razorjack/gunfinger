# Session 7 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-7.md`
for scope and rules.

## Setup

- [x] The reviewed work from after session 6 committed after
  `scripts/check.sh` (cc81599)
- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Release build (`work/bin/s7-start/`); `baseline session-7-start`
  (copies session 6's reports of 04:20-04:22; `regress session-7-start`
  with the HEAD build checks them)
- [x] The NAS store at the start: 27,042 `.peaks`, 27,042 `.tags`, 1,396
  `.skip`, `library.txt`; digest `bf35ea64...e9ae4f5`; revision
  `d428ee9585936326`, as expected
- [-] Mixotic download: all three Google Drive links answer with a
  sign-in page; nothing arrived (`docs/brief-7.md`)
- [x] Test-set evaluation 3: one scan of `ed-rush-optical-essential-mix`
  under the default matcher, ledger entry, nothing changed (14/16, 0
  wrong, pass; Sick Note possible at 233 hits as predicted; 3 of 5 used)

## Items

- [x] 1. Peak verifier (harness diagnostic): reference peaks in the
  aligned span found in the query within a tolerance; found, share,
  support over 1 s slices; chance level from shifted alignments
  - [x] tolerance chosen from at most three settings on the development
    scan and sweep 2026, then frozen: 1×1 (experiment 0054)
  - [x] seeds 2027-2029, leave-outs 3 and 11, `robust --only combined`,
    `mixes --count 12`, `grid` (experiment 0056)
  - [x] three groups: identifying (weakest), unrelated false, related
    false; separation at equal hits (0056: no better than hits)
  - [x] NAS scale: development scan and sweep 2027; the listening
    list's four pairs apart and unlabelled (experiment 0058)
  - [x] proposal in the notes (no verifier rule); no level changed
- [x] 2. Shared-material scenarios (Clockwork ~ its remix at 20:22;
  China Cup ~ The Nine): shared passages from `pair`; a. passage alone
  10/20/30 s; b. looped 60/120/240 s; c. into distinctive material
  10/20/30 s; d. played recording and rips left out, and the positive
  control; hits, windows, level, verifier measures (experiment 0055)
- [~] 3. Relative speed at NAS scale: panels' indexed excerpts with a
  non-identical rip, source and identical copies left out; recall
  against relative speed, wrong answers; again with `--extra-rungs 3`;
  NAS clusters with members more than 4% apart (counted: 7; sweeps
  queued after item 1's NAS runs)
- [ ] 4. Wrap-up: experiments, session 7 summary in the notes, roadmap,
  calibration if moved, status, checklist, NAS store revision, gate,
  commit
- [x] 5. (If time remains) supported time for joins: `pair` prints the
  seconds of the shorter file that hold aligned hits (e9562ea); 4 sparse
  joins, 28 borderline pairs, 8 controls at NAS scale (experiment 0057);
  findings in the listening list after session 7
