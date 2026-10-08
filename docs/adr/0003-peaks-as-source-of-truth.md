# 0003: Peaks as the source of truth

## Status

Accepted.

## Context

Decoding the collection is the expensive step (it will live on a NAS).
Hash design, density and index layout are expected to change as the
project grows. Rebuilding the index must not require decoding audio again.

## Decision

`gunfinger index` stores, per asset, the spectral peaks: local maxima of the
log-power STFT (8 kHz, Hann 1024, hop 128, neighbourhood ±12 frames × ±12
bins, floor -10 dB; Wang 2003, §2.1). Each peak keeps its frame and bin
refined between grid points by parabolic interpolation (Smith & Serra
1987; 1/64 frame, 1/64 bin) and its magnitude
(0.5 dB). Hashes and the index are always derived from these peaks.

Store layout (`gunfinger-core/src/store.rs`):

- one file per asset in `work/peaks/` (`--peaks-dir`, `GUNFINGER_PEAKS_DIR`),
  named by a 64-bit FNV-1a hash of the asset path;
- a self-describing header: magic, format version, front-end profile string,
  source path relative to the library root, size, mtime (ns), duration;
- peaks as frame delta (LEB128), frame offset (i8), bin (u16), magnitude
  (u8): about 5 bytes per peak, 187 bytes per second of audio;
- sequential reads and writes only, atomic replace (temporary file, fsync,
  rename), no mmap.

`index` skips an asset whose record matches path, size, mtime and profile.

## Consequences

- Changing any front-end parameter changes the profile string, and every
  record is re-extracted on the next `index` run (about 50 s for this
  library on 10 cores).
- Hash experiments rebuild the index from peaks in under a second.
- Refined peak times matter: with integer frames, hashes of a query whose
  frame grid is offset from the reference's survived at 30% instead of 70%
  (experiment 0001).
- Two libraries must not share a peak directory: paths are relative to the
  library root. Recorded in the roadmap.
