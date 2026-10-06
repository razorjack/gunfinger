# Roadmap

Work deliberately left out of the proof of concept, roughly in the order the
real project will need it. Each item links to the decision or measurement
that motivates it where one exists.

## Library and storage

- **NAS scanning with root-relative paths.** Asset identity is already the
  path relative to the library root. Missing: tolerance for slow `stat`
  calls, reconnects, and a library identifier in the peak store so two
  libraries cannot share a peak directory by accident.
- **Database.** Not needed so far: the peak store is one file per asset and
  the index is rebuilt from it in 0.4 s. Revisit when detections, owner
  edits or the Track/AudioAsset model need to be stored.
- **More than 32,768 assets.** The posting layout addresses 32,768 assets;
  the owner may grow the collection to about 100,000 tracks with jungle and
  breakbeat hardcore. Shards, separate libraries per genre and other layouts
  are compared in ADR 0007, with the measurements that should decide. Not
  decided.
- **On-disk index.** The index is rebuilt in memory from the peak store on
  every run. The recommended layout for the next step is in
  `docs/adr/0005-index-layout.md`.
- **The Track/AudioAsset model.** An asset is a file. The owner thinks in
  tracks (a recording, possibly with several rips, edits and remixes).
  Duplicate clusters (`gunfinger-eval clusters`) are the evaluation's view of
  this; the product needs its own, editable model.

## Command line

- **A full-screen TUI.** `review` steps through a report by ear in a line
  loop. Browsing detections against the mix's waveform would need a terminal
  UI dependency; worth it only if `review` proves too limited.
- **Owner edits.** Confirming, rejecting or renaming plays in a report, and
  keeping those edits when the mix is identified again.

## Matching

- **Behaviour at scale (the best next task).** The confidence rule (200
  hits, 3 windows) and the possible tier (60 hits) were calibrated against
  262 tracks. With 25,000 tracks there are about 95 times as many postings,
  so both chance alignments and lookup time grow. Measure the strongest
  false candidate and query time against library size (a larger real
  library, or the current one padded with unrelated music) before trusting
  the thresholds at that size (experiments 0004, 0005, 0006).
  `docs/calibration.md` lists the procedure and what to change. The proxy
  with reversed copies (experiment 0012) shows chance alignments growing
  slowly up to 21,109 assets; shared breaks and remixes, which the proxy
  lacks, need a larger real library.
- **Evidence summed across a play.** Plays (ADR 0006) show Fibre Optix
  "Sin", faded out for a station insert, as one possible play of two
  segments (168 hits). Experiment 0011 measured the null offline: summing
  every segment is unsafe (chance sums reach 125 hits), summing segments of
  at least 60 hits gave no false group, but no measured track would gain.
  Revisit when a missed play's possible segments sum past 200.
- **Key lock by default.** `identify --playback both` finds key-locked
  plays (experiment 0010) at twice the search time. Making it the default
  needs the full protocol with both ladders and an idle-machine timing.
- **Robustness gaps.** Experiment 0009 found no false identification under
  32 kinds of damage. Still untested: scratching, doubles (two copies of a
  record played together), long blends of more than 30 s, and speeds
  beyond ±8% (nothing is found there by design).
- **Timeline polish.** Detection boundaries are the first and last aligned
  hit; they are approximate and often extend into the neighbouring tracks'
  overlap.

## Engineering

- **GitHub Actions.** Run `scripts/check.sh` on push. FFmpeg must be
  installed in the runner for the codec tests.
