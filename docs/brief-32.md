# Session 32 brief: Gunfinger on the Linux machine

Written by the agent for itself on 2026-10-18, so the plan survives
context compaction. Work autonomously. Stop and ask only in the prompt's
three cases (the drive or the NAS mount cannot be found; a step needs
`sudo` or a system package; a failed check needs a decision the prompt
does not cover). Otherwise take the conservative option, write the
decision down and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-32-checklist.md` (progress).
3. `AGENTS.md` (the wording rule, "Platforms"), ADR 0020, ADR 0015.
4. `docs/session-notes.md` ("Session 30" to the end) and
   `docs/status.md` (newest entries at the bottom).
5. `README.md` ("Getting started", "Configuration", "Regression
   checks"), `scripts/check.sh`, `scripts/analysis/store_records.py` and
   the drive's `mac-reference/tools/store_skips.py`.

`docs/brief-2.md` to `docs/brief-30.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them. This prompt changes one: step 4 writes the two
configuration files, which earlier briefs said not to edit.

## The prompt, verbatim

```text
Start session 32 of Gunfinger: bring Gunfinger up on a Linux machine (x86-64) from a fresh clone of GitHub's master, index the corpus and the NAS collection here from scratch, and show that Linux delivers what macOS does. Until now every build and run was on one Mac (arm64). ADR 0020 made the code cross-platform, but nothing has run on Linux yet. Where something does not build, run or pass here, this session fixes it, tests it and commits it. Work autonomously. Stop and ask the user only in three cases: you cannot find the USB drive or the NAS mount; a step needs `sudo` or a system package; a failed check needs a decision this prompt does not cover. Otherwise take the conservative option, write the decision down and continue. There is no fixed end. The NAS index takes hours: let it run unattended and finish the session after it.

Inputs.
- The repository: this clone of github.com master (record its commit; expected `b97e3da` or later).
- The USB drive labelled Ultralisk, folder `gunfinger/`. The user gives its path; otherwise look under /media/$USER, /run/media/$USER and /mnt, and ask if it is not there. It holds:
  - `corpus/`: the private corpus (`library/`, the reference tracks, and `sets/`, the mixes with their manifests), gitignored in the repository. `corpus.SHA256SUMS` lists every file with its SHA-256, paths relative to `corpus/`.
  - `mac-reference/`, read-only references from the Mac, never used as this machine's data:
    - `corpus-records.tsv` and `nas-records.tsv`: every peak record of the Mac's two stores (source path, duration in seconds, peak count, a digest of the peak bytes, the record file's own mtime, the source size), written by the repository's `scripts/analysis/store_records.py`;
    - `corpus-skips.tsv` and `nas-skips.tsv`: every skip note (source path, size, mtime, reason, message or limit), written by `tools/store_skips.py`;
    - `baselines/session-30-start/`: the Mac's `gunfinger-eval baseline` reports at 262 tracks; `regress session-30-start` was identical on the Mac at master's current code;
    - `reports/duplicate-clusters.json`: the Mac's corpus clusters;
    - `config/config.toml` and `config/nas-dnb.toml`: the Mac's two configuration files.
  - The Mac's versions: macOS on arm64, FFmpeg and FFprobe 9.0.2, Rust 1.98.0 for the checks.
- The NAS: the drum & bass collection, the folder `Music/dnb` of the NAS share (on the Mac, `/Volumes/atlas/Music/dnb`). Its root holds a `.gunfingerignore`. At the Mac's last listing the store held 40,683 peak records and 1,926 skip notes (11 failed decodes, 1,768 too long, 147 too short). Files may have been added to the NAS since.

First:
1. Read CLAUDE.md, AGENTS.md (every rule there applies, including the wording rule and "Platforms"), docs/brief.md (never edit it), README.md ("Getting started", "Configuration", "Regression checks"), ADR 0020, ADR 0015, docs/status.md (the newest entries at the bottom), docs/session-notes.md ("Session 30" to the end), `scripts/check.sh`, `scripts/analysis/store_records.py` and the drive's `mac-reference/tools/store_skips.py`.
2. Write docs/brief-32.md (this prompt verbatim, the state at the start, the plan) and docs/session-32-checklist.md, point CLAUDE.md's reading list at them (brief 30 joins the older briefs), and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s32/listening-digests-start.txt; it must be the same at the end.
3. Check `git config user.name` and `user.email`. Commits must carry `Jacek Galanciak <jacek.galanciak@gmail.com>`; if the configuration says otherwise or nothing, ask the user before the first commit.

Steps:

1. Prerequisites. Check for, and record the versions of: a C toolchain and linker; `rustup` with the toolchain `rust-toolchain.toml` names (with rustfmt and clippy); `ffmpeg` and `ffprobe` (`ffplay` too if available); `git`; Python 3; Node 22.7 or newer (optional: `check.sh` skips the listening page's JavaScript tests without it); `tmux` or `screen`. Rustup and its toolchains and targets install in the user's home and need no `sudo`; add the macOS target too (`rustup target add aarch64-apple-darwin`), so that `check.sh` also type-checks the macOS branches. For anything that needs `sudo` or the system's package manager, detect the distribution, print the exact command and ask the user to run it.

2. The corpus. Copy the drive's `corpus/` to `corpus/` in the repository (`rsync -rt --exclude '._*'`), then `sha256sum -c` with `corpus.SHA256SUMS` from inside it: every file must match. Check that `git status` shows nothing from corpus/. The corpus is private and never committed.

3. Build and check. `cargo build --release`, then `scripts/check.sh` (formatting, clippy for Linux and for macOS, every test, the Node tests). Record the results. Every failure goes to step 7.

4. The configuration files. Write `~/.config/gunfinger/config.toml` for the corpus and `~/.config/gunfinger/nas-dnb.toml` for the NAS, after the Mac's in `mac-reference/config/`, with the same `playback`, `min_track` and `max_track`. The corpus file sets `library` to this repository's `corpus/library` and `peaks_dir` to its `work/peaks`, as on the Mac, so that the CLI and the harness share one store. The NAS file sets `library` to the NAS mount's `Music/dnb` folder (step 6) and `peaks_dir` to `~/.local/share/gunfinger/nas-dnb-peaks`, on the local disk. Keep the comments general: no machine details. Run `gunfinger doctor` with each and record its output.

5. The corpus from scratch.
   - `gunfinger index` with the corpus configuration. Then compare the new store with the Mac's (`store_records.py` and `store_skips.py` on `work/peaks`, against `corpus-records.tsv` and `corpus-skips.tsv`). For each path, count equal and different durations, peak counts and peak digests, and list the paths only one side has. The Mac skips 3 files as damaged; say whether this machine's FFmpeg decodes them, and how far.
   - The harness at 262 tracks: run `gunfinger-eval clusters` (this machine's own corpus clusters) and compare them with `mac-reference/reports/duplicate-clusters.json`. Copy `mac-reference/baselines/session-30-start/` to `work/baselines/`, then run `gunfinger-eval regress session-30-start`. Search is deterministic, so "identical" means bit for bit. If anything differs, find the cause (decoded samples, peaks, a platform difference in floating point, the order of a sort or a directory listing) and record it. Then run `gunfinger-eval baseline session-32-linux`, for later regressions on this machine.
   - An experiment file (at most 40 lines; the next free number, from 0152): macOS against Linux on the corpus, as peaks, clusters and detections.

6. The NAS from scratch.
   - Find the mount: `findmnt` for cifs, smb3, nfs and nfs4, then a folder `Music/dnb` with a `.gunfingerignore` at its root. If none is found, or more than one, stop and ask the user for the path. Record the mount's type and options. If it is not read-only, tell the user and suggest remounting with `ro`, and continue only with their agreement. Gunfinger never writes to its library.
   - Check `gunfinger --config ~/.config/gunfinger/nas-dnb.toml doctor`. The NAS root holds `EM_1999_02_07_-_Ed_Rush_Optical.mp3`, by its name the broadcast of the held-out test set. It must never be decoded, identified or indexed: being longer than `max_track`, it only has its length probed and gets a too-long skip note, as on the Mac (it is in `nas-skips.tsv`). Confirm that this holds before indexing, or that the ignore file leaves the file out. If neither is true, stop and ask.
   - `gunfinger --config ~/.config/gunfinger/nas-dnb.toml index`, detached under `tmux` or `screen`, with its own log, `time`, and an exit line. Do not use `prune`, `--retry-skipped` or a second index run unless something fails, and record it if one is needed.
   - Compare with the Mac's listings, raw and after Unicode NFC normalization of the paths. 160 of the Mac's NAS paths are not in NFC; count the paths that differ only in normalization. For paths on both sides: equal and different durations, peak counts and digests. Paths only the Mac has: check whether the file is still on the NAS, now left out by the ignore file, or seen under another name. Paths only this machine has: files added since. Skip notes: list every file that failed on one platform only, with both messages. Also check that every NAS path named in docs/pair-verdicts.txt and in the set manifests (`second-library/<path>`) exists on this mount byte for byte, because the harness matches these paths exactly.
   - `gunfinger --config ~/.config/gunfinger/nas-dnb.toml stats`: records, index size, and the time and peak memory of a search's index load. Then `identify` the three development mixes against the NAS store (`corpus/sets/stakka-skynet-knowledge/`, `corpus/sets/razorjack-2003-03-29/` and `corpus/sets/nocny-trans-vertical-2002-08-23/`, the audio file in each folder) with `-f json`, save each report under work/s32/, print its tracklist with `gunfinger show`, and compare each tracklist with its manifest. On the Mac at NAS scale these gave 11/11, 12/12 and 17/17 with nothing wrong (README, "Results").
   - An experiment file: macOS against Linux on the NAS collection.

7. Fixes, whenever a step finds that Gunfinger, its harness, `check.sh` or the listening tool does not build, run or pass on Linux. You may fix, test and commit on master without asking, within these limits:
   - Follow ADR 0020: one function per need, branching on `platform::Platform`, every branch compiled and tested on either system. Each fix adds or changes a test that fails without it. `check.sh` passes on Linux after each commit, including the macOS clippy pass.
   - Master takes only changes that cannot alter decoded audio, peaks, hashes, detections or scores on macOS: build and test fixes, paths, process spawning and its arguments, file locking, the listening tool's desktop calls, messages, scripts. A change that could alter any of those on macOS (in decoding, peak picking, hashing, the search, the confidence rule, plays or scoring) is committed on a branch `s32-detection` instead, never merged. Describe it in the notes, so that a session on the Mac can check it with `regress` first.
   - Do not change the algorithm to make the two platforms' numbers agree. A difference between them is a finding: measure it, explain it, record it. A decision it calls for is ours and needs an ADR.
   - A fix that changes a documented decision needs a new ADR (AGENTS.md). Facts in the README or ADR 0020 that prove wrong on Linux are corrected in the same commit as the fix.

8. The listening tool. With `--no-open`, start `gunfinger-eval audition` on questions made from one development mix's saved report (`questions detections`; the README's "The listening tool" and ADR 0016 describe how). Fetch the page and one audio window over HTTP (curl), check that the browser and reveal commands this platform chooses exist (`xdg-open`), then stop the server. Answer no question: agents never write verdicts or listening answers.

9. A read-only review by a subagent of the session's commits and notes; fix what it finds.

10. Wrap-up.
   - A "Session 32" section at the end of docs/session-notes.md: what ran on Linux unchanged, what was fixed (commit by commit), what is on `s32-detection` and why, and whether Linux reproduces macOS (corpus peaks, clusters and `regress`; NAS records, skips and the three tracklists). Also: what the next Linux session or a Mac session should do. Update docs/status.md, the checklist and, where its facts changed, ADR 0020's consequences.
   - The final checks: `scripts/check.sh` green, `regress session-32-linux` identical with the final build, docs/pair-verdicts.txt unchanged, `git status` clean. Commit.
   - Write the new commits to the drive, for the Mac to fetch: `git bundle create <drive>/gunfinger/session-32.bundle origin/master..master` (and the `s32-detection` branch, if it exists, in the same bundle). Also write a short `<drive>/gunfinger/session-32-summary.md`: the commits, the checks, and what is left.
   - Last, print for the user what was done, the checks, and anything left.

Rules for this session:
- No push. Commit on master (and `s32-detection` only as step 7 says) with the identity of First, item 3. Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- Long runs: the Bash tool kills a background task at its timeout, 30 minutes by default. Every run that may last more than about 20 minutes starts detached under `tmux` or `screen` with its own log and an exit line, or gets an explicit timeout of up to 7,200,000 ms. After each run, check that its log ends with its exit line and holds no error.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root beyond the length probe described in step 6. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The development mixes are the only mixes this session searches.
- The NAS is read, never written, its `.gunfingerignore` included. The NAS peak store here is this machine's own; the drive's `mac-reference/` is only compared against, never copied into a store.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests do not change.
- This session tunes nothing: no threshold, parameter or rule changes because of a result.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts, evaluation JSON in docs holds file names only, cross-platform code (ADR 0020), no filename- or tag-derived logic in the product.
- Repository text follows AGENTS.md's wording rule, and names no hardware model and no detail of how the machine is used. The Mac is "the Mac", this machine is "the Linux machine".
- Leave alone every process this session did not start.
- Scratch data goes under work/s32/; keep it under a few GB, and delete renders and clones you no longer need. The NAS store is about 3 GB.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
```

## State at the start of session 32

- 2026-10-18, 18:32. A fresh clone of GitHub's master at `b97e3da`
  (equal to `origin/master`), clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`, as First, item 3 requires.
- The Linux machine: Fedora Linux 44, x86-64, 32 hardware threads (the
  default `jobs`), 62 GiB of memory, 853 GB free on the disk holding the
  repository.
- The drive: `/run/media/razorjack/Ultralisk/gunfinger/` (the user gave
  the path).
- The NAS: not mounted at the start; the user is mounting it. Steps that
  need it wait; the rest go first.
- Experiments 0001-0151; this session numbers from 0152. ADRs
  0001-0020. Test set: 4 of 5 evaluations used (none in this session).
- `docs/pair-verdicts.txt`: 82 lines, SHA-256 `ad0fc785d26a...`
  (`work/s32/listening-digests-start.txt`). This clone has no
  `work/audition/answers.jsonl`.
- Prerequisites (step 1), found before anything was installed:
  - C toolchain: GCC 16.2.1, GNU ld 2.46.1, glibc 2.43; a C program
    compiles and links.
  - Rust: none. `rustup` 1.29.1 was installed in the home folder with
    rustup's installer (no `sudo`, `--no-modify-path`, so no shell
    file changed), then the toolchains `stable` (1.99.1, the channel
    `rust-toolchain.toml` names) and `1.98.0` (the Mac's), each with
    rustfmt, clippy and the target `aarch64-apple-darwin`.
  - FFmpeg, FFprobe and FFplay 8.1.2 (RPM Fusion's `ffmpeg` package,
    with libmp3lame, libopus, libvorbis and libfdk-aac). The Mac has
    9.0.2: the major version differs, besides the platform.
  - Git 2.55.0, Python 3.14.7, Node 24.18.0, tmux 3.7c (no `screen`),
    `xdg-open`, eSpeak NG 1.52.0, GNU time 1.9, `findmnt`,
    `mount.cifs`, `mount.nfs`, Chrome.

## Plan

| Step | What | Needs the NAS |
|---|---|---|
| First | reading; this brief, the checklist, `CLAUDE.md`; digests | no |
| 1 | prerequisites and their versions (above) | no |
| 2 | the corpus copied from the drive, `sha256sum -c` | no |
| 3 | `cargo build --release`; `scripts/check.sh` (with 1.98.0, as on the Mac, and with `stable`) | no |
| 4 | `~/.config/gunfinger/config.toml` and `nas-dnb.toml`; `doctor` with each (the NAS file's again once mounted) | partly |
| 5 | corpus `index` from scratch; records and skips against the Mac's; clusters; `regress session-30-start`; `baseline session-32-linux`; experiment 0152 | no |
| 8 | the listening tool with `--no-open` on a development mix's questions | no |
| 6 | the NAS: mount checks, the held-out broadcast's file, `index` under `tmux` (hours); records and skips against the Mac's; `stats`; the three development mixes; an experiment | yes |
| 7 | fixes, whenever a step finds one, each with a test; detection-affecting ones on `s32-detection` | |
| 9 | read-only review by a subagent; fixes | |
| 10 | notes, status, checklist, ADR 0020; final checks; the bundle and summary on the drive | |

Step 8 moves before step 6 because it needs no NAS. Steps 1-5 and 8 run
while the NAS is not mounted; step 6's `index` runs unattended once it
is.

## Decisions

(by time, as they are made)

- **(18:35) Rust without `sudo`.** No Rust was installed and the prompt
  says rustup and its toolchains need no `sudo`, so rustup's own
  installer put it in `~/.rustup` and `~/.cargo` with
  `--no-modify-path`. Commands in this session put `~/.cargo/bin` first
  on `PATH`. The user is told how to add it to the shell.
- **(18:35) FFmpeg 8.1.2, as installed.** This machine's FFmpeg is the
  distribution's current one. It stays the FFmpeg of every run, since
  the session shows what Linux does with what it has. Any difference from
  the Mac in decoded audio is then a difference of FFmpeg's version and
  of the platform together; where it matters, a run with another FFmpeg
  separates the two, and is recorded as a diagnostic.
- **(18:40) `check.sh` with both toolchains.** Green on the first run with
  1.98.0 (3:39) and with `stable` 1.99.1 (4:11): formatting, clippy for
  Linux and for macOS (`aarch64-apple-darwin`), 466 Rust tests, 5 Node
  tests. The release binaries are built with 1.98.0, the Mac's version,
  so that the compiler is not a second difference (`work/bin/s32-start/`).
- **(18:50) Two corpus stores.** `index` from scratch extracted all 265
  files: the 3 the Mac's store passes over as damaged are accepted by the
  1% truncation tolerance (`53c6990`), which came after the Mac's skip
  notes. `work/peaks` keeps all 265: it is the store the CLI and the
  harness share, as step 4 asks. For the comparison with the Mac's 262
  tracks, `work/s32/peaks-262` is a copy of it without those 3 records
  and their tags (this machine's own records; nothing from
  `mac-reference/`), with its own work directory `work/s32/w262/`.
  `clusters` and `regress session-30-start` ran on both; `baseline
  session-32-linux` was saved from the default store (265), so that a
  later `regress session-32-linux` on this machine needs no option.
- **(19:10) The cause of the peak differences.** A diagnostic build in a
  scratch worktree (`work/s32/diag/`, never committed) patched a copy of
  rustfft so that an environment variable chooses its code path, and
  indexed the corpus three times. The decoded samples could not be
  compared with the Mac's here; `scripts/analysis/decoded_digests.py`
  lists them for a Mac session to compare. No FFmpeg 9.0.2 was installed
  for a version comparison: building it with its assembly needs `nasm`, a
  system package, and the comparison would not separate the platform.
- **(19:20) The listening tool.** Questions from the corpus report of the
  Stakka mix (`work/s32/corpus-reports/stakka-corpus.json`; the NAS was not
  mounted yet), all files temporary (`/tmp/gf-s32-audition/`, verdicts
  copied there). Everything answered over HTTP as on the Mac
  (`work/s32/audition-check.txt`). An audio request by a question's
  display name gets 403 by design: the page asks for each piece's
  absolute path, which the server lists. `xdg-open` is present; the
  default browser here is Brave, a Chromium-based browser.
