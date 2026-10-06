# 0007: Library size beyond 32,768 assets

## Status

Open (2026-10-06). No decision yet. This record collects the options, what
each costs, and the measurements that should decide between them.

## Context

The index packs each posting into 32 bits: 17 bits for the anchor frame and
15 for the asset (ADR 0005, `index.rs` `Posting`). 17 frame bits are needed
because `index` accepts tracks up to 20 minutes (16 bits would hold 17.5
minutes at 16 ms per frame); the longest indexed track today is 9.6 minutes.
15 asset bits address 32,768 assets, and every file is an asset, duplicate
rips included. The offsets table uses `u32` positions, so one index holds at
most 4.3 billion postings.

The owner may add jungle and breakbeat hardcore, for about 100,000 tracks.
Extrapolated linearly from 262 tracks (74.4 postings per second of audio,
402 s mean track); these figures are not measured:

| Quantity | 262 tracks (measured) | 100,000 tracks (extrapolated) |
|----------|----------------------:|------------------------------:|
| Postings | 7.8 million | 3.0 billion (70% of the `u32` offsets) |
| Index in memory, 4-byte postings | 40 MB | 12 GB |
| Index delta-coded (offline estimate) | 33 MB | 9.3 GB |
| Peak store | 19.7 MB | 7.5 GB |
| Mean postings per hash bucket | 3.7 | 1,400 |
| Lookups, lines and chains, 56-minute mix | 2.6 s | about 16 min (35 min for two hours) |
| Initial indexing, 10 cores | 48 s | about 5 h, plus NAS reads |

Per track that is about 30,000 postings: 120 KB in memory, about 93 KB
delta-coded.

## Options

**A. Shards by asset range.** Several independent indexes of up to 32,768
assets, each with the current 4-byte postings and its own offsets table.
Lines and chains never span two assets, so each shard is searched on its own
and the detections are concatenated.

- Removes both addressing limits; keeps the measured layout and fidelity.
- New tracks go to the newest shard, so adding music rebuilds one shard.
- Memory can be bounded by loading and searching one shard at a time.
- Stored delta-coded on local disk (ADR 0005) and decoded into the
  fixed-width layout on load.
- Every query still searches every shard: no saving in work or in chance
  alignments.

**B. Separate libraries (databases), for example drum & bass and jungle in
one, breakbeat hardcore in another.** Each library has its own peak store and
index (already possible with `--library` and `--peaks-dir`), and the user
chooses which to search.

- A query against one library is as fast, and meets as few chance
  alignments, as that library's size allows.
- Breakbeat hardcore reuses drum breaks heavily (Amen, Think and others).
  Keeping it apart keeps those shared breaks out of drum & bass queries.
- Each library could be calibrated on its own, so a hardcore library could
  need a higher threshold without raising the drum & bass one. That changes
  the "one frozen rule for every input" principle to one rule per library,
  and it needs a development and a test set per library; only drum & bass
  sets exist today.
- Genres overlap: a set that crosses from jungle into hardcore needs both
  libraries, and searching two libraries costs the same and meets the same
  chance alignments as one library of their combined size. A track filed in
  the wrong library is missed when only the other is searched.
- A library over 32,768 assets still needs A or one of the layouts below.

**C. Shards by collection.** A combination of A and B: one library whose
shards follow owner-assigned collections (genres, labels, eras) instead of
asset ranges. `identify` searches all shards by default, or the collections
named on the command line.

**D. Wider postings in one index.** 5 bytes (20 asset bits): 15 GB at
100,000 tracks and a full rebuild for every addition. 8 bytes: identical
results measured (experiment 0004) but about 24 GB, too much for a 32 GB
machine.

**E. Narrower frames.** 16 asset bits with 32 ms frames cost 14% of the
sweep margin (experiment 0004). A 17-minute track limit gives 16 asset bits
at full resolution. Both stop at 65,536 assets, so neither reaches 100,000.

**F. Delta-coded lists decoded on every lookup.** No asset limit and the
smallest index, but decoding enters the hottest loop; the cost is unmeasured.

## What grows whatever the layout

No option above reduces these; they set the practical ceiling:

- **Query time.** Total posting-list length grows linearly with the tracks
  searched. Shards and libraries split the work but do not remove it, and
  the 41 rungs already use every core. Levers to measure: skipping the most
  common hashes (the fullest 1% of buckets hold 31% of postings) and longer,
  more distinctive hashes that still survive the speed ladder.
- **Chance and shared-material alignments.** More tracks give more
  opportunities for an unrelated record to align, by chance or through a
  shared break. Both thresholds (`docs/calibration.md`) were set at 262
  tracks and will probably rise. A high enough threshold misses short plays
  (Sick Note cleared the rule by 9 hits), and the possible tier may stop
  adding anything.
- **Duplicate clustering.** The self-match in `gunfinger-eval clusters` took
  20 minutes for 262 files and does not scale to 100,000; it needs a cheaper
  method (short excerpts at native speed, or a self-join over the index).

## Measurements that should decide

1. Pad the index with time-reversed copies of the library at several
   speeds. They have realistic peak statistics but cannot truly align with
   a forward mix, so a few thousand to 10,000 of them show how query time
   and chance alignments grow, without new music. A proxy, not a substitute.
2. As real tracks arrive, follow the procedure in `docs/calibration.md` at
   steps such as 1,000, 10,000 and 30,000 tracks, so a drifting threshold is
   seen before it fails.
3. Once breakbeat hardcore is indexed, compare the strongest false
   candidates with and without it in the index. If shared breaks raise the
   null for drum & bass queries, that argues for B or C; if not, A is
   simpler.

## Decision

None yet. In discussion (2026-10-06) shards (A) were proposed as the
simplest way past the limit; the owner is also considering a separate
library for breakbeat hardcore (B). The measurements above should decide.

## Consequences

To be written with the decision.
