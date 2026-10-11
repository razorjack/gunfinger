# Session 11 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-11.md`
for scope and rules.

## Setup

- [x] The new ledger section: `scripts/check.sh` green (2:23),
  committed alone (`4678fbd`)
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
- [x] The recall panel's two development sources without a record are
  named and left out (`aa0862a`); regress identical
- [x] Brief, checklist; `CLAUDE.md` points to them; status "After
  session 10" (`8249084`)

## Items

- [x] 2. Session 10's reports kept in `work/s11/before/`; `map-library`
  (2.4 s); the NAS clusters search (19:03:14-03:08:44 under `screen`,
  `work/s11/clusters.sh`; 29,130 s wall (not a speed measurement: the machine had other load), 648
  queries, 267 clusters)
- [x] 3a. Saved index: width audit for 65,536 assets; ADR 0011; format
  with a header; rebuilt and replaced atomically when stale or
  unreadable; `identify`, `explain` and `stats` use it; tests (stale
  header, truncated file, changed store); checks pass (`dbce7d9`);
  regress identical (03:14); review fixes (`84fe0e5`)
- [x] 3a acceptance: the 2003 mix at NAS scale built, built and saved,
  loaded warm and after item 5 (likely cold): byte-identical reports;
  118.1 s, 167.2 s, 3.0 s and 2.8 s to the index; 3,795,332,786 bytes;
  (not a speed measurement: the machine had other load; experiment 0072)
- [x] 3b. Conflicting titles: the other titles of a passage shown without
  choosing one; unit tests, clippy (`3085abb`); the 2003 mix's track 2
  names the three "Specialist" files; regress identical
- [x] 3c. `gunfinger-eval pair-review`: pack items, short sheet, clips
  through `ffplay`, keys, verdict lines appended, skips judged items,
  prints `clusters --reuse-pairs`; tests with a fake output and a
  temporary verdicts file (`5e79413`); regress identical; always prints
  `--cut-sparsest` (`84fe0e5`)
- [x] 4. Clusters experiment against 0066 (experiment 0071); the
  listening pack rebuilt: 23 items (21 kept, 9 dropped with a pruned
  file, 2 new, measured by `pair` and rendered)
- [x] 5. The NAS baseline (03:38-06:19): as 0067; the 24 wrong sweep
  answers gone; 3.00× (experiment 0073); calibration register
- [x] 6. The recall panel's development half (06:23-06:51): 522/522, 9
  wrong (DYkast), 0 held out confident (experiment 0074)
- [x] 7. Evaluation 4 (ledger entry `fab29bb` before; run 06:52-07:04,
  once): 21/23, 2 wrong (the Mind Overload edit, predicted; The Nine's
  VIP, not predicted), 2 possible; "Evaluations used: 4 of 5" (`34b10ba`)
- [x] 8a. Mixed discs among the folders indexed since 0065: none of the
  1,252 new records is a release folder (YouTube uploads); by disc, no
  disc scores like a mixed CD; seven units in between for the ear
  (experiment 0070, `0b8681f`)
- [~] 8b. Where a NAS `identify` spends its time with the saved index
- [~] 8c. Shared material at NAS scale

## Wrap-up

- [ ] Experiments; notes "Session 11" (the pack and the review command,
  the mixed-disc list first); evaluation 4; status; checklist;
  calibration register; roadmap
- [ ] The NAS store's revision at the end; new audio files counted
- [ ] Commit; the final message
