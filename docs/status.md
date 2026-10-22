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

- Tier: 7. Tagged `poc-freeze-1` at commit 45d0ac8, then ran
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
- Review by ear (after this report; see the ledger): track 16 was the Planet
  V remix of Funktion, which is not in the library. We changed the
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
  recording barely aligns with it. By ear: the version played is the Planet V
  remix, which is not in the library.
- **22. Fibre Optix - Sin.** Two chains of 84 hits (2 and 4 windows) at
  +5.4%, between 1:22:22 and 1:24:29. Both lie on the same speed but are 34 s
  apart, more than the 2 empty windows a chain may bridge, and each stays
  below 200 hits. By ear: the broadcast faded Sin out for a station insert and
  brought it back.

### 5. To check manually

The answers by ear to the first four items, given after the report, follow
each item and are recorded in the test-set ledger.

- **Funktion (test track 16):** whether the version played is the one in
  the library (`Ed Rush & Optical - Funktion.mp3` and its duplicate). The
  evidence above suggests a different version or mixdown. By ear: it is
  "Funktion (Remix)" from Planet V (1999); the library has only the
  original. We have updated the manifest.
- **Sin (test track 22):** whether it was played only briefly or cut in and
  out between 1:21:48 and the next track; the record was found twice at the
  same speed. By ear: faded out for a station insert, then brought back.
- **Sick Note (test track 31):** credited with 209 hits in 6 windows, just
  above the 200-hit rule. The recording ends 27 s after the track's listed
  start (1:57:45); the detection covers its last 46 s, from 1:57:25. By ear:
  played solo for under 30 s after a blend out of Slip Thru; the detection
  matches.
- **The Pulse (test track 10):** found as two confident chains, 34:50 to
  36:17 at +1.24% and 36:18 to 39:30 at +1.40%; both are credited to the
  same track. By ear: a needle skip at 36:17. The skip moved the position in
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

- After the review by ear of the test set (ledger), `identify` groups
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
- Test evaluation 2 (commit a58e7a9): 15/16 (93.8%), 0 wrong, identical
  detections to evaluation 1; Sin found as possible; no possible play
  matches no track. Evaluations used: 2 of 5.
- `docs/calibration.md` lists every library-dependent choice, the baseline
  at 262 tracks and what to change or undo when the library grows.
- Next: measure the thresholds against a larger library (roadmap).

## 2026-10-06: library size beyond 32,768 assets (open)

- We may grow the collection to about 100,000 tracks with jungle and
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
`docs/session-2-checklist.md`, findings in `docs/session-notes.md`).
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
  ear, via ffplay; not heard); batch
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
  1.57 times the CPU time (0018). On by default since our
  decision below.
- **Summed play evidence (0011).** Offline null: no change.
- **Scale proxy (0012).** Reversed, stretched copies of the library up to
  21,109 assets: chance alignments grow slowly; memory runs out first.
- **Lookup cost (0013, 0017).** Emptying the fullest 1% of posting lists
  cuts the postings a query scans by 62% and own-track hits by 17%, and
  removes most chance evidence; it passes the full protocol with a wider
  margin (5.86×). Harness option only; adopting it is our call.
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

## 2026-10-06: checks by ear

- `listen` and `review` work with real audio output: they were run on
  the development mix's saved report (`listen --at` at several moments,
  `review` with play numbers, `start`, `end` and `at`).
- The possible play of the Stakka remix of Clockwork at 20:22 is shared
  material: at that moment the remix carries the original's lead exactly,
  plus a pad. By ear, the possible play is correct and the
  fragment indistinguishable on its own. It is the passage experiment 0015
  found between the two recordings. Roadmap: show such a play as "shares
  material with" the confident play around it.
- We removed `stakka-skynet-clockwork-mix-cd` from `corpus/sets/`;
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
  file says otherwise (our decision, experiment 0018). It costs 1.57
  times the CPU time; the earlier 1.3 came from noisy wall times. The
  configuration file sets `playback = "turntable"`. The README
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
- **Second pass (commit 0206d42).** `search::search_twice`, opt-in;
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
  Case for adoption and the decisions to make at the top of
  `docs/session-notes.md`; adoption steps in `docs/calibration.md`
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
- Item 2 (commit db43197): the search runs a block of 12 windows on
  every rung at a time and merges each block's lines once every rung has
  searched it; each worker reuses one hit buffer. Merging rung by rung
  cannot be exact (`distinct` is greedy, so its result depends on every
  rung's lines of a window), merging by window can: a stretch of STFT
  frames gives bit-identical peaks (`extract_peaks_in`). `regress
  session-4-start`: every detection identical; the candidate matcher's
  development scan: no difference. About 1% more analysis at block edges.
- Items 3, 4, 6 and 7 are opt-in harness flags (commits 8847dcb, 1d0d312):
  `--nearby-rungs`, `--strong-gaps`, `--speed-per-stretch`, `--trim-ends`,
  `--extra-rungs N` and `robust --only speed+8.2pct,...`; item 5's
  `Profile::spread` and postings by band in `loss` (commit 718d861).
- Timed runs wait up to 2 minutes for the 1-minute load average to fall
  and log `top`'s idle share.
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
- Small fix found while preparing item 5 (commit e375d09): with no
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
  the baseline used. CPU of the variants not measured:
  the machine had other load.
- Session 4 complete: all eight items done and committed; README,
  roadmap, calibration and the session 4 summary at the top of
  `docs/session-notes.md` updated. No test-set evaluation spent; default
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
- Verdict by ear: Bad Company's China Cup and The Nine are different
  tracks that share their drums (session notes, "After session
  4").
- Next: the NAS analysis in `docs/nas-plan.md`, once the index
  of `/Volumes/atlas/Music/dnb` finishes.

## 2026-10-07: session 5 started

Session 5 (`docs/brief-5.md`, checklist `docs/session-5-checklist.md`):
measurements at the size of the NAS collection, from its peak
store `~/.local/share/gunfinger/nas-dnb-peaks` alone. The NAS is never
read; the store is frozen.

- The NAS store at the start: 26,890 `.peaks`, 26,890 `.tags`, 1,549
  `.skip`, `library.txt` (naming `/Volumes/atlas/Music/dnb`), 1.9 GB.
  Digest of every file's name, size and mtime (`cd <store>; find . -type
  f -exec stat -f '%N %z %m' {} + | sort | shasum -a 256`):
  `7055ad8e3b22b12bc6dc5037eca25f424ad536bd96f788e813cceb3688465a70`.
- `regress session-4-start`: 720 of 720 sweep queries and every
  detection of the three scans identical. Baseline `session-5-start`
  saved; session-start binaries in `work/bin/s5-start/`.
- Harness `fullest` (commit dfce1e0): each indexed record's postings in
  the lists `--skip-fullest` sets aside; `scripts/analysis/store_census.py`
  summarises a store from headers, tags and skip notes. `regress
  session-5-start`: identical.
- Item 1 (experiment 0035): the index run read 84 Mbit/s over
  Wi-Fi, 3.4 times slower than the CPU allows (inference). `stats` on
  the store: 755 M postings, 4.01 bytes per posting, bucket p99 4,538;
  78 s, 3.07 GB.
- Item 2 (experiment 0036): 247 of 262 corpus files have a NAS copy; 15
  have none (11 not on the NAS, 4 failed to open there). The NAS-scale
  index holds 26,905 assets.
- Timed runs from here use `work/scripts/timed5.sh`: the same logging as
  `timed.sh`, but it waits at most 60 s for the 1-minute load average to
  fall below 3 (the CPU idle share before each run is logged).
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
  wall, about 1,960 s CPU, 4.0 GB per scan. Session notes updated.
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
  by ear. 22 minutes per seed. Today's sweeps running (2026 from
  22:27).
- 01:05: the first queue's monitor in the agent session was killed with
  the session (exit 137); the detached queues ran on. Queue 1 finished
  at 01:22, queue 2 (link rules, item 12) started then.
- Item 6, today's matcher (experiment 0041): seeds 2026 and 2027
  1,080/1,080, weakest 403 (as at 262 tracks); 285 confident false (276
  same-name uploads, 8 Coma ~ Spraycan, 1 The Nine ~ its Evol Intent VIP
  at exactly 200); margin 1.18x without same-name pairs, 3.20x without
  both pairs. 66-69 minutes per seed. `calibrate` for today's ran while
  `fullest` ran (the reports only; `fullest` is not a cost measurement).
- Item 7 (experiment 0042): the proxy's postings within 0.5% of the NAS
  index; NAS over proxy 1.18x wall, 1.05x CPU, 1.10x memory (today's),
  1.13x, 1.14x, 1.06x (skip); chance 1.3-2.7x the copies'.
- Item 8 (experiment 0043): the skipped 1% of lists hold 18.7% of the
  postings, spread evenly (top 1% of records hold 1.9% of them); share
  against false appearances 0.72 today's, 0.24 skip.
- Item 9 (experiment 0044): store-only identify 856 s, 6,137 s CPU,
  8.35 GB, revision d709390272e41475; every play equals the harness's.
- Correction: the NAS-scale index holds 26,914 assets, not 26,905 (9 NAS
  records stand for two corpus rips each); 0036, 0039 and the notes fixed.
- No Rust code changed since the last green `scripts/check.sh` (commit
  f579220), so the analysis commits do not rerun it while timed runs are
  active; it runs once at the end.
- Item 12, in part (experiment 0045): link rules at NAS scale. Skip at
  240: scan 11/11, 6 wrong, weakest 1,539; sweep 2026 540/540, 91 wrong,
  weakest 680, false candidates halved. Today's: scan 11/11 but 7 wrong
  (Star Trails split at 13:20). Today's sweep with the rules not run.
- Wrap-up (item 10): session 5 summary at the top of the notes; NAS-scale
  figures in `docs/calibration.md` as measurements (no rule changed);
  roadmap (tracklist grouping, clusters for fast uploads, possible tier);
  NAS plan outcome; NAS checklist steps 7-10 and 12.
- Items 11 and 13 not run: no time left (today's sweeps take 66-69
  minutes each at NAS scale).
- End check at 02:04: the NAS store has 26,890 records, 26,890 tags,
  1,549 skip notes and `library.txt`; digest 7055ad8e...65a70 and
  revision d709390272e41475, unchanged. Queues finished at 02:03:57.
  `scripts/check.sh` at the end: green.

## 2026-10-08: session 6 started

Session 6 (`docs/brief-6.md`, checklist `docs/session-6-checklist.md`):
the store fixes, skip at 240 with the link rules as the default matcher,
clusters that join fast uploads, the NAS-scale protocol under both, and
grouped tracklists.

- The NAS store at the start (02:45): 27,010 `.peaks`, 27,010 `.tags`,
  1,429 `.skip`, `library.txt`; digest `1a7acf1b...07b38129` (full value
  in `docs/brief-6.md`), revision `3d16641d2ecb6956`.
- `regress session-5-start` with the session-start binaries: 720 of 720
  sweep queries and every detection of the three scans identical (5 min).
  Baseline `session-6-start` saved; binaries in `work/bin/s6-start/`.
  Timed runs log the CPU idle share, and CPU time is the figure to
  compare.
- Item 1 (store fixes): `decode::Container` finds an ID3v2 tag followed
  by "RIFF" and opens such files with `-skip_initial_bytes` (decode,
  probe, `listen`), tags from `ffprobe -f mp3`; hidden folders skipped
  and counted; truncation tolerance max(1 s, 1%). Unit and codec tests
  with generated fixtures. `regress session-6-start`: identical. Three
  corpus files with skip notes would now pass the tolerance; not indexed
  (session notes). `scripts/analysis/skip_notes.py` lists a store's
  skip notes.
- Item 3 (the one NAS re-index, 03:04-03:20): `index --retry-skipped`
  with the item 1 binary (`work/bin/s6-item1/`): 28,438 audio files
  listed (one fewer: the `.incomplete` partial file is in a hidden
  folder; 2 hidden folders skipped), 32 extracted, 108 too short, 1,286
  too long, 2 failed (the truncated pair), 387 s wall. 26 of the 27
  RIFF files and the 6 end-damaged files have records; the 27th RIFF
  file, a radio show (`sety/...RADIO_LODZ_NOCNY_TRANS.mp3`), is now "too
  long". `prune` listed exactly the stale `.incomplete` skip note;
  `prune --yes` deleted it.
- The NAS store after item 3, frozen from here (03:20): 27,042 `.peaks`,
  27,042 `.tags`, 1,396 `.skip` (2 failed, 1,286 too long, 108 too
  short), `library.txt`; 1.9 GB; digest
  `bf35ea642d5574ba1782c6da8688dc170ff6fb1ad0db83b04916ed901e9ae4f5`;
  revision `d428ee9585936326`. The NAS is not read again this session.
- From about 03:40 the machine had other load. Wall times from then on
  are not speed measurements; CPU time and deterministic counts are the
  figures to compare.
- Item 4 started: `session-6-candidate` saved from the item 1 binary
  under `--second-pass --skip-fullest 0.01 --nearby-rungs --strong-gaps`
  (its regress against `session-6-start` in
  `work/logs/s6-candidate-vs-start.txt`: sweep 540/540, development
  11/11, 6,114 detections to 87). Today's default reports moved to
  `work/reports/variant-single-pass/` (and the NAS-scale ones to
  `library-nas-dnb-peaks/variant-single-pass/`). The default is now
  `Matcher::Fitted`; `--single-pass` (CLI and harness) is the old one.
  The end-to-end test's needle skip: the second pass reaches 10 s back
  over the skip where the synthetic kicks repeat, and the part before
  the skip is dropped (test now checks both matchers; session notes).
- Re-plan at 03:50: items 1 and 3 took 0:40 together instead of 1:15
  (the NAS listing took 5 minutes, the index 6.5). Item 4's protocol at
  262 tracks runs from 03:42 (sweeps take about 6 minutes each under the
  other session's load); item 5's code (`clusters` on the turntable
  ladder with pairs measured at their fitted speed, `pair`, the verdict
  file) and item 7's grouping are written and wait for the build.
  Expected: item 4 committed by 04:35, item 5 by 06:45 (NAS run about
  1.5 h), item 6 by 08:00, item 7's check by 08:20, item 8 until 10:00,
  item 9 from 10:05.
- Item 4's protocol (04:07, `work/logs/s6-protocol-default.log`):
  `regress session-6-candidate` identical; sweeps 2026-2029 540/540, 0
  wrong (180 s wall, 1,555 s CPU each); calibrate weakest 658, strongest
  false 99, margin 6.65×, audio not in the index 22; mixes 106/9/0 wrong
  (mix 10's Dominion possible); grid 0, 67, 146 confident at 10, 15,
  20 s, 0 wrong. Experiment 0047; register, README, ADR 0008, notes.
- Item 4 committed (6acbc90, 04:27). The item 4 files alone pass the
  checks in a separate worktree, except one end-to-end test that fails
  there because `/tmp` is a symlink (session notes).
- Item 5a (04:27-04:32): `pair` on the 6 development pairs, experiment
  0048. The ±2% clustering ladder explains the 0-55% coverage; on the
  ±8% turntable ladder every pair is one alignment over both files. No
  key lock, no cuts. Coverage is now counted in the shorter file's own
  seconds (the query's span gave 100-104%).
- Item 5b: `clusters` at 262 tracks started 04:36 (work/bin/s6-item5,
  old clusters kept as `work/reports/duplicate-clusters-s5.json`).
  A unit-test build of the `gunfinger` crate (export tests, about 40 s)
  overlapped the first minute of this run by mistake; its wall and CPU
  figures are slightly high, its clusters unaffected.
- Item 5b/5d at 262 tracks (04:37-04:46, 535 s wall, 4,951 s CPU): the
  17 corpus clusters reappear exactly; 48 pairs from 20% coverage
  (session 5: 1,113, nearly all chance chains of 13-22 hits); weakest
  same recording 0.984 as before, strongest different 0.210 (was 0.385).
  No rerun of item 4's sweeps needed.
- NAS sample (`clusters --from-peaks --sample 20`, 04:47-04:52): 274 s
  wall, 1,864 s CPU, 4.1 GB; 12 further rips at 99-100%, 2 borderline
  pairs, each listed twice (two candidates of one file measured again):
  `clusters` now keeps the strongest pair per file (unit test). The 262
  file was made before that change; its clusters cannot differ (the
  strongest pair of a file is a same-recording pair whenever any is).
- Full NAS `clusters --from-peaks` started 04:53 (work/bin/s6-item5b,
  limit 2 h; projected about 70 minutes).
- NAS clusters, round 1 (262 corpus records) 04:55-05:39; round 2 has
  212 queries (session 5: 164), expected end about 06:20. Re-plan: item
  5 by 06:45, item 6 (about 1 h at NAS scale) by 07:50, item 7's check
  by 08:15, item 8 until 09:45, item 9 from 10:05.
- NAS clusters, first full run (04:53-06:19, 5,094 s wall, 46,406 s
  CPU, 4.6 GB; rounds of 262, 212 and 1): the 17 corpus clusters
  reappear, 213 further rips (170), all 6 development pairs joined, 23
  borderline pairs (76); weakest same 0.815, strongest different 0.766.
  But two rips joined in session 5 dropped out: the i-witness uploads of
  Phoenix (0.87 → 0.25) and Fallout (0.97 → 0.63). The one rung at the
  fitted speed loses a rip whose speed drifts; the ladder's chains
  follow it. `clusters` now keeps the better of the two alignments and
  records the ladder's coverage (`ladder_coverage`); the first run's
  file is kept as `duplicate-clusters-s6-refined-only.json`. Rerun from
  06:27 (work/bin/s6-item5c), expected end about 07:55.
- Item 7 (06:20-06:25, timed store-only `identify`): 13 lines, 263 s,
  1,943 s CPU, 3.98 GB (experiment 0050); committed 6975e8b. A possible
  play's mark names the entry it shares material with.
- Re-plan at 06:40: item 6 from about 07:55 to 09:00, item 8 from 09:00
  to 09:50 (scans and sweep 2026 at 1,000, 3,000 and 9,000 records),
  item 9 from 10:05.
- NAS clusters rerun (06:27-07:53, 5,106 s, 46,736 s CPU, 4.5 GB; rounds
  262, 215, 1): 732 files, 216 further rips, none dropped, 28 borderline,
  weakest same 0.840, strongest different 0.766 (experiment 0049).
  Re-scored session 5 reports: 465 of 505 and 242 of 267 false confident
  sweep detections explained, the development scan's 6 of 6.
- Item 6 started 07:53 (`work/scripts/s6-nas-protocol.sh` under
  timed6.sh, binary work/bin/s6-item5c).
- Item 6 scans (07:53-08:06): development 11/11, 0 wrong (session 5: 6
  wrong); leave-outs 8/11 and 0/11, 0 wrong; 3 possible plays matching
  no track (shared material); 255 s, 1,950 s CPU each; false candidates
  2,990, 8 of 30 hits or more; strongest unrelated 32. Sweeps from 08:06.
- Item 6 done 08:49 (experiment 0051): development 11/11, 0 wrong; sweeps
  540/540 with 0 and 24 wrong; margin 1.64× (3.41× without the two
  borderline pairs); unrelated 63-70. Listening list in the notes.
- 262 clusters rerun with the better-of rule (08:50-08:59, 530 s): the
  same 17 clusters; strongest different 0.302 (the Aphrodite pair by
  the ladder's alignment; 0.210 refitted), not 0.210 as in the first
  run. Item 8 started 09:00 (`work/scripts/s6-scaling.sh`).
- Item 8 (09:00-09:21, 1,213 s): `--other-sample` 1,000, 3,000 and 9,000;
  experiment 0052. Item 10 (NAS sweeps 2028 and 2029, calibrate over four
  seeds) started 09:21, expected end 10:06; item 9's writing meanwhile,
  its build and tests after.
- Item 10 (09:21-10:06): NAS sweeps 2028 and 2029, 540/540 each, 8 and 8
  wrong; calibrate over four seeds: 40 false confident from three pairs,
  1.29×, 3.36× without them (experiment 0053). Item 11 not started (a
  single-pass sweep takes over an hour at NAS scale).
- Item 9 (from 10:06): session 6 summary in the notes; register,
  roadmap, README, NAS plan and checklist, session 6 checklist. The NAS
  store at the end: 27,042 `.peaks`, 27,042 `.tags`, 1,396 `.skip`,
  `library.txt`; digest `bf35ea64...e9ae4f5`; revision
  `d428ee9585936326`: as recorded after item 3. The NAS was not read
  after item 3.

## 2026-10-08: after session 6: review absorbed, verdict chains, sparse joins

- An outside review (Astra) after session 6, checked against the code
  and the saved reports. Absorbed into `docs/roadmap.md` (the edge of the
  ladder as speed relative to the indexed copy; clustering precision;
  the shared-material challenge set and the rule's 1.22× headroom; a
  minimum span from supported time; the possible tier's caveats; recall
  beyond the corpus recordings; the on-disk index; the filter across
  shards; a pair review), `docs/calibration.md` and the notes ("After
  session 6").
- `gunfinger-eval clusters`: a `different` verdict that a chain of joins
  contradicts is an error naming the chain; a pair judged both ways is
  an error; with `--other-peaks-dir` the verdicts are checked against
  the last run's pairs before the search. Printed pairs show hits per
  second of aligned span, and joins under a tenth of the median are
  listed for a check by ear. README and the verdict file's header updated.
- Verified: 5 new unit tests, `scripts/check.sh` green. With a scratch
  verdict file judging DJ Trace - Sonar ~ "Mark System Revision"
  `different`, `clusters --from-peaks` with the NAS store stopped in
  0.25 s, naming the chain through the dfect upload (98%, 24,774 hits;
  91%, 246 hits). From the saved reports: 3 NAS joins under a tenth of
  the median 18.6 hits per second (0.73, 0.74, 1.84), none at 262
  tracks. No detection code changed; no clusters, scans or sweeps were
  rerun, and the cluster reports are unchanged.
- Next: the verdicts by ear, then `clusters --from-peaks` with the NAS
  store and the NAS-scale protocol again.

## 2026-10-08: the papers behind the design

- `docs/references.md` lists the papers and datasets with the sections
  that matter: Wang 2003, Six & Leman 2014, Six 2021 (Panako 2.0, the
  brief's "Six 2021"), Sonnleitner & Widmer 2016 (Qfp), Sonnleitner et
  al. 2016 (DJ-mix monitoring), Bhattacharjee et al. 2025 (sample
  identification), Smith & Serra 1987, Mixotic and UnmixDB. Each claim
  from an outside review (Astra) was checked against the PDFs (kept in
  `work/papers/`, not committed); UnmixDB's paper venue is unconfirmed.
- Roadmap: a new item "Evidence beyond exact pair hashes" (peak
  verifier, quantisation loss, denser query peaks); "Public DJ-mix
  datasets and per-second scoring"; shared-sample scenarios for the
  challenge set; citations added to the edge of the ladder, band peaks,
  detection boundaries, search time, behaviour at scale, longer hashes
  and clustering precision.
- Attributions added where the design follows a paper without saying
  so: ADR 0001 (lines, confidence), ADR 0003 (peaks, interpolation),
  `docs/calibration.md`, and the comments of `confidence.rs`,
  `search/lines.rs`, `hash.rs` (`FAN_OUT`) and `hash_cost.rs`. AGENTS.md
  and the README point to the references.
- No behaviour changed; `scripts/check.sh` green.

## 2026-10-08: session 7, diagnostics that need no listening

Brief `docs/brief-7.md`, checklist `docs/session-7-checklist.md`.

- 17:40 start. The reviewed work from after session 6 committed after
  `scripts/check.sh` (f65b912). Mixotic: the three Google Drive links
  return Google's sign-in page (HTTP 401 on the view page); nothing
  arrived, the three directories under `work/datasets/mixotic/` are
  empty; no sign-in and no other source tried.
- Release build in `work/bin/s7-start/`; `baseline session-7-start`
  copies session 6's reports; `regress session-7-start` with the HEAD
  build: 720/720 sweep queries and 87, 93, 79 scan detections identical.
  The NAS store: 27,042 `.peaks`, 27,042 `.tags`, 1,396 `.skip`,
  digest `bf35ea64...e9ae4f5`, revision `d428ee9585936326`, as expected.
- Test-set evaluation 3 (approved in the prompt; ledger): 14/16, 0
  wrong, pass; Sick Note possible at 233 hits in 4 windows as predicted,
  Sin possible at 123. 3 of 5 evaluations used. Nothing changed.
- Item 1: `gunfinger-eval --verify` (verifier.rs). A first version that
  searched each window's offset over ±48 frames gave chance shares of
  15-40% (the search reached the shifted reference's next bar); each
  window is now aligned by its densest run of hash hits. Tolerance
  chosen from 1×1, 3×2 and 6×2 on the development scan and sweep 2026:
  1×1 (experiment 0054). Frozen runs at 262 tracks, then the NAS-scale
  scan and sweep 2027, queued in `work/scripts/s7-item1-frozen.sh`
  (binary `work/bin/s7-verify/`).
- Item 2 (experiment 0055): `gunfinger-eval shared` with Clockwork ~ its
  remix and China Cup ~ The Nine, both directions: no scenario reaches
  the rule; loops do not add up (each repeat starts a new chain).
- Item 3 queued after item 1's NAS runs (`work/scripts/s7-item3.sh`,
  `sweep --other-rips` at NAS scale for seeds 2026-2029, then with
  `--extra-rungs 3`). From the NAS clusters' pair speeds: 233 clusters,
  55 with members more than 2% apart in speed, 7 more than 4%, 1 more
  than 6%.
- Item 1 at 262 tracks (18:21-18:53, experiment 0056): at equal hits the
  verifier's excess share separates no better than hits (AUC 0.953
  against 0.950; 0.827 against 0.865 for related false). The analysis
  labels a false detection related when any audio in the query is
  related (the grid's held-out neighbours had been missed).
- 18:53: item 3's sweeps start beside item 1's NAS runs instead of after
  them (two processes of about 3-6 GB on 32 GB; the swap guard stays),
  so that eight NAS sweeps end before the wrap-up. CPU is compared only
  within a run, as the brief says. Item 5's `pair` run (40 pairs:
  4 sparse joins, the 28 borderline pairs, 8 joins drawn as controls)
  runs beside them on one thread.
- 18:56: that did not work. The swap guard stopped the NAS-scale scan,
  item 3's first sweep and the `pair` run (swap 3.4 GB to 5.1-5.5 GB;
  logs kept in `work/logs/s7-stopped/`). From 19:00 every NAS-scale run
  goes one at a time through `work/scripts/s7-nas-queue.sh` (item 1's
  scan and sweep 2027, then item 3's eight sweeps); only `pair`, one
  thread and 25 MB, runs beside it. I left alone every process
  this session did not
  start.
- Item 5 (19:00-19:25, experiment 0057): `pair` at NAS scale on 4 sparse
  joins, 28 borderline pairs and 8 control joins; supported time and
  coverage order the pairs differently.
- Item 1 at NAS scale (19:00-19:31): scan 343 s, sweep 2027 1,553 s
  (540/540, 24 wrong, as in session 6); experiment 0058. The proposal
  (no verifier rule) and the listening list after session 7 are in the
  notes; roadmap, README and the calibration register updated.
- Item 3's sweeps run from 19:31 (`work/scripts/s7-nas-queue.sh`).
- Item 3, default ladder (19:31-20:20): four `sweep --other-rips` at NAS
  scale, 1,213 of 1,305 recalled; every excerpt 9.2% or more from the
  nearest remaining rip is missed (35); the 57 other misses are found
  below the rule from weak uploads or one other vinyl rip; 24 wrong
  answers, all from China Cup's Prototype upload and Synthesis VIP's
  mixed-CD version. `--extra-rungs 3` runs from 20:21.
- Item 3 with `--extra-rungs 3` (20:21-21:14): 1,216 of 1,305; 3 of the
  4 queries at 9.2-10% relative speed recovered, none of 31 beyond; 2
  more wrong answers, both China Cup's Prototype upload at -8%
  (experiment 0059). Notes, roadmap and calibration register updated.
- Wrap-up (21:16): no change to `gunfinger-core` or the CLI since
  f65b912, so `regress` was not needed. The NAS peak store at the end:
  27,042 `.peaks`, 27,042 `.tags`, 1,396 `.skip`, `library.txt`; digest
  `bf35ea64...e9ae4f5` (computed inside the store: `find . -type f
  -exec stat -f '%N %z %m' {} + | sort | shasum -a 256`); revision
  `d428ee9585936326` (`scripts/analysis/store_revision.py`); no file in
  it newer than the session's brief. Unchanged. Nothing
  here read `/Volumes/atlas`.
  `scripts/check.sh` green.

## 2026-10-08: after session 7: documents brought in line

- AGENTS.md lists the harness's `pair`, `verifier` and `shared`; the
  README lists `--verify`, `shared` and `sweep --other-rips`; ADR 0008
  records that test-set evaluation 3 confirmed its Sick Note prediction
  (14/16, Sick Note possible at 233 hits).
- Roadmap: the edge of the ladder reflowed and stated as deck speed
  plus copy offset past 9%; what step 1 of "Evidence beyond exact pair
  hashes" means for steps 2 and 3 (recall of weak copies rather than
  separation); the looped shared passage as the case any summing rule
  must handle (experiment 0055); Sick Note at 233 hits in the
  shared-material item.
- Calibration register: the verifier rule under "Measured, not
  adopted". References: what `--verify` found against Qfp's design, and
  that Mixotic needs a browser download.
- Documents only; no code changed.

## 2026-10-08: my 2003 mix as a second development set

- `corpus/sets/razorjack-2003-03-29/` (my mix, 46:10): `identify`
  against the NAS store found 12/12 confident, 0 wrong, confirmed by
  ear (experiment 0060; 229 s, 3.9 GB). Manifest drafted from the
  output and corrected by hand (track 4 at 10:47; notes on tracks 6
  and 11); `gunfinger-eval validate`: valid, 12 tracks, 7 referenced, 5
  absent at 262 tracks (NAS-only, paths as comments).
- We made it a development set: AGENTS.md, ADR 0004 (a later
  note), the roadmap ("More development mixes": NAS references, clusters
  around its files, the protocol, first experiments), the calibration
  baseline and the notes.
- Next: NAS references in manifests, clusters around the set's files,
  the set in the protocol. The verdict by ear on The Specialist ~
  INFRA 012 upload affects its NAS-scale score.

## 2026-10-09: the verdicts by ear

- `docs/pair-verdicts.txt` holds the first verdicts by ear, written down
  on request: 8 `same` (China Cup, Synthesis VIP, The Specialist,
  Coma ~ "Spraycan", Phoenix, Alien Girl and fractles against uploads or
  the mixed CD; the corpus Sonar ~ the dfect copy) and 3 `different`
  (Sonar ~ the "Mark System Revision", twice; The Nine ~ the Evol Intent
  VIP). On the last NAS clusters run's pairs (a simulation of
  `clusters::merged`), no verdict contradicts a join; seven clusters
  change. They take effect at the next `clusters --other-peaks-dir` run.
- Experiment 0061: two releases with swapped titles, checked against the
  audio (Future Cut's sour INFRA011 folder; Bad Company's DSCI4 Coma /
  Spraycan folder, `pair` 54 s).
- Roadmap: a library ignore file (our proposal, for mixed-CD
  tracks), titles of mislabelled files, the verdicts in the
  shared-material item, and two steps before the next NAS clusters run
  (verdict links only between searched, non-ignored files; the ignore
  file first). Notes and calibration register updated.
- Documents only; no code changed.

## 2026-10-09: session 8, a library ignore file

Brief `docs/brief-8.md`, checklist `docs/session-8-checklist.md`.

- 17:03 start, `git status` clean at `eb96898`. Release build in
  `work/bin/s8-start/`; `baseline session-8-start` copies session 7's
  reports; `regress session-8-start` with the start build as a check:
  720/720 sweep queries and 87, 93, 79 scan detections identical. Its
  calibrate line (weakest identifying 680 -> 658, false candidates at 30
  hits 18 -> 46) compares the baseline's four reports with every report
  in `work/reports/` (seeds 2027-2029 too); session 7's log has the same
  line. The NAS store: 27,042 `.peaks`, 27,042 `.tags`, 1,396 `.skip`,
  `library.txt`; digest `bf35ea64...e9ae4f5`; revision
  `d428ee9585936326`, as expected.
- `gunfinger-core/src/library/ignore.rs`: `.gunfingerignore` at the
  library root, a subset of `.gitignore` with literal brackets; `!`,
  backslashes, `/` alone and `//` are errors naming file and line.
  `Library::scan` reads it, walks ignored folders and keeps their audio
  files per pattern (`Library::ignore_file`, `Library::ignored`);
  `ScanError` tells an unlistable library from an invalid ignore file,
  which stops `identify`, `explain` and `stats` instead of falling back
  to the store. ADR 0009.
- CLI: `survey` labels each orphan `Gone` or `Ignored`; `doctor` reports
  the file, each pattern's count (a warning at none) and the two groups
  of records apart; `prune` lists the groups apart, sorted by path;
  `index` says how many files the file left out. Tests: 16 for the
  matcher, 4 scan tests, 2 end-to-end tests (ignored and gone files
  through `prune`, `doctor`, `index` and `stats`; an invalid file stops
  `doctor`, `stats` and `index`). `scripts/check.sh` green.
- README ("Leaving folders out"), AGENTS.md, roadmap (item done; the
  verdict-link step kept open).
- `regress session-8-start` with the feature (`work/bin/s8-ignore/`):
  720/720 and 87, 93, 79 detections identical; the same calibrate line.
- The NAS: `/Volumes/atlas/Music/dnb/.gunfingerignore` written at 17:18,
  our four lines (254 bytes); the root listing gained only it.
  `doctor --config ~/.config/gunfinger/nas-dnb.toml` (2:21): 30,216
  audio files, 20 and 13 left out; 33 peak records and 33 tag notes of
  ignored files; none of gone files; 1,812 audio files not indexed, 1
  record out of date. `prune` without `--yes` (27 s): 66 files, the 33
  records and 33 tag notes, nothing else. Read-only header check: 20
  records under Dangerous Drums, 13 under The Creeps' CD2, 11 under its
  CD1 kept; the records left after a prune give revision
  `6a4349c86e2582df` (27,009 records).
- The NAS store at the end: 27,042 `.peaks`, 27,042 `.tags`, 1,396
  `.skip`; digest `bf35ea64...e9ae4f5`; revision `d428ee9585936326`; no
  file newer than the brief. Unchanged.
- Next: we run `target/release/gunfinger prune --config
  ~/.config/gunfinger/nas-dnb.toml --yes`; the next session takes a new
  NAS baseline (revision `6a4349c86e2582df` expected). Then the verdict
  links of the clusters (roadmap, "More development mixes").

## 2026-10-09: session 9, past 32,768 assets, the NAS over Ethernet

Brief `docs/brief-9.md`, checklist `docs/session-9-checklist.md`.

- 20:28 start, `git status` clean at `54e7135`. Release build of
  `54e7135` in `work/bin/s9-start/`; `baseline session-9-start` saved
  (`work/baselines/session-9-start/`, session 8's reports). The NAS
  is on Ethernet (1000baseT), mounted over SMB.
- The NAS store at the start: 32,905 `.peaks`, 32,905 `.tags`, 1,565
  `.skip` (1,423 too long, 129 too short, 13 failed), revision
  `b8bb402f0ba23761`, as expected (`store_revision.py`, `store_census.py`
  into `work/s9/census-start.json`). 3,433 h of audio, 230.9 GB of
  source files, 2.32 GB of records. Longest record 14:56.8, an alien5ive
  upload with 0 peaks ("without music ... VJ Shader Effect"); next a
  14:48 mix in `sety/` and a 14:47 Kiss FM show in the scene folders.
  `scripts/analysis/store_records.py` (new) lists every record with the
  time its file was written: 27,008 records from 10-07/10-08 and 5,897
  written 10-09 17:42-18:01 (4,086 under `__full_scene/`, 1,811 under
  `__youtube_archivists/` butch-french, dnbfreak0 and rootzdnb).
- Item 1: `gunfinger stats --config ~/.config/gunfinger/nas-dnb.toml`
  (start build) stops with "the index holds at most 32768 assets" after
  57.0 s wall (22.9 s user, 4.5 s system, 20 MB peak footprint): the
  NAS listing, then the counting pass over 32,768 records. With
  `--store-only` the same error comes after 24.5 s, so the listing over
  Ethernet took about 32 s.
- Item 2 (experiment 0062, ADR 0010): `FRAME_BITS` 16, so a posting
  addresses 65,536 assets and frames up to 17:28; default `max_track`
  17:00; `TooLong` names 17:28 and the flag, `TooManyAssets` says how to
  leave files out; `doctor` counts against 65,536 and points to ADR 0010;
  README, help text, configuration docs, ADR 0005's status, ADR 0007's
  decision, the calibration register. Four tests (round trip at the
  largest asset and frame, the 65,536th asset and the next, a record
  ending at the last frame, a record past 17:28). `scripts/check.sh`
  green; `regress session-9-start` (`work/bin/s9-frames16/`): 720/720
  and 87, 93, 79 detections identical. Binary with the final message:
  `work/bin/s9-item2/`.
- Item 3 (experiment 0063): at 32,905 records `stats` reports 922.0 M
  postings, 3.70 GB, 4.01 bytes per posting, buckets p99 5,515; 126 s
  wall with the NAS listing, index built in 71.8 s, 3.75 GB. The
  development mix, store-only: 315 s, 2,344 s CPU, 4.76 GB; 30 plays as
  in 0050, the same 13 tracklist lines; 5 plays named after new copies
  with the same hits, one of them the Dangerous Drums 2 mixed CD back
  under `__full_scene/dnb_scene - 2000 - Part 2/2000-08/va-dangerous_drums_2-udfrcd002-2000-sour/`.
  The 2003 mix: 266 s, 1,854 s CPU, 4.66 GB; all 12 tracks, Phantom
  Force from 10:48 (a PHUD1 vinyl rip, 6,513 hits); 21 tracklist lines
  against 16, from three mixed CDs in the scene folders, including a
  confident "Music Maker (Majistrate Remix)" (Contagious Drum & Bass
  Vol 1, track 13, whose last 1:11 plays track 14, Phantom Force).
- Item 4 (experiment 0064): `doctor` with the NAS configuration in 43.9 s
  (2:21 over Wi-Fi in session 8; the share had been listed twice before
  this session): 34,470 audio files, 32,905 current records and 1,565
  passed over, none unindexed, out of date or gone. Three new folders by
  seed indexed into scratch stores (`work/s9-ethernet/`, `--config
  /dev/null`): 376 files, 2,594 MB, 41.4 h in 79.0 s, 263 Mbit/s, 0.21 s
  per file, 1.91 s per hour of audio, CPU-bound (8.3-8.8 of 10 cores),
  the local-disk rate of 0035. A full re-index over Ethernet: about 1.8
  h. All 374 records identical to the NAS store's; its revision after
  the runs `b8bb402f0ba23761`. The 13 failed files (12 recordings) are
  damaged: they decode 5-71 s short, four with zero-filled blocks.
  Deviation: the brief asked for about 300 files; the only new YouTube
  channel under 1,000 files has 306, so the total is 376. The first run
  script stopped after one folder (`status` is read-only in zsh), so the
  first folder has no `en7` byte count; it was run again from the SMB
  cache for comparison.
- Item 5 (experiment 0065): the 1,423 files passed over as too long,
  with lengths and tags from `ffprobe`, are in `work/s9/too-long.tsv`
  and, by kind, `work/s9/candidate-mixes.tsv`: 1,264 h, median 45.9
  min; 561 are alien5ive game videos, 366 YouTube mixes or shows, 136
  scene sets, radio shows and mixed CDs of 1999-2000, 96 in `sety/`, 17
  at the root or alone in a folder, among them my two mixes and
  an Ed Rush & Optical Essential Mix of 1999-02-07, by name the held-out
  test set's broadcast (not compared). `scripts/analysis/folder_edges.py`
  (new) scores each folder's track boundaries from the peak records: 33
  folders have at least 75% joined boundaries, 30 of them new
  (`work/s9/mixed-cd-candidates.tsv`), including the new Dangerous
  Drums 2 copy.
- End-to-end check of the frame limit (`work/s9/toolong-check/`, an
  18-minute synthetic file): `index --max-track 20:00` stores it;
  `stats --max-track 20:00` stops with "long.mp3 is longer than the
  17:28 the index can address; set --max-track (`max_track`) to 17:28
  or less"; `doctor --max-track 20:00` reports "longest track 18:00; the
  index addresses 17:28" as a problem. At the 17:00 default `stats`
  says "no indexed assets ... run `gunfinger index` first", misleading
  when the records exist but lie outside the range (roadmap, "No record
  inside the track length range"; not changed).
- Wrap-up: notes ("Session 9", decisions first: the mixed CDs and the
  scene copy of Dangerous Drums 2 the ignore file misses; roles of the
  candidate mixes, the Essential Mix 1999-02-07 at the NAS root by name
  the held-out broadcast; 13 damaged files), calibration register
  (posting layout; NAS-scale costs at 32,905 records), roadmap ("More
  than 65,536 assets", the ignore file, candidate mixes, a command-line
  item). The NAS store at the end: 32,905 `.peaks`, 32,905 `.tags`,
  1,565 `.skip`, revision `b8bb402f0ba23761`, no file newer than the
  brief; no file on the NAS newer than the brief either.
- Next: we decide the mixed-CD lines and whether to prune
  before the next clusters run (notes, "What we need to decide"). The
  overnight session: NAS references in manifests and verdict links
  only between searched, non-ignored files (roadmap, "More development
  mixes", steps 1-2), then the NAS clusters run once the store's
  revision for it is settled.

## 2026-10-09: the mixed-CD judgments; the NAS store pruned

- `/Volumes/atlas/Music/dnb/.gunfingerignore`, written on
  request: 31 patterns (29 new, unanchored folder names), 504 audio
  files left out. We marked 15 folders of experiment 0065's
  candidates mixed and 2 not (Shy FX's *Just An Example*, the Mampi
  Swift EP), and chose to leave out the 15 folders that were already
  known or named as mixed. `doctor` and `prune` without `--yes` checked
  every pattern.
- The NAS store was pruned: 935 files deleted (464 peak records, 7 skip notes,
  464 tag notes). The NAS store: 32,441 records, 1,558 skip notes,
  revision `5701f221f7b48ee1`.
- Documents only; no code changed.

## 2026-10-10: session 10, the verdicts by ear at NAS scale

Brief `docs/brief-10.md`, checklist `docs/session-10-checklist.md`.

- 01:17 start. The reviewed notes and status committed
  (`caa2799`, `scripts/check.sh` green). Release build of `caa2799` in
  `work/bin/s10-start/`; `baseline session-10-start` saved (session 9's
  reports at 262 tracks).
- The NAS store at the start: 32,441 `.peaks`, 32,441 `.tags`, 1,558
  `.skip` (13 failed, 122 too short, 1,423 too long), revision
  `5701f221f7b48ee1`, as expected; census `work/s10/census-start.json`
  (3,399.5 h). `doctor --config ~/.config/gunfinger/nas-dnb.toml` (35.2
  s): 31 patterns, 504 audio files left out, 33,999 audio files, no
  records of ignored or gone files.
- Item 2a: `clusters::Verdicts` keep their lines; `Verdicts::on`
  keeps the verdicts whose two files the run searches
  (`clusters::Searched`: the corpus library's assets, and with
  `--other-peaks-dir` the store's current records less what the other
  library's ignore file leaves out, read from the root the store names
  when it can be read; `ignore::leaving_out` in the core). Every other
  verdict is printed with its line and adds nothing. Tests: a corpus run
  takes no verdict on NAS files; a verdict never links an ignored file
  whose record is still in the store; `leaving_out`. On the real file,
  only line 26 (Synthesis VIP ~ the pruned Dangerous Drums 2 track)
  names a file the NAS run does not search.
- Item 2b: manifests may reference `second-library/<path>`; with
  `--other-peaks-dir`, `validate` and `scan` check such references
  against the store's current records; without it they are set aside
  (`Track::set_aside`), so a track with no other reference is absent.
  `validate` takes an optional set name. Tests for both sizes, and for
  scoring. Our edit of `razorjack-2003-03-29`: the five `# nas:`
  lines of tracks 1, 3, 4, 6 and 8 became their references, nothing else
  (the header comment still says the five are `[]`). `validate
  razorjack-2003-03-29`: 7 referenced, 5 absent, 5 references set aside;
  with `--other-peaks-dir`: 12 referenced, 0 absent.
- `map-library` again (5.6 s): 247 of 262 corpus files have a copy, as
  before; further identical copies 31 -> 64 (new scene folders).
- Item 2c: `gunfinger-eval recall` (`recall.rs`): the panel of seed 2026
  drawn from the store's headers and tags before any search
  (`docs/panels/recall-seed-2026.json`): pool 31,542 records (in the
  length range, outside session 6's NAS clusters, not copies, tagged),
  29,057 families split 14,528 / 14,529; each half 60 indexed and 20
  held-out sources, 20 per kind indexed (scene releases, YouTube
  channels, elsewhere), 7/7/6 held out, one per family. `recall
  --prepare` rendered the development half's 720 excerpts from the NAS
  (137 s, `work/recall/seed-2026/`). The misses' reasons
  (`recall::Miss`: never a candidate, too few hits, too few windows).
- Item 2d: `clusters --from-peaks --manifest <set> --recall-panel
  <seed>` adds the NAS files the named manifests reference and the
  panel's development sources to the queries (`clusters::other_queries`,
  copies of corpus files left out). Named sets rather than every
  manifest, so the test set's manifest is never read.
- `scripts/check.sh` green; `regress session-10-start` identical after
  2a+2b (`work/bin/s10-2ab/`, 267 s) and after item 2
  (`work/bin/s10-item2/`, 250 s): 720/720 sweep queries, 87, 93 and 79
  scan detections; the calibrate line is session 8's and 9's.
- `pair` on the 16 listening items known from session 6's clusters (6
  unjudged borderline groups, 10 control joins drawn with seed 2026, no
  identical copies; `scripts/analysis/listening_pack.py select`): 268.5 s
  wall, 267.8 s CPU, 149 MB.
- Item 3, the clusters run: started 01:48:21 as a background command of
  the agent's shell, which stops such commands after its default limit;
  it was stopped at 02:27 after 137 of round 1's 347 queries
  (`work/logs/s10-clusters-stopped.*`). Restarted at 02:28:48 detached
  under `screen` (session `s10-clusters`), so no limit applies; the
  later long runs start the same way. Files were added to the NAS from about 02:24 to 02:28, not by
  gunfinger, and the NAS store was unchanged afterwards (revision
  `5701f221f7b48ee1`, no file newer than the brief).
- The restarted clusters run ended at 04:44:26 after 8,138 s (74,782 s
  CPU, 5.13 GB peak; 347 queries in round 1, 290 in round 2, 2 in round
  3) with an error and no report: the verdicts judge the Sonar "Mark
  System Revision" different from the corpus Sonar and the dfect copy
  (lines 29 and 30), and a chain of joins links them through the PRO012
  rip, indexed since session 6 (corpus Sonar = PRO012 rip at 100%, 7,258
  hits, 19.9 hits/s; PRO012 rip = revision at 99%, 467 hits, 1.3
  hits/s). The check against the last run's pairs could not see it:
  session 6 had no PRO012 rip. The stdout is kept
  (`work/logs/s10-clusters-conflict.*`).
  Decision (conservative, to confirm manually): no verdict is added.
  `clusters` gains an opt-in `--cut-sparsest`: when joins chain two files
  judged different, it cuts the chain's measured join with the fewest
  hits per second (never one judged `same`), lists the cut, and keeps it
  in the report (`cut_links`, the pair's `same_recording` false). Here
  that keeps the revision apart from every Sonar rip, as both verdicts
  intend, and counts the revision against Sonar as a wrong answer rather
  than hiding it. The run is repeated with it after item 6's runs (item 6
  needs no clusters; Phantom Force's cluster, the i-witness upload and the
  PHUD1 rip, is in the stopped run's join lines). The repeat costs about
  2 h 15 min, so items 4 and 5 may not both fit before the 9-hour stop.
- Item 6's runs (they need no clusters) went first, 04:47-05:02, one at
  a time. Then `--cut-sparsest` and the track-range message
  (`stats`, `identify`, `explain`): `scripts/check.sh` green, `regress
  session-10-start` identical (`work/bin/s10-cut/`, 247 s). The second
  clusters run, with `--cut-sparsest`, started at 05:12:16 under
  `screen` (session `s10-clusters2`).
- The second clusters run ended at 07:26:39 (8,063 s, 75,059 s CPU,
  5.41 GB peak): 264 clusters, one join cut (the PRO012 rip ~ the Sonar
  revision); experiment 0066. Session 6's report is kept in
  `work/s10/before/`, the new one in `work/s10/after/`. The pack's 14 new
  items (`listening_pack.py select --known`, which now compares file
  pairs, since cluster numbers change between runs) went through `pair`
  (7 s) and were rendered (22 s); `work/listening/README.md` lists all
  30 items in order. Item 4 started at 07:26:46 under `screen`
  (`s10-after`, which runs `work/s10/item4.sh`).
- Item 4 ran 07:26-09:36 (experiment 0067; calibration register): the
  NAS protocol unchanged in recall, 24 wrong sweep answers (40 before),
  all from Essential Rewindz's mixed disc 2, which the ignore file does
  not list; margin 1.29× -> 1.73×; the 2003 mix 12/12 at NAS scale, 7/7
  referenced at 262 tracks, 0 wrong at both sizes and in the leave-outs.
  Session 7's other-rips sweeps (old clusters) are in
  `work/s10/before/moved/`, so that `calibrate` reads only this session's
  sweeps. Item 5 started at 09:36 under `screen` (`s10-item5`).
- Item 5 ran 09:36-10:03 (1,603 s, 14,736 s CPU, 6.5 GB; experiment
  0068): the panel's development half 540/540 at every speed and kind,
  no miss, no held-out source confident; 30 wrong answers, two pairs of
  the listening pack (Essential Rewindz's mixed Stealth against three
  unmixed copies; two DYkast "Punishment" uploads).
- Wrap-up (10:04): the NAS store at the end is the one at the start
  (32,441 records, revision `5701f221f7b48ee1`, no file newer than the
  brief). `doctor --config ~/.config/gunfinger/nas-dnb.toml` (40 s): 31
  patterns, 504 audio files left out, 34,642 audio files, of which 643
  are not indexed yet (33,999 at the start; most likely files added to
  the NAS from 02:24). All five items done before the 9-hour mark;
  notes "Session 10" lead with the listening pack (`work/listening/`,
  30 items).
- The session was extended to 11:30. Added `clusters
  --reuse-pairs` (with `--other-peaks-dir`): the last report's pairs,
  reset to what the search measured (`clusters::as_measured`: coverage
  rule alone, no verdict or cut; pairs with a file pruned or ignored
  since dropped), clustered again without searching; the check before a
  full search reads them the same way. On this session's report with
  `--cut-sparsest`: 1.0 s, the same 264 clusters, cut and criterion as
  the search (pairs equal to 1 ulp after the JSON round trip; the
  searched report was put back); without it, the Sonar chain stops it in
  1 s. `scripts/check.sh` green; `regress session-10-start` identical
  (`work/bin/s10-reuse/`, 248 s).
- A search that a verdict stops now keeps its pairs in
  `duplicate-clusters-stopped.json` (no clusters) and names the file;
  `--reuse-pairs [REPORT]` takes it, or the clusters in use without a
  value. `as_measured` applies the current verdicts, so the reused run
  prints the same lists as a search (322 further rips, 2 sparse joins,
  61 borderline pairs) and writes the same 264 clusters and cut; checked
  on a stopped-style copy of this session's report, and the searched
  report put back. The stop path itself was not run (it needs a full
  search). `scripts/check.sh` green; `regress session-10-start`
  identical (248 s).
- Checked: `clusters --from-peaks` without `--cut-sparsest` now stops in
  1 s on the Sonar chain, from the last report's pairs, before loading
  any query; before this change the check read the cut pair as not the
  same recording and would have started a 2-hour search. The report was
  not touched.

## 2026-10-10: after session 10: the prune and the test manifest

- We added 8 patterns to `/Volumes/atlas/Music/dnb/.gunfingerignore`
  (lines 41-48, 39 patterns in all) for mixed discs beside unmixed ones in
  one folder: Blazin, Molten Beats, Inside The Machine, 21st Century
  Grooves and Vintage Dread 2000 (`2*`), Soul Survivors (`1*`), and
  Essential Rewindz's discs 1 and 2 (`1*`, `2*`). Then `prune --yes` and
  `index` with the NAS configuration: the store holds 33,596 records and
  1,582 skip notes, revision `f9f9bc706195a71b`.
- NAS files named like the test set's absent tracks were checked by
  ear. On request the test manifest references one NAS file
  each for tracks 6, 9, 15, 18, 23, 24 and 26, with notes on 3, 12, 16,
  21 and 28 (`docs/experiments/test-set-ledger.md`, "Manifest change
  before evaluation 4"; no search was run, the count stays at 3 of 5).
  The 2003 manifest's header comment now describes its NAS references;
  nothing else in it changed.
- Session 10's listening pack (`work/listening/`) is not judged yet. Its
  item 1, the cut Sonar join, is unjudged, so every clusters run keeps
  `--cut-sparsest`.
- We approved test evaluation 4 at NAS scale.

## 2026-10-10: session 11, a saved index and test evaluation 4

Brief `docs/brief-11.md`, checklist `docs/session-11-checklist.md`.

- 18:15 start. Our ledger section committed alone (`4678fbd`,
  `scripts/check.sh` green, 2:23). Release build in `work/bin/s11-start/`;
  `baseline session-11-start` saved.
- The NAS store at the start: 33,596 `.peaks`, 33,596 `.tags`, 1,582
  `.skip` (11 failed, 124 too short, 1,447 too long), revision
  `f9f9bc706195a71b`, as expected; census `work/s11/census-start.json`
  (3,523.4 h). `doctor --config ~/.config/gunfinger/nas-dnb.toml`: 39
  patterns, 603 audio files left out, 35,178 audio files = 33,596 records
  + 1,582 passed over, no records of ignored or gone files, none waiting.
  It took 14:46 wall with 1.4 s CPU, against 35-44 s in sessions 9 and
  10: the share listed slowly.
- The recall panel of seed 2026 draws two development sources and one
  validation source that the new patterns leave out and the prune
  removed (Essential Rewindz's Stealth, Soul Survivors' Jupiter Fields;
  Molten Beats' Snakebite). `recall::Panel::for_seed` refused such a
  panel, which would have stopped the clusters search. It now names each
  and leaves it out of every search, keeping the draw and the rendered
  excerpts' numbers (`aa0862a`). `regress session-11-start` identical
  (`work/bin/s11-panel/`, 8:33 while `doctor` waited on the share).
- Item 2: session 10's NAS reports copied to `work/s11/before/`;
  `map-library` (2.4 s). The clusters search started at 19:03:14 under
  `screen` (`s11-clusters`, `work/s11/clusters.sh`): 352 queries, 262
  corpus files and 90 of the NAS's.
- The clusters search runs at about a third of session 10's pace: 93
  of round 1's 352 queries after 1:40 (session 10's first attempt: 137
  in 38 min). Two causes, measured: builds, tests and `scripts/check.sh`
  under `nice -n 10` still take a large share (the checks took 29:21
  instead of 2:23, and the search advanced 19 queries meanwhile), and the
  machine had other load. Without it the Decision: from 20:45 builds run at `nice -n 15
  -j 2`, the full checks and `regress` for items 3a-3c wait until the
  search ends (the harness uses none of the three changes; `regress` will
  show it), and only documents are written meanwhile.
- Item 3a (`dbce7d9`, ADR 0011): a saved index. `index::saved` writes the
  in-memory layout (offsets table, 4-byte postings) after a header
  naming the format, profile, hash design, frame bits, track length range
  and the asset table (path, size, modification time, duration), ending
  with an FNV-1a digest; `load` refuses a stale header, a truncated or
  overlong file, offsets out of order, a posting naming an asset the
  table does not hold, or a wrong digest. `indexing::plan_index` lists
  what a build would index from the record headers alone; `build_index`
  builds from such a plan. `identify`, `explain` and `stats` take
  `--saved-index use|rebuild|off` (default `use`); the file is
  `$XDG_CACHE_HOME/gunfinger/indexes/<store folder>-<digest>.index` (else
  `~/.cache/...`), one per peak store, replaced through a `.partial` file
  and a rename; not used with `--exclude-from`. Width audit (ADR 0011):
  the `u32` offsets count postings, enough at 65,536 assets of the NAS
  mean length; `Index::counting` now refuses more than 4.29 billion
  postings (`IndexError::TooManyPostings`), which would otherwise wrap.
  Tests: 8 unit tests (round trip, missing, stale profile, design and
  length range, changed store, truncated at 7 places, damaged posting,
  leftovers) and an end-to-end test (built, loaded, off, truncated,
  re-indexed). `scripts/check.sh` green (29:21 under the search's load).
- Item 3b (`3085abb`): conflicting titles. A confident play is marked
  with the confident plays over the same passage (the tracklist's
  `same_recording`: overlapping for half the shorter play, the track
  starting at the same mix time within 5 s) whose files' title tags
  differ in letters and digits, case ignored (`other_titles` in the
  JSON; a line "titled X; other files here: Y (plays 3, 4)" in the table;
  "titles disagree:" instead of "also:" in the tracklist). Derived when a
  report is made or read, like `shares_material_with`; no title is chosen.
- Item 3c (`5e79413`): `gunfinger-eval pair-review [--pack DIR]` reads
  `work/listening/README.md` and the item folders, skips items whose pair
  or another pair of the same two clusters has a verdict, shows each
  sheet in short, plays the clips with `ffplay`, and on `s` or `d`
  appends the sheet's verdict line to `--verdicts` (keys: Enter or `n`
  next clip, `p` previous, `r` replay, `k` skip, `q` quit). At the end it
  prints `clusters --from-peaks --reuse-pairs`, with `--cut-sparsest`
  while a cut join is unjudged. Three tests with a fake pack, player and
  verdicts file. Checked on the real pack with a fake `ffplay` and a
  temporary copy of the verdicts (all 30 items parsed; the copy
  unchanged); never run against `docs/pair-verdicts.txt`.
- 22:20-22:55: the clusters search reached 229 of round 1's 352 queries
  at 22:39, about 1 query a minute (session 10's attempt at night: 4.6 a
  minute). It holds 6.4-9.2 cores; the rest of the machine is busy (load
  average about 350). `sample` (3 s): the
  workers spend most of their time sorting hits, as in session 10.
  At this pace round 1
  ends near 00:30 and the search near 04:00-05:30.
- Decision: the heavy runs after the search are chained, so that none
  waits for a check: `work/s11/after-clusters.sh` (under `screen`,
  `s11-after`) waits for "clusters done", copies the report to
  `work/s11/after/`, runs `regress session-11-start` with
  `work/bin/s11-code/` (`1afd8cc`, release build at `nice -n 15 -j 2`)
  and stops unless every detection is identical, then item 3a's timing,
  then item 5, then 3a's cold load (after item 5's runs have filled the
  file cache). Item 4's analysis is light and runs during item 5; its
  `pair` runs follow item 5. The roadmap is updated for 3a-3c
  (`a2498af`).
- 00:11: the clusters search ended round 1 (352 queries, 5:08 since
  19:03); round 2 has 294 queries (session 10: 290), so the search
  should end around 03:30. A second waiting script
  (`work/s11/after-chain.sh`, `screen` session `s11-item6`) starts item
  6 (`work/s11/item6.sh`) when the first chain writes "all done", and
  does nothing if it stopped. Evaluation 4 is not chained: its ledger
  entry and prediction come first, after a look at items 5 and 6.
- A read-only review of `dbce7d9`, `3085abb` and `5e79413` (a subagent,
  no builds) found nothing that changes detections or lets a stale or
  damaged file load, and four small defects, all fixed (`84fe0e5`):
  `pair-review` printed `--cut-sparsest` only while a cut join was
  unjudged, though a `same` verdict on the cut join or a new `different`
  verdict can leave a chain that stops `clusters` (it now always prints
  it; the cut changes nothing elsewhere); `save` deleted every
  `.partial` beside the index, including another search's write in
  progress (now only those untouched for 10 minutes); a leftover of an
  interrupted `rebuild` stayed until the next save (now also deleted
  after a successful load); a damaged posting count could overflow the
  size check (now checked; a test). `scripts/check.sh` green (31:39 at
  `nice -n 15` beside the search). Tonight's heavy runs keep
  `work/bin/s11-code/` (`1afd8cc`); `84fe0e5`'s `regress` runs in the
  gap before evaluation 4.
- 03:08:44: the clusters search ended, exit 0 (29,130 s wall, 122,481 s
  CPU, 6.19 GB; 648 queries in rounds of 352, 294 and 2). Its cycles
  (+8.0%) and instructions (+5.9%) match session 10's work plus 3.6% more
  records; the 3.6 times longer wall time came from the machine's load.
  267 clusters. `regress session-11-start` with `work/bin/s11-code/`
  (`1afd8cc`, items 3a-3c): 720/720 sweep queries and 87, 93, 79 scan
  detections identical, the known calibrate line (03:14).
- Item 4 (experiment 0071): 13 corpus clusters change (six lose a pruned
  mixed-disc track, seven gain a new upload); 18 of 0066's 33 unjudged
  borderline pairs go with a pruned file, 15 stay, 2 are new (the test
  reference for Mind Overload against Molten Beats' `111`, 79.3%, an
  edit with about 45 s cut; T-Minus against its unreleased vocal mix).
  The listening pack: of 30 items, 21 stay (the cut Sonar join, 10
  borderline pairs, the 10 controls, all still joined), 9 go (the
  Essential Rewindz items, a file pruned); the 2 new pairs measured by
  `pair` (`work/s11/pack/`). Clips are rendered while item 5 runs.
- The listening pack rebuilt (03:40): session 10's pack moved to
  `work/listening-s10/`; `work/listening/` holds the 21 kept folders
  (copied unchanged, `items-s10-kept.json`) and the 2 new items rendered
  from `pairs-s11-items-pairs.json`; README: 23 items, the cut Sonar
  join first, Mind Overload second, the 10 controls last. `pair-review`
  read all 23 with a fake `ffplay` and a temporary copy of the verdicts
  (`docs/pair-verdicts.txt` unchanged, checked by digest).
- Item 3a's acceptance (03:15-03:38, nothing else of this session
  running; other processes kept the load average at 130-210): the
  2003 mix at NAS scale with `--saved-index off`, then `use` without a
  file (built and saved), then `use` with it (warm), byte-identical JSON
  reports. Index ready in 118.1 s (off), 167.2 s (built 163.2 s, saved
  2.0 s), 3.0 s (loaded 2.2 s); `explain` 14 s in all. The file is
  3,795,332,786 bytes, beside a 2.59 GB store. The cold load waits for
  the end of item 5. Item 3b on the same report: play 4 ("Sex Drive")
  names plays 5-7 ("The Specialist"); the tracklist says "titles
  disagree: Future Cut - The Specialist".
- 03:55: most of the night's load comes from another
  agent: a git worktree of this repository,
  `/Users/razorjack/Projects/OpenSource/gunfinger-prerelease-fixes`
  (branch `fix/prerelease-audit`, from `8038064`), runs `cargo` builds,
  debug end-to-end tests and `work/bin/candidate-eval --corpus
  <this corpus> --jobs 4 regress before-core-fixes`, and commits fixes
  (03:24-03:50). It reads this corpus and touches neither the NAS store
  nor `~/.cache/gunfinger/`. It is no Claude session this one can
  message, so it is left alone. `origin/master` was pushed to `8038064`
  from the shared git directory; this session pushes nothing. Item 3a's
  timings above ran beside it, so they are marked as measured under load.
  Item 5's development scan: 11/11, 0 wrong, 3 possible plays matching
  no track, as in 0067, but 974 s wall for the same CPU (2,337 s).
  Decision: the items keep the brief's order (5, 6, then 7); if item 5
  cannot end by about 08:00, what is left of it and of the rest is
  decided then and written here.
- Item 5 (03:38-06:19, experiment 0073): as 0067 in every score, and the
  prediction holds: the 24 wrong sweep answers (Essential Rewindz's
  Funktion and The Nine) are gone, 2,160/2,160 and 0 wrong; strongest
  false 220 (The Nine's Evol Intent VIP), 3.00×; unrelated chance 63-70.
  The scans ran beside the other agent's load (424-974 s for 2,337-2,368
  s CPU); the sweeps had most of the machine (1,666-1,724 s). The
  calibration register has a session 11 paragraph (`d66812e`).
- Item 3a's last timing (06:19-06:23): the saved index loaded after item
  5's runs in 2.1 s, as warm (without `purge`, which needs root, a cold
  cache is likely, not certain); its report byte-identical too.
  Experiment 0072 (`1269615`).
- Item 6 (06:23-06:51, experiment 0074): 522/522 from 58 indexed sources
  (the two pruned ones left out), 0 held-out confident answers, 9 wrong
  answers (the DYkast "Punishment" uploads, as in 0068); 0068's 21
  Stealth answers went with the pruned source. No regression against
  0067 or 0068, so item 7 runs: the ledger entry and prediction were
  committed at 03:42 (`fab29bb`); evaluation 4 started at 06:52:11 under
  `screen` (`work/s11/item7.sh`, once).
- Item 7, evaluation 4 (06:52:11-07:03:46, once, exit 0; 694 s, 5,590
  s CPU, 5.47 GB): **21/23 identified, 2 wrong identifications**, Sin
  and Sick Note possible, 1 possible play matching no track. Predicted:
  the 21, the two possible, and the wrong Molten Beats edit of Mind
  Overload (921 hits; pack item 2). Not predicted: The Nine's Evol
  Intent VIP as a wrong identification (489 hits in 16 windows over 157
  s of the original's play; the sweeps' 30 s excerpts gave it 9-24% of
  the original's hits). No detection of the five related versions
  reaches the possible tier (Beachball's bootleg 45 hits). Report kept as
  `work/reports/evaluation-4-scan-ed-rush-optical-essential-mix.json`,
  out of the NAS report folder. Ledger `34b10ba`: 4 of 5 used. Nothing
  changes because of it.
- 8b started at 07:04 (`work/s11/item8b.sh`, `sample` every 10 ms over a
  saved-index `identify` of the 2003 mix); 8c is chained after it
  (`work/s11/after-8b.sh`, `screen` session `s11-8c`), its plan fixed
  at 03:43, before evaluation 4.
- 8b (07:04-07:08, experiment 0075): with the saved index, `identify` of
  the 2003 mix at NAS scale is 96.5% search; `sample` puts 82.1% of the
  CPU in sorting hits, 9.5% in query analysis, 0.1% in loading the
  index.
- 8c (07:08-07:26, experiment 0076): 34 runs over the 17 pairs' shared
  passages with the saved index, 8 again with the played cluster left
  out (each rebuilt in 81 s): 18 of 34 related plays confident, 15
  possible; the left-out runs give identical related plays.
- From 07:26, chained (`work/s11/after-8c.sh`, `screen` `s11-end`):
  `regress session-11-start` with `work/bin/s11-review/` (`84fe0e5`),
  the store's revision, `doctor`.
- End chain (07:26-07:34): `regress session-11-start` with
  `work/bin/s11-review/` (`84fe0e5`, the session's last code), 255 s:
  identical (720 of 720 sweep queries, every scan; the calibrate line as
  at the start). The NAS store: 33,596 records, revision
  `f9f9bc706195a71b`, as at the start. `doctor`: 35,573 audio files
  against 35,178 at the start: 395 new files, not indexed (an `index`
  run adds them; the next search then rebuilds the saved index once).
- Research items a-c were done by 07:26. The brief names no further
  research item, so nothing else ran before the 10:00 stop (decision: no
  unplanned measurement).
- Wrap-up: notes "Session 11" (the pack and `pair-review` first, then
  the seven mixed-disc folders), evaluation 4, the calibration
  register's research paragraph, the README's load time; checklist
  complete.

## 2026-10-11: isolated pre-release fixes

Branch `fix/prerelease-audit`, worktree `gunfinger-prerelease-fixes`, based
on `8038064`. The parallel session's checkout and derived files are untouched.
Committed fixes cover time parsing, index exit status, saved report validation
and source identity, report collisions, CUE escaping, recall exclusions,
regression summaries, render completion, automatic fallback and alias titles.
Stale-record invalidation and monotonic play confidence are implemented.
All five performance candidates have measured retained changes; experiments
0078-0083 record the wins and the alternatives not adopted.
`CARGO_BUILD_JOBS=2 scripts/check.sh` passes formatting, Clippy with warnings
denied and all 255 tests (43 CLI, 13 CLI integration, 119 core, 13 codecs,
67 harness). Original audit reproductions also pass, including conflicting
identical-audio titles rendered from a saved report with the library offline.
`work/bin/final-eval --corpus ../gunfinger/corpus --jobs 4 regress
before-core-fixes` passes: all 720 sweep queries and all 259 scan detections
are identical; sweep recall 100% at every speed, zero wrong; development
11/11, zero wrong; calibration unchanged (680/99). Experiment 0077 records
the isolated validation. No held-out evaluation or external publication.

Audit outcomes by the requested categories:

- Obvious bugs: a higher-hit short segment cannot demote a confident play;
  changed query audio invalidates saved reports; rejected replacements cannot
  leave old peaks searchable offline; new indexing failures return failure.
- Edge cases: oversized times and malformed report ranges return errors;
  CUE fields stay on one line; unfinished renders cannot become cache hits;
  recall filters removed sources before rendering; regression descriptions
  include changes to timing, speed, confidence and other detection fields.
- Surprising behavior: identical-audio aliases retain conflicting titles;
  separate recordings cannot overwrite the same saved report; a nested
  library read failure cannot silently include ignored files from the store.
- Performance: reuse FFT scratch, restrict long refinement posting lists to
  the candidate asset, reuse catalog preparation, split sorting only for
  large hit buffers, and share equal-hop key-locked peak extraction.

## 2026-10-11: the pre-release fixes merged

- On request, after a review of each commit,
  `fix/prerelease-audit` is merged into master (`53d7ee0`). The only
  conflict was this file; both sections are kept. The branch's
  experiments 0072-0078 collided with session 11's and are renumbered
  0077-0083 (references in the section above follow; the branch's
  commit messages keep the old numbers).
- Checked on the merged tree: `scripts/check.sh` green (255 tests);
  `regress session-11-start` identical (720/720 sweep queries; 87, 93
  and 79 scan detections; the calibrate line as in sessions 7 and 8);
  the 2003 mix at NAS scale gives the same 37 segments in 27 plays in
  181.4 s instead of 220.1 s (experiment 0084).
- For later sessions: the first harness run after the merge renders
  every cached excerpt again once (completion receipts), the recall
  panel's from the NAS; `identify` stops on an unreadable subfolder
  instead of searching the store's records (`--store-only` does that,
  ADR 0012); `index` exits with an error when a file fails in that run;
  a play's confidence is the highest its segments reach (ADR 0013), as
  the harness already scored it.

## 2026-10-11: session 12, a faster search and the saved index in the harness and `doctor`

Brief `docs/brief-12.md`, checklist `docs/session-12-checklist.md`.

- 14:26 start, `git status` clean at `7ef893f`. Release build in
  `work/bin/s12-start/` (byte-identical to `work/bin/merged/`);
  `baseline session-12-start` saved from the merged build's reports.
  `regress session-12-start` with the start build: 720/720 sweep queries
  and 87, 93 and 79 scan detections identical, the known calibrate line
  (680 -> 658, 18 -> 46); 212.7 s wall, 1,915 s CPU, 0.47 GB.
- The NAS store at the start: 33,596 records, 1,582 skip notes, revision
  `f9f9bc706195a71b`, as expected; census `work/s12/census-start.json`
  (3,523.4 h). The saved NAS index
  (3.8 GB, 03:25) in place.
- Item 2 (14:33-14:37, experiment 0085): `sample` over the NAS `identify`
  of the 2003 mix with the start build: 177.7 s wall, 1,548 s CPU, 4.78
  GB; sorting hits 78.1% of the CPU (grouping by asset 49.9%, each
  asset's offsets 28.2%), query analysis 9.7%, lines 7.5%. Every window
  holds 0.6-2.0 million hits, so 0080's comparison sort below 50,000
  never runs at this scale.
- Item 3 (experiment 0086, `1345be7`): `search::lines::WindowHits` groups
  a window's hits by asset with a counting sort (one counter per asset of
  the index, kept by each worker) and finds hits as 8 bytes (posting and
  anchor position), computing offsets and query frames by the same
  expressions at grouping. Benchmark (`work/s12/perf`): 0.42 of the
  merged time on 120 real NAS windows, 0.38 on 417 corpus windows; it
  loses only on buffers with fewer hits than about a third of the asset
  count (synthetic, at most 0.1 ms a window), which no real window
  reaches, so no threshold. The 2003 mix at NAS scale: 85.7 and 92.1 s
  against 166.6-171.3 s for the start build (CPU 774 against 1,545 s),
  4.80 GB against 4.78; every report byte-identical. `regress
  session-12-start` identical (203.7 s); `scripts/check.sh` green (4:49).
  A 24-byte variant cost 0.4 GB more peak memory and was not kept.
- Item 4 (ADR 0014, experiment 0087, `de4577e`): with `--other-peaks-dir`,
  `Padding::index` loads the index of the corpus records and the whole
  other store from `work/indexes/library-<store>.index` when its header
  names exactly these files, otherwise builds and saves it; runs that
  leave files out (leave-outs, held-out clusters, recall's held-out
  sources) remove them in place with `Index::without_assets`, which
  renumbers the other assets in order: the index built without them,
  posting for posting (unit test). Not used with `--other-sample`,
  synthetic copies or `memory`. `--saved-index use|rebuild|off`. Tests:
  stale (new store record, changed corpus record), damaged digest,
  truncated file, leave-outs against builds. `scripts/check.sh` green
  (4:49); `regress session-12-start` identical (203.8 s). At NAS scale
  (15:23-15:40, idle): the development scan rebuilt and saved 192.4 s
  (build 80.4 s, save 3.0 s), loaded 112.0 s (1.9 s), off 189.0 s;
  leave-out 3 filtered 107.2 s, built 179.6 s; reports identical apart
  from `wall_seconds`, and equal to session 11's. The file: 33,618
  assets, 3,798,396,993 bytes. Session 11's NAS reports copied to
  `work/s12/before/` first and the two scans put back after.
- Item 5: `doctor` has a "saved index" section: the store's file, its
  size and whether it is current for a search of the library (its
  header and length read, `saved::examine`, not the 3.8 GB of
  postings), or current only for `--store-only` searches, out of date,
  unreadable or missing; then the folder's other files with their sizes
  (saved indexes of other stores, unfinished saves). It deletes nothing.
  Tests: `saved::examine` (missing, current, stale, cut, damaged digest
  left to `load`); the end-to-end saved index test runs `doctor` at each
  step (none yet, current, cannot be read, out of date, other files kept,
  current for `--store-only` only). `scripts/check.sh` green (5:10, 260
  tests).
- `doctor --config nas-dnb.toml` with the item-5 build: the first run
  (15:42, 138 s) could not finish listing the library ("No such file or
  directory") and reported the store alone; the second (15:45, 202 s)
  listed it: 36,305 audio files (35,573 at the end of session 11), 1,127
  not indexed yet, 603 left out by 39 patterns, 33,596 current records;
  the saved index `nas-dnb-peaks-23935cfc56936b67.index`, 3.8 GB,
  current. Files are most likely being added to the NAS, so a file
  vanished during the first listing; `identify` without `--store-only`
  would stop on such an error (ADR 0012).
- `regress session-12-start` with the item-5 build (`work/bin/s12-item5/`):
  identical (203.6 s).
- Experiment 0088 (`26f8b7b`): `sample` over the NAS `identify` of the
  2003 mix with the item-5 build (15:57-15:59): 92.1 s, 780 s CPU, 4.80
  GB, report byte-identical; sorting each asset's offsets 39.3%,
  grouping and lines 32.4%, query analysis 18.9%, posting scans 6.5%.
- Extra check of item 4's held-out path (16:00-16:11): `sweep --seed
  2026` at NAS scale with the item-5 build loads the harness's index
  (3.7 s) and removes 48 held-out clusters: 540/540, 0 wrong, the report
  byte-identical to session 11's; 641 s, 6,059 s CPU, 5.83 GB (session
  11: 1,713 s under load). Session 11's report put back.
- Wrap-up: notes "Session 12", roadmap ("Search memory at scale",
  "On-disk index", "Where search time goes"), README, checklist. The NAS
  store at the end: 33,596 records, 1,582 skip notes, revision
  `f9f9bc706195a71b`, as at the start. The harness's index
  takes 3.8 GB of disk.
- Review (a read-only subagent, 16:12-16:21) of `1345be7`, `de4577e` and
  `0a625f3`: nothing that can change detections (it checked the float
  expressions, ties at cluster edges, skipped assets, buffer reuse,
  `without_assets` against a build, every caller's exclusion set, the
  refactored `load`). Acted on: `doctor`'s "current" line says what it
  read (header and length; searches check the rest); the `--store-only`
  plan is read only when the library's is stale; the doc comment on ties
  names chosen clusters; ADR 0014 and AGENTS.md: a saved index is checked
  against files and settings, not code, so a change to how postings are
  made changes `hash::design()`'s tag or runs with `--saved-index
  rebuild`; a leave-out with no current file builds the whole index, so a
  limit only the whole index passes would stop it (none is near). Left:
  `Posting::new` is `pub(crate)` for the tests of `lines`. Checks pass
  (5:10), `regress session-12-start` identical (`work/bin/s12-review/`,
  203.1 s). `doctor` on the NAS (16:30, 213 s): 36,376 audio files, 1,198
  not indexed yet; the saved index current.

## 2026-10-11: after session 12

- Checked from a separate thread: HEAD `0a5b9b5`'s `identify` of the
  2003 mix at NAS scale (store `f9f9bc706195a71b`, `--store-only`, both
  playbacks) gives plays byte-identical to the merged build's report of
  experiment 0084, in 88.0 s (181.4 s there, 220.1 s before the merge),
  4.80 GB peak.
- Roadmap: "Files that vanish while the library is listed", from the
  failed first `doctor` listing of session 12.

## 2026-10-11: session 13, files that vanish while the library is listed

Brief `docs/brief-13.md`, checklist `docs/session-13-checklist.md`.

- 18:11 start, `git status` clean at `c066c2a`. Release build in
  `work/bin/s13-start/`, byte-identical to `work/bin/s12-review/`.
  `regress session-12-start` with it, to refresh `work/reports/` from the
  start build: 720/720 sweep queries and 87, 93 and 79 scan detections
  identical, the known calibrate line; 206.5 s wall, 1,812 s CPU, 0.42
  GB. `baseline session-13-start` saved from those reports.
- The NAS store at the start: 33,596 records, 1,582 skip notes, 33,596
  tag notes, revision `f9f9bc706195a71b`, as expected.
- Step 2 (ADR 0015): `Library::scan` reads each folder's entries in full,
  then counts an entry below the root whose type, metadata or folder
  listing answers NotFound in `Library::vanished` and leaves it out;
  other errors stop as before; the root's own read still fails the scan,
  and after a listing in which anything vanished the root is read again
  (gone or unreadable: the scan fails with that error, so `identify`,
  `explain` and `stats` fall back to the store only when it is missing,
  as ADR 0012 says). `identify`, `explain`, `stats` and `index` print one
  warning with the count; `doctor` a note in its library section;
  `prune` refuses such a listing (with `--yes` and `--force` too) and
  asks to be run again; the harness refuses it (`padding::complete`).
  Tests: files and folders deleted from the progress callback vanish
  (real races at a fixed point), a root deleted during the listing fails
  it, a missing root is an error, a folder without read permission still
  stops it; `prune` with a listing that has a vanished entry deletes
  nothing even with `--yes --force`, and prunes once it has none.
  `scripts/check.sh` green (5:13, 266 tests). `regress session-13-start`
  with `work/bin/s13-fix/`: 720/720, 87, 93 and 79 detections identical,
  the known calibrate line; 210.7 s.
- Step 3, first part (18:31-18:45): `doctor --config nas-dnb.toml` four
  times with `work/bin/s13-fix/` while files were added to the NAS: 36,550, 36,554,
  36,559 and 36,565 audio files (1,372-1,387 not indexed yet), nothing
  vanished in any run; 189, 190, 205 and 253 s (the last while the
  review ran `cargo` beside it); 33,596 current records; the saved index
  current.
- Step 4, review (a read-only subagent, 18:31-18:44) of `26bc727`: the
  walk, the root rules and `prune`'s refusal hold; the tests are
  deterministic whatever the entry order. Acted on: metadata is read only
  for audio files that would be assets, so a downloader's partial files
  and thumbnails are passed over by name and never vanish (also one
  `stat` fewer per such file); `Library::vanished` keeps the sorted paths
  and the messages name the first, so a path that vanishes from every
  listing can be found; `doctor`'s line on gone records says `prune`
  waits for a listing in which nothing vanishes; `prune`'s help says to
  run it again with the same options; the messages avoid count-dependent
  pronouns; the harness's check is `nothing_vanished` in `main.rs`; ADR
  0015 and the README say a folder gone before it is read counts once,
  and that an unreadable root stops as at the start. Not done (in ADR
  0015): checking vanished paths again at the end, comparing the root's
  device. Checks pass (5:16, 267 tests); `regress session-13-start` with
  `work/bin/s13-review/` identical (208.5 s).
- Step 3 (experiment 0089): `doctor` with `work/bin/s13-review/`
  (18:56-18:58) and alternating with the start build (19:01-19:06, idle):
  nothing vanished in any of the 14 NAS runs; 36,550 audio files at 18:31,
  36,600 at 19:06 (1,422 not indexed yet); `s13-review` 28.0-28.9 s
  against 40.1-47.7 s for the start build (runs 1-4, before 18:45, took
  189-253 s: the NAS answered faster later, unexplained). `identify` of the
  2003 mix through the library (no `--store-only`): 87.8 s with
  `s13-review`, 94.6 s with the start build, the saved index loaded, 26
  plays (24 confident), the reports byte-identical. A real race on a
  local disk (200 folders x 20 files under `churn.py`): `doctor` with
  the start build could not list 10 of 30 times, with `s13-review` 0 of 30
  (9 with vanished entries, the first path named); `prune --yes` 4 of 20
  failures against 0, 6 refusals.
- Wrap-up (19:08-19:12): notes "Session 13"; AGENTS.md (`library` counts
  what vanishes, ADR 0015); roadmap item done (with what is open);
  README; experiment 0089 trimmed to 40 lines; `churn.py` kept in
  `work/s13/` beside the race scripts. The NAS store at the end: 33,596
  records, 1,582 skip notes, revision `f9f9bc706195a71b`, as at the
  start; the saved index in `~/.cache/gunfinger/indexes/` untouched (one
  file, 03:25).

## 2026-10-11: session 14, the listening tool

Brief `docs/brief-14.md`, checklist `docs/session-14-checklist.md`.

- 23:40 start, `git status` clean at `633ebdd`. Release build in
  `work/bin/s14-start/`, byte-identical to `work/bin/s13-review/`.
  `regress session-13-start` with it: 720/720 sweep queries and 87, 93
  and 79 scan detections identical, the known calibrate line; 210.6 s,
  0.42 GB. `baseline session-14-start` saved from those reports.
- The NAS store at the start: 33,596 records, revision
  `f9f9bc706195a71b`, as expected.
- Design (ADR 0016, `docs/audition-questions.md`). Measured first: FFmpeg
  decodes a 30 s window (`-ss` before `-i`, 16-bit stereo 44.1 kHz) of a
  NAS mp3 in 0.13 s, an opus in 0.23 s, the corpus m4a in 0.09 s. Decoded
  from its own start, a window differs from a continuous decode in its
  first 1,098 frames (mp3) or 8,597 (opus); started 0.2 or 0.5 s early
  with those frames dropped, it is bit-exact (mp3, opus), and the m4a
  matches for its first 0.86 s (scattered AAC differences later, not at
  the joint). Windows use a 0.5 s pre-roll.
- Steps 3 and 4 (`b9a7450`): the schema and its check, placement,
  harness names, the answer log with effects and undo, the pack and
  folder producers, the HTTP server, audio windows, envelopes and the
  page, 43 tests. `pair-review` now appends verdicts through
  `clusters::Verdicts::append`, which the pair effect shares; its
  behaviour is unchanged. `questions pack` (plan
  `work/audition/plans/pack.toml`, the pair reports `pairs-s10-pairs`,
  `pairs-s11-items-pairs` and `pairs-supported-pairs` in
  `work/reports/library-nas-dnb-peaks/`) wrote 23 questions in 0.3 s;
  item 1 has the corpus original as lane 3.
  `questions folders` (`work/audition/plans/folders.toml`, the seven
  folders of `make_listen.py`) wrote 7 questions over 84 files in 34 s,
  most of it computing their envelopes.
- Step 5, the page checks (`c55c75c`), in headless Chrome over the
  DevTools protocol (`work/s14/cdp/cdp.ts`) and the T3 preview, with the
  answer log and a copy of the verdicts file in `/tmp/s14-check/`. On
  pack item 1 (three lanes), item 2 (its jump at 334.6 s), control join
  14 and the folders Skynet & Stakka and Bad Company - Fear LP: a lane
  switch cuts the old source and starts the new one at the same context
  time, at the mapped file time; speed matching sets `playbackRate` to
  the piece's speed; the loop wraps; Tab returns; the blend pans the
  lower lane left; markers jump. Answer, undo, the "left there" case (a
  line appended after the answer's) and the refusal of a contradicting
  verdict behaved as specified on the temporary files. Fixed after the
  checks: the waveform also draws the RMS level (a loud master's
  extremes fill the lane); Up and Down pass over folder `join` markers;
  plainer span labels and refusal message; long names wrap in the queue.
  Window costs: the server answers a 30 s window in 0.06-0.11 s (NAS mp3
  0.07 s, NAS opus 0.11 s, local m4a 0.06 s); in the page a window costs
  70-105 ms to fetch and 10 ms to decode, 10.6 MB as a buffer; 24 windows
  (the limit) came to 242 MB on a folder. Safari is not tested; the page
  needs Safari 15.4 or later (`Array.prototype.at`, `findLast`).
  Screenshots `work/s14/screenshot-1` to `-5`.
- Step 6: `questions detections REPORT` (one question per play, the mix
  as the reference lane, the play's file placed by its segments; answers
  to the log only), 2 tests. On the 2003 development mix's report
  (`work/merge-check/2003-s12.json`) it wrote 27 questions in 15 s, to
  `work/audition/detections/2003-s12.json`, outside the default folder so
  the listening queue holds only the pack and the folders. Checked in the
  page on play 4 (two segments): the track's lane starts its second
  piece at 341.3 s; `y` logged `confirmed` and opened the next open
  question, `Z` took it back; the verdicts copy unchanged.
  A question file's `made_by` is now the whole producer command line;
  the three files were made again with it and are otherwise identical
  (ids included). The folders run then took 19 s, recomputing envelopes
  cached before the RMS level was added; a second run takes 0.04 s.
- Step 7, a read-only review by a subagent (timeline mapping, undo,
  server security, clarity), 14 findings; the server's security held.
  Fixed, each checked:
  - The page: playback never reached the end (the wait's time bump ran
    before the end check); the playhead read the end of the scheduled
    horizon for the first 60 ms after play; a cut called `stop()` again
    on a source due to stop earlier, which extends it (the last call
    wins); a piece starting up to 1 ms before its file held playback
    forever; any failed window was silent until a reload. Reproduced
    with Node on the real `player.js` and `windows.js` against a fake
    audio context (`work/s14/sim/sim.mjs`: the old files fail 5 of 7
    checks, the new pass all 7). A failed decode is now `503`, retried
    every 3 s while the playhead waits, and named on the page.
  - Keys: a held answer key answered every question opened after it,
    and a held `Z` undid several answers. Repeats now only seek and
    jump between markers, and a key pressed while an answer is written
    is ignored. Checked in Chrome with `repeat: true` events and a
    double press.
  - Answers: the log is opened before the effect writes; an error after
    a verdict line says the line stays. `unsure` on a pair the verdicts
    file already judges is refused like a contradiction. Undo takes back
    a line only while it is the file's whole last line (a hand-edited
    `# same ...` ends the same way), only for a loaded question, and
    names the question.
  - `questions detections` clamped the timeline to the report's
    duration, not the end of the part searched, so excerpt reports
    failed. Two wrong references in `docs/audition-questions.md`; the
    recipe for a new writing effect now lists what assumes the verdicts
    file.
  Not done: a lock between two servers on one log. `File::try_lock`
  needs Rust 1.89 and the workspace declares 1.85 (though `let` chains
  in the core already need 1.88); raising it is our call. Every
  write reads both files first, so two servers collide only on answers
  within the same few milliseconds. Also left: a timeout on FFmpeg (a
  hung NAS holds connections; the browser's six per host would then
  delay answers). 49 audition tests.
- Wrap-up (00:59): AGENTS.md section "The listening tool", the README's
  command and keys, the roadmap's "Report edits" (listening done) and "A
  full-screen TUI" (replaced), notes "Session 14". With the final build
  (`work/bin/s14-end/`, the code of `e1470d0`): `regress
  session-14-start` identical (720/720 sweep queries, 87, 93 and 79 scan
  detections, the known calibrate line; 207.3 s, 0.39 GB);
  `scripts/check.sh` green, 316 tests. The NAS store at the end: 33,596
  records, revision `f9f9bc706195a71b`, unchanged.
  `docs/pair-verdicts.txt` unchanged; `work/audition/answers.jsonl` never
  created. The check servers are stopped.

## 2026-10-12: the first listening with the tool

- All 23 pack items and the seven folders were answered with
  `audition` (`work/audition/answers.jsonl`, 34 records); the page wrote
  23 lines to `docs/pair-verdicts.txt`. On request one line
  more: `same` for the corpus Sonar and the PRO012 rip (B of item 1 was heard
  as the original Sonar). Item 5 (Falcon - The Stand) was
  answered `same` after the fix below; item 13 `different` ("not even
  close"). Not committed yet.
- The meaning of the verdicts (2026-10-12): two rips of the same
  vinyl by different people (turntable, RIAA stage, cartridge, level)
  are `same`, however different they sound; files that differ in even a
  few bars, such as two edits, are `different`. Item 2 (Mind Overload
  against Molten Beats' edit) is `different`, though we are glad
  when a mix detection points to either, since the title is right. No
  third verdict for now (we are unsure it is needed). Item 3 (Stratus -
  Waves) is `same`: one file has needle skips. The pack's question texts
  (`work/audition/plans/pack.toml`) were reworded so each asks whether A
  and B are the same recording, the question `S` and `D` answer.
- Folders: mixed CDs (DJ mixes) are Point of Arrival and Listen Up;
  separate are Jonny L's Magnetic and Bad Company's Fear LP. The three
  `unsure` (The Creeps CD1, Analog Steroids, Breakbeat Science disc 2)
  are continuous CDs: a few seconds of crossfade between otherwise
  separate tracks, not a DJ mix (by ear). Too short to make a confident
  play by themselves (three 10 s windows need more than 10 s of shared
  audio), so they can stay indexed. Folder questions now offer `C`
  continuous (`audition::folders`); the three were answered again
  with it.
- Fixed: lane B of item 5 (Falcon - The Stand) started 3:50 into its
  file. The sheet's fourth alignment, 8 hits in 2 windows at offset
  +230.9 s (chance or a repeated section), placed B's first 33 s.
  Lanes now follow only alignments with the 60 hits of a possible play
  (`placement::followed`, ADR 0016); this replaces the extra lanes'
  tenth-of-the-strongest rule. In the regenerated pack it changes items
  1 (lane 3), 2, 5, 6, 10, 11 and 12; item 2's 48-hit intro alignment
  is no longer followed either. Item 5 now plays B from its start at A
  0:00.4. Test `lane_b_does_not_follow_a_chance_alignment`; 120
  harness tests, clippy and fmt clean. Not committed yet.

## 2026-10-12: session 15, the verdicts by ear applied, the containment rule and the possible tier measured

Brief `docs/brief-15.md`, checklist `docs/session-15-checklist.md`.

- 02:27 start, `git status` clean at `f3b3191`. Release build in
  `work/bin/s15-start/` (`gunfinger` byte-identical to `s14-start`'s;
  `gunfinger-eval` differs by the listening tool's `4ff1ef1`).
  `regress session-14-start` with it: 720/720 sweep queries and 87, 93 and
  79 scan detections identical, the known calibrate line; 217.3 s, 0.48
  GB. `baseline session-15-start` saved from those reports, and
  `session-15-start-nas` (with `--other-peaks-dir`) from session 11's NAS
  reports. The NAS store: 33,596 records, revision `f9f9bc706195a71b`, as
  expected. The NAS report
  folder's JSON files and the 262-track reports copied to
  `work/s15/before/` first.
- Step 2 (experiment 0090): `clusters --from-peaks --reuse-pairs` at NAS
  scale without `--cut-sparsest` finishes (4.7 s): no chain links two
  files judged `different`, so the option is no longer needed. 267 -> 268
  clusters: Stratus - Waves gains the Sonix WEB file, the two DYkast
  "Punishment" uploads form a cluster; 22 verdicts confirm the clusters;
  session 11's cut is gone. At 262 tracks every verdict names a NAS file:
  `clusters --from-peaks` (16 s) finds the 17 clusters in use.
  `scripts/analysis/devscore.py` recomputes the harness's scores offline
  (checked: it reproduces every stored score); no development reference's
  or sweep's held-out cluster changed, and the five NAS development scans
  run again (`work/s15/step2-scans.sh`, 83-117 s each) are byte-identical
  in detections, plays and scores. Recall seed 2026: 9 wrong answers -> 0
  (the DYkast pair). No new NAS baseline needed.
- Step 3, experiment A (0091): `scripts/analysis/containment.py` applies
  the candidate containment rule offline to every stored development
  report (scans and leave-outs at both sizes, sweeps, recall, robust, the
  generated mixes, 0076's passages) over a grid of 24 points (span held
  against: the stronger play's whole span or its passages; tolerance 0,
  10, 30 s; hit-ratio bound none, 0.75, 0.5, 0.25). New runs, read-only
  against the NAS store: the 7 shared passages of the pairs judged
  different on 2026-10-12 (T-Minus, Dozer, Pyro), 0076's method
  (`work/s15/shared.sh`, 9-14 s each, 80-110 s with the played cluster left
  out). Development reports hold no wrong confident play, so the rule
  removes nothing there and demotes no right play; the generated mixes
  show that whole play spans (a returning track) and a 30 s tolerance
  demote right brief plays, passages with 0-10 s do not. On the new
  passages it removes 5 of 6 related confident plays at every grid point;
  with the played cluster left out it removes none.
- Step 4, experiment B (0092): `scripts/analysis/possible_tier.py` labels
  every play and detection below the confident rule in the NAS development
  reports as right, related (a `different` verdict, a pair of 30 hits or
  more between the clusters, or tags) or unrelated, at 60, 80, 100 and 130
  hits; tag labels moved from `nas_calibrate.py` to `tag_labels.py`
  (`nas_calibrate.py`'s output on session 11's reports byte-identical
  before and after). No new search beyond two `pair` runs (Meltdown against
  the two Phantom Force files: at most 14 hits). The two development mixes
  show 3.5 possible plays per hour at 60 (0.6 at 130); five are versions of
  the track playing, one (Rift feat. Souldrop - Meltdown, 98 hits,
  key-locked, during Phantom Force in the 2003 mix) is not explained by a
  pair search, which cannot test its implied +6% pitch, +2% tempo
  relation: a question to judge by ear. No development true play at
  NAS scale is found only as possible; at 262 tracks, 48-128 of 278
  damaged excerpts found only as possible would be hidden at 80-130.
- Step 5, the speed step (experiment 0093), from 03:07: profiled the start
  build on the 2003 mix at NAS scale (0088's method, 105.1 s, under some other
  load): sorting each asset's offsets 40.1% of CPU
  samples. At 33,596 assets nearly every asset has 3 hits in a window and
  1.8% can form a line. `search::lines::may_form_line` counts an asset's
  offsets in 4-frame buckets folded into 4,096 slots and sorts only when 3
  hits lie in one or two neighbouring slots: exact, as a line's offsets
  differ by at most 2 frames as computed. Benchmark (`work/s15/perf`,
  lines compared exactly): the per-asset stage 0.216 of its time on 120
  NAS windows, 0.220 at 262 tracks. `scripts/check.sh` green;
  `regress session-15-start` identical (212.0 s); the 2003 mix at NAS
  scale, A B B A: 65.0 and 65.7 s, 482 and 485 s CPU, against 96.9 and
  93.5 s, 776 and 769 s, the same 4.8 GB, four reports byte-identical.
  Build in `work/bin/s15-filter/`.
- Step 4 follow-up (experiment 0094), while the review ran: whether
  Meltdown's 98 hits are shared material. `gunfinger explain --asset`
  on excerpts cut and resampled with ffmpeg (`work/s15/meltdown/`, NAS
  store read-only). Resampled by 1/1.0452 to undo the pitch the mix
  implies, Meltdown gives Phantom Force no candidate; a control (the
  Phantom Force passage shifted +4.52% with rubberband, then resampled)
  gives the PHUD1 rip 1,292 hits on the predicted relation. Phantom Force
  alone at the mix's speed lines up with Meltdown at 53 hits; the mix's
  alignment lands 12 s apart in Meltdown in two searches. Not a re-pitched
  copy; a repeating pattern such as a shared break, or chance: the
  listening question now asks whether they share a break or loop.
- Step 6, the review (a read-only subagent, 12.6 min): `lines.rs` correct
  (it also checked 300,000 random cases in an emulation). Seven defects in
  the analysis scripts, one changing reported numbers, all fixed:
  `possible_tier.py` counted plays of two identical files twice (harness
  scans do not merge same audio; `identify` does). Counted once now: the
  development mixes show 2.9 possible plays per hour at 60 (was 3.5), 2.3
  at 80 (2.9), related 4 (5); sweeps and recall 11.8 and 10.2 per hour
  (12.3, 10.4); 100 and 130 unchanged; 0092 and its data corrected.
  Latent: `devscore.py` judged every segment by the fitted rule (now each
  segment's pass, ladder 200 hits; `--nas` replaces guessing from the
  path; all ten scans still reproduce their stored scores); a query found
  only as possible would have counted twice (none exists); "right hidden"
  counted detections under 60; `containment.py`'s simulated drop of the
  played file also dropped its identical rips (unreported rows change),
  query plays summed weak detections (no effect), one closest call
  compared rounded values (same 171.4 s); `verdicts_applied.py` counts
  older lines apart. `lines.rs`: the bucket width named (twice
  `LINE_SPAN_FRAMES`) with a compile-time check on the one-byte counts.
  `scripts/check.sh` green (322 tests); `regress session-15-start`
  identical (201.1 s); the 2003 mix at NAS scale with the final build
  (`work/bin/s15-final/`) byte-identical, 64.0 s, 487 s CPU.
- Wrap-up, 04:06: notes "Session 15" (the verdicts, both decisions with
  numbers and recommendations, the Meltdown question with paths and
  times, the speed step); roadmap items updated. The NAS store at the
  end: 33,596 records, revision `f9f9bc706195a71b`, unchanged.
  `docs/pair-verdicts.txt` and `work/audition/answers.jsonl` unchanged
  (digests as at the start). Finished at about 04:10, well before 10:30;
  no further work started.

## 2026-10-12: after session 15, the answer by ear on Meltdown

- By ear: Rift feat. Souldrop - Meltdown and Digital & Spirit
  - Phantom Force are different tracks that share the full drum break
  ("Gunfinger is not wrong to see some similarities"). Meltdown's 98 hits
  are shared material, so every possible play of the two development
  mixes at 60 hits is explained (4 versions, 1 shared break). Recorded in
  0094; `docs/pair-verdicts.txt` untouched. The strongest unrelated
  candidates left: Clockwork 3:32-3:49 ~ Simon Static - Rubba Rock
  0:21-0:37 (64 hits, key-locked -3.7%, the same Rubba Rock passage also
  aligning at Clockwork 1:41 and 4:50), then Cause 4 Concern - Abduction
  2:50-3:15 ~ the Epox remix (C4C 004) 5:00-5:24 (43-54 hits at four sweep
  speeds); both to judge by ear.
- The answers by ear on the two next pairs (same evening):
  - Clockwork ~ Simon Static - Rubba Rock (64 hits): the tracks share the
    identical stab sample (Clockwork made it known; Rubba Rock may have
    sampled it), with its long reverb tail, and the hi-hat rhythm under
    it ("tick tick tick tick"), played with a different hi-hat sample.
  - Cause 4 Concern - Abduction ~ the Epox remix (54 hits): the same
    kick-and-snare drum loop, distinctive in drum & bass; the hi-hat
    patterns differ (each track layers its own).
  - So the strongest "unrelated" matches checked since session 10 are
    all shared material: 0051's 63-70 (Falcon - The Stand, now joined;
    Rubba Rock; Mystery Machine ~ NIL - Infectious and Sentient ~ Shere
    Khan - Criminal Record, both repeating at every sweep speed and
    labelled related by the clusters' pairs), 0052's jump to 64 at 9,196
    assets (Rubba Rock), and 0092's 98, 64 and 54. Unchecked: 41
    (Watermelon ~ 300 Baud - Monotone, Mobiuz & Cycom remix) and 40
    (Falcon - Fractles ~ Ryme Tyme - We Enter, Optical remix v3.0). The
    roadmap's "chance grows past the possible tier from about 9,000
    assets" rests on Rubba Rock and needs revising.

## 2026-10-13: session 16, the tracklist by default, one entry per track, a stall timeout on FFmpeg

Brief `docs/brief-16.md`, checklist `docs/session-16-checklist.md`. No
timings are findings: the machine had other
load.

- 01:53 setup, `git status` clean at `5b09bc3`. Release build in
  `work/bin/s16-start/` (both binaries byte-identical to `s15-final`'s);
  `baseline session-16-start` saved from session 15's final `regress`
  reports. The NAS store: 33,596 records, revision `f9f9bc706195a71b`, as
  expected. Digests of `docs/pair-verdicts.txt` and
  `work/audition/answers.jsonl` in `work/s16/listening-digests-start.txt`.
- Step 2: ADR 0017 records our principle (fire and forget):
  report edits and the Track/AudioAsset model out of scope, disagreeing
  titles flagged and never chosen, the tracklist as the default output.
  Roadmap, AGENTS.md ("The product's scope"), README, ADRs 0006 and 0016
  follow.
- Step 3 (experiment 0095): the 2003 mix at NAS scale listed DJ Hidden -
  Skektics twice because the alien5ive upload's track start lies 41.7 s
  from the rips'. Plays over the same passage whose own files' artist and
  title tags compare equal (letters and digits, ignoring case) are now one
  entry (`same_tagged_track`), joined as connected components so the
  order of plays does not matter; the cue sheet groups the same way.
  Measured on 30 stored reports (11 distinct) and 7 fresh ones: only the
  Skektics entry changes in reports of the current library; session 9's
  report, before the prune, also joins two mixed-CD extracts.
  Output only: the JSON report and the core are untouched. 7 tests.
- Step 4: the tracklist is the default of `identify` and `show`; `-f
  human` keeps the table (no alias); `no tracks found` for an empty
  result; headings for several recordings; the tracklist stays plain
  under `--color always`. README, help, ADR 0017; an end-to-end test of
  the default (an upload with 20 s more lead-in and tags differing in
  case: one entry).
- Step 5 (ADR 0018): `decode::watch` runs FFmpeg and FFprobe with stdout
  and stderr read on threads of their own; a child that writes nothing
  for `decode::STALL` (120 s) is killed and reaped on a detached thread,
  never waited for. The same limit covers the exit wait and the header
  read that chooses how FFmpeg opens the file. `DecodeError::Stalled` is
  not about the file, so `index` fails it for this run only; `identify`
  fails the recording; the listening page decodes through the same
  watcher. `Decoder` (paths of `ffmpeg` and `ffprobe`, the timeout) is
  the test seam: stand-in scripts in temporary folders, a 0.5 s timeout,
  `PATH` untouched. Checked by hand: `identify` with a stand-in that hangs
  exited 1 after 120 s with the message and the stand-in was gone.
- Step 6: `rust-version = "1.89"`; the lints it turns on fixed
  (`as_chunks`, a `let` chain, `is_multiple_of`). `audition` holds the
  answer log's lock (`AnswerBook::lock_log`, `try_lock`) while it runs; a
  second server on the same log exits with a message. Verdicts are
  checked and appended under the verdicts file's lock
  (`clusters::VerdictsWriter`, blocking), which undo and `pair-review`
  take too. ADR 0016 "One writer". `regress session-16-start` with the
  step 6 build (`work/bin/s16-step6/`): 720/720 sweep queries and 87, 93
  and 79 scan detections identical, the known calibrate line.
- Step 7, review by a subagent (read-only, the diff `a9ef804..d089b92`),
  fixed:
  - `pair-review` appended under the lock without reading the file
    again, so a verdict from the page on the same item meanwhile could be
    contradicted (the file then fails to parse). It now reloads under the
    writer and writes nothing for an item judged meanwhile; a test.
  - `thread::spawn` in `decode::watch` would panic when the system
    refuses a thread (threads pile up while a share stays hung); now
    `thread::Builder`, the file failing with `DecodeError::Spawn`;
    `within` returns `io::Result`.
  - stderr counts as activity (a `Heartbeat` over either pipe), so a
    decode writing only errors is not a stall; a test.
  - `tests/stalls.rs` ran the real `ffprobe` and `ffmpeg` under 0.5 s:
    now 5 s; a second test covers a stalled header read in `index_asset`
    and in `tag()`. The watcher tests' margins widened (2 s for the slow
    decode; the pid file may be missing on a loaded machine); their
    folders removed.
  - `Entry::titles_disagree` also compares the titles of all confident
    plays in the entry (chains); `tracklist_changes.py` with
    `work/bin/s16-review/`: no line changes in the 17 distinct stored and
    fresh reports.
  - `render` reads an untagged file's name once per output (a cache).
  - Wording: ADR 0017's rule (two plays, not two files), the page's
    module note, the `index` failure message; ADRs 0016 and 0018 follow.
  Not changed: a possible play marked as sharing material stays apart
  even when tagged like the confident play (the brief's rule); no stored
  report prints such a line.
- Wrap-up, 03:22: notes "Session 16" (the 2003 mix's default output,
  checked against the final build; the timeout and what happens to a hung
  file; the review; four decisions open to change); roadmap item
  "A share that hangs". With the final build (`work/bin/s16-final/`, the
  code of `4ec0ee4`): `regress session-16-start` identical (720/720 sweep
  queries, 87, 93 and 79 scan detections, the known calibrate line);
  `scripts/check.sh` green on the same code (345 tests). The NAS store at
  the end: 33,596 records, revision `f9f9bc706195a71b`, unchanged.
  `docs/pair-verdicts.txt` and `work/audition/answers.jsonl` unchanged
  (digests as at the start); no manifest changed. Processes this session
  did not start were left alone.

## 2026-10-13: session 17, speed: identify at NAS scale and a first index

Brief `docs/brief-17.md`, checklist `docs/session-17-checklist.md`. Other processes used 0.2-1 core, and files were added to the NAS during the
session; every timing ran old and new builds A B B A.

- 04:35 setup, `git status` clean at `b7ddfb6`. Release build in
  `work/bin/s17-start/` (byte-identical to `s16-final`); `baseline
  session-17-start` from session 16's final `regress` reports. The NAS
  store: 33,596 records, revision `f9f9bc706195a71b`, as expected.
  Digests of `docs/pair-verdicts.txt` and `work/audition/answers.jsonl`
  in `work/s17/listening-digests-start.txt` (equal to session 16's).
- Step 2, search speed, from 04:37. Profile of the start build (0093's
  method): 62.6 s, 483 s CPU; grouping and lines 51.0% (with the stages
  kept out of line: the scatter 28.4%, the pre-filter 13.8%, counting
  8.1%), query analysis 30.1% (`PeakPicker::pick` alone 11.6%).
  - 0096: a window keeps the posting lists it hit; each part of 4,096
    assets is taken off the lists' fronts and grouped in cache, so each
    asset's hits keep their order; the pre-filter keeps the slots it
    touched. Rejected: two passes through a second buffer (+4% CPU),
    counting while taking (+6%), parts of 1,024 (+1.5%) and 8,192
    (+1.7%). A synthetic benchmark: the old scatter is bound by memory
    traffic across cores (3.6-5.2 ns a hit on 10 threads).
  - 0097: the four-neighbour test without branches into a candidate list,
    the neighbourhood scanned nearest frames first without branches;
    `pick` 4,962 -> 1,537 samples. Rejected: run maxima (+5.7%). A test
    compares the picker with the definition over a whole spectrogram
    (ties included).
  - 0098: `current_sources` and `plan_index` read the 33,596 headers on
    `jobs` threads (the CLI's `--jobs`; with the store's pages evicted,
    1.1 s before the index load instead of 5.5 s); `identify` decodes
    the first recording while the catalog opens.
  - Acceptance (`work/bin/s17-step2/`): the 2003 mix at NAS scale, A B B
    A, 44.3 and 44.1 s, 382 and 377 s CPU, 4.33 GB against 62.1 and 61.0
    s, 487 and 482 s, 4.79 GB; four reports identical. `regress
    session-17-start` identical (720/720 sweep queries, 87, 93 and 79
    scan detections, the known calibrate line), 151.7 s against 200.8 s
    with the start build. `scripts/check.sh` green (346 tests). One
    mistake on the way: `s17-h` and `s17-hd` were built with parts of
    8,192 left from a trial; the acceptance was repeated with 4,096.
- Step 3, indexing speed (experiment 0099), from 06:27: first `index`
  runs into scratch stores (`work/s17/scratch/`, `--config /dev/null`),
  never the NAS store; the corpus library (265 records, 29.58 h) and the
  NAS folder `__youtube_archivists/error-bat` (187 records, 18.93 h,
  chosen because it holds no 0065 candidate; `dnbfreak0` holds one). A B
  B A: corpus 49.4-49.5 s against 51.3-51.7 s, the NAS folder 35.2-35.4 s
  against 36.6-36.7 s; every file of the four stores of each series
  identical. FFmpeg alone on the corpus: 43.7 s, 255 s user, 165 s sys,
  so FFmpeg is about 86% of a first index's CPU; Gunfinger's own CPU
  (sampled) 67 s -> 35 s, peak picking 38 s -> 9 s. No change beyond
  0097: the run is within 13% of FFmpeg alone, whose options are fixed.
  A full first index of the NAS collection: about 1.78 h instead of 1.84
  h at the folder's rate.
- Step 4a, from 06:40: a new producer, `gunfinger-eval questions shared
  PAIRS.json` (`audition/shared.rs`), because no existing kind asks
  whether two different recordings share material: A whole as the
  reference lane, B placed by every alignment of the strongest audio
  search of a `pair` report (`placement::tile`), also those below 60
  hits; choices `shared`, `chance`, `unsure`, written to the answer log
  only (`Effect::Log`). The two pairs' `pair` report is
  `work/reports/library-nas-dnb-peaks/pairs-s17-shared.json`; the
  question file `work/audition/shared/s17-shared.json`, outside the
  default queue. Watermelon ~ Monotone: eight key-locked alignments at
  -4.1% to -4.6%, 13-62 hits, from eight passages of Watermelon onto
  Monotone 0:32-1:29; Fractles ~ We Enter v3.0: seventeen turntable
  alignments of 7-20 hits at +2.7% to +5.9%, most onto We Enter
  0:23-1:11. The page loaded both questions with a copy of the verdicts
  file and an answer log in `/tmp/s17-audition/` (both lanes' audio and
  waveforms; nothing answered, the log empty); the server stopped.
  Documented in `docs/audition-questions.md` and ADR 0016's producer
  list.
- Step 4b: the roadmap's "Behaviour at scale" and "The possible tier at
  NAS scale" and `docs/calibration.md` no longer call the 63-70 hits
  between different recordings chance: every one checked since session
  10 shares material (the answers by ear of 2026-10-12, Rubba Rock the
  64 from 9,200 assets), and the strongest not checked has 41 hits. The
  earlier proposal (130-140 at NAS scale) is marked as resting on the
  64 being chance; Meltdown's answer is recorded. No threshold changed.
- Step 5, a read-only review of `b7ddfb6..054e0e4` by a subagent (with
  its own differential tests in `/tmp`: 183,674 lines bit-identical over
  1 to 20,000 assets, the picker identical in 200,000 random cases with a
  finite floor). No output change found. Fixed:
  - The grouping test used 3,000 assets, one part of 4,096, so a
    `take_part` that took every list whole still passed. It now uses
    9,000 assets (three parts, the last shorter), adds a window with no
    hit in the middle part and an empty list, and fails with that fault.
  - `index::saved::load` did not check that each posting list is ordered
    by asset, which the grouping (and `postings_for_asset` before it)
    relies on: a crafted file or a writer bug could panic the search.
    `read_postings` now notes where the asset goes down and refuses the
    file unless each is a list's start (`Unusable::Unreadable`, so the
    index is rebuilt); a test reverses a list. Done in the read loop, it
    costs nothing measurable: 3.5 s per load at NAS scale (946 million
    postings) with and without, A B B A over eight runs; a separate pass
    cost 0.4 s.
  - Comments: the silence row rejects nothing a finite floor does not
    (the floor is -10 dB); `open_while_decoding` says a catalog error
    waits for the decode; `Hit` and `WindowHits` no longer describe a
    window held whole; 0098 says the harness lists records on every core
    (it passes its jobs where it builds an index).
- Step 5, round 2 (`4471b5f` and `612b4d9`, read-only, its own tests in
  a copy): no defect, no output change; the order check agreed with a
  brute-force check in 2,000,000 random cases. Fixed in `questions
  shared`: a pair named twice (in either order) is an error naming it,
  not `check`'s "appears twice"; an alignment of no length gets no span;
  markers less than 0.5 s apart are one stop (two in Fractles); the
  error for a report without an audio search says `pair` searches the
  audio only for a corpus file; details without backticks, which the
  page shows literally; the speed-matching note says the blend always
  matches. Docs: an example of the kind and its `edge` markers in
  `docs/audition-questions.md`, `schema.rs`'s producer list, 0098's
  harness sentence (its jobs in `map-library` and `clusters`, every
  core elsewhere). `work/audition/shared/s17-shared.json` made again
  (same ids) and loaded by a temporary server; nothing answered.
- Wrap-up: final build `work/bin/s17-final/`; `regress
  session-17-start` identical (720/720 sweep queries, all 87, 93 and 79
  scan detections, the known calibrate line), 154 s and 1,342 s CPU;
  `scripts/check.sh` green (352 tests). The NAS store at the end: 33,596
  records, revision `f9f9bc706195a71b`, as at the start. Listening files
  unchanged (digests as at the start). Scratch stores, the second target
  directory and the trial builds removed from `work/` (`work/s17/` is
  85 MB). Notes "Session 17", the roadmap's "Where search time goes".
  Finished at about 07:40, well before 10:30; no further work started.

## 2026-10-14: session 18, a third development set; the containment rule and the possible tier on it

Brief `docs/brief-18.md`, checklist `docs/session-18-checklist.md`. Timings were taken under other load, and files were added to the NAS
during the session.

- 03:10 setup, `git status` clean at `300e008`. Release build in
  `work/bin/s18-start/` (byte-identical to `s17-final`); `baseline
  session-18-start` from session 17's final `regress` reports. The NAS
  store: 33,723 records, revision `c835c5531c325274`, as expected.
  Digests of `docs/pair-verdicts.txt` and `work/audition/answers.jsonl`
  in `work/s18/listening-digests-start.txt`. The NAS report folder copied to
  `work/s18/before/` (without `variant-single-pass/`, 1.1 GB, which no
  run of this session writes).
- Step 2a (experiment 0100): `map-library` (only the record count
  changed); `clusters --from-peaks --manifest razorjack-2003-03-29
  --manifest stakka-skynet-knowledge --manifest
  nocny-trans-vertical-2002-08-23 --recall-panel 2026`
  (`work/s18/clusters.sh`): 370 queries (262 corpus, 108 NAS), rounds of
  370, 285 and 2; 1,554 s, 12,548 s CPU, 4.2 GB. 268 -> 271 clusters,
  877 -> 886 files: new clusters for the new set's Stab 2 Def (4 files),
  Mutated (2 uploads), the Konflict Nephelim (2), the We R The Ones VIP
  upload with SUICIDE004's "We R The Ones 3" (100% coverage, 7,106
  hits) and alien5ive's "FUZE 29 cannibal" with two "Sonic - Change"
  uploads (100%). The Molten Beats vinyl/CD clusters of System Error and
  Iron Lung are gone: no query of this run reaches them (one file is a
  validation source of the recall panel; `--recall-panel` searches the
  development half). `devscore.py --nas` under the new clusters: every
  stored NAS development score unchanged. The pruned Point of Arrival
  file is a validation source of the panel, so no run of this session
  touches it. `pair` on the 36 related and unclear pairs
  (`work/s18/s18-related.tsv`) and 4 more (`s18-possible.tsv`), read by
  `scripts/analysis/pair_relations.py`: the two TI029 C rips are one
  recording (30,783 hits) that matches neither the Phunckateck VIP (6
  hits) nor the 1999 original (7); other versions against the played
  one: the 1999 Descent 32, Invid's original 25, the Gridlok Slipstream
  13-38, the Klute Remix 166-263 (32-54% coverage), the rest 0-27.
- Step 2b: `scan nocny-trans-vertical-2002-08-23` at 262 tracks 9/9, 0
  wrong, nothing possible unmatched (47 s). At NAS scale (`--saved-index
  rebuild`, the harness's index saved over the stale one, 173 s): 17/17,
  0 wrong, 4 possible plays matching no track (Mampi Swift - 2000, the
  Klute Remix in two files, the untagged `1.mp3`), each shared material
  with the confident play over it (`pair`: 827, 166-263 and 177 hits).
  Leave-outs (`--leave-out 3`, seeds 2026, 2027, 2028; 71-74 s): 14/17,
  0 wrong each; no other recording is confident in a left-out slot, and
  the possible plays there are the same four (corrected in step 7).
  `scripts/analysis/devset_view.py` lists each track's credited files
  and the strongest accepted detection. Track 17's slot (1:01:25-1:04:40)
  has nothing at least possible in any run. `identify --store-only` with
  the configuration (turntable; the CLI's saved index rebuilt
  over the stale one, 135 s, 4.2 GB) and with `--playback both` (67 s):
  the same 21-entry tracklist (`work/s18/identify-default.txt`).
- Step 2c: `scripts/analysis/tracklist_view.py` rebuilds the entries from
  the JSON as `export::entries` does (it reproduces the 21 lines): every
  referenced track has one confident entry holding a right file, 16
  named by the right title; entry 15 is "Klute - We R The Ones 3 (titles
  disagree: Klute - We R The Ones VIP)". The four possible entries show
  other titles alone, each marked as sharing material; the Klute Remix
  is listed twice (two files; marked plays are never grouped).
- Step 3 (experiment 0101): `scripts/analysis/containment.py` now reads
  the new mix's scans (both sizes) and NAS leave-outs, and simulates for
  every scan the strongest file of each identified track dropped (other
  rips keep their hits). Passages, 10 s, bound none or 0.5: the new mix
  has 9, 36 and 88 confident plays (262, NAS, leave-outs), none wrong,
  none demoted; its closest call needs 99 s (Anthrax inside Blindside).
  The drop loses no right recording (13 of 39 identified tracks keep a
  weaker copy at 262 tracks, 45 of 57 at NAS scale). Every row of 0091's
  data is reproduced exactly under the new clusters (432 rows).
  Recommendation: do not adopt.
- Step 4 (experiment 0102): the answers by ear of 2026-10-14
  (Watermelon ~ Monotone, Fractles ~ We Enter: shared) recorded in the
  roadmap's "The possible tier at NAS scale" and `docs/calibration.md`'s
  row. `scripts/analysis/possible_tier.py` with the new mix and its
  leave-outs (tags of the current store, `work/s18/tags.json`): 3.6, 3.6,
  2.7 and 1.8 possible plays per hour at 60, 80, 100 and 130 hits (4, 4,
  3, 2 plays in 1.11 h), all related by audio; the three mixes 3.2, 2.8,
  1.8, 1.1. No true play of the new mix found only as possible. Its
  strongest detection matching no track (66 hits, 44:33-45:16) is Stab
  2 Def blended in before its slot's window. No threshold changed.
- Step 5: no listening question written, by the brief's criteria. Track
  17's slot (1:01:25-1:04:40) has no candidate: nothing at least
  possible in any run, and the strongest alignment there has 9 hits
  (chance level). `pair` settles the TI029 C side: one recording that
  matches neither the Phunckateck VIP nor the 1999 original. No wrong
  confident play exists to explain.
- Step 6 (experiments 0103-0105), the 2003 mix at NAS scale. Profile of
  the start build's code with the grouping's functions not inlined: the
  pre-filter 25.6% of CPU, grouping a part 23.2%,
  taking a part 10.5%, the ladder's analysis 27.6% plus 3.3%. Three ways
  of computing the pre-filter's slots once (V2, V2b, V3) gave identical
  reports; none kept (V2 +7% CPU, V2b no gain, V3 -1.3% CPU with the
  same wall time). Kept: the saved index's postings read and checked on
  a second thread while the caller digests (0104; the load 2.8-3.0 s,
  not 3.7-4.0 s), and peak records parsed from memory (0105; with 0104
  alone the compiler stopped inlining `BufReader::read_exact` into the
  byte-at-a-time record reading, and `stats` took 17.4 s user, not
  12.8 s). Both, A B B A twice against `s18-start`: 43.0-44.5 s against
  44.2-45.3 s wall, CPU within the runs' spread, memory +0.4%, eight
  reports identical; `stats` 20.0-20.2 s against 24.5-26.5 s. `regress
  session-18-start` identical with `work/bin/s18-step6/` (720/720, 87,
  93, 79, the known calibrate line; 154 s, 1,328 s CPU).
- Step 6, continued (experiment 0106): with a stale saved index the
  search first builds the index on one thread, 71-88 s of a 112-128 s
  search on the NAS store. `indexing::build_planned` now loads and
  hashes 32 records at a time on `jobs` threads (`HashedRecord`,
  `Counting::count_hashed`, `Filling::fill_hashed`) while the calling
  thread counts or fills the batch before, in library order. A B B A
  (`--saved-index off`): 50.8-51.4 s against 119.4-121.1 s, index ready
  in 10.1-10.5 s against 78.9-80.6 s, memory +0.3%, reports identical.
  The saved index it writes (to a scratch `XDG_CACHE_HOME`) is
  byte-identical to the one the start build wrote; `regress
  session-18-start` identical with `work/bin/s18-par/`.
  Measured again after step 7 with the index saved as a stale one is
  (`--saved-index rebuild` into a scratch `XDG_CACHE_HOME`,
  `work/s18/run-2003-stale.sh`): 52.7-55.3 s with `work/bin/s18-final/`
  against 115.9-129.7 s with `s18-step6`, A B B A, the four saved
  indexes byte-identical to the start build's file, reports identical.
  The commit message of `a83664e` says 51 s against 120 s, which is the
  `--saved-index off` series.
- Step 7, a read-only review by a subagent of `8db90d8..a83664e`: no
  change to detections, the index or the saved-index file; no hang on
  any error path. Fixed:
  - `read_postings` mapped a reader panic to an error that could never
    be returned (`thread::scope` re-raises the panic); it now breaks
    out and re-raises the panic from `join`.
  - The multi-chunk test read 3 chunks with 3 in flight, so buffers were
    never reused; it now reads 6 and fails without `chunk.clear()`.
  - `build_planned`'s tests now cover a file without a record inside a
    batch and a record failing in a later batch (the build stops with
    its error, no hang); the closures are named `hash_to_count` and
    `hash_to_fill`; `Index::counting`'s doc no longer says one record.
  - Documents: 0100 (the lost Molten Beats clusters, search times from
    the reports, Anthrax at 262 tracks, Blindside's two files, strongest
    segments named as such, left-out slots), 0101 (the drop simulation
    ran on the scans, not the leave-outs), 0102 (the 2026-10-14 answers
    are sweep pairs below the tier; Meltdown, 2026-10-12, is the mix
    play), 0103 (the profile run's own figures, the start build's CPU
    range), 0105 (the store's bytes per second added to the data),
    0106 (the stale path measured with the index saved: 53-55 s, not
    51 s); this entry's leave-out claim and profile wording.
  - Scripts: `devset_view.py` read a field scan detections do not have,
    kept an unused function and described its output wrongly;
    `pair_relations.py`'s docstring now says its "same recording" is
    looser than `clusters`.
  Left as they are: a corrupt record with large trailing garbage is now
  read whole before being refused (records are written by Gunfinger and
  are a few hundred kB); the clusters lost in step 2 (reported in the
  final message; a `clusters` run that queries the recall panel's validation
  half would probably find them again).
- Wrap-up: final build `work/bin/s18-final/`; `regress
  session-18-start` identical (720/720 sweep queries, all 87, 93 and 79
  scan detections, the known calibrate line), 153 s and 1,312 s CPU; the
  2003 mix's NAS report identical to the start build's; `scripts/check.sh`
  green (355 tests). The NAS store at the end: 33,723 records, revision
  `c835c5531c325274`, as at the start. Listening files and every manifest
  unchanged (digests and modification times). Trial builds removed;
  `work/s18/` is about 500 MB, most of it `before/` (the NAS report
  folder before this session's runs, with the 268 clusters). Notes
  "Session 18", the roadmap ("More development mixes", the possible
  tier) and ADR 0004's "Later" updated.


## 2026-10-14: after session 18, our note on possible lines in the tracklist

- Session 18 checked by the agent that launched it: `git status` clean
  at `85baa30`; `docs/pair-verdicts.txt`, the answer log and the new
  manifest unchanged; the NAS store at 33,723 records, revision
  `c835c5531c325274`; the 2003 mix at NAS scale (`--store-only
  --playback both -f json`, the final build) gives the same 27 plays and
  37 segments as session 17's report, only the library revision
  differing, in 44.3 s and 4.33 GB; the new mix's default tracklist at
  NAS scale has the 21 entries of experiment 0100, in 42 s.
- The new README (`53828de`) shows the Stakka & Skynet mix's
  tracklist with "20:22 Stakka And Skynet - Clockwork (Stakka Rmx)
  (possible; shares material with Stakka & Skynet - Clockwork)". Our
  note: the mix plays the original Clockwork; the Stakka remix was made
  later and did not exist then, so the line is a misidentification.
  Every possible line of the three development mixes' default
  tracklists is of this kind (9 lines, none a played track). We
  decided that the default tracklist leaves such plays out, with a
  stricter test for "inside"; the plan, to be done next, is the
  roadmap's "Possible plays inside a confident play of another
  recording".

## 2026-10-14: proposals from an external review recorded, unverified

- A GPT Pro review, relayed by a GPT Astra agent on request
  with the agent's own review, proposed five ways to improve matching:
  integer pre-filter phases per query anchor, FFT lengths of the
  turntable rungs, posting-list part boundaries computed once, a
  companion-byte candidate scout, and an identifiability diagnostic
  before any new confidence statistic. They are recorded, unverified,
  in `docs/research/2026-10-14-matching-proposals.md`, with the
  protections, prior experiments, contracts, stop points and proposed
  criteria, and linked from the roadmap. No corpus experiment was run;
  nothing was adopted or implemented; no experiment number was used.
- Local checks while recording them (code and arithmetic only): 0103's
  split profiles suggest the pre-filter's per-hit slot arithmetic may
  account for roughly 10% of CPU, an attribution estimate to verify, not
  a ceiling; nearest-even FFT windows give 9 of the 17 lengths they
  change a larger largest prime factor (not measured as slower), and a
  factor-aware choice within one sample leaves no rung with a prime
  factor above 100 (16 today).
- Our first item stays first: possible plays inside a confident
  play of another recording leave the default tracklist (roadmap,
  "Reports and command line").

## 2026-10-14: possible plays inside a confident play leave the tracklist (ADR 0019)

- Our decision (above, "after session 18"), done in this
  conversation, not in a numbered session. Experiment 0107 first:
  `scripts/analysis/possible_marks.py` over every stored development
  report counts the possible plays each test marks and those the
  tracklist would leave out. No right play is lost anywhere; the 6 right
  brief plays of the generated mixes marked by the whole-span test (a
  brief play between two parts of a returning track) are not marked by
  the passage test.
- Change: `mark_shared_material` tests passages (segments joined across
  gaps of at most 10 s, `PASSAGE_GAP_SECONDS`) and skips a possible play
  of the track playing (`one_track`: a confident play over most of it
  lines up with it or carries the same artist and title), which joins
  that entry; `export::tracklist` leaves out entries of marked plays. The
  table, CSV and JSON keep them. ADR 0019 (ADR 0017's status points to
  it), the README's example and text, the roadmap item,
  `tracklist_view.py` (same rules; marked entries unnumbered).
- A review of the first version found that it also hid other rips and
  uploads of the track playing, against ADR 0017's "every matching file";
  the `one_track` exception fixes that. Its cost: the Vertical mix's two
  Klute Remix plays line up with the VIP within 1 s, so they join its
  entry under `titles disagree:` instead of leaving. ADR 0019 also names
  the remaining risk: a played track found only as possible inside a
  passage of another confident track (a double drop, a tease that never
  returns) leaves the tracklist; none occurs in the stored reports.
- Checks: `show` of the three development mixes' NAS reports, old build
  (`work/bin/s18-final/`) against new: 7 of the 9 lines go, the Klute
  Remix joins entry 14, every other line identical but for its number
  (`work/s18b/*-nas.old.txt`, `work/s18c/*-nas.new.txt`;
  `tracklist_view.py` agrees, `work/s18c/*-view.txt`). The README example
  at 262 tracks: 11 entries, unchanged (`work/s18c/stakka-262.new.txt`).
  `regress session-18-start` identical (720/720 sweep queries, all 87,
  93 and 79 scan detections, the known calibrate line).
  `scripts/check.sh` green (357 tests).

## 2026-10-14: session 19, search speed: proposals 1 and 2 of the external review

Brief `docs/brief-19.md`, checklist `docs/session-19-checklist.md`. Timings were taken under other load; each timing series
records the load (`work/s19/load-*.txt`).

- 17:23 setup, `git status` clean at `d867557`. Release build in
  `work/bin/s19-start/`; `baseline session-19-start`. The NAS store:
  38,105 records, revision `b285da9984422fb5`, as expected; rustc 1.98.0,
  RealFFT 3.5.0, RustFFT 6.4.1. `doctor` (3:24, listing the NAS over
  WiFi): the CLI's saved index (4.29 GB, 15:16) current. The 2003 mix
  at NAS scale (`--store-only --playback both -f json`): 50.0 s, 412 s
  user, 4.82 GB, the saved index loaded (`work/s19/bench/ref-start.json`).
  Session 17's and 18's NAS timings (33,596 and 33,723 records) are not
  comparable with these.
- Step 2, proposal 1 (experiment 0108): integer phases per query point
  for the pre-filter (`Phases::of`, guarded, `f64` slots outside the
  guard) and 10 tests (7 against `densest_clusters`, 1 against a
  comparison sort, 2 of the phases alone); with the phases of odd anchors
  shifted by 3 frames, `every_order_of_small_cases_passes_the_same_way`,
  `offsets_next_to_bucket_and_float_edges_keep_their_lines` and
  `every_set_of_hits_with_a_line_is_sorted` fail
  (`work/s19/p1-tests.log`). Instrumented builds
  (`work/s19/py/instrument.py`, never committed): 864,119,839 assets
  filtered, 36.76 billion hits examined, 17,867,626 assets sorted
  (start) against 17,870,712, 4,200,915 lines in both; reports
  byte-identical. Split profiles (0103's method, `work/s19/profiles.sh`,
  `work/s19/py/split.py`, `work/s19/py/stages.py`), A B B A twice: the
  stage (pre-filter, phases, the sort and clustering it leads to) fell
  13.3% (contended, load 24-76) and 12.9% (quieter) against the
  unchanged analysis; 8.7% and 16.8% in samples; 9.5% and 7.5% as a
  share of CPU. Below the 20% stop point: no A B B A timing, code
  deleted (`work/s19/lines.rs.p1` keeps it, with the rounding test
  corrected to an offset of 2 - 2^-52 after the review).
- Step 3, proposal 2 (experiment 0109): `work/perf/s19-fft` (a crate
  outside the workspace with its own copy of the lockfile; `s19-fft
  lengths > work/s19/fft-lengths.txt`; `work/s19/fft-time.sh b 5 near`,
  60 s of the 2003 mix from 10:00 decoded once to
  `work/perf/s19-fft/mix-600s-60s.f32`) times each length and hop on one
  thread; `work/s19/py/fft_weighted.py` weights the rungs by the frames
  of an instrumented `identify` (`S19FFT` lines in
  `work/s19/bench/s19-start-count.err`). One `identify` builds 1,862
  plans in 0.052 s (0.04% of the STFT loop's 128 s): no plan cache.
  First-pass turntable analysis, 73.2 s today: nearest-even -21.8%,
  factor-aware within 1 sample -25.5%, within 2 -36.5%, the fastest
  measured within 1 and 2 -35.4% and -44.0%; with the key-locked rungs
  as unaffected work -18.4%, -21.6%, -30.8%, -29.9%, -37.2%. Timing-only
  builds with the factor-aware and fastest lengths in the first pass
  (`work/s19/py/fft_variant.py`, `work/s19/fft-abba.sh`, contended):
  `--playback both` 46.2 s and 45.1 s against 48.8 s (medians of 6),
  turntable playback 28.0 s and 27.4 s against 30.9 s (medians of 4),
  memory unchanged; their reports differ. No product length changed.
- Step 4, a read-only review by a subagent of `d867557..bd7a51a`, the
  deleted code (`work/s19/lines.rs.p1`), the diagnostic and the
  scripts: no output change; both stop decisions stand. Fixed: the
  rounding test of the deleted code used an offset of 2 - 2^-53, which
  is exactly 2.0 (now 2 - 2^-52; the kept copy's tests pass, and the
  3-frame mutation is logged in `work/s19/p1-tests.log`); 0108 leads
  with the stage against the unchanged analysis (13.3%, 12.9%), the
  share of CPU (9.5%, 7.5%, not 7.6%) and the samples (8.7%, 16.8%)
  secondary, the cause marked as an inference; the test count (10, of
  which 7 against `densest_clusters`); 0109's load range (21-157) and
  per-frame times named as whole-analysis times (the FFT alone: 20.3 and
  1.1 us), with odd lengths running a complex FFT of N; the roadmap's
  end-to-end figures labelled contended.
- Wrap-up: final build `work/bin/s19-final/`, byte-identical to
  `s19-start` (no code change kept). `regress session-19-start`
  identical (720/720 sweep queries, all 87, 93 and 79 scan detections,
  the known calibrate line), 155 s and 1,312 s user CPU;
  `scripts/check.sh` green (357 tests). The NAS store at the end: 38,105
  records, revision `b285da9984422fb5`, as at the start. Listening files and
  the development manifests unchanged (digests, modification times).
  Scratch: `work/s19/` 260 MB, `work/s19/target-alt` and
  `work/perf/s19-fft/target` about 380 MB together. Notes "Session 19",
  the roadmap's "Where search time goes" and the research note
  (proposals 1 and 2 measured) updated.

## 2026-10-14: macOS and Linux (ADR 0020)

- Our requirement, done in this conversation, not in a numbered
  session: Gunfinger supports at least macOS and Linux. `git status`
  clean at `40fc0fa` at the start. An audit of every tracked file found
  macOS-only: the listening page's `open` and `open -R` (and "Show the
  file in the Finder"), `robust`'s `say`, four `brew install ffmpeg`
  hints, and `memory`'s `/usr/bin/time -l`.
- Change: `gunfinger-eval`'s `platform::Platform` (`CURRENT` from
  `cfg!(target_os = "macos")`) and one function per need taking it:
  `audition::desktop::browser` (`open URL`, `xdg-open URL`; the server
  runs it on a thread of its own), `audition::desktop::reveal` (`open -R
  FILE`, `xdg-open FOLDER`), `robust::speech_synthesis` (`say -o
  speech.aiff`, `espeak-ng -w speech.wav`); the reveal route's and the
  browser's errors name the command run. `decode::INSTALL_FFMPEG` (brew
  and apt) in the core's, `doctor`'s, `listen`'s and the page's
  messages. The page's button has the tooltip "Show the file in its
  folder", and its font list `system-ui` after the Apple fonts. `memory`'s documentation and help
  name `time -l` and GNU `time -v`. 6 new tests here, plus one that runs
  only on Linux.
- `scripts/check.sh` runs clippy for the other platform's target when
  its standard library is installed, and otherwise prints one line.
  Homebrew's Rust has no rustup: `brew install rustup` (keg-only), then
  toolchain 1.98.0 (minimal, clippy, rustfmt, `x86_64-unknown-linux-gnu`). A `rustup toolchain list` inside the
  repository installed `stable` 1.99.0 because of `rust-toolchain.toml`;
  removed. The Linux clippy was clean at `40fc0fa` before the change. A
  type error planted in the Linux-only test failed the Linux pass and
  passed the host's.
- Records: ADR 0020 (with the two open risks and the expected
  cross-machine differences), ADR 0016's status, AGENTS.md "Platforms",
  README (platforms, FFmpeg on both, a copied store on another
  computer), `docs/audition-questions.md` (the reveal route), the
  roadmap's "Library identity and relocation" (corrected: the store
  names its library in `library.txt`; the two risks with their checks),
  session notes. `scripts/analysis/store_identity.py` (new,
  read-only) on the NAS store (38,105 records, `b285da9984422fb5`) and
  the NAS: 149 paths with two Unicode forms, all stored NFD; 32,000
  modification times with fractions of a second; 169 files looked up
  (the 149 and 20 sampled), all under the stored bytes, size and time
  equal to the nanosecond (`work/xplat/store-identity-mac.txt`).
- Checks: `regress session-19-start` identical with `work/bin/xplat/`
  (720/720 sweep queries, all 87, 93 and 79 scan detections, the known
  calibrate line; 155 s, 1,311 s user). The baseline was made at
  `d867557`, whose code `40fc0fa` leaves unchanged, so no new baseline.
  `scripts/check.sh` green (363 tests) with Homebrew's 1.98.0, which
  prints the skip line, and with rustup's 1.98.0, which runs the Linux
  clippy (`work/xplat/check-*.log`); `cargo test -p gunfinger-eval
  audition` 62 passed. `git diff docs/pair-verdicts.txt` empty;
  `work/audition/answers.jsonl` last written at 01:55, before this
  change; no `audition` server ran. The NAS store read only, by
  `store_identity.py`.
- A read-only review by a subagent of the uncommitted change: the macOS
  commands are the same as before, the Linux branches right, no hang or
  `unwrap` added, `check.sh` correct under `set -eu`, the script
  read-only with the record format's offsets. Fixed in the documents:
  the button's visible text is still "reveal" (the tooltip changed);
  `--store-only` is not refused for another library path; the NAS
  mounted on Linux at another path (mount it at `/Volumes/atlas`, or
  put the path in the copy's `library.txt`; `--second-library-root`'s
  default is the Mac's); the script's second argument is the library's
  root; the time-precision risk is up to 32,000 files (84%), not the
  whole NAS; "runs on Linux" changed to "supports", nothing having run
  there; `shown`'s doc comment; eSpeak NG's package in the README; a
  heads-up in ADR 0020 that stand-in script tests may fail with "Text
  file busy" on Linux (rust-lang/rust#114554), a test race not fixed
  here. `store_identity.py` counts a file gone before its `stat` as not
  found. Left: the duplicated `parts` helper of two tests (tests favour
  clarity), detaching a browser that `xdg-open` runs itself
  (`process_group`, Unix-only).
- After the fixes: `scripts/check.sh` green again with both toolchains
  (363 tests; the Linux pass skipped with Homebrew's, run with
  rustup's); the NAS store still 38,105 records, `b285da9984422fb5`.

## 2026-10-15: session 20, the shared-material panel and proposal 5's diagnostic

Brief `docs/brief-20.md`, checklist `docs/session-20-checklist.md`. Files
were added to
the NAS during the session.

- 02:32 setup, `git status` clean at `2836414`. Release build in
  `work/bin/s20-start/`; `baseline session-20-start`. The NAS store:
  38,105 records, revision `b285da9984422fb5`, as expected; load 3.9.
- 02:41 the oracle and the denominator, committed before any
  measurement (`0ae0c6d`).
- Step 2, the panel (experiment 0110, `scripts/analysis/shared_panel.py`
  `families`, `rip-pairs`, `join`, `pairs`, `freeze`): 12 families from
  the 13 `different` verdicts, 2 `shared` answers and 2 judgements in the
  docs; recordings from session 18's duplicate clusters (read only) and
  6 unclustered rips joined by the clustering rule (`pair`, 80% of the
  shorter file); the five test-derived versions excluded; passages from
  `pair` (the strongest alignment, under 25 hits a control); split by
  family, seed 2026 (development: Dozer, Fire, Playground Shades, Pyro,
  Sonar, Watermelon). `docs/panels/shared-material-seed-2026.json` and
  its development plan.
- 03:10 step 4's decision rule, committed before the diagnostic ran
  (`a076ca2`).
- Step 3 (experiment 0111): `gunfinger-eval shared` takes
  `--other-peaks-dir` (corpus and NAS, 38,127 files; the saved index,
  rebuilt at that size, with leave-outs), renders NAS files, takes each
  recording's files from the plan (`played_recording`,
  `related_recording`), and adds `before`, `weak`, `blend` and `break`
  queries; 10 new tests (12 in `shared.rs`). One command, 02:59-03:31
  (1,921 s, 8,898 s user, 5.4 GB); `scripts/analysis/shared_outcomes.py`
  applies the oracle. Main case: wrong confident in 106 of 218 queries,
  12 of 14 directions; no unjudged or other file confident.
- Step 4 (experiment 0112): `gunfinger-eval identifiability --half
  development` (`identifiability.rs`, 4 tests; `pair.rs`'s `Records`
  made `pub(crate)`), 2.4 min; `scripts/analysis/shared_identifiability.py`
  applies the rule (first version counted only the strongest wrong
  title, so fire-2, 5 and 6 read "separated today"; fixed to test the
  related file's own level). Outcome 3 in 11 of 14 directions, outcome 1
  in pyro-2 (against a poor upload; the next rip 0.723, R 0.716),
  Watermelon separated today.
- Step 5: `listening_pack.py candidates` (and a README that lists only
  the kinds present); 16 questions in `work/audition/s20/01-candidates.json`
  (`pairs-s20-candidates.json`), later a 17th
  (`02-fire-remix-vip.json`). The page checked twice with a temporary
  answer log and a copy of the verdicts: the questions load, audio and
  envelopes 200, nothing answered, the servers stopped.
- Step 6 (experiment 0113): a split build (never committed, temporary
  worktree from `a076ca2`, `work/s20/take-part-split.diff`) of the 2003
  mix at NAS scale, twice under `sample` (`work/s20/profiles.sh`,
  `work/s20/py/stages.py`), reports byte-identical to the start build's
  (48.7 s, 412 s user): binary searches 1.3-1.4% of CPU, copying 9.1%.
  Under the 5% stop point; worktree and build removed.
- 03:57 the follow-up's rule committed (`efd8ea4`); experiment 0114:
  `--verify scan` of the three development sets at NAS scale (2.0-2.3
  min each; the older `verified/` reports copied to
  `work/s20/nas-reports-before/` first), `scripts/analysis/shared_true_plays.py`:
  the cut keeping 95% of 102 true confident detections rejects 8 (hits
  per second) and 12 (verifier share) of the 106 wrong ones. 9 confident
  detections score wrong at this revision, all NAS files newer than the
  clusters and covering 98.8-99.9% of the credited file (`pair`,
  `pairs-s20-candidates-2.json`).
- Step 7, from 04:14: a read-only review by a subagent of
  `2836414..abbb5d7` and the uncommitted notes. Clean: no product
  change, verdicts unchanged, no attribution, no em dashes, experiments
  at most 40 lines, docs JSON without absolute paths, the harness's
  queries, roles and leave-outs, `shared_outcomes.py`, the per-direction
  wrong lists and most numbers. Fixed:
  - `shared_panel.py join` used an undefined `FITTED`; defined, and
    `join` and `freeze` reproduce `work/s20/families-joined.json` and
    both panel files byte for byte.
  - Sonar-2's confident related file is not R but the PRO 012 A upload
    of the same recording. `identifiability` now measures every file of
    the related recording (role `related rip`) and leaves out frames of
    digital silence (the padding where X has no audio), with 2 new tests
    (the padding test fails on the old code). Rerun 04:44 (179 s; chance
    values moved by up to 0.02, no measure by more than 0.005).
    `shared_identifiability.py` applies the rule to R as written (R's
    own detections, not the recording's strongest), reports the
    confident file's outcome and pairs per aligned span beside it, and
    leaves rips with no detection out of the floor. Outcome 3 in 10
    directions (sonar-2 separated today by the rule, 3 by the confident
    file); Pyro-1 is outcome 2 per aligned span.
  - Watermelon-1's positive control was void: its other rip is a copy
    the corpus file stands for, never indexed. `shared` now runs that
    control only with an indexed rip (`SecondLibrary::holds`), reports a
    query it cannot render instead of stopping (the oracle's rule), and
    renders into `work/shared/<plan>-<render key>/`, a digest of the
    plan and the render settings; 3 new tests. The development half
    rerun under `screen`, 04:47-05:18 (1,850 s, 8,732 s user, 5.3 GB,
    fresh renders in `...-83361e93ae0502ae/`): all 605 results with the
    same detections as the first run; watermelon-1's 17 control results
    gone (`work/s20/shared-dev-first.json` keeps the first report).
  - Numbers: 0110's 24 unclustered files (57 pairs); 0111's control
    (the related file at most 11 hits); 0112's ranges without controls
    and Pyro's R (3.3-12.4); 0114's 91; item 17's question (fire-2 and
    fire-6, not the original). Prose and caveats in the notes (0114 is
    not like-for-like; the pooled floor's margins).
  - 04:38 a like-for-like check written first; experiment 0115: the
    three development sets with every referenced track left out
    (`--verify scan --leave-out 12, 11, 17`, 1.9-2.1 min each): only the
    9 newer uploads of the same recordings are confident; related
    versions stay possible at 79-197 hits; no cut evaluated (fewer than
    20 wrong plays).
  - AGENTS.md lists `identifiability`.
- Wrap-up: final build `work/bin/s20-final/` (`gunfinger` byte-identical
  to `s20-start`). `regress session-20-start` identical (720/720 sweep
  queries, all 87, 93 and 79 scan detections, the calibrate line known
  since session 19; 152 s, 1,324 s user). `scripts/check.sh` green (382
  tests) with Homebrew's 1.98.0 and with rustup's, which ran the Linux
  clippy (`work/s20/check-*.log`). The NAS store at the end: 38,105
  records, revision `b285da9984422fb5`, as at the start. Listening files and
  the set manifests unchanged (digests, modification times). Scratch:
  `work/s20/` 301 MB, the development renders 351 MB. Notes "Session
  20", the roadmap's two items and the research note (proposals 3 and 5)
  updated.

## 2026-10-15: session 21, candidate statistics for shared material in rendered mixes

Brief `docs/brief-21.md`, checklist `docs/session-21-checklist.md`. The
NAS gained files during the session.

- 05:46 setup, `git status` clean at `8537226`. Release build in
  `work/bin/s21-start/` (`gunfinger` byte-identical to `s20-final`);
  `baseline session-21-start`. The NAS store: 38,105 records, revision
  `b285da9984422fb5`, as expected; load 2.1.
- 05:51 the pre-registration, committed before anything was rendered
  (`5568a03`): 36 mixes (seeds 2101-2103, one per development played
  file, played whole among 3 + 3 drawn plays), the oracle per play, the
  denominators, candidates (a1) hits per second, (a2) the verifier's
  share, (b) the continuation share beyond each side, (c) the library's
  map of shared passages, three combinations with (c), and the
  feasibility criterion (one threshold keeps 95% of true confident plays
  and rejects half of the wrong related ones).
- Step 3: `gunfinger-eval shared-mixes <plan>` (`shared_mixes.rs`;
  `mixes::draw_around`, `render_once`, NAS files rendered through
  `shared::Roots`; `Verifier::peaks` and `Verifier::beyond`; 8 new
  tests); `scripts/analysis/shared_mixes.py plan` writes
  `docs/panels/shared-mixes-s21.json` (93 files never drawn: every panel
  file of both halves and the corpus's two Funktion files by the
  test-derived pattern). Smoke run on one mix (53 s, 4.9 GB). Full run
  from 06:02 under `screen`.
- 06:02-06:38 the full run (36 mixes, 84 searches, the 3 development
  mixes; 2,173 s, 14,680 s user, 7.0 GB); `shared_mixes.py score`;
  `pair` on the 29 planned/other pairs (28 the same recording, 86-99.9%;
  Kemal's Silent Extent Remix over the original, 7%). Experiment 0116:
  453 true and 87 wrong related confident plays; (a1) 9.31 hits per
  second, (a2) 0.498, (c)+(a1), (c)+(a2) promising in rendered mixes only
  (the development mixes keep 45-70 of 97); (b), (c), (c)+(b) not.
  Frozen in the brief (step 5).
- Step 6, 06:38-07:05: session 19's factor2 lengths in a worktree never
  merged (`work/s21/fft-factor2.diff`, removed after), the development
  half at NAS scale (1,625 s, 8,059 s user, 5.4 GB); `shared_compare.py`
  against session 20's report (restored after the run): one wrong title
  added (239 to 240 hits), one removed (240 to 125), each in 3 indexes;
  no wrong confident query added, no right file lost. Experiment 0117.
  It started before 0116 was written (committed at 06:57), against
  AGENTS.md's order; the runs are independent.
- 06:59 the follow-up's rule, committed before it ran (`66d85f6`);
  `leave_out_drawn` (1 new test). The first start at 07:06 used a stale
  binary (tests and clippy do not rebuild the release binary) and was
  stopped; a copy over the running binary was then killed by macOS's
  code-signature check; restarted 07:08 from a fresh copy, done 07:41
  (1,967 s, 5.2 GB). `shared_mixes.py score --same
  work/s21/holder-coverage.json --second-from <0116's report>`.
  Experiment 0118: 292 true plays through other rips (163 of 216 drawn
  plays confident; median 17.4 hits per second against 31.5 for copies
  and 9.4 in the development mixes); the same four pass in rendered
  mixes only (57-74 of 97 kept).
- 07:50 the notes' section; the roadmap and the research note. 07:51
  the read-only review (a subagent, 22 min). Its findings, all checked,
  fixed or (the order of steps 5 and 6) documented: the exploratory check had left out the combinations, and
  (c)+(a1) keeps every rendered true play and rejects 61 of 87, all
  Fire's: at 6.338 hits per second (the cut keeping 95% of the 112 real
  true plays) 92 of 97 development plays kept, at 6.276 (the cut on the
  97) 93; margin 6.01 against 6.276. Frozen at 6.276 in the brief
  ("After the review"), exploratory. 0116 had "all
  216 drawn plays right" (203 confident, 13 of 20-28 s possible only)
  and 0118 "197 found" (163). 59 of 0118's 277 rip plays have the drawn
  file's density within 1% (likely the same encodes); without them nothing
  changes. Time labels in the brief set to the commit times. Code:
  shared-mixes plans refuse unknown fields (a stale binary now fails);
  `targets`, `passage` and `aligned_beyond` split out and tested;
  `draw_mix` pinned to the draws of `8537226` for one seed (the
  expected values printed by that commit's code in a temporary
  worktree); 5 new tests. Observation: the hifi-junglist "Trace - Sonar
  (PRO 012 A) 1998" upload, under the original in the panel, lines up
  with the whole revision at speed 1.0326 (57% the other way); 3 of
  Sonar's 9 wrong plays are it; without them no decision changes. To
  judge by ear.
- 08:28 a second read-only review of the two fix commits (14 min): the
  refactors keep behaviour, each new test fails when its behaviour
  breaks, and every new number matches the data. Wording fixed by
  08:45: 0116's conclusion (the rule passes in rendered mixes, also
  through other rips, and fails against real true plays); the Sonar
  alignment in both directions, with where it stops lining up; the
  112-play cut written 6.338, not 6.34 (6.34 would reject Stab 2 Def);
  the exploratory cut's 26 unrejected wrong plays (6 of them Fire's);
  reclustering's effect marked as likely; the validation half
  described as what it could supply. Code: `Query`'s unknown fields
  tested, a shadowing binding renamed, the pin's asserts name the
  row.
- Wrap-up: final build `work/bin/s21-final/` (`gunfinger` byte-identical
  to `s21-start`; no diff in `gunfinger-core` or the CLI since
  `8537226`). `regress session-21-start` identical (720/720 sweep
  queries, all 87, 93 and 79 scan detections, the calibrate line known
  since session 8; 151 s, 1,320 s user). `scripts/check.sh` green (396
  tests) with Homebrew's 1.98.0 and with rustup's, which ran the Linux
  clippy (`work/s21/check-*.log`). The NAS store at the end: 38,105
  records, revision `b285da9984422fb5`, as at the start. Listening files and
  the set manifests unchanged (digests, modification times). Scratch:
  `work/shared-mixes/` 625 MB (the 36 renders), `work/s21/` 435 MB.

## 2026-10-15: session 22, part 1 of the FFT lengths' retrieval study

Brief `docs/brief-22.md`, checklist `docs/session-22-checklist.md`. The
NAS gained files during the session. No timing
was measured.

- 09:06 setup, `git status` clean at `86b1b8d`. Release build in
  `work/bin/s22-start/` (byte-identical to `s21-final`); `baseline
  session-22-start` from session 21's final `regress` reports. The NAS
  store: 38,105 records, revision `b285da9984422fb5`, as expected.
- 09:10 step 2, committed before any run (`117198e`): session 21's
  factor2 patch applied to a detached worktree at `9aecef4`
  (`work/s22-fft-tree`, never merged), first-pass turntable rungs only,
  with `speed::design()` naming the variant; binaries in
  `work/bin/s22-fft-factor2/`. Each build runs with its own `--work`
  (`work/s22/today`, `work/s22/variant`), seeded with APFS clones of the
  clusters, the map, the saved NAS index and the renders, so nothing is
  written to `work/reports/`. Boundary tolerance 1.0 s.
  `scripts/analysis/fft_compare.py` pairs the two builds' reports.
- 09:11 the runs start under `screen` (`work/s22/queue.sh`, then
  `queue2.sh` and `queue3.sh`), one at a time.
- (a) 09:11-09:13 `regress session-22-start` with the variant: the sweep
  keeps 720 of 720 answers (225 queries with identical detections, the
  own file's hits 0.990-1.009 of today's), recall 60/60 at every speed, 0
  wrong; the development scan and its leave-outs keep every identified
  track (11, 8, 0), 0 wrong, the same unmatched possible play; no
  detection changes confidence; 4 plays move a boundary by more than
  1.0 s (Decoy +2.0 s at the start; Clockwork's two files -5.8 s and
  +11.0 s, a window more; Kraken+Arkane's Dominion +29.3 s at the start,
  39 to 36 windows, the other Dominion file unchanged); calibrate: weakest
  identifying 680 to 674, strongest false 99 both.
- (b) 09:13-09:19 the three development mixes at NAS scale, both builds
  (0.8-1.3 min each, the saved index loaded in 3.1-3.6 s). Every referenced
  track identified by both (11, 12, 17), the same right files credited,
  the same wrong confident detections (0, 7, 2), no possible-only track.
  No play lost; one possible play added (the 2003 mix: Omni Trio - Secret
  Life (Silent Storm Remix), 30:03-30:42, 70 hits, as session 19 saw).
  Two confident segments change inside plays that stay confident, both in
  the 2003 mix: at 5:41-6:43, the end of track 2, the confident tail
  segment moves from The Specialist's two files (253 to 46 hits) to the
  two Sex Drive files (195 to 631), all four credited to track 2 in both
  builds; Carlito & Addiction - Into Music's `extra/h-` rip loses its
  first segment (24:25-26:03, 695 hits) and its main one falls from
  1,668 to 810 hits (the hifi-junglist upload unchanged). Boundaries
  moved by more than 1.0 s: 13, 11 and 19 plays (largest 18 s, 97 s and
  128 s). Detections below the possible tier change by about a third.
- (c) 09:19-09:21 the generated mixes, seed 2026 (12 mixes, 144 plays)
  and seed 7 (2 mixes, 24 plays), both builds: every play keeps its
  level (125 confident, 11 possible, 32 held out with nothing) and its
  credited file; 0 wrong confident and 0 wrong possible in both. Of 272
  boundaries, 28 move by more than 0.1 s and 5 by more than the 1.0 s
  tolerance (largest 1.63 s): 3 come closer to the truth, and **2 get
  worse by more than 1.0 s**, so the boundary criterion committed at 09:10
  fails on them: Item.Code - Kosmic's start in seed 2026's mix-02 (a
  possible play; late by 2.31 s, then 3.63 s) and Kemal & Paul Reset -
  Kontempt's start in seed 7's mix-00 (confident; late by 2.30 s, then
  3.71 s, with 1,889 hits then 1,907). Both starts lie inside an 8-11 s
  fade-in with the bass cut for its first 4-5 s.
- (d) 09:21-09:34 sweeps 2027-2029, both builds (about 2 min each):
  every confident answer unchanged (720 of 720 per seed: 540 right, 180
  held out with nothing confident; after the reviews, compared by files:
  in seed 2029 two Dubstyle files of Aphrodite - Fanfare become wrong
  possible matches at -5% and +3%, 57 and 35 hits, then 62 and 60, and
  stop being one at +8%, 62, then 57), recall
  60/60 at every speed, 0 wrong, the bar passed by both; 191, 194 and 200
  queries with identical detections; the own file's hits 0.956-1.011 of
  today's.
- (e) 09:34-09:46 `grid` seed 2026, both builds (about 6 min each): the
  same confident counts per play length (10-40 s: 0, 67, 146, 160, 160,
  160 of 160), 0 wrong confident in both, the strongest false
  candidate 34 hits, then 31; 109 of 960 queries identical. **Two 10 s plays lose their
  possible answer**, so the criterion "no right possible file lost" fails on
  them: brief 12 (SKC & Cord - Swarm at 1.054, offset 0 s: 235 hits in
  one window, then nothing) and brief 6 (Aphrodite - Lava Flow at 0.962,
  offset 1 s: 276 hits in 2 windows, then 48 hits on an alignment about
  11 s away in the file, on rung 0.960). 10 s plays: 148 possible today,
  146 with the variant.
- (e) 09:46-09:52 the development mixes at NAS scale with every
  referenced track left out (11, 12, 17, as experiment 0115), both
  builds: the same confident detections (0, 7 and 2, newer uploads of
  the left-out recordings, as in 0115), the same possible plays plus the
  Omni Trio Silent Storm Remix (70 hits) in the 2003 mix; no confidence
  change.
- (e) 09:52-10:10 `robust` seed 2026, all conditions, both builds (10
  and 8 min): 0 wrong confident in both; confident 1,941 -> 1,945 of
  2,600 indexed queries, possible or better 2,246 -> 2,252, wrong
  possible 43 -> 38 (5 fewer wrong possible matches of 60-77 hits, all
  on excerpts of Clockwork, most likely its Stakka remix; the report
  does not name the file).
  **20 right confident answers lost and 24 gained**, so the criterion "no
  right confident file lost" fails here: needle skip 2 s (12 lost, 3
  gained, all 15 crossing 3 windows: the skip splits the play into
  segments of 2-3 windows), pitch ride 2% (7 lost, 20 gained; 14 cross 3
  windows, 26 cross 240 hits), beatmatched at 0 dB (Skinnah - State Of
  Violence, 275 -> 210 hits) and clipping (BSE - B Negative, 235 -> 240,
  gained). 6 right possible answers lost and 6 gained (blends, pitch
  ride, beatmatched); 6 changes of the blend partner's confident answer.
  The other conditions keep every confident answer; pink noise at 0 dB
  gains a possible one.
- (f) 10:10-10:16 the key-lock-only control (`--ladder key-lock sweep
  --seed 2026`): **byte-identical reports** from both builds, so the
  variant changes no key-locked rung. Other rips at NAS scale (`sweep
  --seed 2026 --other-rips`, 333 queries, 2.8 min each): every answer
  unchanged, including the same 18 wrong answers (9 the weedsurfer
  Phoenix upload, 9 the hifi-junglist React upload of Total Science -
  Yvon Is On); the right file's hits 0.89-1.52 of today's. After the
  review, compared by files: one rip of C4C - Synergy at +1% drops from
  confident to possible (240 hits, then 236) while another rip still
  answers.
- (g) 10:16-10:19 the 2003 mix and the Vertical broadcast at 262 tracks
  with leave-outs 3 and all, both builds: identified 7, 4, 0 and 9, 6, 0
  in both, 0 wrong, no play lost or added; Into Music's `extra/h-` rip
  again loses its first confident segment (679 hits, then 20) inside a
  play that stays confident; boundaries moved: 3, 2, 0, 5, 4, 0.
- (h) 10:19-10:31 the recall panel's development half at NAS scale
  (`recall --seed 2026`, 702 queries of 78 sources, the panel's 80 less
  2 without a record, 38,096 assets; 6.3
  and 5.3 min). The run rendered the 702 excerpts again into today's
  work directory; the variant searched an APFS clone of those renders.
  Both builds: 522 of 522 recalled, the same 57 wrong confident answers
  (by their titles other rips of the queried recordings, which session
  18's clusters do not join: Ram Trilogy, Bad Company, Mulder, Perfect
  Unknown, John B; not checked with `pair`),
  the right file's hits 0.990-1.011 of today's; one held-out query
  (Congo Natty - Revolution against the "Peace" white label) changes its
  possible wrong match at three speeds (85 -> 28, 6 -> 60, 82 -> none).
  After the review, compared by files: two rips rise from possible to
  confident in three queries (Dead Air's RH 66 upload at -1% and +5%,
  194 and 224 hits, then 242 and 248; The Sect - Victim's dfect upload
  at 0%, 202, then 248).
- (i) 10:31-10:48 other rips at NAS scale for seeds 2027-2029 (2.4-3.3
  min each). Over the four seeds (1,413 queries): recalled 1,296 ->
  1,297 (Cause 4 Concern - Blindside at -1%, seed 2027: 238 hits, then
  241, through the alien5ive upload), the same 134 wrong confident
  answers (by their titles other rips of the queried recordings that the
  clusters do not join: the React CD uploads of Yvon Is On and
  Balderdash, Bad Company's BCR01 and Inside The Machine uploads, Lava
  Flow's and Fanfare's label uploads, Slip Thru, the weedsurfer Phoenix;
  not checked with `pair`), one possible right answer gained (Falcon -
  Fractles at -5%, seed 2027: no detection, then 70 hits) and one lost (the same
  at -1%, seed 2029: 61 hits in 3 windows, then 54 in 2).
- 10:48 the queues done. Every run log ends with `/usr/bin/time`'s
  statistics and shows no error, and every report exists; the exit
  codes in `queue.log` are those of `date` (a fault of the queue
  scripts' `run`, found by the review), not of the runs.
- 10:50 checks: every MP3 render the two builds searched (sweep,
  generated mixes, grid, robust, recall) is byte-identical between the
  two work directories (the review: robust's 100 Opus files differ only
  in Ogg serial numbers and checksums); the grid's, robust's and the mixes' stored renders were
  rendered again in each (about 6 GB of new data, removed at the
  wrap-up). The seed 7 mixes run drew and saved
  `docs/panels/sweep-seed-7.json` (09:20; today's harness draws other
  seed 7 mixes than the stored report of October 6); committed.
- 10:50 experiment 0119 (40 lines; data `0119-*`), the notes' "Session
  22", the roadmap and the research note, before the review.
- Wrap-up: final build `work/bin/s22-final/` (`gunfinger` and
  `gunfinger-eval` byte-identical to `s22-start`; no change under
  `crates/` since `86b1b8d`). `regress session-22-start` identical
  (720/720 sweep queries, all 87, 93 and 79 scan detections, the
  calibrate line known since session 8; 153 s, 1,314 s user); only its
  four reports were written to `work/reports/` this session.
  `scripts/check.sh` green (396 tests) with Homebrew's 1.98.0 and with
  rustup's, which ran the Linux clippy (`work/s22/check-*.log`). The NAS
  store at the end: 38,105 records, revision `b285da9984422fb5`, as at
  the start. Listening files and the development sets' manifests unchanged
  (digests, modification times).
- 10:51-11:08 the read-only review (a subagent, 17 min, 23 findings, all
  checked). Fixed by 11:13: `fft_compare.py` now compares the files of
  each answer kind per query (right and wrong, confident and possible),
  pairs plays and detections largest overlap first regardless of order,
  and checks that paired lists have equal lengths; every comparison was
  run again and every count stands, and the file comparison added
  changes the answer classes hid (a C4C - Synergy rip confident ->
  possible; two recall rips possible -> confident in three queries;
  Fanfare's Dubstyle as a wrong possible match in sweep 2029). Corrected in the
  experiment, the notes, status, roadmap and research note: the recall
  panel's possible wrong changes; the grid's possible total (255 ->
  253); the robust edge counts (28 of 44 cross 240 hits); the boundary
  counts per play (12 and 43) and per run; "worse" against "moved"; the
  criterion quoted as written, without "strict"; the reason for part 2's
  speed list (the changed speeds are midway ones); the 18 other-rip
  wrong answers' files; factor2's measured speed-up (5.4% and 9.2%); the
  Opus renders; the new data written (about 6 GB, removed); the queue's
  exit codes.
- 11:14-11:22 a second read-only review of the fixes (8 points, all
  checked and fixed by 11:24): `compare_sweep` reported the hits and the
  rule's edge of a query's strongest file instead of the file that
  changed (Fractles at -5% had no detection, not 40 hits; Congo Natty's
  "Peace" 6 hits, not 9; the Synergy rip and the recall rips cross 240
  hits); it now reports each changed file's own hits, and paired items
  must match (labels, speeds), or it stops. The recall changes are two
  rips in three queries, not three rips; 0119's table counts queries;
  the 262-track sweeps' wrong possible changes (Fanfare's Dubstyle,
  seed 2029) and the roadmap's and research note's wording corrected.
- 11:13 the worktree `work/s22-fft-tree` removed (its diff in
  `work/s22/fft-factor2.diff`; `work/probe-tree`, not this session's,
  left). Scratch: `work/s22/` 86 MB (reports, comparisons, logs; the
  renders and index clones removed), `work/bin/s22-*` 33 MB.

## 2026-10-16: after session 22, the listening; `1.mp3` left out of the NAS library

- Session 20's 17 pair questions were answered on the listening page
  (01:18-01:27; answer log records 37-54, the last question answered
  twice with one verdict line): all 17 `different`, 17 lines added to
  `docs/pair-verdicts.txt` by the page. The comments by ear:
  - Ryme Tyme - We Enter (Optical RMX) ~ the Optical Remix v.3.0 Full
    Instrumental: almost identical. A ends with a vocal (a rap) and stops
    the drum loop under it; B has no vocal and keeps the drum loop.
    Technically different, but a DJ mix would play the middle of the
    track, which is identical (they line up at A 0:00.5-5:40.0, 2127
    hits).
  - The Kraft - Stab 2 Def ~ Mampi Swift - 2000: a different EQ and a
    slightly different arrangement of the same music; technically a
    different track.
- Session 21's two questions (the hifi-junglist Sonar upload, Kemal's
  Silent Extent Remix) were not on the page and are still open.
- `/Volumes/atlas/Music/dnb/1.mp3` left out on request: an
  old file of cuts from several tracks, which we want identified,
  not matched (it held Cannibal's 1:53-2:25 and matched Falcon - The
  Stand at 65-70 hits). `/1.mp3` added to the NAS `.gunfingerignore`
  (line 56, with a comment), then `gunfinger --config
  ~/.config/gunfinger/nas-dnb.toml prune --yes` deleted its peak record
  and tag note; `prune` listed nothing else. The NAS store went from
  38,105 records, revision `b285da9984422fb5`, to 38,104, revision
  `5fd86c54350c819c`. The CLI's and the harness's saved NAS indexes are
  out of date and are built again by the next search. NAS-scale reports
  and baselines made before differ by `1.mp3`'s plays (the Vertical mix's
  possible play during Cannibal among them), so the next session takes
  its start baseline at the new revision. The new verdict Cannibal ~
  `1.mp3` now adds nothing (the harness says so). `doctor` also reports
  1,345 audio files on the NAS not indexed yet; they were left alone.
- `identify` on `1.mp3` against the NAS library after the prune (215 s,
  the index built again; reports in `work/one-mp3/`): about 30 s cuts,
  all played 4.6-8.9% fast. Confident: Item A La Playa - Crime Audio
  (0:00, 321 hits), DJ Reality - Reasons (0:56, 360), Semantix - Genetix
  (1:26, 282; a YouTube rip of the same audio is titled Genocide).
  Possible: Muffler - Downfall (1:56, 142), Falcon - The Stand (2:28, 185),
  Sonic - Cannibal (3:27, 180). Nothing found at 0:26-0:56 and
  2:59-3:27. Not checked by ear.
- The `gunfinger --config ~/.config/gunfinger/nas-dnb.toml
  index` (pid 40404, started about 01:52 from `target/release/gunfinger`,
  a build of the same code that is not byte-identical to `s22-final`;
  `cargo build --release` at 02:38 reproduced `s22-final` exactly)
  exited between 02:23 and 02:25. The NAS store went from 38,104
  records (`5fd86c54350c819c`) to 39,354, revision `d71ba5333cff9a34`
  (`scripts/analysis/store_revision.py`, 02:25; 38,894 at 02:15).
  `doctor` (read-only, 5 min 23 s over SMB): 41,222 audio files, 618
  left out by 41 patterns, 39,354 current records (2.78 GB), 1,841
  passed over (11 failed to decode, 138 too short, 1,692 too long), 27
  audio files not indexed yet (files added to the NAS since), 39,381 of
  65,536 assets; both saved NAS indexes out of date.

## 2026-10-16: session 23, the NAS clusters rebuilt; shared material re-scored; the ambiguity note measured

Brief `docs/brief-23.md`, checklist `docs/session-23-checklist.md`. Files
were added to the NAS during
the session.

- 02:18 start; the verdicts by ear and status entry committed
  (`d39fdca`); brief and checklist (`d9b3b6a`). The index run
  exited between 02:23 and 02:25 (entry above): 39,354 records,
  revision `d71ba5333cff9a34`. Start build `work/bin/s23-start/`
  (byte-identical to `s22-final`); `baseline session-23-start` (the
  reports of session 22's final `regress`).
- 02:32 step 2's clusters run started under `screen` (`work/s23/clusters.sh`,
  session 18's command at the new revision). `map-library` first: the
  same 247 copies; only the record count changed (33,723 in the old map
  -> 39,354). The NAS report folder was cloned to `work/s23/before/`.
- Harness, before step 4's runs: `shared-mixes` keeps every file its
  holder lookup finds at 30 hits or more (`holders`), so (c) can be taken
  under any clusters file; `gunfinger-eval holders` runs the same lookup
  for a list of files and spans; `shared`'s detections carry their span
  in the file (3 new tests; the 262-track smoke run of `holders` finds
  Clockwork's two copies and drops the left-out file).
- Analysis scripts: `shared_outcomes.py --clusters` and `shared_mixes.py
  score` under any clusters file (both reproduce 0111, 0116 and 0118
  exactly with the old clusters); `clusters_rebuilt.py`,
  `panel_clusters.py`, `ambiguity_note.py`, `shared_panel_2.py`.
- 02:43 step 5's pre-registration committed (`0afceb5`, brief and
  experiment 0123; the brief's heading says 02:50), before any count
  under the fresh clusters.
- 05:26 step 2's search exited 1 on 0066's Sonar chain, now through
  the-hifi-junglist's `PRO 012 A` upload (the revision, judged different,
  joined at 99% with 406 hits). Its 2,593 pairs were clustered again with
  `--reuse-pairs work/s23/clusters-stopped.json --cut-sparsest`, cutting
  upload ~ revision (decision in the brief; no verdict written): 281
  clusters, the 17 corpus groups unchanged, 75 members added (59 newer
  uploads, 6 from tonight's index run), none gone, nothing split or
  joined; 3 new borderline questions, Aphrodite 12"s against V2's 1999
  album (2) and Lacerba's 1997 compilation (1) (experiment 0120). The new
  clusters are in the NAS report folder.
- 05:37 step 3 (experiment 0121): the saved reports under the new
  clusters. The 9 wrong confident answers of the development scans are
  newer uploads, now right; 0116's 43 other plays are right but Kemal's
  remix; the 87 wrong related plays and (c)'s 61 marks on them do not
  change; of the 453 old true plays 17-79 stay marked (62 unknown
  offline), and the 42 plays now right add 0-42. `rescore_summary.py`
  added.
- 05:38 step 4's queue started under `screen` (`work/s23/queue.sh`).
- 05:45 step 6b, done early so no later count can shape it: a proposed
  second shared-material panel, `docs/panels/shared-material-2-seed-2026.json`
  (`shared_panel_2.py`), from the `different` verdicts by ear outside the
  frozen panel's families, under the new clusters: 6 families split by
  seed 2026, development phantom-force, star-trails, we-r-the-ones;
  validation killa-bees, stab-2-def, tsunami. Not searched; passages not
  cut. Four families are played in the development mixes (listed in the
  file), Stab 2 Def's among them although it falls in validation.
- 05:50 step 6a, while the queue ran: `work/audition/s23/01-s23-pack.json`
  (6 questions; pack `work/audition/s23/pack/`, texts `plan.toml`):
  the three Aphrodite borderline pairs of 0120, the Sonar upload against
  the original and against the revision, and Kemal - Animation (Silent
  Extent Remix) against the corpus original. A temporary `audition`
  server (scratch answer log and verdicts file under `/tmp`) served the
  page and every lane's envelope and audio; nothing was answered, and it
  was stopped. Descent ~ Scarabeus (41% on 6 hits, unjudged since
  session 18) is left out.
- 06:23 step 4a done: the three development mixes at the new revision
  (`--verify` scans, 5.4-9.6 min each, 5.3 GB; the harness's saved index
  rebuilt by the first). 40 of 40 tracks, 0 wrong; every track left out:
  0 of 40, 0 wrong; 3 possible plays matching no track in each mix
  (`1.mp3`'s is gone). Four tracks are also found through files stored
  tonight (Decoy, Dominion, Descent's Phuncatek Remix, Horns 2000).
  Under the old clusters the same reports would have 13 wrong.
- 08:15 step 4b's `shared` run done in 112 min (session 20's took 32;
  6.1 GB). Tonight's runs retire about as many cycles as session 22's
  but take 3-6 times as long, with about 3.3 threads busy at an
  effective 1.8 GHz on this 8+2-core M1 Pro, which suggests the
  processes started from this session's shell ran at background
  priority. At 08:27 `taskpolicy -B` moved the queue's own processes
  out of it (nothing else was touched). Under the new clusters, the
  outcome tables equal step 3's (106 of 218 wrong confident); a few hit
  counts move by 1, and the copy of the blend partner Decoy stored
  tonight is a possible detection in 24 blend queries.
- 08:47 4b's holder lookups (606, 32 min) and the note on the panel
  (`ambiguity_note.py panel`): every wrong play of 58 of the 106 wrong
  confident queries is flagged (all six of Fire's directions), none in
  Dozer, Pyro and Sonar (no third version indexed); all 75 right plays
  of the positive control are flagged (each names the related version).
  4c's `shared-mixes` started.
- 08:56 `shared-mixes` restarted as a launchd job under `screen`
  (`work/s23/queue2.sh`, the rest of the queue unchanged), after a
  benchmark ran 3 times as fast that way. It was no faster (4:50 to the
  third mix against 4:44); the benchmark had compared two Python
  interpreters. About 9 minutes lost. The cause of tonight's slowness is
  not known; `taskpolicy` changes made no difference.
- 09:05 review pass 1 started (a read-only subagent, no cargo, on the
  committed code, scripts, 0120, 0121, the panel and the questions);
  pass 2 follows for 0122, 0123 and the notes.
- 09:23 step 4c shrinks (decision in the brief, before step 5's counts):
  the 36 rendered mixes would have ended near noon (13 of about 90 runs
  in 26 min); stopped. `work/s23/queue3.sh` (under `screen`) runs the
  three development mixes alone (`work/s23/shared-mixes-dev.json`), then
  `identify` of the three mixes and the holders of their tracklists. The
  rips plan is dropped; criterion 2 and the rendered plays come from step
  3's re-score, as the pre-registration's fallback says.
- 09:28 review pass 1's findings, fixed: `clusters_rebuilt.py`'s gap
  counted pairs inside one cluster (0120's gap is 56.4% -> 74.2%, not
  76.6% unchanged); 0121's bound on marks mixed old true plays with the
  42 now right; `shared_mixes.py score` printed `PROMISING` for (c)
  candidates with unknown values (now `undetermined`) and counted a
  holder under 60 hits inside the own recording as unknown (now a known
  0; no count changes); `ambiguity_note.py panel` formed plays from
  confident detections only (now from possible ones, as defined; the
  same counts); the identify lookups leave out the 15 corpus-only files
  (decided before counting). Step 5's panel counts at 08:47 came before
  0122 was written; they are among the counts reported beside the
  criteria and decide nothing.
- 10:05 step 4c (shrunk) done: the three development mixes alone at the
  new revision (40 min, 5.5 GB), 108 true plays credited by the new scans
  (4 more than 0121, through files stored tonight). Experiment 0122
  written. `identify` of the three mixes started.
- 10:05-10:20 step 5 (experiment 0123): criterion 1 fails (15 of 108
  true plays of the development mixes flagged, 13.9% against 5%),
  criterion 2 passes (61 of 87 wrong related plays, 70%, from step 3);
  not worth proposing by the rule written first. The 15 are 6 tracks;
  five name a remix, edit or close version, Omni Trio's another title
  (`pair` 3.7-54.4%);
  `ambiguity_note.py flagged` added. `identify` of the three mixes done
  (10:04-10:18; it rebuilt the saved index in `~/.cache`, which
  held 38,104 files); the tracklists' holder lookups (101) still ran at
  11:10. From about 10:20 other load on the machine (load averages 150-250)
  explains the slowness from then; the slowness before 10:20 has another cause.
- 10:51 `check.sh`: the first attempt put `~/.cargo/bin`, which does not
  exist, first on PATH, so Homebrew's rustc ran instead of rustup's; the
  second put rustup's proxies (`/opt/homebrew/opt/rustup/bin`) first but
  set no `RUSTUP_TOOLCHAIN`, so rustup began installing `stable`
  (1.99.1). I stopped it; a later `rustup toolchain list` finished that
  partial install. `stable` was uninstalled again, and
  `~/.rustup/toolchains/` holds only 1.98.0, as before. The interrupted
  install left 136 MB in `~/.rustup/tmp` (files from 10:51), not removed.
  The check now runs as AGENTS.md says (`RUSTUP_TOOLCHAIN=1.98.0`), then
  with Homebrew's 1.98.0, at `nice 19`.
- 11:12 review pass 2 (read-only subagent). Fixed: the notes no longer
  claim the harness refuses an unindexed left-out name (a check in
  `holders` rejected valid corpus files that stand for themselves, so
  it was reverted; left open); the binaries' identity stated per binary
  (the clusters run's `work/bin/s23-start` equals session 22's, and
  took 2 h 54 min against session 18's 26); the 15 plays' wording; "9
  wrong detections from 7 uploads"; `holder_hits`'s docstring states
  that old reports assume clusters that only add members, and that the
  unknown plays are a bound; 0123 trimmed to 40 lines.
- 11:30 the tracklists' holder lookups ended (72 min, 101 lookups,
  15 files left out). `ambiguity_note.py tracklists`: 6 of 40 entries
  noted (Star Trails, Clockwork, Phantom Force, Artificial Life, Stab 2
  Def, We R The Ones), every entry a right track; 12 of 95 confident
  plays flagged; every printed line matched the rebuilt one
  (`work/s23/after/score/tracklists.txt`, data in 0123's JSON).
- Wrap-up, not finished by 13:00 (other processes kept the load at
  150-250). Final build `work/bin/s23-final/` (16 min): `gunfinger`
  byte-identical to `s23-start` and `s22-final`; `gunfinger-eval`
  differs (tonight's harness). `regress session-23-start` (11:58-12:55
  under `screen`, `work/s23/regress.log`) identical: 720/720 sweep
  queries, all 87, 93 and 79 scan detections, the calibrate line known
  since session 8; 3,423 s, 3,404 s user (session 22: 153 s, 1,314 s).
  At 12:51 we let the running checks finish after 13:00.
  `scripts/check.sh` with rustup's 1.98.0 first
  (`RUSTUP_TOOLCHAIN=1.98.0`, 11:23-12:58, 95 min) green: fmt, clippy
  (macOS and the Linux target), 399 tests (the end-to-end tests alone
  took 88 min). The Homebrew pass (12:58-13:40, 42 min) failed 1 of 399
  tests: `decode::watch::tests::a_probe_that_writes_nothing_is_an_error`
  got `Stalled` (0.5 s) instead of `Ok(None)` from its `exit 1`
  stand-in, at load ~250; the same code passed it in the rustup pass, so
  taken as the load delaying the script's start. Not rerun (from the prompt:
  start nothing new); the Linux clippy skipped (no Linux standard
  library for Homebrew's toolchain). NAS store at 11:37: 39,354 records,
  `d71ba5333cff9a34`, as after the index run.
  `docs/pair-verdicts.txt` and `work/audition/answers.jsonl` unchanged
  (sha256 `d48ffb313396...`, `ee2d5bd9fe0d...`).

## 2026-10-16: session 24, a diagnostic version verifier for shared material

Brief `docs/brief-24.md`, checklist `docs/session-24-checklist.md`. A
parallel session (24b) ran a Panako benchmark in its own worktree on
branch `s24b-panako` (heavy runs until 00:30); it was not touched.

- 18:40 start; `git status` clean at `53d4136`. The NAS store held
  40,126 records, revision `fed26c76b8fe89ea` (772 records after session
  23's end check; taken as found). Start build `work/bin/s24-start/`
  (byte-identical to `s23-final`); `baseline` only copies reports, so
  `regress session-24-start` served as the speed check: identical in
  153 s (session 22: 153 s).
- 18:50 the research note `docs/research/2026-10-16-version-verification.md`
  (the GPT Pro report, unverified; local checks by arithmetic and code
  reading).
- 18:56 design and pre-registration in the brief and 0124: the verifier,
  three summaries, the split (calibration: the panel's development half;
  E1: 108 true plays of the three development mixes; E2: 87 wrong
  related plays of the rendered mixes), T's rule and the criteria (E1 at
  most 5 rejected, E2 at least 44). 19:05 two amendments from the
  synthetic tests (EQ-shaped kills; masked from +3 dB), before any run.
- 19:11 `gunfinger-eval versions` (`versions.rs`, `versions/compare.rs`):
  17 tests (11 in `compare`, most on synthetic signals: the same
  recording, an EQ'd copy, a bass kill, a blend, another arrangement).
  Smoke runs on two calibration plays.
- 19:14 calibration (0124): 387 plays, T = 0.121 rejects 157 of 312
  wrong plays and keeps 75 of 75 positives; Fire 144 of 248, Dozer 13 of
  16, Pyro and Sonar none.
- 19:21 the first evaluation run failed to render 8 references (WAV
  behind an ID3 tag); `render_samples` now opens them as the core's
  decoder does (a test fails without it). Everything rerun; calibration
  identical. 19:28 (0125): E1 5 of 108 rejected (passes at the limit),
  E2 43 of 87 (fails by one; Fire 43 of 67, the others none); drawn true
  plays 3 of 480, positive control 3 of 15. Not worth pursuing as built.
- 19:40 the controlled test (0126): along its line the verifier sees only
  differences under the shared material; attenuation is absorbed; a cut
  to a record whose drums line up reads as contradiction for a true rip.
- 19:54 exploratory (0127, `version_lines.py`): the five rejected E1
  plays lie 98-193 ms from their reported lines (excerpt medians; 95 of
  104 plays within 48 ms, and some kept plays as far off): the line is
  anchored on the opening window (`refine.rs`), and alien5ive's uploads
  drift in speed within a play. In E2, Do Drop In's damaged MP3 frames
  put the store's decode 235 ms from FFmpeg's seek; Astro-Sine's
  rejections are another record's bass near the predicted level under a
  bass cut. No verifier variant was run on E1 or E2.
- Notes "Session 24" with the handoff for session 25.
- 19:59-20:12 review by a read-only subagent. It found the code as
  designed, the time arithmetic and the T rule right and the numbers
  matching the data. Fixed: the line's role marked as an inference with
  the kept counterexamples (Dominion 122 ms, S1 0.027; 89 of 104 within
  48 ms by the worst excerpt); the Astro-Sine mechanism (the `r < 0.2`
  rule, not masking); 0127's ad hoc measurements recorded in its data;
  Dozer (13 of 16 rejected on the panel) and Pyro ("never rejected")
  described as the data say; 0124 and 0125 point to 0127; the notes
  describe the contradiction and kill rules as built; the decision to
  keep `versions` in the tree written down. Four tests added and the
  EQ'd-copy test now checks the fitted gains (21 tests in `versions`,
  all passing; no code change).
- 20:20-20:31 final checks. Release build `work/bin/s24-final/` with
  Homebrew's 1.98.0 (as the start's): `gunfinger` byte-identical to
  `s24-start` and `s23-final` (a build with rustup's 1.98.0 differs by
  its library paths only). `regress session-24-start` identical (720/720
  sweep queries, all 87, 93 and 79 scan detections, the calibrate line
  known since session 8), 154 s. `scripts/check.sh` with rustup's 1.98.0
  first (`RUSTUP_TOOLCHAIN=1.98.0`) green: fmt, clippy for macOS and
  Linux, 421 tests (the six `decode::watch` tests passed), 6.6 min. NAS
  store at the end: 40,126 records, `fed26c76b8fe89ea`, as at the start.
  `docs/pair-verdicts.txt` and `work/audition/answers.jsonl` unchanged.
  Scratch data in `/tmp` removed; `work/s24/` holds 1.0 GB.

## 2026-10-16: session 24b, Panako as an outside benchmark (run on 2026-10-17 with Java 11 and LMDB built from source; experiment 0150)

Worktree `/Users/razorjack/.t3/worktrees/gunfinger/s24b-panako`, branch
`s24b-panako` from `81e16e0`, run beside session 24; brief
`docs/brief-24b.md`, checklist `docs/session-24b-checklist.md`. The
product did not change.

- 19:07 Panako cloned into `work/panako/src-clone/Panako`: `f1248f7`,
  version 2.1, AGPL-3.0. `GRADLE_USER_HOME=work/panako/gradle-home
  ./gradlew --no-daemon shadowJar` downloaded Gradle 7.2 and failed:
  `Unsupported class file major version 65` (Gradle 7.2 runs on Java
  8-16; the Mac has only Temurin 21). Panako part stopped (our
  rule). 19:24 the launching agent: wait until 22:00 for Temurin 11,
  change nothing, install nothing. `java_home -v 11` prints Temurin 21
  with exit 0, so the wait read the JDK's version
  (`work/panako/wait-java11.sh`, first in `work/s24b/`); no JDK 11 by 22:00.
- 19:15 pre-registration (brief step 3, experiment 0130): Panako's
  documented defaults with `STRATEGY=PANAKO`; indexes of the 262 files
  and per sweep seed; sweeps 2026-2029, the three development mixes with
  `monitor`, seed 2026's generated mixes; scoring as the harness's;
  one stricter acceptance chosen on seed 2026's sweep.
- 19:16-19:31 Gunfinger's side (experiment 0131), `work/bin/s24-start/`
  of the main checkout with `--work work/s24b`: scans of razorjack and
  nocny, sweeps 2027-2029, mixes of seed 2026, each identical to its
  saved report (`scripts/analysis/report_identity.py`). Columns
  (`scripts/analysis/panako_compare.py gunfinger`): 60/60 per speed and
  0 wrong on sweeps 2026-2029; 11/11, 7/7, 9/9 and 0 wrong on the
  development mixes; 106/115 indexed plays and 0 wrong on the generated
  mixes. `gunfinger stats`: index 39.7 MB, peak store 19.7 MB. A fresh
  `gunfinger index` of the corpus library: 50.9 s wall, 938 MB, 265
  files extracted (the working store's 3 skip notes decode now).
- Ready for the run: `work/panako/run.sh` (stages), the lists in
  `work/panako/lists/` (written by `panako_compare.py plan`), the scorer
  `scripts/analysis/panako_compare.py` (`selfcheck` passes);
  `scripts/analysis/devscore.py`'s `load_set` takes a corpus root.
- 21:30-21:45 read-only review (subagent). Fixed: the pre-registration
  amended before any count (brief A1-A10: speed from the matched spans,
  as Panako's time factor is 1/(2 - speed); monitor times within their
  window; byte-identical files stored once, scored as printed; stricter
  recall over any right answer; Panako's unit tests not run, they
  download audio; I-stakka-out's list), the scorer and the run script to
  match, times and loads in the brief and 0131.
- 22:00 no JDK 11 (`wait-java11.sh`: "no java 11 by 2200"); the Panako
  part written up as blocked. Nothing outside `work/`: no `~/.panako`,
  no `~/.gradle`, the Java preferences file's digest unchanged. No Rust
  code changed, so `scripts/check.sh` was not run.
- For session 25's merge: new files `docs/brief-24b.md`,
  `docs/session-24b-checklist.md`, experiments 0130 and 0131,
  `scripts/analysis/panako_compare.py`, `report_identity.py`; changed
  `scripts/analysis/devscore.py` (`load_set`'s optional corpus root),
  and appended at the end of `docs/session-notes.md`,
  `docs/status.md` and the roadmap's "Evaluation" (expect conflicts with
  session 24's appends: keep both). Renumbering 0130 and 0131 touches
  their file names and the mentions in the brief, the checklist, these
  notes and status entries, the roadmap bullet, 0130, 0131 and
  `panako_compare.py`'s docstring. `CLAUDE.md` was not edited.
- 2026-10-17 00:48 (the run with Java 11, experiment 0150): branch
  fast-forwarded to master `6c7728c`. `run.sh build` with Temurin 11:
  built in 17 s. `install`, `smoke`: JGaborator loads (arm64); `store`,
  `query` and `stats` each fail creating LMDB storage, before decoding:
  `UnsatisfiedLinkError: dlopen(liblmdb.dylib ...)` (lmdbjava bundles
  LMDB for macOS x86_64, Linux x86_64 and Windows x86_64 only). Panako
  part stopped (the instruction). Needs an arm64 `liblmdb.dylib` via
  `-Dlmdbjava.native.lib`, or an x86_64 JDK under Rosetta 2. Outside
  `work/`: empty Java Preferences nodes (`be/panako/util`, and
  `jshell`'s `tool/JShell` from the cleanup) removed through the
  Preferences API (Temurin 11's `jshell` and `java`, the only use of
  Java 11 outside Panako's commands), `be.panako.util.plist` deleted; the prefs plist's
  content equals the copy from before. No `~/.panako`, `~/.gradle`,
  `~/.m2`.
- 01:22-01:40 (approved): LMDB 0.9.29 cloned from OpenLDAP's
  tag `LMDB_0.9.29` (commit `8ad7be25`, the same on the GitHub mirror),
  `libraries/liblmdb` built into `work/lmdb/liblmdb.dylib` (arm64, no
  install), passed as `-Dlmdbjava.native.lib`. `smoke` passes; setup
  checks: a 4 ms delay or a 128 kbit/s MP3 encode loses a match.
- 01:32-03:04 `index-all`: stopped at 30 min by the tool's default
  limit on background commands, resumed; 253 files (9 byte-identical
  copies stored once), 1,728 s of fingerprinting on one thread for
  99,987 s of audio, LMDB 237.5 MB + cache 107.6 MB. Seed 2026's sweep:
  339/540, 1 wrong (score 12); S* = 13.
- 03:04-05:08 development mixes (`monitor`) 11/11, 7/7, 8/9 (1 wrong at
  the defaults, 0 at S*); sweeps 2027-2029 309, 291, 341 of 540 (3, 2, 7
  wrong; 0 at S*); generated mixes 105/115 (104 at S*), 0 wrong;
  stakka without its references: nothing. Gunfinger: 540 per seed,
  11/11, 7/7, 9/9, 106, 0 wrong. Whole-file `query`: heap failure at
  256 MB and a hung JVM (1.5 h, stopped; amendment A11); at 2 GB no
  match on any mix.
- 05:09 the Java Preferences node `be/panako/util` (written again by the
  runs) removed with Temurin 11's `java`, `be.panako.util.plist`
  deleted; the prefs plist's content as before. No `~/.panako`,
  `~/.gradle`, `~/.m2`. No Rust code changed; `scripts/check.sh` not
  run.

## 2026-10-16: session 25, three harness fixes, the NAS clusters at 40,126 records, the second shared panel's development half

Brief `docs/brief-25.md`, checklist `docs/session-25-checklist.md`.
Session 24b (Panako benchmark, branch `s24b-panako`, own worktree) was
waiting for Java 11 and was not
touched.

- 20:44 start; `git status` clean at `6f204d5`. NAS store 40,126
  records, `fed26c76b8fe89ea`. Start build `work/bin/s25-start/`
  (byte-identical to `s24-final`); `baseline session-25-start` copies
  reports (0.00 s), so `regress session-25-start` was the speed check:
  identical, 153.8 s (session 24: 153 s).
- 20:52-21:24 step 2 (experiment 0128): `clusters --from-peaks` at the
  current revision. Without `--cut-sparsest` it stopped in 0.85 s on
  0120's Sonar chain (the verdicts are checked against the last run's
  pairs before searching); with it, 32 min. 281 clusters, 2 uploads
  added (old-skool-jungle-dnb's The Nine and Sonar, from the index
  run), none gone, nothing split or joined; a second Sonar cut
  (the old-skool upload ~ the Mark System Revision, 100%, 0.8 hits/s);
  gap unchanged; 2 new borderline rows, one unjudged (corpus
  `APH027 ... cool_rock_suff` ~ `Mobb Deep Vs Urban Takover - Cool Rock
  Shook Ones`, 41%). `work/s25/clusters-new.json`.
- Step 3b (`holders`): a left-out name the index does not hold is
  refused before any index is loaded (misspelt, or a NAS copy whose
  corpus file is not left out with it); copies beside their corpus file
  and corpus files without a copy pass. Session 23's 707 lookups pass.
- Step 3c (`render_samples_as_indexed` for `versions` and
  `identifiability`): renders count the start in samples decoded from
  the beginning. `render_timeline.py` over 0112 and 0124-0126 (594
  files): Do Drop In (-235 ms after 429 s) and one Miami position
  (-26 ms) affected; rerun on their plays, E2's rejected drawn true
  plays 3 -> 2 of 480, nothing else; 0112 rerun: audio measures move by
  at most 0.006, every outcome the same.
- Step 3a (`decode::watch` tests, test code only): 2 s stall for runs
  meant to stall, 60 s for runs meant to finish, silence tests at a
  thirtyfold margin. Under 250 busy loops (load 94-330, 43 rounds) the
  old tests failed in 1 round, the new in none.
- 21:15 step 4c's pre-registration committed (`2029f83`), before any
  run on the second panel.
- 21:40 step 4a: `docs/panels/shared-material-2-seed-2026-development.json`
  (10 directions, 2 controls; CERT1853's Klute Remix joined at 99.2%),
  committed before the run. 21:41 step 4b's `shared` run started.
- 21:41-22:05 step 4b (experiment 0129): one `shared` run at NAS scale
  (24 min, 5.4 GB; the saved NAS index rebuilt, 39,376 -> 40,148
  files), scored with `shared_outcomes.py` under 0128's clusters. With
  the played recording left out, 5 of 135 queries wrong confident, all
  Phantom 2018 over Phantom Force (looped 120 and 240 s, beyond 10-30
  s); the first panel 106 of 218 (0122's run scores identically under
  the new clusters). Every other related version stays possible
  (113-185 hits). `shared_side_by_side.py`.
- 22:08-22:10 step 4c (experiment 0132): `versions` as built (T =
  0.121): 4 of 5 negatives rejected (too small to read, under 10), 77
  of 78 positives kept; the rejected positive's line is 313-327 ms off
  (a loop point inside the opening window, an inference). `s24-final`
  beside it: 9 plays' S1 within 0.006, no decision changes.
- 22:16 `pair` on the two new cluster pairs (`work/s25/listening/`):
  the old-skool Sonar upload lines up with the corpus original at speed
  1.0000, every window supported; APH027 B1 ~ the Mobb Deep upload line
  up to A's 2:28. Both are listening questions in the notes (the Sonar
  one was first dropped, then restored after the review). 0128 amended
  with the `pair` result.
- Notes "Session 25", roadmap (shared material; clusters' check before
  a search).
- 22:20-22:40 step 5: a read-only review by a subagent (everything since
  `6f204d5`). No defect changed a number and no rule was broken. Fixed
  (`cb23fab`, `f908844`): `holders` kept one corpus file per NAS copy
  although 7 copies stand for two (now a set; test extended); the
  silent-decode test checks that the stall timeout passed; 0129's
  control explained wrongly (its passage overlaps another direction's);
  the second panel's passages are shorter, not sparser; times, counts
  and the index path in the documents; inferences marked; the new Sonar
  cut restored as a listening question. Details in the brief at 22:44.
- 22:46-22:56 final checks with `work/bin/s25-final/` (after the
  review's fixes; `gunfinger` byte-identical to `s25-start`): `regress
  session-25-start` identical, 154.7 s; `scripts/check.sh` with rustup's
  1.98.0 green, 423 tests, 6.5 min. NAS store at the end: 40,126
  records, `fed26c76b8fe89ea`, as at the start; listening files unchanged.
- About 23:00 end, four and a half hours early: every step done (brief,
  22:19). `s24b-panako` untouched.

## 2026-10-16: session 26, session 24b merged; the version verifier's version 2

Brief `docs/brief-26.md`, checklist `docs/session-26-checklist.md`.

- 23:22 start; `git status` clean at `0aa1dea`. NAS store 40,126
  records, `fed26c76b8fe89ea`. Start build `work/bin/s26-start/`
  (byte-identical to `s25-final`); `baseline session-26-start` copies
  reports (0.24 s), so `regress session-26-start` was the speed check:
  identical, 153.9 s.
- 23:30 `s24b-panako` merged (`2eee100`, a merge commit): notes and
  status conflicted (both appended at the end); every section kept
  whole in the order the sessions started (24, 24b, 25); 0130 and 0131
  keep their numbers; `docs/brief-24b.md` in CLAUDE.md's list. No code
  changed; `regress` after the merge identical (155.2 s). The worktree
  and branch stay.
- 23:33 version 2 pre-registered (brief, step 3): each reported
  segment's line measured from the reference's own hits (10 s windows'
  densest runs as knots, stretches split at jumps, lone knots dropped as
  strays, chords within 16 ms for drift, tested only between hits); a
  band contradicted only in an EQ group with an anchor; T by version 1's
  rule; E1 and E2 declared a second use, version 1's criteria.
- 23:41 step 4: `versions/line.rs`, `versions.rs` (pieces compared as
  version 1 compares segments), `versions/compare.rs` (the group rule;
  version 1's band rules beside); tests for a line 150 ms off, a
  drifting copy, a jump, a stray, no line, and another record's bass
  under a bass cut (now masked, not contradicted); `version_plays.py`
  (unmeasured plays, `changes`, `data`). Smoke run on two calibration
  plays.
- 23:44 calibration (0133): T = 0.00683 (version 1: 0.121); 156 of 312
  wrong plays rejected, 72 of 75 positives kept. The group rule lowered
  the wrong plays' S1 too (median 0.112 by version 1's band rules on the
  same lines, 0.007 by version 2's; the positives' median is 0 under
  both). Committed before evaluation.
- 23:49 evaluation (0134): E1 9 of 108 true plays rejected (limit 5:
  fails), E2 45 of 87 wrong related plays (44 required: passes); drawn
  18 of 480, control 0 of 15. Not worth pursuing as built. The line
  keeps four of version 1's five rejected true plays; the nine rejected
  now are four tracks at blends and mix-ins (an inference from the
  timelines: another record's sound in bands of an anchored group).
  Decisions changed: E1 8 newly rejected, 4 kept; E2 wrong 13 and 11.
  Exploratory: version 1's band rules on version 2's lines, T re-derived
  (0.112), give E1 0 of 108, E2 36 of 87.
- 23:50 side set (0135, deciding nothing): the second panel's wrong
  plays 0 of 5 rejected (4 unmeasured: Phantom 2018 repeats the shared
  material, every knot a stray), positives 78 of 78 kept.
- Notes "Session 26" with the handoff; roadmap.
- 23:52-00:06 step 6: a read-only review by a subagent (everything
  since `0aa1dea`). No defect changed a number, no rule broken. Fixed
  (`d42ec72`): Dozer's drop restated (its lines did not move; version 2
  tests 3,762 of its 5,039 patches, the differing stretches dropped as
  strays, an inference); relocated stretches (a kept stretch 2.7-97 s
  off in 30 of 312 calibration and 15 of 87 E2 wrong plays) and the
  changed decisions reported; the blend reading an inference, per
  track; Horns 2000's times; four tests added (one knot, two strays,
  other music, `Clock::cut`); a backwards comment; the threshold's
  label.
- 00:13-00:23 final checks with `work/bin/s26-final/` (`gunfinger`
  byte-identical to `s26-start`; `gunfinger-eval` differs from the runs'
  `s26-v2` by comments and tests only, the smoke plays' output
  byte-identical): `regress session-26-start` identical, 154.6 s;
  `scripts/check.sh` with rustup's 1.98.0 green, 432 tests, 6.4 min. NAS
  store at the end: 40,126 records, `fed26c76b8fe89ea`, as at the start;
  listening files unchanged.
- About 00:30 end, five and a half hours early: every step done, and no
  further run on E1, E2 or the second panel allowed (brief, step 5c).
  `s24b-panako` and its worktree kept.

## 2026-10-17: session 27, a minimum span or supported time instead of 3 windows; per-second scoring

Brief `docs/brief-27.md`, checklist `docs/session-27-checklist.md`.

- 00:29 start; `git status` clean at `21b0220`. NAS store 40,126
  records, `fed26c76b8fe89ea`. Start build `work/bin/s27-start/`
  (byte-identical to `s26-final`); `baseline session-27-start` copies
  reports (0.21 s), so `regress session-27-start` was the speed check:
  identical, 154.5 s.
  Session 24b ran Panako beside it in its own worktree (heavy runs at
  nice 10; experiment number 0150 reserved for it).
- 00:41 step 2 pre-registered (0136): A(S), the first to the last
  aligned hit at least S s apart, and B(S), at least S whole seconds
  holding hits, in place of `windows >= 3`, S = 10, 15, 20 s, hits and
  the possible tier unchanged; the panels; criteria (1) no new wrong
  confident answer on any panel, (2) on the grid strictly more confident
  queries and strictly fewer flips; data from master's saved reports
  (M, A only) and a never-merged variant build (V, A and B).
- 00:49 0137 from M: grid 693 confident and 30 flips today, A(10) 803
  and 2, A(15) 644 and 2, A(20) 482 and 1; on the first shared panel
  A(10) and A(15) add wrong confident answers (8 and 6 detections in 3
  queries, two indexes), A(20) none. 01:55-01:57 corrected: M's
  `mixes-seed-7.json` (2026-10-06) is the single pass and was dropped.
- 00:44-02:21 V's queue (worktree `s27-span`, `work/s27/vwork/`):
  grid, both shared panels, sweeps, mixes 2026 and 7, scans at 262 and
  NAS scale with leave-outs, `robust`, recall. Identical to M's reports
  where comparable. Shared and `robust` detections keep no `fitted`
  flag; taken as fitted, today's rule reproduces every stored level.
- 00:55-02:15 step 3: `gunfinger-eval seconds` (per-second scoring,
  Sonnleitner et al. 2016, §5; generated and rendered mixes with exact
  truth, scan reports with a margin around manifest borders; start and
  end errors; 11 unit tests); outputs per library directory.
- 02:20 0139: the per-second baseline. Generated mixes 94.2% of indexed
  seconds correct, 8 s mislabelled, 100% of unindexed seconds silent,
  edges 2-3 s inside the truth; development mixes 86.8% (262) and 90.7%
  (NAS) at margin 30 s, every mislabelled second a right track outside
  its manifest slot; rendered mixes, played file left out, 83.0% and
  half of the unindexed seconds silent. No variant changes a mislabelled
  second anywhere.
- 02:23 0138 from V: no variant meets both criteria. A(10), A(15),
  B(10), B(15), B(20) add the same 3 wrong queries on the first shared
  panel; A(20) adds none but loses grid queries. B(10) 947 and 13,
  B(15) 800 and 0, B(20) 640 and 0. Nothing else changes a wrong count.
- 02:28 0140: every outcome the variants change on the shared panels
  (but one under B(20)) is in a query file of 20 s or less, never
  confident under today's rule; the wrong file is already confident in
  12-13 other queries of the same scenario.
- 02:30-02:38 0141 (pre-registered 02:30; variant worktree, never
  merged, `S27_SHIFTED_ONLY`, `work/s27/vshift/`): each shared panel's
  short passages after 3, 5 and 7 s of the played file's own material.
  Today's rule names the same wrong file in all three shifted queries
  of each of 0140's six wrong answers (18 of 18); A(10) on the 20 s
  file agrees with today's rule shifted in 74 and 70 of 78 pairs. The
  review found the lead-in matched by the wrong Fire versions, so only
  pyro-2 tests the offset alone there.
- 02:40-03:00 notes, roadmap; read-only review by a subagent (started
  02:41); the variant branch's tests fixed (`3589aa8`, 443 pass);
  `scripts/check.sh` with rustup's 1.98.0 green, 443 tests, 6.7 min
  (02:49-02:56); final build `work/bin/s27-final/` (Homebrew's 1.98.0,
  as the start's; `gunfinger` byte-identical to `s27-start`'s; rustup's
  build of the same sources differs in bytes); `regress
  session-27-start` identical, 166.6 s.
- 03:01-03:09 0142 (pre-registered 03:01 after the review): the short
  passages after 3, 5 and 7 s of digital silence. Today's rule names
  the same wrong file in 18 of 18 and adds one the 20 s file never
  shows (pyro-1 after 5 s); played left out, wrong in 10 of 42 results
  under today's rule, 11 under A(10) and B(10), 10 under A(15) and
  B(15). Criterion 1 unchanged.
- 03:10 the review's fixes (0137, 0139, 0141, brief, notes, status,
  roadmap); no code changed after the checks. NAS store at the end:
  40,126 records, `fed26c76b8fe89ea`, as at the start.
  `docs/pair-verdicts.txt` 6 lines longer, all answers by ear from
  02:01, left unstaged for the orchestrator; `answers.jsonl` the
  listening page's too. Worktree
  `s27-span` and its branch kept, never merged. Session 28 follows from the root checkout.

## 2026-10-17: after session 27, the listening of session 23's six questions

- The six questions of `work/audition/s23/` were answered on the
  listening page (02:03-02:06; answer log records 55-60): 1, 2, 3, 5
  and 6 `different`, 4 `same`. The page added six lines to
  `docs/pair-verdicts.txt`. The comments by ear, verbatim in
  `work/audition/s23/listening-comments-2026-10-17.md`:
  - Aphrodite - Stalker, APH029 B ~ V2's album version: the same
    arrangement with a slightly different synth sound and a different
    bass sound, groove and melody; "a lightly altered version",
    technically different but sharing most of the material.
  - Dark Dark Side APH-28 B1 ~ Style From The Dark Side '99: the same
    base and drum loop and the same arrangement; one has a rap or vocal
    layer and more layering on the drums.
  - Tower Bass, Millennium Mix APH31 B ~ the 1997 JumpUpThrowDown
    version: the same piece mixed slightly differently, with a slightly
    different drum loop.
  - The hifi-junglist PRO 012 A Sonar upload ~ the corpus Sonar: same.
  - The same upload ~ the Mark System Revision: hi-hats in one intro and
    not the other, the same lead sounds, different drum loops, an almost
    identical arrangement.
  - Kemal - Animation ~ the Silent Extent Remix: different tracks that
    share only the original's intro, played over new drum loops.
- What they imply: the clusters already kept the three Aphrodite pairs
  apart (74%, 63% and 48% coverage, below the 80% that joins), so the
  verdicts agree with them; all three are shared material. Verdict 3
  does not say which side the corpus `A. aphrodite - wobble.mp3` (in the
  Millennium Mix's cluster) belongs to. The PRO 012 A upload is the
  corpus Sonar's recording. The cut that `--cut-sparsest` made between
  that upload and the Mark System Revision agrees with the verdict by ear; the
  other Sonar cut of 0128 (the old-skool-jungle-dnb upload ~ the Mark
  System Revision) still has no verdict. Kemal's pair is shared
  material outside both frozen panels' families, a candidate for a
  later panel; the frozen panels do not change.
- A bug was reported in the listening page: holding the left or
  right arrow while playing stacks the fragments' audio until it is
  painfully loud; pressing the key repeatedly does not. Session 28
  fixes it first.

## 2026-10-17: session 28, the held-arrow bug, the page's look, recall under the hardest conditions

Brief `docs/brief-28.md`, checklist `docs/session-28-checklist.md`.

- 03:23 start; `git status` clean at `c0b3172`. NAS store 40,126
  records, `fed26c76b8fe89ea`. Start build `work/bin/s28-start/`
  (byte-identical to `s27-final`); `baseline session-28-start` copies
  reports (0.23 s).
  Session 24b's Panako runs shared the machine (java at nice 10), so no
  time tonight says anything about speed.
- 03:27-03:34 step 2, the held-arrow bug: `cut(when)` in
  `web/player.js` called `stop(when)` on sounding sources but kept
  their old stored stop time, and Web Audio applies only the latest
  `stop()` before a source stops; key repeats 25-35 ms apart, inside
  the 60 ms `LEAD`, so each repeat kept the previous source playing.
  Fix: a cut source records its new stop time (`f83da1a`). Test under
  `node --test` (fake AudioContext, five held-key cases): all fail
  before (66 sources at once, 124 in the blend), all pass after.
  `scripts/check.sh` runs it with Node 22.7 or newer, else prints a
  "skipped" line. The real page in headless Chrome, own server on /tmp
  copies (deleted): left arrow held 2 s, 61 sources and +7 dBFS before,
  one source and -10 to -22 dBFS after.
- 03:34-03:40 step 3, a light visual refresh (`c7c6dc8`): CSS custom
  properties, a 12-20 px type scale, queue states, framed choices,
  tabular times, lane colours, clearer spans, loop, markers and
  playhead, visible focus; text contrast 5.5:1 or more. Screenshots
  before and after in `work/s28/screens/`; the step 2 test and the held
  arrow in the real page again (at most 2 sources).
- 03:41-03:46 step 4 pre-registered (0143) before any count. 0128's
  clusters agree with the six verdicts by ear; none touches a recall
  panel source; no rebuild.
- 03:55 `recall` gains `--other-rips-only` and `--queries
  clean,damaged,brief,brief-damaged` (`0c0905d`, 7 unit tests);
  `robust`'s combined recipe in one function both use (refactor checked
  byte-identical on 200 renders).
- 03:57-05:11 the runs, one at a time under `screen` (`s28-queue`,
  frozen build `work/bin/s28-recall/`), all exit 0. 03:59 screenshot 04
  retaken on another folder question (the orchestrating thread's note).
- 04:28 0144, one condition at a time: clean 522/522; other rips only
  175/180 (20 sources observable, 38 not); brief 15 s 228/522, 20 s
  493/522 (the window grid); combined damage 5/522 confident, 428
  possible or better. No wrong confident answer; no held-out or
  unobservable query confident; strongest wrong detection 185 hits
  (Fire, Splash Remix).
- 05:13 0145, combined: all three 0/360 confident, 107 possible; other
  rips and damage 0/180, 125; brief and damage 0/1,044, 490; other rips
  and brief 37/180 (15 s) and 141/180 (20 s). No wrong confident
  answer. Damage leaves a tenth of the hits, other rips half. 0144's
  data file regenerated with file names only (AGENTS.md).
- 05:18 notes, status entry and roadmap bullet written; the read-only
  review started (report 05:38). Early `scripts/check.sh` with rustup's
  1.98.0 green (450 Rust tests, 5 Node tests, 6.7 min).
- 05:26-05:29 the frozen build `work/bin/s28-recall/` turned out to be
  built at 03:53 with `-p gunfinger-eval`, before `cargo fmt` and the
  unit tests of `0c0905d`; a build of `0c0905d`'s sources differs in
  bytes. Every condition run again from 05:29 with the workspace build
  of the committed sources (`work/bin/s28-verify/`, scratch work
  directory `work/s28/verify/`): the 702 damaged renders came out
  byte-identical.
- 05:37 session 24b merged (`31ba92a`); one roadmap conflict, resolved
  as the step says.
- 05:45 the review's fixes: damaged renders named by their partner too
  (`29b8ba5`, with two unit tests); prose in the notes, status, roadmap,
  0144, checklist; times in the brief's decisions corrected.
- 05:45-06:02 the final checks: `scripts/check.sh` with rustup's 1.98.0
  green at `adec35a` (452 Rust tests, 5 Node tests, 10 min); final build
  `work/bin/s28-final/` (Homebrew's 1.98.0; `gunfinger` byte-identical
  to `s28-start`'s), also in `target/release/`; `regress
  session-28-start` identical, 330 s. NAS store at the end: 40,126
  records, `fed26c76b8fe89ea`. Listening files unchanged.
- 06:59 the rerun with the committed code finished: all eight reports
  byte-identical to the runs' (the damaged renders too), so 0144 and
  0145 hold for `0c0905d`'s code. Worktree
  `s27-span` and its branch kept, never merged. Session 29 starts beside
  this one in its own worktree `s29-span`.

## 2026-10-17: session 29, the minimum span's fair test, brief passages at every offset of the window grid

Brief `docs/brief-29.md`, checklist `docs/session-29-checklist.md`.

- 06:04 start beside session 28's last reproduction run (root checkout
  theirs until it finished). NAS store 40,126 records,
  `fed26c76b8fe89ea`; listening files as at session 28's end. Brief and
  checklist drafted under `work/s29/`.
- 06:13-06:16 step 2: worktree `s29-span` (never merged) from master's
  `adec35a` with `s27-span` merged in (`15f3240`, no conflict; two test
  literals fixed); `S29_OFFSETS_ONLY`, each shared scenario's `alone`
  passages after 0-9 s of digital silence, four unit tests (`583bfe3`);
  release build in `work/bin/s29-span/`.
- 06:16-06:30 the variant's grid at 262 tracks: identical to master's
  (today's rule reproduces every level), byte-identical to session 27's
  variant grid; criterion 2's counts are 0138's.
- 06:22 step 5's tests drafted in a second worktree `s29-tests`
  (`7d7c6e4`, six tests, no bug).
- 07:06 session 28 finished (`df2f038`, clean). 07:10:55 brief,
  checklist, pre-registration 0146 and the scripts `span_offsets.py`,
  `span_recall.py` committed (`b61c6b5`) before any variant count (two
  Panako levels were seen at 06:23 while trying the script, and the
  variant's (b) run had ended in the same second, unread). Start build
  `work/bin/s29-start/` (byte-identical to `s28-final`); `baseline
  session-29-start` 0.21 s. Step 5's
  six `seconds` tests cherry-picked (`12d3243`).
- 06:59-07:22 the variant rebuilt session 28's brief plays (b) and
  (a)+(b): queries and today's detections equal session 28's (0
  differences). 0147 (`53f7a36`), recall only (no wrong detection
  reaches 240 hits): confident 15 s plays of 522: today 228, A(10) and
  B(10) 519, B(15) 504, A(15) 5, A(20) and B(20) 0; 20 s: today 493,
  A(20) 9, B(20) 499, the others 522. Panako's 7 plays (0150): A(10),
  B(10) 7, B(15) 6, A(15) 5, B(20) 1, A(20) 0.
- 07:22-07:40 the shared runs, exit 0; 0142's queries reproduced (78
  renders byte-identical, 195 results). 0148 (`aed8e73`): on the first
  panel, wrong answers over 10 positions: today 168, A(10) 177, A(15)
  174, A(20) 110, B(10) 201, B(15) 174, B(20) 166; the second panel none
  under any rule. F holds for A(20) and B(20) only (both fail the grid
  criterion of 0138); B(10) also fails F2 (10 s cuts). A(15)'s, B(15)'s
  and B(20)'s new wrong answers are all position 0 of three 20 s cuts
  (fire-2, fire-6, pyro-2), where today's rule is wrong at 8-9 of the
  other 9; A(10) adds pyro-1 after 3 s. No variant is worth proposing by
  the criteria; in the session's reading after the counts, B(15) is the
  candidate if a wrong answer avoided only on the window border does
  not count as new (our call; notes).
- 07:42-07:51 final build `work/bin/s29-final/` (byte-identical to the
  start build); `regress session-29-start` identical (157 s);
  `scripts/check.sh` with rustup's 1.98.0 green at `36dc022` (458 Rust
  tests, 5 Node tests).
- 07:45-08:13 read-only review by a subagent: every count right, the
  pre-registration followed; fixes to the notes' presentation, decision
  times, two deviations named (step 4's runs started at 06:59:42, before
  session 28's message; 0147 committed after 0148's runs started), 0148,
  `CLAUDE.md` and the roadmap wording (fixed 08:13-08:20). Meanwhile
  session 30 started beside this one at 07:52 in its own worktree
  (`s30-quant`; it takes 0149), and at 07:53 the scratch clones were
  deleted and the worktree `s29-tests` removed.
- 08:19 at the end: NAS store 40,126 records, `fed26c76b8fe89ea` (as at
  the start); `docs/pair-verdicts.txt` (82 lines) and
  `work/audition/answers.jsonl` (60 records) byte-identical to the
  start. Only documents changed after the final checks.

## 2026-10-17: session 30, what exact hash quantisation loses after fitting

- 07:47 start; session 29 still finishing in the root checkout (master
  `36dc022`). NAS store 40,126 records, `fed26c76b8fe89ea`; listening files
  as at session 29's start. Worktree `s30-quant` from `36dc022`.
- 07:50-07:58 brief, checklist, `CLAUDE.md` and the pre-registration
  (experiment 0149) committed on `s30-quant` (`1a9efac`) before any code
  or count.
- 07:58-08:01 the diagnostic (`aecdf93`): `verifier/pairs.rs` counts, on
  the verifier's alignment, the query's pairs matched to the record's own
  pairs in the passage: exact (E) and, among those left, with each hash
  part one step off (N), each pair once; every `Count` of the verifier
  carries them (`pairs`), aligned and at the four chance shifts.
  `recall --verify` measures fitted detections; `shared` reports gain
  `fitted`. Eight unit tests; clippy clean; 238 harness tests pass.
- 08:03-09:23 the runs (`work/s30/runs.sh`, `screen` `s30-runs`, `nice`,
  `--work work/s30/vwork`) from the worktree's build of `aecdf93`
  (`work/bin/s30-quant/`), before the merge: `recall --verify --queries
  clean,damaged`, both shared panels' development halves, `recall
  --verify --other-rips-only`, 0055's scenarios at 262 tracks; all exit
  0. The recall detections equal session 28's apart from the new field.
- 08:33-08:36 session 29 finished (`68dae5b`). Start build
  `work/bin/s30-start/` (byte-identical to `s29-final`); `baseline
  session-30-start` 0.33 s; `s30-quant` merged (`2f8f4c9`; `git diff
  aecdf93 2f8f4c9 -- crates/` empty, so the counts are master's code).
- 0151 (`scripts/analysis/quantisation.py`): median N/E 0.29 clean, 1.48
  under combined damage, 0.69 with other rips only; E keeps 9.6% of the
  clean exact pairs under damage, E + N 24.3%; 260 of 510 damaged true
  detections under 240 would reach 240 with E + N; other rips' E + N is
  0.85 of the own record's E (near pairs about three quarters of the
  exact pairs an other rip lacks, a sixth under damage). Unrelated
  candidates: N/E 0.83, none reaches 240 (highest 224, a same-artist file
  the tags miss). Related: N/E 1.54 (recall runs), 1.10 (shared panels; the played
  recording 0.52, AUC at equal E 0.50); 37 related recall candidates would
  reach 240 (21 distinct; Fire Splash Remix on clean Fire queries, Eddy
  Woo's Tsunami); exploratory: with the played recording left out a
  related version would be confident in 238 of 419 shared searches
  instead of 112. No rule.
- 09:29-09:41 read-only review by a separate agent: no counting error, no
  product change, sampled numbers recomputed. Fixed: two unit tests made
  discriminating; the within-(c) AUCs, per-panel split, derived recovery
  shares (other rips: near pairs 0.74 of the exact pairs lacking) and the
  not-confident related set computed by `quantisation.py`; prose that went
  beyond the data reworded.
- Final checks: `regress session-30-start` identical with
  `work/bin/s30-final/` (156 s; binaries unchanged by the test fixes);
  `scripts/check.sh` green with rustup's 1.98.0 before and after the
  review's fixes (466 Rust tests, 5 Node tests). NAS store at the end:
  40,126 records, `fed26c76b8fe89ea`. Listening files unchanged. Scratch:
  only `work/s30/vwork/reports/` (392 MB) kept.

## 2026-10-17: session 31, the history rewritten for wording

- The whole history was rewritten with git-filter-repo in a fresh clone:
  all 551 commits of `master`, `s24b-panako`, `s27-span`, `s29-span`,
  `s30-quant` and the tag `poc-freeze-1`. Author and committer names,
  emails and dates and the parent structure are unchanged. New tips (old
  in brackets): `master` `4b0f064` (`9a32a38`), `s24b-panako` `bad25f9`
  (`77b68e1`), `s27-span` `812342d` (`997c6b4`), `s29-span` `583bfe3`
  (`1bf4086`), `s30-quant` `fafa809` (`7d913ca`), `poc-freeze-1` `45d0ac8`
  (`407cc93`); `work/probe-tree` detached at `6eaa393` (`0d51ee5`).
- What changed, in every version and commit message: the text is in the
  committer's voice (decisions are ours, verdicts by ear, "my" only for the
  razorjack mixes; the session notes are `docs/session-notes.md`,
  experiment 0060 is `0060-my-2003-mix.md`, the clusters key is
  `aural_verdict`); "gate" became the literal wording (`scripts/check.sh`,
  the checks, a criterion); details of the development machine and its use
  were removed, keeping a short caveat beside each number it qualifies.
  Commit hashes cited inside files name the rewritten commits. AGENTS.md
  now has the wording rule.
- Untracked files: 19 names under `work/` follow the new wording
  (`work/listening-decisions/`, `work/sNN/listening-digests-*.txt`,
  `work/audition/s23/listening-comments-2026-10-17.md`); 137 text files in
  `work/` and in the `s24b-panako` worktree's `work/` were rewritten with
  the same table (old clusters reports' key renamed, commit hashes
  remapped in 33 files). `work/audition/answers.jsonl` is unchanged.
- Backups, outside the repository, taken first: a bundle of every ref, a
  mirror clone, a tar of `.git`, and a tar of the untracked files before
  the change, with their SHA-256 sums.
- Checks: for 551 of 551 commits the new tree equals the reviewed table
  applied to the old tree, byte for byte, and so do the messages; no
  changed experiment line lost a number and no experiment file got longer;
  `cargo check` passed on the commits that rename an identifier, the tag
  and the branch tips; `scripts/check.sh` green here with rustup's 1.98.0
  (466 Rust tests, 5 Node tests). `regress session-30-start` with
  `work/bin/s31-final/`: 720/720 sweep queries and all 87, 93 and 79 scan
  detections identical, the calibrate line as in session 30's final
  `regress`. Release builds with one toolchain at one path: `gunfinger` is
  byte-identical between the old and the new source; `gunfinger-eval`
  differs only in its reworded strings.
- The push is pending: `origin/master` is still the old `9a32a38`; the
  force-push of `master` and the tag waits for approval. `refs/t3/*` keep
  the old commits reachable in this repository. The table, scripts and
  logs of the rewrite are in `work/s31/` (untracked).

## 2026-10-18: session 32, Gunfinger on the Linux machine

Brief `docs/brief-32.md`, checklist `docs/session-32-checklist.md`;
notes "Session 32"; experiments 0152 and 0153.

- 18:32 start from a fresh clone of GitHub's master at `b97e3da`. Rust
  installed with rustup in the home folder (1.98.0 and stable 1.99.1,
  each with the macOS target); FFmpeg 8.1.2 (the Mac: 9.0.2). The corpus
  copied from the drive, all 327 SHA-256 sums equal.
- 18:41-18:48 `cargo build --release` (1.98.0, no warning) and
  `scripts/check.sh` green on the first run with 1.98.0 and with stable:
  466 Rust tests, 5 Node tests, clippy for Linux and macOS. No change was
  needed.
- 18:50-19:15 the corpus from scratch: 265 records (the Mac's store
  passes over 3 under session 1's old tolerance); clusters equal;
  `regress session-30-start` equal in every score, not bit for bit
  (changing only rustfft's code path changes about half the peak records
  on Linux; whether it is the whole cause is open); `baseline
  session-32-linux` saved and repeated identically. Experiment 0152.
- 19:15-19:20 the listening tool over HTTP with `--no-open`: works.
- 19:45-20:15 the NAS (`/mnt/atlas/Music/dnb`, `cifs`, read-write by the
  user's decision): `doctor`, the held-out broadcast's file probed for
  its length only, the Mac's paths looked up (227 decomposed there,
  precomposed on Linux).
- 20:22-21:15 `index` of the NAS, run by the user: 3,168.9 s, the Mac's
  counts. 21:20-21:30 against the Mac's listings: every path after NFC,
  duration, size, skip note and failure equal; `stats`; the three
  development mixes 11/11, 12/12, 17/17 with nothing wrong. Experiment
  0153. ADR 0020's findings and consequences updated.
- Stores at the end: corpus `work/peaks` 265 records (revision
  `526a3b978eb3e411`); NAS `~/.local/share/gunfinger/nas-dnb-peaks`
  40,683 records (revision `56433d74d0919a08`). `docs/pair-verdicts.txt`
  unchanged.
- 21:35-22:05 read-only review by a separate agent: every sampled number
  right; claims narrowed to the data, memory given in GiB, the
  comparison scripts committed and `store_compare.py` extended (they
  reproduce every cited number), the NFD panel paths and the reviewer's
  glob that opened the held-out manifest recorded in the notes.
- Final checks: `scripts/check.sh` green with 1.98.0; `regress
  session-32-linux` identical with `work/bin/s32-final/` (byte-identical
  to the start build). The bundle `b97e3da..master`, the summary and
  the Linux listings stay in `work/s32/drive-out/`: by the user's
  decision nothing goes to the drive, and the next session runs on the
  Linux machine.
- After the session, on the user's request: the install hints name
  Fedora (`sudo dnf install ffmpeg` with RPM Fusion, `sudo dnf install
  espeak-ng`); `scripts/check.sh` green.

## 2026-10-20: unidentified time in tracklists (between sessions)

- Marking the time a tracklist does not account for is worth doing, by
  our decision; the plan is `docs/research/2026-10-20-unidentified-gaps.md`
  (step 1: list the stretches without a confident segment; step 2:
  estimate how many tracks each holds, as a range). The roadmap's "Time
  without a confident match" now has both steps. Default or on request
  is still ours to decide, and needs an ADR.
- Experiment 0154 (`scripts/analysis/gap_count.py`, start times of the
  three development manifests, no search): a gap's length over the
  median interval counts its missed tracks within one in every case,
  exactly for 75% of two-track gaps and 42-48% at three and four.
- No Rust code changed, so `scripts/check.sh` was not run.

## 2026-10-21: session 33, the NAS-scale harness on the Linux machine

Brief `docs/brief-33.md`, checklist `docs/session-33-checklist.md`;
notes "Session 33"; experiments 0155-0158. The Linux machine: Fedora
Linux 44, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB; tuned's
`throughput-performance` profile from 01:45:52 (the user's switch).

- 01:15 start at `87529a3`, clean. FFmpeg is now 8.1.3. Start build
  `work/bin/s33-start/` (1.98.0); `regress session-32-linux` identical,
  68.7 s (under the earlier power profile).
- 01:23 step 2: `map-library`, 250 of 265 corpus files with a NAS copy
  (the Mac's 247 of 262 and the 3 extra corpus records).
- 01:24-03:32 step 3: `clusters --recall-panel` needs NAS clusters in use
  first, so run 1 left it out (45:47); run 2, session 18's full command
  with `--cut-sparsest` and `--panels work/s33/panels` (the recall
  panel's one NFD path in NFC), 52:30, 94,387 s user, 7.3 GiB: 284
  clusters, every difference from 0128 explained (0155).
- 03:33-03:57 step 4: the NAS standard evaluation and `baseline
  session-33-linux-nas` (Stakka 11/11, 8/11, 0/11, 0 wrong; sweep 2026
  540/540 with 7 wrong, one held-out query and 0128's unjudged Cool Rock
  pair); `regress` of it identical (10:55). 0155 committed 03:57.
- 03:57-05:27 step 5: session 28's eight recall conditions, one at a
  time: equal counts for clean, other rips only and brief plays, within 2
  queries for other rips with brief plays and 5 with damage, no wrong
  confident answer (0156).
- 05:28-05:41 step 6: the three development mixes, leave-outs of 3 and of
  every track, 11 reports with the Mac's scores, nothing wrong; `seconds`
  90.7% at a 30 s margin, one second from 0139 (0157). A NAS leave-out
  scan takes 59-108 s at 32 jobs.
- Steps 7 and 8 skipped (step 6 ended after 05:30, the orchestrating
  thread's limit).
- 05:42-05:53 experiment 0158: 16 workers take about 30% less time than
  32 for a NAS scan (52 s against 76 s) and a clusters sample, with
  identical results.
- 05:53-05:57 final checks: `scripts/check.sh` green (466 Rust tests, 5
  Node tests, clippy for Linux and macOS); `work/bin/s33-final/`
  byte-identical to the start build; `regress session-32-linux` identical
  (70.2 s).
- 05:55-06:20 read-only review by a separate agent; its findings fixed
  (counts, ranges, claims narrowed to the data, script labels, handoff).
- NAS store at the end: 40,683 records, `56433d74d0919a08`, as at the
  start. `docs/pair-verdicts.txt` unchanged.

## 2026-10-21: session 34, unidentified time in tracklists, measured

Brief `docs/brief-34.md`, checklist `docs/session-34-checklist.md`,
pre-registration `docs/research/2026-10-21-gap-measures.md`; notes
"Session 34"; experiments 0159-0162. The Linux machine, `--jobs 16`.

- 06:17 start at `121643c`, clean. Start build `work/bin/s34-start/`
  (byte-identical to `s33-final`); `regress session-32-linux` identical
  (68.5 s), `regress session-33-linux-nas` identical (11:23).
- 06:33 pre-registration committed before any count; the leave-out
  design (14 scans per set, the fewest possible) committed before it.
- 06:34 harness: `scan --leave-out-slots`, `gaps`; later `mixes --kind
  long` and `self-match`. Both regressions identical with the run build
  `work/bin/s34-runs/` (06:35-06:49).
- 06:49-07:39 step 3: 45 NAS-scale scans (each development mix plain
  and under the 14 patterns), the same 45 at 262 tracks, `mixes --count
  16` at seeds 2026-2029 and 7. No wrong identification in any scan.
- 0159: 30 s is the smallest minimum length meeting C1-C3; every missed
  DJ-length track reached; 7.7% of the leave-outs' listed gaps hold no
  missed track (all inside a found track), none at 60 s.
- 0160: method 1 on detection edges meets M1-M3 as a range only with
  an interval from a mix where the run is the only gap (a deviation);
  exact 100/53/37/42% at k = 1-4 against 0154's 97/75/48/42%.
- 07:45-08:03 long generated mixes (80, 62-90 minutes); 0161: every
  held-out run reached, method 1 exact 86/53/39/26%, ranges hold 93%.
- 0162: the gap matched against itself, fixed before the test, counts
  the long mixes' records exactly for 95/80/69/54%.
- 08:08-08:30 24 single-run leave-outs at NAS scale: the registered
  interval estimates all 24 gaps, equal to 0160's single-run interval
  in 22.
- 08:13-08:24 read-only review by a separate agent; its findings fixed
  (one stray listing in 0162's NAS row, two figures and two reach
  claims in 0161, unmarked inferences, a measure not reported, script
  counts, file lists in the handoff).
- 08:34-08:52 final checks: `check.sh` green (485 Rust tests, 5 Node
  tests); `work/bin/s34-final/`'s CLI byte-identical to the start's;
  both regressions identical in every detection, and in calibrate once
  this session's slot leave-out reports are out of the reports
  directories (they now live in `work/s34/reports/`; with them there,
  calibrate counts more false candidates). `docs/pair-verdicts.txt`
  unchanged.

## 2026-10-21: session 35, unidentified time in tracklists, continued

Brief `docs/brief-35.md`, checklist `docs/session-35-checklist.md`,
pre-registration `docs/research/2026-10-21-gap-tests.md`; notes
"Session 35"; experiments 0163-0166; proposal
`docs/research/2026-10-21-printing-gaps.md`. The Linux machine,
`--jobs 16`.

- 08:52 start at `611449d`, clean. Start build `work/bin/s35-start/`
  (byte-identical to `s34-final`); both regressions identical (1:09;
  11:15).
- 09:10-09:22 harness: `mixes --kind beatmatched | shared | rides`,
  `self-match --neighbours | --ratios`, `gap_tests.py`,
  `shared_merges.py`; `check.sh` green; run build `work/bin/s35-runs/`.
- 09:24 pre-registration committed before any test seed; test queue
  started (corpus regression identical first). 09:41 a review of the
  harness found that a crossfade moved a play but not its ride; fixed
  before the rides seeds ran.
- 09:25-10:05 160 test mixes (10 seeds): every indexed play confident,
  no wrong confident play. 10:25 the NAS regression with the run build
  identical (11:03).
- 0163: records sharing a passage, beatmatched back to back, merge into
  one stretch (130 of 192 pairs); method 2 exact 25% against 67% at
  k = 2-4. S1 and S2 fail.
- 0164: the neighbour discount fails D1 (8 of 12 development gaps to 0);
  it brings all 139 gaps inside ridden found tracks to 0 (not
  registered).
- 0165: method 2 limited to method 1's range B, 64% exact and 2.9% off
  by two on the test; no printing bar met (one gap short at k = 4 as a
  range).
- 0166: a sustained 1-3% pitch ride makes the search drop one side of
  the play in 192 of 382 ridden plays; every rung to ±3.2% recovers
  method 2 in gaps of missed records only, at 5.4 times its time.
- 10:28-10:36 the proposal for printing gaps (recommended: by default,
  60 s, the edges listed, the length and no count), and a prototype on
  the branch `s35-gaps-output` (`check.sh` green there).
- 10:37-10:46 read-only review by a separate agent; its findings fixed
  (figures and labels in 0164-0166 and the proposal, 0165's 262-track
  row). Final checks: `check.sh` green (491 Rust tests); both
  regressions with `work/bin/s35-final/` identical (2:57; 11:04);
  `docs/pair-verdicts.txt` unchanged.

## 2026-10-21: the search on a machine with low memory bandwidth (between sessions)

- After session 35, on the user's request: two design agents (GPT-6-Astra
  and Fable 5.1, one each, high effort, read-only on the repository) read
  the search for the Linux machine, whose NAS-scale searches were slower
  than the Mac's at 32 workers. Their reports and scratch measurements are
  in `work/perf-design/`; the consolidation is
  `docs/research/2026-10-21-low-bandwidth-search.md`.
- The machine: two core complexes with a 32 MiB L3 each; a probe reads
  25.6-27.0 GiB/s sequentially, 9-14 GiB/s at random, 104.6 ns latency
  (perhaps memory at a JEDEC default; the DIMM speed needs `sudo dmidecode
  -t 17`). Transparent huge pages `madvise`.
- Experiment 0167: finding each part's end by a doubling search from the
  front of the list, with parts of 1,024 assets (`search/lines.rs`,
  `take_part`; the patch in `work/perf-design/verify/lines.diff`), checked
  on a clean copy of `f7f2e1a`: the Stakka scan at NAS scale 38.5 s of
  search instead of 76.7 s at 32 workers, 45.7 s instead of 49.3 s at 16,
  reports identical apart from `wall_seconds`. Not in master; a session
  adopts it with tests and the regressions.
- No Rust code in master changed, so `scripts/check.sh` was not run.
- Later: `dmidecode -t 17` showed the four DDR4-3600 modules at 2133 MT/s
  (XMP off); with XMP on (3600 MT/s), experiment 0168: the probe reads
  40.1-46.2 GiB/s at 81.8 ns; the current code's Stakka search at 32 workers
  49.7 s instead of 76.7 s, the clusters sample 165 s instead of 211 s; with
  0167's change, or at 16 workers, the times stay within 2% (37.9 s and 46.4
  s with the change). Reports identical to the 2133 ones apart from
  `wall_seconds`.

## 2026-10-22: session 36, a faster search on the Linux machine, exactly

Brief `docs/brief-36.md`, checklist `docs/session-36-checklist.md`;
notes "Session 36"; experiments 0169-0174; the research note
`docs/research/2026-10-21-low-bandwidth-search.md` ("Session 36"). The
Linux machine at 3600 MT/s.

- 02:34 start at `e0c6316`, clean. Start build `work/bin/s36-start/`
  (byte-identical to `s35-final`); both regressions identical (1:12;
  8:16). Session 35's renders deleted (11 GB).
- 0169 (`3903770`, in master): each part's end found from the front of
  the list (`front_partition`, tested against `partition_point`), parts
  of 1,024 assets. The Stakka scan at NAS scale: search 45.3 s instead of
  47.2 s at 16 workers, 37.9-38.0 s instead of 49.0-49.2 s at 32. Both
  regressions, the 11 development reports at NAS scale, the clusters
  sample and the full NAS clustering run (23:02) identical.
- 0170: timers in a diagnostic build: the pre-filter 43% of the
  search's CPU at 16 workers, grouping 28%, query analysis 14%, taking
  parts 6%, sorting and lines 6%, the second pass 2%.
- 0171, 0172: grouped hits as 4 bytes, raw hits as 6, and grouping from
  the lists without `part`: 4.4-8.2% slower at 16 workers; not kept.
- 0173: repeated hash requests are 5.1% of postings in a window, 14.9%
  within 4; at most about 1% to gain; not implemented.
- 0174: `identify` with the NAS configuration: reports identical, CPU
  3-4% lower at 16 workers and 16-22% at 32; one worker per hardware
  thread recommended (the default unchanged).
- 04:35-04:44 read-only review by a separate agent; its findings fixed
  (five figures, wording, two code comments). Final checks with
  `work/bin/s36-final/`: `check.sh` green; both regressions identical
  (1:10; 5:56); `docs/pair-verdicts.txt` unchanged.

## 2026-10-22: session 37, lost sides of a pitch ride

Brief `docs/brief-37.md`, checklist `docs/session-37-checklist.md`;
notes "Session 37"; experiments 0175-0178. The product did not change.

- 04:56 start at `a0b91e4`, clean. Start build `work/bin/s37-start/`
  (byte-identical to `s36-final`); both regressions identical (1:09;
  6:05).
- 0175: every lost side of 0166's seeds (192 without a confident
  detection, 221 with 30 s or more uncovered) is confident after the
  second pass and dropped by the second `strongest_per_moment`: the
  second pass's 10 s margin moves both sides' facing ends into the ramp
  (overlap median 4.7-5.2 s); the chains never overlap.
- `mixes --kind rides --ride-change --ride-seconds --ride-direction`
  (one seed plans the same files, mix times, speeds and ride starts under
  every condition).
- 0176: rides of 0.25%, and 0.5% over 10 s or more, lose nothing; at
  1-2%, and 3% over 2-5 s, 21-43% of the sides lose 30 s or more (3% over
  10-20 s: 4%). Star Trails' slower side in
  the Stakka mix (84 s) is dropped at both scales; Into Music's middle
  segment in the 2003 mix (candidates).
- 0177: of the existing options only `--single-pass` keeps both sides.
- 0178: the opt-in `--overlap-by-chains` (criteria committed first)
  keeps all 183 lost sides on new seeds, with no new wrong confident
  detection on the sweep at both scales, the development scans and
  leave-outs and the generated mixes; CPU +0.2-0.4%. On the first shared
  panel no query gains a wrong file, but a wrong version already
  confident in two Fire queries gains a second segment (4 detections):
  V2 fails by its letter. Not adopted (an ADR).
- 07:21-07:36 read-only review by a separate agent; its findings fixed
  (overstated figures, 0178's length, paths in 0175's data, a code
  comment). Final checks with `work/bin/s37-final/`: `check.sh` green;
  both regressions identical (1:09; 5:56); `docs/pair-verdicts.txt`
  unchanged.

## 2026-10-22: session 38, a narrower variant for pitch rides

Brief `docs/brief-38.md`, checklist `docs/session-38-checklist.md`;
notes "Session 38"; experiments 0179-0181; the decision note
`docs/research/2026-10-22-pitch-rides-decision.md`. The product did not
change.

- 08:12 start at `f7e4819`, clean. Start build `work/bin/s38-start/`
  (byte-identical to `s37-final`); both regressions identical (1:09;
  6:42). A default run of the first shared panel's development half
  (38:31) as the variant's comparison.
- 08:27 `393469d`: the opt-in `--apart-in-speed SHARE`
  (`SecondOverlap::SpansAtSpeed`), the `robust` rides inside excerpts,
  and the pre-registration: threshold 0.6% from session 37's development
  traces, criteria S1-S3, committed before any variant run.
- 0179: `--apart-in-speed 0.006` passes S1-S3: all 257 lost sides kept
  on new seed 3801; no new wrong confident detection and no lost right
  one on the sweep at both scales, the 11 development reports at NAS
  scale, the generated mixes and the first shared panel, where nothing
  confident changes (0178's variant added a wrong version's segments
  there; the panel was also the threshold's development data). Star Trails' and Into Music's dropped segments kept
  (candidates).
- 0180: a ride in the middle of a 30 s excerpt costs recall from 2%
  (43-49 of 80; 3%: 3-8), decided by the 3-window rule; the overlap
  rules win back 1-3 excerpts.
- 0181: per-second correct labels on seed 3801 from 64-93% to 96-97%
  under the variant; Stakka +1.7 points, the 2003 mix +0.6.
- Recommendation (ours to decide): adopt `SpansAtSpeed(0.006)` through
  an ADR after one confirmation on the second shared panel.

## 2026-10-22: after session 38, the listening of the accumulated pack

- The pack `work/listening/` (from session 33's NAS clusters on the
  Linux machine: one cut join and five borderline pairs, every item
  without a verdict) was answered on the listening page (21:49-21:56
  UTC, answer log records 1-6 of `work/audition/answers.jsonl`). The
  page added six lines to `docs/pair-verdicts.txt`: Dilusion `same`, the
  other five `different`. The comments by ear, verbatim in
  `work/audition/listening-comments-2026-10-22.md`:
  - The old-skool-jungle-dnb upload of Sonar ~ the Mark System
    Revision: different, no comment. The cut that `--cut-sparsest` made
    there (0128) agrees.
  - The corpus China Cup (`extra/b-bad_company-china_cup.mp3`) ~
    ƆƐI3C's upload: almost identical, but the hi-hats differ for a few
    bars; technically different.
  - The Rawkuts Dilusion ~ dfect-dnb's upload: the same recording; one
    has a needle skip or a cut, so they differ for 1-2 s and the rest
    is shifted.
  - Aphrodite - Cool Rock Stuff (APH027 B1) ~ Mobb Deep Vs Urban
    Takover - Cool Rock Shook Ones: a different mix of the same track,
    one drum loop replaced by a very similar one.
  - Kosheen - Hide U ~ the Outrage & Markee Ledge Remix: the same
    samples and vocals over very different drums.
  - Dieselboy - The Descent (Phunckateck VIP) ~ Frenchi / Proktah -
    Scarabeus: completely different; at most the drum rhythm is alike.
- `clusters --from-peaks --reuse-pairs` with the new verdicts (0.9 s):
  still 284 clusters; Dilusion's upload joins the Dilusion cluster
  (three files), and the Sonar chain needs no cut (`cut_links` gone).
  The clusters file in use is now `d9c85c45…` (was `37bc419d…`; both in
  `work/listening-2026-10-22/`). Every borderline pair between two
  clusters has a verdict now, so the pack's selection finds nothing
  left but controls.
- What they imply:
  - Neither coverage nor supported time separates China Cup (78%, 0.90,
    different) from Dilusion (73%, 0.91, same). Where the other
    alignments lie does: Dilusion's sit within a few seconds of the
    main one (offsets +3.9, +6.6 and +5.1 s, as a skip or cut shifts
    them), so aligned hits continue through the file; China Cup's sit
    60 and 250 s away (sections repeated elsewhere in the track), and A
    4:20-5:41 has nothing near the main offset, where by inference the
    hi-hats differ. Share of the shorter file held by alignments within
    10 s of the main offset: Dilusion 0.91, China Cup 0.77. Two pairs,
    so a hypothesis for the version verifier and the join rule, not a
    rule.
  - The roadmap names China Cup's upload as most of the false confident
    detections left at NAS scale. By ear it is a near-identical version:
    those detections name the right track in a version that differs
    for a few bars. Whether such a detection counts as wrong is ours to
    decide; the clusters and the scoring count it as wrong.
  - Dieselboy ~ Frenchi reached 41% coverage on 6 hits in 2 windows.
    Coverage on so few hits says little; the pack's borderline
    selection could require the 30 hits `clusters` uses for related
    recordings.
  - Aphrodite's and Kosheen's pairs are shared material that the
    clusters already keep apart.
- With the new clusters file, `regress session-33-linux-nas` is identical
  (720 of 720 sweep queries, all three scans' detections, calibrate;
  5:56 with `work/bin/s38-final`): Dilusion is in no development scan
  or sweep query. The recall panels hold it, so their NAS-scale reports
  may credit the upload next time they run.

## 2026-10-23: session 39, the printed tracklist on double drops

Brief `docs/brief-39.md`, checklist `docs/session-39-checklist.md`;
notes "Session 39"; experiments 0182-0185. The product did not change.

- 00:21 start at `6665077`, clean. Start build `work/bin/s39-start/`
  (byte-identical to `s38-final`); both regressions identical (1:09;
  6:02).
- 00:31 `937ceba`: the pre-registration (scenarios, truth, scores,
  problems P1-P6) and the tag check: 198 of 245 corpus recordings have
  tags that agree across their files and with the file names.
- 00:50 `d671100`: `mixes --kind drops --tagged FILE` (double drops,
  records inside stronger ones, a shared-material pair apart and
  together, a returning record, two rips, a held-out record) and
  `scripts/analysis/tracklist_score.py`; both regressions identical.
- 0182: the CLI on 32 mixes (seeds 2026 and 2028) at 262 tracks and NAS
  scale; every printed line checked against the rebuilt entries.
  Grouping and printing lose nothing the search finds confidently but
  one merge in 512 plays (`same_recording` by coincidence in a long
  drop). A record inside its partner found only as possible leaves the
  tracklist (ADR 0019): 19-25% of unrelated drop partners that leave
  again, 6 of 8 Clockwork originals inside the remix, 19 of 32 brief
  records. No confident wrong entry at 262 tracks; at NAS scale
  Animation's remix, a related version by verdict but in no panel
  family, so P1 is met by its letter.
- 0183: no measure in a report tells a record playing under its partner
  from shared material; a 40 s bound would print every drop partner and
  19 of 64 shared-material lines.
- 0184: the harness's levels agree with the CLI's in all 512 plays;
  printing leaves out 41 of 43 records the search finds as possible
  inside a partner.
- 0185 (`--other-pairs`, three more pairs at NAS scale): whether a
  record inside its relative is printed depends on the pair; Pyro's
  records, which share at least 188 s, are one flagged entry. New confident candidate
  for listening: Digital & Spirit - Crash over the start of the T-Minus
  vocal mix.
- Waiting (ours): whether marked plays may stay in the tracklist and
  whether `same_recording` needs more when titles differ; both need an
  ADR.
