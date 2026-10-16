<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/assets/gunfinger-hand-dark.png">
    <source media="(prefers-color-scheme: light)" srcset="docs/assets/gunfinger-hand-light.png">
    <img src="docs/assets/gunfinger-hand-light.png" alt="Gunfinger's hand making a gunfinger gesture and holding a vinyl record with an orange label" width="320">
  </picture>
</p>

# Gunfinger

Gunfinger tells you which tracks from your own music collection play in a
DJ mix. Point it at a mix, wait a few seconds, and get a tracklist.

It is built for jungle and drum & bass: vinyl sets, where the turntable
speeds a record up or slows it down by up to about ±8%, pitch and tempo
together, and digital sets played with key lock, where the tempo changes
and the pitch does not. It runs locally, on your files, with nothing but
FFmpeg beside it.

Here is Stakka & Skynet's Knowledge Magazine mix from 2000, 56 minutes of
vinyl, searched against a library of 262 tracks:

```console
$ gunfinger identify "Stakka & Skynet - Knowledge Magazine Mix (2000).m4a"
 1.    0:00  Stakka And Skynet - Nightlore
 2.    4:45  Stakka & Skynet - Decoy
 3.    9:45  Konflict - Star Trails
 4.   14:22  Stakka & Skynet - Pathogen
 5.   17:32  Stakka & Skynet - Clockwork
 6.   21:22  Kraken - Side Effects (Teebee remix)
 7.   25:20  Kraken+Arkane - Dominion  (also: KRAKEN - dominion)
 8.   31:03  Kemal And Paul Reset - Kontempt
 9.   37:07  Stakka & Skynet feat. Kemal an - Bios-Fear
10.   43:22  Stakka + Skynet - Logistics
11.   49:46  Stakka & K.Tee - Auto Ignition
```

All eleven tracks of the mix, in order, in 20 seconds, with nothing wrong.
The library also holds the later Stakka remix of Clockwork, which carries
a passage of the original; that weaker match inside the original's play
stays out of the tracklist instead of passing for a track. Entry 7 is two
files in the library, named from their tags, both listed. A start time is
where the track's audio is first found, so in a long blend it can come
before the point where you would call the changeover.

A few principles shape everything else:

- **Unknown audio stays unknown.** A confident wrong answer is worse than
  a miss. Gunfinger reports a track only on sustained, aligned evidence,
  and shows weaker evidence as `possible`, never as an identification.
- **Fire and forget.** You run it, read the result and are done. When
  several files match one track, it names all of them and says when their
  titles disagree, without picking one.
- **Your tags, as they are.** Titles come from your files' tags.
  Gunfinger never edits tags or tries to fix a library.

**Status: proof of concept.** It works well on the collections I have
measured it on (below), but it is not packaged, and the thresholds were
set against one library of 262 tracks.

## Contents

- [Getting started](#getting-started)
- [Usage](#usage)
- [Configuration](#configuration)
- [How it works](#how-it-works)
- [How it is built](#how-it-is-built)
- [Documentation](#documentation)
- [Bibliography](#bibliography)

## Getting started

Gunfinger supports macOS and Linux. You need:

- Rust 1.89 or later (`rust-toolchain.toml`).
- `ffmpeg` and `ffprobe` on `PATH`; `ffplay` too if you want to listen
  to results. FFmpeg brings all three: `brew install ffmpeg` on macOS,
  `sudo apt install ffmpeg` on Debian or Ubuntu.

Build it and put `target/release/gunfinger` somewhere on your `PATH`:

```sh
cargo build --release
```

Then index your library once and identify mixes against it:

```sh
gunfinger index ~/Music/library
gunfinger identify mix.m4a --library ~/Music/library
```

`index` decodes each audio file and keeps a small record of its
spectrogram peaks, about 180 bytes per second of audio. It is incremental:
the next run only reads files that are new or changed. Set `library` and
`peaks_dir` in the [configuration file](#configuration) and both commands
work without flags from any directory.

## Usage

### Identifying mixes

```sh
# A tracklist (the default).
gunfinger identify mix.m4a

# Part of a recording, as JSON for scripts.
gunfinger identify mix.m4a --start 45:00 --duration 10:00 --format json > mix.json

# Many recordings at once. Each report is saved, and recordings already
# reported are passed over unless something that affects the result has
# changed (--again searches them anyway).
gunfinger identify ~/Mixes/*.m4a --save-dir ~/Mixes/reports
```

A recording in which nothing is found prints `no tracks found`.

The tracklist lists tracks, not files. Plays of several library files over
the same stretch of the mix are one entry when the files line up with each
other (rips, masters, uploads sped up by a few percent) or when their tags
give the same artist and title (an edit, a re-press, an upload with a
longer intro). The entry is named after its strongest file; the others
follow as `also:`, or as `titles disagree:` when their titles differ. Two
tracks that overlap in a blend stay separate entries. A possible match
lying inside a confident track's play, while that track's matches run on,
such as a remix that carries a passage of the original, is material the
two share, not a track, and is left out; the table (`--format human`)
shows it, marked `shares material with`, and JSON keeps it.

`--format` (`-f`) also takes:

- `human`: a table of every play: the span in the mix, the part of the
  track heard, speed, confidence (`confident` or `possible`), hits and
  file path. Plays found with key lock end with `(key lock)`.
- `timeline`: each play as a bar across the recording.
- `json`: the whole report, including each file's tags. Fields may be
  added without a change of `schema_version`.
- `csv`: one row per play, times in seconds.
- `cue`: a cue sheet of the confident tracks, for players and splitters.

### Other commands

```sh
gunfinger show mix.json                   # print a saved report again, in any format
gunfinger listen mix.json --at 20:30      # hear the mix at 20:30, then each track found there, at the speed it was played
gunfinger review mix.json                 # step through a report's plays by ear
gunfinger explain mix.m4a --at 20:30      # every candidate near 20:30, weak ones too, and what each lacks
gunfinger stats                           # sizes of the peak store and the index
gunfinger doctor                          # check FFmpeg, the settings, the library and the peak store
gunfinger prune                           # list peak records of files gone from the library (--yes deletes them)
gunfinger completions zsh                 # shell completions (bash, zsh, fish, elvish, powershell)
gunfinger man                             # the man page
```

Results go to stdout, progress and warnings to stderr. `--verbose` (`-v`)
adds timings and every file left out of the index; `--quiet` (`-q`) keeps
only results, warnings and errors.

## Configuration

Settings come from command-line flags, then environment variables, then a
TOML file, then the built-in defaults. The file is
`$XDG_CONFIG_HOME/gunfinger/config.toml` (or
`~/.config/gunfinger/config.toml`); `--config FILE` or `GUNFINGER_CONFIG`
names another. Every key is optional and an unknown key is an error.
`gunfinger doctor` prints the settings in effect and where they came from.

```toml
library = "~/Music/library"
peaks_dir = "~/.local/share/gunfinger/peaks"
jobs = 8
color = "auto"
playback = "both"
min_track = "1:30"
max_track = "15:00"
```

| Key | Flag | Environment | Default | Meaning |
|-----|------|-------------|---------|---------|
| `library` | `--library` | | none | The library root |
| `peaks_dir` | `--peaks-dir` | `GUNFINGER_PEAKS_DIR` | `work/peaks` in the current directory | The peak store |
| `jobs` | `--jobs` | `GUNFINGER_JOBS` | one per core | Worker threads |
| `color` | `--color` | `NO_COLOR` turns `auto` off | `auto` | `auto`, `always` or `never`; the tracklist is always plain |
| `playback` | `--playback` | | `both` | `both`, `turntable` or `key-lock` |
| `min_track` | `--min-track` | | none | Shorter files are samples and loops |
| `max_track` | `--max-track` | | `17:00` | Longer files are mixes and album rips; the index addresses up to 17:28 |

Lengths are written as on the command line: seconds (`"90"`), `M:SS` or
`H:MM:SS`.

### Vinyl only

Turntable playback and key lock need different searches, and by default
Gunfinger runs both, so it finds a track however it was played. If you only
identify vinyl mixes, `playback = "turntable"` skips the key-lock search and
makes searches about a third faster. `--playback both` still handles the
occasional digital set.

### Leaving files out

A folder that also holds DJ sets, samples and loops needs a narrower track
length range (`min_track`, `max_track`). Files outside it are not decoded,
and narrowing it later needs no new index.

Some files belong in your collection but not in the search. A track of a
DJ-mixed CD already blends into the next one, so it matches two recordings
and joins them. List such files and folders in `.gunfingerignore` at the
library root:

```gitignore
# Mixed CDs: each track already blends into the next one.
/Underfire UDFRCD003 - Dangerous Drums Volume 2 (2000)/
/2001 - VRSCD003 - Ed Rush & Optical - The Creeps [Virus]/CD2/
```

The syntax is a subset of `.gitignore`: `#` comments, `/` anchors a
pattern at the root, a trailing `/` matches folders only, and `*`, `?` and
`**` work as usual. Square brackets match themselves, so release folders
such as `[Virus]` are written as they are. There is no negation.
`gunfinger doctor` shows how many files each pattern leaves out and warns
about a pattern that matches nothing. Gunfinger never writes this file.

### The peak store and the saved index

Keep one peak store per library; records are keyed by their path relative
to the library root, and every command refuses a store that names another
library. With several libraries, give each its own configuration file.

Each search builds an index of the peak store and saves it under
`$XDG_CACHE_HOME/gunfinger/indexes/` (or `~/.cache/gunfinger/indexes/`),
about 4 bytes per posting. The next search loads it in a few seconds
instead of building it again, as long as nothing it was built from has
changed. The folder can be deleted at any time; `--saved-index off`
builds the index in memory only.

### Without the library

The peak store holds each file's peaks and tags, so it is enough to
identify and name tracks. When the library is not mounted, `identify`,
`explain` and `stats` search the store's records and say so; `--store-only`
does it on purpose. To identify mixes on another computer, copy the peak
store there and point `--peaks-dir` at it. Leave `library` unset there,
or set it to the path the store was made from, even if that path does
not exist on that computer: the store names its library and refuses
another.

## How it works

Gunfinger is a landmark fingerprinter in the line of Shazam's (Wang 2003),
extended to search a mix at many assumed speeds.

1. **Peaks.** `index` decodes each file with FFmpeg to 8 kHz mono and
   keeps the local maxima of its spectrogram, one record per file. The
   peaks are the source of truth; everything else is rebuilt from them.
2. **Hashes.** Each search builds an index of peak-pair hashes: an anchor
   peak's frequency, the frequency difference to a nearby peak and the
   time between them, packed into 21 bits, 4 bytes per posting.
3. **Speed ladder.** `identify` analyses the mix under 41 assumed speeds
   from 0.92 to 1.08, each in two ways: as a turntable plays it (time and
   frequency scaled together) and with key lock (time scaled, frequency
   kept). Under the right assumption, the mix's hashes meet a track's
   hashes on a straight line through mix time and track time.
4. **Candidates.** Lines found in successive 10 s windows are chained into
   candidates; a chain's slope gives the speed. This first pass leaves out
   the 1% fullest posting lists: the most common hashes cost the most
   lookups and tell tracks apart the least.
5. **Measurement.** Each candidate's stretch of the mix is analysed again
   at exactly its fitted speed, and its hits counted in every posting list.
   It is confident with at least 240 hits in at least 3 windows. That rule
   was calibrated against the strongest false alignment measured, then
   frozen. From 60 hits a detection is possible: the track, or one sharing
   material with it, probably plays there.
6. **Plays and tracks.** Detections of one file with gaps of up to 90 s
   are one play, so a needle skip or a radio insert does not split a
   record. Plays of files that hold the same recording become one
   tracklist entry.

### Results

| Evaluation | Result |
|------------|--------|
| Speed sweep: 2,160 excerpts across four seeds, −8% to +8% | 100% recall, 0 wrong |
| Stakka & Skynet, Knowledge Magazine mix (56 min, 11 tracks) | 11/11, 0 wrong |
| The same mix with 3 or 11 of its tracks removed from the index | 0 wrong |
| Held-out test mix: an Essential Mix radio broadcast (1 h 58 min) | 14 of 16 tracks in the library, 0 wrong; the other two shown as possible |
| A collection of 27,000 tracks, the Stakka & Skynet mix | 11/11, 0 wrong |

The margin between the weakest correct identification and the strongest
false candidate is 6.65× on the 262-track library. At 27,000 tracks,
unrelated chance reaches 63-70 hits, above the possible tier's 60, so
possible plays are less reliable in a library that size; confident ones
are not affected. With the saved index loaded, a 46-minute mix against
33,000 tracks takes about 90 s and 4.8 GB of memory.

Heavy damage (noise at 0 dB SNR, a blend of two tracks at equal level)
loses evidence but has not produced a false identification.

## How it is built

Gunfinger is written in synchronous Rust: no async runtime, database,
network service, neural model, GPU code or `unsafe`, and no third-party
fingerprinting crate. The workspace has three crates: `gunfinger-core`
(decoding, peaks, the peak store, hashing, the index, matching),
`gunfinger` (the CLI) and `gunfinger-eval` (the evaluation harness).
[AGENTS.md](AGENTS.md) has the house style and the rules for changing
code.

```sh
scripts/check.sh    # fmt, clippy with -D warnings, tests
```

Three parts of the way it is developed are worth a closer look.

### Experiment-driven development

Every change that could affect detection starts as a hypothesis and ends
as a measurement. Each experiment gets a short record in
`docs/experiments/`: the hypothesis, the change, the exact command, the
measured result and the conclusion, at most 40 lines, written before the
next experiment starts. There are over a hundred of them so far.

Experiments that lose are deleted from the code, but their records stay,
so the reasons an idea was rejected are as easy to find as the reasons
another was kept. Decisions that come out of experiments go into decision
records in `docs/adr/`, and a decision changes only through a new record
that supersedes the old one. Every threshold chosen against the library is
listed in `docs/calibration.md` with what it rests on and what to measure
again when the library grows.

The evaluation follows the same discipline as a study with a test set.
Two mixes are development sets, measured as often as needed. A third, a
two-hour radio broadcast, is held out: it was searched for the first time
only after the confidence rule was frozen, it may be evaluated at most
five times in total, and every evaluation is logged. Ground-truth
tracklists are never edited to make a result look better; a suspected
error in one is checked by ear and still counts as wrong until then.

### Regression checks

Search is deterministic, so a change that is not meant to affect
detection can be shown not to, bit for bit. `gunfinger-eval baseline`
saves the standard evaluation's reports (the seeded speed sweep, the
development mix and its leave-outs) under a name, and `regress` runs it
again on the current code and reports what changed:

```console
$ gunfinger-eval baseline before
$ # ... change the code ...
$ gunfinger-eval regress before
compared with baseline work/baselines/before
sweep-seed-2026.json: 720 of 720 queries with identical detections
scan-stakka-skynet-knowledge.json: identified 11 -> 11, wrong 0 -> 0, possible 0 -> 0, unmatched possible plays 1 -> 1
  all 87 detections identical
scan-stakka-skynet-knowledge-leave-out-3-seed-2026.json: identified 8 -> 8, wrong 0 -> 0, possible 0 -> 0, unmatched possible plays 1 -> 1
  all 93 detections identical
scan-stakka-skynet-knowledge-leave-out-11-seed-2026.json: identified 0 -> 0, wrong 0 -> 0, possible 0 -> 0, unmatched possible plays 1 -> 1
  all 79 detections identical
calibrate: weakest identifying 680 -> 658, strongest false 99 -> 99, missed 0 -> 0, false accepted 0 -> 0
calibrate: false candidates at 30 hits or more 18 -> 46, strongest on audio not in the index 20 -> 22
```

Every commit that could change detection runs `regress` first. A
performance change is expected to come back identical; a matching change
shows exactly which detections moved and how the confidence margin
changed.

`regress` sits on top of the rest of the harness, which reads a private
corpus (`corpus/library/` and one `corpus/sets/<set>/tracklist.toml` per
mix) that is not part of the repository:

| Command | What it measures |
|---------|------------------|
| `sweep` | Held-out excerpts at nine speeds from −8% to +8%; the panels are drawn once per seed and kept in `docs/panels/` |
| `scan` | A real mix against its tracklist, optionally with tracks left out of the index |
| `calibrate` | The margin between the weakest correct and the strongest false detection |
| `robust` | Excerpts under EQ, noise, codecs, blends, speech, skips, pitch rides and key lock (speech from `say` on macOS, from eSpeak NG on Linux: `sudo apt install espeak-ng`) |
| `mixes` | Generated mixes with exact truth: speeds, bass swaps, crossfades, cuts, short plays |
| `grid` | Brief plays slid across the 10 s windows, to test where a short play stops counting |
| `seconds` | Saved `mixes` and `scan` reports scored second by second: labelled correctly, possibly or wrongly, unlabelled, and silent where nothing indexed plays; where each track's labels start and end |
| `clusters`, `related` | The library matched against itself: duplicate rips, remixes and shared material |
| `loss` | Where a mix loses evidence compared with a clean render of the same stretch |
| `memory` | Resident memory by phase, also on a synthetic library many times the corpus |

### The listening tool

Some questions only a listener can settle: whether two files are the same
recording, whether a release folder is a DJ-mixed CD, whether a detection
is right. `gunfinger-eval audition` serves a small web page on
`127.0.0.1` for answering them by ear, one key per answer.

Each question is a set of lanes on one timeline: the mix in one lane, the
candidate tracks in the others, placed where and at the speed Gunfinger
found them. Switching lanes keeps the musical moment, so you can jump
from the mix to the track mid-bar and hear whether it is the same
pressing. Speed matching plays each lane at its placed speed; `B` plays
two lanes together.

| Key | Action |
|---|---|
| Space | play or pause |
| 1-9 | switch to that lane at the same moment |
| Tab | back to the lane played before |
| Left, Right | seek 5 s (with Shift 30 s) |
| Up, Down | previous or next marker |
| L | loop 8 s around the moment |
| M | speed matching on or off |
| B | this lane and the one played before together |
| J, K | next or previous question |
| Z | undo the latest answer |
| ? | help |

Questions are plain JSON files written by `gunfinger-eval questions` from
the harness's evidence; the page knows no kinds of question, only lanes,
markers and choices, so a new kind needs no change to the page. Answers go
to an append-only log, and verdicts on pairs of files feed back into
`clusters`. The format is in
[docs/audition-questions.md](docs/audition-questions.md). The tool is for
building ground truth; nothing in Gunfinger itself reads its answers.

## Documentation

Everything else is in `docs/`:

- `docs/adr/`: decision records, one per design decision, with the
  context and the alternatives.
- `docs/experiments/`: one record per experiment, numbered in order, with
  data in `docs/experiments/data/` and the held-out test mix's ledger in
  `test-set-ledger.md`.
- `docs/calibration.md`: every number chosen against the library and what
  to measure again as it grows.
- `docs/references.md`: the papers below, with the sections that matter
  and where Gunfinger uses them.
- `docs/roadmap.md`: what is missing and ideas not yet tried.
- `docs/status.md`: a running log of the work, with exact commands and
  numbers.
- `docs/brief*.md` and `docs/session-*-checklist.md`: the brief for each
  development session and its progress.
- `docs/audition-questions.md`: the listening tool's question format.
- `docs/panels/`: the seeded excerpt panels the sweeps measure.

## Bibliography

### Fingerprinting

- A. L.-C. Wang, "An Industrial-Strength Audio Search Algorithm," Proc.
  4th ISMIR, Baltimore, 2003, pp. 7-13.
  <https://www.ee.columbia.edu/~dpwe/papers/Wang03-shazam.pdf>
  Spectrogram peaks, pair hashes, matching as a diagonal line of time
  pairs, and a threshold measured against the best wrong match.
- J. Six and M. Leman, "Panako – A Scalable Acoustic Fingerprinting System
  Handling Time-Scale and Pitch Modification," Proc. 15th ISMIR, Taipei,
  2014. <https://archives.ismir.net/ismir2014/paper/000122.pdf>
  Triplet hashes invariant to speed changes; the alternative to the speed
  ladder.
- J. Six, "Panako 2.0 – Updates for an Acoustic Fingerprinting System,"
  Late-Breaking Demo, 22nd ISMIR, 2021.
  <https://backoffice.biblio.ugent.be/download/8726851/8726856>
  Near-exact hashing and matching around a regression line.
- R. Sonnleitner and G. Widmer, "Robust Quad-Based Audio Fingerprinting,"
  IEEE/ACM Transactions on Audio, Speech, and Language Processing, 2016,
  doi:10.1109/TASLP.2015.2509248.
  <https://www.cp.jku.at/research/papers/preprint_TASLP2509248.pdf>
  Quad hashes and peak verification, the basis of the harness's
  `--verify` diagnostic.
- R. Sonnleitner, A. Arzt and G. Widmer, "Landmark-Based Audio
  Fingerprinting for DJ Mix Monitoring," Proc. 17th ISMIR, New York,
  2016, pp. 185-191.
  <https://archives.ismir.net/ismir2016/paper/000187.pdf>
  Pairs, triplets and quads compared on real DJ mixes, scored per second.
- J. O. Smith and X. Serra, "PARSHL: An Analysis/Synthesis Program for
  Non-Harmonic Sounds Based on a Sinusoidal Representation," Proc.
  International Computer Music Conference, 1987.
  Quadratic interpolation of spectral peaks, used for each peak's
  fractional bin.

### Sample identification

- A. Bhattacharjee, I. Meresman Higgs, M. Sandler and E. Benetos,
  "Refining Music Sample Identification with a Self-Supervised Graph
  Neural Network," Proc. 26th ISMIR, 2025, arXiv:2506.14684.
  <https://arxiv.org/abs/2506.14684>
  Finding a sampled recording, which for Gunfinger is a wrong answer
  resting on the same evidence.

### Datasets

- Mixotic (Sonnleitner et al. 2016): 10 Creative Commons DJ mixes of
  techno and house with 723 reference tracks and song-border annotations.
  <https://www.cp.jku.at/datasets/fingerprinting/>
- D. Schwarz and D. Fourer, "UnmixDB: A Dataset for DJ-Mix Information
  Retrieval," 2018, doi:10.5281/zenodo.1422385. Generated mixes of the
  Mixotic tracks with exact ground truth.

### Other

- G. L. Steele Jr., D. Lea and C. H. Flood, "Fast Splittable
  Pseudorandom Number Generators," Proc. OOPSLA, 2014. SplitMix64, the
  harness's seeded random number generator.

### Further reading

Cited in the project as leads, not yet read:

- T. Kim, M. Choi, E. Sacks, Y.-H. Yang and J. Nam, "A Computational
  Analysis of Real-World DJ Mixes Using Mix-To-Track Subsequence
  Alignment," Proc. 21st ISMIR, 2020.
- D. Schwarz and D. Fourer, "Methods and Datasets for DJ-Mix Reverse
  Engineering," Proc. CMMR, 2019.
- T. Kim, J. Kim, H. Kim and J. Nam, "Raveform: A Dataset of Metrical and
  Functional Structure Annotations for EDM Tracks in DJ Mixes,"
  Transactions of the ISMIR 9(1), 2026.
- D. Lang, D. W. Hogg, K. Mierle, M. Blanton and S. Roweis,
  "Astrometry.net: Blind Astrometric Calibration of Arbitrary
  Astronomical Images," The Astronomical Journal, 2010.
