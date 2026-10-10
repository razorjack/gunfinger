# 0005: Index layout, and the recommended on-disk design

## Status

Accepted for the in-memory index; proposed for the on-disk index. The
split of the posting below (17 frame bits, 15 asset bits) is superseded
by ADR 0010 (2026-10-09): 16 frame bits address tracks up to 17:28 and 16
asset bits 65,536 assets, in the same 4 bytes. The proposed on-disk design
is superseded by ADR 0011 (2026-10-10): the saved index keeps the
in-memory layout, and a delta-coded file would need `u64` byte offsets.

## Context

The index maps each 21-bit pair hash to its postings (asset, anchor frame).
The collection will reach 25,000+ tracks on a NAS. The proof of concept
rebuilds the index in memory from the peak store on every run, using the
layout an on-disk index would use, so its sizes are real (experiment 0004).

Measured on 262 tracks (29.3 h), fan-out 2, ±12 neighbourhood:

- 74.4 postings per second of audio; 7,836,148 postings;
- 5.07 bytes per posting including the offsets table (4 for the posting,
  1.07 for the 8.4 MB table at this size);
- buckets: mean 3.74, p99 55, max 1,796; fullest 1% hold 30.7% of postings;
- peak store: 187 bytes per second of audio.

## Decision

**In memory (now).** An offsets table of `2^21 + 1` `u32` entries and one
array of 4-byte postings ordered by hash, then asset, then frame. Each
stored field is needed:

- the hash is implicit in the posting's position (offsets table), costing
  nothing per posting;
- asset (15 bits, 32,768 assets) identifies the recording;
- anchor frame (17 bits, 35 min at 16 ms) is needed at full resolution to
  find lines: storing frames at 32 ms cost 14% of the sweep margin.

8-byte postings gave identical results at 9.07 against 5.07 bytes per
posting, so 4 bytes is the leanest lossless layout.

**On disk (recommended for the next session).** One file per library,
rebuilt from the peak store (never the reverse):

1. a header: format version, front-end profile, hash design version, asset
   count, and the asset table (path, size, mtime, duration), so a stale
   index is detected the same way as a stale peak record;
2. the offsets table (`2^21 + 1` × `u32` byte offsets, 8.4 MB);
3. posting lists, each delta-coded: asset gap as a varint, then the frame as
   a varint gap when the asset repeats or in full when it changes. Measured:
   3.12 bytes per posting against 4.00 (22% smaller). Lists are decoded one
   at a time, which suits the access pattern: a query reads every list it
   hits once per rung.

At 25,000 tracks of 402 s on average this is 7.5e8 postings: 3.0 GB with
fixed 4-byte postings, about 2.3 GB delta-coded, plus 8.4 MB of offsets.
Write it atomically (temporary file, then rename) and read it sequentially;
no mmap, as for the peak store. Keep it on local disk when the library is on
a NAS: a two-hour mix touches most lists on 41 rungs.

Beyond 32,768 assets the layout must change. The options (shards,
separate libraries, wider or narrower postings) are compared in ADR 0007,
which is still open.

## Consequences

- Size grows linearly with the collection; so do posting-list lengths and
  therefore lookup time. Today a 56-minute mix takes 22 s, of which lookups,
  lines and chains are 2.6 s and the 41 STFTs most of the rest (experiment
  0005). With 95 times the postings at 25,000 tracks the lookups are likely
  to dominate; the next measurement to make is query time against library
  size.
- The offsets table is a fixed 8.4 MB at any size.
- The hash design is part of the index format: changing fan-out or
  quantisation means rebuilding the index from peaks, which takes about a
  second for this library.
