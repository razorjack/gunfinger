# Roadmap

Work deliberately left out of the proof of concept, roughly in the order the
real project will need it. Each item links to the decision or measurement
that motivates it where one exists. Items marked *(larger library)* need
more real tracks; the rest can be done with the current library. Figures
marked as inferences were derived, not measured.

## Library and storage

- **Search memory at scale.** The search now runs a block of 12 windows
  on every rung at a time and merges each block's lines once every rung
  has searched it, with identical detections (experiment 0029). The
  development scan (56 minutes) peaks at 2.9 GB at 8,122 assets and 7.7
  GB at 26,462 (skip at 240: 1.4 and 3.8 GB). What is left at 26,462: the
  index (3.0 GB), the merged lines of the whole mix (1.4 GB), the
  workers' hit buffers (about 2 GB) and the final sort of the lines.
  16-byte hits and sorting the lines without a half-size copy would take
  off about 1.2 GB, also with identical detections; not done.
- **On-disk index.** The index is rebuilt in memory from the peak store on
  every run. The recommended layout is in `docs/adr/0005-index-layout.md`.
  Audit its widths first: its `u32` byte offsets into delta-coded lists
  address 4.3 GB, and 100,000 tracks would need about 9.3 GB (ADR 0007);
  posting offsets and the 16 asset bits have limits of their own. At NAS
  scale every `identify` and `explain` builds the index from the store
  in 65-72 s at 32,905 records (experiment 0063; 56.5 s at 27,042),
  most of the time of a short `explain`. Compare loading a saved index
  with rebuilding it first, and measure compression separately.
- **More than 65,536 assets.** *(first step done in session 9, ADR
  0010)* The NAS store passed 32,768 records on 2026-10-09 (32,905), so
  a posting now holds 16 frame bits and 16 asset bits: 65,536 assets,
  tracks up to 17:28 (default track limit 17:00). 32,905 is 50% of
  that; at 65,536 assets of the NAS mean length the index would hold
  about 1.84 billion postings, 7.4 GB in memory, before the search's
  own memory (4.7 GB peak at 32,905 records, experiment 0063). The next
  limit: past 65,536 assets, shard by asset range (ADR 0007, option A),
  each shard with this layout; memory may call for loading shards one at
  a time before addressing does. The owner may grow the collection to
  about 100,000 tracks with jungle and breakbeat hardcore. Shards,
  separate libraries per genre and other layouts are compared in ADR
  0007. Shards solve addressing and can bound memory, but searching
  every shard does not reduce the work or the chances of a false
  match. *(larger library)* Decide collection boundaries from
  evidence: the same queries against drum & bass alone, hardcore alone and
  their union, measuring false candidates, missed cross-genre matches and
  cost, with one confidence rule throughout (ADR 0007, measurement 3).
  Searched shard by shard, the default matcher must still choose the
  fullest 1% of posting lists it skips over the whole searched library:
  chosen per shard, a file's candidates would depend on which shard
  holds it.
- **Library identity and relocation.** Asset identity is already the path
  relative to the library root. Missing: a library identifier in the peak
  store, so two libraries cannot share a peak directory by accident; an
  explicit way to move a library or its peak store; on a NAS, indexing
  throughput, slow `stat` calls and reconnects.
- **Library ignore file.** *(owner's proposal, 2026-10-09; done in
  session 8, ADR 0009)* A track of a DJ-mixed CD already blends into the
  next one; indexed, it causes false joins and wrong answers (the mixed
  CD's "Synthesis (Remix)", experiments 0049-0053). `.gunfingerignore` at
  the library root, a subset of `.gitignore` syntax with literal square
  brackets and no negation, leaves files and folders out of every command
  that scans the library; `doctor` counts the audio files per pattern and
  the records of ignored files apart from those of gone files, and `prune
  --yes` deletes both. Store-only runs see ignored files until they are
  pruned. The owner's first file on the NAS leaves out Dangerous Drums
  Volume 2 and The Creeps' CD2 (20 and 13 audio files); the owner runs
  `prune --yes` with the NAS configuration, which deletes their 33 peak
  records and 33 tag notes. Open: `2002 - CPT003 - Kemal vs. Rob F &
  Impulse - 256`, the third candidate in the owner notes ("Mixed-CD
  tracks in the library"), is not listed; and the clusters' verdict
  links must leave out ignored files (below, "More development mixes",
  step 2). Session 9: the owner pruned, then indexed a scene copy of
  Dangerous Drums Volume 2
  (`__full_scene/dnb_scene - 2000 - Part 2/2000-08/va-dangerous_drums_2-udfrcd002-2000-sour/`),
  which the root-anchored pattern does not cover, and three mixed CDs
  in the new folders give the 2003 mix a confident wrong title
  (experiment 0063). `scripts/analysis/folder_edges.py` scores each
  folder's track boundaries from the peak records; 33 folders have at
  least 75% joined boundaries, 30 of them new (experiment 0065,
  `work/s9/mixed-cd-candidates.tsv`), a listening list for the owner.
- **Database.** Not needed so far: the peak store is one file per asset and
  the index is rebuilt from it in 0.5 s. Revisit when detections, owner
  edits or the Track/AudioAsset model need to be stored.
- **The Track/AudioAsset model.** An asset is a file. The owner thinks in
  tracks (a recording, possibly with several rips, edits and remixes).
  Duplicate clusters and related recordings (experiment 0015) are the
  evaluation's view of this; the product needs its own, editable model,
  shaped by how the owner uses `review`.

## Reports and command line

- **Owner edits.** Confirming, rejecting or renaming plays in a report, and
  keeping those edits when the mix is identified again. `listen` and
  `review` work with real audio output (owner, 2026-10-06). The owner's
  ear now settles most open questions: wrong tags, versions, cluster
  joins and useful possible plays. A first step smaller than a TUI is a
  pair review: play two library files aligned at their fitted speed
  (`gunfinger-eval pair` finds the alignment), show where they agree and
  where they differ, and write the verdict to `docs/pair-verdicts.txt`.
  Decisions stay apart from measured evidence, survive identifying a mix
  again, and keep the original asset names visible. Session 10 built the
  first step as files: `scripts/analysis/listening_pack.py` writes, per
  pair, a sheet and FFmpeg clips (each file in turn, and both aligned in
  stereo) under `work/listening/`, in listening order; the owner writes
  the verdicts by hand.
- **No record inside the track length range.** *(done in session 10)*
  With a store whose only records lie outside `--min-track` and
  `--max-track`, `stats`, `identify` and `explain` said "no indexed
  assets ... run `gunfinger index` first", though the files are indexed
  and only the range leaves them out (seen in session 9 with an
  18-minute file and the 17:00 default). They now say that the range
  leaves out every file with a peak record, how many and how long, and
  how to widen it.
- **Titles of mislabelled files.** A play takes the title in its best
  file's tags, so a mislabelled file gives a wrong title on a right
  match: track 2 of the 2003 mix is printed as "Future Cut - Sex Drive"
  (experiment 0061). Plays of other files over the same passage often
  carry the right title (there, the INFRA 012 upload). Printing the other
  titles when the files matching one passage disagree would show the
  conflict without guessing which title is right; owner edits and the
  Track/AudioAsset model would settle it.
- **A full-screen TUI.** `review` steps through a report by ear in a line
  loop. Browsing detections against the mix's waveform would need a terminal
  UI dependency; worth it only if `review` proves too limited.

## Matching

- **A minimum span instead of 3 windows.** Whether a 15-20 s play is
  confident depends on where the 10 s windows fall: 76 of 160 grid
  positions at 15 s, 148 at 20 s; 200 hits over at least 10 s between the
  first and last hit makes all of them confident and changes nothing in
  the sweep or the development scans (experiment 0020, offline). It lowers
  the shortest identification from about 25 s to about 11 s, which is the
  owner's call. The default matcher's link rules (experiment 0030, ADR
  0008) keep chance lines from lengthening the span; measure the minimum
  span with them (the grid's 15 and 20 s plays: 67 and 146 of 160
  confident under the default, experiment 0047). The span from the
  first to the last hit counts gaps as evidence; compare it with the
  time that hits support (for example, the seconds holding aligned
  hits), including sparse alignments and repeated breaks. `pair` now
  reports that supported time; for whole files it orders pairs
  differently from coverage (experiment 0057). Keep the hit
  threshold fixed to isolate the change. Success: fewer level changes
  caused only by where a play falls on the window grid (silence put
  before the same audio should not change its level), more brief plays
  confident, and no new confident error on the shared-material challenge
  set below, which comes first.
- **Speed that wanders within a play.** The second pass analyses a play at
  one fitted speed; Star Trails plays 0.1-0.3% above it in many windows
  and loses 15% of its hits (experiment 0024), and within-play speed
  varies by ±0.2% in the development mix (experiment 0022). The opt-in
  `--speed-per-stretch` fits each stretch of 3 windows: Star Trails +11%,
  a 2% pitch ride +9%, no measurable CPU, no level changes in any set
  (experiment 0032). Wow is too fast for it. Adopt if a real play near
  the 240-hit rule needs it. Under the default matcher (ADR 0008) it is
  one field of `Matcher::Fitted`'s options; rerun the protocol.
- **The edge of the ladder.** Recall stays 100% to ±8.4% and is gone at
  ±9% (experiment 0033). Three extra rungs at each end (`--extra-rungs
  3`) recover every speed to ±9% for 14% more search CPU and change
  nothing else measured. The limit applies to the mix's speed relative
  to the indexed file, not to the deck's pitch. Copies of one recording
  differ in speed: of the 872 pairs of files joined in the NAS clusters
  (experiment 0049), 180 differ by more than 2%, 17 by more than 4%,
  and one "Speed Up" upload by 7.8%. A play at +5.65% (the test mix's
  fastest) against a copy 4% slow is at +10%, out of reach when that
  copy is the only one indexed; which copy runs at the record's true
  speed is unknown. Measure before deciding: a sweep that leaves out the
  excerpt's source file and its identical copies but keeps its other
  rips, so that misses beyond the ladder and misses from lost evidence
  can be told apart. Then choose between a wider search, a targeted
  one, or none. Measured in session 7 (`sweep --other-rips`, experiment
  0059, 1,305 queries at NAS scale): recall holds to 9% of relative
  speed and is gone from 10.2%; `--extra-rungs 3` adds 9.2-9.5% only.
  Inside 9% every miss is a weak copy (lossy uploads, one other vinyl
  rip) found below the rule. Of the 233 NAS clusters, 7 span more than
  4% in speed (Falcon - The Stand 11.5%, its "Speed Up" upload against a
  slow rip). A play is lost only when the deck's speed and the
  remaining copy's offset add up past 9%, so the choice depends on how
  far the owner's decks go beyond about ±5%. Replay speed also changes
  by accident when analogue media is digitised (Six & Leman 2014, §1).
- **Peaks across frequency bands.** Anchors at 2-4 kHz are 64% of the
  postings the development mix's plays look up and 21% of their true
  hits. A profile whose peak neighbourhoods widen with frequency (variant
  a, experiment 0034) has 9% fewer postings, gives the mix 34% more true
  hits and raises the margins under both matchers, but related records
  (remixes, the same artist) reach the possible tier more easily. The
  owner decides; it means extracting every peak record again, and one
  test evaluation should confirm it. Measure its search cost at scale.
  Before extracting the NAS records again, test it under the default
  matcher on the shared-material challenge set below. Wang (2003, §2.1)
  picks peaks by a density criterion so that the time-frequency plane
  is covered evenly; the band profile asks the same of each band.
- **Detection boundaries.** Boundaries are the first and last aligned hit.
  In generated mixes they lie within 0.5 s of hard cuts, and start or end
  a median 2.7 s inside crossfades and bass swaps (experiment 0020); shared
  kicks once extended a synthetic detection 26 s past its track
  (experiment 0008), and linked chance lines up to 30 s (experiment 0020).
  Trimming weak chain ends (`--trim-ends`) removed the overshoot but cut
  2.6 s of true play per second removed (experiment 0031); the link rules
  of 0030 fix the overshoot at a tenth of the cost. Crossfade edges remain
  a median 2.7 s inside the truth. Gunfinger reports the matched
  interval, not the query window: in a comparison of fingerprinters on
  DJ mixes, crediting each answer to its matched interval instead of the
  whole 20 s query raised specificity from 0.56 to 0.80 (Sonnleitner et
  al. 2016, §6). Per-second scoring (Evaluation) would measure how
  accurate the boundaries are.
- **Where search time goes.** Measured on an idle machine (experiment
  0028): at 262 assets 80-88% of search CPU analyses the query on 82
  rungs; at 26,462 assets 77-87% sorts each window's hits by asset and
  offset (the offset histogram of Wang 2003, §2.3, found by sorting).
  Any correct sort gives identical lines, so grouping hits by
  asset before sorting each asset's few hundred would keep detections
  identical; not tried. Skip at 240 is 3.5× cheaper at 31,964 assets.
  Profile the default matcher at NAS scale first. This comes before any
  change of hash design; `regress` must show identical detections.
- **Behaviour at scale.** *(larger library)* The confidence rule (240
  hits in 3 windows for the default matcher, 200 for `--single-pass`) and the possible tier
  (60 hits) were calibrated against 262 tracks. Measured since at 1,200,
  3,200, 9,200 and 27,000 assets of the owner's collection (experiments
  0049-0052): recall holds, the confident rule holds against everything
  but a few pairs for the owner's ear, and chance grows past the
  possible tier from about 9,000 assets. Wang (2003, §2.3.1) sets the
  threshold from the score distribution of the highest-scoring wrong
  track, which depends on the number of tracks. Recalibrate at each new size
  (`docs/calibration.md`): strongest false candidates, weakest true
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
  (experiment 0026). The band profile of experiment 0034 makes it more
  pressing: related records (remixes, the same artist) reach up to 50
  hits from audio not in the index, near the possible tier.
  At NAS scale it is the main open accuracy question (experiments 0051,
  0053). The owner has judged the three pairs of the listening list one
  recording each and The Nine's Evol Intent VIP a different recording
  (2026-10-09). The strongest false candidate is therefore The Nine
  against that VIP at 197 hits; the VIP plays the original's first 3:34
  unchanged, then a 135 bpm slowdown. The rule of 240 is only 1.22×
  above it, where the calibration register asks for about 2×. Raising the
  rule to about 400 would lose brief plays without resolving the
  ambiguity. The held-out mix shows the cost already: Sick Note, a true
  play, is possible at 233 hits, 7 below the rule (test-set evaluation
  3), so true plays and shared material now meet in the same range of
  hits. Method: a challenge set of originals, VIPs, remixes and
  tracks sharing breaks, each pair confirmed by the owner's ear;
  excerpts from shared and from distinctive passages at several
  durations, with the played recording in the index and left out.
  Compare the current rule with measures such as distinct anchors,
  support spread over time, and evidence outside passages the library
  itself shows to be shared; these are hypotheses to test. Validate on
  recordings not used to develop them. Where the played passage cannot
  tell two versions apart, report the ambiguity instead of choosing one.
  Sample identification counts finding the sampled recording as the
  right answer; for Gunfinger it is a wrong identification, on the same
  evidence. Bhattacharjee et al. (2025) annotate Sample100 per segment
  and label drum-break samples apart from riffs. Scenarios that tell
  sustained evidence from distinguishing evidence: a short shared break
  repeated for a long time (does repetition alone reach the rule?);
  excerpts wholly inside a shared passage against excerpts reaching
  into distinctive material; the played recording and all its rips left
  out while a related remix or original stays; and, as a positive
  control, the source file left out while another rip of the same
  recording stays. Repetitive material is where fingerprinters lose
  specificity (Sonnleitner & Widmer 2016, §VI; Sonnleitner et al. 2016,
  §6). Run the challenge set before adopting anything that adds
  evidence (the items below, the band profile, a minimum span).
  Session 7 ran the four scenarios on two judged pairs (`gunfinger-eval
  shared`, experiment 0055): nothing reaches the rule; looping a shared
  passage to 240 s does not add hits, because each repeat starts a new
  alignment (summing a play's segments would change that); running into
  distinctive material adds hits only to the played file. More judged
  pairs (the owner's verdicts) would make it a challenge set.
- **Evidence beyond exact pair hashes.** The second pass counts only
  exact pair-hash matches at the fitted speed (`search/refine.rs`).
  Peaks can survive where their hashes do not: a peak half a bin away
  falls into the neighbouring bin, and a changed neighbour changes which
  pairs form (Six 2021, §2). Three experiments in the harness, in this
  order, with candidate discovery and the confidence rule unchanged
  until they show separation:
  1. A peak verifier after Qfp's (Sonnleitner & Widmer 2016, §VI-C):
     for each candidate, look for its reference peaks in the query at
     the fitted alignment, within a small time and frequency tolerance,
     and record the distinct reference peaks found, their share of the
     reference peaks in the aligned passage, and how they spread over
     time. Count reference peaks found in the query, not query peaks
     matched: a blend adds query peaks. In a comparison on DJ mixes,
     Qfp's verification raised specificity on the Mixotic mixes from
     0.647 to 0.927 while accuracy went from 0.889 to 0.876 (Sonnleitner
     et al. 2016, table 2); those are its figures on its data. Test weak
     true detections, unrelated false candidates and confirmed
     shared-material pairs separately; the combined-damage condition
     offers many candidates (skip at 240: 65 of 80 possible or better, 4
     confident; experiment 0027). Success: better separation of true and
     false candidates on recordings not used to develop it. Shared
     passages may verify as strongly as the played recording.
     Done in session 7 as `gunfinger-eval --verify` (tolerance ±1 bin,
     ±1 frame; experiments 0054, 0056, 0058). At equal hits its share
     separates no better than hits (AUC 0.953 against 0.950 at 262
     tracks) and worse against related records: weak blend partners
     (0.16-0.24) verify like shared passages (0.11-0.41). Kept as a
     diagnostic; no rule. Its share could order a listening list.
  2. What exact quantisation loses after fitting: on the same
     alignments, count exact pair hashes, pair hashes that allow a
     neighbouring value in each component (as Panako 2.0's near-exact
     hashing does, Six 2021, §2), and the verifier's peaks, each
     correspondence once. Stay within the candidate's reference passage,
     so that accuracy is not mixed with the cost of tolerant lookups
     over the whole index. In Qfp, narrowing the tolerance (likened by
     its authors to quantised hashes) cut Mixotic accuracy from 0.876 to
     0.570 and raised specificity to 0.987 (Sonnleitner et al. 2016,
     §6). The comparison separates features that land in neighbouring
     bins from features or pairings that are gone; tolerant hashing
     recovers only the first.
  3. Denser query peaks around candidate passages, the reference store
     unchanged: Qfp analyses queries denser than references, keeping its
     database small (Sonnleitner & Widmer 2016, §IV-B). Experiment 0004
     changed the density of both at once, so it does not answer this.
     Use the extra peaks in the verifier first, so that the pairs the
     first pass forms stay as they are; measure peaks recovered under EQ
     and blends, accidental agreement, and CPU.

  After step 1, most reference peaks are found in the query (median
  share 0.84 for the source file, 0.65 for another upload, experiment
  0058); how many of them the exact hashes miss is what step 2
  measures. Since more evidence did not separate true from related
  candidates at equal hits, steps 2 and 3 matter more for recall than
  for separation: the 57 misses within 9% of relative speed in
  experiment 0059 are weak copies found at 25-239 hits, below the rule.
  Measure those copies and the shared-material scenarios (experiment
  0055) together, since both would gain.
- **Longer hashes.** Triplet hashes would scan 25-79 times fewer postings
  but lose about half their evidence under heavy damage (experiment 0014,
  estimated offline, no search). Revisit if profiling on a real large
  library shows lookups still dominate after simpler changes; compare
  recall under damage, related-recording errors, memory including the
  lookup structure, and wall time. Triplets come from Panako (Six &
  Leman 2014), which makes them invariant to speed with time ratios;
  experiment 0014's keep exact coordinates. Qfp uses quads (Sonnleitner
  & Widmer 2016). On DJ mixes Panako's triplets were more specific than
  Wang-style pairs (0.746 against 0.487 on club mixes, 0.349 against
  0.255 on Mixotic), at lower accuracy (Sonnleitner et al. 2016, table
  2). Each extra peak a hash needs lowers its chance of surviving (Wang
  2003, §2.2, for pairs).
- **Evidence summed across a play.** Plays (ADR 0006) show Fibre Optix
  "Sin", faded out for a station insert, as one possible play of two
  segments (168 hits). Experiment 0011: summing every segment is unsafe
  (chance sums reach 125 hits), summing segments of at least 60 hits gave
  no false group, but no measured track would gain. Revisit when several
  independently reviewed missed plays have strong, compatible segments that
  together pass 200, and test a restricted rule against related recordings
  and long unknown passages. A looped shared passage is the case to beat:
  the passage Clockwork shares with its Stakka remix, looped to 240 s,
  gives the other recording 13-22 detections of up to 144 hits each,
  which summed would be confident (experiment 0055). A rule that counts
  only segments advancing through the reference would leave the repeats
  out (proposal, not measured).

## Evaluation

- **The owner's verdicts on borderline pairs.** At NAS scale 28 pairs of
  files cover 40-80% of each other (experiment 0049), among them China
  Cup against its Prototype upload, whose detections are most of the
  false confident ones left. `docs/pair-verdicts.txt` takes the owner's
  verdicts; rerun `clusters --from-peaks` and the NAS-scale protocol
  after adding them. `clusters` stops when a chain of joins through
  other files contradicts a `different` verdict, and names the chain;
  with the NAS store it checks the last run's pairs first, before an
  85-minute search. A chain only the new search finds still stops the
  run after it, and the search is lost: in session 10 the PRO012 rip of
  Sonar, indexed since session 6, joined the revision (467 hits, 1.3
  hits per second) after 2 h 16 min. `--cut-sparsest` (session 10, opt
  in) cuts each such chain's sparsest measured join and lists it for
  the owner instead. Still to do: keep a stopped run's pairs and
  cluster them again without searching. *(larger library)*
- **Clustering precision.** Most of session 6's gain at NAS scale came
  from joining rips the clusters had missed, so a wrong join would now
  count a false identification as correct. Coverage counts the span
  from the first to the last hit and ignores how much evidence lies in
  it: DJ Trace - Sonar joins its "Mark System Revision" at 91% with 0.7
  hits per second of aligned span, against 18.6 for the median join and
  6.5 for the 5th percentile. `clusters` now lists joins below a tenth
  of the median for the owner's ear (at NAS scale the two Sonar
  revision pairs and one Phoenix upload pair; none at 262 tracks).
  Supported time (experiment 0057): the Sonar joins hold aligned hits
  on 0.39-0.43 of the file, Phoenix's on 0.63-0.72, random joins
  0.98-1.00; China Cup and The Specialist against the uploads the
  clusters keep apart, 0.96-0.99 at two offsets. A rule on supported
  time would need the owner's verdicts to set it. Still to do: inspect
  long sparse alignments and chains through mixed-CD tracks (a mixed
  CD's track can hold the start of the next one, experiment 0058),
  and have the owner check a sample of joins, not only the pairs the
  clusters keep apart. Evaluations at scale elsewhere meet the same
  effect: with 430,000 added tracks, a copy of a played song among them
  counted as a false positive (Sonnleitner et al. 2016, §6).
- **The possible tier at NAS scale.** Unrelated chance reaches 63-70
  hits over the NAS sweeps (experiments 0040, 0041, 0051), above the
  tier of 60; on nested subsets of the NAS records it is 21-34 hits up
  to 3,200 assets and 64 from 9,200 (experiment 0052). Proposed: 60
  while that chance stays below 30 hits, twice it above (130-140 at NAS
  scale), which would hide the development mix's shared-material notes.
  The owner decides. *(larger library)* The jump to 64 hits is one
  coincidence (Clockwork against Simon Static - Rubba Rock), so the
  library's makeup matters as much as its size: measure at each size
  rather than build the breakpoints into a rule, and count what a
  higher tier costs (weak true plays lost, possible plays to review per
  hour of mix). Shared-material notes could keep a lower threshold than
  standalone possible plays, but only where the two library files match
  each other in that passage. Two plays overlapping in a mix, or similar
  tags, do not show shared material: chance lines overlap confident
  plays too.
- **Recall beyond the corpus recordings.** *(development half measured
  in session 10, experiment 0068: 540/540 at every speed and kind, no
  miss, held-out sources never confident; 30 wrong answers from a
  mixed-disc track and an upload pair the clusters keep apart. The
  harness reports why a miss failed. Still to do: the validation half,
  once the rules change, and the hardest conditions combined.)* The four sweep panels draw
  320 excerpts from 208 corpus files (experiment 0025). At NAS scale
  they measure those recordings against 27,000 records, not recall on
  the rest of the collection. A panel drawn from NAS records, split into
  development and validation groups by recording family before any
  result is seen, would cover other upload sources, producer
  catalogues, break-heavy tracks, mixed-CD extracts and other masters.
  Its sources need clusters first (`clusters --from-peaks` around them,
  about 85 minutes a round), or their other copies count as wrong. The
  hardest conditions have not been combined at this size: other rips
  only, brief plays and damage together (combined damage at 262 tracks
  under skip at 240: 4 of 80 confident, 65 possible or better,
  experiment 0027). The harness should also say why a query failed:
  never a candidate, too few hits in the second pass, or enough hits in
  too few windows.

- **More development mixes.** Mixes of existing library tracks, especially
  with brief plays and long blends, would test what the single development
  mix cannot. Agree each mix's role before looking at its results. The
  held-out test mix has two evaluations left (ADR 0004). *(larger
  library)* Each new genre needs its own development and held-out mixes;
  check the speed range those DJs used. The owner's 2003 mix is the second
  development set (`razorjack-2003-03-29`, experiment 0060: 12/12 at NAS
  scale). To use it in the harness:
  1. *(done in session 10)* Manifests that reference NAS files
     (`second-library/<path>`, as in `docs/pair-verdicts.txt`), valid
     with `--other-peaks-dir`; the five NAS-only tracks are now
     references, set aside at 262 tracks.
  2. *(done in session 10, experiment 0066)* Clusters around its files
     (`clusters --from-peaks --manifest <set>`), so that other rips and
     uploads count as the track.
     The owner judged the alien5ive INFRA012 upload of The Specialist,
     confident in the mix, the same recording (2026-10-09). Before the
     run: corpus-only `clusters` adds every `same` verdict as a link, so
     the NAS files in the owner's verdicts would enter the corpus
     clusters; add verdict links only between files of the libraries
     searched, and not to files their ignore files leave out. The ignore
     file exists (ADR 0009); after the owner prunes the NAS store, one
     85-minute run serves both.
  3. *(done in session 10, experiment 0067)* The set in the standard
     protocol at both sizes, with leave-outs: 12/12 at NAS scale, 7/7
     referenced and nothing possible over the 5 absent tracks at 262
     tracks, 0 wrong everywhere.
  Then the experiments it offers *(done in session 10, experiment
  0069)*: without Phantom Force's rips neither Phantom 2018 nor the
  Fracture edit becomes confident (the track is missed, with possible
  notes only); without the PHUD1 vinyl rip the start moves back from
  10:49 to 11:55.8; the Kinetic tease at 17:23 is the track's 1:11-1:51,
  possible on its own, and the track starts again from 0:15 at 18:35.
  Candidates on the NAS (experiment 0065, `work/s9/candidate-mixes.tsv`;
  no `identify` before their roles are agreed): the owner's own
  `Razor Jack - 2003-08-27.mp3` (20.7 min); 136 sets, radio shows and
  single-file mixed CDs of 1999-2000 in the scene folders; single-file
  mixed CDs with published tracklists in folders at the root (Stakka &
  Skynet's Clockwork mix CD, Andy C's Ram Raiders, DJ TeeBee's Through
  The Eyes Of A Scorpion); 96 radio shows and sets in `sety/`. The root's
  `EM_1999_02_07_-_Ed_Rush_Optical.mp3` is by name the broadcast of the
  held-out test set: keep it out of development use.
- **Public DJ-mix datasets and per-second scoring.** Mixotic (10
  Creative Commons techno and house mixes, 723 reference tracks,
  approximate song borders) and UnmixDB (mixes generated
  beat-synchronously from the Mixotic tracks, with exact starts, cue
  points and speed factors) would give evidence without spending a test
  evaluation (`docs/references.md`). They are another genre, so they test
  how the design generalises, not the jungle calibration, and they need
  their own library and peak store. UnmixDB's time scaling comes from
  SoX; check which variants resample and which keep the pitch. UnmixDB
  reuses Mixotic's tracks, so split development from validation by
  recording family across both, and run the frozen matcher on the
  validation part. Besides track recall, score per second as Sonnleitner
  et al. (2016, §5) do: correctly labelled time, confidently mislabelled
  time, silence where the playing track is not in the library
  (specificity), and boundary errors, with both tracks correct during a
  transition and explicit uncertainty around annotated borders. Keep the
  current scoring for continuity. These measures would also judge the
  minimum span and the time that hits support (Matching). Published
  scores do not compare with Gunfinger's track recall; compare systems
  only by running them under one protocol. Session 7 could not download
  Mixotic: the three archives are Google Drive files that answer with a
  sign-in page; a browser download is needed.

## Engineering

- **Before publishing.** Check that the history holds no audio from
  `corpus/` (`git log --all --stat`). The owner keeps the library file
  names in `docs/` as they are (2026-10-06).
- **GitHub Actions.** Run `scripts/check.sh` on push. FFmpeg must be
  installed in the runner for the codec tests.
