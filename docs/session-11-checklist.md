# Session 11 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-11.md`
for scope and rules.

## Setup

- [x] The owner's ledger section: `scripts/check.sh` green (2:23),
  committed alone (`51d6f4c`)
- [x] Release build in `work/bin/s11-start/`; `baseline
  session-11-start`
- [x] The NAS store at the start: 33,596 `.peaks`, 33,596 `.tags`, 1,582
  `.skip` (11 failed, 124 too short, 1,447 too long); revision
  `f9f9bc706195a71b`, as expected; census `work/s11/census-start.json`
  (3,523.4 h)
- [x] `doctor --config ~/.config/gunfinger/nas-dnb.toml` (14:46 wall):
  39 patterns, 603 audio files left out, 35,178 audio files = 33,596
  records + 1,582 passed over; no records of ignored or gone files, none
  waiting
- [x] The laptop's disk: 62 GiB free
- [x] The recall panel's two development sources without a record are
  named and left out (`e446040`); regress identical
- [~] Brief, checklist; `CLAUDE.md` points to them; status "After
  session 10"

## Items

- [~] 2. Session 10's reports kept in `work/s11/before/`; `map-library`
  (2.4 s); the NAS clusters search (started 19:03:14 under `screen`,
  session `s11-clusters`, `work/s11/clusters.sh`)
- [ ] 3a. Saved index: width audit for 65,536 assets; ADR; format with a
  header; rebuilt and replaced atomically when stale or unreadable;
  `identify` and `explain` with the NAS configuration use it; tests
  (stale header, truncated file, changed store); regress identical
- [ ] 3a acceptance (idle machine, after item 2): the 2003 mix at NAS
  scale with a rebuilt and a saved index, identical; rebuild, first run,
  cold and warm loads timed; file size; experiment
- [ ] 3b. Conflicting titles: the other titles of a passage shown without
  choosing one; the 2003 mix's track 2; detections unchanged; regress
- [ ] 3c. `gunfinger-eval pair-review`: pack items, short sheet, clips
  through the audio output, keys, verdict lines appended, skips judged
  items, prints `clusters --reuse-pairs`; tests with a fake output and
  a temporary verdicts file; regress
- [ ] 4. Clusters experiment against 0066; the listening pack rebuilt
  (`pair`, clips)
- [ ] 5. The NAS baseline (scan, leave-outs 3 and 11, sweeps 2026-2029,
  `calibrate`, the 2003 mix at both sizes with leave-out 3); against
  0067; calibration register
- [ ] 6. The recall panel's development half; against 0068
- [ ] 7. Evaluation 4: ledger entry with the prediction, one run,
  verbatim result, comparison, "Evaluations used: 4 of 5"
- [ ] 8a. Mixed discs among the folders indexed since 0065: a list for
  the owner's ear
- [ ] 8b. Where a NAS `identify` spends its time with the saved index
- [ ] 8c. Shared material at NAS scale

## Wrap-up

- [ ] Experiments; notes "Session 11" (the pack and the review command,
  the mixed-disc list first); evaluation 4; status; checklist;
  calibration register; roadmap
- [ ] The NAS store's revision at the end; new audio files counted
- [ ] Commit; the message to the owner
