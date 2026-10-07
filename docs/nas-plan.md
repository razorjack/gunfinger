# The NAS library: what to measure once it is indexed

Written on 2026-10-07 while the owner's drum & bass collection on the NAS
was being indexed, so the ideas survive context compaction. The code these
steps need is in `docs/brief-nas.md`; this file is the analysis.

## The collection

- Library root `/Volumes/atlas/Music/dnb` (SMB over Wi-Fi), 28,439 audio
  files before the track length range; configuration
  `~/.config/gunfinger/nas-dnb.toml`: track length 1:30 to 15:00, playback
  turntable, peak store `~/.local/share/gunfinger/nas-dnb-peaks` on the
  laptop (it names its library in `library.txt`).
- Every file in `corpus/library/` is a copy of a NAS file: the corpus is a
  262-track subset of this collection.
- 28,439 files is 87% of the index's 32,768-asset limit before the range
  leaves out sets and samples. Whatever `index` extracts is what counts.
  Growth past the limit makes ADR 0007 (shards, separate libraries) the
  next decision.
- Expected sizes (inference): peak store about 75 KB per track, about 2 GB;
  index about 3.2 GB in memory (experiment 0019: 3.1 GB at 26,462
  assets); a 56-minute development scan at that size about 7.7 GB peak
  with today's matcher and 3.8 GB with skip at 240 (experiment 0029).

## Rules for the analysis

- The NAS is read-only. Nothing is written there; `index` only reads it.
- The peak store is the owner's. Only the owner's `index` writes to it,
  except the tags pass in step 1, run with the owner's configuration.
- No test-set scan without the owner's approval (3 evaluations left), and
  not before the owner has added references for its unreferenced tracks.
- Never edit a manifest. References to NAS files come from the content map
  (step 3) or from the owner.
- Each step that measures something is an experiment
  (`docs/experiments/NNNN-slug.md` plus data with file names only).
- Measure wall time, CPU time and peak memory one run at a time, and record
  the load average; other sessions may be using the machine.

## Commands

Run in the main checkout with a release build. `E` stands for
`target/release/gunfinger-eval --other-peaks-dir
~/.local/share/gunfinger/nas-dnb-peaks` (the corpus, its store and `work/`
are the defaults); its map, clusters and reports go to
`work/reports/library-nas-dnb-peaks/`.

```sh
# Step 1: tags for the records indexed before tags were kept (NAS mounted).
target/release/gunfinger --config ~/.config/gunfinger/nas-dnb.toml index
target/release/gunfinger --config ~/.config/gunfinger/nas-dnb.toml doctor
# Step 3: the content map (no mount needed from here on).
$E map-library
# Step 4: clusters of the corpus recordings at NAS scale.
$E clusters --from-peaks
# Steps 6 and 7: the standard evaluation at NAS scale, then the panels.
$E sweep --seed 2026          # and 2027-2029
$E scan stakka-skynet-knowledge
$E scan stakka-skynet-knowledge --leave-out 3
$E scan stakka-skynet-knowledge --leave-out 11
$E calibrate
# Step 10: identifying without the NAS.
target/release/gunfinger --config ~/.config/gunfinger/nas-dnb.toml identify <mix> --store-only -f json
```

`clusters --from-peaks` with the other store also writes every pair with
20% coverage or 30 hits, the input of step 5.

## Steps

1. **Indexing as it happened.** From the `index` summary: how long the
   full run took, how many files were extracted, too short, too long,
   failed. Compare the throughput over Wi-Fi with the 425 Mbit/s the CPU
   could take (experiment 0028: 262 tracks in 52.5 s from the local disk).
   Then run `index` once more with the new binary: it fills in the tags of
   records indexed before tags were stored, reading only file headers.
   `doctor`: files within the range against the asset limit.
2. **Tag coverage.** How many records have an artist and title, a title
   only, or neither (names fall back to file names). Which folders lack
   tags.
3. **The content map** (`gunfinger-eval map-library`). Each corpus file
   with its NAS copy, matched by identical peak records. Report corpus
   files with no copy or several copies, for the owner.
4. **Clusters of the corpus recordings at NAS scale** (`clusters
   --from-peaks` with the mapped corpus files as queries, repeated for new
   members until none appear). The 17 corpus clusters must reappear under
   NAS paths; any difference is a bug or a finding. List new members with
   their coverage; pairs between 40% and 80% are borderline and go to the
   owner to judge by ear. Estimated 30-60 minutes on 10 cores (inference:
   a 6-minute query costs about 60 s of CPU against 20,000 assets).
5. **Related recordings at NAS scale.** Experiment 0015's census around
   the corpus recordings: which different recordings share 30 hits or
   more with them, which reach the possible tier, and whether any reaches
   200 or 240. Label each pair from tags and paths as same artist or label
   (likely shared material) or unrelated (chance). This is the first real
   test of "shared material, not chance, sets the threshold". One known
   case: Bad Company's China Cup and The Nine share their drums (owner,
   by ear; 24-51 hits in experiment 0034), and Fresh & Vegas - Mekon
   reportedly has the same drums; measure every pair of them the
   collection holds.
6. **The development set at NAS scale**, today's matcher and skip at 240:
   the development scan, leave-outs 3 and 11 (held-out recordings with all
   their NAS rips), `calibrate`. Identified, wrong, the strongest false
   candidate and margin, possible plays, wall and CPU time, peak memory.
   Every unexpected confident detection goes to the owner by name and time
   to check by ear; under the rules it counts as wrong until confirmed.
7. **The sweep at NAS scale** on the saved panels (seeds 2026-2029), with
   held-out recordings and their NAS rips left out: weakest identifying
   detection, strongest false candidate, margin over four draws, against
   the 262-track figures (experiment 0025: weakest 403, strongest false
   97, margin 4.15× today; 658, 119, 5.53× with skip at 240).
8. **What the synthetic proxy predicted.** Experiments 0012, 0027-0029
   padded the library with reversed copies. Compare their predictions
   (chance at 18-31 hits, search CPU about 0.21 s per asset today and
   0.055 s with skip, memory) with the real collection of the same size.
   If real chance or cost differs much, the proxy must not be used for
   the next size step.
9. **Posting lists and famous breaks** (roadmap, "The filter and famous
   breaks"). `gunfinger stats` on the NAS store: postings per second,
   bytes per posting, bucket p99. Which hashes form the fullest 1%, and do
   tracks built on common breaks (Amen, Think) stop becoming candidates
   under skip at 240? Compare candidates per track with and without skip.
10. **Identifying without the NAS.** With the NAS unmounted, `identify
    --store-only` on the development mix, from the store and from a copy
    of it on another disk: the report must hold the same detections as a
    run with the NAS mounted, with names from the stored tags. Measure how
    long the store-only index build takes at this size from the internal
    disk and from a USB stick; if it is slow, a packed store or the
    on-disk index (ADR 0005) is the next step.
11. **References for the test set** (the owner's part). The test mix lists
    31 tracks, 16 with references in the corpus. For the other 15, list
    NAS files whose tags or paths match the manifest's artist and title,
    searching from the manifest side so the ground truth does not depend
    on what the matcher found. The owner confirms by ear. Open question
    for the owner: where NAS references live, since manifests reference
    corpus files (for example a per-set overlay file). Then one test-set
    evaluation at NAS scale, with the owner's approval, would be the most
    informative use of one of the 3 left; it changes the denominator, so
    evaluation 2's 15/16 does not compare directly.
12. **Afterwards.** Update `docs/calibration.md` with the NAS-scale figures
    and apply its "If it moves" column; record whether the matcher
    decision (today's against skip at 240) looks different at this size.

## Further ideas

- **Duplicates across the collection.** Clustering every NAS file against
  every other costs on the order of 30 hours at 20,000 tracks
  (inference). Byte-identical copies are free to find (size, then a hash
  of the bytes or identical peak records). Worth a census of exact copies
  first: they inflate the index and split evidence between same-audio
  assets in reports.
- **Indexing over Wi-Fi.** Per-file latency of SMB with 10 workers was
  never measured. If step 1 shows throughput far below the link speed,
  try fewer or more workers.
- **Portable stores.** If the store-only build is slow from a USB stick,
  measure the cost of reading 28,000 small files against one packed file.
