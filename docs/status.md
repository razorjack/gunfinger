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
  they were. Opt-in: `identify --playback both`; with both ladders the
  full protocol passes with unchanged thresholds (0016), about a third
  slower.
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
  unchanged thresholds at about 1.3 times the search time. `playback` in
  the configuration file sets the default per library; the built-in
  default stays turntable.
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
