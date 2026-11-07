# Session 81 brief: short chains under library perturbations, and a bounded fit for short chains only

Written on 2026-11-07, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it), `AGENTS.md` and
   `docs/session-rules.md`.
2. This file, and `docs/session-81-checklist.md` (progress).
3. ADR 0008 and 0014, experiments 0242 and 0251-0254 and 0278,
   `docs/research/2026-10-30-second-pass-fit-decision.md`, the roadmap's
   "A minimum span instead of 3 windows", `docs/decisions-queue.md`
   item 1.
4. The code: `crates/gunfinger-core/src/search/refine.rs` and
   `refine/fit.rs` (the second pass and its fits), `search/chains.rs`
   (`chain_speed`), `crates/gunfinger-eval/src/matching.rs` (the opt-in
   variants) and `scripts/analysis/perturbation.py` (this session's
   scores).

## The orchestrator's prompt, verbatim

> Start session 81 of Gunfinger: how stable short chains' fitted speeds and hits are when the library changes, and whether a bounded second-pass fit restricted to short chains helps. Background: the second pass fits a play's speed from its chain of hits; experiment 0242 showed that a prune moves posting lists across the set-aside cut (ADR 0014) and so moves fitted speeds of untouched files, and that a short chain's speed can hinge on one hit; sessions 64 and 75 (0254, 0278) saw the same mechanism move refits and weak candidates. Session 63 (0251-0253, `docs/research/2026-10-30-second-pass-fit-decision.md`) measured three second-pass fits: the bounded fit stabilised one short query (602 hits, while the default moved from 503 to 455) and over the wider trial gained 19 confident answers and lost 4, and lifted shared material too; it was not adopted, and its note named a bounded fit "for short chains only" as the candidate, never measured. The minimum span (docs/decisions-queue.md, item 1) and any wider ladder depend on short chains, so their stability is measured before either. The external review of 2026-11-05 (`docs/research/2026-11-05-external-review.md`) asks for a perturbation panel rather than one attractive example. It is a measurement session: the product does not change.
>
> You run unattended: the user cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.
>
> The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).
>
> Time box: started about 19:21 on 2026-11-07; aim to finish within about 2.5 hours. If a step would run past that, scale it down (262 tracks and two perturbations first), record the decision in the brief and continue.
>
> Environment (sessions 33-80's handoffs in docs/session-notes.md; check each):
> - Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
> - Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
> - NAS store: 40,666 records, revision `0bd3f0c23fabc698` (session 75 pruned the records of DJ sets and a mixed CD's track fix). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-78-linux` (262 tracks) and `session-78-linux-nas` (NAS scale), both with the 2003 and Nocny scans since session 78; `regress --strict` fails on any difference; a session that adds baselines names them in its handoff, and the next prompt says which to use.
> - Build `work/bin/s80-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
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
> 1. Read CLAUDE.md and follow its reading list, notes "Session 59", "Session 63", "Session 64", "Session 75" and the newest session, ADR 0008 and 0014, experiments 0242, 0251-0254 and 0278, the second-pass fit note, the roadmap's "A minimum span instead of 3 windows", and the code of the second pass's fit (`search/`, `SecondPassFit`), the set-aside lists, `sweep --set-aside-like` and how the harness builds an index that leaves files out.
> 2. Write docs/brief-81.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-81-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s81/listening-digests-start.txt. Experiments number from 0285.
> 3. Build `work/bin/s81-start`; the start regressions follow docs/session-rules.md.
>
> Steps:
>
> 1. Pre-registration, committed before the scored runs: the perturbation panel (indexes built in memory or in scratch directories, never a store: unrelated files added; unrelated files removed; duplicate padding; and lists moved across the set-aside cut by changing only the cut, as `--set-aside-like` does), each at 262 tracks and, where it fits the time, NAS scale; the queries (the sweep's, `grid`'s 10-20 s plays, session 28's brief plays, and the shared panels' development halves, never their validation halves); the measures, kept apart: candidate survival, fitted-speed movement, hit movement, confident identity changes, stratified by chain length and by distance to the confident rule (240 hits, 3 windows); the bounded fit applied only to chains below a length fixed in the registration, the default everywhere else, compared on the same perturbations; what would make it a candidate for an ADR (for example fewer confident changes under perturbation, no new wrong confident answer on the shared panels' development halves, and a confirmation on fresh seeds after it is frozen).
> 2. The variant, harness only and opt-in, tested; `regress` identical with it off.
> 3. The runs, under tmux with logs where long.
> 4. Experiments (at most 40 lines each, written before the next starts): stability under perturbation by chain length; the short-chain bounded fit.
> 5. Update the second-pass fit note, the roadmap's items ("A minimum span instead of 3 windows" and the second-pass fit), and docs/decisions-queue.md item 1 with what the stability means for the minimum span.
> 6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
> 7. Wrap-up: a "Session 81" section at the end of docs/session-notes.md with a handoff; update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-78-linux` and `regress session-78-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: how stable short chains are, what the bounded short-chain fit does, what it means for the minimum span, and anything left.
>
> Rules for this session:
> - `docs/session-rules.md` binds as well; where it and this list differ, this list wins for this session.
> - No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
> - The product's behaviour does not change: no threshold, parameter, rule, default or output change; `identify --under` stays off by default (the default is ours, in ADR 0025). Harness changes and variants are tested, opt-in where they are variants, and keep `regress` identical.
> - Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
> - The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
> - The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
> - Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
> - AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
> - Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
> - Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
> - Scratch data goes under work/s81/; delete renders you no longer need.
> - Prose: no em dashes (spaced en dashes in prose, ASCII in code).
> - Commit after each step that changes tracked files.
> - Time: the user's soft deadline is 11:00 on 2026-11-08. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.

## State at the start

- `master` at `aa11f95`, `git status` clean (apart from the untracked
  `yafc20261031.log`). Build `work/bin/s80-final/`: `gunfinger`
  `5829ad77...`, `gunfinger-eval` `27fdbf19...`.
- `work/bin/s81-start/`, built from `aa11f95`, is byte-identical to
  `s80-final` (`gunfinger` `5829ad77d609...`, `gunfinger-eval`
  `27fdbf197b71...`). The NAS store's revision is unchanged
  (`0bd3f0c23fabc698`, read from a 10 s `identify` with the NAS
  configuration; the corpus store's is `526a3b978eb3e411`). By
  `docs/session-rules.md` the start regressions are skipped; the final
  ones run.
- Baselines `session-78-linux` and `session-78-linux-nas`; NAS clusters
  `c4d3da2e7e37...` (284 clusters). Digests at the start:
  `work/s81/listening-digests-start.txt`.
- The second pass's fits (`--second-pass-fit grid|cross|iterated`) are in
  the tree as session 63 left them; no fit applies to short chains only.
  No harness option moves the set-aside cut by itself: `--set-aside-like`
  takes another saved index's lists, and `--drop-fullest` empties lists
  instead of setting them aside.

## Plan

1. Harness and core options, opt-in and tested, `regress` identical with
   them off:
   - `--second-pass-fit iterated-short`: the bounded fit of 0252
     (`SecondPassFit::Iterated`) only where `chain_speed` gives the mean
     of the rungs (a chain of fewer than 3 lines, or less than
     `MIN_FIT_FRAMES`, 1,000 frames or 16 s, between its first and last
     line's centres); the chain's speed everywhere else.
   - `--set-aside-share SHARE`: the first pass sets aside the fullest
     SHARE of posting lists instead of 1%, so that only the cut moves.
   - `grid` takes `--other-sample`, as `sweep` does.
2. The registration (below), committed before the scored runs.
3. The runs at 262 tracks, then NAS scale where it fits the time.
4. Experiments 0285 (stability by chain length) and 0286 (the short-chain
   bounded fit), each at most 40 lines, written before the next starts.
5. The note, the roadmap, the decisions queue; the review; the wrap-up.

## Registration (committed before the scored runs)

**Perturbations.** Each is compared with its own base, under the same
fit, query by query. Indexes are built in memory; no store is written.

| Name | Base | Perturbed | Scale |
|---|---|---|---|
| `added` | 262 tracks | 262 tracks and 400 seeded random NAS records (`--other-sample 400`) | 262 |
| `removed` | the `added` index | 100 of its NAS records removed (`--other-sample 300`; the samples are nested) | 262 |
| `cut-down`, `cut-up` | 262 tracks | the fullest 0.99% or 1.01% of lists set aside (`--set-aside-share`) | 262 |
| `copies` | 262 tracks | 2 exact copies of each of 50 seeded random indexed corpus records (`sweep --exact-copies 2`, sweep only; copies count as their original) | 262 |
| `removed-nas` | NAS scale | about 400 NAS records removed (`--other-sample 40000`) | NAS, sweep |
| `cut-up-nas` | NAS scale | the fullest 1.01% set aside | NAS, sweep |

**Queries.** `sweep` seed 2026 (720 queries of 30 s) and `grid` seed
2026 (960 brief plays of 10-40 s; its 10-20 s plays are the short
stratum). Recall's brief plays (session 28's) and the shared panels'
development halves run only at NAS scale and took 24-85 minutes per run
in session 63; they are out of this session's time box (decided below).
Never their validation halves.

**Measures**, kept apart, per perturbation and fit, from
`scripts/analysis/perturbation.py`:

- M1 survival: of the base's detections of 10 hits or more (sweep: every
  detection; grid: the plays' matching detections), the share whose file
  still has a detection in the query. Files only one of the two indexes
  holds are counted apart.
- M2 fitted-speed movement: for each surviving file's strongest
  detection, the change of speed in percentage points; the share above
  1e-4, median and largest.
- M3 hit movement: the share whose hits change, and the share that
  change by 5% or more.
- M4 confident identity changes: per query, a confident right answer
  gained or lost (sweep: a confident detection of the truth's cluster or
  title; grid: the play's level `confident`), and wrong confident
  detections added or removed.
- Strata: chain length by its proxy, the query's length (grid 10, 15,
  20; 25, 30, 40 s; sweep 30 s), since the reports hold no first-pass
  chain; distance to the rule by the base's hits (under 120, 120-239,
  240-479, 480 or more) and windows (under 3, 3 or more).

**The variant.** `--second-pass-fit iterated-short` against the default,
on the same perturbations, the same measures. The registered short-chain
condition is `chain_speed`'s own fallback, above; it is not tuned.

**What makes it a candidate for an ADR proposal** (all of):

- A1: summed over the 262-track perturbations, fewer confident identity
  changes (M4, right and wrong, gained and lost) than the default's,
  and no more in any one perturbation.
- A2: on the grid's 10-20 s plays, a smaller share of strongest matching
  detections whose hits move by 5% or more (M3) than the default's,
  summed over the perturbations.
- A3: at the bases, no wrong confident answer added against the default
  (the sweep, the grid, and `regress session-78-linux` run with the
  variant in a scratch directory: its scans, leave-outs and calibrate's
  strongest false at 262 tracks, no higher than the default's 100).
- A4 (not run this session): no new wrong confident answer on the shared
  panels' development halves, then a confirmation on fresh seeds after
  the variant is frozen in a commit. Passing A1-A3 makes it a candidate
  for A4, not an ADR.

A failed bar is reported as a failure. A revised variant would need a
new registration and data the first run did not use.

## Decisions

- *(19:25)* Start regressions skipped: `s81-start` is byte-identical to
  `s80-final` and the store revisions are unchanged.
- *(19:28)* Scale-down, by the prompt's time box: recall's brief plays
  and the shared panels' development halves are not run (NAS scale
  only, 24-85 minutes per run in session 63). The panels are the A4
  check; the brief plays are covered in part by the grid's 10-20 s
  plays at 262 tracks.
- *(19:28)* Chain length is measured by the query's length, because
  neither report holds the first-pass chain (`--trace-candidates` exists
  for the sweep and recall only).
- *(19:28)* `copies` runs for the sweep only, the one command with
  `--exact-copies`.
- *(19:33)* `regress session-78-linux --strict` with the options off
  failed on provenance alone: every report was identical, but the
  baseline's "matching" string had no `set_aside_share`. `Matching` now
  has a `Debug` that prints the field only when given (tested against
  the old string); rerun with `s81-run2`: every report equal, strict
  passes (19:37).
- *(19:38)* `grid` took no `--other-peaks-dir`; added to the commands
  that take it (`s81-run3`). The 262-track queue restarted as
  `q262b.sh`; the default's bases from the first queue are kept (the
  search code is the same in `s81-run` and `s81-run3`).
- *(19:38)* The variant's `regress session-78-linux` (A3) ran before
  the perturbations: calibrate's strongest false 100 to 107, so A3
  fails as registered (the same rise as 0252's B4). The runs continue,
  since the stability measures are the session's main question.
- *(20:05)* The registration put both fits on one panel, so the
  variant's runs (0286) started in the same queues as the default's.
  0285 is written and committed from the default's runs before any of
  0286's comparisons is read (its A3 figure above came from `regress`,
  before the perturbations).
- *(20:11)* Time allows more than the registration's NAS rows: the
  grid at NAS scale under the default, with the registered NAS
  perturbations (`removed-nas`, `cut-up-nas`), the same measures, for
  0285's question (short plays at the scale of the minimum span's
  evidence). Decided before the runs; the variant is not run there,
  since it has already failed A1 and A3 at 262 tracks.
- *(21:00, the review's corrections)* Two records above were not exact.
  The 20:05 entry says 0285 was committed before any of 0286's
  comparisons was read; `work/s81/summary-partial.txt` (20:00), read
  while 0285 was drafted, already held the variant's first comparisons
  (`added`) and its base comparison, so 0285 was written with those in
  view (its text uses the default's runs only). 0287's runs started at
  20:11, before 0286 was written (20:17), against the rule that an
  experiment is written before the next starts. The decision times of
  19:25 and 19:28 were first written as 19:50 and 19:55 by mistake and
  corrected the same hour. The review's other corrections are in the
  documents (0285-0287, the note, the roadmap, decisions queue item 1,
  the status entry) and in commit `ae03fb8` (code: no behaviour
  change).
