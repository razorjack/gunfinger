# Session 68 brief: a pilot of automated counterexample search

Written on 2026-10-31, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file (its pre-registration above all), and
   `docs/session-68-checklist.md` (progress).
3. The proposals note `docs/research/2026-10-29-matching-and-archive-proposals.md`
   ("Automated counterexample search", "Evaluation and adoption
   boundaries"); `docs/brief-37.md`, `docs/brief-39.md`, `docs/brief-67.md`.
4. `docs/session-notes.md` ("Session 37", "Session 39", "Session 48",
   "Session 67"); experiments 0116, 0118, 0166, 0175-0178, 0182-0185,
   0198, 0215-0218 and this session's (0264 and up).
5. `crates/gunfinger-eval/src/mixes.rs` (`PlannedPlay`, `render`,
   `score`, `search_mix`), `robust.rs` (`Fault`, the conditions) and
   this session's module, `crates/gunfinger-eval/src/counterexamples.rs`.

`docs/brief-2.md` to `docs/brief-67.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 68 of Gunfinger: a pilot of automated counterexample search (the proposals note's "Automated counterexample search", `docs/research/2026-10-29-matching-and-archive-proposals.md`). Synthetic construction supplies the truth, so a bounded search over realistic transformations can find small inputs that lose a true answer or create a wrong one, then shrink each to a minimal failing recipe. The note warns that fresh seeds of one recording family are not independent validation (the marked-play experiments found many examples of one shared-material relationship, 0215-0218), and that real positive controls stay necessary because statistics that separated clean renders rejected real true plays (0116, 0118). This session builds the smallest useful version: a harness driver that samples transformations within fixed realistic bounds (speed and pitch motion inside the ladder, EQ including bass kills, blend level and length, channel faults, placement within a play), renders through the existing mix generator, searches, compares with the truth, and minimizes each failure while preserving it; then it runs a budgeted search and reports the failure classes it finds. It is a measurement session: the search and the product do not change.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32). Rendering runs well in parallel here.

Time box: started about 03:30; aim to finish within about 2.5 hours, because another session may follow you tonight. If a step would run past that, scale it down (262 tracks only, fewer transformation axes, a smaller budget), record the decision in the brief and continue.

Environment (sessions 33-67's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,674 records, revision `7210fc479da58376` (session 64 pruned four ignored uploads). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-64-linux-nas` (NAS scale; compare with it, not `session-54-linux-nas`).
- Build `work/bin/s67-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Since `45f92d5` (2026-10-28) its calibrate reads only the reports its baseline holds; still run variants and other seeds in a scratch work directory, never in the reports directory `regress` compares.
- The user restructured the code on 2026-10-28 without meaning to change behaviour (commits `b5a84e3` to `f94604a`: harness subcommands in modules, named results, `Search` passed through the core's search, enums for closed sets). Session 60 confirmed both regressions identical on a build of master after it.
- Since sessions 53-56: `SpansAtSpeed(0.006)` is the default second suppression (ADR 0021); same-title detections (`docs/same-titles.txt`) are scored right (ADR 0022); `identify` prints gaps as "ID - ID" lines with an estimate of their tracks (ADR 0023, 0024). The minimum span stays undecided. A change to an existing decision needs a new ADR (the next number is in `docs/adr/`).
- The research note `docs/research/2026-10-29-matching-and-archive-proposals.md` (2026-10-29) records six unmeasured proposals and its evaluation boundaries; any session working on one follows its "Evaluation and adoption boundaries".
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen. On the evening of 2026-10-30 the user answered 8 of 12 questions (docs/status.md, "the listening of the 2026-10-30 pack"): the four same-title candidates (Cool Rock, Tower Bass twice, Stalker) are different titles; Vintage Dread 2000 disc 1 is separate tracks; Advent, Keltech - Axis and Plasmic - The Weapon have an inverted channel, We - Hello World is unsure. The pitch rides (Star Trails, Into Music) and the Planet Dust cut stay unanswered. When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-37.md, docs/brief-39.md and the newest brief, docs/session-notes.md ("Session 37", "Session 39", "Session 48" and the newest session), docs/status.md (the newest entries), the proposals note ("Automated counterexample search" and "Evaluation and adoption boundaries"), experiments 0116, 0118, 0166, 0175-0178, 0182-0185, 0198 and 0215-0218, and the code of the mix generators (`mixes/`, their kinds and truth), `robust.rs`'s conditions, and how the harness scores a rendered mix.
2. Write docs/brief-68.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-68-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s68/listening-digests-start.txt. Experiments number from 0264.
3. Build `work/bin/s68-start`; `regress session-54-linux` and `regress session-64-linux-nas` identical.

Steps:

1. Pre-registration, committed before the budgeted search: the transformation axes and their bounds (each bound justified from the generator's existing ranges or a measured DJ practice in the notes, never wider than the brief's realistic playback), the failure definitions (a true play lost: no confident detection where the truth has one of at least a stated length; a wrong answer: a confident detection of a file the truth does not play there, with same-title credits apart under ADR 0022 and related versions named), the search (random sampling with a fixed seed and budget, then greedy minimization: drop or shrink one transformation at a time while the failure persists), the recipe saved per failure (seed, sources, transformations, expected answer, the build) under work/s68/, how failures are grouped into classes (by which minimal transformation is needed and by recording family), and what the pilot must show to be worth keeping (failure classes not already listed in the roadmap, or a known one reproduced from a minimal recipe).
2. The driver, tested, harness only (an opt-in subcommand; `regress` identical): unit tests for the minimizer on a synthetic predicate (it returns a minimal failing set) and for recipe round-trips (a saved recipe renders the same audio again).
3. The budgeted search at 262 tracks, then a smaller one at NAS scale if time allows (never write any store), under tmux with logs; renders deleted once their recipes are saved.
4. Experiments (at most 40 lines each, written before the next starts): the driver's first budget and its failure classes; the minimized recipes against the known failures in the roadmap.
5. A short note in the proposals note's "Automated counterexample search" and the roadmap: what the pilot found, its cost per failure, and whether it is worth running beside later sessions.
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 68" section at the end of docs/session-notes.md with a handoff; update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-54-linux` and `regress session-64-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the failure classes found, the minimal recipes, the cost, and anything left.

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
- Scratch data goes under work/s68/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00 on 2026-10-31. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 68

- 2026-10-31, 03:27 by the Linux machine's clock (CET). The time box is
  about 2.5 hours: aim to finish by about 06:00. The soft deadline is
  11:00 on 2026-10-31; no new run starts after about 10:20.
- `master` at `dc137bb`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s68/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `c4d3da2e7e37...`, `work/audition/answers.jsonl` `3dc509cb1ec9...`.
- Experiments 0001-0263; this session numbers from 0264. ADRs
  0001-0024. Test set: 4 of 5 evaluations used (none in this session).
- The NAS store: 40,674 records, revision `7210fc479da58376`. Builds:
  `work/bin/s67-final/` (master). Baselines `session-54-linux` and
  `session-64-linux-nas`.
- What the mix generator offers per play (`mixes::PlannedPlay`): the
  asset and whether it is held out, the start in the track, a turntable
  speed, a pitch ride (linear from the play's speed to another over a
  given time, then held), a bass cut (two 200 Hz two-pole high-passes)
  before a mix time and after another, linear fades, and a gain against
  the common level; per mix, a stereo fault of the right channel over
  the mix's second half (`render`). Renders are 128 kbit/s MP3s.
  Scoring (`mixes::score`): a detection matches a play when its file
  shares the play's cluster or is judged to carry its title (ADR 0022)
  and overlaps the play's audible span; confident detections matching
  no play are wrong.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build and both regressions (tmux `s68start`) | 03:27-03:45 |
| 2 | the driver `counterexamples` (harness only, opt-in), unit tests for the minimizer and the recipe round trip; `regress` identical | 03:45-04:35 |
| 1 | pre-registration (below), committed before the budgeted search | 04:35-04:45 |
| 3 | the budgeted search at 262 tracks (tmux), then a smaller one at NAS scale if time allows | 04:45-05:25 |
| 4 | experiments (0264 and up) | with the runs |
| 5 | the proposals note's "Automated counterexample search" and the roadmap | 05:25-05:35 |
| 6-7 | review, notes, status, checklist, final checks | 05:35-06:05 |

The driver is written before the pre-registration is committed, because
the pre-registration names the driver's exact axes and rules; no
budgeted search runs before the commit. Smoke runs of a few samples test
the driver and are not reported as results.

## Pre-registration (step 1; written at 03:40-03:50, before the budgeted search)

**Question.** Within realistic bounds of the transformations the mix
generator can render, which small transformations lose a true play,
leave a long stretch of a found play without a confident detection, or
make a confident wrong answer, and are those failure classes already in
the roadmap?

**What a sample plays** (`counterexamples::draw`, `Recipe::planned`).
A held-out lead-in of 30 s, cut into the target play (indexed, or held
out with probability 0.25, the generated mixes' share), then a partner
for 30 s after the target ends (indexed, or held out with probability
0.25). The three come from distinct clusters. Placement: the target
lasts 40-120 s in the mix (uniform; at most 85% of its track at the
fastest speed), and each play starts in its track uniformly between
10% of its length and the point that leaves it room before 95%, the
generator's rule (`mixes::drawn_play`). The lead-in and the partner
play at native speed.

**The transformation axes and their bounds.** Each joins a sample with
probability 0.5, independently; every axis's values are drawn whether
or not it joins.

| Axis | Bounds | Justified by |
|---|---|---|
| Speed (the target, turntable) | 0.94-1.06 | the generated mixes' `SPEED_RANGE` (±6%), inside the brief's ±8% |
| Pitch ride (the target) | a change of 1-3% either way over 5-15 s, starting 30-70% into the play; turned back if it would end more than 6% from native | the rides kind's default draw (`RideDraw`, experiments 0175-0176) |
| Bass kill (the target) | the generator's bass cut (two 200 Hz two-pole high-passes) from the target's entry for 10-60 s, at most the target's length | the long mixes' crossfades (10-60 s), in which the bass swap cuts the incoming record |
| Blend (the partner) | the partner comes in 10-60 s before the target ends (at most the target's length), at -6 to 0 dB against it | the long crossfades' lengths; `robust`'s blends at -6 and 0 dB |
| Channel fault (the render) | one of brief 42's eight faults (inverted, silent, noise at 10, 0 and -10 dB, delays of 0.2, 1 and 5 ms) on the right channel over the mix's second half | `mixes::render`'s stereo fault (experiment 0198); inverted and delayed rips exist in the NAS store (0248, 0262) |

EQ is the bass kill only: the generator renders no other EQ, and the
product does not change this session. Key-locked playback, codecs and
noise are `robust`'s conditions and stay out. The draws' rates are not
prevalence estimates: a channel fault joins half the samples, while
0262 found inverted rips in about 0.08% of the store.

**Failure definitions.** A sample is rendered as `mixes` renders (a
128 kbit/s MP3), searched on one thread by the harness's search with
today's defaults and scored by `mixes::score` (a detection matches a
play when its file shares the play's cluster or is judged to carry its
title, ADR 0022, and overlaps the play).

1. *Lost*: the target is indexed and has no confident matching
   detection. Its play lasts at least 40 s.
2. *Gap*: the target is indexed and confident, but a stretch of 30 s
   or more of its play is covered by no confident detection of its
   cluster or title (0176's measure of a side lost, 30 s).
3. *Wrong*: a confident detection of a file that matches no play there
   (a held-out play included). Same-title credits are right (ADR 0022)
   and not counted; each wrong file is named with the play it overlaps
   most, so that related versions show.

**The search.** `gunfinger-eval counterexamples --seed 2026 --samples
2000` at 262 tracks (the sweep index of seed 2026: its held-out
clusters out of the index), one render per sample, 32 workers. Each
failure is minimized greedily (`counterexamples::minimized`): if the
base alone fails, the minimal recipe is the base; otherwise drop one
transformation at a time while the same failure persists (the target
still lost, still with a gap, or the same file still found wrongly),
repeating until no drop keeps it; then halve each remaining
transformation (the speed's deviation, the ride's change, the kill's
and the blend's length; a channel fault has no size) up to 3 times
while it persists. Then, if time allows, `--samples 500` at NAS scale
(`--other-peaks-dir`, the saved index; the NAS clusters, so the draws
can differ from the 262 run's). Renders are deleted once searched.

**The recipe saved per failure** (`<out>/recipes/NNNN.json`): the seed,
the build's path, the ladder and the indexed count, the sample, the
failure, the base (sources and placements), the sampled and the minimal
transformations, the expected plays (asset, held out or not, mix span),
the minimal recipe's outcome and the evaluations spent. Out dirs:
`work/s68/c262/` and `work/s68/cnas/`; builds `work/bin/s68-run/`.

**Classes.** A failure's class is its kind and the kinds of its minimal
transformations (such as `lost: blend+channel-fault`; `none` when the
base alone fails). Within a class failures are grouped by recording
family: the first file of the target's cluster, and for a wrong answer
the wrong file's family over the overlapped play's.

**Comparison with the roadmap.** Known classes: a channel fault over
the play (0198; the roadmap's "Stereo faults before the downmix"), a
side of a pitch ride lost (0175-0176, "Speed that wanders within a
play"), a record inside a stronger one (0184, the printed tracklist's
quiet and brief records), shared material and related versions found
wrongly ("A confidence statistic for shared material"), a play lost
without a transformation (brief plays, intros). Each minimal recipe's
class is assigned to one of these or to "new".

**What the pilot must show to be worth keeping** (either): (a) a
failure class not already in the roadmap, reproduced from its minimal
recipe in at least two recording families, or in one family with a
replay of the saved recipe; or (b) a known class reproduced from a
minimal recipe of one or two transformations. With neither, the pilot
is reported as not worth running beside later sessions in this form.
The cost is reported per failure and per class (renders searched,
elapsed time and CPU).

## Decisions

- **(03:27) Start build and regressions** (`work/s68/start.sh`, tmux
  `s68start`, logs in `work/s68/logs/`; `run.sh` and `same.py` are
  session 67's with the paths changed). The start build is
  byte-identical to `s67-final`.
- **(03:32) Both regressions with the start build identical** (0:51;
  3:47), every report equal to its baseline's apart from `wall_seconds`.
- **(03:33-03:40) The driver** (`counterexamples.rs`, the subcommand in
  `commands/mixes.rs`). Smoke runs of 16 and 64 samples (`work/s68/smoke/`,
  not results) took 24 s and 60 s; renders search in 3-8 s on one
  thread. The gap failure was added after the first smoke run, before
  the pre-registration, so that a ride's lost side (0176), which leaves
  the target confident, can be found.
- **(03:41) Budget: 2,000 samples at 262 tracks, 500 at NAS scale**, by
  the smoke runs' cost (about 15 and 15 minutes expected).
