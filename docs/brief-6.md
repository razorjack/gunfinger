# Session 6 brief: store fixes, the new default matcher, clusters for fast uploads, grouped tracklists

Written by the agent for itself on 2026-10-08, so the plan survives context
compaction. The owner is away and has left the machine idle for about 8
hours. Work autonomously and never stop to ask. When something is
ambiguous, take the conservative option, write the decision down and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-6-checklist.md` (progress).
3. `docs/notes-for-owner.md` (findings to report).
4. `docs/status.md` (newest entries at the bottom).
5. The newest file in `docs/experiments/`.
6. `docs/brief-nas.md`, `docs/nas-plan.md` and `docs/nas-checklist.md`.

`docs/brief-2.md` to `docs/brief-5.md` hold rules that still apply unless
this brief changes them.

## The owner's prompt, verbatim

```text
Start session 6 of Gunfinger: the store fixes, the new default matcher,
clusters that join fast uploads, the NAS-scale protocol under both, and
grouped tracklists. The owner is away and has left the machine idle for
about 8 hours. Work autonomously and never stop to ask. When something is
ambiguous, take the conservative option, write the decision down and
continue.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it), docs/brief-5.md,
   docs/brief-4.md, docs/brief-3.md and docs/brief-2.md (their rules hold
   unless this prompt changes them), docs/brief-nas.md, docs/nas-plan.md,
   docs/nas-checklist.md, docs/notes-for-owner.md (session 5 summary and
   findings), docs/status.md (newest entries at the bottom), docs/roadmap.md,
   docs/calibration.md and experiments 0039, 0040, 0044 and 0045.
2. Write docs/brief-6.md: this prompt verbatim, the state at the start and
   the plan with time estimates. Write docs/session-6-checklist.md with the
   items below. Point CLAUDE.md's reading list at brief-6 and the session-6
   checklist (keep brief-5 among the older briefs and the NAS docs entry).
   Commit.
3. Build with --release. Run `gunfinger-eval regress session-5-start`
   (detections must be identical), then `gunfinger-eval baseline
   session-6-start`.

The owner's decisions (not proposals; implement them):
- Adopt skip at 240 with the link rules as the default matcher:
  `--second-pass --skip-fullest 0.01 --nearby-rungs --strong-gaps`, rule
  `FITTED_RULE` (240 hits in 3 windows), possible tier 60 unchanged.
- Widen the clustering search to the matcher's speed range (±8%), so
  uploads that play a few percent fast can join their recordings.
- Group plays in the tracklist, in this format:
  ` 1.    0:00  STAKKA and SKYNET - knight lore  (also: Stakka And Skynet - Nightlore)`
- Three store fixes: MP3s inside a WAV (RIFF) container behind an ID3v2
  tag; skip hidden folders; a truncation tolerance of about 1%.
- The possible tier stays at 60 this session; measure and propose only.

Facts for the brief:
- HEAD 3a3915a, experiments 0001-0045, ADRs 0001-0007 (docs/adr). Test
  set: 3 evaluations left.
- NAS library /Volumes/atlas/Music/dnb (SMB, mounted at the start),
  configuration ~/.config/gunfinger/nas-dnb.toml (track length 1:30 to
  15:00), peak store ~/.local/share/gunfinger/nas-dnb-peaks on the internal
  disk. After session 5 the owner re-downloaded the 120 damaged YouTube
  files and indexed them: the store now has 27,010 `.peaks`, 27,010 `.tags`
  and 1,429 `.skip`, so its revision differs from session 5's
  d709390272e41475.
- The remaining failures (work/logs/nas-dnb-failures.tsv lists the first
  run's 156; verify against the skip notes): 27 MP3s in a RIFF container
  behind an ID3v2 tag ("Invalid data found when processing input"; FFmpeg
  opens them with `-skip_initial_bytes <tag size>`), 6 damaged only in
  their last 1.3-2.4 s (rejected by `reject_if_truncated`'s 1 s tolerance
  in crates/gunfinger-core/src/decode.rs), 2 truncated by 16.2 and 21.2 s
  (the owner replaces them; they must still fail), and 1 partial download
  in `__youtube_archivists/unxplored-dnb/.incomplete/`, the only entry in a
  hidden folder. Four of the 27 are NAS copies of corpus files (SKC & Cord
  - Swarm, SKC - Recharger (Black Sun Empire remix), Usual Suspects -
  Bleach, Cause 4 Concern - Give It 2 Em; experiment 0036).
- library.rs already skips hidden files, not hidden folders.
- The tracklist (crates/gunfinger/src/export.rs) already merges plays of
  different records into one entry, but only within 0.5% of speed
  (`SAME_RECORDING_SPEED`), at the same place in the track; the fast
  uploads differ by 2.7-4.7%, so the development mix from the NAS store
  gives 20 lines for 11 tracks and 2 possible plays (experiment 0044).

Rules (the older briefs' rules hold unless changed here):
- The NAS is read only in item 3, read-only, through `gunfinger index` and
  `prune`'s listing. After item 3, never read /Volumes/atlas again. If it
  is not mounted or cannot be listed when item 3 starts, skip item 3, say so
  and continue on the store as it is.
- The NAS store is written only in item 3. Record its file counts, a digest
  (as in brief 5) and its revision before and after item 3; after it, the
  store is frozen. If it changes later, stop measuring and report.
- Default detection changes only in item 4, through the full protocol.
  Every other code change must keep `regress` identical (store fixes,
  harness options, display changes). Never change or re-extract work/peaks;
  if a store fix would make a corpus file indexable that is not today,
  report it and do not index it.
- Ground truth: the clusters change in item 5, from library audio or
  stored peaks only, never from set results. Never edit a manifest; add no
  references. The owner's verdict file (item 5) is created empty; never
  write a verdict into it.
- Test set: spend no evaluations (3 left). Say in the notes what the next
  one should measure.
- For the CLI, pass every setting as a flag or use --config /dev/null;
  item 3 may read ~/.config/gunfinger/nas-dnb.toml. Edit no configuration
  file. Do not install or replace the owner's installed gunfinger binary.
- Timing: the machine is idle. Heavy runs one at a time through a timed
  script (`/usr/bin/time -l`, `caffeinate -i`, a time limit, load average
  and swap logged). While a timed run goes, do only light work (reading,
  writing, Python); building and tests wait.
- Save every measurement immediately as docs/experiments/NNNN-slug.md (at
  most 40 lines, numbering continues at 0046), with data in
  docs/experiments/data/ (file names and tags only, never audio). Put
  findings into docs/notes-for-owner.md as soon as you have them, and add
  entries to docs/status.md.
- corpus/ is private, copyrighted audio: read-only, never commit any of it.
  Test fixtures are generated (tones, noise), never copied from the corpus
  or the NAS. Commit each piece once scripts/check.sh is green, and tick
  it in the checklist. Never `git add -A` without checking. No remotes, no
  pushing, nothing published. No AI attribution in commits. No em dashes
  anywhere.

Time: about 8 hours. My estimates, to be checked: at 262 tracks a sweep
seed takes minutes; at NAS scale, under skip at 240 with the link rules, a
sweep seed takes about 23 minutes, the development scan about 5 and each
leave-out about 5. Session 5's clusters run took 72 minutes on 11 rungs
(0.98-1.02); ±8% has about 4 times the rungs per playback, so measure on
a sample before the full run. Measure the first run of each kind,
re-plan in docs/status.md, and keep the order below. Finish and commit
what you start. Keep the last 40 minutes for item 9. At the end the
checklist says what was done, what was not, and why.

Items:
1. Store fixes in code (estimate 0:50).
   a. MP3 in RIFF behind ID3v2: detect the case (ID3v2 header, then
      "RIFF" right after the tag, including a footer if flagged) and
      decode and probe with the tag skipped. Keep the files' tags: read
      them from the ID3 tag (test what works, for example ffprobe with
      `-f mp3`, or parsing the tag), so the tracklist can name them.
      Normal files must take exactly today's path.
   b. Skip folders whose name starts with a dot, counted as "hidden
      folder" in the library's skipped files.
   c. `reject_if_truncated`: accept a shortfall up to the larger of 1 s
      and 1% of the expected length. Check against the 6 files' and the 2
      truncated files' real lengths (read-only probes of those NAS files
      are allowed here) that the 6 pass and the 2 still fail; if 1% does
      not cover all 6, keep 1% and report.
   Unit tests with generated fixtures for each. `regress session-6-start`
   identical. README and doctor wording where they describe skipping or
   truncation.
2. (Reserved number; nothing here. Keep the numbering.)
3. Re-index the NAS store once (estimate 0:25, needs the NAS). With the
   release build: `gunfinger --config ~/.config/gunfinger/nas-dnb.toml
   index --retry-skipped`. Then `prune` without --yes; only if it lists
   exactly the stale `.incomplete` skip note, run `prune --yes` (never
   --force). Expect about 33 new records and 1,394 skip notes; explain any
   other difference. Record counts, digest and revision; the store is
   frozen from here. Rerun `map-library` (the four corpus copies may now
   map) and note the NAS-scale asset count.
4. The new default matcher (estimate 1:15). Before changing code, save
   `gunfinger-eval --second-pass --skip-fullest 0.01 --nearby-rungs
   --strong-gaps baseline session-6-candidate`. Make that combination the
   default in `identify`, `explain` and the harness; keep today's matcher
   reachable through one opt-in flag, with its reports in a variant
   directory. After the change, plain `regress session-6-candidate` must be
   identical, and `regress session-6-start` must show only differences
   experiments 0026 and 0030 predict (more hits on true plays, far fewer
   weak false candidates, mix 10's Dominion confident to possible, the
   grid's 10 s plays confident to possible); explain every other one. Full
   protocol at 262 tracks under the new default: sweeps 2026-2029, the
   development scan, leave-outs 3 and 11, calibrate, `mixes --count 12`,
   `grid`. Update the calibration register (adopted row), README, e2e
   tests, and write ADR 0008. Note in the notes that Sick Note (209 hits
   today on the test set) most likely becomes possible.
5. Clusters that join fast uploads (estimate 2:00).
   a. First the 6 development pairs of experiment 0039 (uploads 2.9-4.7%
      faster than the corpus rips): they align at 0.970-0.972 but cover
      only 44-55%. Find out why: the speed step (0.4% rungs drift over
      several minutes), peaks rescaled by `search_peaks`, edits or fades.
      Save it as an experiment.
   b. Widen `clusters` to the matcher's ±8% (both playbacks if the
      uploads need it) and refine each pair's speed before measuring
      coverage (for example the second pass at the fitted speed). Keep the
      80% rule and the 40% borderline as they are; report the new gap
      between the weakest same-recording pair and the strongest different
      pair. Pick a design whose NAS run fits about 1.5 h (measure on a
      sample of queries first); skipping the fullest lists for candidates
      is allowed.
   c. An owner verdict file for pairs (same recording / different),
      plain text, read by `clusters` and overriding the coverage rule;
      create it empty with its format explained in a header comment.
   d. Run at 262 tracks: the 17 corpus clusters must reappear; if any
      changes, report it and rerun item 4's sweeps and calibrate at 262.
      Then `clusters --from-peaks` at NAS scale, timed. Report which of
      the 6 development wrong identifications and of session 5's false
      confident sweep detections (0040, 0041) the new clusters explain
      (re-score session 5's reports if the harness allows), the new
      borderline pairs, and any new cluster that joins different titles.
6. The NAS-scale protocol under the new default and the new clusters
   (estimate 1:15): the development scan, leave-outs 3 and 11, sweeps 2026
   and 2027, calibrate. Against 0039, 0040 and 0045: identified, wrong,
   weakest identifying, strongest false and margin, strongest unrelated,
   cost. End with an updated listening list for the owner in the notes:
   the pairs from session 5 that the clusters still do not explain (the 6
   development pairs if still open, Synthesis VIP ~ "Synthesis (Remix)",
   Coma ~ Spraycan, The Nine ~ its Evol Intent VIP) and the new borderline
   pairs, by file path, tags, hits and mix time.
7. Grouped tracklists (estimate 0:40; display only). In
   crates/gunfinger/src/export.rs, plays of different records over the
   same stretch of the mix are one entry even at different speeds: compare
   the places in the track after correcting for each record's speed,
   instead of requiring speeds within 0.5%. A possible play that shares
   material with another play (the Clockwork remix's passage, Star Trails'
   Synergy remix) must stay its own entry, and so must two tracks that
   overlap only in a blend. The entry is named after its strongest play;
   other distinct names follow as `  (also: name, name)` (compare names
   case-insensitively; leave out repeats). The cue sheet uses the same
   entries. Unit tests; JSON output unchanged. Check with
   `identify --store-only` on the development mix against the NAS store
   (one timed run): 13 lines for 11 tracks and 2 possible plays expected;
   explain any other count. Update the README's tracklist example and the
   roadmap item.
8. A scaling curve on real records (estimate 1:00): a harness option
   that adds a seeded random subset of the other library (1,000, 3,000
   and 9,000 records; all of it is item 6), `regress` identical without
   it. The development scan and sweep seed 2026 at each size under the
   new default: strongest unrelated chance, false candidates, wall, CPU
   and peak memory against size. Propose a possible tier as a function of
   index size (or postings) with the evidence; do not change the tier.
9. Wrap-up (always, the last 40 minutes): a session 6 summary at the top
   of docs/notes-for-owner.md (what changed for users, the new default's
   cost and evidence at both scales, what the clusters now explain, the
   listening list, the decisions left: the possible tier, the next test
   evaluation). Update docs/calibration.md, the roadmap, status, the NAS
   plan and checklist, and the session-6 checklist. Check the NAS store
   against the record from item 3. scripts/check.sh green, commit.

Only if time remains, in this order:
10. NAS-scale sweeps 2028 and 2029 under the new default; calibrate over
    four seeds.
11. Today's matcher at NAS scale with the new clusters: the development
    scan and sweep seed 2026, for comparison with item 6.
```

## State at the start of session 6

- Commit `3a3915a` on `master`, working tree clean. Experiments
  0001-0045; ADRs 0001-0007. Test set: 2 of 5 evaluations used, 3 left.
- Default matcher: speed-ladder pair hashes, lines per 10 s window,
  chains, plays; confident 200 hits in 3 windows, possible 60; both
  playbacks (82 rungs). Opt-in: skip at 240 (`--second-pass
  --skip-fullest 0.01`, `FITTED_RULE`), the link rules (`--nearby-rungs
  --strong-gaps`) and the other session 4 variants.
- At 262 tracks: development 11/11, 0 wrong; leave-outs 0 wrong; sweeps
  2026-2029 540/540 each; margins over four draws 4.15× (today's), 5.53×
  (skip at 240), 6.65× (skip at 240 with the link rules, seed 2026 and
  scans; experiment 0030).
- At NAS scale (session 5, 26,914 assets): both matchers 11/11 with 6
  wrong (fast YouTube uploads outside the clustering ladder); skip at 240
  with the link rules: sweep 2026 540/540, 91 wrong (same-name uploads),
  development scan 273 s, 1,963 s CPU, 4.0 GB; sweep seed 1,351 s.
- `/Volumes/atlas` is mounted (SMB) at 02:45.
- The NAS store at the start (02:45): 27,010 `.peaks`, 27,010 `.tags`,
  1,429 `.skip`, `library.txt` (naming `/Volumes/atlas/Music/dnb`); 1.9
  GB. Digest (`cd <store>; find . -type f -exec stat -f '%N %z %m' {} + |
  sort | shasum -a 256`):
  `1a7acf1b37b0c67389fee082f9f079967e181f20a19a357ba3f32df407b38129`.
  Revision of its current records (`scripts/analysis/store_revision.py`):
  `3d16641d2ecb6956`.
- Machine: Apple M1 Pro, 10 cores, 32 GB; load average 4.4 and falling
  (15-minute 9.7); swap 2.0 GB used of 3 GB (left over); internal disk
  128 GB free.

## Plan, with time estimates

Start 02:45; end about 10:45. Item 9 starts no later than 10:05. Heavy
runs go through `work/scripts/timed6.sh` (session 5's `timed5.sh` with
its log names), one at a time; long ones start with `nohup` so they
survive the agent tool's limits. While one runs, only reading, writing
and Python; building and tests wait.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| Setup | brief, checklist, release build, `regress session-5-start`, `baseline session-6-start` | 0:25 | 03:10 |
| 1 | store fixes: RIFF behind ID3v2 (detect, skip, tags), hidden folders, 1% truncation; tests; probes of the 8 NAS files; `regress` | 0:50 | 04:00 |
| 3 | `index --retry-skipped` on the NAS, `prune` listing, record the store, `map-library` | 0:25 | 04:25 |
| 4 | baseline `session-6-candidate`; the default change; regress both; full protocol at 262 (4 sweeps, scan, 2 leave-outs, calibrate, mixes, grid); ADR 0008, register, README, e2e | 1:15 | 05:40 |
| 5 | the 6 pairs (experiment); `clusters` at ±8% with refined speed, verdict file; 262 run; NAS sample, then the full NAS run (about 1.5 h, timed) | 2:00 | 07:40 |
| 6 | NAS scan, 2 leave-outs (3 × 5 min), sweeps 2026, 2027 (2 × 23 min), calibrate; listening list | 1:15 | 08:55 |
| 7 | grouped tracklist (code written during item 5's and 6's runs), tests, one timed `identify --store-only` | 0:40 | 09:35 |
| 8 | subsets of the other library: 1,000, 3,000, 9,000 records; scan and sweep 2026 at each | 1:00 | 10:05 at best |
| 9 | wrap-up | 0:40 | 10:45 |

The estimates add up to 8:30 against 8:00 available, so item 8 is the
one most likely to be cut short; the code for item 7 is written while
item 5's NAS run goes, so item 7's slot is mostly its check run. Each
first run of a kind is measured and the plan re-estimated in
`docs/status.md`.

Conservative decisions taken up front:

- Store fixes change only files that fail today: a file without an ID3v2
  tag followed by "RIFF", in a visible folder and within 1 s of its
  declared length takes exactly today's path, so `work/peaks` and the
  corpus results do not move. Before item 3 the agent checks which
  corpus files would change: none should (every corpus file is indexed
  today), and none is re-extracted.
- `prune --yes` runs only if the listing is exactly the stale
  `.incomplete` skip note.
- The new default keeps today's matcher behind one flag in the CLI
  (`--matcher classic` or similar) and in the harness, whose reports for
  it go to a variant directory.
- The verdict file lives in the repository as a plain text file read by
  `clusters`, created empty with its format in a header comment; the
  agent never writes a verdict into it.
