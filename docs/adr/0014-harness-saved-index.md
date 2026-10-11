# 0014: The harness's saved index

## Status

Accepted (2026-10-11, session 12). Amends ADR 0011: its last decision,
"The harness keeps building in memory", is replaced by this one. The
file format, the header check and the atomic replacement of ADR 0011
stand.

## Context

With `--other-peaks-dir`, each harness run builds an index of the corpus
records followed by the other store's current records less the copies
corpus files stand for (`Padding::index`): 80-118 s at 33,596 records
(experiments 0072, 0076, 0087), before every scan, sweep, recall, robust
and `fullest` run. Session 11's NAS baseline built it nine times. The runs
index different sets of files: a leave-out leaves out the referenced
tracks' clusters, a sweep or robust run its seed's held-out clusters
(with `--other-rips`, also their identical copies), recall its held-out
sources. Each of these is the whole index less some files.

A build places each hash's postings asset by asset, in record order, and
each asset's postings in the order its record gives them; asset numbers
are positions in that order. Removing the postings of some assets and
numbering the others again in order therefore gives, posting for posting,
the index built without those records (`Index::without_assets`; a unit
test compares the two). The matcher sets the fullest lists aside after
that, on the index the run searches, as before.

## Decision

- **One saved index per other store**: the corpus records in library
  order, then every current record of the other store less the copies
  corpus files stand for (`second-library/<path>`), with no track length
  range (recorded as "up to 24:00:00"). ADR 0011's format and header: the
  asset table names every file with its size and modification time, so a
  changed corpus record, a changed or new record in the other store, or a
  changed `library-map.json` makes it stale.
- **Where**: `<work>/indexes/library-<store folder>.index`, beside
  `reports/library-<store folder>/`, on the local disk with the rest of
  the harness's derived data (AGENTS.md). Not in
  `~/.cache/gunfinger/indexes/`, which holds the files of `identify`,
  `explain` and `stats` that `doctor` reports. One file per other store
  folder name, replaced through a `.partial` file and a rename when stale
  or unreadable. About as large as the CLI's file of the same store: 3.8 GB
  for the NAS store at 33,596 records.
- **Who uses it**: every run with `--other-peaks-dir` that builds this
  index or one less some files: `scan` (with and without leave-outs),
  `sweep` (also `--other-rips`), `robust`, `recall`, `fullest`, and the
  runs of `regress`. Each loads the file when
  current, otherwise builds the whole index and saves it, then removes the
  files it leaves out. A leave-out that finds no current file builds the
  whole index once instead of the smaller one: the same reading of the
  store, plus the save.
- **Not used**: without `--other-peaks-dir` (the corpus index builds in
  about a second, and `regress` measures that build); with
  `--other-sample` or synthetic copies (other sets of files, built in
  memory, nothing saved); `memory`, which measures the build; `clusters
  --from-peaks` and `pair`, which build other indexes.
- `--saved-index use|rebuild|off` (default `use`), as for `identify`.

## Consequences

- A NAS-scale harness run loads the file in about 2 s instead of building
  for 80 s: the development scan takes 112 s instead of 189 s, with
  identical reports, a leave-out 107 s instead of 180 s (experiment 0087).
  The file holds 33,618 assets in 3.80 GB.
- Memory is unchanged: the file loads into the vectors a build fills, and
  the left-out files are removed in place.
- With the CLI's file of the NAS store, the local disk holds two saved
  indexes of about 3.8 GB each. Deleting either costs one rebuild.
