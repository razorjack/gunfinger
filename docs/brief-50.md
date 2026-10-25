# Session 50 brief: the fullest-list filter and many copies of the same records

Written on 2026-10-25, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-50-checklist.md` (progress).
3. ADR 0014; the roadmap's "The filter and famous breaks", "A confidence
   statistic for shared material", "Behaviour at scale" and "Recall
   beyond the corpus recordings"; experiments 0012, 0013, 0017,
   0049-0052, 0094, 0143-0145 and this session's (0222 and up).
4. `docs/session-notes.md` ("Session 48", "Session 49"), `docs/status.md`
   (newest entries, "after session 46" among them); the code of
   `index.rs` (`skipping_fullest`, `longest_kept`, `scanned_postings`),
   `search.rs` (`Matcher::index`, `detections_and_chains`),
   `search/lines.rs` (`in_windows`), `search/refine.rs`
   (`MIN_HITS_TO_REFINE`), and in the harness `padding.rs`, `fullest.rs`,
   `sweep.rs` and `matching.rs`.

`docs/brief-2.md` to `docs/brief-49.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 50 of Gunfinger: is the fullest-list filter stable when a library holds many copies of the same records? The roadmap's "The filter and famous breaks" (with a proposal from an external review, 2026-10-17): the first pass sets aside the fullest 1% of posting lists (`Index::skipping_fullest`, ADR 0014; the second pass counts those hashes again), and the lists are ranked by length, which counts every copy of a recording. A collection with many rips of the same releases could push a track's own hashes, or a shared drum loop's, into the skipped lists without adding new music, and then the track stops becoming a candidate. The listening of 2026-10-25 added two more pairs where one drum loop dominates different tracks (Aphrodite's Wikki Wikki Plate ~ Defjammer, Klute's We R The Ones VIP ~ the Klute Remix; docs/status.md, "after session 46"). At 40,683 records with real duplicate rips, other rips of the recall panel stayed confident (experiment 0144), so this checks stability rather than a known failure. This session pads indexes with exact copies of existing records, searches fixed development queries, and traces which files become candidates in the first pass, apart from the second pass's counts; and compares ranking the lists by distinct recordings (clusters) instead of by postings. It is a measurement session: the filter, its share and the product do not change; a change is ours, with an ADR.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32). Padded indexes grow in memory: keep one search at a time and watch peak memory (the NAS-scale index is 4.6 GB).

Time box: started about 04:42 (CET; the clocks went back an hour tonight); aim to finish within about 3 hours, because another session may follow you. If a step would run past that, scale it down (fewer padding levels, 262 tracks first), record the decision in the brief and continue.

Environment (sessions 33-49's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records), library `/mnt/atlas/Music/dnb` (read only; on Ethernet again tonight), reached with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: after the listening of 2026-10-25 (docs/status.md, "after session 46") SHA-256 `fdf7428a6322...`, the same 284 clusters as before. Baselines `session-32-linux` and `session-33-linux-nas`. Build `work/bin/s49-final/` (master). Since sessions 41, 43 and 46 a NAS-scale development scan searches in about 24 s and `regress session-33-linux-nas` takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans, sweep reports of seeds other than 2026 (calibrate reads every sweep report there), and reports of opt-in variants must not sit in a reports directory during `regress`; run variants in a scratch work directory.
- Session 38's opt-in `--apart-in-speed` (confirmed by session 45; adoption ours, with an ADR) and session 37's `--overlap-by-chains` are variants, not defaults.
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen tonight. Pending listening (items 4, 5, 7 and 9 of session 40's pack, in `work/s47/questions/05-pack-s40.json`, session 39's questions, Star Trails, Into Music) stays for later.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-48.md and docs/brief-49.md, docs/session-notes.md ("Session 48" and "Session 49"), docs/status.md (newest entries), ADR 0014, the roadmap's "The filter and famous breaks", "A confidence statistic for shared material", "Behaviour at scale" and "Recall beyond the corpus recordings", experiments 0012, 0013, 0017, 0049-0052, 0094, 0143-0145, and the code of `index.rs` (`skipping_fullest`, how the fullest lists are chosen), `search/lines.rs` (`in_windows`, `scanned_postings`), `search/refine.rs`, `fullest.rs`, `holders.rs`, `sweep.rs`, `recall.rs` and how `--other-sample` builds an index.
2. Write docs/brief-50.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-50-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s50/listening-digests-start.txt; it must be the same at the end. Experiments number from 0222.
3. Build `work/bin/s50-start`; `regress session-32-linux` and `regress session-33-linux-nas` identical.

Steps:

1. Pre-registration, committed before any padded search: the padding (exact copies of existing records added to the index as new assets, never written to any store: which records are copied, how many copies, for example 1, 3 and 9 extra copies of every record of chosen families, or of a seeded share of the library), the queries (fixed development queries: the sweep panel of seed 2026, the recall panel's other-rips queries, the shared panels' development halves where time allows), the trace (for each query, the files that become first-pass candidates and their first-pass hits, and the confident and possible answers after the second pass; the hashes that move into the skipped set at each level), the comparison ranking (lists ranked by the number of distinct clusters or files holding them, rather than by postings), and the criteria: what change in candidates or answers would show the filter is unstable, and what would make the distinct-holder ranking worth an ADR.
2. The harness, tested: padding as an opt-in harness option (in memory only; the copies count as their originals' cluster for scoring), the first-pass candidate trace if `--trace` or the existing reports do not give it, and the distinct-holder ranking as an opt-in variant (harness only; `regress` identical with them off).
3. The runs: at 262 tracks first (cheap, padding levels in turn), then at NAS scale for the levels that matter, under tmux with logs.
4. Experiments (at most 40 lines each, written before the next starts): what padding moves into the skipped lists, which queries lose candidates or answers, and the distinct-holder ranking against the default.
5. Ahead, only when 1-4 are done: the two drum-loop pairs of 2026-10-25 and the Aphrodite family, as a small case study: how many of their shared hashes sit in the skipped lists now, and under padding.
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 50" section at the end of docs/session-notes.md with a handoff and the questions it leaves for us; update docs/status.md, the checklist and the roadmap's bullets above. Final checks: `scripts/check.sh` green, `regress session-32-linux` and `regress session-33-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: what padding does to the skipped lists, candidates and answers, the ranking variant's result, and anything left.

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
- Scratch data goes under work/s50/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:59. Start no new run after about 11:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 50

- 2026-10-25, 04:39 by the Linux machine's clock (CET). The time box is
  read as 3 hours from about 04:42: finish by about 07:45.
- `master` at `4f135b7`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s50/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `d3bb1fe061f7...`, the NAS clusters file
  `fdf7428a6322...`, `work/audition/answers.jsonl` `78c3fb0e7ce0...`
  (recorded as well, though no step here touches it).
- Experiments 0001-0221; this session numbers from 0222. Test set: 4 of
  5 evaluations used (none in this session).
- How the filter works today (`index.rs`): `Matcher::Fitted` calls
  `skipping_fullest(0.01)`, which sorts the lengths of the non-empty
  lists, takes the length at rank round(1% of them) and sets aside every
  list longer than it (ties at the cut are kept). Only
  `scanned_postings`, which the first pass reads (`lines::in_windows`),
  leaves them out; the second pass (`refine`) counts every list for each
  first-pass detection of at least 10 hits (`MIN_HITS_TO_REFINE`) over
  its chain's span and 10 s either side. With 21-bit hashes there are at
  most 2,097,152 lists. Two consequences fixed by the code: exact copies
  add no new hash, so the number of lists set aside does not change; and
  copying every record the same number of times multiplies every length
  alike and moves nothing. Only uneven copying can move lists.
- A first-pass trace does not exist: `search::trace_with` (for
  `explain`) returns the first pass's lines and the second pass's
  detections, and a candidate dropped or left unrefined is not
  distinguishable there; the sweep report keeps detections of two
  windows or more after the second pass.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build and both regressions (tmux `s50start`) | 04:39-05:00 |
| 1 | pre-registration in this brief; commit | 05:00-05:10 |
| 2 | harness: exact copies, the candidate trace, the distinct-holder ranking; tests; both regressions | 05:10-06:00 |
| 3 | runs at 262 tracks, then NAS scale | 06:00-06:40 |
| 4 | experiments 0222 and up | 06:20-07:00 |
| 5 | the drum-loop pairs (if time) | 07:00-07:15 |
| 6-7 | review, notes, status, roadmap, final checks | 07:10-07:50 |

## Decisions

- **(04:40) Start build and regressions** (`work/s50/start.sh`, tmux
  `s50start`, logs in `work/s50/logs/`; `run.sh` and `same.py` are
  session 49's with the paths changed). The start build is
  byte-identical to `work/bin/s49-final/` (cargo had nothing to
  rebuild). `regress session-32-linux` (0:51) and `regress
  session-33-linux-nas` (3:45, 5.5 GB peak) identical: every report
  identical to its baseline's apart from `wall_seconds`.
