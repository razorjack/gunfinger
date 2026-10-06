# Roadmap

Work deliberately left out of the proof of concept, roughly in the order the
real project will need it. Each item links to the decision or measurement
that motivates it where one exists.

## Library and storage

- **Configuration.** An XDG location (`$XDG_CONFIG_HOME/gunfinger/config.toml`)
  for the library root, the peak directory and defaults such as `--jobs`.
  Today everything is a flag or an environment variable.
- **NAS scanning with root-relative paths.** Asset identity is already the
  path relative to the library root. Missing: tolerance for slow `stat`
  calls, reconnects, and a library identifier in the peak store so two
  libraries cannot share a peak directory by accident.
- **Prune safety and a failed state.** `index` never deletes records. A
  `prune` command must refuse to act when the library root is unreachable
  (an unmounted NAS looks like an empty library). Failed and too-long assets
  should be recorded, so they are not decoded again on every run, and retried
  when their size or mtime changes.
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

- **`doctor`.** Check FFmpeg and ffprobe versions, the peak store's
  consistency, records whose source has vanished, and stale profiles.
- **CLI polish.** Progress bars, `--quiet`, colour, shell completions, a
  timeline view of a mix, export to cue sheets and tracklist formats.
- **TUI.** Browsing detections against the mix's waveform.

## Matching

- **Behaviour at scale (the best next task).** The confidence rule (200
  hits, 3 windows) and the possible tier (60 hits) were calibrated against
  262 tracks. With 25,000 tracks there are about 95 times as many postings,
  so both chance alignments and lookup time grow. Measure the strongest
  false candidate and query time against library size (a larger real
  library, or the current one padded with unrelated music) before trusting
  the thresholds at that size (experiments 0004, 0005, 0006).
  `docs/calibration.md` lists the procedure and what to change.
- **Evidence summed across a play.** Plays (ADR 0006) show Fibre Optix
  "Sin", faded out for a station insert, as one possible play of two
  segments (168 hits). Letting a play's summed evidence reach `confident`
  would need a measured null for sums across gaps; requiring the segments to
  agree on speed and track position would make chance sums rarer.
- **Position in the track.** A detection reports where it lies in the mix,
  not which part of the track was played. The line's offset gives the track
  position directly. Reporting it would show edits and intros, and how far a
  needle skip jumped (The Pulse in the test set skipped at 36:17 and was
  found as two chains).
- **Key-locked (pitch-preserved) sets.** Digital DJs often change tempo
  without changing pitch. The speed ladder assumes the turntable model
  (pitch and tempo together). Key lock needs a time-stretch ladder or hashes
  invariant to time scaling only.
- **EQ and blend robustness.** No work has gone into heavy EQ, filtering,
  scratching, doubles or long blends beyond what the chained-window design
  gives for free.
- **Timeline polish.** Detection boundaries are the first and last aligned
  hit; they are approximate and often extend into the neighbouring tracks'
  overlap.

## Engineering

- **GitHub Actions.** Run `scripts/check.sh` on push. FFmpeg must be
  installed in the runner for the codec tests.
