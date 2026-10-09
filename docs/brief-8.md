# Session 8 brief: a library ignore file

Written by the agent for itself on 2026-10-09, so the plan survives context
compaction. Work autonomously and never stop to ask. When something is
ambiguous, take the conservative option, write the decision down and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-8-checklist.md` (progress).
3. `docs/notes-for-owner.md` ("After session 7" at the end).
4. `docs/status.md` (newest entries at the bottom).
5. `docs/roadmap.md` ("Library ignore file").

`docs/brief-2.md` to `docs/brief-7.md` hold rules that still apply unless
this brief changes them.

## The owner's prompt, verbatim

```text
Start session 8 of Gunfinger: a library ignore file (`.gunfingerignore`).
Build only this feature, then write the owner's first ignore file on the
NAS. Work autonomously and never stop to ask. When something is
ambiguous, take the conservative option, write the decision down and
continue. Expected length: about 2 hours.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it),
   docs/brief-7.md (brief-2 to brief-6 hold rules that still apply
   unless this prompt changes them), docs/notes-for-owner.md ("After
   session 7" at the end, especially "Mixed-CD tracks in the library"),
   docs/status.md (newest entries at the bottom) and docs/roadmap.md
   ("Library ignore file"). Then the code this touches: gunfinger-core's
   `library` and the CLI's `catalog`, `survey`, `doctor`, `prune` and
   `index`.
2. `git status` should be clean at e66d9e7. If it is not, leave those
   changes alone and list them in the notes.
3. Write docs/brief-8.md (this prompt verbatim, the state at the start,
   the plan) and docs/session-8-checklist.md. Point CLAUDE.md's reading
   list at them; keep brief-7 among the older briefs. Commit.
4. Build with --release. Run `gunfinger-eval baseline session-8-start`.
   Record the NAS peak store's revision
   (~/.local/share/gunfinger/nas-dnb-peaks; d428ee9585936326 expected).

Rules for this session:
- The NAS (/Volumes/atlas/Music/dnb) may be read. Exactly one file may
  be written there: /Volumes/atlas/Music/dnb/.gunfingerignore, which
  does not exist yet. Nothing else on the NAS is created, changed,
  moved or deleted.
- Do not prune. Never run `prune --yes` or `prune --force`, and never
  run `index` with the NAS configuration (it would also decode files
  added to the NAS since indexing). `doctor` and `prune` without `--yes`
  only list: confirm in the code that neither writes to the store
  before running them against the NAS. The NAS peak store stays
  unchanged; check its revision again at the end.
- `corpus/` stays read-only: no .gunfingerignore in corpus/library.
  Tests build their libraries in temporary directories.
- Detection does not change: `regress session-8-start` must be
  identical.
- Out of scope, for a later session: the clusters' verdict links,
  clusters runs, NAS references in manifests and the 2003 mix in the
  protocol.
- Commit after each step. Check `git status` before every commit and
  never use `git add -A`.

The feature (record the design in docs/adr/0009-library-ignore-file.md):
- The file: `.gunfingerignore` at the library root (the configuration's
  `library`, or `--library`). No file means nothing is ignored. Only the
  root file is read; there are no nested ignore files.
- Syntax, a subset of .gitignore: one pattern per line; blank lines and
  lines starting with `#` are skipped; trailing spaces are trimmed; `*`
  matches within one path component, `**` across components, `?` one
  character; a trailing `/` matches folders only; a pattern with a `/`
  at its start or in its middle is relative to the root, otherwise it
  matches a name at any depth. Matching is case-sensitive, like the
  store's paths. Square brackets are literal, not character classes:
  the owner's folder names are full of them (`[Virus]`, `[TECH012]`).
  A leading `!` (negation) and a backslash are errors; every error
  names the file and line and says what to write instead. Write a small
  matcher of its own; add no dependency unless that is clearly simpler.
- The effect: the library scan leaves ignored files and folders out and
  keeps, per pattern, the relative paths of the audio files it left
  out. Every command that scans the library (`index`, `identify` and
  `explain` with a library, `doctor`, `prune`, the harness) then treats
  them as files not in the library, so their peak records and skip
  notes are what `prune --yes` deletes. Store-only runs (`--store-only`,
  `--other-peaks-dir`) never read the library and keep seeing ignored
  files until they are pruned; say so in the README and the ADR.
- `doctor` reports whether a .gunfingerignore was read, how many audio
  files each pattern leaves out, a warning for a pattern that leaves out
  nothing (a typo or a renamed folder), and the records and notes of
  ignored files counted apart from those of files gone from the
  library, with the command that removes them. `prune` without `--yes`
  lists the two kinds apart as well.
- The patterns are the owner's configuration, not logic derived from
  file names (AGENTS.md); the ADR says so.
- Tests: the matcher (anchored and unanchored patterns, `*` not crossing
  `/`, `**`, `?`, a trailing `/`, literal brackets, names with `&`,
  `%2F` and a full-width `：`, comments, `!` and backslash rejected); a
  scan of a temporary library with an ignored folder, an ignored file,
  and no ignore file; survey, doctor and prune telling ignored files
  from missing ones. scripts/check.sh stays green.
- Documents: the README (how to ignore folders, with mixed CDs as the
  example), AGENTS.md (the `library` module), and the roadmap item
  marked done, keeping the open step on verdict links to ignored files.

Then, on the NAS:
1. Write /Volumes/atlas/Music/dnb/.gunfingerignore. Dangerous Drums is
   ignored with both discs; of The Creeps only CD2 is ignored, and CD1
   stays. The file holds exactly these four lines:

   # Mixed CDs: each track already blends into the next one, so its
   # matches join unrelated recordings. Listed by the owner, 2026-10-09.
   /Underfire UDFRCD003 - Dangerous Drums Volume 2 (2000)/
   /2001 - VRSCD003 - Ed Rush & Optical - The Creeps [Virus]/CD2/

2. Run `target/release/gunfinger doctor --config
   ~/.config/gunfinger/nas-dnb.toml` (listing the NAS takes minutes),
   then `prune` with the same configuration and no `--yes`. Expected:
   the first pattern leaves out 20 audio files (CD1 and CD2, 10 each)
   and the second 13. Report the records and notes of ignored files,
   and separately anything else prune would delete (files gone from the
   NAS since indexing).

Wrap-up:
- docs/notes-for-owner.md: a short "Session 8" section at the end (what
  was built, the file on the NAS, the numbers from doctor and prune).
  Then docs/status.md and the checklist. Commit.
- Last, print for the owner: what was built and how it was verified;
  the exact command to prune, checked against `gunfinger prune --help`
  and meant to run from the repository root, such as
  `target/release/gunfinger prune --config ~/.config/gunfinger/nas-dnb.toml --yes`;
  what that command will delete (the ignored files' records and notes,
  and anything else, counted apart); and that pruning changes the NAS
  store's revision, so the next session starts from a new baseline.
```

## State at the start of session 8

- 17:03. `git status` clean at `e66d9e7` (the branch is one commit ahead
  of `origin/master`; nothing is pushed this session). Experiments
  0001-0061; ADRs 0001-0008. Test set: 3 of 5 evaluations used.
- The NAS is mounted. `/Volumes/atlas/Music/dnb/.gunfingerignore` does
  not exist. Listed read-only: `Underfire UDFRCD003 - Dangerous Drums
  Volume 2 (2000)/` holds `CD1/` and `CD2/` with 10 mp3 each;
  `2001 - VRSCD003 - Ed Rush & Optical - The Creeps [Virus]/CD2/` holds
  13 mp3 (names such as `cd2%2F10_-_Ed_Rush_%26_Optical_-_Bleep_Bleep.mp3`),
  `CD1/` 11. Both release folders also hold a `.DS_Store`.
- The last code change is `e9562ea` (harness only); `work/reports/`
  holds the standard reports written by session 7's `regress
  session-7-start` (2026-10-08 17:51-17:53).
- What the code does today (read before planning): `Library::scan` in
  `gunfinger-core/src/library.rs` walks the root, passes over hidden
  files and folders and non-audio extensions, and returns the assets and
  the counts of what it passed over. The CLI's `catalog::scan_library`
  wraps it for `index`, `doctor` and `prune`; `catalog::Indexable::find`
  uses it for `identify`, `explain` and `stats` and falls back to the
  store's records when the library cannot be read. `survey` compares the
  store with the library; its `orphans` are the records and notes of
  files not in the library, which `prune --yes` deletes. The harness
  scans `corpus/library` (`Paths::library`) and, with
  `--second-library`, another library (`SecondLibrary::open`);
  `--other-peaks-dir` reads the store alone (`SecondLibrary::from_store`).
- `doctor` and `prune` without `--yes` do not write to the store, checked
  in the code: `PeakStore::open` calls `fs::create_dir_all`, which
  changes nothing when the directory exists; `check_library` and
  `library` read `library.txt`; `PeakStore::survey` opens each file to
  read its header; `prune` calls `PeakStore::remove` only after the
  `if !request.yes { ... return }` branch. `claim_library`, the only
  writer of `library.txt`, is called by `index` alone.

## Plan, with times

Start 17:03; about 2 hours.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| Setup | brief, checklist, `CLAUDE.md`, commit; release build (`work/bin/s8-start/`), `baseline session-8-start`, `regress session-8-start` with the start build as a check, the NAS store's revision | 0:15 | 17:20 |
| 1 | the matcher, `gunfinger-core/src/library/ignore.rs`, with its tests | 0:25 | 17:45 |
| 2 | the scan reads the root's `.gunfingerignore` and keeps the left-out audio files per pattern; scan tests | 0:15 | 18:00 |
| 3 | CLI: `survey` tells ignored files from gone ones; `doctor`, `prune`, `index`; an invalid ignore file stops `identify`, `explain` and `stats` instead of falling back to the store; end-to-end test | 0:30 | 18:30 |
| 4 | ADR 0009, README, AGENTS.md, roadmap; `scripts/check.sh`; `regress session-8-start`; commit | 0:15 | 18:45 |
| 5 | the NAS: write the ignore file, `doctor`, `prune` without `--yes` | 0:15 | 19:00 |
| 6 | wrap-up: notes, status, checklist, the NAS store's revision, commit, the message to the owner | 0:10 | 19:10 |

Conservative decisions taken up front:

- An ignore file that cannot be read or parsed stops every command that
  scans the library, with the file, the line and what to write instead.
  `identify`, `explain` and `stats` do not fall back to the store's
  records then: that fallback is for a library that cannot be listed
  (an unmounted share), and searching the store would silently include
  the files the owner meant to leave out.
- As in `.gitignore`, only trailing spaces are trimmed; leading spaces
  belong to the pattern. `**` spans folders only as a whole component
  (`**/`, `/**/`, `/**`); inside a name, as in `a**b`, it is `*`, as in
  Git. A pattern made of slashes alone, or with `//` in it, is an error.
- Hidden files and folders are passed over before the patterns are
  tried, so a pattern counts only audio files the scan would otherwise
  have kept. A file left out by several patterns is counted under one:
  the first in the file that leaves out its outermost ignored folder, or
  else the first that matches the file. A pattern whose files another
  pattern already leaves out therefore counts nothing, and `doctor`'s
  warning says so as a possible reason.
- Non-audio files inside ignored folders are counted among the files
  passed over, by the same reasons as elsewhere (`extension .nfo`): the
  scan would pass over them anyway, and one rule for every folder keeps
  the code simpler. (Changed while building step 2; the first plan left
  them uncounted.)
- Records and notes of ignored files and of gone files are both
  warnings in `doctor`, with `gunfinger prune --yes` as the command.
- The harness's `--second-library` follows the ignore file, because it
  scans the library; `--other-peaks-dir` does not, as the prompt says.
