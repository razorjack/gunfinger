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
- [x] 4. The tracklist as the default of `identify` and `show`; `-f human`
  kept without an alias (ADR 0017); several recordings (headings), `--quiet`
  (stderr only), colour (the tracklist stays plain), no plays (`no tracks
  found`), `--save-dir` checked; README, help, ADR 0017; an end-to-end test
  of the default (an upload with 20 s more lead-in and tags differing in
  case: one entry); the batch test reads the tracklist; no script or
  harness code parses the default (the harness calls the core)
- [x] 5. A stall timeout on FFmpeg (and FFprobe): 120 s without output
  (`decode::STALL`, ADR 0018), also on FFprobe, the wait for the exit and
  the header read; the hung child is killed and never waited for
  (`decode::watch`); `index` fails the file for this run only (not
  remembered; the next run tries it again); `identify` fails the recording
  with the message (checked by hand with a stand-in that hangs: exit 1
  after 120 s, the stand-in killed); the listening page decodes through
  the same watcher; tests through `Decoder` (stand-in scripts, a 0.5 s
  timeout, `PATH` untouched): 5 in `watch`, 1 in `tests/stalls.rs`, 1 on
  the page
- [x] 6. `rust-version` 1.89 (README's prerequisites); the lints it turns
  on fixed (`as_chunks`, a `let` chain, `is_multiple_of`); a server holds
  the answer log's lock (`AnswerBook::lock_log`) and a second server on
  the same log exits with a message instead of waiting (decided: the
  running server is the one to answer in); verdicts checked and appended
  under the verdicts file's lock (`clusters::VerdictsWriter`), which undo
  and `pair-review` take too, waiting for each other; ADR 0016 "One
  writer", `docs/audition-questions.md`, AGENTS.md; 3 tests
- [x] 7. Read-only review of the session's code by a subagent; fixed:
  `pair-review` reloads the verdicts under the lock; thread start
  failures are errors; stderr counts as activity; the stall tests give
  the real programs 5 s and cover a stalled header read; titles disagree
  over the whole entry (no stored report changes); untagged names read
  once; wording in ADRs 0016-0018, the page's note and `index`'s message

## Wrap-up

- [ ] Notes "Session 16", status, roadmap
- [ ] `regress session-16-start` identical; `scripts/check.sh` green
- [ ] The NAS store's revision at the end
- [ ] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [ ] Commit; the final message
