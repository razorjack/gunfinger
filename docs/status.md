# Status

Running log, newest entry at the bottom. The proof-of-concept report is
the section "Final report"; later entries follow it.

## 2026-10-05: seed

- Tier: 1, seed and safety.
- Git repository on `master` (already initialised, no commits). `corpus/` is
  ignored by both the root `.gitignore` and `corpus/.gitignore` (`*`).
- Environment: macOS 27, Rust 1.98 (Homebrew, no rustup), ffmpeg 9.0.1,
  10 cores, 32 GB RAM.
- Corpus survey: 318 files under `corpus/library` (259 mp3, 5 m4a, 1 opus,
  the rest junk). Three set directories; `stakka-skynet-clockwork-mix-cd` has
  no `tracklist.toml` and is ignored.
- Next: Cargo workspace and a green `scripts/check.sh`.

## 2026-10-05: decode and peaks

- Tier: 3, peaks. Workspace of three crates; `scripts/check.sh` green.
- `gunfinger index corpus/library` extracts peaks of the whole library in
  48 s with 10 jobs: 265 audio files, 262 extracted, 3 failed, 53 other files
  skipped by extension (gif, jpeg, jpg, md, nfo, sfv, txt, one
  `.mp3_bad_or_incomplete`, one hidden file, two without extension).
- The 3 failures are CBR MP3s whose audio stops about 1.3 s before the length
  the container declares, with FFmpeg reporting "Header missing". The brief
  counts error output plus truncated audio as a failed asset. None is
  referenced by a manifest:
  `extra/03-Profound_Noize-Dropzone-sour.mp3`,
  `extra/kosheen [resist] -02- hide u.mp3`,
  `extra/rawkuts-gridlok-rkt337-2001-sour/a-gridlok-dilusion-sour.mp3`.
- `gunfinger-eval validate`: both manifests valid.
  `stakka-skynet-knowledge`: 11 tracks, all referenced (18 reference files).
  `ed-rush-optical-essential-mix`: 31 tracks, 17 referenced (24 files), 14
  absent.
- Front-end profile: 8 kHz, Hann 1024, hop 128 (16 ms), bins 5..500, local
  maximum over ±12 frames × ±12 bins, floor -10 dB. Peaks keep a fractional
  bin (parabolic interpolation, 1/64 bin).
- Next: pair hashing, in-memory index, `stats`, then hash survival under
  speed change.

## 2026-10-06: candidate A end to end

- Tier: 4, matching. Candidate A built: pair hashes (anchor in hybrid-log
  steps, Δbin, Δframe; 21 bits), in-memory index (offsets table + 4-byte
  postings), speed ladder 0.92..1.08 in 0.4% steps realised by a scaled STFT
  (experiment 0001), lines per 10 s window, windows chained into detections.
- `gunfinger identify`, `gunfinger stats` work. `stats` on the library: 262
  assets, 29.3 h, 37.3 peaks/s, 181.7 postings/s, 84.9 MB index (4.44 bytes
  per posting with offsets), peak store 187 bytes/s. Peaks by octave: 48% in
  2-4 kHz, 6.5% below 250 Hz (linear bins favour the top octave).
- Deviation: one smoke run of `identify` on the development set happened
  before duplicate clusters and the sweep (brief 7.2 and 7.3 order). It found
  all 11 tracks (best chains 4,097 to 13,976 hits) and showed fragmented
  same-asset chains, which led to the general "one asset plays once at any
  moment" rule. No threshold was set from it. Clusters use library audio
  only; their code and criterion were written before that run finished.
- Running: `gunfinger-eval clusters` (library self-match, ±2% ladder,
  coverage ≥ 80% of the shorter file).
- Next: clusters, sweep, confidence calibration.

## 2026-10-06: development bar met

- Tier: 5 done, starting 6 (index size).
- Duplicate clusters: 17 clusters with duplicates, criterion coverage ≥ 80%
  of the shorter file; same-recording pairs ≥ 0.984, all others ≤ 0.39
  (experiment 0002).
- Sweep seed 2026: recall 100% at all 9 speeds, 0 wrong answers, speed error
  ≤ 0.014% (experiment 0003).
- Development set: 11/11, 0 wrong, 56:09 scanned in 30.6 s. Leave-out 3 and
  leave-out 11: 0 wrong.
- Confidence rule `hits >= 500 && windows >= 3`. Margin 5.24× (weakest
  identifying 1,274 hits, strongest false 243 hits).
- Commands:
  `target/release/gunfinger index corpus/library`;
  `target/release/gunfinger-eval clusters`;
  `target/release/gunfinger-eval sweep --seed 2026`;
  `target/release/gunfinger-eval scan stakka-skynet-knowledge [--leave-out N --seed 2026]`;
  `target/release/gunfinger-eval calibrate`.
- Next: section 8 (density against recall, posting width, delta coding),
  then freeze and the test set.

## 2026-10-06: index size done; freeze

- Tier: 6 done (section 8), 7 starting.
- Density (experiment 0004): fan-out 2 with the ±12 neighbourhood is the
  leanest variant no worse than the best: 74.4 postings/s (baseline 181.7),
  sweep margin 5.27× (baseline 5.24×), development margin 20.2× (16.9×).
  Wider neighbourhoods (±14, ±16, ±20) lose margin; fan-out 1 loses sweep
  margin (4.87×).
- Postings: 4 bytes (15-bit asset, 17-bit frame) are lossless (8-byte
  postings gave the identical 4,181 detections). 16/16 with 32 ms frames
  costs 14% of the sweep margin. Delta + varint lists: 3.12 bytes/posting.
- `gunfinger stats`: 5.07 bytes/posting with the offsets table; buckets mean
  3.74, p99 55, max 1,796, fullest 1% hold 30.7%; peak store 187 bytes/s.
  25,000-track projection (402 s mean): 7.5e8 postings, 3.0 GB index
  (about 2.3 GB delta-coded), 1.9 GB peak store.
- Peak bands: share per octave follows bins per octave (48% in 2-4 kHz, 6.5%
  below 250 Hz); local-maximum picking is level-independent, so the bass does
  not monopolise the peaks and no spreading was applied.
- Rule recalibrated: hits ≥ 200 and windows ≥ 3. Final evaluation: sweep 100%
  at every speed, 0 wrong; development 11/11, 0 wrong (26.0 s); leave-out 3
  and 11: 0 wrong; margin 5.27× (weakest identifying 501, strongest false 95).
- ADRs 0001-0005 written; roadmap drafted.
- Next: tag `poc-freeze-1`, then the first test-set evaluation.

## 2026-10-06: test set, evaluation 1

- Tier: 7. Tagged `poc-freeze-1` at commit 407cc93, then ran
  `target/release/gunfinger-eval scan ed-rush-optical-essential-mix` once.
- Result: 15/17 referenced tracks identified (88.2%), 0 wrong
  identifications; 1:58:11 scanned in 52.8 s. Pass. Recorded verbatim, with
  the diagnosis of the two misses, in `docs/experiments/test-set-ledger.md`.
  Report: `docs/experiments/data/test-evaluation-1.json`.
- No further test evaluations.

## 2026-10-06: elegance pass and documents

- Tier: 8. `search` split into `search/lines.rs` (alignment per window) and
  `search/chains.rs` (detections across windows); `timecode` parsing
  rewritten as one match; `gunfinger-eval` restructured around a `Paths`
  argument group; constants used by one module made private; unused
  `thiserror` dependencies dropped from the CLI and the harness.
- Verified unchanged after the refactor: the development scan (4,181
  detections), both leave-outs, the sweep (720 queries, 9,577 detections),
  `calibrate` and `stats` give identical reports. `index` finds all 262
  records up to date.
- Experiment 0005: query time on the development mix is 3.7 s decoding,
  about 15.8 s for the 41-rung ladder and 2.6 s for lookups, lines and
  chains at 262 tracks.
- `README.md` written; ADRs, `AGENTS.md` and the roadmap brought up to
  date. Experiment 0001 named a 9-bit anchor; the code has always used 8
  bits (167 levels), and the note is corrected.
- `scripts/check.sh` passes: fmt, clippy `-D warnings`, 61 tests.

## Final report

All numbers below were measured in this session on 262 library tracks
(29.3 h, mean 402 s), on an Apple M1 Pro with 10 cores.

### 1. Verdict

| Set | Role | Referenced | Identified | Wrong | Verdict |
|-----|------|-----------:|-----------:|------:|---------|
| `stakka-skynet-knowledge` | development | 11 | 11 (100%) | 0 | pass |
| `ed-rush-optical-essential-mix` | held-out test | 17 | 15 (88.2%) | 0 | pass |

- The test result is the first run at `poc-freeze-1`. Test evaluations: 1
  of the 5 allowed.
- Development leave-outs (seed 2026): leaving out Night Lore, Side Effects
  and Kontempt with their clusters gives 8/11 and 0 wrong; leaving out all
  11 gives 0/11 and 0 wrong. No left-out slot produced a confident
  detection.
- Owner review (after this report; see the ledger): track 16 was the Planet
  V remix of Funktion, which is not in the library. The owner changed the
  manifest to mark it absent (now 16 referenced, 15 absent). Under it, the
  evaluation-1 detections score 15/16 (93.8%) with 0 wrong. No new run was
  made.

### 2. Candidate

Candidate A won: exact pair hashes (21 bits: anchor in hybrid-log steps,
bin difference, frame difference; fan-out 2) searched on a ladder of 41
speeds from 0.92 to 1.08, where each rung is an STFT with window and hop
scaled by the speed. Lines of hits per 10 s window are chained across
windows into detections. It met the development bar on its first build, so
candidate B was not built. The measurements behind the design (experiment
0001):

- with no residual speed error, a scaled STFT keeps 78.8% of hashes at
  speed 0.92 and transformed peak coordinates 39.3%; resampling the audio
  kept 72.9% and needs a decode per rung;
- 2% relative anchor steps keep 46% of hashes at ±0.2% residual speed
  error, against 26% for linear one-bin anchors, which sets the 0.4% ladder
  step.

### 3. Sweep

Seed 2026: 209 indexed assets, 48 clusters held out; per speed 60 indexed and
20 held-out 30 s excerpts, rendered with pitch and tempo together and
encoded as 128 kbit/s MP3. Held-out excerpts produced no confident
detection.

| Speed | Recalled | Recall | Wrong | Mean speed error | Max speed error |
|------:|---------:|-------:|------:|-----------------:|----------------:|
| -8% | 60/60 | 100% | 0 | 0.002% | 0.006% |
| -5% | 60/60 | 100% | 0 | 0.004% | 0.016% |
| -3% | 60/60 | 100% | 0 | 0.005% | 0.012% |
| -1% | 60/60 | 100% | 0 | 0.005% | 0.014% |
| 0% | 60/60 | 100% | 0 | 0.002% | 0.005% |
| +1% | 60/60 | 100% | 0 | 0.004% | 0.012% |
| +3% | 60/60 | 100% | 0 | 0.005% | 0.012% |
| +5% | 60/60 | 100% | 0 | 0.005% | 0.012% |
| +8% | 60/60 | 100% | 0 | 0.002% | 0.007% |

### 4. Wrong identifications and missed tracks

There are no wrong identifications in the sweep, the development set, its
leave-outs or the test set.

Missed tracks (both in the test set; the development set and the sweep have
none):

- **16. Ed Rush & Optical - Funktion.** The strongest detection of its
  references in its window has 18 hits in 2 windows, at -1.87%, 1:01:45 to
  1:01:54. The slot holds many short chains of 8 to 18 hits that agree on a
  speed of about -1.8%, so related audio was playing, but the library
  recording barely aligns with it. Owner: the version played is the Planet V
  remix, which is not in the library.
- **22. Fibre Optix - Sin.** Two chains of 84 hits (2 and 4 windows) at
  +5.4%, between 1:22:22 and 1:24:29. Both lie on the same speed but are 34 s
  apart, more than the 2 empty windows a chain may bridge, and each stays
  below 200 hits. Owner: the broadcast faded Sin out for a station insert and
  brought it back.

### 5. For the owner to check

The owner's answers to the first four items, given after the report, follow
each item and are recorded in the test-set ledger.

- **Funktion (test track 16):** whether the version played is the one in
  the library (`Ed Rush & Optical - Funktion.mp3` and its duplicate). The
  evidence above suggests a different version or mixdown. Owner: it is
  "Funktion (Remix)" from Planet V (1999); the library has only the
  original. The owner has updated the manifest.
- **Sin (test track 22):** whether it was played only briefly or cut in and
  out between 1:21:48 and the next track; the record was found twice at the
  same speed. Owner: faded out for a station insert, then brought back.
- **Sick Note (test track 31):** credited with 209 hits in 6 windows, just
  above the 200-hit rule. The recording ends 27 s after the track's listed
  start (1:57:45); the detection covers its last 46 s, from 1:57:25. Owner:
  played solo for under 30 s after a blend out of Slip Thru; the detection
  matches.
- **The Pulse (test track 10):** found as two confident chains, 34:50 to
  36:17 at +1.24% and 36:18 to 39:30 at +1.40%; both are credited to the
  same track. Owner: a needle skip at 36:17. The skip moved the position in
  the track, which starts a new chain.
- **Cluster credits:** none. Every credited detection in both sets is of a
  listed reference.
- **Duplicate clusters** (experiment 0002, `data/0002-duplicate-clusters.json`):
  17 clusters treated as the same recording. Two are worth a look: Decoy (a
  vinyl sampler rip and the Clockwork CD version, which is 22 s shorter) and
  Dominion (`Kraken - Dominion.opus` and the Kraken+Arkane MP3).
- **Damaged files:** three MP3s decode about 1.3 s short of their declared
  length with "Header missing" errors and are left out of the index:
  `extra/03-Profound_Noize-Dropzone-sour.mp3`,
  `extra/kosheen [resist] -02- hide u.mp3`,
  `extra/rawkuts-gridlok-rkt337-2001-sour/a-gridlok-dilusion-sour.mp3`.
  None is referenced by a manifest.
- **Ground truth:** no evidence of manifest errors. No confident detection
  falls outside its track's window.

### 6. Confidence margin

Rule (frozen): at least 200 hits and at least 3 windows in one chain.

| Population | Weakest identifying | Strongest false | Ratio |
|------------|--------------------:|----------------:|------:|
| Calibration: sweep, development, leave-outs | 501 hits, 3 windows | 95 hits, 2 windows | 5.27× |
| Test set (not used for calibration) | 209 hits, 6 windows | 24 hits, 3 windows | 8.7× |

- Calibration covered 559 identifying detections and 8,986 false
  candidates; the rule accepts every identifying one and none of the false
  ones. The threshold is 2.11× the strongest false candidate and 2.50× below
  the weakest identifying detection.
- Weakest identifying: SKC & Cord "Swarm", sweep excerpt at -3% (between
  rungs). Strongest false: the Stakka remix of Clockwork on the sweep excerpt
  of the original at +1%; in the development mix the same remix reaches 91
  hits in 2 windows while the original plays.
- In the test set the weakest credited detection is Sick Note (see above);
  the strongest unmatched one is the Gridlok remix of Mo Funk "Slipstream".

### 7. Index

From `gunfinger stats` (fan-out 2, ±12 × ±12 neighbourhood; experiment 0004,
ADR 0005):

| Measure | Value |
|---------|-------|
| Peaks | 37.3 per second of audio |
| Peak store | 19.7 MB, 187 bytes per second of audio |
| Postings | 7,836,148; 74.4 per second of audio |
| Index in memory | 39.7 MB; 5.07 bytes per posting with the 8.4 MB offsets table |
| Delta + varint posting lists (offline) | 4.19 bytes per posting with the offsets table, 3.12 without |
| Hash buckets (2^21) | mean 3.74, p99 55, max 1,796; fullest 1% hold 30.7% of postings |
| Peak bands (below 125, 250, 500, 1k, 2k, 4k Hz) | 4.5, 2.0, 5.3, 13.5, 26.7, 47.9% |

Projection to 25,000 tracks of 402 s (2,791 h): 7.48e8 postings, a 3.0 GB
index with 4-byte postings (about 2.3 GB delta-coded), and a 1.88 GB peak
store. The posting is 15 bits of asset and 17 bits of frame; 8-byte postings
gave identical results, and 16 asset bits with 32 ms frames cost 14% of the
sweep margin. Beyond 32,768 assets the index must be sharded or move to that
16/16 split.

### 8. Wall times

| Run | Audio | Wall time |
|-----|------:|----------:|
| `gunfinger-eval scan stakka-skynet-knowledge` | 56:09 | 26.0 s at the freeze; 22.5 s and 22.9 s after the elegance pass |
| `gunfinger-eval scan ed-rush-optical-essential-mix` | 1:58:11 | 52.8 s |
| `gunfinger index corpus/library`, from empty | 29.3 h | 48.3 s and 53.3 s |
| `gunfinger index corpus/library`, up to date | | 0.6 s |
| Building the index from the peak store | | 0.4 s |

Query time on the development mix (experiment 0005): 3.7 s decoding, about
15.8 s for the 41-rung ladder, about 2.6 s for lookups, lines and chains.

### 9. Deviations from the brief

- **Order of work.** One `identify` smoke run on the development set
  happened before duplicate clusters and the sweep (brief 7.2, 7.3). No
  threshold was taken from it; the clustering code and criterion were
  written before it finished.
- **Rule recalibrated in section 8.** The rule went from 500 to 200 hits
  when fan-out 2 reduced every hit count. This happened before the freeze,
  on the sweep, development and leave-out reports only.
- **Section 8 criterion.** Every density variant reached the same recall
  and zero wrong answers, so "no worse than the best" was decided on the
  calibration margins, over the sweep and the development set (experiment
  0004). That margin metric is my choice; the brief does not define one.
- **`--jobs` is global.** It applies to `identify` and `stats` as well as
  `index`. `identify` searches one file and runs the 41 rungs in parallel;
  the harness runs one file per worker with a single-threaded search, so
  there is no nested parallelism.
- **Code changed after the freeze.** The elegance pass changed code after
  `poc-freeze-1`. The sweep, the development scan and its leave-outs give
  identical reports apart from wall times and speeds that differ by at most
  4.4e-16, because the speed fit was rewritten (experiment 0007); the test
  set was not run again.

### 10. Limitations and the best next task

- The rule was calibrated against a 262-track library. At 25,000 tracks
  there are about 95 times as many postings; how strong chance alignments
  become at that size has not been measured.
- Lookup time grows with posting-list length. It is 2.6 s for the
  56-minute development mix at 262 tracks and has not been measured on a
  larger library.
- Evidence can be thin near the edges of a recording. The weakest credited
  test detection (209 hits) is within 5% of the threshold because the
  recording ends 27 s into the track.
- A chain bridges at most 2 empty windows, so a track that is cut in and out
  can fall apart into chains that are each below the rule. Sin was faded out
  for a station insert; even joined, its two chains hold 168 hits, below the
  rule.
- Remixes and VIPs that share long sections with the original are the
  strongest false candidates (95 hits). A remix sharing more material could
  cross the rule.
- Key-locked (pitch-preserved) mixes are out of reach of the speed ladder.
  Heavy EQ, filtering and long blends have had no specific work.
- The index is rebuilt in memory on every run; at 25,000 tracks that would
  be about 3 GB of RAM. Failed decodes are retried on every `index` run.

**Best next task:** measure the strongest false candidate and query time
against library size, by growing the library to thousands of tracks of
unrelated music, before trusting the 200-hit rule at collection scale. Then
build the on-disk index described in ADR 0005.

## 2026-10-06: plays and a possible tier; test evaluation 2

- After the owner's review of the test set (ledger), `identify` groups
  detections into plays: segments of one asset with gaps up to 90 s, such as
  The Pulse around its needle skip and Sin around a station insert. Below
  the frozen rule, a detection with at least 60 hits is `possible`. ADR 0006,
  experiment 0006.
- Possible threshold: at least twice every unrelated false candidate on the
  sweep and the development set (strongest 28 hits); the only false
  candidates above half of it are the Stakka remix of Clockwork.
- Scoring is unchanged. Development set: 11/11, 0 wrong, identical
  detections; one possible play matches no track (the Clockwork remix while
  the original plays).
- Test evaluation 2 (commit e87b79f): 15/16 (93.8%), 0 wrong, identical
  detections to evaluation 1; Sin found as possible; no possible play
  matches no track. Evaluations used: 2 of 5.
- `docs/calibration.md` lists every library-dependent choice, the baseline
  at 262 tracks and what to change or undo when the library grows.
- Next: measure the thresholds against a larger library (roadmap).

## 2026-10-06: library size beyond 32,768 assets (open)

- The owner may grow the collection to about 100,000 tracks with jungle and
  breakbeat hardcore. The posting layout addresses 32,768 assets. ADR 0007
  (status open) compares the options: shards by asset range, separate
  libraries per genre, shards by collection, wider or narrower postings, and
  decoding delta-coded lists on every lookup. It records the linear
  extrapolation to 100,000 tracks (12 GB index, about 35 minutes of lookups
  for a two-hour mix) and what grows whatever the layout: query time, chance
  and shared-break alignments, and the cost of duplicate clustering.
- No decision. The next measurements are listed in ADR 0007.

## 2026-10-06: session 2, command line, test infrastructure, transformed copies

Autonomous overnight session (`docs/brief-2.md`, checklist in
`docs/session-2-checklist.md`, findings in `docs/notes-for-owner.md`).
Default detection is unchanged: `regress session-2-start` reports
identical detections after every change to the core. No test-set
evaluation was spent (2 of 5 used).

- **Test infrastructure.** Search is deterministic, bit for bit
  (experiment 0007). A synthetic end-to-end test drives the real binary
  through a generated library and mix, with no private corpus (0008).
  `gunfinger-eval baseline` and `regress` save and compare the standard
  reports. Property tests cover plays, timecodes and pair hashing.
- **Command line.** Colour and `--quiet`; position in the track; plays of
  files with the same audio shown once; `--format
  human|timeline|json|csv|cue|tracklist` and `show` for saved reports;
  `explain` (every candidate around a moment); `listen` and `review` (by
  ear, via ffplay; not heard, as this machine has no audio device); batch
  identification with `--save-dir`; completions and a man page; a
  configuration file; `doctor`; `prune`; failed and too-long files are
  remembered until they change. Report schema 3.
- **Robustness (0009).** 32 transforms of 50 excerpts: no wrong
  identification; damage only costs hits. Key lock defeated the turntable
  ladder.
- **Key lock (0010).** Key-locked rungs (time stretched, frequency kept)
  find every key-locked excerpt at ±2-8% and leave turntable results as
  they were. Opt-in at first: `identify --playback both`; with both
  ladders the full protocol passes with unchanged thresholds (0016) at
  1.57 times the CPU time (0018). On by default since the owner's
  decision below.
- **Summed play evidence (0011).** Offline null: no change.
- **Scale proxy (0012).** Reversed, stretched copies of the library up to
  21,109 assets: chance alignments grow slowly; memory runs out first.
- **Lookup cost (0013, 0017).** Emptying the fullest 1% of posting lists
  cuts the postings a query scans by 62% and own-track hits by 17%, and
  removes most chance evidence; it passes the full protocol with a wider
  margin (5.86×). Harness option only; adopting it is the owner's call.
- **Triplet hashes (0014).** Estimated offline: 25-79 times fewer lookups,
  about half the evidence under heavy damage. Not built into search.
- **Related recordings (0015).** 21 pairs of different recordings share 30
  or more hits, 5 reach the possible tier (strongest 130, two Aphrodite
  drum versions), none 200.
- **Key lock through the full protocol (0016).** Both ladders pass with
  unchanged thresholds at about 1.5 times the search time (the first
  estimate, 1.3, came from noisy wall times; experiment 0018 measured
  1.57 times the CPU time). `playback` in the configuration file sets the
  default per library.
- Timings this night were noisy (load average above 100 while the
  harness ran); relative timings were interleaved, lookup savings counted
  exactly.

## 2026-10-06: owner checks by ear

- `listen` and `review` work with real audio output: the owner ran them on
  the development mix's saved report (`listen --at` at several moments,
  `review` with play numbers, `start`, `end` and `at`).
- The possible play of the Stakka remix of Clockwork at 20:22 is shared
  material: at that moment the remix carries the original's lead exactly,
  plus a pad. The owner considers the possible play correct and the
  fragment indistinguishable on its own. It is the passage experiment 0015
  found between the two recordings. Roadmap: show such a play as "shares
  material with" the confident play around it.
- The owner removed `stakka-skynet-clockwork-mix-cd` from `corpus/sets/`;
  it is not needed as an evaluation set.

## 2026-10-06: quieter output; key lock on by default

- `identify` and the other commands print timings, the index size and the
  files left out of the index only with `--verbose` (`-v`). Library files
  that need `gunfinger index` still get a one-line warning; a file
  remembered as damaged or too long names `--retry-skipped` instead of
  `gunfinger index`. `--format` has the short flag `-f`.
- Saved reports record their playback (`query.playback`), and
  `identify --save-dir` searches a recording again when it differs.
- Key lock is on by default: `identify`, `explain` and the harness search
  both playbacks unless `--playback` or `playback` in the configuration
  file says otherwise (owner's decision, experiment 0018). It costs 1.57
  times the CPU time; the earlier 1.3 came from noisy wall times. The
  owner's configuration file sets `playback = "turntable"`. The README
  documents the configuration file and the vinyl-only setting.

## 2026-10-06: session 3, memory and reports

Session 3 (`docs/brief-3.md`, checklist `docs/session-3-checklist.md`).
Baseline `session-3-start` saved after a rerun that reproduced
`key-lock-default` exactly.

- **Memory by phase (experiment 0019).** The build held records, points
  and postings at once: 5.0 GB at 8,122 assets of the scale proxy. The
  index is now built in two passes reading one peak record at a time
  (`Index::counting`, `Counting::into_filling`, `indexing::build_index`);
  `identify` and `explain` keep no records and `stats` reads them again.
  Peak at 26,462 assets: 3.1 GB. `gunfinger-eval memory` measures resident
  memory by phase; the harness has a global `--jobs`. The search is now
  the peak at scale: 1.9 GB (1 worker) to 4.7 GB (10 workers) for 10
  minutes of query at 8,122 assets. `regress session-3-start`: identical.
- **Reports.** A `search` block (profile, hash design, matching settings,
  confidence rule, library revision) and `query.requested_duration_seconds`;
  `--save-dir` searches again when anything differs. Atomic writes.
- **explain --windows.** Each window's lines and the chain that took
  them; explain's excerpt starts on the 10 s grid. `regress
  session-3-start`: identical.
- **Generated mixes and the window grid (experiment 0020).** `gunfinger-eval
  mixes` renders seeded mixes of library tracks with exact truth; `grid`
  slides brief plays across the 10 s grid. 107/115 plays confident, 8
  possible (20-26 s, 2 windows), none wrong. A 10 s minimum span would
  make all 15-20 s grid plays confident; offline only. Chains can take a
  chance line 30 s away.
- **Search memory at scale (experiment 0021).** At 26,462 assets the
  search state grows with the query (about 180 MB per minute), not with
  workers: 4.9 GB peak for 10 minutes of query.
- **Evidence loss and new conditions (experiment 0022).** `gunfinger-eval
  loss` attributes the development mix's lost hashes: the mix itself
  costs most, mostly at 1-4 kHz; blends three quarters more; the rung 7%.
  Robust conditions wow 0.55/0.75 Hz, broadcast, beatmatched -6/0 dB and
  combined; the beatmatcher accepts tempos found at a metrical factor.
- **Second pass (commit 633da10).** `search::search_twice`, opt-in;
  `Evidence` records its `Pass`, each with its own rule. Harness
  `--second-pass` and `--drop-fullest` are global and write to
  `reports/variant-<name>/`. `regress session-3-start`: identical.
- **Panels and clusters from peaks (experiment 0023).** Sweep panels in
  `docs/panels/`; `search::search_peaks`; `clusters --from-peaks` finds the
  same 17 clusters at a seventh of the CPU.
- **Second pass evaluated (experiment 0024).** With the frozen rule:
  margin 5.16× → 7.01×, sweep excerpts between rungs +40% own hits,
  recall up under damage (needle skip, broadcast, blends), no wrong
  answer; Star Trails -15% (speed wanders within the play).
- **Four sweep seeds (experiment 0025).** Seeds 2027-2029 on new panels:
  100% recall, 0 wrong each. Calibrate over four draws: weakest
  identifying 403, strongest false 97, margin 4.15×; audio not in the
  index up to 28.
- **Second pass with the filter (experiment 0026).** `--skip-fullest`
  (`Index::skipping_fullest`): the fullest 1% left out of the search for
  candidates, kept for the second pass. Drop and skip pass the protocol
  over four seeds; false candidates -94%. Calibration alone puts the
  opt-in `FITTED_RULE` at 240 hits (skip's strongest false is 119, the
  Clockwork remix's shared passage). `regress session-3-start`: identical.
- **Item 6 under damage and at scale (experiment 0027).** Robust with
  both ladders: skip at 240 confirms 1,995 against 1,959 today, drop
  1,884; no wrong answer. 57-63% less search CPU at 8,122 and 26,462
  assets; first-pass lines 1.74 million → 112,000. `memory` takes the
  matching options and `--count-lines`. Case for adoption in the notes.

## 2026-10-07: session 3 wrap-up

- All seven items done (experiments 0019-0027); checklist ticked.
- Default detection unchanged since `session-3-start` (`regress`:
  identical after every core change). Test set: 2 of 5 evaluations used,
  none this session.
- Opt-in, not adopted: the second pass (`search_twice`) with the fullest 1%
  of posting lists skipped for candidates, rule 240 hits in 3 windows.
  Case for adoption and the decisions for the owner at the top of
  `docs/notes-for-owner.md`; adoption steps in `docs/calibration.md`
  ("Measured, not adopted") and the roadmap.
- Measurements this session ran beside other sessions' load (load average
  up to about 200): CPU times compare, wall times and peak memory do not.

## 2026-10-07: track length range

- `--min-track` and `--max-track` (global, timecodes; `min_track` and
  `max_track` in the configuration file) replace `index
  --max-track-minutes`. Default unchanged: up to 20:00, no minimum.
- `index` rejects a file from the length its header declares when that is
  more than 10% outside the range (36 ms for the development mix's header
  against about 1.1 s to decode its first 15 minutes); nearer a limit the
  decoded length decides. Too-short files get skip notes (reason 2).
- `build_index` and the library revision leave out records outside the
  current range, so narrowing it needs no new index and `--save-dir`
  searches again when the indexed set changes. The harness loads records
  without a range: detection is unchanged (longest record 9:38).

## 2026-10-07: session 4 started

Session 4 (`docs/brief-4.md`, checklist `docs/session-4-checklist.md`).
`regress session-3-start` reproduced it exactly (identical detections);
baseline `session-4-start` saved; the session-start binaries are kept in
`work/bin/s4-start/` for the idle-machine measurements.

- Item 8 done first, in the gaps between timed runs, each its own commit:
  reports mark a possible play inside a confident play of another
  recording as sharing material with it (display only); `doctor` counts
  only files within the track length range against the 32,768-asset
  limit; the peak store names its library in `library.txt` (written by
  `index`, checked by every command; `work/peaks` has none yet, since
  this session never indexes into it); the harness's `--second-library`
  adds another library's records to the index, tested with ten reversed
  renders (development scan 11/11, 0 wrong).
- Item 1 under way: `sample` profiles and timed `identify` runs at 262
  assets done; scale runs running. Found: at 26,462 assets most of the
  search's memory is freed hit buffers kept by macOS's allocator (15.7 GB
  peak for 10 minutes of query with 10 workers, 5.0 GB with
  `MallocLargeCache=0`), not lines.
- Item 2 (commit d4d8fb1): the search runs a block of 12 windows on
  every rung at a time and merges each block's lines once every rung has
  searched it; each worker reuses one hit buffer. Merging rung by rung
  cannot be exact (`distinct` is greedy, so its result depends on every
  rung's lines of a window), merging by window can: a stretch of STFT
  frames gives bit-identical peaks (`extract_peaks_in`). `regress
  session-4-start`: every detection identical; the candidate matcher's
  development scan: no difference. About 1% more analysis at block edges.
- Items 3, 4, 6 and 7 are opt-in harness flags (commits afc731c, 804c65c):
  `--nearby-rungs`, `--strong-gaps`, `--speed-per-stretch`, `--trim-ends`,
  `--extra-rungs N` and `robust --only speed+8.2pct,...`; item 5's
  `Profile::spread` and postings by band in `loss` (commit d533142).
- Background on this machine tonight: the Aerial screensaver, an rsync
  and a fetch script of the owner's, and system daemons use about a fifth
  of the CPU; the 1-minute load average rarely falls below 2. Timed runs
  wait up to 2 minutes for it and log `top`'s idle share.
- Items 1 and 2 measured (experiments 0028 and 0029). Scale runs at 262,
  8,122, 26,462 and 31,964 assets, both matchers, three rounds each,
  stable to 2% in wall time and 1% in peak memory; no swap growth. At
  26,462 assets sorting each window's hits is 77-87% of the search's CPU.
  Item 2's block-wise search lowers peaks by 36-45% at 8,122 assets and
  14-22% at 26,462 with every scale report identical.
- Item 3 measured (experiment 0030): the link rules leave every sweep
  unchanged under both matchers. Under today's matcher they cut false
  candidates by 68% and the mixes' overshoot from 86 to 7.6 s; mix 10's
  Dominion and the grid's eight 10 s plays drop to possible (their third
  window was chance). Under skip at 240 the margin rises from 5.53× to
  6.65×. Star Trails splits into two segments at its speed change.
- Small fix found while preparing item 5 (commit 0651b12): with no
  indexed assets, the advice read "run `gunfinger index {}` first"; it now
  names the library. Test added; `scripts/check.sh` green.
- Item 6 measured (experiment 0031): `--trim-ends` takes the mixes'
  overshoot from 86.0 to 1.3 s under today's matcher but loses 218.8 s
  of true play on 72 of 115 plays; under skip at 240 it removes 0.5 s
  for 221 s. Not recommended; the link rules fix the overshoot at a
  tenth of the cost.
- Item 4 measured (experiment 0032, under skip at 240, since it needs the
  second pass): Star Trails +11% and the robust pitch ride +9% hits; wow
  unchanged; Dominion -3.2%; no level changes anywhere; no measurable CPU
  (development scan 241 s either way). Recommended to stay opt-in.
- Item 5 under way. `loss` by band: anchors at 2-4 kHz are 64% of the
  postings looked up and give 0.65 true hits per 1,000; at 125-250 Hz
  18.05. Variant a (neighbourhoods of 9.4% of the bin, 4-24 bins; store in
  `work/variants/band-a/`): 9% fewer postings; development mix +34% hits;
  sweep 2026 and scans margin 5.16× → 6.44×. Robust and the full protocol
  running.
- Item 7: 3 extra rungs at each end recover every speed to ±9% (today's
  ladders: 100% to ±8.4%, 0 confident at ±9%) for 14% more CPU; today's
  matcher's sweeps and mixes are otherwise identical.
- Item 7 done (experiment 0033): full protocol with 3 extra rungs under
  both matchers: sweeps, margins, scans, mixes and grid unchanged; false
  candidates +8-11%; every edge speed to ±9% 40/40. The grid was left out
  of later protocols (not in the brief's protocol; 37 minutes under load).
- Item 5 done (experiment 0034): two band profiles in their own stores,
  full protocol under both matchers. Variant a: 9% fewer postings, the
  development mix +34% true hits, margins 4.15× → 4.96× and 5.53× →
  5.82×, robust level; shared-material false candidates grow. The
  restored full robust report under skip at 240 (2,031 confident) was of
  unknown provenance; a fresh run gives 1,995, as in session 3, and is
  the baseline used. CPU of the variants not measured: the machine was
  loaded by other jobs from early afternoon.
- Session 4 complete: all eight items done and committed; README,
  roadmap, calibration and the session 4 summary at the top of
  `docs/notes-for-owner.md` updated. No test-set evaluation spent; default
  detection unchanged.

## 2026-10-07: portable peak store and NAS evaluation merged

- Built in a worktree beside session 4 (`docs/brief-nas.md`,
  `docs/nas-checklist.md`), rebased onto session 4's end and
  fast-forwarded. `regress session-4-start`: identical detections.
- The peak store keeps tags (`<hash>.tags`); reports carry each play's
  tags; `identify`, `explain` and `stats` work from the store alone;
  `doctor` checks a store without a library; listing a library shows
  progress.
- Harness: `--other-peaks-dir`, `map-library`, `clusters --from-peaks`
  around the corpus recordings; `sweep`, `scan`, `robust` and `memory`
  at the other library's scale; reports in
  `work/reports/library-<store>/`.
- Owner's verdict: Bad Company's China Cup and The Nine are different
  tracks that share their drums (notes for the owner, "After session
  4").
- Next: the NAS analysis in `docs/nas-plan.md`, once the owner's index
  of `/Volumes/atlas/Music/dnb` finishes.

## 2026-10-07: session 5 started

Session 5 (`docs/brief-5.md`, checklist `docs/session-5-checklist.md`):
measurements at the size of the owner's NAS collection, from its peak
store `~/.local/share/gunfinger/nas-dnb-peaks` alone. The NAS is never
read; the store is frozen.

- The NAS store at the start: 26,890 `.peaks`, 26,890 `.tags`, 1,549
  `.skip`, `library.txt` (naming `/Volumes/atlas/Music/dnb`), 1.9 GB.
  Digest of every file's name, size and mtime (`cd <store>; find . -type
  f -exec stat -f '%N %z %m' {} + | sort | shasum -a 256`):
  `7055ad8e3b22b12bc6dc5037eca25f424ad536bd96f788e813cceb3688465a70`.
- Machine: load average 2.3 and falling, swap 2.1 GB used of 3 GB (left
  over), no local external disk attached.
- `regress session-4-start`: 720 of 720 sweep queries and every
  detection of the three scans identical. Baseline `session-5-start`
  saved; session-start binaries in `work/bin/s5-start/`.
- Harness `fullest` (commit d7db672): each indexed record's postings in
  the lists `--skip-fullest` sets aside; `scripts/analysis/store_census.py`
  summarises a store from headers, tags and skip notes. `regress
  session-5-start`: identical.
- Item 1 (experiment 0035): the owner's index run read 84 Mbit/s over
  Wi-Fi, 3.4 times slower than the CPU allows (inference). `stats` on
  the store: 755 M postings, 4.01 bytes per posting, bucket p99 4,538;
  78 s, 3.07 GB.
- Item 2 (experiment 0036): 247 of 262 corpus files have a NAS copy; 15
  have none (11 not on the NAS, 4 failed to open there). The NAS-scale
  index holds 26,905 assets.
- Timed runs from here use `work/scripts/timed5.sh`: the same logging as
  `timed.sh`, but it waits at most 60 s for the 1-minute load average to
  fall below 3 (the machine idles near 2-3; the CPU idle share before
  each run is logged).
- Item 3 restarted at 18:37: the first clusters run was started under
  the agent tool's 30-minute limit for background commands and would
  have been stopped at about a third of round 1 (5.5 queries a minute).
  Long runs now start with `nohup`. `work/scripts/s5-queue.sh` runs items
  5-9 one timed run at a time once the clusters finish.
- The NAS store's library revision at the start: `d709390272e41475`
  (26,890 records), from `scripts/analysis/store_revision.py`, which
  reproduces the CLI's revision of the corpus store (`575e370289be7883`,
  as in session 4's `identify` reports).
- Item 3 (experiment 0037): `clusters --from-peaks` at NAS scale, 4,328
  s wall, 38,056 s CPU, 6.7 GB; the 17 corpus clusters reappear exactly;
  170 further rips; 233 clusters with duplicates; 76 borderline pairs.
  The coverage gap narrowed: same recording from 0.825, different up to
  0.737.
- Item 4 (experiment 0038): 292 pairs of different recordings with 30
  hits or more, 63 at 60 or more, 22 at 200 or more (16 probably one
  recording kept apart by the 80% rule, 4 versions, 2 one artist's other
  title); the strongest unrelated 153.
- Item 5 under way. Skip at 240 at NAS scale: development scan 11/11
  with 6 wrong (YouTube uploads of the played tracks, 3-4.7% faster than
  the corpus rips, outside the clustering ladder); leave-outs 3 and 11:
  8/11 and 0/11, the same 6 wrong and nothing else confident. 260-266 s
  wall, about 1,960 s CPU, 4.0 GB per scan. Notes for the owner updated.
- Re-plan at 20:06 from the measured runs: clusters took 72 minutes, not
  under an hour; skip scans 4.4 minutes. Expected: today's scans 3 x 13
  minutes, skip sweeps 4 x 22, today's sweeps 2 x 64, then memory,
  fullest and identify (35 minutes): the queue ends near 00:55. Items
  11-13 will most likely not fit.
- Item 5 done (experiment 0039): both matchers 11/11 with the same 6
  wrong (fast uploads); leave-outs 8/11 and 0/11 with the same 6.
  Today's 864.6 s, 6,097 s CPU, 8.37 GB; skip 260.6 s, 1,963 s, 4.01 GB.
  Strongest unrelated 40 and 48 hits (21-24 at 262 tracks).
- Item 6, skip at 240 (experiment 0040): sweeps 2026-2029 2,160/2,160,
  weakest 661; 523 confident false (507 other uploads of the same track,
  8 Synthesis VIP ~ mixed-CD "Synthesis (Remix)", 8 Coma ~ Spraycan);
  margin 0.11x under the rules, 3.0x with those three groups confirmed
  by the owner. 22 minutes per seed. Today's sweeps running (2026 from
  22:27).
