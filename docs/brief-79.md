# Session 79 brief: the fourth drop rule of `identify --under` and a parallel mask

Written on 2026-11-07, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it), `AGENTS.md` and
   `docs/session-rules.md`.
2. This file, and `docs/session-79-checklist.md` (progress).
3. ADR 0025, experiments 0272-0275 (registered in `docs/brief-73.md`),
   `docs/decisions-queue.md` item 2, notes "Session 73".
4. The code: `crates/gunfinger/src/under.rs`,
   `crates/gunfinger-core/src/residual.rs`, `parallel.rs`, and
   `identify.rs` where it calls `under::find`.

## The orchestrator's prompt, verbatim

> Start session 79 of Gunfinger: the two fixes `identify --under` needs before its default can be decided (the roadmap's "The active queue", item 2; docs/decisions-queue.md, item 2). Session 73 measured the option (experiments 0273-0275, notes "Session 73"): on the three development mixes it gained and lost no entry; on generated double drops it gained 38 right entries with 0 wrong; at NAS scale it cost 2.8-2.9 times the default's time (about 35 s per hour of mix), which fails the registered bar of twice; most of that time is the mask, which runs on one thread, and the confident entries are processed one after another. Its fourth drop rule (`crates/gunfinger/src/under.rs`, `named_as_one_track`) accepts `lines_up_with` within a 90 s neighbourhood without requiring the two plays to overlap, and dropped two right partners whose projected file starts happen to resemble another upload's, 33-36 s away (0274; one was confident at 333 hits). This session fixes the drop rule, makes the step use the machine's threads, and measures both against session 73's numbers. The option stays off by default and ADR 0025 stays Proposed: the default is the user's decision.
>
> You run unattended: the user cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.
>
> The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32). Timing comparisons need an otherwise idle machine: run them alone, A B B A.
>
> Time box: started about 15:18 on 2026-11-07; aim to finish within about 2.5 hours. If a step would run past that, scale it down (the drop rule's fix and the parallel step with byte-identical output first; the cost at NAS scale on the three development mixes; the generated mixes on the seeds 0274 used), record the decision in the brief and continue.
>
> Environment (sessions 33-78's handoffs in docs/session-notes.md; check each):
> - Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
> - Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
> - NAS store: 40,666 records, revision `0bd3f0c23fabc698` (session 75 pruned the records of DJ sets and a mixed CD's track fix). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-78-linux` (262 tracks) and `session-78-linux-nas` (NAS scale), both with the 2003 and Nocny scans since session 78; `regress --strict` fails on any difference; a session that adds baselines names them in its handoff, and the next prompt says which to use.
> - Build `work/bin/s78-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
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
> 1. Read CLAUDE.md and follow its reading list, notes "Session 72", "Session 73" and the newest session, ADR 0025 and 0017, experiments 0272-0275 (their registration in docs/brief-73.md), docs/decisions-queue.md item 2, and the code of `crates/gunfinger/src/under.rs`, `crates/gunfinger-core/src/residual.rs`, how `identify` runs it, and how the core runs work in parallel elsewhere (`index`, the search's workers).
> 2. Write docs/brief-79.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-79-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s79/listening-digests-start.txt. Experiments number from 0281.
> 3. Build `work/bin/s79-start`; the start regressions follow docs/session-rules.md.
>
> Steps:
>
> 1. Pre-registration, committed before the scored runs: the drop rule's fix (for example: the fourth rule applies only to plays that overlap in the mix or that its other conditions tie to the same passage; the exact-file and tag-based duplicate handling stays as it is; no title is preferred), written so that the two saved failures of 0274 can be replayed; the parallel design (confident entries in parallel, the mask's frames in parallel, or both), with the requirement that the step's output is byte-identical to the serial step's for any number of workers; the scores (on the development mixes and on 0274's generated seeds, with the option against itself before the fix: entries gained and lost, wrong lines, wrong confident partners, partners dropped by each rule; at NAS scale, the cost per mix and per confident entry, A B B A, wall time and CPU and peak memory, against the default and against session 73's numbers); fresh generated seeds that no session has used, for a confirmation run after the fix is frozen; and what the numbers would mean for the default (written as evidence for the decisions queue, not a decision).
> 2. The implementation, tested: a unit test for each saved failure (the right partner kept) and for a genuine repeated upload and an aligned unrelated drop (still dropped where they should be); a test that the step's output is identical with 1 and with many workers. With the option off, every output and both regressions are byte-identical.
> 3. The runs, under tmux with logs where long: the development mixes at 262 tracks and NAS scale; 0274's generated drops at NAS scale; the confirmation seeds; the cost A B B A on the idle machine.
> 4. Experiments (at most 40 lines each, written before the next starts): the drop rule's fix; the cost of the parallel step.
> 5. Update ADR 0025's open points with the numbers (status stays Proposed), docs/decisions-queue.md item 2 (the evidence now, and what is still unmeasured: real annotated double drops, key lock, broadcast compression, a mistaken foreground), and the roadmap's item.
> 6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
> 7. Wrap-up: a "Session 79" section at the end of docs/session-notes.md with a handoff; update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-78-linux` and `regress session-78-linux-nas` identical with the final build (option off; `--strict` if session 77 added it), docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the fix and what it recovered, the cost before and after against the bar of twice, the confirmation seeds, and anything left.
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
> - Scratch data goes under work/s79/; delete renders you no longer need.
> - Prose: no em dashes (spaced en dashes in prose, ASCII in code).
> - Commit after each step that changes tracked files.
> - Time: the user's soft deadline is 11:00 on 2026-11-08. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.

## State at the start (15:20)

- Master at `f432981` (session 78's wrap-up); `git status` clean apart
  from `yafc20261031.log`.
- `work/bin/s79-start/` is byte-identical to `work/bin/s78-final/`
  (`gunfinger` `5c0e2c66...`, `gunfinger-eval` `8e01bb7a...`), and the
  stores are unchanged (`scripts/analysis/store_revision.py`: corpus 265
  records at `526a3b978eb3e411`, NAS 40,666 at `0bd3f0c23fabc698`), so
  the start regressions are skipped (`docs/session-rules.md`).
- Start digests in `work/s79/listening-digests-start.txt`:
  `docs/pair-verdicts.txt` `51dd1bba...`, the NAS clusters file
  `c4d3da2e...`, the answer log `3e99ebca...`.
- Session 73's generated renders are in `work/s73/gen/mixes/` (drops
  2050 and 2053, long 2052) with their exclusion lists
  `work/s73/nas-exclude-<seed>.txt`. Session 73 ran them against the NAS
  store at revision `7210fc479da58376` (40,674 records); session 75's
  prune removed 8 records since, so session 73's reports are not
  today's default: every comparison below is rerun on today's store.
- Session 73's timing variant recorded each detection's fate
  (`work/s73/fates-<seed>.json`). The fourth rule
  (`named_as_one_track`) dropped 11 plays on seed 2050 and 20 on seed
  2053, none on the long seed 2052. The two right partners among them:
  seed 2050 `mix-14`, Chris SU & SKC - Blackout
  (`DSCI4-DSCI4LP002EF-.../E-Chris_SU_and_SKC-Blackout-sour.mp3`, 333
  hits, confident, 589.8-633.3 s, under Aphrodite - Cool Rock Stuff),
  whose line puts its file's start 0.5 s from where the confident play of
  Tee Bee - Two Faced (421.2-553.3 s) puts that file's start; and seed
  2053 `mix-00`, Ed Rush & Optical - Splinter (`...Wormhole, Virus
  Recordings VRS 001 LP track B) RSD 2023.opus`, 111 hits, possible,
  646.2-686.2 s), 3.5 s from Carlito & Addiction - Into Music's
  (461.0-611.1 s).

## Plan

1. Pre-registration (below), committed before any scored run.
2. The fix and the parallel step, with tests; `scripts/check.sh`; a run
   build `work/bin/s79-run/`.
3. Runs, one queue under tmux (`work/s79/queue.sh`), the machine
   otherwise idle: the cost A B C C B A on the development mixes at both
   scales first, then the generated drops 2050 and 2053 at NAS scale
   (default, the option before the fix, the option after), then the
   confirmation seeds.
4. Experiments 0281 (the drop rule) and 0282 (the cost), ADR 0025's open
   points, decisions queue item 2, the roadmap's item.
5. Review by a subagent, fixes, wrap-up, final checks.

## Pre-registration (step 1)

Written before any scored run. Builds: *before* is `work/bin/s79-start/`
(session 73's option as shipped); *after* is the build with the fix and
the parallel step (`work/bin/s79-run/`, frozen before the scored runs;
any later change gets a new registration and new seeds).

### The fix of the fourth drop rule

Today `named_as_one_track` drops a play found under an entry when a
confident play of the recording overlaps it or lies within 90 s of it
(`plays::MAX_GAP_SECONDS`) and the two line up (`lines_up_with`: the
projected file starts agree to 5 s) or their files are tagged alike.
Within 90 s the line test alone matches different tracks whose
projected starts happen to agree, which dropped the two partners above.

The fix: the line test counts only when the two plays are one passage
of the mix, that is when they overlap or the gap between them is at
most 10 s (`PASSAGE_GAP_SECONDS`, the gap across which a play's own
segments are joined into one passage, `report.rs`). Lining up across a
gap of 10 s or less is a track going on alone after the louder one
stops (the case the rule was written for, which this keeps); lining up
across a longer silence of the track is not evidence that it is one
track. The tag test (`tagged_alike`) keeps the 90 s neighbourhood, and
the first three rules (the entry's files; shared material; the same file
confident within 90 s) are unchanged. No title is preferred and no
threshold, rule or output changes without `--under`.

The two saved failures are replayed twice: as unit tests built on their
geometry (Two Faced's confident play as saved in session 73's report and
a find whose line puts its file's start 0.5 s away, starting 36 s after
that play ends: kept; the same find 8 s after: dropped), and by
searching the two mixes again (below). Further unit tests: a genuine
repeated upload over the same passage, lining up (still dropped); an
aligned unrelated drop inside the same passage, lining up within 5 s
(still dropped: the line test cannot tell it from an upload, and the
rule keeps that cost, as ADR 0017's entry rule does); an upload tagged
alike 60 s away (still dropped).

### The parallel step

The step today processes the confident entries one after another; for
each, the decode (FFmpeg) and the mask (`residual`) run on one thread,
then the residual is searched with the workers. The design:

- Phase 1, in parallel: for every confident entry, the reference is
  decoded and placed (`placed`) and the residual computed (`residual`),
  one entry per worker (`parallel::map_in_order`, `jobs` workers), each
  on one thread. Results come back in entry order.
- Phase 2, in entry order on the calling thread: as today, each
  residual is searched with the workers (`search_with`, `jobs`), the
  drop rules and the shared-material check applied, warnings printed.
  No nested parallelism: phase 1 ends before phase 2 starts.
- The mask's frames are not parallelised (a core change with more
  code; considered only if phase 1 leaves the bar unmet, then with its
  own registration).
- Requirement: the step's output is byte-identical to the serial
  step's for any number of workers. Phase 1 is pure per entry and keeps
  entry order, and phase 2 is today's code. Tested end to end: a mix of
  two double drops, `identify --under -f json` with `--jobs 1` and
  `--jobs 4`, the same bytes; and on the Linux machine the Stakka mix at
  262 tracks with `--jobs 1` and the default 32, the same JSON.
- Memory: phase 1 holds every entry's residual at once (4 bytes per
  sample at 8 kHz, about 115 MB per hour of entries) and up to `jobs`
  masks in flight; the peak memory is measured.

### Scores

**D, the three development mixes** (Stakka, razorjack 2003, Nocny), at
262 tracks and NAS scale, the whole library: default (*after* build
without the option), *after* with `--under`, *before* with `--under`.
Scored: entries gained and lost against the default, wrong lines, plays
found under (by level), and *after* against *before* (expected: the same
tracklist). The development mixes score against their manifests as in
0273.

**G, session 73's generated drops** (seeds 2050 and 2053, 32 mixes, the
renders and exclusion lists of 0274), at NAS scale: default, *before*
and *after*. Scored with `tracklist_score.py` against the generator's
truth: entries gained and lost against the default, wrong lines, wrong
confident finds (of plays found under), truth partners printed by kind
(quiet, brief, unrelated, pair), and *after* against *before*: the
plays the fix releases, each judged against the truth (right partner;
another upload of a track the tracklist names nearby, that is a
repeated entry; wrong). Partners dropped by each rule: *before*'s from
a recount on today's store is not possible without session 73's timing
variant, so rules 1-3 are taken as unchanged by construction (same
plays, same code) and rule 4's drops after the fix are counted as
*before*'s rule-4 drops (session 73's fates, cross-checked against
today's *before* reports) less the plays the fix releases. The long seed
2052 is not rerun: the fourth rule dropped nothing there, so the fix
cannot change it, and the parallel step's identity is tested apart.

**Bars for the fix (F).** F1: both saved failures recovered: Blackout
printed as an entry marked `(under N.)` in seed 2050 `mix-14`, Splinter
in `under` as possible in seed 2053 `mix-00`. F2: no wrong line and no
wrong confident find gained, and no entry lost, on D and G. F3, reported
and not a bar: repeated entries gained (a released play naming a truth
track the tracklist already names within 90 s), each named.

**C, the cost at NAS scale and 262 tracks**, the machine otherwise
idle, 32 workers, after one warm-up per scale: each development mix in
the order A B C C B A (A default, B *after* with `--under`, C *before*
with `--under`), `/usr/bin/time -v`: wall time, CPU (user + system),
peak memory; per mix, per confident entry searched under, per hour of
mix; means of the two runs of each kind. Against the default and
against session 73's numbers (0275). **Bar (J3, as registered in
session 73):** at NAS scale the option takes at most twice the
default's wall time on each development mix. Reported apart: the same
at 262 tracks (not a bar), the G runs' wall times (not A B B A, a
second figure), and the step's own time from `--verbose`'s "searched
under N confident tracks in S s" line.

**Confirmation, fresh seeds.** After the build is frozen, generated
drops on seeds 2091 and 2092, which no session has used (`mixes --kind
drops --count 16 --other-root /mnt/atlas/Music/dnb --tagged
work/s39/tags-262.json` in `work/s79/gen/`, their exclusion lists by
`tracklist_score.py exclude` with each seed's sweep panel, plus the
held-out files' NAS copies as `work/s73/exclude_copies.py` adds them);
if a seed draws no pair, the next unused seed (2093). At NAS scale,
default, *before* and *after*, scored as G. Bar: no wrong line and no
wrong confident find gained (F2 on fresh data); reported: partners
printed by kind, plays the fix releases and their truth. These rest on
the same 262-track recordings as 0274's seeds (the generator draws from
them): they test repeatability on new draws, not other recordings.

### What the numbers would mean for the default (evidence, not a decision)

- If F1, F2 and J3 hold, both fixes asked for before the default are
  measured and every registered criterion of session 73 is met on
  generated mixes and the development mixes; what stays unmeasured is
  real annotated double drops, key lock, broadcast compression and a
  mistaken foreground (a wrong confident louder entry taken out). The
  decisions queue would then say that, with the added seconds per hour
  of mix, and keep the recommendation of one real double drop with a
  known tracklist before turning it on.
- If J3 fails, the queue gives the ratio and the added seconds per hour,
  and what is left of the step's time by phase.
- If F2 fails, the fix is not adopted in that form; the failure is
  reported and the rule stays as shipped.

## Decisions

- (15:20) The start regressions are skipped: the start build is
  byte-identical to `s78-final` and the stores are unchanged.
- (15:20) Session 73's default reports are not reused, since the NAS
  store changed after them (session 75's prune); default, *before* and
  *after* all run on today's store.
- (15:20) A third kind (C, *before* with `--under`) joins the A B B A
  order, so that the cost before and after is measured on one store and
  one machine state; session 73's numbers are compared as well.
- (15:20) Confirmation seeds 2091 and 2092: no document or work
  directory uses them as seeds (the numbers occur only inside unrelated
  data files).
