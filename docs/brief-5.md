# Session 5 brief: measurements at the size of the NAS collection

Written by the agent for itself on 2026-10-07, so the plan survives context
compaction. The owner is away and has left the machine idle for about 8
hours. Work autonomously and never stop to ask. When something is
ambiguous, take the conservative option, write the decision down and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-5-checklist.md` (progress).
3. `docs/notes-for-owner.md` (findings to report).
4. `docs/status.md` (newest entries at the bottom).
5. The newest file in `docs/experiments/`.
6. `docs/brief-nas.md`, `docs/nas-plan.md` and `docs/nas-checklist.md`.

`docs/brief-2.md`, `docs/brief-3.md` and `docs/brief-4.md` hold rules
that still apply.

## The owner's prompt, verbatim

```text
Start session 5 of Gunfinger: measurements at the size of the owner's NAS
collection, from its peak store alone. The owner is away and has left the
machine idle for about 8 hours. Work autonomously and never stop to ask. When
something is ambiguous, take the conservative option, write the decision
down and continue.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it), docs/brief-4.md,
   docs/brief-3.md and docs/brief-2.md (their rules still hold),
   docs/brief-nas.md, docs/nas-plan.md, docs/nas-checklist.md,
   docs/notes-for-owner.md (session 4 summary and "After session 4"),
   docs/status.md (newest entries at the bottom), docs/roadmap.md,
   docs/calibration.md and the newest experiment.
2. Write docs/brief-5.md: this prompt verbatim, the state at the start and
   the plan with time estimates. Write docs/session-5-checklist.md with the
   items below. Point CLAUDE.md's reading list at brief-5 and the session-5
   checklist (keep the NAS docs entry). Commit.
3. Build with --release. Run `gunfinger-eval regress session-4-start`
   (detections must be identical), then `gunfinger-eval baseline
   session-5-start`.

The NAS collection (facts for the brief):
- Library /Volumes/atlas/Music/dnb, configuration
  ~/.config/gunfinger/nas-dnb.toml (track length 1:30 to 15:00, playback
  turntable), peak store ~/.local/share/gunfinger/nas-dnb-peaks on the
  internal disk. The store names its library in library.txt and holds tags
  for every record.
- The owner's index run (Wi-Fi, 10 jobs): 28,439 audio files in 17,317 s;
  26,890 extracted, 108 too short, 1,285 too long, 156 failed. 120 of the
  failures are damaged YouTube downloads being fetched again elsewhere, 27
  are MP3s inside a WAV container behind an ID3 tag that FFmpeg cannot open,
  6 are damaged only in their last 2 s, 2 are truncated and 1 is a leftover
  partial download. A second `index` run added the tags in 490 s.
- Store: 26,890 records, 2,806 h of audio, mean 6.3 min, 1.9 GB; 82% of
  the 32,768-asset limit. 90% are YouTube downloads (AAC or Opus around 128
  kbit/s) from eight archive channels; 2,597 come from other folders (scene
  and vinyl rips; every corpus file is a copy of one of them). 103 files are
  exact copies of another (identical peaks); about 1,500 artist-and-title
  names appear on two or more files, mostly across channels (an estimate
  from the tags).

Rules (brief-2's, brief-3's and brief-4's rules still hold):
- The NAS may be unmounted for the whole session. Never read /Volumes/atlas.
  Everything comes from the NAS peak store (records and tags) and corpus/.
  If a step seems to need NAS audio, leave that part out and say so.
- The NAS store is read-only and frozen. Never run `gunfinger index` or
  `prune` against it, and write nothing into it. At the start, record its
  file counts and the library revision of its current records; check them
  again at the end. If they change during the session, stop measuring and
  report.
- Use `gunfinger-eval --other-peaks-dir ~/.local/share/gunfinger/nas-dnb-peaks`
  (docs/nas-plan.md, Commands). Its map, clusters and reports go to
  work/reports/library-nas-dnb-peaks/. For the CLI on the NAS store, pass
  every setting as a flag or use --config /dev/null; do not edit the
  owner's configuration files.
- Ground truth does not change this session. Never edit a manifest; add no
  references. Clusters come from stored peaks, never from set results.
  Every confident detection at NAS scale that the clusters do not explain
  counts as wrong under the rules, and goes to the owner by mix time,
  file path and tags, to check by ear. Pairs between 40% and 80% coverage go
  to the owner too.
- Test set: spend no evaluations (3 left).
- Default detection must not change. Only harness and analysis code may be
  added; `regress session-5-start` must stay identical after every code
  change. Never change or re-extract work/peaks.
- Measure under both today's matcher and skip at 240 (`--second-pass
  --skip-fullest 0.01`): the owner has not chosen between them.
- Timing: the machine is idle. One timed run at a time; record wall time,
  CPU time, peak memory (`/usr/bin/time -l`) and the load average.
- Save every measurement immediately as docs/experiments/NNNN-slug.md (at
  most 40 lines; numbering continues after the newest), with data in
  docs/experiments/data/. Data may name NAS files and their tags; it holds
  no audio. Put findings into docs/notes-for-owner.md as soon as you have
  them, and add entries to docs/status.md.
- corpus/ is private, copyrighted audio: read-only, never commit any of it.
  Commit each piece once scripts/check.sh is green, and tick it in the
  checklist. Never `git add -A` without checking. No remotes, no pushing,
  nothing published. No AI attribution in commits. No em dashes anywhere.

Time: plan for about 8 hours. My estimates, to be checked: an hour per sweep
seed under today's matcher and a quarter of that under skip at 240 (from
session 4's 0.21 and 0.055 s of CPU per asset for the 56-minute mix), about
15 and 5 minutes per development scan, under an hour for the clusters.
Measure the first run of each kind, re-plan, and keep the order below so the
most informative results land first. Finish and commit what you start. At
the end the checklist says what was done, what was not, and why.

Items (docs/nas-plan.md steps in brackets):
1. The store as indexed [1, 2]. Indexing throughput over Wi-Fi from the
   records' source sizes and the 17,317 s, against experiment 0028's figures
   for the local disk. Composition: channels and folders, formats, lengths,
   tag coverage, exact copies, names on several files. `doctor` and `stats`
   on the store alone.
2. The content map [3]: `map-library`. Corpus files with no copy or several,
   for the owner.
3. Clusters of the corpus recordings at NAS scale [4]: `clusters
   --from-peaks`. Do the 17 corpus clusters reappear? Further rips of corpus
   recordings, their coverage and hits; borderline pairs for the owner.
4. Related recordings at NAS scale [5], from the pairs item 3 keeps (30 hits
   or more): which reach the possible tier, 200 or 240 hits; same artist or
   label against unrelated, from tags and paths. Include Bad Company's China
   Cup and The Nine (they share their drums, owner by ear) and Fresh & Vegas
   - Mekon if the store holds it.
5. The development set at NAS scale [6], both matchers: the development
   scan, leave-outs 3 and 11. Identified, wrong, strongest false candidate
   and margin, possible plays, wall and CPU time, peak memory, against the
   262-track figures.
6. Sweeps at NAS scale [7] on the saved panels: seeds 2026-2029 under skip
   at 240, seeds 2026 and 2027 under today's matcher; then `calibrate` for
   each matcher. Weakest identifying detection, strongest false candidate
   and margin against experiment 0025 (4.15x today, 5.53x skip at 240 over
   four draws at 262 tracks). Name the strongest false candidates.
7. The proxy against reality [8]: `memory` at NAS scale under
   `/usr/bin/time -l`, both matchers; compare chance, search CPU and memory
   with what the synthetic copies predicted at 26,462 assets (experiments
   0012, 0019, 0027-0029). If they differ much, say which predictions not to
   trust for the next size step.
8. Posting lists and famous breaks [9]: postings per second, bytes per
   posting and bucket p99 for the store; which records hold most of the
   fullest 1% of buckets; false candidates per track with and without skip,
   from items 5 and 6. Does skip at 240 keep break-heavy records out of the
   candidates at this size?
9. Identifying without the NAS [10]: time and peak memory of the store-only
   index build at 26,890 records from the internal disk (and from a copy on
   another disk only if one is attached); `identify --store-only` on the
   development mix, its tracklist naming tracks from the stored tags.
10. Wrap-up: a session 5 summary at the top of docs/notes-for-owner.md with
    what the NAS-scale figures mean for the open decisions (matcher, link
    rules, band profile). In docs/calibration.md, add the NAS-scale figures
    as measurements without changing any adopted rule. Update the roadmap,
    status, docs/nas-checklist.md and docs/nas-plan.md.

Only if time remains, in this order:
11. Today's matcher, sweep seeds 2028 and 2029 at NAS scale, and calibrate
    again.
12. The link rules (`--nearby-rungs --strong-gaps`) at NAS scale: the
    development scan and sweep seed 2026, both matchers.
13. A scaling curve on real records: a harness option that adds a seeded
    random subset of the other library (for example 3,000 and 9,000
    records, then all), with `regress` identical; the development scan and
    sweep seed 2026 under skip at 240 at each size.
```

## State at the start of session 5

- Commit `44ddfd2` on `master`. Experiments 0001-0034; ADRs 0001-0007.
- Default matcher: speed-ladder pair hashes, lines per 10 s window,
  chains, plays; confident 200 hits in 3 windows, possible 60; both
  playbacks (82 rungs). Opt-in candidate, not adopted: skip at 240
  (`--second-pass --skip-fullest 0.01`, `FITTED_RULE`). Opt-in, not
  adopted: link rules, band profile, extra rungs, speed per stretch.
- At 262 tracks: development set 11/11, 0 wrong; leave-outs 0 wrong;
  sweeps 2026-2029 540/540 each; margins over four draws 4.15× (today's)
  and 5.53× (skip at 240). Test set 15/16, 0 wrong; 2 of 5 evaluations
  used, 3 left.
- Harness against another library merged (`--other-peaks-dir`,
  `map-library`, `clusters --from-peaks`); no NAS-scale report exists
  yet (`work/reports/library-nas-dnb-peaks/` is absent).
- The NAS store at the start (frozen for this session): 26,890 `.peaks`,
  26,890 `.tags`, 1,549 `.skip` (108 + 1,285 + 156), `library.txt`
  naming `/Volumes/atlas/Music/dnb`; 1.9 GB. A digest of every file's
  name, size and mtime (`stat -f '%N %z %m'`, sorted, SHA-256):
  `7055ad8e3b22b12b...465a70` (full value in `docs/status.md`). The
  library revision of its current records is recorded from the first
  store-only `identify` report.
- `/Volumes/atlas` is mounted (SMB), but this session never reads it. No
  local external disk is attached (`/Volumes` holds only SMB shares), so
  item 9 measures the internal disk alone.
- Machine: Apple M1 Pro, 10 cores, 32 GB; load average 2.3 and falling;
  swap 2.1 GB used of 3 GB (left over); internal disk 110 GB free.

## Plan, with time estimates

Every heavy run goes through `work/scripts/timed.sh` (`/usr/bin/time -l`,
`caffeinate -i`, a time limit, load average and swap logged), one at a
time. While one runs, the agent does only light work (reading reports,
Python summaries, writing). `E` is `target/release/gunfinger-eval
--other-peaks-dir ~/.local/share/gunfinger/nas-dnb-peaks`.

| Step | What | Estimate |
|---|---|--:|
| Setup | brief, checklist, release build, `regress session-4-start`, `baseline session-5-start` | 0:20 |
| 1 | throughput from record sizes; composition from headers and tags (a script reading the store, no audio); exact copies from identical peaks; `doctor`, `stats` with `--config /dev/null --peaks-dir` | 0:35 |
| 2 | `$E map-library` | 0:10 |
| 3 | `$E clusters --from-peaks` (timed) | 0:50 |
| 4 | related pairs from item 3's census, labelled from tags and paths | 0:30, beside item 5 |
| 5 | `$E scan` dev, leave-outs 3 and 11: skip at 240 first (3 × 5 min), then today's (3 × 15 min) | 1:00 |
| 6 | sweeps: skip at 240 seeds 2026-2029 (4 × 15 min), today's 2026, 2027 (2 × 60 min), `calibrate` for each | 3:00 |
| 7 | `$E memory` under `time -l`, both matchers | 0:20 |
| 8 | `stats` on the store; which records hold the fullest 1% (a harness report or a script); false candidates per track from 5 and 6 | 0:30, beside 6 |
| 9 | `identify --store-only` on the development mix (timed); the index build alone | 0:20 |
| 10 | notes, calibration, roadmap, status, NAS docs | 0:30 |

About 8 hours in all. The first run of each kind is measured and the plan
re-estimated in `docs/status.md`. Items 4 and 8 are analysis and run
beside the heavy runs of items 5 and 6. Items 11-13 only if time remains.

Conservative decisions taken up front:

- The NAS mount is never touched; the CLI on the NAS store runs with
  `--config /dev/null --peaks-dir <store>` and no `--library`, so it
  searches the store's own records (store-only).
- If the store's file counts or digest change during the session, all
  measuring stops and the change is reported.
- Analysis scripts that read the store live in `scripts/analysis/`;
  they open files read-only.
- Today's matcher runs the sweeps only for 2026 and 2027 unless time
  remains (item 11).
