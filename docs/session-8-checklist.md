# Session 8 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-8.md`
for scope and rules.

## Setup

- [x] `git status` clean at `e66d9e7`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [ ] Release build (`work/bin/s8-start/`); `baseline session-8-start`
- [ ] The NAS store at the start: counts, digest, revision
  (`d428ee9585936326` expected)
- [x] `doctor` and `prune` without `--yes` checked in the code: no
  writes to the store (`docs/brief-8.md`)

## The feature

- [ ] 1. Matcher (`library/ignore.rs`) and its tests: anchored and
  unanchored, `*` within a component, `**`, `?`, trailing `/`, literal
  brackets, `&`, `%2F`, full-width `：`, comments, `!` and backslash
  rejected with file, line and advice
- [ ] 2. The scan reads `.gunfingerignore` at the root and keeps the
  left-out audio files per pattern; tests with an ignored folder, an
  ignored file and no ignore file
- [ ] 3. CLI: `survey` tells ignored from gone; `doctor` (file read or
  not, files per pattern, warning for a pattern that leaves out nothing,
  records and notes counted apart, the command); `prune` lists the two
  kinds apart; `index` says what was left out; end-to-end test
- [ ] 4. ADR 0009; README (mixed CDs as the example, store-only runs);
  AGENTS.md (`library`); roadmap item done, verdict-link step kept
- [ ] `scripts/check.sh` green; `regress session-8-start` identical

## The NAS

- [ ] `/Volumes/atlas/Music/dnb/.gunfingerignore` written, the four
  lines exactly; nothing else on the NAS touched
- [ ] `doctor --config ~/.config/gunfinger/nas-dnb.toml`: 20 and 13
  audio files expected
- [ ] `prune --config ~/.config/gunfinger/nas-dnb.toml` without `--yes`:
  ignored files' records and notes, and anything else, apart

## Wrap-up

- [ ] Notes ("Session 8"), status, checklist
- [ ] The NAS store's revision at the end, unchanged
- [ ] Commit; the message to the owner with the prune command
