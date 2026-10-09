# 0009: A library ignore file

## Status

Accepted (2026-10-09), the owner's proposal and design (session 8).

## Context

A track of a DJ-mixed CD already blends into the next one. Indexed, its
tail matches the next recording, so clustering joins unrelated recordings
and sweeps count wrong answers: the mixed CD's "Synthesis (Remix)" in
experiments 0049-0053. The owner wants to leave such files in the
collection but out of Gunfinger's library, and to say which by hand.
Nothing in the audio or the file names tells a mixed CD apart reliably,
and AGENTS.md forbids logic derived from file names.

## Decision

`.gunfingerignore` at the library root (the configuration's `library`, or
`--library`) lists files and folders to leave out. Only the root file is
read; there are no nested ignore files, and no file means nothing is
ignored. The patterns are the owner's configuration: Gunfinger reads them
as written and derives nothing from file names.

The syntax is a subset of `.gitignore`
(`gunfinger-core/src/library/ignore.rs`, a matcher of its own, no new
dependency):

- One pattern per line. Blank lines and lines starting with `#` are
  skipped; trailing spaces are trimmed, leading ones kept, as in Git.
- `*` matches any characters within one name, `?` one character (not one
  byte), and `**` as a whole name any number of folders: none in the
  middle (`a/**/b` matches `a/b`), at least one at the end (`a/**` matches
  what is inside `a`). Inside a name `**` is `*`, as in Git.
- A trailing `/` matches folders only. A pattern with a `/` at its start or
  in its middle is a path from the root; otherwise it matches a name at
  any depth.
- Matching is case-sensitive, as the store's paths are. Names are compared
  as written: `&`, `%2F` and a full-width `：` are ordinary characters.
- Unlike Git, square brackets are literal, not character classes: the
  owner's folder names are full of them (`[Virus]`, `[TECH012]`).
- A leading `!` (negation), a backslash, a pattern of slashes alone and
  `//` are errors. Each names the file and line and says what to write
  instead. An ignore file that cannot be read or parsed stops every
  command that scans the library; `identify`, `explain` and `stats` do not
  fall back to the store's records then, because those would include the
  files the owner meant to leave out.

`Library::scan` reads the file before it walks the root. Hidden files and
folders are passed over first, as before. A folder a pattern matches is
walked but its audio files are kept apart, per pattern, instead of
becoming assets; a file inside it counts for the first pattern that left
out its outermost ignored folder, any other file for the first pattern
that matches it. `Library::ignore_file` and `Library::ignored` hold the
file and, per pattern, the paths left out.

Every command that scans the library then treats ignored files as files
not in the library: `index` does not decode them, `identify`, `explain`
and `stats` with a library leave them out of the index, and the harness
leaves them out wherever it scans a library (`corpus/library` and
`--second-library`). `survey` labels each record or note of a file not in
the library `Gone` or `Ignored`. `doctor` reports whether the file was
read, how many audio files each pattern leaves out, a warning for a
pattern that leaves out none (a typo, a renamed folder, or files an
earlier pattern already leaves out), and the records and notes of ignored
files apart from those of gone files, with `gunfinger prune --yes` as the
command that deletes them. `prune` lists the two groups apart; `prune
--yes` deletes both, as before.

## Consequences

- Store-only runs never read the library and keep seeing ignored files
  until they are pruned: `identify`, `explain` and `stats` with
  `--store-only` or without a library, and the harness's
  `--other-peaks-dir`, which reads the NAS store alone. `doctor` says so
  when ignored files still have peak records. Ignoring a folder takes
  effect for them only after `prune --yes`.
- Ignoring files changes the library revision of an index built from the
  library, so `identify --save-dir` searches recordings again.
- Un-ignoring a folder makes its files unindexed again if they were
  pruned; `index` decodes them anew.
- An unignored file inside an ignored folder cannot be expressed (no
  negation). To keep one disc of a release, list the other disc, as the
  owner's first file does with The Creeps' CD2.
- The corpus library has no ignore file (`corpus/` is read-only), so
  detection and the standard evaluation are unchanged:
  `regress session-8-start` is identical.
- Still open (roadmap, "More development mixes"): the clusters' verdict
  links must not join files the libraries searched leave out.
