# Session 40 brief: where two recordings stop lining up, as evidence for same or different

Written by the agent for itself on 2026-10-23, so the plan survives
context compaction. Work autonomously and never stop to ask: where a
decision is needed, take the conservative option, write it down here and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-40-checklist.md` (progress).
3. `docs/status.md` ("after session 38" and the newest entries); the
   roadmap's "The verdicts by ear on borderline pairs", "Clustering
   precision" and the version verifier's bullets; experiments 0048,
   0049, 0057, 0058 and this session's (0186 and up);
   `docs/research/2026-10-16-version-verification.md`.
4. `docs/session-notes.md` ("Session 38" to the end),
   `docs/pair-verdicts.txt` (read only),
   `work/audition/listening-comments-2026-10-22.md`, the code of
   `crates/gunfinger-eval/src/pair.rs` and `clusters.rs`, and
   `scripts/analysis/listening_pack.py` and `pair_offsets.py`.

`docs/brief-2.md` to `docs/brief-39.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 40 of Gunfinger: where two recordings stop lining up, as evidence for same or different, measured on every pair judged by ear. The listening of 2026-10-22 (docs/status.md, "after session 38"; the roadmap's "The verdicts by ear on borderline pairs") gave two pairs that neither coverage nor supported time separates: China Cup against ƆƐI3C's upload (78% coverage, supported 0.90, judged different: almost identical, but the hi-hats differ for a few bars) and Dilusion against dfect-dnb's upload (73%, 0.91, judged same: one copy has a needle skip or cut that shifts the rest by a few seconds). Their sheets (`work/listening/borderline-*/sheet.txt`) show a difference in where the other alignments lie: Dilusion's sit within a few seconds of the main offset, China Cup's 60 and 250 s away, leaving 4:20-5:41 with nothing near the main offset. The share of the shorter file held by alignments within 10 s of the main offset is 0.91 against 0.77. This came from two pairs. This session measures such offset-aware measures on every judged pair, and on larger labelled populations, against coverage and supported time. It is a measurement session: the clusters' join rule, the product and the clusters file do not change.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).

Time box: started about 02:10; aim to finish within about 3 hours, because another session may follow you tonight. If a step would run past that, scale it down, record the decision in the brief and continue.

Environment (sessions 33-39's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records), library `/mnt/atlas/Music/dnb` (read only), reached with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: since the listening of 2026-10-22 (docs/status.md, "after session 38") SHA-256 `d9c85c4586ad...`, 284 clusters, Dilusion's upload joined to its cluster; `regress session-33-linux-nas` is identical with it. Baselines `session-32-linux` and `session-33-linux-nas`. Session 39 ended at `5c02814` (its final harness build is in `work/bin/s39-run2/` or the newest `work/bin/s39-*`; your own start build is what counts). `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- Session 38's opt-in `--apart-in-speed` and session 37's `--overlap-by-chains` are variants, not defaults; their adoption waits for an ADR (ours).
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans must not sit in a reports directory during `regress`.
- `pair` writes `reports/.../pairs-<file stem>.json`: give your pair files stems of their own so that `work/reports/library-nas-dnb-peaks/pairs-pairs.json` (the 2026-10-22 pack's report) is not overwritten.
- Renders from earlier sessions sit in `work/mixes/`; leave them. This session needs no renders.
- The listening pack of 2026-10-22 and its pairs report: `work/listening/` (`pairs.json`, `items.json`, `store-paths.txt`, each item's `sheet.txt`) and `work/reports/library-nas-dnb-peaks/pairs-pairs.json`; the clusters file before that listening's verdicts: `work/listening-2026-10-22/duplicate-clusters-before.json`.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-39.md, docs/session-notes.md ("Session 38" and "Session 39"), docs/status.md (newest entries, including "after session 38"), the roadmap's "The verdicts by ear on borderline pairs", "Clustering precision" and the version verifier's bullets, experiments 0048, 0049, 0057 and 0058, `docs/research/2026-10-16-version-verification.md`, docs/pair-verdicts.txt (read only) and its comments in docs/status.md's listening entries, `work/audition/listening-comments-2026-10-22.md`, the code of `pair.rs`, `pair_review.rs`, `clusters.rs` (the join rule, coverage, supported time, sparse joins) and `versions.rs`, and `scripts/analysis/listening_pack.py`.
2. Write docs/brief-40.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-40-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s40/listening-digests-start.txt; it must be the same at the end. Experiments number from 0186.
3. Build `work/bin/s40-start`; `regress session-32-linux` and `regress session-33-linux-nas` identical.

Steps:

1. Pre-registration, committed before any measure is computed on anything but the two pairs above (say that those two are the hypothesis's source and are reported apart):
   - The labelled populations: (a) every verdict by ear in docs/pair-verdicts.txt whose two files still have records (say how many do not, and why); (b) a seeded sample of the clusters' joins that no verdict touches (coverage of 80% or more, presumed the same recording; label them "presumed", never "judged"); (c) pairs of different recordings that share material from the frozen panels in `docs/panels/` (original against remix or VIP, labelled by the panels' construction); (d) the strongest pairs between files of different artist and title tags among the clusters' "related" pairs, as a population of presumed-different pairs. Fix sample sizes and seeds before running.
   - The measures, each computed from the same `pair` alignments: coverage and supported time (today's), hits per second of aligned span, the share of the shorter file held by alignments within T s of the main alignment's offset (T = 2, 5, 10, 20), the longest stretch of the shorter file with no aligned hit near the main offset, and the number of offset jumps between consecutive alignments. Define each exactly.
   - The scores: for each measure and population pair, how well it orders same above different (the share of same/different pairs ordered correctly, that is the AUC), the best threshold's errors with leave-one-out cross-validation on the judged pairs, and the judged pairs each measure gets wrong, listed by name. What would count as a measure worth a roadmap entry (for example: on the judged pairs, fewer errors than coverage and supported time at their best thresholds, and no presumed-same join of population (b) placed below the threshold).
2. The harness or a script, tested: compute the measures from `pair` reports (`scripts/analysis/`, with unit tests on fixtures, or in `pair.rs` if the alignments it reports are not enough; harness only, `regress` identical). Also, in `listening_pack.py select`, a minimum of hits for a borderline pair (`--min-hits`, default 30, the floor `clusters` uses for related recordings), tested: the 2026-10-22 pack held Dieselboy - The Descent ~ Frenchi / Proktah - Scarabeus at 41% coverage on 6 hits, completely different by ear.
3. The runs: `gunfinger-eval --other-peaks-dir ... pair` on each population (NAS scale), under tmux if long. Never write the NAS store, the clusters file or docs/pair-verdicts.txt.
4. Experiments (at most 40 lines each, written before the next starts): the measures against the pre-registered scores on each population; the judged pairs every measure gets wrong, and what their sheets show; whether the offset-aware measures separate near-identical versions (China Cup's kind) from damaged copies of one recording (Dilusion's kind).
5. Ahead, only when 1-4 are done, in this order:
   a. The false confident detections left at NAS scale (calibrate's "false accepted" in `regress session-33-linux-nas`, and the wrong confident detections in the development scans and the sweep at NAS scale): which recordings they pair, the measures for each pair, and how many are near-identical versions by the measures (the roadmap says China Cup's upload accounts for most of them; check). Whether such a detection counts as wrong is ours to decide; report the numbers.
   b. A listening pack of the pairs whose measures disagree most with the clusters' decision (joined but low on the best measure, or apart but high), built with `listening_pack.py select/render` into `work/s40/pack/`, for the user to judge later. Do not listen, do not write verdicts.
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 40" section at the end of docs/session-notes.md with a handoff and the questions it leaves for us (a change to the join rule or to how detections of near-identical versions are scored is ours, with an ADR); update docs/status.md, the checklist and the roadmap's bullets above. Final checks: `scripts/check.sh` green, `regress session-32-linux` and `regress session-33-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: what was built, each measure's scores in a few lines, the pairs every measure gets wrong, the pack's path, and anything left.

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
- Scratch data goes under work/s40/ (your session number).
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00. Start no new run after about 10:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 40

- 2026-10-23, 02:10. `master` at `5c02814`, clean. Git identity `Jacek
  Galanciak <jacek.galanciak@gmail.com>`. No tmux server running.
- `docs/pair-verdicts.txt`: SHA-256 `f4b1867956ba...`
  (`work/s40/listening-digests-start.txt`), 64 verdict lines (24 `same`,
  40 `different`). The NAS clusters file: SHA-256 `d9c85c4586ad...`
  (`work/s40/clusters-digest-start.txt`), 284 clusters, 2,903 pairs
  (1,685 joined, 30 of them by a `same` verdict; 1,218 apart, 37 of them
  by a `different` verdict).
- Experiments 0001-0185; this session numbers from 0186. Test set: 4 of
  5 evaluations used (none in this session).
- The start build `work/bin/s40-start/` is byte-identical to
  `s39-final` (cargo had nothing to rebuild).
- `pair` searches its pairs one after another, and for a corpus query
  also searches the decoded audio on both ladders. It reports each
  alignment's supported share at the fitted speed, not which seconds
  hold the hits, so the near-offset share (a union over alignments) and
  the longest stretch without a near hit cannot be computed exactly from
  today's report.
- No Python unit tests run in `scripts/check.sh` today.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build and both regressions (tmux `s40start`) | 02:10-02:30 |
| 1 | the pre-registration in this brief (populations, measures, scores, criteria); commit | 02:30-02:50 |
| 2 | `pair` reports each alignment's supported seconds and runs its pairs in parallel; `pair_offsets.py` with unit tests; `listening_pack.py select --min-hits`; check.sh runs the Python tests; both regressions identical | 02:50-03:30 |
| 3 | the populations' pair files and `pair` runs (tmux) | 03:30-04:00 |
| 4 | experiments 0186-0188 | 04:00-04:30 |
| 5 | false confident detections at NAS scale; the listening pack | 04:30-04:50 |
| 6-7 | review, notes, final checks | 04:50-05:20 |

## Decisions

- **(02:10) Start build and regressions** (`work/s40/start.sh`, tmux
  `s40start`), while the brief is written.
- **(02:17) Start checks.** `regress session-32-linux` (1:10) and
  `regress session-33-linux-nas` (6:15) identical with
  `work/bin/s40-start/`; every report identical to its baseline's apart
  from `wall_seconds` (`work/s40/same.py`).
- **(02:15) The source pairs measured** (allowed before the
  pre-registration): `pair work/s40/pairs/s40-source.txt` with the run
  build (`work/bin/s40-run/`, the `pair` change below), 45 s for the two
  pairs. Its alignments at the fitted speed reproduce both sheets.
- **(02:20) Step 1, the pre-registration** (committed before `pair` runs
  on any population pair; only the two source pairs have been measured).

  *The source pairs.* China Cup (`extra/b-bad_company-china_cup.mp3`)
  against ƆƐI3C's upload (judged different) and Dilusion
  (`extra/rawkuts-gridlok-rkt337-2001-sour/a-gridlok-dilusion-sour.mp3`)
  against dfect-dnb's upload (judged same) are where the hypothesis
  came from. They are reported apart: every score below leaves them out,
  and one comparison repeats the judged pairs' scores with them in.

  *The populations* (`scripts/analysis/pair_offsets.py populations`,
  from the NAS clusters file, `docs/pair-verdicts.txt`, the `.tags`
  beside every record of both stores, the library map and both shared
  panels; pair files `work/s40/populations/s40-{a,b,c,d}.txt`, labels in
  `populations.json`). Every population leaves out pairs with a file in
  one of the 33 folders experiment 0065 lists as mixed-CD candidates
  (joined boundaries 0.75 or more; decision: nothing runs on them, and
  one population (d) pair had such a file).
  - (a) *judged*: every verdict line whose two files have records, in
    the line's order (first file the query): 62 of 64, 23 `same` and 39
    `different` (22 and 38 without the source pairs; written as 21 at
    first, corrected at 02:35 before any score was read against it). Two have none:
    `second-library/1.mp3` (left out of the NAS library by
    `.gunfingerignore` after session 22) and the "Synthesis (Remix)"
    track of Underfire UDFRCD003, Dangerous Drums 2, a mixed CD the
    `.gunfingerignore` lists (experiments 0065, 0066).
  - (b) *presumed same*: 100 joins drawn with seed 4001 from the 1,177
    clusters pairs joined at 80% coverage or more without a verdict,
    neither file named in any verdict, identical copies (library map)
    left out. Labelled "presumed same", never "judged". They were
    chosen by coverage, so coverage cannot misplace them; they test the
    other measures.
  - (c) *panel different*: for each family of
    `docs/panels/shared-material-seed-2026.json` and
    `shared-material-2-seed-2026.json`, every two of its recordings, at
    most 4 file pairs (seed 4002) among their files' cross pairs that no
    verdict names (those are in (a)): 51 pairs in 16 families (The
    Nine 12, Phantom Force 6, Fire 5, Clockwork 4, Fractles 4, Stab 2
    Def 3, Star Trails 3, two each for Night Gasp, Playground Shades,
    Pyro, T-Minus and Watermelon, one each for Dozer, Mind Overload,
    Tsunami and We R The Ones). Labelled different by the panels'
    construction.
  - (d) *presumed different*: among the clusters pairs not joined and
    without a verdict, whose two clusters no verdict touches (a verdict
    naming a file of each), not in (c), whose two files both have artist
    and title tags and differ in both the compared artist and the
    compared title (letters and digits in lower case, as session 39's
    `tracklist_tags.py`): the strongest pair of each two clusters, the 60
    with the most hits (of 702). Presumed different; a mistagged copy
    would be label noise.
  In all, 273 pairs; 82 of them have a corpus query, which `pair` also
  searches by decoded audio.

  *The measures*, all from one `pair` report per pair. F is the search
  "peaks, at the fitted speed" (its alignments strongest first), L the
  search "peaks, turntable ladder"; S the shorter file's length in
  seconds; an alignment's span in the shorter file is its query span
  when the query is the shorter file, else its found span; the main
  alignment M is F's first; an alignment's offset is `pair`'s
  `offset_seconds` (where the query's start falls in the found file).
  `pair` now also reports, for each alignment of F, the seconds of the
  shorter file counted in `supported` (`supported_seconds`).
  1. `coverage`: the largest span in the shorter file of an alignment of
     L or F, over S (as the clusters measure a candidate and its refit).
  2. `supported`: the sum of F's `supported` (today's supported time, as
     the sheets print it).
  3. `hits_per_second`: M's hits over M's query span in seconds (as
     `Pair::hits_per_second`).
  4. `near_T` for T = 2, 5, 10, 20 s: the number of distinct seconds in
     the union of `supported_seconds` of F's alignments whose offset is
     within T s of M's, over S. T = 10 is the hypothesis's; the other
     three are reported beside it.
  5. `longest_gap`: the longest run of whole seconds 0 to ceil(S)-1 of
     the shorter file, its ends included, holding no second of
     `near_10`'s union, in seconds.
  6. `far_jumps`: F's alignments ordered by the start of their span in
     the shorter file; the number of consecutive two whose offsets
     differ by more than 10 s.
  Higher means the same recording for 1-4, lower for 5-6. A pair whose F
  is empty has 0 for 2-4, S for 5 and the most possible far jumps (it is
  placed beyond every other pair) for 6.

  *The scores* (`pair_offsets.py scores`), for each measure:
  - the AUC (the share of same/different pairs ordered correctly, ties
    half) for: judged same against judged different (the primary
    comparison); the same with the source pairs; (b) against judged
    different; (b) against (c); (b) against (d); (a) same and (b)
    against (a) different, (c) and (d);
  - on the judged pairs: the best threshold's errors (candidates the
    midpoints between consecutive distinct values and one beyond each
    end; ties broken by the lower median candidate), the errors with
    leave-one-out cross-validation (the threshold refitted without each
    pair, then applied to it), and the judged pairs it gets wrong, by
    name, both ways;
  - at the threshold fitted on all judged pairs: the (b) joins on the
    different side and the (c) and (d) pairs on the same side, by
    name, and the side each source pair falls on.

  *Worth a roadmap entry*: a measure of 3-6 whose leave-one-out errors
  on the judged pairs are fewer than both coverage's and supported
  time's, and whose threshold fitted on all judged pairs places no (b)
  join on the different side. `near_10`, `longest_gap` and `far_jumps`
  are the registered candidates; a `near_T` with another T that meets
  the bar is reported as found after the run, not as confirmed. The
  (c) and (d) pairs placed on the same side are reported whatever their
  number, and the source pairs' sides are reported, not counted.
- **(02:20) Step 2, the harness and scripts.** `pair` reports, for each
  alignment at the fitted speed, the seconds of the shorter file it
  supports (`supported_seconds`, beside `supported`, which is unchanged)
  and searches its pairs one per worker thread (`--jobs`, output in the
  listed order); a unit test (`pair::tests`) checks that a stretch the
  query replaces holds no supported second. `pair_offsets.py` computes
  the measures and scores; `listening_pack.py select --min-hits`
  (default 30) leaves out borderline pairs on fewer hits. Both scripts
  have unit tests on fixtures (`scripts/analysis/test_*.py`, Python's
  `unittest`), which `scripts/check.sh` now runs when Python 3 is
  installed (skipped with a line otherwise).
- **(02:22) Step 3, the runs** (`work/s40/pairs.sh`, tmux `s40pairs`):
  `pair` on populations a-d with `work/bin/s40-run`, 31-50 s each
  (32 jobs), reports `work/reports/library-nas-dnb-peaks/pairs-s40-{a,b,c,d}.json`
  (own stems; `pairs-pairs.json` untouched).
- **(02:25) Experiment 0186** (pre-registered scores). `near_10` meets
  the registered bar by one pair (leave-one-out 6 against 9 and 7, no
  (b) join below its threshold); `longest_gap`, `far_jumps` and
  `hits_per_second` do not.
- **(02:27) Experiment 0187** (exploratory, chosen after 0186): the
  pairs the measures get wrong; within 14 hard judged pairs every
  measure is at chance. An exploratory variant, the near-offset share
  over the spans of every search's alignments (ladders and audio, which
  follow drift; `work/s40/explore/any_search.py`), does worse (AUC 0.94,
  9 leave-one-out errors, two (b) joins below its threshold).
- **(02:28) Step 5a.** The NAS-scale reports hold 7 false confident
  detections, all in `sweep-seed-2026.json` (calibrate's "false
  accepted" 7), all one pair: Aphrodite's Cool Rock Stuff (APH027 B1, a
  held-out query) found as the Mobb Deep / Urban Takover upload (judged
  different on 2026-10-22). The 11 development scans at NAS scale
  (`work/reports/library-nas-dnb-peaks/scan-*.json`, the full scans and
  their leave-outs) hold no wrong confident detection. No other NAS-scale
  sweep report exists; no new sweep runs (decision: the saved reports
  answer the question, and another seed would add a run outside the
  plan). `pair` on the 26 pairs behind the sweep's false candidates of
  30 hits or more (`work/s40/pairs/s40-false.txt`, tmux `s40false`).
- **(02:30) `check.sh` failed one test**,
  `decode::watch::tests::a_probe_that_writes_nothing_is_an_error`
  (`Ok(None)` where `Stalled` was expected), while the 5a `pair` run
  used all 32 threads beside it; sessions 23 and 36 saw this test fail
  under load and pass on a rerun, and the code it tests did not change.
  Every other test, fmt and clippy passed. Rerun queued after the sweeps
  (`work/s40/check2.sh`, tmux `s40check2`).
- **(02:31) Two NAS-scale sweeps added to step 5a** (decision: seed
  2026, the only saved NAS sweep, has no China Cup query, so it cannot
  check the roadmap's claim): seeds 2027 (China Cup indexed) and 2028
  (China Cup held out), `--panels work/s40/panels` (a copy of
  `work/s33/panels`; both panels equal `docs/panels/`'s), after the
  step 2 regressions (`work/s40/sweeps.sh`, tmux `s40sweeps`). Their
  reports go to `work/reports/library-nas-dnb-peaks/sweep-seed-{2027,2028}.json`;
  `regress` reads only its named reports.
- **(02:32) Step 5b's selection, fixed before its pairs are measured.**
  `listening_pack.py select` gains `--disagreements N --measures FILE`
  (tested): the N joins without a verdict that hold the least of the
  shorter file near the main offset (`near_10`, 0186's best measure) and
  the N pairs between clusters no verdict touches, with 30 hits or more,
  that hold the most, one per pair of clusters. The candidates: every
  join without a verdict (identical copies left out) and every pair kept
  apart at 20% coverage or more between unjudged clusters, less the
  populations' pairs, mixed-CD candidate folders and files without
  records left out: 1,837 pairs (`work/s40/pack_pairs.py`,
  `work/s40/pairs/s40-pack.txt`), measured with `pair` beside the
  sweeps (`work/s40/packpairs.sh`, tmux `s40packpairs`). The pack: N = 6,
  `--controls 2 --seed 2026 --map` the library map, the measures of the
  populations and of these pairs, rendered into `work/s40/pack/`. The
  `check.sh` rerun waits for both runs (tmux `s40check2`, restarted).
- **(02:37) Step 2's regressions** with `work/bin/s40-run`: `regress
  session-32-linux` (1:09) and `regress session-33-linux-nas` (5:56)
  identical, every report identical to its baseline's apart from
  `wall_seconds`; its CLI is byte-identical to the start build's.
- **(02:54) The sweeps** (8:34 and 8:19 beside the pack's `pair` run):
  every speed 60 of 60 at both seeds. Wrong confident: seed 2027 11, all
  the two corpus copies `Bad Company - China Cup.mp3` and `extra/02. Bad
  Company - China Cup.mp3` (indexed) found as ƆƐI3C's upload; seed 2028
  7, all `extra/b-bad_company-china_cup.mp3` (held out) found as the
  same upload. With seed 2026's 7 (Cool Rock), 25 false confident
  detections over three NAS sweeps, 18 of them China Cup ~ ƆƐI3C. The
  58 pairs behind seeds 2027 and 2028's false candidates of 30 hits or
  more (47 not measured yet) go to `pair`
  (`work/s40/pairs/s40-false-2.txt`, tmux `s40false2`); the `check.sh`
  rerun waits for it too.
- **(02:58) Experiment 0188** (step 5a): 18 of the 25 false confident
  detections over the three NAS sweeps are China Cup's three corpus
  rips found as ƆƐI3C's upload, the only false candidate pairs (of 84)
  above `near_10`'s threshold; the other 7 are Cool Rock (0.37).
- **(03:01) Step 5b, the pack** (`work/s40/pack/`, 14 items: 12
  disagreements, 2 control joins; README, sheets with a `near_10` line,
  clips rendered by FFmpeg from the corpus and NAS files in 2:01). The
  pack's `pair` run took 22:51 (1,837 pairs). Over every measured join
  without a verdict (1,298) `near_10` is 0.63 at the lowest (5th
  percentile 0.95, 14 below 0.8), and over the 639 apart pairs 0.30 at
  the highest: none crosses 0186's threshold (0.552) either way, so the
  pack holds the pairs nearest it, for the user to judge later; no
  agent listens. The joins: Pyro - Time Is Broken (two rips, 6:00-6:42
  apart), Alien Girl's PRO014 rip ~ an upload, Known Unknown - Drumtrip
  ~ its "Rollers Edit", Fallout (Rob F & Impulse upload ~ the Sinthetix
  rip), The Stand's "HD Audio & Speed Up" upload, Phoenix's upload ~
  the corpus rip; apart: Konflict - Pendulum ~ the Konflict remix of
  Paragliders, Drumtrip ~ DJ Dazee & Substance - Propaganda, Bacteria ~
  Space Cake, two Tower Bass files, We R The Ones VIP ~ the Klute
  remix, Wikki Wikki Plate ~ Defjammer.
- **(03:04) `check.sh` green on the rerun** (3:47, on a quiet machine):
  fmt, clippy, every Rust test (275 in the harness, the new `pair` test
  among them), the page's 5 Node tests and the 14 Python tests.
- **(03:05-03:13) The read-only review** (a separate agent; it changed
  nothing). Its findings, and what was done (03:15-03:30):
  - The clusters file lists 313 of its 2,590 pairs in both directions,
    so several counts were list entries: (b) holds 98 distinct joins
    (Klute's Silent Weapons remix rips and Slip Thru drawn both ways,
    scored twice; scored once, the (b) AUCs move by 0.001 at most and
    nothing else changes, `work/s40/scores-distinct.json`); the pack's
    1,837 entries are 1,595 pairs; its joins without a verdict 1,055
    (lowest 0.63, 9 below 0.8, not 1,298 and 14) and its apart pairs 638
    (not 639); 0188's 84 rows are 73 distinct pairs. Corrected in the
    experiments, notes, status and roadmap; the figures earlier in this
    brief stand as first written. `populations` now draws each join
    once (a later draw differs from this session's).
  - The first pack held the Phoenix join whose TECH012 file is an
    identical copy of `extra/[AA]_-_Phoenix.mp3`, judged `same` against
    the same upload. `--disagreements` now counts identical copies of
    judged files as judged (tested); the pack was selected and rendered
    again (Noisia, Phace & Stu - Dead Air, 0.81, takes Phoenix's place;
    the second control join changed with the set it is drawn from).
    Kept: We R The Ones VIP ~ the CERT1853 Klute remix, whose verdict is
    on another file of the remix (no copy, no cluster).
  - 0187's last sentence contradicted 0186 (China Cup falls on the same
    side); rewritten. "At chance" on the 14 hard pairs now says they
    were chosen from the measures' errors, so near chance by that choice,
    and gives coverage's 0.06, in 0187, the notes, status and roadmap.
  - The population runs took 0:31-0:52 and 0.6-1.1 GiB (not 31-50 s and
    0.6-0.8 GiB). Inferences now say so (why drifting uploads fail; 0188's
    "near-identical version" now "placed with the same recordings").
  - The 02:35 correction above: the populations file and scores always
    held 22 judged `same` pairs; only this brief's text said 21.
  - Minor, recorded rather than changed: with no fitted alignment
    `longest_gap` is ceil(S), not S as registered (one judged pair, the
    Night Gasp ~ Dark Soldier VIP pair, has none); `near_T` can exceed 1
    slightly (1.002 at most) because whole seconds are counted over S;
    (d) leaves out every cross pair of the panels' recordings, not only
    the pairs drawn into (c). Removed: `finite()`, a no-op (scores
    byte-identical without it). The tie test now has a tie.
- **(03:16) The final NAS regression changed calibrate** (false accepted
  7 -> 25, false candidates 95 -> 318) while the sweep and the scans were
  identical: calibrate reads every `sweep-seed-*.json` in the reports
  directory, and the two new NAS sweeps sat there. They were moved to
  `work/s40/sweeps-nas/` and the regression run again (03:16-03:22,
  5:58): identical, calibrate included. The 25 confirm 0188's count.
