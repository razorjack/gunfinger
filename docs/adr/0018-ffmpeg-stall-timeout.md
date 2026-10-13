# 0018: A stall timeout on FFmpeg and FFprobe

## Status

Accepted (2026-10-13, session 16). Extends ADR 0002: decoding and probing
stop when the child writes nothing for two minutes. Follows from ADR
0017's priority of unattended reliability.

## Context

The library lives on a NAS mounted over SMB (`/Volumes/atlas`, not a
`soft` mount). When the share stops answering, a read of a file on it
can block for as long as the share stays silent. FFmpeg and FFprobe then
wait, and so did Gunfinger: `index` stopped making progress on that
worker, `identify` never returned, and the listening page held the
browser's request open. Nobody watches an unattended run, so a run must
end with results or a clear error. Session 14 left this open for the
listening page (notes, "A timeout on FFmpeg").

## Decision

- **A stall timeout, not a total one.** `gunfinger_core::decode::STALL`
  is 120 s. A run of `ffmpeg` that writes nothing to stdout for that long
  is killed and the file fails with `DecodeError::Stalled`: "<path> could
  not be read: nothing came from `ffmpeg` for 120 s, so it was stopped.
  The storage holding the file may be unresponsive, such as a NAS that
  hangs; check that the file opens, then try again". A decode writes
  about 32 KB per second of 8 kHz audio, every fraction of a second,
  however long the file, so a two-hour mix on a slow share still decodes.
  The same limit applies to `ffprobe` (which writes only at its end, after
  reading a header), to the wait for the child to exit once its output has
  closed, and to the first bytes Gunfinger reads itself to choose how
  FFmpeg opens the file (`Container::of`, run on a thread of its own).
- **Why 120 s.** A healthy run is silent only while storage is slow to
  answer: a NAS spinning up its disks takes tens of seconds, and an SMB
  client that reconnects may take as long. Two minutes leaves room for
  both. A false stall fails a whole `identify`, while a true one costs
  only the wait, so the value errs long. It is not measured against a
  hung share (none was
  available). One constant serves every caller; it is not configurable.
- **Not waiting for a hung child.** A process blocked in a read of a hung
  share does not die until the read returns, even when killed. Stdout and
  stderr are read on threads of their own, and the killed child is reaped
  on another; the caller waits for none of them after a stall, and they
  end when the child does (`decode::watch`).
- **`index`: failed for this run only.** A stalled file is reported as
  failed and counted in the run's failures (the run exits unsuccessfully),
  but it is not remembered in the peak store the way a damaged file is
  (`DecodeError::is_about_the_file` is false): a NAS hiccup is often
  transient, and the next `index` tries the file again. A stall while
  reading the tags of a file already indexed leaves its tags for the next
  run, as any unreadable header does.
- **`identify`** fails that recording with the message; with several
  recordings it continues with the next and exits unsuccessfully, as for
  any recording that cannot be searched.
- **The listening page** decodes its windows through the same watcher and
  the same limit, and its envelopes through the core's decoder: a stalled
  window answers `503` with the message, which the page retries.
- **A seam for tests.** `Decoder` holds the paths of `ffmpeg` and
  `ffprobe` and the timeout; `Decoder::default()` runs them from `PATH`
  with `STALL`. Tests point it at stand-in shell scripts in temporary
  folders with a 0.5 s timeout, never changing `PATH`. `IndexingOptions`
  carries the decoder.

## Consequences

- A share that hangs fails each file after two minutes instead of
  stopping the run. If it never answers again, every remaining file of an
  `index` run waits its two minutes on each worker; the run ends, slowly.
  Stopping a run after many stalls in a row would be a separate decision.
- Calls outside FFmpeg still block on a hung share: listing the library
  (`Library::scan`), reading a file's size and modification time, and
  opening the recording `identify` checks first. They are outside this
  decision.
- `ffplay` (`listen`, `review`, `pair-review`) plays to a person, who can
  stop it, and has no timeout.
