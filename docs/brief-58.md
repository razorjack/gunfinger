# Session 58 brief: library files that hold more than one recording, for the user to judge

Written on 2026-10-27, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-58-checklist.md` (progress).
3. ADR 0009 (the ignore file) and ADR 0016 (the listening tool);
   `docs/audition-questions.md`.
4. `docs/session-notes.md` ("Session 52" and "Session 57"); the
   pre-registration `docs/research/2026-10-27-files-holding-several-recordings.md`;
   this session's experiment (0241 and up).

`docs/brief-2.md` to `docs/brief-57.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 58 of Gunfinger: find library files that hold more than one recording, for the user to judge. The listenings of 2026-10-25 and 2026-10-27 found three kinds of such files in the NAS collection: a scene release that fixes tracks of a mixed CD (its tracks blend into each other), an upload cut from a mix (`Optical & Ryme Tyme - Space Cake [CUT].m4a`: Bacteria, then Space Cake), and an upload whose first 56 s are another record (`Ryme Tyme - T-Minus (Unreleased Vocal Mix).m4a` opened with Digital & Spirit - Crash). The user added each to the NAS `.gunfingerignore`, and session 52 pruned them. Such a file joins unrelated recordings in the clusters, gives wrong answers in sweeps, and would print a wrong title in a tracklist. The i-witness-dnb channel alone holds 58 more files named `[CUT]`, and earlier sessions found mixed CDs one by one (experiment 0065, ADR 0009). This session builds a harness report that lists candidate files, each with the evidence a listener needs, and a listening pack for the strongest candidates; the user judges them and decides what to ignore. It is a measurement session: nothing is ignored, pruned or written to the NAS.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).

Time box: started about 10:25 (CET); aim to finish by about 14:30 (the user's soft deadline is 15:00). If a step would run past that, scale it down (fewer candidates in the pack, the [CUT] files first), record the decision in the brief and continue.

Environment (sessions 33-57's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,678 records, revision `1fc6fe50293ad330`. NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `ce293fe6165e...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-54-linux-nas`. Tonight's changes: session 52 pruned five ignored files (0231); session 53 made `SpansAtSpeed(0.006)` the default second suppression (ADR 0021, old rule `--overlap-by-spans`); session 54 scores same-title detections as right (ADR 0022, `docs/same-titles.txt`); session 55 prints gaps of 45 s or more as "ID - ID" lines (ADR 0023); session 56 adds each gap's estimate of the tracks it holds (ADR 0024).
- Build `work/bin/s57-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans, sweep reports of seeds other than 2026 (calibrate reads every sweep report there), and reports of opt-in variants must not sit in a reports directory during `regress`; run variants in a scratch work directory.
- The user's decisions of 2026-10-27 (docs/status.md, "session 39's listening questions, and four decisions"; the roadmap's bullets carry them): `--apart-in-speed 0.006` becomes the default; gaps are printed by default ("ID - ID", 45 s or more, the mix's start and end, possible lines kept, an estimate of the tracks a gap holds, JSON yes, CSV and cue sheet unchanged); a confident detection of a near-identical version counts as right; the minimum span stays undecided. Each is implemented with an ADR (the next number is in `docs/adr/`).
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen. Pending listening (Star Trails, Into Music) stays for later. When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), the newest brief and docs/brief-52.md, docs/session-notes.md (the newest session and "Session 52"), docs/status.md (the newest entries, and those of the listenings of 2026-10-25 and 2026-10-27), ADR 0009 and 0016, experiment 0065 (its candidate mixes are out of every run: read only its list, to leave them out), the NAS `.gunfingerignore` (read only), `docs/audition-questions.md`, and the code of `clusters.rs` (the saved pairs and their alignments), `pair.rs`, `identifiability.rs` and the `questions` subcommands.
2. Write docs/brief-58.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-58-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s58/listening-digests-start.txt. Experiments number from 0241.
3. Build `work/bin/s58-start`; `regress session-54-linux` and `regress session-54-linux-nas` identical.

Steps:

1. Pre-registration, committed before the candidates are listed: the signal (a file whose aligned stretches against other files fall into two or more parts that match different clusters, one part at the file's start or end and another elsewhere, each part long and strong enough to be a play; from the clusters' saved pairs where they hold the alignments, otherwise from `pair` or an `identify`-style search of the file against the NAS index, at a cost you estimate first), the thresholds, and the known positives as the check: the five pruned files' old records are gone, so use their old reports or alignments where saved (session 52 lists where they appeared), and the mixed CDs already ignored are out of the store. Name what would count as a candidate worth the user's ear.
2. The harness, tested: a report (`gunfinger-eval` subcommand or an option of an existing one; harness only, `regress` identical) listing each candidate file with its parts (file times, the matched cluster or file, hits, speed), sorted by strength, and a `questions` pack of the strongest candidates (each as a question with the file as the reference lane and the matched recordings as lanes, so the listener hears whether the file changes record), written under work/s58/. Unit tests on fixtures.
3. The run at NAS scale, under tmux with a log if long. Leave out experiment 0065's candidate mixes and anything the ignore file already leaves out.
4. One experiment (at most 40 lines): how many candidates, by kind (cut from a mix, a mixed CD's track, an upload opening with another record, other), how many of the [CUT] uploads are candidates, and the check against the known cases.
5. The listening pack: at most 20 questions, the strongest and most varied candidates first, with the command to open it (`gunfinger-eval audition ...`), ready for the user. Nothing is answered by an agent.
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 58" section at the end of docs/session-notes.md with a handoff (the pack's command and what each answer would lead to: an ignore entry the user writes, or nothing); update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-54-linux` and `regress session-54-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the candidates by kind, the check against the known cases, the pack's command, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's behaviour does not change: no threshold, parameter, rule, default or output change. Harness additions are tested and keep `regress` identical. Nothing is ignored, pruned or written to the NAS; the user decides what to ignore.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
- Scratch data goes under work/s58/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 15:00. Start no new run after about 14:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 58

- 2026-10-27, 10:20 by the Linux machine's clock (CET). Aim to finish by
  about 14:30; the soft deadline is 15:00, and no new run starts after
  about 14:30.
- `master` at `89329ba`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s58/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `ce293fe6165e...`, `work/audition/answers.jsonl` `3e50aaa62181...`.
- Experiments 0001-0240; this session numbers from 0241. ADRs
  0001-0024; this session writes none (a measurement session). Test
  set: 4 of 5 evaluations used (none in this session).
- Baselines `session-54-linux` (262 tracks) and `session-54-linux-nas`.
  Build `work/bin/s57-final/` (master).
- The NAS store: 40,678 records (`work/s58/store-paths.tsv`, read from
  the records' headers by `work/s58/store_paths.py`), 4,260 hours;
  34,575 records under `__youtube_archivists/` (6,339 of them in
  `i-witness-dnb/`), 3,538 under `__full_scene/`. 46 records are of
  files named `[CUT]`, all in `i-witness-dnb/`; the channel's other
  `[CUT]` files have no peak record (outside the track length range or
  failed; checked in step 3).
- Experiment 0065's candidate mixes: none of its 1,423 files passed over
  as too long has a peak record. Of the 33 folders its boundary measure
  scored 0.75 or more, 31 are left out by the NAS `.gunfingerignore`; the
  other two (`shy_fx-just_an_example-1995-sour`,
  `mampi_swift-blade_runners_ep-bld003-2002-sour`) hold 15 records.
- What a search costs: session 10's NAS clusters run searched 639 whole
  files in 75,059 s of CPU, about 117 s per file; at that rate all
  40,678 files would take about 41 hours on 32 threads. The search has
  become cheaper since (sessions 41, 43, 46), but not by a factor that
  fits the time box, so the run searches less than every file whole
  (the pre-registration fixes how).

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build, both regressions (tmux `s58start`) | 10:20-10:40 |
| 1 | the pre-registration (signal, thresholds, query set, known cases), committed before any candidate is listed | 10:40-11:05 |
| 2 | the harness: `gunfinger-eval parts` (report) and `questions parts` (pack), tested; `regress` identical | 11:05-12:15 |
| 3 | the run at NAS scale (tmux, log) | 12:15-13:15 |
| 4 | experiment 0241 | 13:15-13:35 |
| 5 | the listening pack (at most 20 questions) and its command | 13:35-13:50 |
| 6-7 | review by a separate agent; notes, status, checklist, final checks | 13:50-14:40 |

## Decisions

- **(10:24) Start build and regressions** (`work/s58/start.sh`, tmux
  `s58start`, logs in `work/s58/logs/`; `run.sh` and `same.py` are
  session 57's with the paths changed). The build was a no-op: both
  binaries `cmp` equal to `work/bin/s57-final/`.
- **(10:33) Experiment 0065's candidate mixes are left out of the
  queries**: the 15 records of its two unignored mixed-CD folders are
  not searched as queries (a leave-out list,
  `work/s58/leave-out-0065.txt`, of the 33 folders). They stay in the
  index, as in every NAS run, so they can appear as a matched file; a
  candidate whose parts name one of them is listed in the report but
  left out of the listening pack, so that nothing decodes them. The
  ignore file's files have no record (session 52's prune), and the
  harness also reads the NAS `.gunfingerignore` (read only) and leaves
  out any record it matches.
- **(10:29) The pre-registration** committed at `6f183aa` before any
  search: `docs/research/2026-10-27-files-holding-several-recordings.md`.
- **(10:35) The harness** (`gunfinger-eval parts`, `parts.rs`;
  `questions parts`, `audition/parts.rs`): the queries are a list of
  store paths given in a file (`work/s58/tiers.py` writes the tiers), so
  the harness derives nothing from names; `--leave-out` takes the 0065
  list (`work/s58/leave-out-0065.txt`: its 33 folders and 1,423 long
  files), and the NAS ignore file is read where the store names its
  library. The pack's choice by kind is made outside the harness, by a
  script that writes the `--files` list, for the same reason.
- **(10:35) Timing** (the first 64 files of tier 2, output not read):
  the index of 40,678 records built in about 23 s; the 64 files took 38 s
  of wall time, most of them in the first 15 s and the rest in two whole
  searches of files with an edge part: about 0.3 s per file on 32
  threads. So tiers 1-3 take about 65 minutes and all of tier 4 would
  take about 2.4 hours more. Tier 4 runs in path-order chunks of 1,600
  files, none started after the 90 minutes of the budget
  (`work/s58/main-run.sh`).
- **(10:38) `regress session-54-linux` identical** with the run's build
  `work/bin/s58-run/` (0:51); the NAS regress runs after the main run,
  so that it does not share the machine with it.
- **(10:38) The main run started** (tmux `s58main`, logs
  `work/s58/logs/tier-*.{log,err}`, reports `work/s58/out/tier-*.json`).
- **(10:40) The check against the known cases** (`work/s58/known.py`,
  output `work/s58/known-cases.txt`): the pre-registration's rules,
  re-implemented in Python, on the five pruned files' saved pairs, with
  their lengths read by FFprobe from the NAS audio (`work/s58/pruned-durations.tsv`;
  read only, nothing decoded). As expected: the Space Cake `[CUT]` file
  is a candidate worth the ear (Bacteria over 0:01-1:05 from 2:40 in,
  three files, 118-149 hits); the track-fix Drumtrip is one with a second
  part (the 3zb Propaganda upload over 0:01-0:40, 207 hits; Known
  Unknown over 0:07-2:55); the T-Minus vocal mix upload is not a
  candidate from the saved pairs (only the corpus T-Minus over
  1:08-3:54 is saved); the Kraken has no saved pair; the track-fix
  Propaganda is not a candidate (its part ends 25 s before its end, not
  within 15 s).
- **(10:40) The pack leaves out the controls** (a conservative choice
  the pre-registration did not fix): a candidate among Listen Up's tracks
  or the unmixed discs is counted, not asked again, since the user has
  judged those releases.
- **(10:41) `questions parts` checked** on tier 1's report: the file it
  writes loads in `audition` on temporary files (`/tmp/s58q/`, the
  verdicts file copied there); `docs/pair-verdicts.txt` unchanged.
- **(11:16) Tier 2 done** (37:11, 6.7 GB peak): 62 candidates of 6,293
  files, 56 worth the ear; with tier 1, 65 and 59. Reading the strongest
  shows that most are another version of the same tune: an edit, VIP,
  remix or bootleg that shares a long passage with the original, which
  starts inside the original and so passes the pre-registered rule for
  an edge part (Bad Company - Blind's Tinted Edit, 8,419 hits over its
  last 177 s). The pre-registered rules, report, kinds and pack rule stay
  as they are (no change after seeing the results). The experiment adds
  one split marked post hoc, for reading only (`tally.py`): whether the
  strongest edge part's file name shares a word of five or more letters
  with the candidate's. It is crude (a Book Of The Bad track named
  `02.mp3` counts as different; Vaccine and Questions share
  "counterstrike") and decides nothing.
- **(11:51) Tier 3 done** (34:25, 6.9 GB peak): 117 candidates of 6,088
  files, 106 worth the ear. Two long records among them are mixes or
  radio segments indexed as tracks (`sety/Weekenders_1_-_Nocny_Trans.mp3`,
  12 min, not the Nocny development mix, whose audio is `vertical.mp3`;
  the Virus on Kiss FM show of 3 May 2000, 15 min); neither is on
  0065's lists. Seven candidates are controls: five tracks of Ray Keith
  presents Vintage Dread 2000's first disc (whose second disc the ignore
  file leaves out), one of 21st Century Grooves' and one of Inside The
  Machine's. Their edge parts are other records (Vintage Dread's Sing
  ← Londons Most Wanted, Chopper ← LTJ Bukem - Horizons), so the first
  disc may be mixed too. They stay out of the pack, as decided at 10:40,
  and go to the handoff as a question for the user.
- **(11:51) The pack keeps the pre-registered rule**, although most of
  its strongest candidates are other versions of one tune: the pack is
  part of the measurement, and the user's answers show how often the
  signal is right. The post-hoc list of candidates whose strongest edge
  part has a different name (`work/s58/different-name-files.txt`, by
  `tally.py`) goes to the handoff, ready for a later pack with
  `questions parts ... --files`.
