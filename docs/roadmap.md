# Roadmap

Work deliberately left out of the proof of concept, roughly in the order the
real project will need it. Each item links to the decision or measurement
that motivates it where one exists. Items marked *(larger library)* need
more real tracks; the rest can be done with the current library. Figures
marked as inferences were derived, not measured.

## Library and storage

- **Memory by phase.** The scan at 26,462 assets swapped (experiment 0012),
  but what used the memory was not measured. Inference from struct sizes:
  a track costs about 0.6 MB while the index is built (37 peaks per second,
  a 16-byte `Peak` in the loaded record and a 16-byte `Point` made by
  `Index::build`, then 4-byte postings) and 0.12 MB once it is built, so
  26,462 assets need about 16 GB, enough to explain the swap. `Catalog`
  also keeps every peak record after the build (only `stats` reads them
  later). Measure peak memory while loading, building and searching, at
  several worker counts, on the synthetic scale proxy. If the inference
  holds, build in two passes reading one record at a time from the peak
  store (count, then fill), which needs little more than the index itself
  (about 3 GB at 25,000 tracks). Then compare a persisted index and
  loading one shard at a time. Every variant must give identical
  detections.
- **On-disk index.** The index is rebuilt in memory from the peak store on
  every run. The recommended layout is in `docs/adr/0005-index-layout.md`.
  Build it after the memory measurement, and audit its widths first: its
  `u32` byte offsets into delta-coded lists address 4.3 GB, and 100,000
  tracks would need about 9.3 GB (ADR 0007); posting offsets and the 15
  asset bits have limits of their own.
- **More than 32,768 assets.** The posting layout addresses 32,768 assets;
  the owner may grow the collection to about 100,000 tracks with jungle and
  breakbeat hardcore. Shards, separate libraries per genre and other layouts
  are compared in ADR 0007. Shards solve addressing and can bound memory,
  but searching every shard does not reduce the work or the chances of a
  false match. *(larger library)* Decide collection boundaries from
  evidence: the same queries against drum & bass alone, hardcore alone and
  their union, measuring false candidates, missed cross-genre matches and
  cost, with one confidence rule throughout (ADR 0007, measurement 3).
- **Library identity and relocation.** Asset identity is already the path
  relative to the library root. Missing: a library identifier in the peak
  store, so two libraries cannot share a peak directory by accident; an
  explicit way to move a library or its peak store; on a NAS, indexing
  throughput, slow `stat` calls and reconnects.
- **Cheaper duplicate clustering.** The self-match in `gunfinger-eval
  clusters` decodes every file and searches it on 11 rungs (7 to 20
  minutes for 262 files); it will not scale. At speed 1.0 a file's query
  points are its stored peaks, so a self-join over the peak store needs no
  decode and no STFT; all 46 same-recording pairs lie within ±0.17% of
  native speed, so transformed coordinates covering ±0.4% should find the
  candidates, and the current strict check then confirms them. Compare
  candidate recall and final clusters with the exhaustive method. Add
  controlled cases first: different rips, cropped recordings, edits, and
  chains where A matches B and B matches C. A wrong cluster can hide a
  wrong identification or leak a held-out recording into the index.
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
  took them) would make boundary and chaining errors visible, and is the
  tool for debugging brief plays and boundaries below.
- **Owner edits.** Confirming, rejecting or renaming plays in a report, and
  keeping those edits when the mix is identified again. First try `listen`
  and `review` with real audio output (untested: no audio device during
  session 2).
- **A full-screen TUI.** `review` steps through a report by ear in a line
  loop. Browsing detections against the mix's waveform would need a terminal
  UI dependency; worth it only if `review` proves too limited.

## Matching

- **Where real mixes lose evidence.** Clean 30 s excerpts give about 29
  hits per second (experiment 0009). Plays in real mixes give far fewer:
  1.6-5.9 per second in the test mix (ledger), 4.8-14.6 in the
  development mix. At 3 hits per second the confident rule needs about a
  minute of play, and no single `robust` condition loses that much. On the
  development mix only, align each confirmed play with its library file
  and attribute the loss: solo against blended passages, frequency band,
  distance from the nearest rung, speed variation within a window. Add
  `robust` conditions for wow (speed modulated once per revolution, 0.55
  or 0.75 Hz), broadcast compression, beatmatched blends (the current blend
  starts the partner at a random position, so its drums do not land on the
  first track's), and combined damage.
- **A second pass at the fitted speed.** Hash survival is 79% on a rung
  and 45% at 0.2% from it (experiment 0001), while a chain's fitted speed
  is accurate to about 0.016%. Analysing each candidate's span again with
  one STFT at its fitted speed and counting hits against that asset alone
  should give plays between rungs up to 1.7 times the evidence, and chance
  candidates nothing; related recordings gain as true plays do. It changes
  the evidence scale, so it needs its own measured null and calibration.
- **Brief plays and window placement.** Most measurements use 30 s
  excerpts, and every development track plays for minutes; brief plays are
  where 200 hits in 3 windows decide. Slide excerpts across the 10 s
  window grid in 1 s steps, vary length and source position, add cuts,
  skips, fades and interrupted returns, and measure how often the same
  audio changes level only because of where the windows fall. Try a
  minimum aligned span in seconds instead of 3 windows, which does not
  depend on the grid. This also tests whether a longer damaged play gathers
  enough evidence: it does only if the evidence still chains. Extend
  experiment 0009 with long blends, doubles, scratching and faster pitch
  changes.
- **The common-hash filter.** Emptying the fullest 1% of posting lists
  cuts the postings a query scans by 62% and passes the full protocol with
  a wider margin against false candidates (experiments 0013, 0017). It
  also lowers own-track hits to 74-89% in the development mix; applied to
  the weakest test-set identification (Sick Note, 209 hits) that predicts
  155-186, a miss at 200 (inference). Before adopting it, set the threshold
  from calibration data alone (about twice the strongest false candidate,
  76 with the filter), run every robustness condition with the filter on
  and off (with both ladders too, if that combination is a candidate), plus
  brief excerpts and combined damage, and report confident recall,
  possible recall and false candidates separately. A second pass at the
  fitted speed may recover what the filter costs.
- **The edge of the ladder.** Recall is 100% at ±8% and 0% at ±10%, with
  nothing measured between. Measure ±8.2% to ±9%; if a deck's fader
  reaches past 8%, a few more rungs may be cheap insurance.
- **Hash yield by frequency band.** 48% of peaks lie between 2 and 4 kHz.
  Measure true hits per 1,000 postings scanned by anchor band; if the top
  octave yields least, try a peak neighbourhood that widens with
  frequency.
- **Detection boundaries.** Boundaries are the first and last aligned hit.
  Shared kicks extended one synthetic detection 26 s past the end of its
  track (experiment 0008), and the scoring tolerance of 90 s cannot see
  such errors. Measure start and end error against known boundaries
  (generated mixes, including fades and quiet passages), then try trimming
  weak chain ends, reporting lost true coverage beside the gain.
- **Where search time goes.** Every timing in experiments 0012-0017 was
  taken on a loaded machine, and the scan at 26,462 assets did not finish.
  On an idle machine, finish the scale proxy and profile the search:
  scanning postings, sorting each window's hits, clustering offsets,
  chaining. Optimise only what the profile shows.
- **Behaviour at scale.** *(larger library)* The confidence rule (200
  hits, 3 windows) and the possible tier (60 hits) were calibrated against
  262 tracks. Recalibrate at staged sizes such as 1,000, 10,000 and 30,000
  assets (`docs/calibration.md`): strongest false candidates, weakest true
  detections, the possible tier's use, query time and peak memory, on the
  fixed query panel and the normal protocol. At each stage, repeat the
  related-recordings census (experiment 0015; needs cheaper clustering)
  and check whether any two different recordings reach the confident rule.
  The reversed-copy proxy shows chance growing slowly up to 21,109 assets
  (experiment 0012), but shared breaks and remixes already give 130 hits
  between different recordings in this library, so the new tracks must
  include jungle, hardcore, remixes, versions and producer catalogues that
  reuse material.
- **The filter and famous breaks.** *(larger library)* As breaks such as
  the Amen and Think become common, their hashes should move into the
  fullest lists. Check whether the filter then weakens related pairs more
  than the 30% measured here, and whether it starves break-heavy tracks of
  their own evidence. The dropped set changes with the library, so the same
  play's hit count drifts; if the drift is large, try weighting hits by
  the rarity of their hash instead.
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
  does, so it cannot isolate the effect of added tracks. Before the first
  new track, build a fixed set of queries, source positions and held-out
  clusters, kept across library growth, and run more sweep seeds beside
  the normal sweep: every margin so far comes from seed 2026.
- **Generated mixes of library tracks.** A harness command that renders
  seeded mixes with exact truth: speeds, crossfades, bass-swap EQ, plays of
  20-60 s, cuts, a returning track, held-out tracks. It supplies the brief
  plays and boundary truth the matching items above need. Rendered by the
  same tools as the robustness excerpts, it complements real mixes rather
  than replacing them.
- **The Clockwork mix CD.** `corpus/sets/stakka-skynet-clockwork-mix-cd/`
  holds a 59:45 mix and a 13-track cue sheet but no `tracklist.toml`, so it
  has never been scanned. By file name, 12 of its 13 tracks are in the
  library (the Kemal & Rob Data remix of Side Effects appears absent; the
  TeeBee remix is present). The owner writes its manifest and decides its
  role before anyone scans it; one option is a second held-out set with its
  own ledger, used to confirm default changes. It shares artists and tracks
  with the development mix, so it is an easier test than the Essential
  Mix.
- **More development mixes.** Mixes of existing library tracks, especially
  with brief plays and long blends, would test what the single development
  mix cannot. Agree each mix's role before looking at its results. The
  held-out test mix has three evaluations left (ADR 0004). *(larger
  library)* Each new genre needs its own development and held-out mixes;
  check the speed range those DJs used.

## Engineering

- **Before publishing.** 13 committed files under `docs/` name library
  files, including rip-group suffixes. Decide whether to keep, shorten or
  replace those names. `docs/assets/`, which the README uses, is not
  committed yet.
- **GitHub Actions.** Run `scripts/check.sh` on push. FFmpeg must be
  installed in the runner for the codec tests.
