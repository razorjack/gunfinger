# Session 74 brief: which tracks use this break? A passage query

Written on 2026-11-02, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file (its pre-registration above all), and
   `docs/session-74-checklist.md` (progress).
3. `docs/brief-69.md`; ADR 0014 and 0017; the roadmap's "The filter and
   famous breaks"; `docs/status.md`, "2026-10-31: the listening of the
   2026-10-31 packs"; `docs/session-notes.md` ("Session 50", "Session
   58", "Session 69", "Session 73"); experiments 0163, 0222-0225, 0241,
   0267-0268 and this session's (0276 and up).
4. The code: `crates/gunfinger-eval/src/break_search.rs` (this
   session's), `holders.rs`, `shared_mixes.rs` (`lookup`, `listed`),
   `parts.rs`, `pair.rs`; `crates/gunfinger-core/src/index.rs`
   (`SetAside`, `skipping_fullest`).

`docs/brief-2.md` to `docs/brief-73.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 74 of Gunfinger: a pilot of a new question, "which tracks use this break?". Listening to session 58's `parts` candidates (answered 2026-10-31; docs/status.md, "2026-10-31: the listening of the 2026-10-31 packs"), the user heard that where several different titles line up with a file's edges (other than a remix of the same track), they share a classic jungle break (Amen, Think, Hot Pants), sometimes as differently sampled variants: Special Branch (UFO, UFOLP001) is built on the Amen and variants of it and lines up with other tracks using them. The user asked whether Gunfinger could answer "I like this break and have heard it before: which tracks use it?". It is a different question from `identify`'s (which recording plays here), and it meets a built-in obstacle: the search sets aside the fullest 1% of the index's posting lists (ADR 0014), and famous breaks' hashes are expected there (the roadmap's "The filter and famous breaks"). Session 69's passage matcher (`gunfinger-eval passages`, pairwise links with their spans, no merging through chains) and `pair` (every alignment between two files) are the nearest tools. This session measures whether a passage query (a file and a span of about 10-20 s) can list the library files holding that passage, strongest first, and how much unrelated material comes with it. It is a measurement session: the product does not change; a new command would need its own ADR.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).

Time box: started about 07:45; aim to finish within about 2.5 hours, and by 10:40 at the latest (the user's soft deadline is 11:00). If a step would run past that, scale it down (262 tracks and the judged cases first), record the decision in the brief and continue.

Environment (sessions 33-73's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,674 records, revision `7210fc479da58376` (session 64 pruned four ignored uploads). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-64-linux-nas` (NAS scale; compare with it, not `session-54-linux-nas`).
- Build `work/bin/s73-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Since `45f92d5` (2026-10-28) its calibrate reads only the reports its baseline holds; still run variants and other seeds in a scratch work directory, never in the reports directory `regress` compares.
- The user restructured the code on 2026-10-28 without meaning to change behaviour (commits `b5a84e3` to `f94604a`: harness subcommands in modules, named results, `Search` passed through the core's search, enums for closed sets). Session 60 confirmed both regressions identical on a build of master after it.
- Since sessions 53-56: `SpansAtSpeed(0.006)` is the default second suppression (ADR 0021); same-title detections (`docs/same-titles.txt`) are scored right (ADR 0022); `identify` prints gaps as "ID - ID" lines with an estimate of their tracks (ADR 0023, 0024). The minimum span stays undecided. A change to an existing decision needs a new ADR (the next number is in `docs/adr/`).
- The research note `docs/research/2026-10-29-matching-and-archive-proposals.md` (2026-10-29) records six unmeasured proposals and its evaluation boundaries; any session working on one follows its "Evaluation and adoption boundaries".
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen. Every listening question asked so far has an answer (docs/status.md, the entries of 2026-10-31 and 2026-11-02), apart from the Planet Dust (DC Breaks Remix) cut (unsure). The user's decisions on a quiet track under an identified one are in docs/status.md, "2026-11-02: the user's answers on a quiet track under an identified one". When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.
- The repository root holds an untracked `yafc20261031.log` that is not this project's work: leave it alone, never commit or delete it; "`git status` clean" means clean apart from it.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-69.md and the newest brief, docs/session-notes.md ("Session 50", "Session 58", "Session 69" and the newest session), docs/status.md (the newest entries), ADR 0014 and 0017, the roadmap's "The filter and famous breaks", experiments 0163, 0222-0225, 0241 and 0267-0268, the shared-material panels in `docs/panels/` and `docs/pair-verdicts.txt` (the judged shared material: Wikki Wikki Plate ~ Defjammer, Pyro ~ Watermelon and others), and the code of the first pass's set-aside lists, `pair`, `passages` and `parts`.
2. Write docs/brief-74.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-74-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s74/listening-digests-start.txt. Experiments number from 0276.
3. Build `work/bin/s74-start`; `regress session-54-linux` and `regress session-64-linux-nas` identical.

Steps:

1. Pre-registration, committed before the scored runs: the query (a file and a span; the passage's peaks searched against the whole index including the set-aside lists, on the ladder, with a relative-speed range stated); the output (files with an alignment to the passage, ranked by a stated measure such as supported hits per second of the passage, each with its span in the found file and its speed; a file's own duplicate cluster listed apart); the cases (positives: the judged shared material in the verdicts and panels, the user's Special Branch case at NAS scale, and passages of the parts candidates whose edge parts the user heard as a shared break; negatives: passages without a break, such as pads or vocals, and random passages); the scores (shared-material files found per query, unrelated files listed above them, by rank; whether the set-aside lists are needed, measured with and without them; the share of the index a famous break matches); the cost per query; and what would make it worth a design note and an ADR for a command.
2. The prototype, tested, harness only (an opt-in subcommand; `regress` identical): unit tests for the ranking and for including the set-aside lists.
3. The runs at 262 tracks and NAS scale (never write any store), under tmux with logs where long.
4. Experiments (at most 40 lines each, written before the next starts): retrieval of shared material and its noise; the set-aside lists and the cost.
5. A short research note (`docs/research/2026-11-02-break-search.md`): what a passage query finds, how famous breaks behave, the cost, what a command would print, and a recommendation that is ours to take; a pointer from the roadmap.
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 74" section at the end of docs/session-notes.md with a handoff and any questions for listening (for example a short list of tracks the query says share a break, for the user to hear); update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-54-linux` and `regress session-64-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: what the query finds, the noise, the set-aside lists, the cost, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's behaviour does not change: no threshold, parameter, rule, default or output change; `identify --under` stays off by default. Variants are opt-in, tested, and keep `regress` identical when off. Harness additions are tested and keep `regress` identical.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
- Scratch data goes under work/s74/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00 on 2026-11-02. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 74

- 2026-11-02, 07:40 by the Linux machine's clock (CET). Aim to finish by
  about 10:15; no new run after 10:20 in any case.
- `master` at `8c404d1`, clean apart from the untracked
  `yafc20261031.log`. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s74/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `c4d3da2e7e37...`, `work/audition/answers.jsonl` `3e99ebca589b...`.
- Experiments 0001-0275; this session numbers from 0276. ADRs
  0001-0025 (0025 Proposed). Test set: 4 of 5 evaluations used (none in
  this session).
- The NAS store: 40,674 records, revision `7210fc479da58376`. Build
  `work/bin/s73-final/` (master). Baselines `session-54-linux` and
  `session-64-linux-nas`.
- What exists: `holders` (session 23) searches a file's stored peaks
  over a span (`shared_mixes::lookup`: `search_peaks`, the product's
  ladder, one pass) against the harness's index with the matcher's
  lists set aside, and lists every file of 30 hits or more at its
  strongest detection. `parts` (session 58) searches files' ends the
  same way and measures each part again against the found file alone.
  `pair` lists every alignment between two files. `passages` (session
  69) links passages between mixes pairwise. None searches the lists
  the first pass sets aside (the fullest 1% by length, 18.7% of the
  postings at NAS scale; ADR 0014), and none ranks a passage's holders
  for this question.
- Session 58's `parts` report of Special Branch (UFO, UFOLP001) lists
  about 60 other titles lining up with it between 173 s and 346 s, most
  at 1.01-1.08 relative speed, 25-120 hits each: the user's Amen case.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build and both regressions (tmux `s74start`) | 07:40-08:05 |
| 1 | pre-registration (below) and the cases file, committed before any scored run | 08:05-08:15 |
| 2 | the prototype `break-search` (harness only, opt-in), tests for the ranking and the set-aside lists; `regress` identical | 08:15-09:00 |
| 3 | the runs at 262 tracks and NAS scale (tmux) | 09:00-09:20 |
| 4 | experiments 0276 (retrieval and noise), 0277 (set-aside lists and cost) | 09:20-09:40 |
| 5 | research note `docs/research/2026-11-02-break-search.md`, roadmap pointer | 09:40-09:55 |
| 6-7 | review, notes, status, checklist, final checks | 09:55-10:35 |

## Pre-registration (step 1; written 07:55-08:10, before any scored run)

**Question.** Given a file and a span of 20 s (a passage the listener
likes, such as a break), can a search of that passage list the library
files that hold it, strongest first, with little unrelated material
above them, at 262 tracks and at NAS scale; do the lists the first pass
sets aside (ADR 0014) have to be searched for it; and what does a query
cost?

**The query** (`gunfinger-eval break-search`, `break_search.rs`). A
query names a file of the harness's index (a corpus file, or
`second-library/<path>` with `--other-peaks-dir`) and a span; its
passage is the file's stored peaks within the span (as `holders` cuts
it, `shared_mixes::lookup`'s passage). The passage is searched against
the whole index the harness builds (the 262 corpus records, or with
`--other-peaks-dir` the saved NAS-scale index; no file left out, the
query's own file included) with the stored-peak search
(`search_peaks`: peaks rescaled to each rung, one pass, any chain), on
the product's turntable ladder, 0.92-1.08 in 41 rungs: a relative speed
range of ±8% between the query file and a found file. Key-locked rungs
are not searched (a break sampled at another pitch is a resampling).
Every query is searched twice, in one run, on one index:

- *all lists*: every posting list scanned (nothing set aside);
- *filtered*: the fullest 1% of lists set aside as the product's first
  pass sets them aside (`Index::skipping_fullest(SKIPPED_SHARE)`).

Each query runs on one thread; queries run in parallel over the default
jobs (32).

**The output.** Per query and mode, every file with a detection of at
least 20 hits (the *floor*), once, at its strongest detection: the
file, hits, windows, hits per second of the passage (hits / 20 s, the
*ranking measure*; ties by path), the span in the found file, the
relative speed, and how many detections of 20 hits or more the file
has (a looped break can line up at several offsets). The query's own
file and the files of its duplicate cluster (the harness's clusters
file: the corpus clusters at 262 tracks, the NAS clusters at NAS scale)
are listed apart (*own*), never ranked. For the 20 strongest other
files, the *reach*: the whole query file's peaks searched against that
file alone, every list, on the rung of its strongest detection; the
strongest alignment overlapping the passage, its span in the query
file and hits. Per query and mode also: the passage's distinct hashes at
speed 1.0 and how many of them lie in lists the filtered mode sets
aside; the files listed at 20, 30 and 60 hits or more over the index's
files; and the query's thread seconds.

**The cases** (written to `docs/experiments/data/0276-cases.json`
before any scored run, by `work/s74/cases.py`; each case: name, group,
query file, span, *expected* files and, where known by audio, other
rips of the query's recording outside the clusters file, *own extra*):

- *A, the panels' shared material*: every direction of
  `docs/panels/shared-material-seed-2026.json` whose passage is not a
  control (Clockwork, Dozer, Fire, Mind Overload, Pyro, Sonar, T-Minus,
  The Nine, Watermelon; 20 directions with Fire's first four). The span
  is 20 s centred on the middle of the panel's passage in the played
  file (the passage itself where shorter than 20 s), clamped to the
  file. Expected: the related recording's files as the panel lists
  them. The control directions (Fractles, Night Gasp, Playground
  Shades: under 25 hits) are left out.
- *B, the drum-loop verdicts*: the seven pairs of experiment 0225
  (Wikki Wikki Plate ~ Defjammer, the two Tower Bass pairs, Stalker,
  Dark Dark Side ~ Style From The Dark Side '99, the Klute VIP against
  two rips of the Klute Remix). Their passage is the strongest
  alignment by hits of `gunfinger-eval pair` between the two files
  (NAS scale), the span 20 s centred on its middle in the first file.
  Expected: the second file.
- *C, the user's Special Branch case* (NAS scale only): the UFOLP001
  rip, six spans: 0-20, 30-50, 65-85, 190-210, 250-270 and 305-325 s.
  Expected: the other-title files of session 58's `parts` report whose
  part overlaps the span by 10 s or more in Special Branch. Own extra:
  the parts report's whole-length alignment (`U.F.O - Special
  Branch.m4a`, 13,346 hits over 0-348 s at speed 1.000).
- *D, the parts candidates heard as one recording* (NAS scale only):
  candidates answered `one` in the 2026-10-31 listening whose edge part
  is another title, not another version of the same tune (chosen by
  reading the titles, a list written into the cases file; the user's
  reading is that such parts share a classic break): one query per
  candidate recording (one rip each), the span 20 s centred on its
  strongest other-title edge part, clamped to the file. Expected: the
  found files of its other-title edge parts. These cases were found by
  the filtered stored-peak search (`parts`), so they favour the
  filtered mode.
- *N1, random passages*: 40 at each scale, seed 7401: a file drawn
  uniformly from the index's files of at least 60 s, outside `sety/`
  and the folders of experiment 0065's candidate lists, and a 20 s span
  starting uniformly in [10 s, length - 30 s]. Expected: none.
- *N2, intros*: 40 at each scale, seed 7402, drawn as N1, the span
  0.5-20.5 s: intros are often beatless (pads, vocals), but these are
  not checked by ear. Expected: none.

At 262 tracks the cases are A and B whose query file and at least one
expected file are corpus files, and N1 and N2 drawn from the corpus
records. At NAS scale, every case.

**The scores** (`scripts/analysis/break_search.py`, per scale and
mode):

- Positives (A-D): *found*, an expected file listed (20 hits or more);
  *rank*, 1 + the number of other files (not own, own extra or
  expected) listed above the strongest expected file; *expected
  listed*, of the expected files.
- Noise: for N1 and N2, the other files listed at 20, 30 and 60 hits,
  and the strongest other file's hits per second; for positives, the
  other files listed above the best expected file.
- The null: *T*, the 95th percentile over N1's queries of the strongest
  other file's hits; for positives, the share whose best expected file
  reaches *T*.
- The set-aside lists: found and rank in both modes; the share of each
  passage's hashes set aside (median and range per group); a positive
  found only with all lists.
- Famous breaks: the share of the index's files listed per query (at
  20 and 60 hits), its largest values and the cases they come from.
- Reach: for the other files listed, how many reach 60 s or more of
  the query file (a version or another rip, more than a passage),
  reported, deciding nothing.
- Cost: thread seconds per query (median and maximum) per mode and
  scale; the run's index load and peak memory.

**What would make it worth a design note and an ADR for a command.**
At NAS scale, in the better of the two modes:

1. an expected file is found for at least 80% of the positives (A-D
   pooled), ranked first for at least 50% and within the first 5 for at
   least 70%;
2. on N1, the median query lists at most 5 other files at 20 hits or
   more, and *T* lies below the median hits of the found expected
   files;
3. the median query costs at most 60 thread seconds (about 2 s on 32
   threads) in that mode.

If all three hold, a design note for a command (`gunfinger` reading a
file and a span, printing holders) is worth writing, and an ADR follows
only its decision. If (1) holds only with all lists, a command would
search every list for the passage. If (1) fails, the note says where
the query fails (which groups, which ranks) and no design is proposed.
The decision is ours; nothing in the product changes in this session.

## Decisions

- **(07:44) Start build and regressions** (`work/s74/start.sh`, tmux
  `s74start`, logs in `work/s74/logs/`; `run.sh` and `same.py` are
  session 73's with the paths changed). The start build is
  byte-identical to `s73-final`.
- **(07:49) Both regressions identical** with the start build (0:51;
  3:57), every report equal to its baseline's apart from
  `wall_seconds`.
- **(07:48) `pair` for group B** (`work/s74/s74-drum-loops.txt`, the
  seven pairs of 0225's NAS data, `s73-final`; 0:21): case data, not a
  scored run.
- **(07:53) The cases file** (`docs/experiments/data/0276-cases.json`,
  written by `work/s74/cases.py` from the panels, the `pair` report,
  session 58's parts reports and the answer log; query files
  `work/s74/queries-262.json` and `queries-nas.json`). Corrections to
  the registration's text before any scored run: group A's rule (every
  non-control direction, Fire's first four) gives 22 directions, not
  20, because The Nine has four; the 262-track run holds A and B cases
  whose query file is a corpus file and whose expected files include a
  corpus file (4 of A, 1 of B), with N1 and N2 drawn from the corpus
  records: 85 queries; NAS scale 147 (A 22, B 7, C 6, D 32, N1 40, N2
  40). Group D's selection is the 32 fragments in `cases.py` (one per
  recording; the Planet Dust, Blind, Reece and other same-tune
  candidates left out). Special Branch's spans at 0-20 and 30-50 s
  expect only Judgement Day; at 305-325 s, 32 files.
- **(07:53) The report lists a query's expected files the index does
  not hold** (`expected_not_indexed`; an ignored file, such as `OMR -
  Bad Vibes`, or a copy a corpus file stands for); the scoring counts a
  positive only against expected files the index holds, and a positive
  with none is reported apart.
- **(07:55) The queue** (`work/s74/queue.sh`, tmux `s74queue`): the
  run build `work/bin/s74-run/` from `716fbad` (its `gunfinger`
  byte-identical to `s73-final`'s), `break-search` at 262 tracks (1.2 s
  for 85 queries in both modes) and at NAS scale, then both
  regressions. `scripts/analysis/break_search.py` scores the reports
  (tested, `test_break_search.py`); counting other files at 20 and 60
  hits counts the other files only (own apart), as registered.
- **(07:58) A cost run alone** (`work/s74/cost.sh`, tmux `s74cost`,
  after the queue): ten NAS-scale queries (the six Special Branch spans,
  N1's first four) with `--jobs 1`, so that each query's thread seconds
  are measured without 31 others running; decided after the main run,
  reported in 0277 beside the registered figures. Its listings equal the
  main run's.
- **(08:00) Both regressions identical** with the run build (0:51;
  3:48), every report equal to its baseline's apart from
  `wall_seconds`.
- **(08:02) Results.** By the registered bar the query is not worth a
  design note and an ADR: (1) found 59 of 64 positives (92%) and 51 in
  the first five (80%), but first only 25 (39%, bar 50%); (2) the
  median random passage lists no other file, but *T* (726 hits) lies
  above the found expected files' median (72), because random passages
  list other rips of their own recording outside the NAS clusters; (3)
  holds (a median 8 thread seconds with every list). Exploratory
  readings (`work/s74/posthoc.py`, not registered): with files reaching
  60 s of the query set apart, 39 first and 53 in the first five.
  Experiments 0276 (`32b486e`) and 0277.
- **(08:05) The research note** `docs/research/2026-11-02-break-search.md`
  and the roadmap's "The filter and famous breaks" (a pointer).
- **(08:17) The review** (a read-only subagent, 08:04-08:17) confirmed
  that the product did not change, the regressions, the digests and
  most figures (recomputed), and found:
  (1) group D's expected files include every part of the candidate
  whose found file matches the titles read, not only edge parts as
  registered (13 files in 11 cases come only through other parts).
  Not rescored; with edge-only expected files D gives 3 first and 21 in
  the first five (24 and 48 pooled), which does not change the bar's
  outcome (first still fails; first five 75%, still above 70%);
  (2) "40-110 hits each" held only for the first 30 holders of the
  famous-break listings (most have 20-39): corrected in 0276, the note
  and the roadmap;
  (3) "the exceptions are breaks" was unheard: marked as an inference;
  (4) the reach is measured at the detection's fitted speed, not a rung
  of the ladder as registered, and 238 of 737 measured files (every
  list) have none, because the search keeps one alignment per moment;
  stated in 0276 and the code, and failures to load a record or build
  its index are now printed instead of passing silently;
  (5) "another rip" was read from reach and titles (one of the 13
  random passages' files carries another title): reworded;
  (6) registered figures not reported (reach counts, *T* reached by no
  expected file, the share at 60 hits, set-aside ranges): added to 0276
  and 0277;
  (7) the note did not say where the ranking fails: it now names group
  D (4 of 31 first). The note's "What a command would print" sketches a
  print-out, which the prompt asks for; the registration said no design
  is proposed when (1) fails. The note proposes no design: it lists
  what a command would need and asks the ear first;
  (8)-(9) `CLAUDE.md`'s "unless brief 74" and the checklist: fixed;
  (10)-(14) wording in 0277 and the note (set-aside shares "only
  slightly more", the filter hides "about half" for Special Branch and
  hits kept count only files still listed, 1.83-1.94 times with the
  cause marked an inference, "usually" removed, "about 2 minutes");
  (15) the N1 and N2 draws are from the stores' record lists
  (`nas-paths.tsv`, `corpus-paths.tsv`) of files of 60-1,200 s, also
  leaving out the folders of the Virus on Kiss FM show and the Lowdown
  Vol 6 track fix (ignored since 2026-10-31), not from the index's
  asset list; every drawn file was in the index;
  (16) `detections` is nearly always 1 (one detection per moment of the
  query), so whether a looped break lines up at several offsets was not
  measured; documented on the field;
  (17) house style: the passage reuses `shared_mixes::passage`, the
  helpers are private, `SpeedRatio` imported, load problems printed;
  `Records` stays beside `pair::Records` (different sources);
  (18) the tie test cannot fail on order alone (kept as a statement of
  intent); the two-mode loop of `run` has no unit test;
  (19) `AGENTS.md`'s module map names `break_search`; the query file's
  `expected` documented. `cases.py` and `posthoc.py` stay in
  `work/s74/` (session scripts).
  The code changes leave every listing and number unchanged: the final
  build runs the NAS-scale queries again for comparison.
