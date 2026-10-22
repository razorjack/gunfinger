# Session 39 brief: scoring the printed tracklist, with double drops

Written by the agent for itself on 2026-10-23, so the plan survives
context compaction. Work autonomously and never stop to ask: where a
decision is needed, take the conservative option, write it down here and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-39-checklist.md` (progress).
3. ADR 0006, 0017 and 0019; experiments 0095, 0101, 0102, 0107 and this
   session's (0182 and up); the roadmap's "The printed tracklist" and
   "Possible plays inside a confident play of another recording".
4. `docs/session-notes.md` ("Session 35" to the end), `docs/status.md`
   (newest entries), `crates/gunfinger-eval/src/mixes.rs`,
   `tempo.rs`, `robust.rs` (`partner_confident`),
   `crates/gunfinger/src/export.rs` (`entries`), `report.rs`,
   `output.rs`, `crates/gunfinger-core/src/plays.rs`.

`docs/brief-2.md` to `docs/brief-38.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 39 of Gunfinger: score the tracklist `identify` prints, with double drops. The roadmap's "The printed tracklist" (a proposal from an external review): the harness scores detections, plays and seconds, but nothing scores the entries `identify` prints after grouping files (ADR 0017), choosing each entry's name, flagging disagreeing titles and leaving out possible plays inside a confident play (ADR 0019). ADR 0019 names a cost no stored report could measure: a track found only as possible inside a passage of a stronger track, as in a double drop, leaves the tracklist. Double drops are common in drum & bass, and the generated mixes only move from one track to the next with fades, bass swaps and (since session 35) beatmatched blends. This session adds double drops to the generator, renders mixes, runs the real CLI on them and scores what it prints. It is a measurement session: the product's rules and output do not change.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600. The search is faster since session 36 (0169): use the default jobs (32). Long mixes render at about 14 GiB per seed of 16; one heavy job at a time.

Time box: started about 00:30; aim to finish by about 04:00, because further sessions follow you tonight. If a step would run past that, scale it down (fewer seeds, 262 tracks only), record the decision in the brief and continue.

Environment (sessions 33-38's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records), library `/mnt/atlas/Music/dnb` (read only), reached with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: since the listening of 2026-10-22 (docs/status.md, "after session 38") SHA-256 `d9c85c4586ad...`, 284 clusters, Dilusion's upload joined to its cluster; `regress session-33-linux-nas` is identical with it (5:56). Baselines `session-32-linux` and `session-33-linux-nas`. Build `work/bin/s38-final/` (master). `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- Session 38's opt-in `--apart-in-speed` and session 37's `--overlap-by-chains` are variants, not defaults; their adoption waits for an ADR (ours).
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans must not sit in a reports directory during `regress`.
- Renders from earlier sessions sit in `work/mixes/` (about 21 GB); leave them, and keep your own renders' total under about 40 GB.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-38.md, docs/session-notes.md ("Session 35" to the end), docs/status.md (newest entries), the roadmap's "The printed tracklist" and "Possible plays inside a confident play of another recording", ADR 0006, 0017 and 0019, experiments 0095, 0101, 0102 and 0107, `robust`'s blend conditions (`partner_confident`), and the code of `mixes.rs` (every `--kind`, `tempo.rs`'s beatmatching), the CLI's `export.rs` (`entries`), `report.rs`, `output/` and `plays.rs`.
2. Write docs/brief-39.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-39-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s39/listening-digests-start.txt; it must be the same at the end. Experiments number from 0182.
3. Build `work/bin/s39-start`; `regress session-32-linux` and `regress session-33-linux-nas` identical.

Steps:

1. Pre-registration, committed before any count: the scenarios, the truth each mix carries (which recordings play when, which files of each recording are indexed, their tags), the scores (per mix: extra entries; played tracks missing; two played tracks merged into one entry; matching files missing from an entry; disagreeing titles not flagged; beside today's confident-only score, by recording family as well as by entry), and what would count as a problem worth a roadmap entry. Tags: use recordings whose files' artist and title tags agree across their cluster and with the file names (checked by a script, no listening), so that a mislabelled file does not hide a grouping error; say how many qualify.

2. The generator: a double drop (two records at about equal level for 30-90 s, beatmatched with `tempo.rs`, entering and leaving with the usual blends), tested, and a mix kind with these scenarios: an original alone; its remix alone; both together in a double drop (shared-material pairs from the panels in `docs/panels/`, as session 35's `--kind shared` draws them); two unrelated records together; a quiet (for example -9 dB) or brief (20-40 s) record inside a stronger one; a returning record; several rips of one recording played at different times. Exact truth in the report. Harness only; `regress` identical.

3. The runs: render the mixes (several seeds, enough for each scenario to occur at least about 30 times), search them the way the user does, with the CLI's `identify -f json` and its tracklist (`gunfinger show`), at 262 tracks (the corpus configuration, with the sweep's held-out clusters left out of the index by a separate peak store or store-only copy, so that held-out records are truly unknown; never write to `work/peaks`) and, if time allows, at NAS scale with a store-only copy or a scratch configuration (never index or write the NAS store). Score with a script against the truth.

4. Experiments (at most 40 lines each, written before the next starts): what the printed tracklist gets right and wrong in each scenario, with the double drop's partner the main question (ADR 0019's cost: how often a played record found only as possible inside its partner's passage leaves the tracklist, and whether the table or JSON still shows it), and where entries merge or split wrongly.

5. Ahead, only when 1-4 are done: the same scenarios scored at the detection level by the harness (`mixes`'s own scoring and `seconds`), to tell what the search loses from what grouping and printing lose.

6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.

7. Wrap-up: a "Session 39" section at the end of docs/session-notes.md with a handoff and the questions it leaves for us (a change to what the tracklist prints is ours to decide, with an ADR); update docs/status.md, the checklist and the roadmap's "The printed tracklist". Final checks: `scripts/check.sh` green, `regress session-32-linux` and `regress session-33-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, `git status` clean. Commit. Your final message: what was built, the scores per scenario in a few lines, the problems found, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's behaviour does not change: no threshold, parameter, rule, default or output change; variants are opt-in, tested, and keep `regress` identical when off. Harness additions are tested and keep `regress` identical.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user runs among them).
- Scratch data goes under work/s39/ (your session number) and renders under `work/mixes/`; delete your renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00. Start no new run after about 10:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 39

- 2026-10-23, 00:21. `master` at `6665077`, clean. Git identity `Jacek
  Galanciak <jacek.galanciak@gmail.com>`. No tmux server running.
- `docs/pair-verdicts.txt`: SHA-256 `f4b1867956ba...`
  (`work/s39/listening-digests-start.txt`), the file after the listening
  of 2026-10-22 (six lines more than session 38's `ad0fc785d26a...`).
- Experiments 0001-0181; this session numbers from 0182. Test set: 4 of
  5 evaluations used (none in this session).
- `work/mixes/`: 21 GB of earlier sessions' renders (left alone). Disk:
  803 GB free.
- The corpus store `work/peaks` holds 265 records with tags (`.tags`
  beside each record), so `identify` reports at 262 tracks carry tags
  (session 16's reports did not).
- `mixes` builds its index from the corpus store only; it has no
  `--other-peaks-dir`. Of the first shared panel's pairs, only
  Clockwork's (the original and the Stakka remix) has both records in
  the corpus library; the others need files of the NAS.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digest; brief; start build and both regressions (tmux `s39start`) | 00:21-00:45 |
| 1 | tag check (`tracklist_tags.py`); the pre-registration in this brief; commit | 00:45-01:15 |
| 2 | `mixes --kind drops` (double drops, overlays, scenarios), tests, `regress` identical | 01:15-02:15 |
| 3 | render and search (harness), store-only copy without the held-out records, CLI `identify -f json` and `show`; the scoring script | 02:15-03:00 |
| 4 | experiments | 03:00-03:30 |
| 5 | detection level (`mixes`'s scores, `seconds`), if 1-4 are done | 03:30-03:45 |
| 6-7 | review, notes, final checks | 03:45-04:30 |

NAS scale only if the 262-track work is done by about 03:00.

## Decisions

- **(00:26) Start build and regressions** (`work/s39/start.sh`, tmux
  `s39start`), while the brief and the tag check are written.
- **(00:31) Step 1, the pre-registration** (committed before any mix of
  this kind is drawn, rendered or searched).

  *Recordings with fit tags* (`scripts/analysis/tracklist_tags.py
  work/peaks work/reports/duplicate-clusters.json
  work/s39/tags-262.json`, reading the store's `.tags` beside each
  record; no listening). A recording (a duplicate cluster of the
  262-track clusters file, or a file in none) qualifies when every file
  has an artist and a title tag, the files' artists and titles compare
  equal as the CLI compares them (`compared_text`: letters and digits in
  lower case), and every word of that artist and title is in each
  file's name. Of 245 recordings, 198 qualify (208 files): 38 fail on a
  file name without the tagged words, 8 on a missing tag, 1 on files
  tagged differently. 9 qualifying recordings have two files or more.
  No two qualifying recordings share a tagged name, so a printed name
  tells its recording. Every indexed record a mix plays comes from a
  qualifying recording; held-out records are drawn from every held-out
  file (they must print nothing, whatever their tags).

  *Seeds and the index.* Seeds 2026 and 2028, 16 mixes each, with each
  seed's sweep panel (`docs/panels/sweep-seed-<seed>.json`, copied to
  `work/s39/panels`) holding its clusters out. These two keep both
  Clockwork records indexed; 2027 and 2029 hold one out. The CLI
  searches a copy of the corpus store's records and tags without the
  seed's held-out files (`work/s39/store-<seed>/`, made by copying, so
  `work/peaks` is only read), with `identify --store-only --saved-index
  off -f json` and the corpus configuration's other settings
  (turntable playback); `gunfinger show` prints the tracklist from each
  saved report.

  *The pairs.* Of the first shared-material panel's directions with a
  passage (session 35's `--kind shared`), the generator uses those whose
  two files are both indexed and qualify. At 262 tracks that is
  Clockwork only (the original and the Stakka remix, both directions):
  the other families' second records are files of the NAS. So at 262
  tracks the pair scenarios rest on one pair; NAS scale would widen
  them and is optional.

  *The scenarios* (`mixes --kind drops`), in this order in every mix,
  each block joined to the next by `join`'s transitions (bass swap of
  8-16 s, crossfade of 4-10 s or cut); main plays 100-200 s, lengthened
  where a drop or overlay needs room; every play beatmatched (tempo, not
  beat phase, as session 35's kinds) to the play it is mixed with:
  1. `original`: a pair's played record alone, over its passage.
  2. `unrelated-drop` (through): A plays; B enters (fade 4-16 s), both
     at full level for 30-90 s, A leaves (fade 4-16 s), B plays on.
  3. `returning`, first play of R.
  4. `quiet`: Q2 at -9 dB for 30-90 s (fades 4-16 s) inside Q1.
  5. `rips`, first: one file of a recording with two or more files.
  6. `pair-drop`: a pair's played record A, its related record B
     entering so that B's passage starts where A's does where the
     tracks allow; full level 30-90 s; through or inside, drawn.
  7. `brief`: B2 for 20-40 s in all (fades 2-5 s) at full level inside
     B1.
  8. `held-out`: a held-out record alone.
  9. `unrelated-drop` (inside): as 2, but B leaves after the drop and A
     plays on.
  10. `remix`: the pair's related record alone, over its passage.
  11. `returning`, R again from another place in the track.
  12. `rips`, second: another file of the same recording.
  Records of different blocks come from different recordings, apart
  from the repeats the scenarios make; the pair family's files are in
  no other block.

  *Truth* (in the report, per play): file, held out or not, mix times
  of the first and last audible sample, fades, speed, source position,
  gain in dB, the scenario and the side (`first`: playing when the other
  comes in; `second`: coming in). The scoring script adds each file's
  recording (262-track clusters file), family (the panel's for the pair,
  else the recording), indexed files and tags.

  *Scores* (`scripts/analysis/tracklist_score.py`), from each `identify`
  JSON and the tracklist `show` prints; the script rebuilds the entries
  from the JSON as `export::entries` does and checks every printed line
  (number, start, name, other names, marks) against them. A play
  overlaps a truth play when their spans intersect, as `mixes`'s
  scoring. For every truth play of an indexed record:
  - *today's score*: a confident play of a file of its recording
    overlapping it (the confident-only score, as the harness counts it);
  - *printed*: an entry named after its recording (its first play's
    file) whose plays of that recording overlap it, confident or
    possible; otherwise *missing*, split into: *left out* (a possible
    play of its recording overlaps it and carries `shares_material_with`,
    ADR 0019; the table and JSON still show it), *under another entry*
    (a play of its recording is in an entry named otherwise), *not found*
    (no play of its recording overlaps it);
  - by family as well: printed when an entry named after any recording
    of its family overlaps it.
  For every printed entry: *right* (a truth play of its named recording
  overlaps the named play), else *extra*, split into a related version
  (same family as a record playing then) and unrelated, confident and
  possible apart; *split* (a second entry naming the same truth play);
  *merged* (its plays hold files of two or more recordings each
  overlapping a truth play of its own; within one family or across
  families); *files missing* (indexed files of its named recording not
  among its plays' files and identical copies; split into found in
  another entry or marked, and not found); *titles not flagged* (its
  files carry two compared artist-and-title names or more and the line
  has no `titles disagree:`; whether the other title comes from a
  confident or a possible play). Wrong confident plays (no truth play of
  their recording overlapping) are counted beside, as today's score.

  *Problems worth a roadmap entry* (each reported whatever its size):
  - P1: any confident entry naming a recording that does not play then
    and is not a related version of one that does.
  - P2 (ADR 0019's cost): in a scenario, 10% or more of the indexed
    records played at full level for 30 s or more (drops, both sides)
    missing from the tracklist while the JSON holds a play of them.
  - P3: 5% or more of the two-record blocks merged into one entry
    across families.
  - P4: 10% or more of a scenario's truth plays split into two entries
    or more.
  - P5: 10% or more of the entries of recordings with two indexed files
    or more missing a file the JSON has a play of over the entry.
  - P6: any entry holding a confident play of another title without
    `titles disagree:`; possible plays named under `also:` with another
    title are reported, and are a problem at 5% of entries or more.
  The quiet and brief records are reported per scenario but not held to
  P2 (they are below full level or shorter than 30 s by design).

  *Optional, NAS scale:* the same renders searched with a store-only
  copy of the NAS store less every file of the held-out recordings
  (NAS clusters file and library map), scored the same way with the
  NAS clusters for recordings. Done only if the 262-track work is
  finished by about 03:00.
- **(00:40) The generator** (`crates/gunfinger-eval/src/mixes/drops.rs`,
  `mixes --kind drops --tagged FILE`). Decisions taken while writing it,
  none of which changes a registered score: both records of every
  two-record block come from tracks of 240 s or more, so that a drop of
  the longest kind fits; the rips are drawn first, so that the returning
  record cannot take the only recording with two files; the pair's
  passages are aligned at the start of the full-level stretch and moved
  to the track's start or end where they do not fit (the remix's passage
  lies at 399.5 s of a shorter track than the drop needs after it).
  `shared_pairs` reads the length of every panel file before the pairs
  are filtered, so the runs pass `--other-root /mnt/atlas/Music/dnb`
  (FFprobe on the NAS files only). `PlannedPlay` gains `gain_db`,
  `scenario` and `side`, left out of the JSON when unset, so earlier
  truth files and reports read and render as before.
- **(00:41) Queues.** `work/s39/step2.sh` (tmux `s39step2`):
  `check.sh`, the run build `work/bin/s39-run/`, both regressions;
  `work/s39/step3.sh` (tmux `s39step3`) waits for it, then renders and
  searches seeds 2026 and 2028 with the harness, runs the CLI on the
  renders (`--peaks-dir work/s39/store-<seed> identify --store-only
  --saved-index off -f json --save-dir`), `show` on each report, and
  `tracklist_score.py score`. The store copies hold 212 and 210
  records (265 in the corpus store, less 53 and 55 held-out files).
- **(00:43) NAS scale queued** (`work/s39/step3b.sh`, tmux `s39nas`,
  after step 3): the same renders, `gunfinger --config
  ~/.config/gunfinger/nas-dnb.toml identify --exclude-from
  work/s39/nas-exclude-<seed>.txt --saved-index off -f json --save-dir`,
  so the NAS library is only listed and the NAS store only read, and no
  saved index is written. The exclude lists hold the NAS files of every
  NAS cluster that holds a held-out corpus file (120 and 130 files;
  every held-out file of both seeds is in a NAS cluster). Scored with the
  NAS clusters for recordings; the recordings were chosen by their tags
  at 262 tracks, so at NAS scale their other rips may carry other tags,
  and the title scores there are reported but not read against P6.
  The NAS scale's indexed files are the store's records less the
  excluded ones, which also counts files the library's
  `.gunfingerignore` leaves out; its "files missing" count is read with
  that caveat.
- **(00:50) Step 2 checks** (`work/s39/step2.sh`): `check.sh` green
  (4:13); the run build `work/bin/s39-run/` (its CLI byte-identical to
  the start build's); `regress session-32-linux` (1:14) and `regress
  session-33-linux-nas` (6:12) identical, every report identical to its
  baseline's apart from `wall_seconds`. The scoring script's entry
  rebuild was checked first against `show` on session 35's three
  development reports at 262 tracks (`work/s35/identify/`): every line
  matches.
- **(00:54) The CLI step again.** `identify -f json` refuses several
  recordings ("several recordings can only be printed in the tracklist,
  human or timeline format"), so both CLI steps failed at once and
  nothing was searched (the NAS queue was stopped). They run again
  (`work/s39/step3c.sh`, tmux `s39cli`) with `--save-dir` and the
  default output: `--save-dir` writes the same JSON report `-f json`
  prints, one per mix, and the tracklist goes to the log. The harness
  searched with its default ladder (`both`, `identify`'s default
  without a configuration), the CLI with the corpus configuration's
  turntable ladder; step 5 searches the cached renders again with
  `--ladder turntable` to compare like with like.
- **(00:54) Renders**: 16 mixes per seed in 1:40 each (7.1-7.5 GiB),
  468 and 480 MB.
- **(01:03) Step 3's runs done.** 262 tracks: `identify` 0:55-0:56 per
  seed (0.17 GiB); NAS scale 3:33-3:38 (5.4 GiB). The scoring script's
  first NAS pass stopped on one line of seed 2026's mix-04: its line
  pattern was greedy, so an `also:` list followed by `(possible)`
  swallowed the mark. The pattern was fixed (the CLI's line was right)
  and every report scored again from the saved files: all 64 tracklists
  match their rebuilt entries. Two counts were added to the script after
  the first look, both from the saved reports: the marked plays split by
  what was playing, and entries with two names split into titles alike
  (artists differ, which ADR 0017 does not compare) and titles
  differing. Neither changes a registered score; P6 is read on the
  second.
- **(01:06) Experiment 0182** (both scales, one pre-registered
  experiment). P1 and P3-P6 not met; P2 not met by its letter (both
  sides of a scenario pooled: 5.5-9.4%), while the second record inside
  its partner loses 19-32%. The NAS run started before 0182 was
  written, as part of the same pre-registered experiment.
- **(01:07) Experiment 0183** (exploratory, saved reports only): no
  measure in a report separates a record playing under its partner from
  shared material; a length bound of 40 s would print every left-out
  drop partner and 19 of 64 shared-material lines at 262 tracks, and 2
  of 5 marked plays of the development mixes' NAS-scale reports.
- **(01:12) Step 5, experiment 0184.** The harness on the cached renders
  with `--ladder turntable` (0:59-1:00 per seed) gives the same row per
  scenario as with `both`, and every indexed play's level agrees with
  the CLI's report (512 of 512). `seconds` on both reports. The
  `detections` summary first grouped records across all mixes at once
  (wrong "through" counts); fixed to group within each mix before any
  number was written down.
- **(01:15) Widening the pairs at NAS scale** (beyond the registered
  plan, decided after 0182-0184 because their pair findings rest on one
  pair). Of the first panel's families with passages, three have
  recordings whose tags agree across the files the panel lists and with
  their names, the related record being a NAS file: Pyro (Time Is
  Broken ~ Ignorance), T-Minus (~ Unreleased Vocal Mix) and Watermelon
  (~ 300 Baud's Mobiuz & Cycom Remix). Clockwork's original does not
  qualify at NAS scale (a rip tagged "Clockwork (Remastered)"). The
  drops kind gains `--other-pairs`: pairs whose files are tagged and not
  held out, whether or not this index holds them; a pair record outside
  the index is `held_out` in the truth, so the harness's own score stays
  right. The tagged list for it holds the 262-track list less
  Clockwork's family, plus the three families' files
  (`work/s39/tags-other-pairs.json`). Seeds 2026 and 2028 again (their
  held-out sets), 16 mixes each, reports and renders named
  `drops-other-pairs-seed-<seed>`, searched by the CLI at NAS scale
  only, scored as 0182 with the truth's indexed records taken from the
  NAS index (a recording is indexed when the NAS index holds one of its
  files). Same scores and criteria as 0182; the pair scenarios are the
  question.
- **(01:36) The other pairs' runs** (`work/s39/other.sh`, tmux
  `s39other`): `check.sh` green (3:58); the build `work/bin/s39-run2/`
  (its CLI byte-identical to the start build's); both regressions
  identical (1:10; 5:59); renders 1:37-1:39; the CLI at NAS scale
  3:25-3:35. Both seeds hold one family's original out (2026 Time Is
  Broken, 2028 T-Minus), so each plays two families. The scoring script
  now takes a truth record as indexed when the searched library holds a
  file of its recording (the other pairs' related files are `held_out`
  for the harness); 0182's summaries are unchanged by it. The outcome
  order was also changed (a play in another printed entry counts as
  "under another entry" before "left out"); 0182's summaries are
  unchanged by that too.
- **(01:45) Experiment 0185.** A new confident identification at NAS
  scale: Digital & Spirit - Crash over the first 56 s of the T-Minus
  vocal mix (P1 by its letter, pending listening).
