# AGENTS.md

Gunfinger is a local Rust CLI that identifies tracks from a personal collection
inside DJ mixes (jungle and drum & bass, vinyl sets where pitch and tempo move
together within about ±8%). Unknown audio must stay unknown: a confident wrong
identification is worse than a miss.

## The product's scope

Gunfinger is fired and forgotten (ADR 0017): the user
runs it, reads the result and is done. Nobody reviews the output, so
precision comes first; then listing every matching file and flagging
disagreeing titles without choosing one; then finishing unattended runs
reliably; then speed. The user owns the tags, and titles are only as
good as the tags indexed. Do not propose editing workflows (confirming,
rejecting or renaming plays, decisions kept across runs), a stored model
of tracks above files, tag fixing or library cleaning, or ways to pick
one of several titles. The listening tools (`audition`, `pair-review`)
are development tools for evaluation ground truth, not product features.

## Before changing a decision

Read, in this order:

1. `docs/brief.md`: the brief. Items tagged [FIXED] are not up for
   debate. Never edit this file.
2. `docs/status.md`: current state, exact commands and numbers.
3. `docs/adr/`: the decision records. Change a decision only with a new ADR
   that supersedes the old one.
4. `docs/experiments/`: what has been measured, including what lost.
5. `docs/calibration.md`: every number chosen against this library, what it
   rests on and what to change when the library grows.
6. `docs/references.md`: the papers and datasets behind the design and
   the open questions, with the sections that matter. Cite them by its
   short keys, such as (Wang 2003, §2.3).

## Repository rules

- `corpus/` is private, copyrighted audio. It is read-only and never committed.
  `corpus/.gitignore` (containing `*`) is the only file written there. Check
  `git status` before every commit and never use `git add -A` blindly.
- All derived data (peaks, generated audio, logs, reports) lives in `work/`.
  Evaluation JSON copied into `docs/experiments/data/` holds file names only.
- Branch `master`. No remotes, no pushing.
- `scripts/check.sh` runs the checks (fmt, clippy with `-D warnings`,
  tests). Keep it green.
- Run every corpus experiment with `--release`.
- Saved indexes (ADR 0011, ADR 0014) are checked against the files,
  profile and `hash::design()`, not the code. A change to how postings
  are made from peaks changes `hash::design()`'s tag, or its runs use
  `--saved-index rebuild`.
- Each experiment gets `docs/experiments/NNNN-slug.md` (hypothesis, change,
  command, measured result, conclusion; at most 40 lines), written before the
  next experiment starts.

## Platforms

Gunfinger is cross-platform: the CLI, the harness and the listening tool
support at least macOS and Linux (ADR 0020).

- Platform-specific behaviour goes through one function per need, with a
  branch for each supported platform, and each branch is tested. In the
  harness the function takes a `platform::Platform` and returns the
  command (`audition::desktop`, `robust::speech_synthesis`), so both
  branches run in tests on either system.
- Messages and docs that name a tool or a package manager name one for
  each platform (`decode::INSTALL_FFMPEG`).
- `scripts/check.sh` also runs clippy for the other platform's target
  when that target's standard library is installed, and prints a line
  when it skips. With rustup: `rustup target add x86_64-unknown-linux-gnu`
  on macOS, `rustup target add aarch64-apple-darwin` on Linux. The
  development Mac has Homebrew's Rust and no rustup; Homebrew's keg-only
  rustup holds a toolchain of Homebrew's version (`rustc --version`)
  with the target. Run the checks this way before committing a change to
  platform-specific code:

  ```sh
  brew install rustup    # once
  "$(brew --prefix rustup)/bin/rustup" toolchain install 1.98.0 \
    --profile minimal -c clippy -c rustfmt -t x86_64-unknown-linux-gnu  # once
  PATH="$(brew --prefix rustup)/bin:$PATH" RUSTUP_TOOLCHAIN=1.98.0 scripts/check.sh
  ```

  Inside the repository, rustup commands that show the active toolchain
  (`rustup show`, `rustup toolchain list`) install `stable` first,
  because of `rust-toolchain.toml`.

## Crate boundaries

- `gunfinger-core`: decoding, peaks, peak store, hashing, index, matching.
- `gunfinger`: the CLI.
- `gunfinger-eval`: development harness (manifests, sweep, scoring).

The core follows the pipeline: `decode` (FFmpeg and FFprobe as child
processes, stopped when they write nothing for two minutes: `decode::watch`,
ADR 0018) → `spectrogram` → `peaks` →
`store` (the peak store, source of truth) → `hash` → `index` → `speed` (the
ladders of turntable and key-locked rungs) → `search` (`lines` per window,
`chains` across windows) → `confidence` → `plays` (segments of one asset
grouped, same-audio plays merged). `profile` holds the front-end
parameters; changing one invalidates every peak record. Around it:
`library` finds the audio files, leaving out what the
`.gunfingerignore` at the root lists (`library::ignore`, a subset of
`.gitignore`; ADR 0009) and counting what vanishes while it is listed
(ADR 0015), `indexing` brings the peak store up to
date (and remembers files that failed or are outside the track length
range), `tags` holds the artist, title and album the store keeps for each
file, `parallel` runs one item per worker thread, `timecode` parses and
formats times.

In the CLI, `identify` searches and builds a `report` (the JSON report),
`explain` shows the evidence at one moment of a mix, and `show`, `listen`
and `review` read a saved report. Reports render through `output`
(`table`, `timeline`, `export`). `config` resolves
flags, environment and the configuration file; `console` and `style` own
messages and colour; `catalog` loads the index from the peak store, for
the library's files or, without a library, the store's own records;
`survey` compares the peak store with the library for `doctor` and
`prune`, telling records of files gone from the library from those of
ignored files; `names` reads track names from tags; `playback` picks the
ladder.

The harness adds `sweep`, `scan`, `calibrate`, `regress`, `robust`
(transformed excerpts), `synthetic` (the scale proxy), `clusters` and
`related` (self-match of the library), `pair` (two files' alignments and
the time their hits support), `recall` (a panel drawn from the larger
library's records, beyond the corpus recordings), `survival` and `hash_cost` (hash
measurements without a search) on top of `manifest` and `scoring`.
`verifier` is a peak verifier after Qfp, a diagnostic behind `--verify`
that leaves detections unchanged; `shared` searches queries cut from a
passage two recordings share, and `identifiability` compares the files
of two recordings over that passage in audio, peaks and pairs (proposal
5's diagnostic, `docs/panels/`); `shared_mixes` renders mixes that play
a panel's played files whole among drawn tracks (`mixes::draw_around`),
searches them with the played recording indexed and left out, and
measures each detection for session 21's candidate statistics;
`holders` runs its lookup in the library's map of shared passages for
a list of files and spans.
`padding` adds records to the index for scale experiments, including a
larger library the corpus was drawn from (`--other-peaks-dir`), whose
copies of corpus files `library_map` finds and whose rips of the corpus
recordings `clusters::find_around` finds; with it, the index is saved in
`work/indexes/` and runs that leave files out remove them from the
loaded index (ADR 0014).
`audition` is the listening tool (below). `platform` names the
system, for the few programs that only macOS or only Linux has (ADR
0020).
Python scripts that summarise harness reports for experiments live in
`scripts/analysis/`.

`gunfinger-eval` depends on `gunfinger-core`, never the reverse. The CLI and the
core never see a manifest, a set name or a track title. No per-track, per-set
or filename-derived logic anywhere. The patterns of `.gunfingerignore` are the
configuration, read as written; never write or derive them.

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

## The listening tool

`gunfinger-eval audition` (ADR 0016) serves a page on `127.0.0.1` on
which listening questions are answered by ear. It is a development
tool: its answers settle evaluation ground truth, and nothing in the
product reads them (ADR 0017). The code is in
`crates/gunfinger-eval/src/audition/` (the module map is in
`audition.rs`), the page in `audition/web/`, embedded with
`include_str!`. `docs/audition-questions.md` documents the question
format field by field, the answer log and the HTTP routes.

Invariants:

- Agents never answer. Run `audition` for a check only with temporary
  files: `gunfinger-eval --verdicts /tmp/x/pair-verdicts.txt audition
  --answers /tmp/x/answers.jsonl` (`--verdicts` is a global option, before
  the subcommand; copy the real verdicts file there first). Before every
  commit, `git diff docs/pair-verdicts.txt` is empty and
  `work/audition/answers.jsonl` holds only the listening answers.
- The page knows no kinds of question; producers know nothing of HTTP or
  the page. A question is data: lanes of pieces on a timeline, markers,
  spans, choices, an effect.
- One placement rule: file time = `offset + speed × t`. Producers compute
  pieces with `audition::placement`; the page evaluates the rule only in
  `timeline.js`.
- A question's id comes from `QuestionId::derive(kind, parts)`. Answers
  attach by id, so changing the parts of an existing kind orphans the
  listening answers.
- `schema::check` runs in every producer and in the server. A choice key
  is never a control key (`schema::CONTROL_KEYS`, kept equal to the
  letters in `web/keys.js`).
- The server serves, decodes, describes and reveals only files named in
  the loaded question files, compared exactly. Every request carries the
  token and passes the `Host` check; every POST is JSON and, with an
  `Origin`, from the page's origin. FFmpeg runs as a child process with
  explicit arguments, never through a shell, under the core's stall
  timeout (`gunfinger_core::decode::watch`, ADR 0018).
- The answer log is append-only; undo appends a record. A verdict line is
  removed only while it is still the verdicts file's whole last line,
  byte for byte. Answers run one at a time in a server. A server holds
  the answer log's file lock while it runs, and a second server on the
  same log exits with a message instead of waiting; verdict lines are
  checked and written under the verdicts file's lock, by the server and
  by `pair-review` (ADR 0016, "One writer").
- No build step, no Node, no new crate.

Adding a kind of question:

1. A producer module, `audition/<kind>.rs`, that turns evidence into a
   `QuestionFile`: `files` as harness names (`names::Libraries`), lanes
   placed with `placement`, choices with their own keys, `Effect::Log`.
   Test it on fixtures (fixed lengths, or `audio::tests::tone` for real
   files) and run `schema::check` in the test.
2. A `QuestionSource` variant in `main.rs` and its arm in
   `make_questions`, writing `work/audition/questions/NN-<kind>.json` by
   default.
3. The kind in `docs/audition-questions.md`, with an example.
4. Only if its answers must be written somewhere besides the log: a new
   `Effect` variant (`schema.rs`) and its arms in `effects.rs`. The
   answer book and undo assume the verdicts file today; `effects.rs`'s
   module documentation lists what a new file needs, so undo never
   shortens the wrong file.

Testing: `cargo test -p gunfinger-eval audition` (the server tests use a
real socket; the audio tests generate tones with FFmpeg). Trying the
page: start `audition --no-open` on temporary files as above, open the
printed URL (a browser, or headless Chrome over the DevTools
protocol: session 14's client is `work/s14/cdp/cdp.ts`, uncommitted), and
read `window.audition.snapshot()` for the open question, the player's
position, lanes and sources, and the queue; `window.audition.open(id)`
opens a question and `window.audition.player()` returns the player. Keys
reach the page only once it has focus (click the title first).

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
- `stakka-skynet-knowledge` and `razorjack-2003-03-29` (my own
  mix, experiment 0060) are the development sets. Iterate freely on them.
  The second's manifest was drafted from Gunfinger's output and checked by
  ear: its starts are Gunfinger's boundaries except track 4's,
  so do not measure boundary accuracy on it beyond that track.
- `ed-rush-optical-essential-mix` is the held-out test set. It is evaluated
  only after the `poc-freeze-1` tag, at most five times in total, and every
  evaluation is logged in `docs/experiments/test-set-ledger.md`.
  The NAS root holds `EM_1999_02_07_-_Ed_Rush_Optical.mp3`, by its name
  a copy of the test set's broadcast (not yet confirmed manually):
  never identify, decode or index it, and keep it out of every set.
- Never edit a manifest. Suspected ground-truth errors are listed under "To check
  manually" and still count as wrong.
