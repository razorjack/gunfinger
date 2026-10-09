# Session 9 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-9.md`
for scope and rules.

## Setup

- [x] `git status` clean at `9fda7db`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Release build (`work/bin/s9-start/`); `baseline session-9-start`
- [x] The NAS store at the start: records, skip notes, revision
  (`b8bb402f0ba23761` expected), census, the longest record

## Items

- [x] 1. `stats` with the NAS configuration stops at 32,768 assets: the
  message and the time to fail
- [x] 2. 16 frame bits: `index.rs`, default track limit 17:00, messages,
  help, README, configuration docs, doctor; tests (round trip, the
  65,536th asset and the next, a record past 17:28); ADR 0010, ADR 0005's
  status, ADR 0007; `scripts/check.sh` green; `regress session-9-start`
  identical (720/720; 87, 93, 79); experiment 0062
- [x] 3. `stats` at 32,905 records; `identify` of the development mix
  (0050's command) and of `razorjack-2003-03-29` (0060's): tracks
  against 0050 and 0060, files from the new folders; wall, CPU, memory
  (experiment 0063)
- [ ] 4. `doctor` over Ethernet (listing time, not indexed, failed, too
  short, too long, gone); ~300 files from new folders into scratch
  stores (Mbit/s, s per file, a full re-index estimate); peaks against
  the NAS store's; the NAS store's revision; failed files with `ffprobe`
- [ ] 5. Too-long files listed (path, length, folder) in `work/`; DJ-mix
  candidates; new folders that look like mixed CDs

## Wrap-up

- [ ] Experiments; notes ("Session 9", decisions first); status;
  calibration register; roadmap ("More than 32,768 assets")
- [ ] The NAS store's revision at the end
- [ ] Commit; the message to the owner
