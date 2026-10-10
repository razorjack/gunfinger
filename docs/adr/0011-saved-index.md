# 0011: A saved index

## Status

Accepted (2026-10-10, session 11). Takes up the on-disk index that ADR
0005 proposed, with the in-memory layout instead of its delta-coded lists;
ADR 0005's in-memory decision and ADR 0010's posting stand.

## Context

At NAS scale every `identify`, `explain` and `stats` builds the index from
the peak store: 65-72 s at 32,905 records (experiment 0063), most of a
short `explain`. The index is a pure function of the peak records, the
profile and the hash design, so a file holding it can be loaded instead,
as long as it is never used when it no longer matches the store.

Width audit, at 65,536 assets of the NAS mean length (6.29 minutes; about
1.84 billion postings, 7.4 GB):

| Field | Width | Addresses | At 65,536 assets |
|---|---|---|---|
| posting: asset | 16 bits | 65,536 assets | the limit (ADR 0010) |
| posting: anchor frame | 16 bits | 17:28 | NAS `max_track` 15:00 |
| offsets table entry | `u32` posting position | 4.29 billion postings | 43%; full at a mean of 14.6 minutes |
| list length (`longest_scanned`) | `u32` | as above | as above |
| record path | `u16` length | 65,535 bytes | longest NAS path 258 bytes |
| file positions | `u64` | | a 7.4 GB file |

- The offsets table counts postings, not bytes, so the fixed 4-byte
  postings keep it within `u32` at this size. 65,536 files near the
  15:00 limit would pass 4.29 billion postings, and the sums of the
  offsets table would wrap silently in a release build; `Index::counting`
  now refuses such an index (`IndexError::TooManyPostings`).
- ADR 0005's proposed `u32` byte offsets into delta-coded lists address
  4.29 GB. At 3.12 bytes per posting, 1.84 billion postings need 5.7 GB:
  any compressed layout needs `u64` byte offsets, or `u32` offsets per
  block of lists.

## Decision

- **What the file holds**: the index as the run would build it, before
  the matcher sets the fullest lists aside, in the in-memory layout,
  little-endian: a header, the offsets table (`2^21 + 1` `u32` posting
  positions), the posting count (`u64`) and the 4-byte postings, then a
  64-bit FNV-1a digest of everything before it (the header byte by byte,
  the tables a word at a time). No compression; it is measured apart, if
  at all.
- **The header names what it was built from**: magic `GFINDEX\0`, format
  1, the profile id, `hash::design()`, the posting's frame bits, the track
  length range, and the asset table: each indexed file's path, size and
  modification time (the identity a peak record carries) and its
  duration, in asset order. The library revision is a digest of that
  table, so the table itself is kept and compared.
- **Never used stale or damaged**: a search reads the headers of the peak
  records (`indexing::plan_index`) to know what it would index, then
  loads the file only if its header names exactly that. A file that ends
  early or runs long, an offsets table out of order, a posting naming an
  asset the table does not hold, or a digest that does not match is
  unreadable. Either way the index is built from the store and saved over
  the file.
- **Where**: `$XDG_CACHE_HOME/gunfinger/indexes/`, else
  `~/.cache/gunfinger/indexes/`, one file per peak store, named after the
  store folder and a digest of its absolute path
  (`nas-dnb-peaks-<16 hex>.index`). It is derived data on the local disk,
  never beside the peak store, which may live on a NAS; deleting it costs
  one rebuild. A store has one file, replaced through
  `<name>.<pid>.partial` and a rename; partial files of writes that did
  not finish are deleted by the next save.
- **Who uses it**: `identify`, `explain` and `stats`, by default
  (`--saved-index use`). `rebuild` builds and saves; `off` builds in
  memory only, as before. With `--exclude-from` the index is not the
  library's, so the file is neither read nor written. A file that cannot
  be written is a warning; the search goes on.
- **The harness** keeps building in memory. Its runs index other sets of
  files (held-out clusters, padding, other stores), which one file per
  store could not hold, and `regress` measures that build. A harness use
  would go through `Provenance` with the run's own asset table, under a
  name of its own, and is not needed now.

## Consequences

- At 33,596 records the file is about 3.8 GB on the laptop's disk, and
  about 7.4 GB at 65,536 assets; `doctor` does not count it yet.
- A load reads the whole file sequentially into the same vectors a build
  fills, so memory is unchanged. A search after `index` added or changed
  files rebuilds once and saves again.
- Records whose header reads but whose peaks do not are left out of a
  build and stay in the plan, so such a store rebuilds on every search
  until the record is indexed again; the search warns about the record
  each time.
- A peak record rewritten under the same source identity and profile
  (path, size, modification time) holds the same peaks, as decoding is
  deterministic, so the asset table identifies the postings.
