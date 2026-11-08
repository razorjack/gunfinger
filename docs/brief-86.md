# Session 86 brief: a library-wide clustering, and break search as retrieval

Written on 2026-11-08, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it), `AGENTS.md` and
   `docs/session-rules.md`.
2. This file, and `docs/session-86-checklist.md` (progress).
3. Experiments 0049, 0158, 0169, 0276-0277; the notes "Session 74" and
   "Session 85"; `docs/research/2026-11-02-break-search.md`; the
   roadmap's active queue item 10 and "The filter and famous breaks";
   `docs/decisions-queue.md` item 4; `docs/calibration.md`'s duplicate
   cluster row; ADR 0016 and `docs/audition-questions.md`.
4. The code: `crates/gunfinger-eval/src/clusters.rs` (`find_around`,
   `at_fitted_speed`, `pair`, `better_of`, `clusters_of`), the
   library-wide run added this session, `break_search.rs` and
   `commands/shared.rs`, and `audition/shared.rs` (a question producer).

## The prompt (verbatim)

> Start session 86 of Gunfinger: a library-wide duplicate clustering in a scratch file, and break search measured again as retrieval with many relevant results. Session 74 (experiments 0276-0277, `docs/research/2026-11-02-break-search.md`) found 59 of 64 expected holders of a passage, 51 in the first five, but the expected file first only 25 times, because a famous break lines up with 120-403 files; its registered null failed as a null because 13 of 40 random passages found another rip of their own recording, which the NAS clusters do not join: the NAS clusters file holds only the corpus recordings' rips (284 clusters). The external review of 2026-11-05 (`docs/research/2026-11-05-external-review.md`) adds that ranking one nominated file first is the wrong objective when many files are correct, and that random NAS-wide negatives are contaminated by unclustered rips everywhere (recall panels, break-search nulls, counterexample wrong answers). This session (1) clusters the whole NAS library into a scratch file, never replacing the NAS clusters file, and (2) measures break search again with every holder in a known duplicate cluster or judged relation counted as relevant, and prepares a listening pack for the holders nobody has judged. It is a measurement session: the product, the stores and the NAS clusters file do not change.
>
> You run unattended: the user cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.
>
> The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).
>
> Time box: started about 06:05 on 2026-11-08; aim to finish within about 2.5 hours. If a step would run past that, scale it down (the clustering and the null first; the retrieval scores on session 74's positives; the pack as a list of paths), record the decision in the brief and continue.
>
> Environment (sessions 33-85's handoffs in docs/session-notes.md; check each):
> - Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
> - Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
> - NAS store: 40,666 records, revision `0bd3f0c23fabc698` (session 75 pruned the records of DJ sets and a mixed CD's track fix). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-78-linux` (262 tracks) and `session-78-linux-nas` (NAS scale), both with the 2003 and Nocny scans since session 78; `regress --strict` fails on any difference; a session that adds baselines names them in its handoff, and the next prompt says which to use.
> - Build `work/bin/s85-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
> - `gunfinger-eval regress` prints what changed and returns success either way: read its output. Since `45f92d5` (2026-10-28) its calibrate reads only the reports its baseline holds; still run variants and other seeds in a scratch work directory, never in the reports directory `regress` compares.
> - The user restructured the code on 2026-10-28 without meaning to change behaviour (commits `b5a84e3` to `f94604a`: harness subcommands in modules, named results, `Search` passed through the core's search, enums for closed sets). Session 60 confirmed both regressions identical on a build of master after it.
> - Since sessions 53-56: `SpansAtSpeed(0.006)` is the default second suppression (ADR 0021); same-title detections (`docs/same-titles.txt`) are scored right (ADR 0022); `identify` prints gaps as "ID - ID" lines with an estimate of their tracks (ADR 0023, 0024). The minimum span stays undecided. A change to an existing decision needs a new ADR (the next number is in `docs/adr/`).
> - Since 2026-11-05: `docs/session-rules.md` holds the standing rules (the older briefs are history), `docs/decisions-queue.md` the decisions waiting for the user, and the roadmap begins with "The active queue" (`docs/research/2026-11-05-external-review.md` explains why). A session that leaves a decision for us adds it to the decisions queue. The six proposals of `docs/research/2026-10-29-matching-and-archive-proposals.md` have all been measured; a session working on one follows its "Evaluation and adoption boundaries".
> - `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
> - Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
> - Tonight (2026-11-07 to 11:00 on 2026-11-08) sessions run one after another, unattended; the user cannot decide or listen. Every listening question asked so far has an answer (docs/status.md, the entries of 2026-10-31 and 2026-11-02), apart from the Planet Dust (DC Breaks Remix) cut (unsure). The user's decisions on a quiet track under an identified one are in docs/status.md, "2026-11-02: the user's answers on a quiet track under an identified one". When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
> - The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.
> - The repository root holds an untracked `yafc20261031.log` that is not this project's work: leave it alone, never commit or delete it; "`git status` clean" means clean apart from it.
>
> First:
> 1. Read CLAUDE.md and follow its reading list, notes "Session 33", "Session 74" and the newest session, experiments 0049, 0158, 0169 and 0276-0277, the break-search note, ADR 0016 and `docs/audition-questions.md` (how a pack is built), `docs/calibration.md` (the duplicate-cluster row), and the code of `clusters` (its options for a library-wide run, `--from-peaks`, where it writes), `break-search` and the audition question builders.
> 2. Write docs/brief-86.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-86-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s86/listening-digests-start.txt. Experiments number from 0295.
> 3. Build `work/bin/s86-start`; the start regressions follow docs/session-rules.md.
>
> Steps:
>
> 1. Pre-registration, committed before the scored runs: the clustering (library-wide, from stored peaks, the calibration register's criterion unchanged, the verdicts by ear applied; written to `work/s86/`, never over the NAS clusters file; its time and memory recorded; what is compared with the existing clusters); the retrieval scores (precision at 5, 10 and 20 over judged holders, expected-holder recall, the share of returned passages that are unrelated by judgment or by the new clusters, unjudged candidates kept apart as unjudged; both playback ladders; new query families if the panels hold them); the null (random passages, scored with the new clusters joining rips); and what would make a command worth a design note.
> 2. Any harness change, tested, opt-in; `regress` identical.
> 3. The runs, under tmux with logs where long (the clustering first).
> 4. Experiments (at most 40 lines each, written before the next starts): the library-wide clustering (its clusters against the existing ones, the sparse joins to listen to); break search as retrieval and the null.
> 5. A listening pack for later (not answered by anyone tonight): sampled holders at the top, middle and bottom of three famous-break lists and the null's strongest finds, built with the audition tool's question format, with its command written in the handoff; nothing is written to the answer log.
> 6. Update the break-search note, the roadmap's items and docs/decisions-queue.md item 4.
> 7. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
> 8. Wrap-up: a "Session 86" section at the end of docs/session-notes.md with a handoff (where the scratch clusters are, how a later session may use them as negatives' labels, the pack's command); update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-78-linux` and `regress session-78-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the NAS clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the library-wide clusters, break search as retrieval, the null, the pack, and anything left.
>
> Rules for this session:
> - `docs/session-rules.md` binds as well; where it and this list differ, this list wins for this session.
> - No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
> - The product's behaviour does not change: no threshold, parameter, rule, default or output change; `identify --under` stays off by default (the default is ours, in ADR 0025). Harness changes and variants are tested, opt-in where they are variants, and keep `regress` identical.
> - Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
> - The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
> - The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters file (`work/reports/library-nas-dnb-peaks/duplicate-clusters.json`): this session's library-wide clustering reads the stored peaks and writes only under `work/s86/`; check the output path before the run starts, and the NAS clusters file's SHA-256 before and after. Never write to `work/peaks`.
> - Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
> - AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
> - Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
> - Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
> - Scratch data goes under work/s86/; delete renders you no longer need.
> - Prose: no em dashes (spaced en dashes in prose, ASCII in code).
> - Commit after each step that changes tracked files.
> - Time: the user's soft deadline is 11:00 on 2026-11-08. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.

## State at the start

- `master` at `5a46442`, `git status` clean (apart from the untracked
  `yafc20261031.log`). Builds `s85-final` (master) and `s84-final`.
  `work/bin/s86-start/`, built from `7909c1e`, is byte-identical to
  `s85-final` (`gunfinger` `cf8393e36bad...`, `gunfinger-eval`
  `c20a5f9cd80f...`; `work/s86/start-binaries.txt`), and the store
  revisions are unchanged (corpus `526a3b978eb3e411`, NAS
  `0bd3f0c23fabc698`, from a 10 s `identify` with each configuration),
  so by `docs/session-rules.md` the start regressions are skipped; the
  final ones run.
- Baselines `session-78-linux` and `session-78-linux-nas`; NAS store
  40,666 records, revision `0bd3f0c23fabc698`; NAS clusters
  `c4d3da2e7e37...`: 284 clusters, 1,004 files, 2,875 pairs, no cut
  links. Digests at the start: `work/s86/listening-digests-start.txt`
  (`docs/pair-verdicts.txt` `51dd1bbac501...`, 100 lines).
- `clusters --from-peaks` with `--other-peaks-dir` (`find_around`)
  searches only the corpus files, the files `--manifest` and
  `--recall-panel` name and, round after round, the rips it finds; each
  query is a whole file's stored peaks on the turntable ladder against
  the other library, the fullest 1% of lists set aside, about 40 thread
  seconds per query (0169: 23 minutes for the full run at 32 jobs). Its
  module comment says that clustering every file this way would take
  days: at that cost 40,696 queries would take about 14 hours on 32
  threads (an estimate). It writes the clusters file in use.
- `break-search` searches the turntable ladder only, every list and
  filtered; its own cluster comes from the clusters file in use.
- Session 74's group A (0276) took the directions of the whole frozen
  panel 1, so 10 of its 22 cases (clockwork, mind-overload, t-minus,
  the-nine) are of the validation half, which the standing rules keep
  unsearched. Panel 2's development half (10 directions: phantom-force,
  star-trails, we-r-the-ones) was not used by 0276.
- The answer log holds no `shared` answers: no judgment says that a
  file does not hold a passage. Judged relations are the cases'
  expected files (panels, drum-loop verdicts, Special Branch, `parts`
  answers), `docs/same-titles.txt` and the verdicts by ear.

## Plan

1. Registration (below) committed with this brief, before any scored
   run.
2. Harness, opt-in and tested:
   - `clusters --from-peaks --library-wide OUT`: every file of the
     index the harness builds with `--other-peaks-dir` (the corpus and
     the other library) is a query; candidates come from a 20 s excerpt
     of its stored peaks, and every candidate is measured with the
     clusters' criterion unchanged (below). It writes OUT only, refuses
     the clusters file in use, and never replaces it.
   - `break-search --ladder turntable|both` (default turntable, as
     today): with `both`, the key-locked ladder as well.
   - `questions breaks`: a question producer for the pack (a break-search
     report and a list of holders), if time allows; otherwise the pack
     is a list of paths (the prompt's fallback).
3. Runs: a timing sample of the clustering, then the full clustering
   under tmux; break search at NAS scale on the development cases, both
   ladders.
4. Experiments 0295 (the clustering) and 0296 (break search as
   retrieval, the null); the pack; documents; review; wrap-up.

## Registration (committed before any scored run)

### R1. The library-wide clustering (experiment 0295)

- **Files.** Every file of the NAS-scale index the harness builds: the
  262 corpus files and the NAS store's current records except the
  corpus files' copies (40,696 files in session 74's index).
- **Candidates.** Each file's middle 20 s of stored peaks (the whole
  file when shorter) searched with `search_peaks` on the turntable
  ladder (0.92-1.08, 41 rungs) against the whole index with the fullest
  1% of posting lists set aside, as `clusters` sets them aside. Each
  other file's strongest detection of 30 hits or more
  (`MIN_HITS_TO_MEASURE`) is a candidate; the 20 strongest per query
  are measured. The middle of the shorter file of a pair lies inside
  any alignment covering 80% of it (for files of 33 s or more), so a
  pair the criterion joins is a candidate when the shorter file's
  excerpt finds the longer one among its 20 strongest.
- **Criterion, unchanged.** The whole query's stored peaks against the
  found file alone at the candidate's fitted speed and playback
  (`at_fitted_speed`); when that covers 20-80% of the shorter file, the
  whole query also against the found file alone on the turntable
  ladder (the ladder's chains follow a drifting rip, 0049); the
  alignment covering more counts, in the shorter file's own seconds;
  same recording at 80% or more. The verdicts by ear
  (`docs/pair-verdicts.txt`) override it. A chain of joins between
  two files judged different is cut at its sparsest join and listed
  (`--cut-sparsest`; decided below). Pairs below 20% coverage are
  counted, not kept.
- **Where it writes.** `work/s86/clusters/library-wide.json`; the run
  checks that the path is not the clusters file in use; the NAS
  clusters file's SHA-256 is checked before and after.
- **Recorded.** Wall time, CPU time and peak memory (`/usr/bin/time
  -v`), queries, candidates measured.
- **Compared with the existing clusters** (284 clusters, 1,004 files):
  (a) the existing same-recording joins (pairs whose two files the
  existing file clusters) that the new clusters keep together, and the
  existing clusters kept whole; (b) new clusters that join two or more
  existing clusters; (c) the new clusters without a corpus file: their
  count, files and sizes, the largest listed; (d) sparse joins (under a
  tenth of the median hits per second) and borderline pairs (40-80%),
  counted and the strongest listed for listening; (e) cut links.
- **Usable as labels for negatives** (the handoff's condition): (1)
  95% or more of the existing same-recording joins kept together; (2)
  no new cluster joins two existing clusters, unless a verdict by ear
  joins them; (3) sparse joins under 1% of the joins. Otherwise the
  scratch clusters are reported with the failed condition, and later
  sessions use them only with that caveat.

### R2. Break search as retrieval, and the null (experiment 0296)

- **Queries.** Session 74's NAS-scale cases
  (`docs/experiments/data/0276-cases.json`) without the 10 group A cases
  of panel 1's validation half: A (12 development directions), B (7
  drum-loop pairs), C (6 Special Branch spans), D (32 `parts` edges), N1
  (40 random passages), N2 (40 intros). New family E: panel 2's
  development half, its 10 directions, built as 0276 built group A (the
  passage's middle 20 s in the played file; expected the related
  recording's files; the played recording's other files apart), written
  to `work/s86/` and committed as `docs/experiments/data/0296-cases.json`
  before the scored run.
- **Runs.** `break-search` at NAS scale, every list and filtered, on
  the turntable ladder and on both ladders (`--ladder both`), with the
  library-wide clusters as the clusters in use (a scratch work
  directory).
- **Labels of each listed file** (every holder of 20 hits or more,
  ranked by hits, the query file itself left out):
  - *relevant*: in the query file's new cluster; an expected file or
    one of the played recording's other files of the case, or a file
    in the new cluster of one of them; or a file
    `docs/same-titles.txt` pairs with the query file or an expected
    file;
  - *unrelated by judgment*: a file judged by ear not to share the
    passage (none exists today: the answer log holds no `shared`
    answer), or in the new cluster of such a file;
  - *unjudged*: every other file, kept apart.
- **Scores**, per group, mode and ladder, pooled over the positives
  (A-E): precision at 5, 10 and 20 over judged holders (relevant over
  relevant plus unrelated, among the first K), with the judged share of
  the first K and the relevant share of the first K (the lower bound
  that counts unjudged files as not relevant); expected-holder recall
  (an expected file or a member of its new cluster listed at all, and in
  the first 20); the share of listed files unrelated by judgment or by
  the new clusters; the number of distinct recordings or families the
  scores rest on.
- **The null.** N1 and N2: the strongest listed file outside the query
  file's new cluster and outside the case's own files; *T* = the 95th
  percentile of N1's; the found expected files' median hits; the
  random passages that still list a file reaching 60 s of the query
  outside the new cluster (rips or versions the new clusters missed).
- **Worth a design note** (with both ladders, every list): (1)
  expected-holder recall at least 90%; (2) precision at 10 over judged
  holders at least 0.9 pooled, decidable only when the judged share of
  the first 10 is at least 50% (otherwise the pack's answers decide
  it); (3) *T* below the found expected files' median hits; (4) a
  median of at most 60 thread seconds per query. All four needed; the
  decision stays ours (decisions queue item 4).

### R3. The listening pack

Three famous-break lists (Special Branch 250-270 s, Krome & Time -
License, Bigger Star - Raid; the research note's recommendation) from
the both-ladder, every-list run: unjudged holders at ranks 1-3, the
middle three and the last three; and the null's five strongest finds
outside the new clusters. One question each: does B play the same
passage as A's span? Written under `work/s86/`, never answered tonight.

## Decisions

- *(06:15)* **Group A's validation directions are left out.** Session
  74 searched them; the standing rules keep the validation halves
  unsearched, so this session neither searches nor scores them, and
  0296's comparisons with 0276 use the development cases only, rerun
  with this session's build (0276's data is not read for them).
- *(06:15)* **A chain contradicting a verdict is cut, not stopped.** A
  stop would leave the scratch run without clusters; `--cut-sparsest`
  keeps the files judged different apart and lists each cut as a
  question for the ear. The existing NAS clusters file has no cut link.
- *(06:15)* **Candidates from an excerpt, not the whole file.** A whole
  file per query costs about 40 thread seconds (0169), so the library
  would take about 14 hours. The excerpt changes which pairs are
  measured, not how a pair is judged; (a) of R1 measures what the
  excerpt loses against the existing clusters.
- *(06:15)* **Timing sample first.** A sample of the queries (one
  round, not scored) times the full run; if the full run would take
  more than about 60 minutes, the candidate count or the excerpt is
  reduced before the full run, and the change is written here.
- *(06:17)* **The clustering started without a separate timing
  sample.** The run prints its progress every 500 queries, which served
  as the sample: 500 queries in 27 s, so the full run was due in about
  37 minutes, under the 60 minutes the registration allowed; nothing was
  scaled down. It started at 06:17 with `work/bin/s86-run/` (`58c53c1`)
  in tmux `s86clu` (`work/s86/cluster.sh`, log
  `work/s86/logs/cluster.log`); the index holds 40,688 files (session 75
  pruned some since session 74's 40,696). The harness's later changes
  (`break-search --passage-ladder`, `questions breaks`) were built and
  tested on the same machine while it ran, so its wall time is not a
  product timing.
