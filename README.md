<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/assets/gunfinger-hand-dark.png">
    <source media="(prefers-color-scheme: light)" srcset="docs/assets/gunfinger-hand-light.png">
    <img src="docs/assets/gunfinger-hand-light.png" alt="Gunfinger's hand making a gunfinger gesture and holding a vinyl record with an orange label" width="320">
  </picture>
</p>

# Gunfinger

Gunfinger is a local command-line tool that finds tracks from your own music
collection inside DJ mixes. It targets jungle and drum & bass sets: vinyl,
where the turntable changes pitch and tempo together by up to about ±8%, and
CDJ or software sets played with key lock, where the tempo changes and the
pitch does not. It searches for both by default; if you only identify vinyl
mixes, [turn key lock off](#key-lock-and-vinyl) for faster searches. It
reports a track only on sustained, aligned evidence: unknown audio should stay
unknown, because a confident wrong answer is worse than a miss.

**Status: proof of concept.** It has been evaluated on one library of 262
tracks and two mixes. It is not packaged and keeps its index in memory. See [docs/status.md](docs/status.md) for the full report
and [docs/roadmap.md](docs/roadmap.md) for what is missing.

## How it works

1. `index` decodes every audio file with FFmpeg to 8 kHz mono, picks the
   local maxima of its spectrogram (peaks) and stores them, one record per
   file. The peaks are the source of truth; everything else is rebuilt from
   them.
2. Each run builds an in-memory index of peak-pair hashes (Wang 2003): anchor
   frequency, frequency difference and time difference, 21 bits, 4 bytes per
   posting.
3. `identify` analyses the mix under 41 assumed speeds from 0.92 to 1.08,
   each in two ways: as a turntable plays it (time and frequency scaled
   together) and with key lock (time scaled, frequency kept). On the right
   assumption the mix's hashes meet the reference's hashes on a straight
   line through mix time and track time. This search for candidates leaves
   out the 1% of posting lists with the most postings: the most common
   hashes, which cost most lookups and tell records apart least. Lines
   found in successive 10 s windows are chained when they come from rungs
   at most a step apart, and across an empty window only when both are
   strong; a chain is a candidate, and its slope gives the speed.
4. Each candidate is measured again at its fitted speed: its stretch of the
   mix is analysed once more at exactly that speed and its hits counted in
   every posting list. It is confident with at least 240 hits in at least
   3 windows. The rule was calibrated against the strongest false
   alignment measured and then frozen. Below it, a detection with at least
   60 hits is possible: the recording, or one sharing material with it,
   probably plays there. Possible detections are shown but never count as
   identifications. `--single-pass` searches the way Gunfinger did before
   session 6: every posting list, one pass, 200 hits in 3 windows (ADR
   0008).
5. Detections of one file with gaps of up to 90 s are listed as one play,
   so a needle skip or a radio insert does not split a record into
   unrelated rows.

The design decisions and the measurements behind them are in `docs/adr/` and
`docs/experiments/`; the papers they draw on are in `docs/references.md`.

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

# Plays found with key lock are marked (key lock). For a vinyl-only mix,
# search turntable playback alone, about a third faster (or set it in the
# configuration file, below).
target/release/gunfinger identify mix.m4a --playback turntable

# Several recordings share one index build; --save-dir keeps a JSON report
# of each and passes over recordings already reported there, unless the
# playback, the part of the recording, the peak profile, the hash design,
# the matching settings, the confidence rule or the indexed library changed
# since (--again searches them anyway).
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
# each lacks for the next level; --asset narrows it to matching paths, and
# --windows lists each 10 s window's lines of hits and the chain that took
# them.
target/release/gunfinger explain mix.m4a --library ~/Music/library --at 20:30

# Sizes of the peak store and the index, with a 25,000-track projection.
target/release/gunfinger stats --library ~/Music/library

# Check FFmpeg, the settings, the library and the peak store.
target/release/gunfinger doctor --library ~/Music/library

# List, then delete, peak records of files no longer in the library or left
# out by its .gunfingerignore.
target/release/gunfinger prune --library ~/Music/library
target/release/gunfinger prune --library ~/Music/library --yes
```

`index` takes files up to 17 minutes long; longer ones are mixes or album
rips (`--min-track` and `--max-track` set the range; see
[Tracks among sets and samples](#tracks-among-sets-and-samples)). Hidden
files and folders (names starting with a dot, such as a downloader's
`.incomplete/`) are passed over, and so is whatever the library's
`.gunfingerignore` lists ([Leaving folders out](#leaving-folders-out)).
A file whose audio stops short of the
length its header declares by more than 1 s or 1% of that length, whichever
is larger, counts as damaged. An MP3 inside a WAV container behind an ID3
tag, which FFmpeg cannot open as it is, is read with the tag skipped and
keeps the tag's names. Files that
fail to decode or are outside the range are remembered in the peak store and
passed over on later runs until they change (`--retry-skipped` tries them
again). `identify --exclude-from FILE` leaves the listed library paths out of
the index.

Results go to stdout, everything else to stderr. By default stderr shows a
progress line on a terminal and warnings, such as library files that are not
indexed yet. `--verbose` (`-v`) adds timings, the size of the index and every
file left out of it, including the damaged and too-long files `index` passes
over; `--quiet` (`-q`) keeps only results, warnings and errors.
`gunfinger completions <shell>` prints a completion script (bash, zsh, fish,
elvish, powershell) and `gunfinger man` prints the man page.

Human output is one table of plays in time order: time span in the
recording, the part of the track that was heard, speed, confidence
(`confident` or `possible`), hits and asset path. When several library files
hold the same audio (copies, or rips with identical peaks), their plays are
identical and shown once, with the other paths underneath (`also ...`). A play
of several segments lists them underneath. A possible play that lies
entirely inside a confident play of another recording ends with `shares
material with <asset> (play N)`, N counting plays as `review` does: most
likely the two recordings share that passage, as a remix can carry the
original's lead. This is display only; a remix played in its own right
reaches past the original's play, or is confident itself, and is not
marked.

`--format` (`-f`, for `identify` and `show`) also takes:

- `timeline`: each play as a bar across the recording.
- `json`: the whole report: the playback searched (`query.playback`), plays
  with their segments, `same_audio` paths, `shares_material_with` and the
  file's `tags` (artist, title, album) as the peak store holds them
  (`schema_version` 3; fields may be added without a version change).
- `csv`: one row per play, times in seconds.
- `tracklist`: a numbered list of recordings with start times, named
  `artist - title` from their tags (or the file name), possible ones
  marked. The tags come from the report, so `show` names tracks without the
  library; for reports made before the store kept tags, they are read from
  the files.
- `cue`: a cue sheet of the confident recordings, for players and splitters.

The tracklist and the cue sheet list recordings rather than plays. Plays
of several library files over the same stretch of the recording are one
entry when, with each file's own speed taken out, the track would have
started at the same moment (within 5 s): rips, masters and uploads sped up
by a few percent. The entry is named after its strongest play; other names
the tags give follow it. Two tracks that overlap in a blend stay apart, and
a possible play that shares material with another entry stays its own
entry, naming it. The development mix against a large collection, where
each track is found on 2-4 records (experiment 0050):

```text
 1.    0:00  Stakka And Skynet - Nightlore  (also: Stakka And Skynet - Night Lore, STAKKA and SKYNET - knight lore, Stakka & Skynet - Nightlore)
 2.    4:36  Stakka & Skynet - Decoy  (also: Stakka & Skynet - Decoy (Remastered))
 3.    9:43  KEMAL & ROB DATA - star trails  (also: Konflict - Star Trails)
 4.   10:28  Kemal & Rob Data - Star Trails (Synergy Remix) (possible; shares material with KEMAL & ROB DATA - star trails)
 5.   14:22  Stakka & Skynet - Pathogen  (also: SKYNET & STAKKA - pathogen)
```

The JSON, CSV and table keep every play.

## Configuration

Settings come from command-line flags first, then environment variables,
then a TOML file, then the built-in defaults. The file is
`$XDG_CONFIG_HOME/gunfinger/config.toml`, or `~/.config/gunfinger/config.toml`
when `XDG_CONFIG_HOME` is not set; `--config FILE` (or `GUNFINGER_CONFIG`)
reads another. Every key is optional, an unknown key is an error, and `~` at
the start of a path is your home directory. `gunfinger doctor` prints the
settings in effect and the file they came from.

| Key | Flag | Environment | Default | Meaning |
|-----|------|-------------|---------|---------|
| `library` | `--library` | | none | Library root for `index`, `identify`, `explain`, `stats`, `doctor` and `prune`; `identify`, `explain` and `stats` search the peak store alone without it ([Without the library](#without-the-library)) |
| `peaks_dir` | `--peaks-dir` | `GUNFINGER_PEAKS_DIR` | `work/peaks`, relative to the current directory | The peak store |
| `jobs` | `--jobs` | `GUNFINGER_JOBS` | one per core | Worker threads |
| `color` | `--color` | `NO_COLOR` turns `auto` off | `auto` | Colour in human output: `auto` (on a terminal), `always` or `never` |
| `playback` | `--playback` | | `both` | Playback searched by `identify` and `explain`: `both`, `turntable` or `key-lock` |
| `min_track` | `--min-track` | | none | Shortest library file that counts as a track |
| `max_track` | `--max-track` | | `17:00` | Longest library file that counts as a track; the index addresses up to 17:28 |

A complete file:

```toml
library = "~/Music/library"
peaks_dir = "~/.local/share/gunfinger/peaks"
jobs = 8
color = "auto"
playback = "both"
min_track = "1:30"
max_track = "15:00"
```

Lengths are written as on the command line: seconds (`"90"`), `M:SS` or
`H:MM:SS`.

### Key lock and vinyl

A turntable changes pitch and tempo together. CDJs and DJ software can play
with key lock (master tempo), which many digital DJs use: the tempo changes
and the pitch does not. The two need different searches, and Gunfinger runs
both by default, so it finds a track however it was played; plays found with
key lock are marked `(key lock)`. Searching both takes about half again the
time of one (on a 56-minute mix, 261 instead of 166 CPU seconds) and lists a
few more weak candidates in `explain`. On the vinyl development mix and on
turntable-style test excerpts, the identifications are the same either way
(experiments 0010, 0016 and 0018).

If you only identify vinyl mixes, search turntable playback alone:

```toml
# ~/.config/gunfinger/config.toml: vinyl only
library = "~/Music/library"
peaks_dir = "~/.local/share/gunfinger/peaks"
playback = "turntable"
```

`--playback both` still searches the occasional digital set in full, and
`playback = "key-lock"` suits a collection of digital sets only.

### Tracks among sets and samples

`index` takes only library files whose length is within the track length
range: by default up to 17 minutes, with no minimum. A folder that also
holds DJ sets, minimixes, samples and loops needs a narrower range:

```toml
min_track = "1:30"   # shorter files are samples and loops
max_track = "15:00"  # longer files are minimixes and sets
```

`index` first reads the length a file declares in its header, so a set is
passed over without being decoded; near a limit, the decoded length
decides. Files outside the range are remembered in the peak store, and
`index` takes them when the range is widened to include them. Narrowing
the range needs no new index: `identify`, `explain` and `stats` leave out
indexed files outside it, and `--verbose` lists them. `gunfinger doctor`
shows the range in effect and how many files it passed over, and counts
only files within the range against the index's limit of 65,536 assets.
The index addresses tracks up to 17:28: with a longer `--max-track`,
searches stop at the first longer file and name it.

### Leaving folders out

Some files belong in the collection but not in the library Gunfinger
searches. A track of a DJ-mixed CD already blends into the next one, so it
matches two recordings and joins them. List such files and folders in
`.gunfingerignore` at the library root, one pattern per line:

```gitignore
# Mixed CDs: each track already blends into the next one.
/Underfire UDFRCD003 - Dangerous Drums Volume 2 (2000)/
/2001 - VRSCD003 - Ed Rush & Optical - The Creeps [Virus]/CD2/
```

The syntax is a subset of `.gitignore`. Lines starting with `#` are
comments. A pattern starting with `/`, or with a `/` in its middle, is a
path from the library root; otherwise it matches a name at any depth
(`CD2/` would leave out every folder named `CD2`). A trailing `/` matches
folders only. `*` matches within one name, `?` one character and `**` any
number of folders. Matching is case-sensitive, and square brackets match
themselves, so release folders such as `[Virus]` are written as they are.
There is no negation (`!`) and no backslash escape: to keep one disc of a
release, list the other, as above. Only the file at the root is read.

Every command that reads the library (`index`, `identify`, `explain` and
`stats` with a library, `doctor` and `prune`) then treats the files left
out as files not in the library: `index` does not decode them and searches
leave them out. `gunfinger doctor` shows how many audio files each pattern
leaves out and warns about a pattern that leaves out none, most likely a
typo or a renamed folder. Peak records made before a file was ignored stay
in the store until `prune --yes` deletes them; `doctor` and `prune` count
them apart from the records of files gone from the library. Searches
without the library (`--store-only`, or no library given) read only the
store, so they find ignored files until those records are pruned. Removing
a pattern brings its files back; `index` decodes them again if they were
pruned. Gunfinger never writes this file; the patterns are yours.

### One file per library

Keep one peak store per library: records are keyed by the path relative to
the library root. The store names its library in `library.txt`, which the
first `index` writes (an older store keeps its records and adopts the
library it is indexed with next), and every command refuses a store that
names another library; `prune` also refuses to delete most of a store,
which is what a wrong `--library` looks like. If the library moves, put
its new path in `library.txt`. With several libraries, give each its own
file:

```sh
target/release/gunfinger --config ~/.config/gunfinger/jungle.toml index
GUNFINGER_CONFIG=~/.config/gunfinger/jungle.toml target/release/gunfinger identify mix.m4a
```

### Without the library

Each peak record names its file, and `index` keeps each file's artist,
title and album beside its record, so the peak store alone is enough to
identify tracks and name them. When no library is given, or the library
cannot be read (an unmounted network share, for example), `identify`,
`explain` and `stats` search the store's current records and say so in one
line; `--store-only` does the same when the library can be read. Reports
name the library the store names, so `listen` and `review` play the tracks
once it is back. Without the library there is no telling which files have
changed or been deleted since they were indexed, or which the library's
`.gunfingerignore` leaves out: their records are searched until `index` or
`prune` replaces or removes them. A store indexed before
it kept tags gets them from the next `index`, which reads only the files'
headers.

To identify tracks on another computer, run `prune` (and `index`), copy the
store's directory, about 180 bytes per second of audio (`gunfinger stats`
shows its size), install Gunfinger and FFmpeg there, and name the copy:

```sh
target/release/gunfinger --peaks-dir /Volumes/STICK/peaks identify mix.m4a --format tracklist
```

The track length range in effect there applies, as it does to a library.
`gunfinger doctor --peaks-dir ...` checks the copy.

## Development

```sh
scripts/check.sh    # fmt, clippy with -D warnings, tests
```

`gunfinger-eval` is the evaluation harness. It reads a private corpus
(`corpus/library/` and `corpus/sets/<set>/tracklist.toml`) that is not part
of the repository:

```sh
target/release/gunfinger-eval validate [<set>]         # check the manifests, or one
target/release/gunfinger-eval clusters                 # duplicate rips in the library
target/release/gunfinger-eval clusters --from-peaks    # the same from stored peaks, compared
target/release/gunfinger-eval --other-peaks-dir STORE map-library   # corpus files' copies in a larger library
target/release/gunfinger-eval --other-peaks-dir STORE recall --seed 2026   # recall on the larger library's recordings
target/release/gunfinger-eval pair PAIRS.tsv           # two files' alignments and supported time
target/release/gunfinger-eval sweep --seed 2026        # speed sweep
target/release/gunfinger-eval --other-peaks-dir STORE sweep --seed 2026 --other-rips   # only other rips can answer
target/release/gunfinger-eval --verify sweep --seed 2026   # every detection measured by the peak verifier
target/release/gunfinger-eval shared PLAN.json         # queries from a passage two recordings share
target/release/gunfinger-eval scan <set> [--leave-out 3 --seed 2026]
target/release/gunfinger-eval calibrate                # confidence margin
target/release/gunfinger-eval baseline <name>          # save the standard reports
target/release/gunfinger-eval regress <name>           # rerun them and show what changed
target/release/gunfinger-eval robust --seed 2026       # excerpts under EQ, noise, codecs, key lock...
target/release/gunfinger-eval related                  # remixes and shared material in the library
target/release/gunfinger-eval hash-cost                # pairs and triplets: postings, lookups, evidence
target/release/gunfinger-eval mixes --count 12         # generated mixes with exact truth
target/release/gunfinger-eval grid                     # brief plays slid across the 10 s windows
target/release/gunfinger-eval loss                     # where the development mix loses evidence
target/release/gunfinger-eval memory --synthetic-copies 30   # memory by phase (run under /usr/bin/time -l)
target/release/gunfinger-eval memory --count-lines     # the first pass's lines and detections instead
```

`clusters` searches each file against the library on the turntable
ladder (0.92-1.08, so uploads a few percent fast are found), measures
each candidate pair again alone at its fitted speed, and joins two files
when one alignment covers 80% of the shorter one, counted in that file's
own seconds. The owner's verdicts in `docs/pair-verdicts.txt` (`same` or
`different`, then two paths, tab-separated) override that rule; run
`clusters` again after editing it. A cluster holds every file a chain of
joins reaches, so when joins through other files link two files judged
`different`, `clusters` stops and names the chain (with another
library's store, it checks the last run's pairs before searching).
With `--cut-sparsest` it instead cuts each such chain's measured join
with the fewest hits per second, never one judged `same`, and lists the
cuts (`cut_links` in the report) for the owner to judge. It
prints each pair's hits per second of aligned span and lists the joins
under a tenth of the median: their coverage rests on little evidence.
A verdict links only files the run searches: the corpus library's files
and, with another library's store, the files it holds a current record
of, never a file a library's ignore file leaves out (with a store alone,
read from the library the store names when it is mounted). Every other
verdict is printed with its line and adds nothing.

The sweep's held-out recordings and excerpts for each seed are drawn the
first time the seed is used and kept in `docs/panels/`, so the same
queries are measured as the library grows; `robust`, `mixes`, `grid` and
`hash-cost` use the same panels.

`--ladder both|turntable|key-lock` (before the command) chooses the rungs
every search uses; the default is `both`, as in `identify`, and other
ladders keep their reports in `work/reports/ladder-<name>/`. The harness
searches with `identify`'s matcher; options before the command change it
for every command in the same way: `--single-pass` is the matcher before
session 6 (every posting list, one pass, 200 hits in 3 windows), and
`--drop-fullest SHARE` empties the fullest posting lists instead of
setting them aside. Session 4's variants, not adopted:
`--speed-per-stretch` measures each stretch of 3 windows again at its
own speed when its hits drift from the fitted one; `--trim-ends` leaves
weak windows at either end out of a detection's boundaries, its evidence
unchanged; `--extra-rungs N` extends both ladders by N rungs past ±8%
(reports in `work/reports/extra-rungs-<n>/`), and `robust --only
speed+8.2pct,...` runs speeds from ±8.2% to ±9%. Their reports go
to `work/reports/variant-<name>/`, where `calibrate` and `regress` read
them when given the same options. `scan` and `robust` take
`--synthetic-copies N`, which adds N time-reversed, stretched copies of
every record to the index as a proxy for a larger library, and
`--second-library <root> --second-peaks-dir <store>`, which adds another
library's peak records; those reports are kept apart from the ones
`calibrate` and `regress` read. Peak profiles with neighbourhoods that
widen with frequency (`Profile::spread`) need a build of their own and a
store of their own under `work/variants/` (`--peaks-dir`, `--work`); `loss`
reports postings and true hits by anchor band.

A larger library the corpus was drawn from, which holds a copy of every
corpus file, can be measured without being mounted: `--other-peaks-dir
STORE` (before the command) names its peak store. `map-library` pairs each
corpus file with its copies there (identical peak records), and `clusters
--from-peaks` finds the other rips of the corpus recordings there,
following chains of rips; `--manifest <set>` (repeatable) and
`--recall-panel <seed>` add the other library's files that set's manifest
references and the recall panel's development sources to the queries.
A manifest may reference the other library's files as
`second-library/<path>`: with `--other-peaks-dir`, `validate` and `scan`
check them against the store; without it they are set aside, so a track
with no other reference counts as absent. Then `sweep`, `scan`, `robust` and `memory`
search an index of the corpus and the other library's remaining records,
named `second-library/<path>`, and count its rips of a recording as that
recording; held-out and left-out recordings take their rips with them.
The map, the clusters and the reports go to
`work/reports/library-<store directory name>/`, where `calibrate` and
`regress` read them when given the same option. `--other-sample N` adds
only N of the other library's records, a seeded choice (each smaller
choice is part of the larger), to measure against index size; its
reports go to a `sample-<N>/` directory below. `recall --seed N` draws a
panel from the other library's records outside the corpus recordings'
clusters (kept in `docs/panels/recall-seed-<N>.json`): recording families
(normalised artist and title) split into a development and a validation
half, each with 60 indexed and 20 held-out sources, a third each from
scene releases, YouTube channels and the rest. It renders the development
half's excerpts from the library's audio as the sweep does (`--prepare`
stops there), searches them, and says why each miss failed: never a
candidate, too few hits, or enough hits in too few windows.

Read [AGENTS.md](AGENTS.md) before changing code: it holds the house style,
the crate boundaries and the evaluation rules.

## Results

Measured on 262 library tracks (29.3 hours) with the rule frozen at tag
`poc-freeze-1`, searching turntable playback only. Searching both playbacks,
the default now, gives the same sweep, development and leave-out results
(experiment 0016); the test mix was searched that way in its third
evaluation, with the matcher below.

| Evaluation | Result |
|------------|--------|
| Speed sweep (80 excerpts × 9 speeds, −8% to +8%, MP3) | 100% recall at every speed, 0 wrong, speed error ≤ 0.016% |
| Development mix (56 min, 11 tracks) | 11/11 identified, 0 wrong, 22–26 s |
| Development leave-outs (3 and 11 tracks removed from the index) | 0 wrong |
| Held-out test mix (radio broadcast, 1 h 58 min, 31 tracks) | First run: 15/17 identified (88%), 0 wrong, 53 s. One miss was a remix not in the library; with the manifest corrected, 15/16 (94%). The other miss, cut by a radio insert, is shown as possible (second evaluation) |

Since session 6 the default matcher skips the most common hashes while
looking for candidates and measures each candidate again at its fitted
speed (ADR 0008). Over four sweep seeds (2026-2029, 2,160 excerpt queries)
recall stays 100% with 0 wrong, the development mix and leave-outs keep
their results, and the margin between the weakest identifying detection
and the strongest false candidate is 6.65× (4.15× with `--single-pass`,
experiments 0025 and 0047). Plays of 10 s are possible, no longer
confident through a chance window. On the test mix (third evaluation)
it identifies 14/16 with 0 wrong: Sick Note, confident at 209 hits under
the single pass's rule of 200, is possible at 233 hits under the rule of
240, and Sin stays possible.

The pass bar was at least 80% identified and zero wrong on each mix. The
index takes 74 postings per second of audio and 5.07 bytes per posting; for
25,000 tracks that projects to 3.0 GB in memory, or about 2.3 GB with the
delta-coded on-disk layout recommended in ADR 0005.

At scale, measured with synthetic copies on an idle machine (experiments
0028, 0029): the development mix takes 3.8 minutes at 8,122 assets and
12.0 at 26,462 (skip at 240: 1.5 and 3.8 minutes), and peaks at 2.9 and
7.7 GB (1.4 and 3.8 GB) with 10 workers. Most of that CPU sorts each
window's hits.

On the owner's collection of 27,042 tracks, from its peak store alone
(experiments 0050-0052): the development mix is identified 11/11 with 0
wrong in 4.4 minutes and 4 GB, the sweeps keep 100% recall, and the false
confident answers left come from a few pairs of records that may be the
same recording, for the owner to judge by ear.

Known limits: the thresholds were set on 262 real tracks; at 27,000
tracks unrelated chance reaches 63-70 hits, above the possible tier of
60, so possible plays are less reliable at that size
([docs/calibration.md](docs/calibration.md) lists what to measure again).
A search of turntable playback alone misses key-locked (pitch-preserved)
plays (experiment 0009). Heavy damage
(noise at 0 dB SNR, a blend at equal level) loses evidence but has not
produced a false identification (experiment 0009).
