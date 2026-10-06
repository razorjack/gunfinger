# Roadmap

Work deliberately left out of the proof of concept, roughly in the order the
real project will need it. Each item links to the decision or measurement
that motivates it where one exists. Items marked *(larger library)* need
more real tracks; the rest can be done with the current library. Figures
marked as inferences were derived, not measured.

## Library and storage

- **Search memory at scale.** The index is now built in two passes and
  is the only large thing a build holds (experiment 0019). The search then
  keeps every rung's lines for the whole query until chains are built:
  about 180 MB per minute of query at 26,462 assets, independent of
  workers (experiment 0021), so an hour-long mix there needs about 11 GB
  beside the 3 GB index (inference). Skipping the fullest 1% of posting
  lists in the first pass leaves 112,000 of its 1.74 million distinct
  lines for 5 minutes of query there (experiment 0027). Measure the peak
  again on an idle machine (today's peaks varied by half for the same
  binary), then merge neighbouring rungs' lines as each rung finishes,
  which can keep detections identical.
- **On-disk index.** The index is rebuilt in memory from the peak store on
  every run. The recommended layout is in `docs/adr/0005-index-layout.md`.
  Audit its widths first: its `u32` byte offsets into delta-coded lists
  address 4.3 GB, and 100,000 tracks would need about 9.3 GB (ADR 0007);
  posting offsets and the 15 asset bits have limits of their own.
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
- **Database.** Not needed so far: the peak store is one file per asset and
  the index is rebuilt from it in 0.5 s. Revisit when detections, owner
  edits or the Track/AudioAsset model need to be stored.
- **The Track/AudioAsset model.** An asset is a file. The owner thinks in
  tracks (a recording, possibly with several rips, edits and remixes).
  Duplicate clusters and related recordings (experiment 0015) are the
  evaluation's view of this; the product needs its own, editable model,
  shaped by how the owner uses `review`.

## Reports and command line

- **Shared material in the report.** The possible play of the Clockwork
  remix at 20:22 in the development mix lies inside the confident play of
  Clockwork itself, and the owner confirmed by ear that the passage is
  shared (the original's lead plus a pad). A possible play that lies
  entirely inside a confident play of another recording could be shown as
  "shares material with Clockwork (play 6)" instead of as a play of its
  own; display only, detection unchanged. It must not fold in a remix that
  the DJ plays in its own right next to the original, whose other sections
  form detections of their own.
- **Owner edits.** Confirming, rejecting or renaming plays in a report, and
  keeping those edits when the mix is identified again. `listen` and
  `review` work with real audio output (owner, 2026-10-06).
- **A full-screen TUI.** `review` steps through a report by ear in a line
  loop. Browsing detections against the mix's waveform would need a terminal
  UI dependency; worth it only if `review` proves too limited.

## Matching

- **Chance lines in chains.** A chain may skip two empty windows and
  link lines whose rungs are far apart, so a 3-hit chance line up to 30 s
  away can join a real play: it adds a window and span, and moved the
  fitted speed by up to 2.4% in the window grid (experiment 0020; the
  Clockwork remix's third window in `explain --windows` is one too).
  Require linked lines to come from nearby rungs, or a stronger line
  across a gap, and run the protocol.
- **A minimum span instead of 3 windows.** Whether a 15-20 s play is
  confident depends on where the 10 s windows fall: 76 of 160 grid
  positions at 15 s, 148 at 20 s; 200 hits over at least 10 s between the
  first and last hit makes all of them confident and changes nothing in
  the sweep or the development scans (experiment 0020, offline). It lowers
  the shortest identification from about 25 s to about 11 s, which is the
  owner's call; fix chance lines in chains first, since a linked chance
  line lengthens the span.
- **Speed that wanders within a play.** The second pass analyses a play at
  one fitted speed; Star Trails plays 0.1-0.3% above it in many windows
  and loses 15% of its hits (experiment 0024), and within-play speed
  varies by ±0.2% in the development mix (experiment 0022). Fit the speed
  per stretch of a few windows, at the cost of more analysis. This matters
  most for brief or weak plays near the 240-hit rule.
- **Adopting the second pass with common hashes skipped.** The second
  pass with the fullest 1% of posting lists left out of the search for
  candidates (`--second-pass --skip-fullest 0.01`) passes the protocol
  over four sweep seeds with its own rule, 240 hits in 3 windows: 94% fewer
  false candidates, the second pass's evidence, more confident plays under
  damage than today except combined damage, and 57-63% less search CPU at
  8,122 and 26,462 assets (experiments 0026, 0027). Emptying the lists
  instead (`--drop-fullest`) loses 4-28% of own-track evidence and is not
  worth it. To adopt: call `search_twice` on an index from
  `Index::skipping_fullest(0.01)` in `identify` and `explain`, make both the
  harness default, rerun the protocol, and spend one test evaluation: Sick
  Note (209 hits today) would most likely become possible (inference).
- **The edge of the ladder.** Recall is 100% at ±8% and 0% at ±10%, with
  nothing measured between. Measure ±8.2% to ±9%; if a deck's fader
  reaches past 8%, a few more rungs may be cheap insurance.
- **Peaks across frequency bands.** 74% of reference hashes are anchored
  at 1-4 kHz, where the development mix keeps the least: 6% at 2-4 kHz
  against 36% below 250 Hz; a clean render of the same stretch keeps 54%
  and 82% (experiment 0022). Measure true hits per 1,000 postings scanned
  by anchor band, then try peak budgets spread more evenly across bands.
  It changes the profile, so every peak record is extracted again.
- **Detection boundaries.** Boundaries are the first and last aligned hit.
  In generated mixes they lie within 0.5 s of hard cuts, and start or end
  a median 2.7 s inside crossfades and bass swaps (experiment 0020); shared
  kicks once extended a synthetic detection 26 s past its track
  (experiment 0008), and linked chance lines up to 30 s (experiment 0020).
  Try trimming weak chain ends, reporting lost true coverage beside the
  gain.
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
  saved sweep panels (seeds 2026-2029) and the normal protocol. At each
  stage, repeat the related-recordings census (experiment 0015; a
  self-match from stored peaks is cheap, experiment 0023) and check
  whether any two different recordings reach the confident rule.
  The reversed-copy proxy shows chance growing slowly up to 21,109 assets
  (experiment 0012), but shared breaks and remixes already give 130 hits
  between different recordings in this library, so the new tracks must
  include jungle, hardcore, remixes, versions and producer catalogues that
  reuse material.
- **The filter and famous breaks.** *(larger library)* As breaks such as
  the Amen and Think become common, their hashes should move into the
  fullest lists. Check whether skipping them then hides break-heavy tracks
  from the search for candidates; the second pass counts those hashes
  again, so a candidate's evidence does not depend on the filter, but
  whether it becomes a candidate does. The skipped set changes with the
  library; if candidates go missing, try weighting hits by the rarity of
  their hash instead.
- **A confidence statistic for shared material.** *(larger library)* If
  identifying and shared-material evidence overlap at scale, raising the
  hit threshold would lose brief plays without resolving the ambiguity.
  It has started: the second pass measures the passage the Clockwork remix
  shares with the original at up to 119 hits, which set its rule at 240
  (experiment 0026).
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

## Evaluation

- **More development mixes.** Mixes of existing library tracks, especially
  with brief plays and long blends, would test what the single development
  mix cannot. Agree each mix's role before looking at its results. The
  held-out test mix has three evaluations left (ADR 0004). *(larger
  library)* Each new genre needs its own development and held-out mixes;
  check the speed range those DJs used.

## Engineering

- **Before publishing.** Check that the history holds no audio from
  `corpus/` (`git log --all --stat`). The owner keeps the library file
  names in `docs/` as they are (2026-10-06).
- **GitHub Actions.** Run `scripts/check.sh` on push. FFmpeg must be
  installed in the runner for the codec tests.
