# Session 16 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-16.md`
for scope and rules.

## Setup

- [x] `git status` clean at `5b09bc3`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Release build in `work/bin/s16-start/` (both binaries byte-identical
  to `work/bin/s15-final/`); `baseline session-16-start` saved from the
  standard reports of session 15's final `regress`, made by that build
- [x] The NAS store at the start: 33,596 records, revision
  `f9f9bc706195a71b`, as expected (`work/s16/store-revision-start.txt`)

## Steps

- [x] 2. The principle recorded: ADR 0017 (scope, priorities, the
  default output); roadmap items "Report edits", "The Track/AudioAsset
  model" out of scope, "Titles of mislabelled files" flags and never
  chooses, "Database" no longer waits on them; AGENTS.md paragraph "The
  product's scope"; ADR 0016's status, the README's intro and
  `questions detections`' module note point to ADR 0017
- [x] 3. One tracklist entry per track: the cause confirmed (the upload's
  track start 41.7 s from the rips'); plays over the same passage with
  equal artist and title tags grouped (`same_tagged_track`, chains joined
  in any order); the rule in ADR 0017, a note in ADR 0006, the README;
  37 reports measured (experiment 0095): only the Skektics entry changes
  in reports of the current library; 7 tests
- [ ] 4. The tracklist as the default of `identify` and `show`; `-f human`
  kept; several recordings, `--quiet`, colour, no plays, `--save-dir`
  checked; README, help, docs, tests; no script parses the default
- [ ] 5. A stall timeout on FFmpeg (and FFprobe): N chosen and
  documented; `index` fails the file for this run only; `identify` fails
  with the message; the listening page's decoding too; tests through a
  seam
- [ ] 6. `rust-version` 1.89; the answer log and the verdicts file locked
  against a second server; decided, documented, tested
- [ ] 7. Read-only review of the session's code by a subagent; fixes

## Wrap-up

- [ ] Notes "Session 16", status, roadmap
- [ ] `regress session-16-start` identical; `scripts/check.sh` green
- [ ] The NAS store's revision at the end
- [ ] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [ ] Commit; the final message
