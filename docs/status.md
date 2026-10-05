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
