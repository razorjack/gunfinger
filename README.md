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
   least 3 windows. The rule was calibrated against the strongest chance
   alignment measured and then frozen.

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

# Find library tracks in a mix; JSON goes to stdout.
target/release/gunfinger identify mix.m4a --library ~/Music/library
target/release/gunfinger identify mix.m4a --library ~/Music/library \
    --start 45:00 --duration 10:00 --format json

# Sizes of the peak store and the index, with a 25,000-track projection.
target/release/gunfinger stats --library ~/Music/library
```

The peak store is in `work/peaks` by default (`--peaks-dir`,
`GUNFINGER_PEAKS_DIR`). `--jobs` (`GUNFINGER_JOBS`) sets the worker threads;
the default is one per core. Files longer than 20 minutes are skipped by
`index` (`--max-track-minutes`). `identify --exclude-from FILE` leaves the
listed library paths out of the index.

Human output is a table of time span, speed, confidence, hits and asset path,
followed by up to ten sub-threshold candidates.

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
```

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
| Held-out test mix (radio broadcast, 1 h 58 min, 31 tracks) | 15/17 identified (88%), 0 wrong, 53 s; first and only run. One miss was a remix not in the library; with the manifest corrected, 15/16 (94%) |

The pass bar was at least 80% identified and zero wrong on each mix. The
index takes 74 postings per second of audio and 5.07 bytes per posting; for
25,000 tracks that projects to 3.0 GB in memory, or about 2.3 GB with the
delta-coded on-disk layout recommended in ADR 0005.

Known limits: the rule has not been tested against a library larger than 262
tracks, key-locked (pitch-preserved) mixes are not supported, and nothing has
been done for heavy EQ or long blends.
