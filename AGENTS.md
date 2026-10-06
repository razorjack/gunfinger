# AGENTS.md

Gunfinger is a local Rust CLI that identifies tracks from a personal collection
inside DJ mixes (jungle and drum & bass, vinyl sets where pitch and tempo move
together within about ±8%). Unknown audio must stay unknown: a confident wrong
identification is worse than a miss.

## Before changing a decision

Read, in this order:

1. `docs/brief.md`: the owner's brief. Items tagged [FIXED] are not up for
   debate. Never edit this file.
2. `docs/status.md`: current state, exact commands and numbers.
3. `docs/adr/`: the decision records. Change a decision only with a new ADR
   that supersedes the old one.
4. `docs/experiments/`: what has been measured, including what lost.
5. `docs/calibration.md`: every number chosen against this library, what it
   rests on and what to change when the library grows.

## Repository rules

- `corpus/` is private, copyrighted audio. It is read-only and never committed.
  `corpus/.gitignore` (containing `*`) is the only file written there. Check
  `git status` before every commit and never use `git add -A` blindly.
- All derived data (peaks, generated audio, logs, reports) lives in `work/`.
  Evaluation JSON copied into `docs/experiments/data/` holds file names only.
- Branch `master`. No remotes, no pushing.
- `scripts/check.sh` is the quality gate (fmt, clippy with `-D warnings`,
  tests). Keep it green.
- Run every corpus experiment with `--release`.
- Each experiment gets `docs/experiments/NNNN-slug.md` (hypothesis, change,
  command, measured result, conclusion; at most 40 lines), written before the
  next experiment starts.

## Crate boundaries

- `gunfinger-core`: decoding, peaks, peak store, hashing, index, matching.
- `gunfinger`: the CLI.
- `gunfinger-eval`: development harness (manifests, sweep, scoring).

The core follows the pipeline: `decode` (FFmpeg) → `spectrogram` → `peaks` →
`store` (the peak store, source of truth) → `hash` → `index` → `speed` (the
ladders of turntable and key-locked rungs) → `search` (`lines` per window,
`chains` across windows) → `confidence` → `plays` (segments of one asset
grouped, same-audio plays merged). `profile` holds the front-end
parameters; changing one invalidates every peak record. Around it:
`library` finds the audio files, `indexing` brings the peak store up to
date (and remembers files that failed or are too long), `parallel` runs one
item per worker thread, `timecode` parses and formats times.

In the CLI, `identify` searches and builds a `report` (the JSON report),
`explain` shows the evidence at one moment of a mix, and `show`, `listen`
and `review` read a saved report. Reports render through `output`
(`table`, `timeline`, `export`). `config` resolves
flags, environment and the configuration file; `console` and `style` own
messages and colour; `catalog` loads the index from the peak store;
`survey` compares the peak store with the library for `doctor` and
`prune`; `names` reads track names from tags; `playback` picks the ladder.

The harness adds `sweep`, `scan`, `calibrate`, `regress`, `robust`
(transformed excerpts), `synthetic` (the scale proxy), `clusters` and
`related` (self-match of the library) on top of `manifest` and `scoring`.
Python scripts that summarise harness reports for experiments live in
`scripts/analysis/`.

`gunfinger-eval` depends on `gunfinger-core`, never the reverse. The CLI and the
core never see a manifest, a set name or a track title. No per-track, per-set
or filename-derived logic anywhere.

## House style

Idiomatic Rust for a reader with a Rubyist's intolerance of accidental
complexity. Minimise conceptual complexity, not line count.

- Code reads top to bottom in domain vocabulary, in small functions whose names
  reveal intent. Types carry meaning where they clarify (`AssetId`,
  `SpeedRatio`, `Peak`, `PairHash`, `Posting`, `Evidence`); no wrappers for
  ceremony.
- Ownership is obvious. No `clone()` to silence the borrow checker, no lifetime
  gymnastics to save a harmless clone, no reflexive `Arc<Mutex<_>>`, no
  `Box<dyn Trait>`, single-implementation traits or generics without need.
- Enums over boolean flags. `?` and clear `Option`/`Result` flows. No
  `unwrap`/`expect` on production paths, no panics for user or environment
  errors.
- DSP code uses explicit loops that visibly follow the algorithm. Comments
  explain why, the maths, invariants and paper provenance, never syntax.
- Side effects sit at the edges (FFmpeg, files, CLI); the core is pure where
  practical.
- Errors are actionable: what failed, why it matters, what to do.
- Tests optimise for clarity, not DRY.
- Optimise only from measurement.
- No dead code. An experiment that loses is deleted from the tree; its result
  lives in `docs/experiments/`.
- Synchronous Rust only. No async runtime, database, network service, neural
  model, GPU, SIMD or `unsafe` (forbidden at workspace level).
- Gunfinger owns its fingerprinter. No third-party fingerprinting crate and no
  code copied from existing implementations.

## Index-size rule

The index must be space-efficient without losing fidelity. Every layout
decision reports bytes per posting and justifies each stored field. Choose the
leanest configuration whose sweep and development results are no worse than
the best. `gunfinger stats` reports the measurements.

## Evaluation rules

- Before committing a change that can affect detection, run
  `gunfinger-eval regress <baseline>` (after `gunfinger-eval baseline
  <name>` on the previous commit) and record what changed. Search is
  deterministic, so "identical" means bit for bit.
- Only `confident` detections count. One frozen confidence rule for every
  input. `possible` plays are shown to help and scored apart; they never
  identify a track or count as wrong (ADR 0006).
- Duplicate clusters come from library audio alone, never from set results.
- The sweep must clear its diagnostic bar (≥95% recall within ±5%, zero wrong
  answers) before any set is scanned.
- `stakka-skynet-knowledge` is the development set. Iterate freely on it.
- `ed-rush-optical-essential-mix` is the held-out test set. It is evaluated
  only after the `poc-freeze-1` tag, at most five times in total, and every
  evaluation is logged in `docs/experiments/test-set-ledger.md`.
- Never edit a manifest. Suspected ground-truth errors go to "for the owner to
  check" and still count as wrong.
