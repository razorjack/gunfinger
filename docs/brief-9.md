# Session 9 brief: past 32,768 assets, and the NAS over Ethernet

Written by the agent for itself on 2026-10-09, so the plan survives context
compaction. Work autonomously and never stop to ask. When something is
ambiguous, take the conservative option, write the decision down and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-9-checklist.md` (progress).
3. `docs/notes-for-owner.md` ("After session 7" to the end).
4. `docs/status.md` (newest entries at the bottom).
5. The newest file in `docs/experiments/`.
6. `docs/adr/0005-index-layout.md`, `docs/adr/0007-library-size-beyond-32768-assets.md`.

`docs/brief-2.md` to `docs/brief-8.md` hold rules that still apply unless
this brief changes them.

## The owner's prompt, verbatim

```text
Start session 9 of Gunfinger: past 32,768 assets, and the NAS checked
over Ethernet. The owner indexed new uploads, scene releases and label
packs: the NAS peak store now holds 32,905 records, more than the
32,768 assets a posting can address, so every NAS-scale command stops
with "the index holds at most 32768 assets". The laptop is on Ethernet
with the NAS mounted at /Volumes/atlas. Work autonomously and never stop
to ask. When something is ambiguous, take the conservative option, write
the decision down and continue. Expected length: about 3 hours.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it),
   docs/brief-8.md (brief-2 to brief-7 hold rules that still apply
   unless this prompt changes them), docs/notes-for-owner.md (from
   "After session 7" to the end), docs/status.md (newest entries at the
   bottom), docs/adr/0005-index-layout.md,
   docs/adr/0007-library-size-beyond-32768-assets.md, docs/roadmap.md
   ("More than 32,768 assets", "More development mixes") and experiments
   0035, 0046, 0050 and 0060.
2. `git status` should be clean at 9fda7db. If it is not, leave those
   changes alone and list them in the notes.
3. Write docs/brief-9.md (this prompt verbatim, the state at the start,
   the plan with times) and docs/session-9-checklist.md. Point
   CLAUDE.md's reading list at them; keep brief-8 among the older
   briefs. Commit.
4. Build with --release. Run `gunfinger-eval baseline session-9-start`.
   Record the NAS peak store's state with
   scripts/analysis/store_revision.py and store_census.py: 32,905
   records, 1,565 skip notes and revision b8bb402f0ba23761 expected,
   and the longest record.

Rules for this session:
- The NAS (/Volumes/atlas/Music/dnb) is read, never written.
- The NAS peak store (~/.local/share/gunfinger/nas-dnb-peaks) is
  read-only: no `index` or `prune` against it, so its revision stays
  b8bb402f0ba23761 for this session and the next. Never use `index
  --retry-skipped` on it: that also decodes again every file passed
  over as too long (more than 1,286 mixes). Report files that are not
  indexed or failed, for the owner.
- Never touch the held-out test set (ed-rush-optical-essential-mix).
- Detection at 262 tracks does not change: `regress session-9-start`
  must be identical.
- Do not run `identify` on any mix found on the NAS: a mix's role
  (development or held-out) is agreed with the owner before anyone looks
  at Gunfinger's results on it.
- Out of scope, for an overnight session: the clusters' verdict links,
  clusters runs, sweeps and the NAS-scale protocol. Harness scoring at
  NAS scale waits for the next clusters run, because the new folders
  hold rips the clusters do not know yet.
- The owner may use the machine: record wall time, CPU time and peak
  memory, and compare CPU only within one run.
- Each measurement is an experiment (docs/experiments/NNNN-slug.md, at
  most 40 lines; data with file names only in docs/experiments/data/).
  Commit after each step; check `git status` before every commit and
  never use `git add -A`.

Items, in order:

1. Reproduce the limit (5 min): `target/release/gunfinger stats --config
   ~/.config/gunfinger/nas-dnb.toml` should stop with "the index holds
   at most 32768 assets". Record the message and how long it took to
   fail.

2. 65,536 assets with 16 frame bits (about 1 h). This is ADR 0007's
   option E at full resolution: postings keep 4 bytes and 16 ms frames;
   16 frame bits address tracks up to 17:28 and 16 asset bits address
   65,536 assets.
   - FRAME_BITS becomes 16 in gunfinger-core's index.rs. The default
     track limit (DEFAULT_MAX_TRACK in the CLI's config.rs, 20:00 today;
     the brief marks it [DEFAULT]) becomes 17:00, below the 17:28 the
     frames address. Searches already leave out records longer than
     --max-track; check that a record the frames cannot address gives
     the actionable TooLong error, never a wrong posting. Update every
     message, help text and doc comment that says 20 minutes, 17 frame
     bits or 32,768 assets (README, the configuration docs, doctor's
     share line).
   - Tests: a posting round trip at the largest asset and frame; the
     65,536th asset accepted and the next refused; a record past 17:28
     refused.
   - docs/adr/0010-65536-assets.md supersedes ADR 0005's 17/15 split
     (say so in 0005's status) and records the step in ADR 0007: option
     E now, shards (option A) once a library passes 65,536 assets. State
     the cost: tracks between 17:00 and 20:00 are no longer indexed by
     default (the corpus's longest is 9.6 minutes and the NAS
     configuration stops at 15:00, so neither changes).
   - scripts/check.sh green; `regress session-9-start` identical.

3. The NAS store at 32,905 records (30-40 min, store-only, no scoring):
   - `stats` with the NAS configuration: index build time, peak memory,
     postings, bytes per posting and bucket p99, against experiment 0050
     (27,042 records).
   - `identify` of the development mix with experiment 0050's command,
     and of razorjack-2003-03-29 with experiment 0060's (both
     `--store-only --playback both`). Compare each track with 0050 and
     0060: are the tracks the same? Which files from the new folders
     appear, and are they other rips of the same recordings (same artist
     and title) or something else? Wall, CPU, peak memory.

4. The NAS over Ethernet (30 min):
   - `doctor --config ~/.config/gunfinger/nas-dnb.toml`: the listing
     time against session 8's 2:21 over Wi-Fi; files not indexed,
     failed, too short and too long; records of files gone from the NAS
     (the owner may have moved folders). Report; do not prune.
   - Throughput and repeatability: pick about 300 audio files by seed
     from several new folders (scene releases, label packs, YouTube
     uploads). Index those folders as libraries of their own into
     scratch stores, without the NAS configuration so nothing can reach
     the NAS store: `gunfinger --config /dev/null --peaks-dir
     work/s9-ethernet/<n> --min-track 1:30 --max-track 15:00 index
     <folder>`, timed. Report Mbit/s and seconds per file against
     experiment 0035 (84 Mbit/s over Wi-Fi, 423 Mbit/s from the local
     disk), and estimate a full re-index of the NAS over Ethernet.
     Compare the scratch records' peaks with the NAS store's records of
     the same files (the paths in the headers differ by the folder
     prefix): they should be identical. Check the NAS store's revision
     afterwards.
   - The files that failed to decode: try them with ffprobe and report
     whether they are damaged.

5. Candidate mixes on the NAS (15 min, no identify): the files passed
   over as too long (over 15:00) are mixes and album rips. List them in
   work/ (path, length, folder) and summarise them in the notes: how
   many, their lengths, and which folders look like DJ mixes of records
   the collection may hold, as candidates for development and held-out
   sets. Also list new folders that look like mixed CDs (names or tags)
   as candidates for the owner's .gunfingerignore; do not change that
   file.

6. Wrap-up (20 min): experiment files; a "Session 9" section at the end
   of docs/notes-for-owner.md, leading with what the owner must decide;
   docs/status.md, the checklist, the calibration register (the asset
   limit; NAS-scale costs at 32,905 records), and the roadmap ("More
   than 32,768 assets", now about the next limit). Commit. Last, print
   for the owner what was done, the numbers, the decisions waiting for
   them, and what the overnight session should do next.
```

## State at the start of session 9

- 20:28. `git status` clean at `9fda7db` (six commits ahead of
  `origin/master`; nothing is pushed). Experiments 0001-0061; ADRs
  0001-0009. Test set: 3 of 5 evaluations used.
- The release build of `9fda7db` is copied to `work/bin/s9-start/`.
- The machine: 10 cores, 32 GB, macOS 27.0.1. The default route is the
  Thunderbolt Ethernet adapter (`en7`, 1000baseT full duplex). The NAS
  is mounted over SMB at `/Volumes/atlas` (935 entries at
  `/Volumes/atlas/Music/dnb`).
- The NAS store holds 32,905 `.peaks`, 32,905 `.tags`, 1,565 `.skip` and
  `library.txt`. Session 8 ended at 27,042 records and 1,396 notes, so
  the owner has indexed new folders and probably pruned the 33 ignored
  files.
- What the code does today: `Posting` packs 17 frame bits and 15 asset
  bits (`MAX_FRAMES` 131,072, `MAX_ASSETS` 32,768). `Counting::count`
  refuses the 32,769th record with `IndexError::TooManyAssets` ("the
  index holds at most 32768 assets"); `points_of` refuses a record whose
  last peak's rounded frame is at or past `MAX_FRAMES` with
  `IndexError::TooLong` ("... lower --max-track"). The CLI's
  `DEFAULT_MAX_TRACK` is 20:00. `doctor`'s "index limits" section has
  the share line ("N of 32768 assets") and a line comparing the longest
  track with the length the frames address.

## Plan, with times

Start 20:28; about 3 hours.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| Setup | brief, checklist, `CLAUDE.md`, commit; `baseline session-9-start`; store revision and census | 0:20 | 20:50 |
| 1 | `stats` with the NAS configuration, the error and its time | 0:05 | 20:55 |
| 2 | 16 frame bits, 17:00 default, messages and docs, tests, ADR 0010, check, regress, commit | 1:00 | 21:55 |
| 3 | `stats` and two store-only `identify` runs at 32,905 records | 0:35 | 22:30 |
| 4 | `doctor` over Ethernet; ~300 files into scratch stores; peaks compared; failed files with `ffprobe` | 0:30 | 23:00 |
| 5 | too-long files listed with lengths; mixed-CD candidates | 0:15 | 23:15 |
| 6 | notes, status, checklist, calibration, roadmap, commit, the message | 0:20 | 23:35 |

Conservative decisions taken up front:

- A record the frames cannot address stays an error of the whole
  command (`IndexError::TooLong`), as today. Its message gains the
  length the index addresses, so the user knows what `--max-track` to
  set. `--max-track` above 17:28 is not refused at configuration time:
  `index` stores peak records of any length, and only the in-memory
  index is limited; `doctor` already flags a longest track beyond the
  addressable length.
- New records are told from old ones by their files' modification time
  in the store (newer than session 8's last commit, 17:24:55), not by
  comparing with a list from session 8, which was never saved.
- Throughput runs read the NAS through the SMB client, which may hold
  recently read files in memory. The bytes received on `en7` during each
  run are recorded beside the bytes of the files, to show whether the
  files came over the wire.
