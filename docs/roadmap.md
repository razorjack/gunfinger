# Roadmap

Work deliberately left out of the proof of concept, roughly in the order the
real project will need it. Each item links to the decision or measurement
that motivates it where one exists. Items marked *(larger library)* need
more real tracks; the rest can be done with the current library.

## Library and storage

- **Memory by phase.** The scan at 26,462 assets swapped (experiment 0012),
  but what used the memory was not measured. Measure peak memory separately
  while loading peak records, building the index and searching, at several
  worker counts, on the synthetic scale proxy. Known costs: `Catalog` keeps
  every peak record after the index is built (only `stats` reads them
  later), and `Index::build` first turns the whole library into points.
  Then compare building sequentially, releasing records, a persisted index
  and loading one shard at a time, each with identical detections. An
  on-disk index alone does not bound memory.
- **On-disk index.** The index is rebuilt in memory from the peak store on
  every run. The recommended layout is in `docs/adr/0005-index-layout.md`.
  Audit its widths before building it: its `u32` byte offsets into
  delta-coded lists address 4.3 GB, and 100,000 tracks would need about
  9.3 GB (ADR 0007); posting offsets and the 15 asset bits have limits of
  their own.
- **More than 32,768 assets.** The posting layout addresses 32,768 assets;
  the owner may grow the collection to about 100,000 tracks with jungle and
  breakbeat hardcore. Shards, separate libraries per genre and other layouts
  are compared in ADR 0007. Shards solve addressing and can bound memory,
  but searching every shard does not reduce the work or the chances of a
  false match. *(larger library)* Decide collection boundaries from
  evidence: the same queries against drum & bass alone, hardcore alone and
  their union, measuring false candidates, missed cross-genre matches and
  cost, with one confidence rule throughout.
- **Library identity and relocation.** Asset identity is already the path
  relative to the library root. Missing: a library identifier in the peak
  store, so two libraries cannot share a peak directory by accident; an
  explicit way to move a library or its peak store; tolerance for slow
  `stat` calls and reconnects on a NAS.
- **Cheaper duplicate clustering.** The self-match in `gunfinger-eval
  clusters` searches every file against the library (7 to 20 minutes for
  262 files) and will not scale. Try a cheap candidate search (short
  excerpts, or a join over stored peaks) followed by the current strict
  check, and compare candidate recall and final clusters with the
  exhaustive method. Add controlled cases first: different rips, cropped
  recordings, edits, and chains where A matches B and B matches C. A wrong
  cluster can hide a wrong identification or leak a held-out recording into
  the index.
- **Database.** Not needed so far: the peak store is one file per asset and
  the index is rebuilt from it in 0.5 s. Revisit when detections, owner
  edits or the Track/AudioAsset model need to be stored.
- **The Track/AudioAsset model.** An asset is a file. The owner thinks in
  tracks (a recording, possibly with several rips, edits and remixes).
  Duplicate clusters and related recordings (experiment 0015) are the
  evaluation's view of this; the product needs its own, editable model,
  shaped by how the owner uses `review`.

## Reports and command line

- **Reports record how they were made.** A saved report holds no search
  settings: playback, profile and hash design, the confidence rule, the
  library's revision. `identify --save-dir` passes over a recording with a
  report regardless of how it was made; it should search again when the
  inputs or settings differ.
- **Atomic report writes.** Reports are written in place, so an
  interrupted run can leave a truncated file that a batch then treats as
  done. Write a temporary file and rename it, as the peak store does.
- **Evidence per window in `explain`.** `explain` lists detections around a
  moment. Showing each window's lines (hits, offset, rung, which chain
  took them) would make boundary and chaining errors visible.
- **Owner edits.** Confirming, rejecting or renaming plays in a report, and
  keeping those edits when the mix is identified again. First try `listen`
  and `review` with real audio output (untested: no audio device during
  session 2).
- **A full-screen TUI.** `review` steps through a report by ear in a line
  loop. Browsing detections against the mix's waveform would need a terminal
  UI dependency; worth it only if `review` proves too limited.

## Matching

- **The common-hash filter under damage.** Emptying the fullest 1% of
  posting lists cuts the postings a query scans by 62% and passes the full
  protocol with a wider margin against false candidates (experiments 0013,
  0017). It also lowers the weakest identifying detection from 501 to 445
  hits, so misses near the rule become likelier. Before adopting it, run
  every robustness condition with the filter on and off (and with both
  ladders, if that combination is a candidate), plus brief excerpts and
  combined damage such as EQ with a blend and a low bitrate; report
  confident recall, possible recall and false candidates separately.
- **Brief plays and window placement.** Most measurements use 30 s
  excerpts, and every development track plays for minutes; brief plays are
  where 200 hits in 3 windows decide. Vary excerpt length, source position
  and offset against the 10 s search windows; add cuts, skips, fades and
  interrupted returns; measure how often the same audio changes level only
  because of where the windows fall. This also tests the assumption that a
  longer damaged play gathers enough evidence: it does only if the evidence
  still chains. Extend experiment 0009 with long blends, doubles,
  scratching and faster pitch changes.
- **Detection boundaries.** Boundaries are the first and last aligned hit.
  Shared kicks extended one synthetic detection 26 s past the end of its
  track (experiment 0008), and the scoring tolerance of 90 s cannot see
  such errors. Measure start and end error against synthetic boundaries
  (including fades and quiet passages), then try trimming weak chain ends,
  reporting lost true coverage beside the gain.
- **Behaviour at scale.** *(larger library)* The confidence rule (200
  hits, 3 windows) and the possible tier (60 hits) were calibrated against
  262 tracks. Recalibrate at staged sizes such as 1,000, 10,000 and 30,000
  assets (`docs/calibration.md`): strongest false candidates, weakest true
  detections, the possible tier's use, query time and peak memory, on the
  fixed query panel and the normal protocol. The reversed-copy proxy shows
  chance growing slowly up to 21,109 assets (experiment 0012), but shared
  breaks and remixes already give 130 hits between different recordings in
  this library (experiment 0015), so the new tracks must include jungle,
  hardcore, remixes, versions and producer catalogues that reuse material.
- **A confidence statistic for shared material.** *(larger library)* If
  identifying and shared-material evidence overlap at scale, raising the
  hit threshold would lose brief plays without resolving the ambiguity.
  Hold out a recording while keeping its versions and the tracks that share
  its breaks, and compare the current rule with rules that need evidence
  across more time regions or more distinct features; validate on
  recordings not used to develop them. Where the played passage is
  identical in two versions, the honest answer may be a related-recording
  suggestion or none.
- **Longer hashes.** Triplet hashes would scan 25-79 times fewer postings
  but lose about half their evidence under heavy damage (experiment 0014,
  estimated offline, no search). Revisit if profiling on a real large
  library shows lookups still dominate after simpler changes; compare
  recall under damage, related-recording errors, memory including the
  lookup structure, and wall time.
- **Evidence summed across a play.** Plays (ADR 0006) show Fibre Optix
  "Sin", faded out for a station insert, as one possible play of two
  segments (168 hits). Experiment 0011: summing every segment is unsafe
  (chance sums reach 125 hits), summing segments of at least 60 hits gave
  no false group, but no measured track would gain. Revisit when several
  independently reviewed missed plays have strong, compatible segments that
  together pass 200, and test a restricted rule against related recordings
  and long unknown passages.
- **Key lock by default.** `identify --playback both` finds key-locked
  plays (experiment 0010) and passes the full protocol with unchanged
  thresholds (experiment 0016); the search takes about a third longer.
  The owner decides whether it becomes the default.

## Evaluation

- **A fixed query panel.** Most measurements reuse seed 2026 and
  overlapping excerpts, and the sweep's draw changes whenever the library
  does, so it cannot isolate the effect of added tracks. Build a fixed set
  of queries, source positions and held-out clusters, kept across library
  growth, and run more seeds and positions beside the normal sweep.
- **More development mixes.** Mixes of existing library tracks, especially
  with brief plays and long blends, would test what the single development
  mix cannot. Agree each mix's role before looking at its results. The
  held-out test mix has three evaluations left (ADR 0004).

## Engineering

- **GitHub Actions.** Run `scripts/check.sh` on push. FFmpeg must be
  installed in the runner for the codec tests.
