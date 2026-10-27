# Session 59 brief: why the prune moved detections of files it did not touch

Written on 2026-10-27, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-59-checklist.md` (progress).
3. ADR 0014 (the harness's saved index); the roadmap's "The filter and
   famous breaks".
4. `docs/session-notes.md` ("Session 52", "Session 54" and "Session
   57"); experiments 0231, 0238, 0222-0225 and this session's (0242 and
   up).

`docs/brief-2.md` to `docs/brief-58.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 59 of Gunfinger: why pruning five files changed detections of files the prune never touched, and two missing tests. Session 57 (experiments 0236-0240, docs/session-notes.md "Session 57", brief 57) found that after session 52 pruned five records from the NAS store, some confident detections of other files differ by a hit or two, and in one recall brief query (911, Organikismness - Echos at +1%) a confident right detection went from 455 hits at +1.147% to 503 at +0.933%. Binaries of sessions 33, 40 and 48 run on today's store reproduce the change, so the store's content causes it, not the code; `work/s57/trace-brief33.sh` reproduces it. A plausible mechanism, to be checked rather than assumed: the first pass sets aside the fullest 1% of posting lists (ADR 0014), chosen by length over the whole index, so removing five files' postings can move which hashes are set aside, and that changes which candidates and lines the first pass finds (the second pass counts every hash again, but its fitted speed and span start from the first pass). Other candidates: asset numbering, tie-breaking by asset order, or the index's revision. This session finds the cause, says whether it is expected behaviour or a defect, and records it. Also, session 54's handoff left the `robust` and `grid` same-title branches without unit tests. It is a measurement session.

You run unattended: the user may not answer. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).

Time box: started about 12:45 (CET); aim to finish by about 14:50 (the user's soft deadline is 15:00; work in progress may run past it). If a step would run past that, scale it down, record the decision in the brief and continue.

Environment (sessions 33-58's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,678 records, revision `1fc6fe50293ad330`. NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `ce293fe6165e...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-54-linux-nas`. Tonight's changes: session 52 pruned five ignored files (0231); session 53 made `SpansAtSpeed(0.006)` the default second suppression (ADR 0021); session 54 scores same-title detections as right (ADR 0022); sessions 55-56 print gaps with estimates (ADR 0023, 0024); session 57 reran the protocol (0236-0240); session 58 added `gunfinger-eval parts` (0241).
- Build `work/bin/s58-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans, sweep reports of seeds other than 2026 (calibrate reads every sweep report there), and reports of opt-in variants must not sit in a reports directory during `regress`; run variants in a scratch work directory.
- The user's decisions of 2026-10-27 (docs/status.md, "session 39's listening questions, and four decisions"; the roadmap's bullets carry them): `--apart-in-speed 0.006` becomes the default; gaps are printed by default ("ID - ID", 45 s or more, the mix's start and end, possible lines kept, an estimate of the tracks a gap holds, JSON yes, CSV and cue sheet unchanged); a confident detection of a near-identical version counts as right; the minimum span stays undecided. Each is implemented with an ADR (the next number is in `docs/adr/`).
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user may be away and cannot listen now. Pending listening (Star Trails, Into Music, session 58's pack) stays for later. When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-57.md and the newest brief, docs/session-notes.md ("Session 52", "Session 54" and "Session 57"), docs/status.md (the newest entries), ADR 0014, experiments 0231, 0238 and 0222-0225 (the filter under padding), `work/s57/trace-brief33.sh`, and the code of `index.rs` (`skipping_fullest`, how the fullest lists are chosen and how assets are numbered), `search/lines.rs`, `search/refine.rs` and the harness's `--trace`.
2. Write docs/brief-59.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-59-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s59/listening-digests-start.txt. Experiments number from 0242.
3. Build `work/bin/s59-start`; `regress session-54-linux` and `regress session-54-linux-nas` identical.

Steps:

1. The trace: for query 911 (and one or two of the hit-count changes), compare the first pass before and after the prune without writing any store: an index built from the store with the five records' peaks added back in memory if the harness can (session 50's padding option adds copies in memory; a read-only copy of the old records may sit in a baseline or saved index), or the old saved index if one is kept. Find which hashes move into or out of the skipped set, which candidates or lines change, and how that moves the fitted speed and the hits. Name the cause with evidence.
2. Judge it: is a detection's hit count or fitted speed depending on the rest of the library expected under ADR 0014 (the filter's share is a share of the whole index), or a defect? If it is expected, say so in ADR 0014's terms and in the roadmap's "The filter and famous breaks"; if it is a defect, describe it, reproduce it with the smallest command, and do not change the product.
3. The tests: unit tests for the same-title branches of `robust` and `grid` (session 54's handoff), harness only, `regress` identical.
4. One experiment (at most 40 lines): the cause, the evidence, and the judgment.
5. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
6. Wrap-up: a "Session 59" section at the end of docs/session-notes.md with a handoff; update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-54-linux` and `regress session-54-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the cause, whether it is expected, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's behaviour does not change: no threshold, parameter, rule, default or output change. Harness additions are tested and keep `regress` identical. Never write any peak store; in-memory indexes only.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
- Scratch data goes under work/s59/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 15:00. Start no new run after about 14:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 59

- 2026-10-27, 12:38 by the Linux machine's clock (CET). Aim to finish by
  about 14:50; the soft deadline is 15:00, and no new run starts after
  about 14:30.
- `master` at `2d6d549`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s59/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `ce293fe6165e...`, `work/audition/answers.jsonl` `3e50aaa62181...`.
- Experiments 0001-0241; this session numbers from 0242. ADRs
  0001-0024; this session writes none (a measurement session). Test
  set: 4 of 5 evaluations used (none in this session).
- Baselines `session-54-linux` (262 tracks) and `session-54-linux-nas`.
  Build `work/bin/s58-final/` (master).
- The old index survives. The harness's saved index of the NAS store
  from before the prune (2026-10-21, 4,578,598,764 bytes, the NAS store's
  40,683 records then) is still on disk as 14 hard links in older
  sessions' work directories (`work/s36/w/indexes/library-nas-dnb-peaks.index`
  among them); `work/indexes/` holds the one session 52 built after the
  prune (40,678 records). Both are only read this session. The five
  pruned records' peaks are gone from the store, so the old file is the
  only copy of their postings.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build, both regressions (tmux `s59start`) | 12:38-12:55 |
| 1 | the trace: a harness option that searches today's postings with the lists the old index set aside (read from the old saved file, in memory), a first-pass trace of recall queries; runs on recall's brief queries; the moved lists counted | 12:55-13:55 |
| 2 | the judgment under ADR 0014; the roadmap's "The filter and famous breaks" | 13:55-14:05 |
| 3 | unit tests of the same-title branches of `robust` and `grid` | 14:05-14:20 |
| 4 | experiment 0242 | 14:20-14:30 |
| 5-6 | review by a separate agent; notes, status, checklist, final checks | 14:30-14:50 |

## Decisions

- **(12:39) Start build and regressions** (`work/s59/start.sh`, tmux
  `s59start`, logs in `work/s59/logs/`; `run.sh` and `same.py` are
  session 58's with the paths changed). The build was a no-op: both
  binaries `cmp` equal to `work/bin/s58-final/`.
- **(12:44) Start checks.** `regress session-54-linux` (0:50) and
  `regress session-54-linux-nas` (3:47, 5.4 GiB peak) identical: every
  report equals its baseline's apart from `wall_seconds` (`same.py`).
- **(12:46) The "before" index is the pre-prune saved file**, read in
  memory and never written: `work/s36/w/indexes/library-nas-dnb-peaks.index`
  (one of the 14 links). Runs never get `--work work/s36/w`, so no
  harness run can replace it. No peak record is rebuilt: the old file
  holds the five records' postings exactly as the old runs searched them.
- **(12:48) The harness for the trace** (committed at `b900171`, tested):
  `recall --set-aside-like FILE` loads the saved file by the provenance
  its own header names (`saved::provenance`, new in the core, a reader
  only), leaves out the run's held-out files, sets aside its fullest 1%
  of lists, and searches today's postings with those lists set aside
  instead of today's (`Index::skipping_hashes`); it also records
  whether the old index less the files only it holds has today's
  postings, and every list set aside by one index only.
  `--trace-candidates` keeps each query's first-pass candidates with
  their speed and span. Both are opt-in and add to the report's name.
- **(12:49) The runs** (tmux `s59like`, `s59today`; scratch work
  directory `work/s59/wrecall` with the renders as reflink copies):
  recall's clean and brief queries with `--overlap-by-spans`, the rule
  the old reports (session 33's, before ADR 0021) were made with, once
  with the old index's lists set aside and once with today's. If the
  first reproduces the old reports except the pruned files' own
  detections, the lists set aside alone carry the change.
- **(12:50) The two indexes.** The old index less the five pruned files
  has today's postings, posting for posting (so neither the asset
  numbering nor anything else in the store differs). The lists set
  aside differ by one: the old index sets aside 10,863, today's 10,864.
- **(12:54) Step 3** (committed at `0672e5b`): `robust`'s per-detection
  counting is now `Role::of` and `count`, `grid`'s match of detections
  against the brief play `matching_the_play`, unchanged in behaviour;
  five tests cover the same-title branches.
- **(13:00) Clean queries done** (`work/s59/compare.py`, by position;
  the pruned files' detections set aside): with the old index's lists set
  aside, today's postings give the old clean report (session 33's)
  exactly, 0 of 702 queries different; with today's lists, the three
  moves session 57 found (two weak 6-hit candidates and Echos at -8%
  refitted by 0.000003%). The one list that moved (hash 1,286,800) holds
  9,930 postings in both indexes and none of a pruned file: recall's
  index (the held-out clusters left out) set aside lists longer than
  9,930 before the prune and longer than 9,929 after it.
- **(13:02) The whole index's cut** (`work/s59/py/cut.py`, reading the
  offsets tables of both saved files): 1,086,463 lists, 10,865 set
  aside, longest kept 9,938, in both; no list crosses it. The prune moves
  nothing in runs of the whole index (the development scan); runs that
  leave files out (recall, sweeps, the leave-out scans of `regress`)
  have cuts of their own, measured this session for recall only.
- **(13:20) Brief queries done** (both runs 29:20-29:27 at once, 5.6 and
  8.7 GiB peak): with the old index's lists set aside, today's postings
  give the old brief report exactly, 0 of 1,404 queries different (the
  pruned files' 27 detections set aside); with today's lists, only
  queries 910 and 911 (Echos at +1%) differ, and the run equals session
  57's `--overlap-by-spans` report. Query 911's first-pass candidate:
  370 hits at +1.147% with the old lists, 369 at +0.933% with today's
  (3 windows, 3.13-22.81 s both); the second pass measures 455 and 503
  hits at those speeds. One hit fewer in the first pass moves the
  chain's speed by 0.21 percentage points. The same list moved in recall's brief index
  (hash 1,286,800).
- **(13:22) The chain's lines added to the trace** (committed at
  `e5a6702`, tested): `Matching::first_pass` returns the core's trace and
  each candidate keeps its chain's lines, to see which line moves the
  speed. The brief queries run again in both conditions (tmux
  `s59lines`, build `work/bin/s59-run2/`; the first run's reports moved
  to `variant-spans/run1/`). Clean queries are not run again: their
  three moves are small and the brief query is the one in question.
- **(13:41) The line trace done** (18:44-18:57, both at once): the
  reports equal the first run's query for query. Query 911's window 1
  (10-20 s) is the +1.2% rung's line of 198 hits with the old lists and
  the +0.8% rung's line of 197 with today's; the chain's speed is the
  hit-weighted mean of its three lines' rungs (+1.147%, +0.933%). By
  inference the moved list cost the +1.2% line one hit and `distinct`
  kept the other view at a tie.
- **(13:43) The judgment: expected, not a defect.** The lists set aside
  are a share of the index each run searches (ADR 0014); one list at a
  tie moved, and a play between two rungs makes a short chain's speed
  hinge on one hit. Both detections are confident and right. Recorded in
  ADR 0014's consequences (a dated note; the decision does not change)
  and the roadmap's "The filter and famous breaks", whose sentence that
  a candidate's evidence does not depend on the filter it now qualifies.
  Experiment 0242. The analysis scripts are committed in
  `scripts/analysis/` (`recall_by_position.py`, `chain_lines.py`,
  `saved_index_cut.py`).
- **(13:43) The CLI binary is not byte-identical** to `s58-final`'s: the
  core gained `Index::skipping_hashes` and `saved::provenance`, which
  the CLI never calls. The final checks compare the CLI's JSON report of
  the Stakka mix from both builds.
- **(13:53) Final checks** with `work/bin/s59-final/` (`cmp`-equal to
  `s59-run2/`): `scripts/check.sh` green (4:50); `regress
  session-54-linux` (0:52) and `regress session-54-linux-nas` (3:49)
  identical, 16 of 16 reports equal to their baselines' apart from
  `wall_seconds`; the CLI's JSON report of the Stakka mix identical from
  `s58-final` and `s59-final`; the digests unchanged.
- **(13:55) The review** (a read-only agent, 13:44-13:52): no code
  defect; numbers, comparisons and the judgment confirmed. Fixed: 0242,
  the notes and this brief no longer say `regress` is untouched (its
  leave-out scans and sweep set their own cut; only recall's was
  measured); the roadmap marks the two ties as inferences; ADR 0014 and
  the roadmap give the conditions of "the old reports exactly"; the
  memory figures (5.6 and 8.7 GiB); the checklist; one sentence of this
  brief; `saved_index_cut.py` rounds as Rust does (no effect at 1,086,463
  lists). Left as notes: the help text of `--trace-candidates` does not
  name the chain's lines (a rebuild for a help string was not worth the
  checks again), and 0242 points to `work/s59/trace-*.sh` for commands.
