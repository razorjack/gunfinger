# Status

Running log. Newest entry at the bottom.

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
