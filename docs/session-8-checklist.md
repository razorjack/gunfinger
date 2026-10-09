# Session 8 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-8.md`
for scope and rules.

## Setup

- [x] `git status` clean at `e66d9e7`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Release build (`work/bin/s8-start/`); `baseline session-8-start`;
  `regress session-8-start` with the start build: identical
- [x] The NAS store at the start: 27,042 `.peaks`, 27,042 `.tags`, 1,396
  `.skip`; digest `bf35ea64...e9ae4f5`; revision `d428ee9585936326`
- [x] `doctor` and `prune` without `--yes` checked in the code: no
  writes to the store (`docs/brief-8.md`)

## The feature

- [x] 1. Matcher (`library/ignore.rs`) and its tests: anchored and
  unanchored, `*` within a component, `**`, `?`, trailing `/`, literal
  brackets, `&`, `%2F`, full-width `：`, comments, `!` and backslash
  rejected with file, line and advice
- [x] 2. The scan reads `.gunfingerignore` at the root and keeps the
  left-out audio files per pattern; tests with an ignored folder, an
  ignored file and no ignore file
- [x] 3. CLI: `survey` tells ignored from gone; `doctor` (file read or
  not, files per pattern, warning for a pattern that leaves out nothing,
  records and notes counted apart, the command); `prune` lists the two
  kinds apart; `index` says what was left out; end-to-end test
- [x] 4. ADR 0009; README (mixed CDs as the example, store-only runs);
  AGENTS.md (`library`); roadmap item done, verdict-link step kept
- [x] `scripts/check.sh` green; `regress session-8-start` identical
  (720/720; 87, 93, 79 detections)

## The NAS

- [x] `/Volumes/atlas/Music/dnb/.gunfingerignore` written, the four
  lines exactly (254 bytes); the root listing gained only it
- [x] `doctor --config ~/.config/gunfinger/nas-dnb.toml`: 20 and 13
  audio files left out; 33 peak records and 33 tag notes of ignored
  files; none of gone files
- [x] `prune --config ~/.config/gunfinger/nas-dnb.toml` without `--yes`:
  66 files, the ignored files' 33 records and 33 tag notes, nothing else

## Wrap-up

- [x] Notes ("Session 8"), status, checklist
- [x] The NAS store's revision at the end: `d428ee9585936326`, unchanged
- [x] Commit; the message to the owner with the prune command
