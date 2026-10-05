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
