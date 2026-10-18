# Session 32 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-32.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `docs/pair-verdicts.txt` digest in
  `work/s32/listening-digests-start.txt`
- [x] Git identity: `Jacek Galanciak <jacek.galanciak@gmail.com>`

## Steps

- [x] 1. Prerequisites and versions (rustup installed in the home
  folder; FFmpeg 8.1.2 here, 9.0.2 on the Mac)
- [x] 2. The corpus copied from the drive (327 files, 1:38); `sha256sum
  -c` all OK; nothing from `corpus/` in `git status`
- [x] 3. `cargo build --release` (1.98.0, no warning); `scripts/check.sh`
  green with 1.98.0 and with stable 1.99.1, unchanged: 466 Rust tests, 5
  Node tests, clippy for Linux and macOS
- [x] 4. Configuration files; `doctor` with each (corpus: 265 files;
  NAS: 42,609 audio files, 618 left out by 41 patterns)
- [x] 5. The corpus from scratch: `index` (265 records); records and
  skips against the Mac's; the 3 damaged files (decoded, accepted by the
  1% tolerance); clusters (17 of 17 equal); `regress session-30-start`
  (scores equal, not bit for bit; cause: rustfft's SIMD path);
  `baseline session-32-linux` (repeats identically); experiment 0152
- [x] 8. The listening tool with `--no-open` on the Stakka mix's corpus
  report: page, session, audio windows and envelope over HTTP; `Host` and
  token checks; `xdg-open` 1.2.1 present; no answer written
- [x] 6. The NAS: mount found (cifs, read-write by the user's
  decision); `doctor`; the held-out broadcast's file only probed (7,091.7 s,
  too long from the header); paths against the Mac's before the index
  (227 NFD there, NFC here); `index` (run by the user, 3,168.9 s, the
  Mac's counts); records and skips against the Mac's (all paths after NFC,
  durations, sizes, skips and failures equal; peaks as on the corpus);
  verdict and manifest paths (114 present); `stats`; the three
  development mixes (11/11, 12/12, 17/17, nothing wrong); experiment 0153
- [ ] 7. Fixes (each with a test; `scripts/check.sh` green after each)

## Wrap-up

- [ ] 9. Read-only review by a subagent; fixes
- [ ] 10. Notes "Session 32", status, checklist, ADR 0020's
  consequences
- [ ] `scripts/check.sh` green; `regress session-32-linux` identical
  with the final build; `docs/pair-verdicts.txt` unchanged; `git status`
  clean; commit
- [ ] `session-32.bundle` and `session-32-summary.md` on the drive
- [ ] The final message
