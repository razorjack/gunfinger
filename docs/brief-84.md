# Session 84 brief: a second-chance search over the gaps

Written on 2026-11-08, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it), `AGENTS.md` and
   `docs/session-rules.md`.
2. This file, and `docs/session-84-checklist.md` (progress).
3. `docs/research/2026-10-25-ladder-edge-decision.md`, experiments 0219
   and 0226-0230, the notes "Session 51" (`docs/history/session-notes-40-69.md`)
   and "Session 83", the roadmap's "The edge of the ladder" and its
   active queue item 8, `docs/decisions-queue.md` item 9 (key lock),
   ADR 0008, 0021 and 0023.
4. The code: `crates/gunfinger-eval/src/matching.rs` (the gap search's
   options), `crates/gunfinger-eval/src/gap_search.rs` (the search over
   the gaps), `crates/gunfinger-core/src/search.rs`
   (`strongest_per_moment`, the second suppression), `gaps::uncovered`,
   and `mixes.rs` (`Playing`, the generated mixes at the extremes and
   key-locked).

## The orchestrator's prompt, verbatim

> Start session 84 of Gunfinger: a second-chance search over the gaps, starting with targeted extra rungs. Background: the turntable ladder reaches ±8.4%; each extra rung pair moves the reach by 0.4% and costs 4.3-5.3% more search CPU over the whole mix (session 51, experiments 0226-0230, `docs/research/2026-10-25-ladder-edge-decision.md`); for a deck at ±5%, the NAS copies with no other copy in reach fall from 29 at today's reach to 19, 7, 3 and 1 with one to four extra pairs. Session 51 estimated that a targeted search, extra rungs only over the stretches with no confident match (2% of the Stakka mix), costs under 1% more CPU, and that with one extra rung pair it passes the registration, "as a variant to build and measure"; it was never built. (The roadmap's active queue, item 8, cites 0251-0252 for that estimate; the estimate is session 51's, 0226-0230: correct the citation.) The same shape may hold for the key-locked ladder, which `--playback both` searches over the whole mix at 1.57 times the turntable ladder's CPU although no development play uses key lock (`docs/calibration.md`): the product already computes the stretches without a confident match (`gaps::uncovered`, ADR 0023). The external review of 2026-11-05 proposes one design for both. This session builds the targeted extra rungs behind an option and measures them, then, if time allows, the key-locked ladder over the gaps only, measured beside the default. It is a measurement session: defaults do not change (the key-locked default is the user's decision of 2026-10-06; docs/decisions-queue.md item 9).
>
> You run unattended: the user cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.
>
> The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32). Timing comparisons need an otherwise idle machine: run them alone, A B B A.
>
> Time box: started about 01:10 on 2026-11-08; aim to finish within about 3 hours. If a step would run past that, scale it down (the targeted extra rungs at NAS scale first; key lock over the gaps as a recorded next step), record the decision in the brief and continue.
>
> Environment (sessions 33-83's handoffs in docs/session-notes.md; check each):
> - Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
> - Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
> - NAS store: 40,666 records, revision `0bd3f0c23fabc698` (session 75 pruned the records of DJ sets and a mixed CD's track fix). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-78-linux` (262 tracks) and `session-78-linux-nas` (NAS scale), both with the 2003 and Nocny scans since session 78; `regress --strict` fails on any difference; a session that adds baselines names them in its handoff, and the next prompt says which to use.
> - Build `work/bin/s83-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
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
> 1. Read CLAUDE.md and follow its reading list, notes "Session 51" and the newest session, the ladder-edge decision note, experiments 0219 and 0226-0230, ADR 0008, 0021 and 0023, `docs/calibration.md` (the playback rows), the roadmap's "The edge of the ladder", and the code of the ladder (`--extra-rungs`), the search's passes, `gaps::uncovered`, and how `identify` and the harness's scans run the search.
> 2. Write docs/brief-84.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-84-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s84/listening-digests-start.txt. Experiments number from 0292.
> 3. Build `work/bin/s84-start`; the start regressions follow docs/session-rules.md.
>
> Steps:
>
> 1. Pre-registration, committed before the scored runs: the design (after the default search, the stretches without a confident match, padded by a stated margin, are searched again on the extra rungs only; the second search's detections pass the same confidence rule; how they join the first search's plays, with ADR 0021's second suppression unchanged); the scores, against session 51's registration where it applies: plays at a deck extreme recovered (the copies with no other copy in reach; the generated mixes and `robust` at the extremes), wrong confident answers (the bootleg of 0226-0230 that one more speed reaches, on its own line), the development scans and sweeps unchanged or every change explained, the CPU against the whole-mix extra rungs and against the default, A B B A; the same for key lock over the gaps if time allows (the generated key-locked mixes' plays recovered against `--playback both` over the whole mix, and the CPU); and what would justify an ADR.
> 2. The option, tested (the stretches searched, the joining of detections); with it off, every output and both regressions byte-identical.
> 3. The runs at NAS scale (262 tracks where cheap), under tmux with logs where long.
> 4. Experiments (at most 40 lines each, written before the next starts): targeted extra rungs; key lock over the gaps.
> 5. Update the ladder-edge note, the roadmap's items and its active queue (the corrected citation), and docs/decisions-queue.md item 9 with the numbers.
> 6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
> 7. Wrap-up: a "Session 84" section at the end of docs/session-notes.md with a handoff; update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-78-linux` and `regress session-78-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: what the gap-only searches recover and cost against the whole-mix ones, any wrong answer, and anything left.
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
> - Scratch data goes under work/s84/; delete renders you no longer need.
> - Prose: no em dashes (spaced en dashes in prose, ASCII in code).
> - Commit after each step that changes tracked files.
> - Time: the user's soft deadline is 11:00 on 2026-11-08. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.

## State at the start

- `master` at `251c58e`, `git status` clean (apart from the untracked
  `yafc20261031.log`). `work/bin/s84-start/`, built from `251c58e`, is
  byte-identical to `s83-final` (`gunfinger` `f3ad06283ce8...`,
  `gunfinger-eval` `f4e86de4b796...`). The store revisions are unchanged
  (NAS `0bd3f0c23fabc698`, corpus `526a3b978eb3e411`, read from a 10 s
  `identify` with each configuration). By `docs/session-rules.md` the
  start regressions are skipped; the final ones run.
- Baselines `session-78-linux` and `session-78-linux-nas`; NAS clusters
  `c4d3da2e7e37...` (284 clusters). Digests at the start:
  `work/s84/listening-digests-start.txt`.
- The ladder: `speed::ladder_with_extra_rungs(k)` and its key-locked
  twin; the harness's `--extra-rungs k` extends both ladders over the
  whole query (`Ladder::rungs`). No harness option searches extra rungs
  or key lock over part of a query. `gaps::uncovered` (ADR 0023) gives
  the stretches no span covers; the CLI calls it with the segments of
  confident plays (a play is as confident as its strongest segment) and
  45 s.
- Session 51's reports (`work/s51/`) were searched before ADR 0021's
  second suppression (sessions 53-56), so they are not compared with
  this session's runs; every ladder is run again with today's build.
  Its edge renders (`work/s51/renders/`, 2.0 GB) are reused through
  links where their receipts still match.
- Generated mixes draw turntable speeds within ±6% (`mixes::SPEED_RANGE`)
  and key lock only in two `drops` cases: no generated mix plays at the
  ladder's edge, and none plays whole tracks key-locked.

## Plan

1. **The option, in the harness** (`gunfinger-eval`, tested, opt-in):
   - `--gap-rungs K` (global, beside `--extra-rungs`): after the
     search of the ladder, the stretches without a confident match are
     searched again on the K rungs past either end of each ladder
     searched (the rungs `--extra-rungs K` would add) and nothing else.
     Reports go to `reports/variant-gap-rungs-K/`.
   - `--gap-key-lock` (with `--ladder turntable`): the same, on the
     key-locked rungs at the ladder's tempos. Reports go to
     `reports/ladder-turntable/variant-gap-key-lock/`.
   - The search over the gaps applies wherever the harness searches
     through `Matching::search` or `Matching::search_audio` (sweep,
     scan, regress, robust, mixes, recall), in
     `crates/gunfinger-eval/src/gap_search.rs`. The core gains only a
     public form of the second suppression for detections without
     chains (`search::strongest_per_moment_of`); the CLI does not change.
   - `mixes --playing edge-speeds|key-locked` (short mixes): every
     second play at a speed 8-10% from native (either side, drawn), or
     key-locked at its drawn tempo; reports `mixes-edge-speeds-seed-S`
     and `mixes-key-locked-seed-S`.
   - Tests: the stretches searched (padding, merging, the minimum, the
     recording's edges), the joining (the second suppression across the
     two searches, times shifted), and a synthetic query at 9% that only
     the gap search finds; with the options off, every output and both
     regressions byte-identical.
2. **The runs** (scratch work directories under `work/s84/`, NAS scale
   first, under tmux with logs): the registration's conditions below.
3. **Experiments** 0292 (targeted extra rungs) and 0293 (key lock over
   the gaps), each written before the next one's runs start.
4. The ladder-edge note, the roadmap's "The edge of the ladder" and
   active queue item 8 (the corrected citation: session 51's estimate is
   0226-0230, not 0251-0252), decisions queue item 9.
5. Review by a subagent (read-only, no further agents); fixes.
6. Wrap-up: notes, status, checklist, final checks, commit.

Time box: started 01:10; aim to finish by about 04:10-05:00. If time
runs short: the targeted extra rungs at NAS scale first; key lock over
the gaps as a recorded next step.

## Decisions during the session

- **(01:40) The gap search is a harness option over the matching
  options, not a variant build.** It needs a second search after the
  first, which a profile patch cannot express, and the harness's
  commands then take it without change. Conservative: the product's
  code paths are untouched (the core gains one public function the
  harness calls).
- **(01:40) Extra rungs extend both ladders**, as `--extra-rungs` and
  session 51's T_k: with `--ladder both` (the default), T_k searches 4k
  rungs over the gaps.
- **(01:40) T1 and T2 are measured; T3 only if time allows.** T1 is the
  variant session 51's estimate passed; T2 tests whether the known
  bootleg answer (0227) comes back, as the estimate inferred.
- **(01:40) Generated mixes at the extremes need a new option** (above),
  for short mixes only: they are quick to render and search, and each
  play's speed is in the truth.

## Registration (committed before the scored runs)

**Design.** `--gap-rungs K` (T_K) and `--gap-key-lock` (G), harness
options of the matching (`crates/gunfinger-eval/src/gap_search.rs`):

1. The default search of the ladder runs as today.
2. Its detections are grouped into plays (`plays::group`); every segment
   of a confident play (a play is as confident as its strongest
   segment, ADR 0013) covers its time. `gaps::uncovered` lists the
   stretches of the query that no such segment covers, at least 10 s
   long (one window; a shorter stretch holds less than a window of its
   own); each is padded by 10 s on either side within the query, and
   padded stretches that touch are merged. The CLI's 45 s minimum is not
   used: a play at a deck's extreme can leave a shorter stretch between
   two blends.
3. Each padded stretch's audio is cut out and searched with the same
   matcher and index on the gap rungs only: for T_K the 4K rungs that
   `--extra-rungs K` adds (K past either end of both ladders, as session
   51's L_K and T_K); for G the key-locked rungs at the turntable
   ladder's 41 tempos (with `--ladder turntable`). The detections' query
   times are shifted by the stretch's start.
4. The two searches' detections are joined: sorted by hits, the
   first search's first on ties, and passed once through the search's
   last suppression, which for the default matcher is ADR 0021's second
   suppression (`SpansAtSpeed(0.006)`), unchanged (`search::joined`).
   A detection of either search is dropped only when it overlaps a
   stronger one of the same asset within 0.6% in speed.
5. Every detection keeps its own evidence and meets the frozen rule (240
   hits in 3 windows; possible at 60); plays and scores are built from
   the joined detections as from any search.

**Arms.** L0 (the default, 82 rungs), L1 and L2 (`--extra-rungs 1, 2`,
the whole query), T1 and T2 (`--gap-rungs 1, 2`); for key lock, K
(`--ladder turntable`), G (`--ladder turntable --gap-key-lock`) and B
(`--ladder both`, which is L0). Today's build for every arm: session
51's reports predate ADR 0021 and are not compared.

**Conditions** (seed 2026; NAS scale is the saved index with
`--other-peaks-dir`; scratch work directory `work/s84/w`):

- C1, `sweep --speeds edge` (1,440 queries, the full index) at 262
  tracks and NAS scale: L0, L1, L2, T1, T2.
- C2, `sweep --other-rips` at the sweep's 9 speeds and the edge speeds
  (NAS scale): L0, L1, T1; L2 and T2 if time allows.
- C3, the standard protocol: `regress session-78-linux` and `regress
  session-78-linux-nas` with T1, T2 and G (the sweep's 9 speeds, the
  three development scans, Stakka's leave-outs 3 and 11), what changes
  against the baselines (L0), and `calibrate` per arm and scale.
- C4, generated mixes at the extremes: `mixes --playing edge-speeds
  --count 24` (short mixes; every second play 8-10% from native speed),
  at 262 tracks and NAS scale: L0, L1, L2, T1, T2.
- C5, `robust --seed 2026` at 262 tracks: L0, L1, T1 (its speeds
  outside the ladder and its key lock among the conditions), and K, G.
- C6, the cost: the three development scans at NAS scale, alone on the
  otherwise idle machine, in the order L0 T1 L1 L1 T1 L0 per mix, and
  T2 L2 L2 T2 for Stakka; process CPU (user + system, `/usr/bin/time`)
  and the report's `wall_seconds`; the share of each mix the gap search
  covers.
- K1, generated key-locked mixes: `mixes --playing key-locked --count 24`
  (every second play key-locked at its drawn tempo, ±6%) at 262 tracks
  and NAS scale: K, G, B.
- K2, the cost: the three development scans at NAS scale in the order
  B G K K G B per mix, as C6.

**Scores.**

- Recall: confident recall by played speed (C1) and by relative speed
  to the nearest copy left in (C2, `ladder_edge.py recall`, bins of
  0226); C4's plays at the edge (|speed - 1| in [8, 10]%) recovered
  confidently (the scorer's highest level among a play's matching
  detections), by bins of 0.4%, and the other plays; K1's key-locked
  plays and its turntable plays apart.
- Wrong answers: confident detections outside the truth's cluster (and
  outside the same-title credits, ADR 0022) in every run, with which are
  new against L0 (B for key lock); the known bootleg of 0227 (Aphrodite
  - Cool Rock Stuff against the "Cool Rock Shook Ones" upload) and
  China Cup against ƆƐI3C's upload on their own lines; wrong possible
  answers.
- The standard protocol: every change against the baselines listed and
  explained; calibrate's weakest identifying and strongest false
  candidate per arm and scale.
- Cost: the medians and ranges of C6 and K2 and their ratios to L0 (B),
  and T_K against L_K.
- Copies in reach (no search): T_K's reach R(T_K) from C1, and 0229's
  counts at that reach.

**Criteria.** T_K passes, as a proposal for an ADR whose adoption is
ours, only if all hold:

- (G1) R(T_K) > R(L0) in C1 at 262 tracks (the largest edge speed at
  which at least 57 of 60 indexed excerpts are confident at both signs,
  and at every edge speed below it).
- (G2) In C2, of the queries with |relative speed| in (R(L0), R(L_K) +
  0.4], T_K's confident recall is at least 30 points above L0's, and
  within R(L0) T_K recalls no fewer queries than L0.
- (G3) In C4, of the edge plays beyond R(L0), T_K recovers at least as
  many as L_K less 5 points, and of the other plays no fewer than L0.
- (W) No new wrong confident answer against L0 in any run (C1-C5) at
  either scale; calibrate's strongest false candidate at most 10% above
  L0's and its weakest identifying not lower; the development scans
  keep the same confident plays.
- (C) Process CPU of each NAS-scale development scan within +5% of L0's
  (session 51's bound for T_K).

G passes, as evidence for decisions queue item 9 (the default is the
user's), only if: (K-G1) in K1 G recovers at least B's key-locked plays
less 5 points and every turntable play B recovers; (K-W) no new wrong
confident answer against B in K1, C3 and C5; the development scans keep
B's confident plays; (K-C) its CPU on the development scans is at most
0.8 times B's.

*Expected, declared now.* T1 reaches 8.8% as L1 does, within a few
queries; T2 brings back the Cool Rock bootleg answer at +5% (inferred in
0227), so T2 fails W; T1's CPU about +1% on Stakka, more on the mixes
with more time uncovered (Nocny); G finds the key-locked plays that lie
in gaps, which in generated mixes is nearly every key-locked play past
1%, at a cost near K's on the development scans.

What would justify an ADR: a T_K or G that passes every criterion above.
A failure is reported as a failure; a revised candidate needs a new
registration and data these runs did not use.

## Decisions during the runs

- **(02:05) Key lock's runs deferred until 0292 is written.** The first
  queues (`work/s84/queue-a.sh`, `queue-b.sh`) interleaved experiment
  0293's runs (G, K and B on key-locked mixes, `regress` with G, `robust`
  with K and G) with 0292's, against the rule that an experiment file is
  written before the next experiment's runs start. The queue shells
  could not be stopped from the session, so the run build's
  `gunfinger-eval` became a wrapper (`work/bin/s84-run/gunfinger-eval`,
  the binary beside it as `gunfinger-eval-real`) that exits with status
  3 for any key-lock run; those runs show `exit 3` in the queue log and
  run again in `work/s84/queue-k.sh` after 0292 is committed. The timing
  scans are split the same way: C6 (`queue-cpu.sh`) and K2 (in
  `queue-k.sh`), each palindromic on its own.
- **(02:20) Queue A split.** Run one after another, queue A's NAS-scale
  runs would end near 04:05. Its later runs (the NAS-scale edge mixes,
  then L2's and T2's other-copies sweeps) run in a parallel queue D
  (`work/s84/queue-d.sh`, 16 workers) from 02:20, and the wrapper exits
  with status 3 for queue A's copies of them. The 262-track edge mixes
  ran again in queue C after the report-name fix (`8d20d58`: the first
  reports were named `mixes-seed-2026.json`, without the label, and were
  deleted). Runs after 02:12 use the fixed build (`gunfinger-eval`
  `a0df0182f47d...`; the change is only the mixes report's name and
  field). The timing queue (`queue-cpu.sh`) starts once A, B and D are
  done (`work/s84/after.sh`).
- **(03:20) Key lock scaled down.** The 3-hour aim ends near 04:10. Key
  lock over the gaps runs in a reduced form: K1 at both scales, `regress`
  with G at both scales and K2's timing; C5's `robust` with K and G is
  left out (robust's key-lock conditions are at ±2-8%, which K misses
  past about 1% by 0009). Experiment 0292 was first committed at 44
  lines (`a7ba8b0`) and cut to 40 before key lock's runs started.
- **(03:40) C5's key-lock part brought back.** Queue K ended at 03:39,
  earlier than expected, so `robust` with K and G at 262 tracks runs
  (`work/s84/queue-r.sh`) before 0293 is written; B's `robust` is L0's
  from 0292's runs (the same arm and behaviour).
