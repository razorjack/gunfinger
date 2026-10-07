# Brief: a portable peak store and evaluation against the NAS library

Written by the agent for itself on 2026-10-07, so the plan survives context
compaction. This work runs beside session 4 (`docs/brief-4.md`), in its own
worktree.

## Where the work happens

- Worktree `/Users/razorjack/Projects/OpenSource/gunfinger-portable`,
  branch `portable-store`, started from master at 2131970. Session 4 works
  in the main checkout (`/Users/razorjack/Projects/OpenSource/gunfinger`):
  never edit, build or commit there until this work is merged.
- When done: rebase onto master once session 4 has finished, run
  `scripts/check.sh`, fast-forward master (no merge commit), remove the
  worktree and the branch. The owner decides when.
- Progress: `docs/nas-checklist.md`. The analysis to run once the NAS is
  indexed: `docs/nas-plan.md`.

## Read after every compaction

1. `docs/brief.md` (the owner's brief; never edit it) and `AGENTS.md`.
2. This file and `docs/nas-checklist.md`, in the worktree.
3. `docs/nas-plan.md`.
4. `git log --oneline master..portable-store` in the worktree.

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
6. **The harness against another library** (`gunfinger-eval`).
   `--library <root>` beside the existing `--peaks-dir`, and
   `--library-map <map>`, which translates manifest references and the
   sweep panels' held-out recordings to the other library's paths, so no
   manifest is edited. Reports go to `work/reports/library-<name>/`.
7. **`gunfinger-eval map-library`.** Pairs each corpus file with the
   files of another peak store that have identical peak records (and
   size, where it agrees). Needs no audio and no mount. Writes the map
   (file names only) and lists corpus files with no copy or several.
8. **Clusters from chosen queries.** `clusters --from-peaks` with a set of
   query files (the mapped corpus files) searched against the whole other
   library, repeated for newly found members until none appear, so chains
   are found. Same criterion as today. Clustering every NAS file against
   every other is out of scope.

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
- Session 4 and the owner's `index` share the CPU: no timing claims from
  this worktree.
