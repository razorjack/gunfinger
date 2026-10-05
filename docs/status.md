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
