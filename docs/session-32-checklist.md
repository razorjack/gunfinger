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
- [ ] 3. `cargo build --release`; `scripts/check.sh` (1.98.0 and stable)
- [ ] 4. Configuration files; `doctor` with each
- [ ] 5. The corpus from scratch: `index`; records and skips against the
  Mac's; the 3 damaged files; clusters; `regress session-30-start`;
  `baseline session-32-linux`; experiment 0152
- [ ] 8. The listening tool with `--no-open`: page and an audio window
  over HTTP; `xdg-open` present; no answer
- [ ] 6. The NAS: mount found, type and options, read-only; `doctor`;
  the held-out broadcast's file only probed; `index` under `tmux`;
  records and skips against the Mac's (raw and NFC); verdict and
  manifest paths; `stats`; the three development mixes; an experiment
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
