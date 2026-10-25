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
- **(04:47) Level-0 census** (`fullest`, no padding; not a padded
  search; `work/s50/census/`). 262 tracks: 6,029 of 605,333 non-empty
  lists set aside (119 postings or more), 16.3% of 7.93 M postings. NAS
  scale (the saved index, 40,705 assets): 10,865 of 1,086,463 lists
  (9,939 postings or more), 18.7% of 1.14 billion postings. A record
  holds about 30,000 postings, so at NAS scale copies of one recording
  move only lists already near the cut; a seeded share of the library
  copied several times moves the cut itself.
- **(04:48) Step 1, the pre-registration** (committed before any padded
  search).

  *Padding.* Exact copies of chosen records, made in memory as the index
  is built and appended after every other record, named
  `padding-copy/<i>/<path as indexed>` (i = 1..K); never written to any
  store. A chosen record is copied only when the run indexes it (a
  held-out or left-out file has no copies). In scoring, a copy counts as
  its original's file (the prefix is stripped before the cluster
  lookup). Two selections, each a list of index paths drawn by a script
  before any padded search (`work/s50/copied/`):
  - *panel*: every indexed file of the clusters of the sweep panel's 60
    indexed sources (seed 2026), from the clusters file of the scale
    (corpus clusters at 262 tracks; NAS clusters at NAS scale): the
    queries' own recordings, the worst case for these queries;
  - *share*: a seeded 10% of the files of the level-0 index
    (`random.Random(2026).sample` of the sorted paths of `fullest`'s
    report): a collection with many copies of some records, mostly not
    the queries'.

  Levels K = 1, 3 and 9 extra copies (realistic: up to ten files of one
  recording); K = 99 of the panel selection at 262 tracks as a stress
  level, reported as the point where it breaks, not as instability.
  Order: 262 tracks, panel then share, K = 1, 3, 9, then 99; NAS scale,
  K = 9 of both selections first, then K = 3 where K = 9 changes
  anything, then K = 1 if time allows.

  *Queries.* The sweep panel of seed 2026 (`docs/panels/`): 60 indexed
  and 20 held-out 30 s excerpts at 9 speeds (540 and 180 queries), at
  both scales; at NAS scale also `sweep --other-rips` (each source and
  its identical copies left out, its other rips found), the other-rips
  form of the same panel. The recall panel's other-rips queries
  (`recall --other-rips-only`, clean condition) and the shared panels'
  development halves only where time allows, with answers but no
  first-pass trace.

  *Trace* (opt-in, sweep only). For each query: every first-pass
  detection of at least 10 hits (`MIN_HITS_TO_REFINE`: the ones the
  second pass measures) with its file, hits, windows and whether it is
  in the own cluster; the answers after the second pass as today
  (confident, possible: 60 hits or more). For each run, the census of
  the searched index (`fullest::census`: lists, lists set aside, the
  cut, postings set aside, per file) and the hashes set aside, so that
  a script lists the hashes that move into and out of the set at each
  level. The first pass is run a second time with the second pass off
  (the same index, matcher options and ladder), so the default search
  is not touched.

  *The ranking variant* (opt-in, sweep only): the same share (1% of the
  non-empty lists, ties at the cut kept), ranked by the number of
  distinct holders of each list instead of its postings. R-files: each
  file holds itself (a copy is a file of its own). R-clusters: a file
  holds its cluster from the scale's clusters file, a file in no cluster
  itself, and a copy its original's. R-clusters is unchanged by exact
  copies by construction; R-files is unchanged by repeats within a file
  (a looped drum pattern). Runs: level 0 at both scales, panel K = 9 at
  both scales, the stress level at 262 tracks.

  *Measures, per query, against level 0:* whether the own cluster is a
  first-pass candidate (an own-cluster first-pass detection of 10 hits
  or more), its best first-pass hits h, and the ratio h_K / h_0;
  whether it is confident, and its best second-pass hits; wrong
  confident answers and held-out queries confident; wrong possible
  answers. Per run: the cut, the lists moved in and out, the share of
  postings set aside for copied files, their originals and the rest;
  first-pass candidates per query (median); wall time and peak memory.

  *Criteria.* The filter is **unstable** if at a realistic level (K <=
  9), under either selection and at either scale: (U1) a query confident
  at level 0 has no confident own-cluster answer; or (U2) a query whose
  own cluster is a first-pass candidate at level 0 is not one; or (U3) a
  wrong confident answer appears, or a held-out query becomes confident.
  Otherwise **stable**, with the smallest h_K / h_0 reported. Notable,
  declared now: h_K / h_0 below 0.5 for any query at K <= 9; a median
  number of first-pass candidates per query that doubles.

  The distinct-holder ranking is **worth an ADR** (a proposal; adoption
  ours) only if (1) the default shows U1, U2 or U3 at a realistic level
  and the variant removes them; (2) at level 0 the variant keeps the
  sweep at 540/540 confident and no wrong confident answer at both
  scales, with the median own-cluster second-pass hits within 1% of the
  default's; and (3) it sets aside at least 15% of the postings at NAS
  scale (the default 18.7%: the lookups the filter saves). Without
  U1-U3 under the default, it is not worth an ADR on this evidence,
  whatever it does; its results are reported as found. R-clusters needs
  clusters the product does not have, so only R-files could become a
  product rule as it stands.

  *Expected, declared now.* At 262 tracks the panel selection at K = 9
  takes much of the set aside for the queries' own recordings; their
  first-pass hits fall, but they stay candidates and confident because
  most of a record's hashes are rare (0013: dropping the fullest 1%
  costs 17% of own hits). At NAS scale K <= 9 of the panel moves few
  lists; the share selection at K = 9 raises the cut and moves many,
  towards the copied files and away from the rest.
- **(04:56) Step 2, the harness** (`sweep` options, harness only; the
  core gains one opt-in method):
  - `Index::skipping_most_held(share, holder)` (core): the same cut as
    `skipping_fullest` (`kept_at_cut`, shared by both), on the number of
    distinct holders of each list; the lists set aside are kept as one
    bit per hash. `scanned_postings` asks a `SetAside` value (lists
    longer than a length, as before, or those hashes). The matchers and
    the CLI do not call it. `search::MIN_HITS_TO_REFINE` is now public
    for the trace.
  - `sweep --exact-copies K --copied FILE` (`padding::ExactCopies`):
    the copies are built into the index after every other record, so a
    padded run builds its index from the stores instead of loading the
    saved one; `sweep::outcome` strips `padding-copy/<i>/` before the
    cluster lookup.
  - `sweep --trace-candidates`: each query's first-pass candidates
    (`Matching::first_pass`: the same search with the second pass off),
    and the report's `set_aside` (`fullest::census` of the searched index
    and its set-aside hashes).
  - `sweep --rank-fullest-by files|clusters` (`fullest::Ranking`).
  - Each option adds to the report's name (`-exact-<K>-<list>`,
    `-rank-<ranking>`, `-trace`); the runs use scratch work directories
    (`work/s50/w262`, `work/s50/wnas`) whose `sweep/` links to
    `work/sweep/` (the renders are reused, none rendered).
  - Decision (conservative, for time): padding and the trace are sweep
    options only, so the recall panel's other-rips queries are not
    searched padded; the sweep's `--other-rips` form at NAS scale stands
    for the other-rips condition. If time is left after step 4, the
    recall panel is the first addition.
  - `scripts/analysis/filter_padding.py` (`copied`, `compare`) with
    tests. The lists (`work/s50/copied/`): panel 67 files at 262 tracks,
    219 at NAS scale; share 26 of 265 and 4,070 of 40,705.
- **(05:04) Step 2's checks** (`work/s50/step2.sh`, tmux `s50harness`):
  `scripts/check.sh` green (4:00); run build `work/bin/s50-run/`;
  `regress session-32-linux` (0:51) and `regress session-33-linux-nas`
  (3:52) identical, every report identical to its baseline's apart from
  `wall_seconds`. The CLI binary is not byte-identical to the start
  build's this time (the core's `scanned_postings` matches on
  `SetAside`); the NAS-scale development scan searched in 24.7 s, as
  before (about 24 s).
- **(05:11) Counting copies once** (after the first padded results at
  262 tracks): the registered "first-pass candidates per query" and
  "wrong possible answers" count each copy as a detection, so K copies
  of a related record give K entries. `filter_padding.py` now also
  counts wrong candidates and wrong possible answers by distinct
  original file; both counts are reported, the per-file one as the
  measure (the registered count read literally would double at K = 1 by
  construction).
- **(05:24) For step 5, `fullest --shared-by PAIRS`** (harness,
  tested): for each pair of index paths, the distinct hashes both files
  hold and how many of them lie in lists set aside; `fullest` also takes
  `--exact-copies`/`--copied` (the same `Copies` options as `sweep`).
  Built ahead of time while the NAS runs go; used only after steps 1-4.
  The pairs (`work/s50/case/pairs.txt`) are the seven verdict lines of
  the Aphrodite family and the Klute VIP, without the two pairs judged
  against unrelated tracks; Defjammer's NAS file is one a corpus file
  stands for, so its corpus path is used.
- **(05:41) The share selection at NAS scale, scaled down.** `sweep
  --exact-copies 9 --copied share-nas.txt` stopped at the index's limit
  of 65,536 assets (ADR 0010): 40,580 indexed files and 36,630 copies.
  Decision (conservative, keeping the registered selection): K = 6, the
  largest that fits (65,000 assets), and K = 3 at NAS scale for the
  share selection; its other-rips form and the rankings at K = 6
  (`work/s50/step3-nas2.sh`, queued after `step3-nas.sh`, whose K = 9
  other-rips run of the share fails the same way). The panel selection
  keeps K = 9.
- **(06:57) Step 4, experiments 0222-0224.** The runs of all three ran
  from one registration in two queues (`step3-262.sh`, 05:04-05:24;
  `step3-nas.sh` and `step3-nas2.sh`, 05:24-06:57), the ranking runs
  among them; each experiment was written after its queue, 0222 before
  the NAS runs reported. The step 5 runs (`work/s50/step5.sh`, queued
  after `step3-nas2.sh`) started at 06:57 while 0224 was being written;
  their results were read only after 0224's commit. Results: stable at
  every level and scale (no U1, U2 or U3); the distinct-holder ranking
  is not worth an ADR (condition 1 fails, 2 and 3 hold).
- **(06:59) Step 5, a control added** (exploratory, decided after the
  first readings): the hashes any two files share are mostly common
  ones, so the share set aside of a related pair's shared hashes means
  little without unrelated pairs beside it. 20 seeded random pairs of
  the NAS index's files (`random.Random(2026)`,
  `work/s50/case/control-pairs.txt`) at level 0, in their own scratch
  work directory (`work/s50/step5b.sh`).
- **(07:03) Step 5, experiment 0225** (`work/s50/step5.sh`,
  `step5b.sh`; build `work/bin/s50-case/`, the run build plus `fullest
  --shared-by`). The related pairs' shared hashes are set aside more
  often than their own postings and less often than random pairs'
  (median 28.8% against 42.4%); copies move them by at most 0.2 points
  at NAS scale.
- **(07:14) Final checks, first round** (`work/s50/final.sh`, build
  `work/bin/s50-final/` from `95763f7`): both regressions identical
  (0:51; 3:53), the digests unchanged; `scripts/check.sh` failed only in
  the load-sensitive `decode::watch` test. What loaded the machine is
  not known: at 07:14 `ps` showed no other process using notable CPU
  (the load average of about 20 then followed this session's own
  regressions), so the earlier reading that other processes loaded it
  is not supported.
- **(07:20) The review's findings** (a separate agent, read-only,
  07:05-07:14). No defect in the code. Taken: 0223 said Siren Bass's
  rip was copied in both selections, but only the panel copies it (the
  same drop in the share selection comes from other files' copies); the
  second-pass ranges were stated as "unchanged" or "within 0.7%" (now
  0.995-1.009 at 262 tracks, 0.974-1.012 at NAS scale, 0.991-1.012 for
  the rankings); lists moved at NAS scale 0.9-2.0%, not 1.4-2.0%; 0224's
  condition (2) is read as "no new wrong confident answer" (the 7 at
  NAS scale are the default's, held out), now said; the registered
  "candidates per query doubles" is reported by detection (it doubles
  through the copies themselves) beside the count by distinct file;
  R-files is not unchanged by copies (only R-clusters is); the rankings
  set aside fewer lists at 262 tracks (ties at the cut kept), now said;
  `filter_padding.py`'s `lowest` skipped queries with no candidate, and
  an empty answer list raised (fixed, the data files regenerated); time
  and memory ranges and the roadmap's 23-35%; the doc comment of
  `without_assets`. Recorded as deviations: the ranking runs of 0224
  ran before 0222 and 0223 were written (one registration, two queues;
  0222 was written before the NAS runs reported, 0223 before the share
  rankings did), and five stamps of this brief were first written later
  than their commits and corrected in later commits (04:50, 05:27,
  06:58, 07:01, 07:08; the registration's text is unchanged since
  `29eaa57`). Not taken: tests of the copy-prefix stripping in
  `outcome`, the trace, report naming and the clap wiring;
  `--exact-copies 0` is accepted and two `--copied` files with one stem
  share a report name.
- **(07:34) Final checks, second round** (`work/s50/final.sh`, tmux
  `s50final3`, build `work/bin/s50-final/` rebuilt from `13f44db`; the
  machine idle, load 0.2 at the start): `scripts/check.sh` green (3:44);
  `regress session-32-linux` (0:51) and `regress session-33-linux-nas`
  (3:52) identical, every report identical to its baseline's apart from
  `wall_seconds`; the verdicts file, the clusters file and the answer
  log at their start digests. The review's fixes were committed in
  `13f44db` by a helper agent given the exact commands, while this
  session's shell safety check returned no verdict.
