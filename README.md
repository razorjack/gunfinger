# Gunfinger

Gunfinger is a local command-line tool that finds tracks from your own music
collection inside DJ mixes. It targets jungle and drum & bass vinyl sets,
where the turntable changes pitch and tempo together by up to about ±8%. It
reports a track only on sustained, aligned evidence: unknown audio should stay
unknown, because a confident wrong answer is worse than a miss.

**Status: proof of concept.** It has been evaluated on one library of 262
tracks and two mixes. It is not packaged, has no configuration file and keeps
its index in memory. See [docs/status.md](docs/status.md) for the full report
and [docs/roadmap.md](docs/roadmap.md) for what is missing.

## How it works

1. `index` decodes every audio file with FFmpeg to 8 kHz mono, picks the
   local maxima of its spectrogram (peaks) and stores them, one record per
   file. The peaks are the source of truth; everything else is rebuilt from
   them.
2. Each run builds an in-memory index of peak-pair hashes (Wang 2003): anchor
   frequency, frequency difference and time difference, 21 bits, 4 bytes per
   posting.
3. `identify` analyses the mix under 41 assumed speeds from 0.92 to 1.08. On
   the right speed the mix's hashes meet the reference's hashes on a straight
   line through mix time and track time. Lines found in successive 10 s
   windows are chained; a chain is a detection, and its slope gives the speed.
4. A detection is confident when its chain holds at least 200 hits in at
   least 3 windows. The rule was calibrated against the strongest false
   alignment measured and then frozen. Below it, a detection with at least
   60 hits is possible: the recording, or one sharing material with it,
   probably plays there. Possible detections are shown but never count as
   identifications.
5. Detections of one file with gaps of up to 90 s are listed as one play,
   so a needle skip or a radio insert does not split a record into
   unrelated rows.

The design decisions and the measurements behind them are in `docs/adr/` and
`docs/experiments/`.

## Prerequisites

- Rust stable with edition 2024 support (`rust-toolchain.toml`).
- `ffmpeg` and `ffprobe` on `PATH` (for example `brew install ffmpeg`).

Developed on macOS (Apple M1 Pro, 10 cores) with Rust 1.98 and FFmpeg 9.0.1.

## Usage

```sh
cargo build --release

# Extract peaks for every audio file under the library root (incremental).
target/release/gunfinger index ~/Music/library

# Find library tracks in a mix.
target/release/gunfinger identify mix.m4a --library ~/Music/library
target/release/gunfinger identify mix.m4a --library ~/Music/library \
    --start 45:00 --duration 10:00 --format json > mix.json

# Also find records played with key lock (CDJ master tempo: tempo changes,
# pitch does not); about a third slower. Such plays are marked (key lock).
target/release/gunfinger identify mix.m4a --playback both

# Several recordings share one index build; --save-dir keeps a JSON report
# of each and passes over recordings already reported there (--again).
target/release/gunfinger identify ~/Mixes/*.m4a --save-dir ~/Mixes/reports

# Write a saved report in another format without searching again.
target/release/gunfinger show mix.json --format tracklist

# Hear the mix at 20:30, then each track found there, from the same place in
# the track and at the speed it was played (needs ffplay; --print shows the
# commands instead).
target/release/gunfinger listen mix.json --at 20:30

# Step through a report by ear: list its plays, then type a play's number
# (or `<n> start`, `<n> end`, `at 20:30`) to hear the mix and the tracks.
target/release/gunfinger review mix.json

# Every candidate within a minute of 20:30, weak ones included, with what
# each lacks for the next level; --asset narrows it to matching paths.
target/release/gunfinger explain mix.m4a --library ~/Music/library --at 20:30

# Sizes of the peak store and the index, with a 25,000-track projection.
target/release/gunfinger stats --library ~/Music/library

# Check FFmpeg, the settings, the library and the peak store.
target/release/gunfinger doctor --library ~/Music/library

# List, then delete, peak records of files no longer in the library.
target/release/gunfinger prune --library ~/Music/library
target/release/gunfinger prune --library ~/Music/library --yes
```

Settings come from flags, then environment variables, then a TOML
configuration file (`$XDG_CONFIG_HOME/gunfinger/config.toml` or
`~/.config/gunfinger/config.toml`; `--config FILE` or `GUNFINGER_CONFIG`
for another, for example one per library):

```toml
library = "~/Music/library"   # used when --library is not given
peaks_dir = "~/.local/share/gunfinger/peaks"
jobs = 8
color = "auto"
playback = "turntable"         # "both" if the sets are from CDJs or software
```

The peak store is in `work/peaks` by default (`--peaks-dir`,
`GUNFINGER_PEAKS_DIR`). `--jobs` (`GUNFINGER_JOBS`) sets the worker threads;
the default is one per core. Files longer than 20 minutes are skipped by
`index` (`--max-track-minutes`). Files that fail to decode or are too long
are remembered in the peak store and passed over on later runs until they
change (`--retry-skipped` tries them again). Keep one peak store per
library: records are keyed by the path relative to the library root, and
`prune` refuses to delete most of a store, which is what a shared store or
a wrong `--library` looks like. `identify --exclude-from FILE` leaves the
listed library paths out of the index. Human output is coloured on a terminal
(`--color auto|always|never`; `NO_COLOR` turns `auto` off). `--quiet` keeps
results, warnings and errors and drops progress and timing.
`gunfinger completions <shell>` prints a completion script (bash, zsh, fish,
elvish, powershell) and `gunfinger man` prints the man page.

Human output is one table of plays in time order: time span in the
recording, the part of the track that was heard, speed, confidence
(`confident` or `possible`), hits and asset path. When several library files
hold the same audio (copies, or rips with identical peaks), their plays are
identical and shown once, with the other paths underneath (`also ...`). A play
of several segments lists them underneath.

`--format` (for `identify` and `show`) also takes:

- `timeline`: each play as a bar across the recording.
- `json`: the whole report: plays with their segments and `same_audio` paths
  (`schema_version` 3; fields may be added without a version change).
- `csv`: one row per play, times in seconds.
- `tracklist`: a numbered list of recordings with start times, named
  `artist - title` from the files' tags (or the file name), possible ones
  marked.
- `cue`: a cue sheet of the confident recordings, for players and splitters.

The tracklist and the cue sheet list recordings rather than plays: when two
rips or masters of one recording are both found at the same time, speed and
place in the track, they are one entry, named after the stronger.

## Development

```sh
scripts/check.sh    # fmt, clippy with -D warnings, tests
```

`gunfinger-eval` is the evaluation harness. It reads a private corpus
(`corpus/library/` and `corpus/sets/<set>/tracklist.toml`) that is not part
of the repository:

```sh
target/release/gunfinger-eval validate                 # check the manifests
target/release/gunfinger-eval clusters                 # duplicate rips in the library
target/release/gunfinger-eval sweep --seed 2026        # speed sweep
target/release/gunfinger-eval scan <set> [--leave-out 3 --seed 2026]
target/release/gunfinger-eval calibrate                # confidence margin
target/release/gunfinger-eval baseline <name>          # save the standard reports
target/release/gunfinger-eval regress <name>           # rerun them and show what changed
target/release/gunfinger-eval robust --seed 2026       # excerpts under EQ, noise, codecs, key lock...
target/release/gunfinger-eval related                  # remixes and shared material in the library
target/release/gunfinger-eval hash-cost                # pairs and triplets: postings, lookups, evidence
```

`--ladder turntable|key-lock|both` (before the command) chooses the rungs
every search uses. `scan` and `robust` take `--synthetic-copies N`, which
adds N time-reversed, stretched copies of every record to the index as a
proxy for a larger library, and `--drop-fullest SHARE`, which empties the
fullest posting lists; those reports are kept apart from the ones
`calibrate` and `regress` read.

Read [AGENTS.md](AGENTS.md) before changing code: it holds the house style,
the crate boundaries and the evaluation rules.

## Results

Measured on 262 library tracks (29.3 hours) with the rule frozen at tag
`poc-freeze-1`:

| Evaluation | Result |
|------------|--------|
| Speed sweep (80 excerpts × 9 speeds, −8% to +8%, MP3) | 100% recall at every speed, 0 wrong, speed error ≤ 0.016% |
| Development mix (56 min, 11 tracks) | 11/11 identified, 0 wrong, 22–26 s |
| Development leave-outs (3 and 11 tracks removed from the index) | 0 wrong |
| Held-out test mix (radio broadcast, 1 h 58 min, 31 tracks) | First run: 15/17 identified (88%), 0 wrong, 53 s. One miss was a remix not in the library; with the manifest corrected, 15/16 (94%). The other miss, cut by a radio insert, is shown as possible (second evaluation) |

The pass bar was at least 80% identified and zero wrong on each mix. The
index takes 74 postings per second of audio and 5.07 bytes per posting; for
25,000 tracks that projects to 3.0 GB in memory, or about 2.3 GB with the
delta-coded on-disk layout recommended in ADR 0005.

Known limits: the thresholds have been measured on 262 real tracks, and on
up to 21,109 assets only with synthetic reversed copies, which lack the
shared breaks and remixes of a real library
([docs/calibration.md](docs/calibration.md) lists what to measure again).
Key-locked (pitch-preserved) mixes need `--playback both`. Heavy damage
(noise at 0 dB SNR, a blend at equal level) loses evidence but has not
produced a false identification (experiment 0009).
