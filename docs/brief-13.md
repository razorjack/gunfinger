# Session 13 brief: files that vanish while the library is listed

Written by the agent for itself on 2026-10-11, so the plan survives context
compaction. Work autonomously and never stop to ask. When something is
ambiguous, take the conservative option, write the decision down and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-13-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 12" to the end).
4. `docs/status.md` (newest entries at the bottom).
5. The newest file in `docs/experiments/`.
6. `docs/roadmap.md` ("Files that vanish while the library is listed",
   "Library identity and relocation"); ADRs 0009, 0011, 0012, 0014.

`docs/brief-2.md` to `docs/brief-12.md` hold rules that still apply
unless this brief changes them (among them: edit no configuration file;
test fixtures are generated, never copied from the corpus or the NAS;
long runs start detached under `screen`).

## The prompt, verbatim

```text
Start session 13 of Gunfinger: one code fix, files that vanish while the
library is listed (docs/roadmap.md, "Files that vanish while the library
is listed"). The session may run up to 3 hours, unattended. The NAS is
mounted at /Volumes/atlas. Files may be added
to the NAS while the library is listed; that
is expected, and it is the situation this fix is for.
Work autonomously and never stop to ask. When something is ambiguous,
take the conservative option, write the decision down and continue.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it),
   docs/brief-12.md and docs/session-12-checklist.md (brief-2 to brief-11
   hold rules that still apply unless this prompt changes them),
   docs/session-notes.md (from "Session 12" to the end), docs/status.md
   (newest entries at the bottom), the roadmap item above and "Library
   identity and relocation", and ADRs 0009, 0011, 0012 and 0014.
2. `git status` should be clean at c066c2a. Write docs/brief-13.md (this
   prompt verbatim, the state at the start, the plan with times) and
   docs/session-13-checklist.md. Point CLAUDE.md's reading list at them;
   keep brief-12 among the older briefs. Commit.
3. Build with --release and copy the binaries to work/bin/s13-start/. Run
   `gunfinger-eval baseline session-13-start`. Record the NAS peak store's
   state (scripts/analysis/store_revision.py): 33,596 records and revision
   f9f9bc706195a71b expected.

The problem: `Library::scan` (crates/gunfinger-core/src/library.rs) stops
at its first read error. A file or folder renamed or deleted between
`read_dir` and its `file_type`/`metadata`, or a folder gone before its
`read_dir`, fails the whole listing with "No such file or directory".
`doctor`'s first run of session 12 failed so while files were being added to the NAS.
Since ADR 0012, `identify`, `explain` and `stats` then stop instead of
searching the store's records.

The fix: an entry below the root that is gone when it is read (NotFound)
is not part of the library: skip it, count it, and say so in one line
(how many entries vanished while listing). Other errors still stop, as
today. Constraints, because a NAS that disconnects in the middle of a
listing can also answer NotFound for everything below the mount:
- The root is never skipped. A missing root stays an error or ADR 0012's
  fallback to the store, exactly as today.
- After a listing that skipped entries, check the root again; if it is
  gone or unreadable, fail as a missing root does.
- `prune` must never delete a record on the strength of a listing that
  skipped entries without confirming that file is gone by its own check
  while the root is present; refusing with a message to run it again is
  an acceptable conservative choice. Decide and document it.
- `index` and `doctor` follow the same listing; `doctor` reports the count.
- Detections do not change.
Write the decision in a new ADR or an amendment of ADR 0012. Tests: the
listing's handling of NotFound on files and on folders, other errors
still stopping, the root rules, and `prune` with a listing that skipped
entries (inject the errors through a seam if a real race cannot be
staged in a test; temporary folders only).

Rules for this session:
- The NAS (/Volumes/atlas/Music/dnb) is read, never written. The NAS peak
  store is read-only: no `index`, `prune` or `--retry-skipped` against it
  (we do not want the new files indexed); check its revision at
  the end. An `identify` of the library may rebuild and replace the saved
  index in ~/.cache/gunfinger/indexes/ by design; keep one file per store.
- Never touch the held-out test set (ed-rush-optical-essential-mix), and
  never identify, decode or index EM_1999_02_07_-_Ed_Rush_Optical.mp3 at
  the NAS root. Run nothing on the candidate mixes of experiment 0065.
- Verdicts and manifests do not change. The matcher and the confidence
  rule do not change; `regress session-13-start` must be identical after
  the change; scripts/check.sh green.
- Out of scope: the speed work of experiment 0088 and everything we
  have not decided (the containment rule, shared material, the
  possible tier, edits, clusters, test evaluation 5).
- Commit after each step; check `git status` before every commit and
  never use `git add -A`. Do not push.

Steps:
1. Setup (20 min): the steps above.
2. The fix, the ADR and the tests (about 1 h).
3. Check on the live NAS (about 20 min), while files are being added:
   `doctor --config ~/.config/gunfinger/nas-dnb.toml` several times with
   the new build, recording each run's audio file count, vanished count
   and duration; then one `identify` of
   corpus/sets/razorjack-2003-03-29/razorjack-2003-03-29.mp3 with that
   configuration and without --store-only, to show it lists the library
   and searches. A short experiment file (at most 40 lines, next number
   after 0088).
4. A read-only review of the change by a subagent; fix what it finds.
5. Wrap-up (20 min): a "Session 13" section at the end of
   docs/session-notes.md (what changed, in a few lines);
   docs/status.md, the checklist, the roadmap item marked done, the README
   where it describes listing the library. Check the NAS store's
   revision. Commit. Last, print what was done, the checks,
   and anything left.
```

## State at the start of session 13

- 18:11. `git status` clean at `c066c2a` on `master`. Experiments
  0001-0088; ADRs 0001-0014. Test set: 4 of 5 evaluations used (none in
  this session).
- `/Volumes/atlas/Music/dnb` mounted
  (935 entries at
  the root).
- The saved NAS index of `identify`:
  `~/.cache/gunfinger/indexes/nas-dnb-peaks-23935cfc56936b67.index`,
  3,795,332,786 bytes, written 2026-10-11 03:25.
- What the code does today:
  - `Library::scan_with_progress` walks the root with `scan_dir`, which
    returns at the first `io::Error` of `read_dir`, an entry, its
    `file_type` or its `metadata` (`ScanError::Io`).
  - `catalog::Indexable::find` (identify, explain, stats): on
    `ScanError::Io` it falls back to the store's records only when
    `fs::metadata(root)` answers NotFound; otherwise it stops and
    suggests `--store-only` (ADR 0012).
  - `catalog::scan_library` (index, prune) stops on any error.
  - `doctor::check_library` reports the store alone after a listing
    error.
  - `prune` refuses an empty library and, without `--force`, deleting
    more than half the store; otherwise it deletes the records of every
    file the listing did not hold (`survey`: `Gone` or `Ignored`).
  - The harness lists libraries with `Library::scan` (corpus library,
    `--second-library`).

## Plan, with times

Start 18:11; the session may run until 21:11.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1 | setup: brief, checklist, build, baseline, store revision | 0:20 | 18:31 |
| 2 | the fix in `Library::scan`, the root rules, `prune`, `doctor`, `index`; ADR 0015; tests; checks; regress | 1:05 | 19:36 |
| 3 | the live NAS: `doctor` several times, one `identify` without `--store-only`; experiment 0089 | 0:25 | 20:01 |
| 4 | read-only review by a subagent; fixes | 0:25 | 20:26 |
| 5 | wrap-up: notes, status, checklist, roadmap, README, store revision | 0:20 | 20:46 |
| - | buffer | 0:25 | 21:11 |

## Conservative decisions taken up front

- Only `io::ErrorKind::NotFound` on an entry below the root is skipped.
  Every other kind (permission, timeouts, `ENOTCONN`, stale handles)
  stops the listing as today.
- The root is checked by its own `read_dir` before the walk (as today,
  its error stops the listing) and again after a listing that skipped
  entries.
- `prune` decides from the survey as before, but when the listing
  skipped entries it refuses to delete, with a message to run it again;
  the dry run still lists. Recorded in the ADR.
- The harness keeps `Library::scan`'s behaviour through the same
  function; the corpus library never changes during a run, so its
  listings skip nothing and `regress` stays identical.
