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
