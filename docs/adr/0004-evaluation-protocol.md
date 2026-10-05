# 0004: Evaluation protocol

## Status

Accepted.

## Context

The owner's question is whether tracks can be found in real vinyl mixes
without confident wrong answers. Two sets have manifests: a development set
(`stakka-skynet-knowledge`, 11 tracks, all referenced) and a held-out test
set (`ed-rush-optical-essential-mix`, 31 tracks, 17 referenced, 14 absent,
radio broadcast). Tuning on the test set would make its verdict meaningless.

## Decision

All ground truth lives in `gunfinger-eval`, which depends on
`gunfinger-core`; the core and the CLI cannot reach a manifest.

1. **Duplicate clusters** (`gunfinger-eval clusters`): every library file is
   searched against the whole library on a ±2% ladder. Two files are the same
   recording when one alignment covers at least 80% of the shorter file.
   Clusters are the transitive closure, written to
   `work/reports/duplicate-clusters.json`. They depend on library audio only.
2. **Speed sweep** (`gunfinger-eval sweep --seed N`): 20% of clusters are
   held out of the index; 30 s excerpts of 60 indexed and 20 held-out assets
   are rendered by FFmpeg at -8, -5, -3, -1, 0, +1, +3, +5, +8% (pitch and
   tempo together) and encoded as 128 kbit/s MP3. Indexed excerpts must
   return their own cluster; held-out excerpts must return nothing
   confident. Bar: at least 95% recall within ±5%, zero wrong answers.
3. **Development set** (`gunfinger-eval scan stakka-skynet-knowledge`), plus
   leave-outs (`--leave-out 3 --seed N`): the clusters of three seeded
   referenced tracks are removed from the index and their slots must then
   produce no confident detection.
4. **Scoring**: only confident detections count. A referenced track is
   identified when a confident detection of a reference or a cluster member
   overlaps the window from 90 s before its start to 90 s after the next
   track's start. Every other confident detection is wrong. Cluster credits
   are listed separately. Manifests are never edited.
5. **Pass bar** per set: at least 80% of referenced tracks identified and
   zero wrong identifications.
6. **Freeze, then test**: tag `poc-freeze-1`, then scan the test set. At most
   five test evaluations in total, each logged in
   `docs/experiments/test-set-ledger.md`.

## Consequences

- The sweep measures speed handling and the null on unrelated audio; the
  development set measures real mixing (overlaps, EQ, vinyl); leave-outs
  measure the null where it matters most, in a slot whose record is missing.
- The confidence rule is calibrated on all three together and frozen before
  the test set is touched.
- A suspected ground-truth error is reported for the owner and still counts
  as wrong.
