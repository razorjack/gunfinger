# 0015: Files that vanish while the library is listed

## Status

Accepted (2026-10-11, session 13). Refines ADR 0012: a file or folder
below the root that is gone when the listing reads it is not a scan
failure. ADR 0012's rule stands for every other error, and so does its
fallback to the store's records when the root itself is missing.

## Context

`Library::scan` stopped at its first read error. Another program may add,
rename or delete files while Gunfinger runs: a partial file renamed
or deleted between its folder's listing and the `stat` of its metadata,
or a folder removed before it is read, failed the whole listing with "No
such file or directory". `doctor`'s first run of session 12 failed so.
Since ADR 0012, `identify`, `explain` and `stats` then stop instead of
searching the store's records, and `index` and `prune` stop too.

A share that drops during the listing can also answer NotFound for
everything below its mount point. Skipped entry by entry, that would look
like a library that lost most of its files, and `prune` would delete
their records.

## Decision

- **What vanishes**: an entry below the root whose type answers
  `io::ErrorKind::NotFound`; a folder whose listing does (opening it or
  reading its entries); an audio file that would be an asset whose
  metadata does. Files passed over by their names (hidden, another
  extension, a name that is not UTF-8) and audio files the ignore file
  leaves out are not read, as their size and time are not needed, so they
  never vanish: a downloader's partial files and thumbnails are passed
  over as before. What vanishes is not part of the library: not an asset,
  not ignored, not passed over. `Library::vanished` holds its paths,
  relative to the root and sorted. A folder gone before it is read counts
  once, whatever it held; one that goes while the walk is inside it
  leaves each of its remaining entries.
- **Other errors stop the listing**, as before: permissions, a folder
  replaced by a file (`NotADirectory`), timeouts and the errors of a
  disconnected share.
- **The root is never skipped.** If its entries cannot be read, the
  listing fails as before: a missing root is ADR 0012's fallback for
  `identify`, `explain` and `stats`, an error for `index` and `prune`, and
  a report of the store alone for `doctor`; any other error stops them.
  After a listing in which something vanished, the root is read again; if
  it is gone or cannot be read, the listing fails with that error, which
  the commands treat as a root that could not be read at the start.
- Each folder's entries are read in full before any is looked at, so the
  folder is closed before the walk goes deeper. The order of the walk and
  the resulting library are unchanged.
- **Saying so**: `identify`, `explain`, `stats` and `index` print one
  warning, such as `this run leaves out what vanished while
  /Volumes/atlas/Music/dnb was listed (moved or deleted meanwhile): 2
  files or folders (`a/b.mp3` and 1 more)`. The first path is named so
  that one that vanishes from every listing (a name the server lists but
  cannot read, for example) can be found. `doctor` reports the same as a
  note in its library section, and its line on records of files gone from
  the library then says that `prune` waits for a listing in which nothing
  vanishes.
- **`prune` refuses a listing in which anything vanished**, with or
  without `--yes` and with `--force`, names what vanished and asks to be
  run again with the same options: a folder that vanished may have been
  moved to a name the listing had already passed, and a dropped share may
  have answered for files that are still there, so the records it would
  call gone may not be. Confirming each file gone by its own check while
  the root is present would let it prune during a download; refusing
  cannot delete a record wrongly, and `prune` is rare. As only folders and
  audio files are read, a download's partial files do not make it refuse.
- **The harness refuses such a listing too** (`nothing_vanished`, for
  the corpus library and `--second-library`), as it stopped before: an
  evaluation must be repeatable. The corpus library never changes.
- Not done, both for a share that drops and comes back: checking each
  vanished path again at the end, and comparing the root's device before
  and after the listing. A downloader that replaces a file by deleting it
  and renaming another makes a path vanish and come back legitimately,
  which would fail the listing as before. The device check would catch a
  root that is itself a mount point left behind empty after an unmount;
  the NAS library's root is a folder inside the share, which is then missing.
  The root check covers both, and `prune` refuses anyway.

## Consequences

- A listing of a library being written finishes. A search's index and
  its saved file are unchanged unless an indexed file vanished.
- A share that drops and comes back between a vanished entry and the
  root check leaves that run with a partial library; the warning says how
  many entries vanished. `prune` does not act on such a listing.
- Files passed over by name are no longer read at all, which saves one
  `stat` per such file on a share (4,702 on the NAS in session 13).
- Detections do not change: a listing in which nothing vanishes gives
  the same library as before (`regress session-13-start` identical).
- Tests stage the race for real: the listing reads a folder's entries
  before it looks at any, so deleting them from the progress callback
  makes them vanish when read (files, folders, the root), while a partial
  file and a thumbnail are passed over by name. A folder without read
  permission still stops the listing. `prune` is tested with a listing
  that has a vanished entry.
