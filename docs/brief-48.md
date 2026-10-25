# Session 48 brief: telling a record playing from shared material

Written by the agent for itself on 2026-10-25, so the plan survives
context compaction. Work autonomously and never stop to ask: where a
decision is needed, take the conservative option, write it down here and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-48-checklist.md` (progress).
3. ADR 0006, 0017 and 0019; experiments 0182-0185 and 0210, and this
   session's (0215 and up); the roadmap's "Possible plays inside a
   confident play of another recording", "A confidence statistic for
   shared material" and "The printed tracklist".
4. `docs/session-notes.md` ("Session 39", "Session 45", "Session 47"),
   `docs/status.md` (newest entries); the code of
   `crates/gunfinger/src/report.rs` (`mark_shared_material`,
   `same_recording`), `export.rs` (`entries`),
   `crates/gunfinger-eval/src/mixes/drops.rs`, `tempo.rs`, `pair.rs`,
   `verifier.rs`; `scripts/analysis/tracklist_score.py`,
   `tracklist_marked.py` and this session's `marked_features.py`.

`docs/brief-2.md` to `docs/brief-47.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 48 of Gunfinger: telling a record playing from shared material, among the plays the tracklist leaves out. ADR 0019 leaves out of the tracklist a possible play inside a passage of a confident play of another file (a "marked" play), because it is usually material a related version shares. Session 39 (experiments 0182-0185) measured its cost: of 43 records the search finds as possible inside a partner's passage, the tracklist prints 2 and leaves out 41 (double drops, quiet and brief records), and 0183 found that nothing the report holds (hits, length, the ratio to the partner) separates those from shared material (the Clockwork pair's 64 related plays). The roadmap's "A confidence statistic for shared material" names a hypothesis: evidence outside passages the library itself shows to be shared. An unrelated drop partner shares nothing with the confident play's file; a related version's segment maps, through the alignment between the two files, onto the moment the confident play is at; a record playing under its relative brings evidence from its own distinctive material. This session measures features of that kind on mixes with exact truth, and how often double drops whose drops are lined up in phase merge two records into one entry (session 39 matched tempo, not beat phase; DJs line drops up). It is a measurement session: the tracklist's rules do not change; a change is ours, with an ADR.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32). Rendering runs well in parallel here.

Time box: started about 02:40 (CET; the clocks went back an hour tonight); aim to finish within about 2.5 hours, because other sessions follow you tonight. If a step would run past that, scale it down (262 tracks first, fewer seeds), record the decision in the brief and continue.

Environment (sessions 33-47's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records), library `/mnt/atlas/Music/dnb` (read only; on Ethernet again tonight), reached with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: after the listening of 2026-10-25 (docs/status.md, "after session 46") SHA-256 `fdf7428a6322...`, the same 284 clusters as before. Baselines `session-32-linux` and `session-33-linux-nas`. Build `work/bin/s47-final/` (master). Since sessions 41, 43 and 46 a NAS-scale development scan searches in about 24 s and `regress session-33-linux-nas` takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans, sweep reports of seeds other than 2026 (calibrate reads every sweep report there), and reports of opt-in variants must not sit in a reports directory during `regress`; run variants in a scratch work directory.
- Session 38's opt-in `--apart-in-speed` (confirmed by session 45; adoption ours, with an ADR) and session 37's `--overlap-by-chains` are variants, not defaults.
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen tonight. Pending listening (items 4, 5, 7 and 9 of session 40's pack, in `work/s47/questions/05-pack-s40.json`, session 39's questions, Star Trails, Into Music) stays for later.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-39.md and docs/brief-47.md, docs/session-notes.md ("Session 39", "Session 45" and "Session 47"), docs/status.md (newest entries), ADR 0006, 0017 and 0019, experiments 0026, 0076, 0091, 0102, 0107, 0182-0185 and 0210, the roadmap's "Possible plays inside a confident play of another recording", "A confidence statistic for shared material" and "The printed tracklist", and the code of `plays.rs`, the CLI's `export.rs` (`entries`, the marking), `mixes/drops.rs`, `tempo.rs`, `pair.rs`, `verifier.rs`, and the scripts `tracklist_score.py` and `tracklist_marked.py`.
2. Write docs/brief-48.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-48-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s48/listening-digests-start.txt; it must be the same at the end. Experiments number from 0215.
3. Build `work/bin/s48-start`; `regress session-32-linux` and `regress session-33-linux-nas` identical.

Steps:

1. Pre-registration, committed before any feature is computed on new mixes: the features (at least: whether the two files share material at all by the library's own evidence, from `pair` between the marked play's file and its partner's, or the clusters' related pairs; where the marked segment falls through that alignment relative to the confident play's position at the same moment; the marked play's hits outside the passages the two files share; the peak verifier's share on the marked passage; the speed relation to the partner), each defined exactly; the development data (session 39's renders or the same seeds rendered again: 2026 and 2028) and fresh test seeds that no feature is tuned on (at least two); the scores (marked plays that are records playing against those that are shared material, per scenario: separation, the share of playing records a rule would print and the share of shared material it would let into the tracklist); and what would count as a feature worth an ADR proposal. Also the merge measure: with drops lined up in beat phase, how often `same_recording` joins two different records into one entry, against session 39's tempo-only drops.
2. The harness, tested: beat-phase alignment for the drops kind (an opt-in option, so session 39's renders and reports stay reproducible), and the features in a script or the harness (harness only; `regress` identical).
3. The runs: the development seeds, then the test seeds, at 262 tracks and, if time allows, at NAS scale (a store-only copy without the held-out files, as session 39 did; never write the NAS store), under tmux with logs.
4. Experiments (at most 40 lines each, written before the next starts): the features on the development seeds; the registered rule on the test seeds; the phase-aligned merges.
5. Ahead, only when 1-4 are done: the same features on the development mixes' saved NAS-scale reports (the five marked plays of 0183's last paragraph), as a check that the rule would not print known shared material there.
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 48" section at the end of docs/session-notes.md with a handoff and the decision it leaves for us; update docs/status.md, the checklist and the roadmap's bullets above. Final checks: `scripts/check.sh` green, `regress session-32-linux` and `regress session-33-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the features' separation on the test seeds, what a rule would print and let in, the merges, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's behaviour does not change: no threshold, parameter, rule, default or output change; variants are opt-in, tested, and keep `regress` identical when off. Harness additions are tested and keep `regress` identical.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
- Scratch data goes under work/s48/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:59. Start no new run after about 11:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 48

- 2026-10-25, 02:35 by the Linux machine's clock (CET). The time box
  is read as 2.5 hours from about 02:40: finish by about 05:10.
- `master` at `28615bf`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s48/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `d3bb1fe061f7...`, the NAS clusters file
  `fdf7428a6322...`, `work/audition/answers.jsonl` `78c3fb0e7ce0...`
  (recorded as well, though no step here touches it).
- Experiments 0001-0214; this session numbers from 0215. Test set: 4 of
  5 evaluations used (none in this session).
- The start build `work/bin/s48-start/` is `target/release` of
  `28615bf`; cargo had nothing to rebuild, and both binaries are
  byte-identical to `work/bin/s47-final/`'s.
- Session 39's data: renders `work/mixes/drops-seed-{2026,2028}` and
  `drops-other-pairs-seed-{2026,2028}`; harness reports
  `work/reports/mixes-drops-seed-*.json`,
  `work/reports/mixes-drops-other-pairs-seed-*.json`; the CLI's reports
  `work/s39/cli-262/`, `cli-nas/`, `cli-nas-other/`; store-only copies
  `work/s39/store-{2026,2028}/`, NAS exclude lists
  `work/s39/nas-exclude-{2026,2028}.txt`, tag lists
  `work/s39/tags-262.json` and `tags-other-pairs.json`, panels
  `work/s39/panels/`.
- Sweep panels exist for seeds 2026-2029 and 7. Of these, 2027 and 2029
  hold a Clockwork record out (as session 39 found), so the drops kind
  at 262 tracks has no pair there. A new seed's sweep panel is drawn
  on first use into the `--panels` directory; this session's runs use a
  copy, `work/s48/panels/`, so no committed panel changes.
- Disk: 794 GB free; `work/mixes/` holds 23 GB.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build and both regressions (tmux `s48start`) | 02:35-02:55 |
| 1 | pre-registration in this brief; commit | 02:55-03:10 |
| 2 | `--aligned-drops` (drop points, beat phase) with tests; `marked_features.py`; `check.sh`, both regressions | 03:10-03:50 |
| 3 | test seeds: panels, store copies, renders, CLI at 262 and NAS scale; `pair` and the verifier for every marked play; aligned renders and the CLI | 03:30-04:20 |
| 4 | experiments 0215 (development), 0216 (test), 0217 (merges) | 04:00-04:35 |
| 5 | the development mixes' NAS-scale reports, if 1-4 are done | 04:35 |
| 6-7 | review, notes, status, roadmap, final checks | 04:35-05:10 |

If the time box runs short: NAS scale for the test seeds' other pairs
goes first, then the aligned drops at NAS scale.

## Decisions

- **(02:36) Start build and regressions** (`work/s48/start.sh`, tmux
  `s48start`, logs in `work/s48/logs/`; `run.sh` and `same.py` are
  session 47's with the paths changed).
- **(02:41) Start checks.** `regress session-32-linux` (0:51) and
  `regress session-33-linux-nas` (3:45) identical: every report
  identical to its baseline's apart from `wall_seconds`
  (`work/s48/same.py`).
