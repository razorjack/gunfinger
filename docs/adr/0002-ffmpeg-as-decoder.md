# 0002: FFmpeg as the decoder

## Status

Accepted.

## Context

The collection holds MP3, AAC in M4A, Opus and, later, FLAC, WAV, AIFF and
ALAC. Files carry embedded cover art, some are damaged, and a few are other
audio renamed. Gunfinger needs mono PCM at the analysis rate (8 kHz) from all
of them, with honest failure reporting.

## Decision

Decode with the `ffmpeg` executable through `std::process::Command`, never a
shell string:

```text
ffmpeg -nostdin -v error -threads 1 [-ss S] [-t T] -i <path>
       -map 0:a:0 -vn -sn -dn -ac 1 -ar 8000 -f f32le pipe:1
```

- stdin is null; stdout is streamed into Rust and converted from
  little-endian `f32`; stderr is drained on a second thread so neither pipe
  can stall the child. On a read error the child is killed; it is always
  reaped.
- A non-zero exit is a failed asset. Error output alone is accepted (FFmpeg
  reports recoverable glitches such as one damaged MP3 frame), unless the
  decoded audio is more than 1 s shorter than the duration `ffprobe` reports
  for the container: then the asset is truncated and failed. A failed asset
  is reported and nothing is written for it.
- `-map 0:a:0` selects the first audio stream, so cover art is never decoded
  as audio (covered by codec tests with the image stream first).
- Indexing decodes at most `--max-track-minutes` plus one second, so a mix in
  the library costs little before it is skipped.

## Consequences

- One external dependency to install (`brew install ffmpeg`), in exchange for
  every codec and container without native decoding crates.
- Resampling quality is FFmpeg's default `swr` resampler. The speed ladder
  does not depend on it: rungs are realised by a scaled STFT, not by
  resampling (experiment 0001).
- Three library MP3s fail today: their audio stops about 1.3 s before the
  declared length, with "Header missing" errors. They are excluded, not
  silently half-indexed.
- Codec tests generate fixtures with FFmpeg at test time and skip with a
  message when it is missing.
