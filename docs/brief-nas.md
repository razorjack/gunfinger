# Brief: a portable peak store and evaluation against the NAS library

Written by the agent for itself on 2026-10-07, so the plan survives context
compaction. This work runs beside session 4 (`docs/brief-4.md`), in its own
worktree.

## Where the work happens

- The code was written in a worktree beside session 4 (branch
  `portable-store`, from master at 2131970), rebased onto session 4's end
  and fast-forwarded into master on 2026-10-07; the worktree and branch
  are removed. The analysis runs in the main checkout.
- Progress: `docs/nas-checklist.md`. The analysis to run once the NAS is
  indexed: `docs/nas-plan.md`.

## Read after every compaction

1. `docs/brief.md` (the owner's brief; never edit it) and `AGENTS.md`.
2. This file and `docs/nas-checklist.md`.
3. `docs/nas-plan.md`.

## The owner's request

The owner is indexing the drum & bass collection on the NAS
(`/Volumes/atlas/Music/dnb`, 28,439 audio files, configuration
`~/.config/gunfinger/nas-dnb.toml`, peak store
`~/.local/share/gunfinger/nas-dnb-peaks`). Every corpus file is a copy of
a NAS file. The owner wants:

1. Identification without the NAS mounted. The peak store is the
   database: it should be portable, so that someone else can identify
   tracks from a copy on a thumb drive. Listening needs the audio;
   obtaining the title must not.
2. Correct identifications of NAS files to count as correct in the
   evaluation. Agreed approach: map corpus files to their NAS copies by
   content, cluster the corpus recordings' rips at NAS scale, and leave
   new references for unreferenced tracks to the owner.

The owner asked for all the code below, autonomously until complete,
assuming the NAS index is still running (it will finish today). The
analysis in `docs/nas-plan.md` waits for it.

## Code to write

Each item is its own commit, `scripts/check.sh` green, ticked in the
checklist.

1. **Progress while `index` lists the library.** The scan of a large
   share prints nothing for minutes. Show a progress line ("scanning:
   12,400 files") on a terminal, as `identify` does for rungs.
2. **Tags in the peak store** (`decode.rs`, `store.rs`, `indexing.rs`).
   One `ffprobe` call per file gives the declared length and the tags
   (artist, title, album). Tags go in a sidecar beside each record
   (`<hash>.tags`), keyed by path, size and modification time like skip
   notes, written atomically; the record format does not change. `index`
   writes them with each new record and fills them in for up-to-date
   records that have none, reading only the file's header. The store's
   survey, `prune` and `doctor` know the new kind of file.
3. **Names in the report** (`report.rs`, `names.rs`, `output.rs`,
   `export.rs`). Each play carries its artist and title, from the stored
   tags at identify time (file name when there are none); a new report
   schema version. Renderers use the report's names and fall back to
   reading tags for older reports.
4. **A store-only index** (`catalog.rs`, `indexing.rs`, `main.rs`,
   `config.rs`). When the library cannot be read or none is given,
   `identify`, `explain` and `stats` build the index from the store's own
   current records and say so in one line; `--store-only` forces it. The
   track length range applies; the library revision is computed the same
   way. Records of deleted files are included until `prune` removes them;
   the `library.txt` check is skipped (there is no library to compare).
5. **Tests and documentation.** An end-to-end test: index a synthetic
   library with tagged files, move the library away, identify from a copy
   of the store with no configuration, and check the tag title in the
   output. Unit tests for the tag sidecar and the probe parsing. README:
   identifying without the library, and taking a store to another
   machine.
6. **The harness against another library** (`gunfinger-eval`). Design
   settled while writing it (replaces the first sketch of `--library` and
   path translation): a global `--other-peaks-dir <store>` names the other
   library's peak store; its library is never read, so the NAS need not be
   mounted (records come from `PeakStore::current_sources`). The corpus
   stays the library: manifests, panels and excerpt rendering are
   untouched. The index is the corpus records plus the other store's
   records, streamed and named `second-library/<path>` as `--second-library`
   does, less one copy per corpus file (from the map, item 7), so its
   content equals the other library's. Clusters are the corpus clusters
   merged with the other library's clusters of the corpus recordings (item
   8), so a detection of a NAS rip counts as correct and held-out or
   left-out recordings take their NAS rips with them. `Padding::index`
   gains the excluded set; the sweep builds its index through it (the same
   algorithm as `Index::build`, so default results do not change). Reports
   go to `work/reports/library-<store dir name>/`, where `calibrate`,
   `baseline` and `regress` read them. Supported by `sweep`, `scan`,
   `robust`, `memory`, `calibrate`; other commands refuse the flag.
7. **`gunfinger-eval map-library`** (with `--other-peaks-dir`). Pairs each
   corpus file with the other store's files that have identical peak
   records (duration first, from headers, then the peaks), noting whether
   the size agrees. Needs no audio and no mount. Writes
   `library-map.json` in the reports directory of item 6 (file names
   only): every copy, the one copy each corpus file stands for in the
   index, and the corpus files with no copy or several.
8. **Clusters from chosen queries** (`clusters --from-peaks` with
   `--other-peaks-dir`). The index is the other library alone (two-pass,
   one record at a time). Each corpus file's stored peaks are searched on
   the clustering ladder; same-recording members that are not copies of a
   corpus file are searched in turn until none appear, so chains are
   found. Same criterion as today. Pairs with 20% coverage or 30 hits are
   kept (the related-recordings census of the plan's step 5). Writes
   `duplicate-clusters.json` in the reports directory of item 6, in the
   index's names, merged with the corpus clusters, and prints whether the
   corpus clusters reappear. Clustering every NAS file against every
   other is out of scope.

## Rules

- `corpus/` is private, copyrighted audio: read-only, never committed.
  Never edit a manifest.
- The NAS and the owner's NAS peak store are not touched while `index`
  runs. Test with scratch libraries and stores under `work/`.
- Default detection must not change: `regress` against a baseline taken
  in this worktree before the first core change must stay identical.
- Commit each piece once `scripts/check.sh` is green. Never `git add -A`
  without checking. No remotes, no pushing, nothing published. No AI
  attribution. No em dashes anywhere.
- House style from `AGENTS.md`. New dependencies must earn their place.
- Timings only from an idle machine: the owner's `index` and other
  sessions share the CPU.
