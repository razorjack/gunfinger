# Session 11 brief: a saved index, conflicting titles, pair review, the NAS after the prune, test evaluation 4

Written by the agent for itself on 2026-10-10, so the plan survives context
compaction. Work autonomously and never stop to ask. When something is
ambiguous, take the conservative option, write the decision down and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-11-checklist.md` (progress).
3. `docs/notes-for-owner.md` ("After session 7" to the end).
4. `docs/status.md` (newest entries at the bottom).
5. The newest file in `docs/experiments/`, and
   `docs/experiments/test-set-ledger.md`.
6. `docs/pair-verdicts.txt`, `docs/calibration.md`, ADRs 0005, 0007,
   0009, 0010.

`docs/brief-2.md` to `docs/brief-10.md` hold rules that still apply
unless this brief changes them (among them: edit no configuration file;
test fixtures are generated, never copied from the corpus or the NAS;
long runs start detached under `screen`).

## The owner's prompt, verbatim

```text
Start session 11 of Gunfinger: a saved index, conflicting titles, a
pair-review command, the NAS clusters and baseline after the owner's
prune, test evaluation 4 at NAS scale, then research until morning. The
session runs unattended from about 18:30 until 11:00 on 2026-10-11. The
laptop is on Ethernet with the NAS mounted at /Volumes/atlas. Work
autonomously and never stop to ask. When something is ambiguous, take
the conservative option, write the decision down and continue.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it),
   docs/brief-10.md and docs/session-10-checklist.md (brief-2 to brief-9
   hold rules that still apply unless this prompt changes them),
   docs/notes-for-owner.md (from "After session 7" to the end),
   docs/status.md (newest entries at the bottom),
   docs/experiments/test-set-ledger.md, docs/pair-verdicts.txt,
   docs/roadmap.md ("On-disk index", "More than 65,536 assets", "Titles
   of mislabelled files", "Owner edits", "Clustering precision", "A
   confidence statistic for shared material", "Where search time
   goes"), docs/calibration.md, ADRs 0005, 0007, 0009 and 0010, and
   experiments 0061 and 0063-0069.
2. `git status` shows an uncommitted section at the end of
   docs/experiments/test-set-ledger.md, "Manifest change before
   evaluation 4", reviewed by the owner. Run scripts/check.sh and commit
   it alone before anything else.
3. Write docs/brief-11.md (this prompt verbatim, the state at the start,
   the plan with times) and docs/session-11-checklist.md. Point
   CLAUDE.md's reading list at them; keep brief-10 among the older
   briefs. Add a docs/status.md entry "After session 10" with what the
   owner did since (below). Commit.
4. Build with --release and copy the binaries to work/bin/s11-start/.
   Run `gunfinger-eval baseline session-11-start`. Record the NAS peak
   store's state (scripts/analysis/store_revision.py and
   store_census.py): 33,596 records, 1,582 skip notes and revision
   f9f9bc706195a71b expected. `doctor --config
   ~/.config/gunfinger/nas-dnb.toml` should read 39 patterns leaving out
   603 audio files, no records of ignored or gone files and no file
   waiting to be indexed. The laptop's disk had 62 GB free; check again.

What the owner did after session 10 (2026-10-10):
- Added 8 patterns to /Volumes/atlas/Music/dnb/.gunfingerignore for
  mixed discs beside unmixed ones (Blazin, Molten Beats, Inside The
  Machine, 21st Century Grooves, Vintage Dread 2000, Soul Survivors,
  and Essential Rewindz's discs 1 and 2), then ran `prune --yes` and
  `index` with the NAS configuration.
- Listened to NAS files named like the test set's absent tracks. At the
  owner's request the test manifest now references one NAS file each
  for tracks 6, 9, 15, 18, 23, 24 and 26, with notes on 3, 12, 16, 21
  and 28 (the ledger section above). The 2003 manifest's header comment
  now describes its NAS references; nothing else in it changed.
- Has not listened to session 10's pack (work/listening/). Item 1, the
  cut Sonar join, is unjudged, so every clusters run uses --cut-sparsest.
- Approved test evaluation 4 at NAS scale (item 7).

Rules for this session:
- The NAS (/Volumes/atlas/Music/dnb) is read, never written. The NAS
  peak store's records are read-only: no `index`, `prune` or
  `--retry-skipped` against it; its revision stays f9f9bc706195a71b,
  checked at the end. Audio files that appear on the NAS during the
  session stay unindexed; count them at the end. Item 3a's saved index
  may be written on the laptop's disk, never on the NAS, and must not
  change the store's records or revision. Keep at most one saved index
  per library.
- The test set (ed-rush-optical-essential-mix): this session may read
  its manifest (`validate`, `clusters --manifest`) and decodes its audio
  exactly once, in evaluation 4. Nothing else touches it: no identify,
  explain, leave-out, listen or clip of the test mix, and no rerun.
  Nothing changes because of its result. Never identify, decode or index
  EM_1999_02_07_-_Ed_Rush_Optical.mp3 at the NAS root. Run nothing on
  the candidate mixes of experiment 0065.
- Five NAS versions were found through the test tracklist and judged by
  the owner as other versions: Ram Trilogy's Terminal 2, the Orders DJ
  Reality RMX and Distortion Mix, the Marky & Bungle remix of Funktion,
  Clear Skyz's Break remix and 1996 VIP, and the Black Barrel bootleg of
  Beachball. Keep them out of every research item, so that no rule is
  fitted or chosen on them.
- Verdicts are the owner's: never add or change a line in
  docs/pair-verdicts.txt, and never run item 3c's command against that
  file (its tests use temporary copies).
- Manifests do not change.
- The matcher and the confidence rule do not change. `regress
  session-11-start` must be identical in detections after every code
  change.
- Heavy runs (clusters, pair, scans, sweeps, recall, timing
  measurements) go one at a time, each under `screen`, from a copy of
  the binaries in work/bin/, with wall time, CPU time and peak memory
  recorded. Changed from earlier briefs: while a heavy run goes,
  builds, unit tests, scripts/check.sh and `regress` may run under
  `nice -n 10`; timings measured under that load are marked as such and
  not compared. Item 3a's timings run only on an idle machine.
- Heavy runs do not wait for code: each uses the newest binary whose
  regress is identical.
- Each measurement is an experiment (docs/experiments/NNNN-slug.md, at
  most 40 lines; data with file names only in docs/experiments/data/).
  Commit after each item; check `git status` before every commit and
  never use `git add -A`.
- Time: items are in priority order. At 10:00 stop what is running at a
  safe point, write down what is left and do the wrap-up; the hard end
  is 11:00.

Items, in order:

1. Setup (25 min): the steps above.

2. The NAS clusters search (start by 19:00; about 2.5-3 h). Keep session
   10's report (copy to work/s11/before/), then `map-library` and
   `gunfinger-eval --other-peaks-dir ~/.local/share/gunfinger/nas-dnb-peaks
   clusters --from-peaks --manifest stakka-skynet-knowledge --manifest
   razorjack-2003-03-29 --manifest ed-rush-optical-essential-mix
   --recall-panel 2026 --cut-sparsest`, as session 10 ran it
   (work/s10/*.sh). If a recall panel source is now ignored or pruned,
   report it and leave it out; do not draw again. Write item 3a while it
   runs.

3. Code (about 6 h in all, written while items 2, 5 and 6 run):
   a. A saved index (roadmap "On-disk index", ADR 0005), about 3 h.
      First audit the widths for 65,536 assets (ADR 0005's u32 byte
      offsets address 4.3 GB; at 65,536 assets the index is about
      7.4 GB). Write an ADR for the format and where the file lives. The
      file holds what the run would build; its header names what it was
      built from (format, profile, hash design, the asset table or
      library revision, the track length range). A stale or unreadable
      file is never used: it is rebuilt and replaced atomically. Keep
      the in-memory layout; measure compression separately, if at all.
      Required: `identify` and `explain` with --config
      ~/.config/gunfinger/nas-dnb.toml use it; the harness may, if the
      ADR says how. Tests: a stale header, a truncated file, a changed
      store. Acceptance, on an idle machine after item 2: the 2003 mix
      identified at NAS scale with a rebuilt and with a saved index gives
      identical detections; time the rebuild, the first run (build and
      save), and a load from a cold and from a warm file cache; record
      the file size. Experiment.
   b. Conflicting titles (roadmap "Titles of mislabelled files"), about
      1 h. When the files matching one passage carry different titles,
      the report shows the other titles without choosing one; the 2003
      mix's track 2 ("Future Cut - Sex Drive", experiment 0061) is the
      example. Detections do not change; say what the text and JSON
      reports add.
   c. A pair-review command in gunfinger-eval (roadmap "Owner edits",
      the first step), about 2 h. It reads the listening pack
      (work/listening/README.md and the item folders), shows each
      unjudged item's sheet in short (both full paths, coverage, where
      the files stop lining up), plays its clips through the audio
      output `listen` and `review` use, and on the owner's key appends
      the `same` or `different` line the sheet shows to
      docs/pair-verdicts.txt. Keys to replay, go to the next or previous
      clip, skip an item and quit. It skips items already judged and at
      the end prints the `clusters --reuse-pairs` command that applies
      the new verdicts. Tests with a fake audio output and a temporary
      verdicts file.

4. After the clusters search (about 30 min): the experiment against
   0066: the new rips; the eight new patterns (the pack's Essential
   Rewindz items 3-6, 8, 12-14 and 16); the clusters of the test
   manifest's seven new references and their other copies (file names
   only); new borderline pairs and sparse joins. Then rebuild the
   listening pack from this run: drop items that verdicts or the ignore
   file settle, keep item 1 first and the 10 controls if they still
   join, add the new pairs (`pair`, then clips). Then item 3a's timing
   on the idle machine.

5. The new NAS baseline (about 2 h 15 min), as experiment 0067
   (work/s10/item4.sh): development scan, leave-outs 3 and 11 (seed
   2026), sweeps 2026-2029, `calibrate`, and razorjack-2003-03-29 at
   both sizes with leave-out 3. Prediction: Essential Rewindz's 24 wrong
   sweep answers are gone. Compare with 0067 and explain each change.
   Calibration register.

6. The recall panel's development half (about 30 min), as experiment
   0068.

7. Test evaluation 4 at NAS scale (about 40 min), approved by the owner
   on 2026-10-10. If items 5 or 6 show a regression against 0067 or
   0068 that is not explained, skip it and write down why. Before the
   run, add "Evaluation 4" to the ledger: code commit and binary, store
   revision, the clusters report, the manifest at both sizes (validate
   output), the command, why, and a prediction per track written from
   evaluation 3 and the clusters alone, including whether any of tracks
   3, 12, 16, 21 and 28 gets a wrong confident detection from a related
   version. Run it once with the baseline's binary: evaluation 3's
   command with `--other-peaks-dir
   ~/.local/share/gunfinger/nas-dnb-peaks`. Keep the report as
   work/reports/evaluation-4-scan-ed-rush-optical-essential-mix.json; it
   is never an input to `calibrate` or another measurement. Record the
   output verbatim, compare it with the prediction and with evaluation
   3, and update the count to "Evaluations used: 4 of 5". If the run
   fails, write down how and do not run it again. The Sonar join is cut;
   if the owner later judges pack item 1 `same`, the score is
   recomputed from the stored report, not run again.

8. Research until 10:00, measurement only, no rule change, in this
   order:
   a. Mixed discs among the folders indexed since experiment 0065:
      scripts/analysis/folder_edges.py with session 10's per-disc split;
      a list for the owner's ear with full paths and each disc's
      track-number prefix, as in the notes "Your mixed-CD judgments".
      Never write the ignore file.
   b. Where a NAS `identify` spends its time once the index loads from
      disk (roadmap "Where search time goes"), on the 2003 mix.
   c. Shared material at NAS scale (roadmap "A confidence statistic for
      shared material"): for the pairs the owner judged `different` and
      other known version families, outside the five files above, how
      close their shared passages come to the confidence rule as plays.

9. Wrap-up (45 min, by 10:45): experiment files; a "Session 11" section
   at the end of docs/notes-for-owner.md, leading with what the owner
   must do (the rebuilt listening pack and the command to step through
   it; the mixed-disc list); evaluation 4's result; docs/status.md, the
   checklist, the calibration register and the roadmap. Check the NAS
   store's revision. Commit. Last, print for the owner what was done,
   the numbers, evaluation 4, the decisions waiting for them and what is
   left.
```

## State at the start of session 11

- 18:15. `git status`: the owner's ledger section, uncommitted;
  `scripts/check.sh` green (2:23), committed alone as `51d6f4c`.
  Experiments 0001-0069; ADRs 0001-0010. Test set: 3 of 5 evaluations
  used.
- The machine: 10 cores, 32 GB; default route `en7` (Ethernet);
  `/Volumes/atlas/Music/dnb` mounted (935 entries at the root). The
  laptop's disk: 62 GiB free (`df -h /`).
- Release build of `1247e87`'s code (the ledger commit changes no code)
  in `work/bin/s11-start/`; `baseline session-11-start` saved.
- The NAS store (`~/.local/share/gunfinger/nas-dnb-peaks`): 33,596
  `.peaks`, 33,596 `.tags`, 1,582 `.skip` (11 failed, 124 too short,
  1,447 too long), `library.txt`; revision `f9f9bc706195a71b`, as
  expected; census `work/s11/census-start.json` (3,523.4 h); record list
  `work/s11/store-records-start.tsv`.
- `doctor --config ~/.config/gunfinger/nas-dnb.toml` (14:46 wall, the
  SMB listing slow at first): 39 patterns, 603 audio files left out,
  35,178 audio files = 33,596 current records + 1,582 passed over; no
  records of ignored or gone files, none waiting.
- The recall panel of seed 2026 draws two development sources (indexed,
  scene releases) and one validation source that the owner's new
  patterns leave out and the prune removed: Essential Rewindz's
  `112-future_cut-stealth_(domination)-sour.mp3`, Soul Survivors'
  `103-makoto-jupiter_fields-sour.mp3` (development) and Molten Beats'
  `203-ram_trilogy-snakebite-mixed-sour.mp3` (validation).
  `recall::Panel::for_seed` failed on such a panel; it now names each
  and leaves it out, keeping the draw and the excerpts' numbers
  (`e446040`; regress identical, 8:33 under the load of `doctor`).
- What the code does today (read before planning):
  - `catalog` builds the index from the peak store on every `identify`,
    `explain` and `stats` (65-72 s at 32,905 records, experiment 0063);
    `Index::build` keeps postings sorted by hash with an offsets table
    (ADR 0005, ADR 0010: 16 frame bits and 16 asset bits per posting).
  - A play's title comes from the tags of its best file (experiment
    0061); other files of the same play are not named.
  - `scripts/analysis/listening_pack.py` builds `work/listening/`;
    `listen` and `review` play clips through `playback`'s audio output.

## Plan, with times

Start 18:15; the stop is 10:00, the hard end 11:00.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1 | setup: ledger commit, build, baseline, store, `doctor`, panel fix, brief, checklist, status | 0:45 | 19:15 |
| 2 | the NAS clusters search (started 19:03 under `screen`) | 2:20 | 21:25 |
| 3a | saved index: width audit, ADR 0011, code, tests (during 2) | 2:30 | 21:45 |
| 4 | clusters experiment 0070; the listening pack rebuilt (`pair`, clips) | 0:35 | 22:20 |
| 3a | saved index acceptance on the idle machine; experiment | 0:30 | 22:50 |
| 5 | the NAS baseline (`screen`); 3b and 3c written meanwhile | 2:15 | 01:05 |
| 6 | the recall panel's development half | 0:30 | 01:35 |
| 7 | evaluation 4: ledger entry and prediction first, then one run | 0:40 | 02:15 |
| 8a | mixed discs since 0065 | 0:45 | 03:00 |
| 8b | where a NAS `identify` spends its time | 1:30 | 04:30 |
| 8c | shared material at NAS scale | 2:30 | 07:00 |
| - | buffer for overruns, code review, documents | 3:00 | 10:00 |
| 9 | wrap-up | 0:45 | 10:45 |

## Conservative decisions taken up front

- The panel's left-out sources are named on stderr by every command
  that loads the panel (`clusters --recall-panel`, `recall`) and stay in
  the draw (`docs/panels/recall-seed-2026.json` is not rewritten), so the
  rendered excerpts in `work/recall/seed-2026/` keep their numbers. The
  development half is searched with 78 sources (58 indexed, 20 held out).
- `doctor` took 14:46 at the start against 35-44 s in sessions 9 and 10,
  with 1.4 s CPU: the share answered slowly, not the code. It is not a
  heavy run; the clusters search started after it ended.
- Evaluation 4's binary is "the baseline's binary": the binary item 5
  runs with. Its ledger entry is written before the run.
- The other versions of five test tracks (Terminal 2, the Orders DJ
  Reality RMX and Distortion Mix, the Marky & Bungle Funktion remix,
  Clear Skyz's Break remix and 1996 VIP, the Black Barrel Beachball) are
  found by their paths in the NAS store, listed once in
  `work/s11/excluded-versions.txt` and filtered out of every research
  item's input.
