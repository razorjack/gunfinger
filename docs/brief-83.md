# Session 83 brief: the band profile of experiment 0034 under the default matcher

Written on 2026-11-07, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it), `AGENTS.md` and
   `docs/session-rules.md`.
2. This file, and `docs/session-83-checklist.md` (progress).
3. Experiments 0034 and 0207, the roadmap's "Peaks across frequency
   bands" and its active queue item 7, `docs/calibration.md`'s row
   "Peak neighbourhoods that widen with frequency", ADR 0003, 0008 and
   0014.
4. The code: `crates/gunfinger-core/src/profile.rs` (`Profile::spread`,
   `neighbourhood_bins_at`, `Profile::id`), `peaks.rs` (the picker uses
   `neighbourhood_bins_at`), and the variant patches
   `work/s83/variant-a.patch` and `work/s83/variant-b.patch`.

## The orchestrator's prompt, verbatim

> Start session 83 of Gunfinger: retest the band profile of experiment 0034 under today's default matcher. In session 4, `Profile::spread` (variant a: a peak dominates ±9.4% of its bin, within 4-24 bins, instead of ±12) put fewer peaks at 2-4 kHz, where a mix keeps least, and raised the development mix's true hits by about a third (39,555 to 53,046) with 9% fewer postings and better margins; but shared material between related records grew too (the strongest audio not in the index neared the possible tier's 60 hits, 28 to 43), and on `robust` variant a lost on equal-level blends while variant b gained on blends and broadcast. That was measured under the matcher of session 4 and the skip at 240; the default since ADR 0008 is the fitted matcher with link rules, ADR 0014's set-aside lists, ADR 0021 and ADR 0022. The roadmap asked for a test under the default matcher on the shared-material challenge set "before extracting the NAS records again", and no later experiment ran it. Experiment 0207 found that most lost reference pairs under damage have a missing peak endpoint, so peak placement, not pairing, is where recall is lost; the external review of 2026-11-05 ranks this retest among the open measurements. It is a measurement session at 262 tracks: the product's profile does not change, no NAS records are extracted, and no store the product uses is written.
>
> You run unattended: the user cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.
>
> The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).
>
> Time box: started about 23:40 on 2026-11-07; aim to finish within about 2.5 hours. If a step would run past that, scale it down (variant a first; the shared panels' development halves and `recall` first, then `robust`), record the decision in the brief and continue.
>
> Environment (sessions 33-82's handoffs in docs/session-notes.md; check each):
> - Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
> - Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
> - NAS store: 40,666 records, revision `0bd3f0c23fabc698` (session 75 pruned the records of DJ sets and a mixed CD's track fix). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-78-linux` (262 tracks) and `session-78-linux-nas` (NAS scale), both with the 2003 and Nocny scans since session 78; `regress --strict` fails on any difference; a session that adds baselines names them in its handoff, and the next prompt says which to use.
> - Build `work/bin/s82-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
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
> 1. Read CLAUDE.md and follow its reading list, notes "Session 4" and the newest session, experiments 0034, 0207 and the shared panels' newest (the roadmap's "A confidence statistic for shared material" lists them), ADR 0003, 0008 and 0014, `docs/calibration.md`, and the code of `profile.rs` (`Profile::spread`, the profile's version in the record), how a variant store is extracted at 262 tracks into a scratch directory (session 4 used `work/variants/band-{a,b}/`), and how the harness's commands take a store.
> 2. Write docs/brief-83.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-83-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s83/listening-digests-start.txt. Experiments number from 0290.
> 3. Build `work/bin/s83-start`; the start regressions follow docs/session-rules.md.
>
> Steps:
>
> 1. Pre-registration, committed before the scored runs: the variants (a, and b if time allows, exactly as 0034 defined them unless the code has moved; say how they map onto today's code); the stores (262 tracks, extracted into `work/s83/`, never into `work/peaks` or the NAS store); the evaluations under the default matcher: the standard protocol (the sweep's seeds, the development scans), `recall`'s development panel, the shared panels' development halves (never the validation halves), `robust`, `grid`; the scores: identifications and wrong confident answers, margins (weakest identifying, strongest false), the possible tier's false candidates and audio not in the index, true hits per mix, postings and index size, shared material's strongest detections; the bars (for example no new wrong confident answer on the shared panels' development halves, no fall in the sweeps or development scans, a stated gain under damage), written first; and what passing would lead to (a NAS re-extraction measured in another session, a profile bump and an ADR; all the user's decision).
> 2. Any harness change needed to run a variant store through today's commands, tested; `regress` identical.
> 3. The runs, under tmux with logs where long; variant stores deleted at the end unless a saved report depends on them.
> 4. Experiments (at most 40 lines each, written before the next starts): the protocol and recall; the shared panels and the false candidates.
> 5. Update the roadmap's item, `docs/calibration.md` only if a figure it states is contradicted (with a pointer), and docs/decisions-queue.md if the result leaves a decision for the user.
> 6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
> 7. Wrap-up: a "Session 83" section at the end of docs/session-notes.md with a handoff; update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-78-linux` and `regress session-78-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: what the band profile does today under the default matcher, against the bars, and anything left.
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
> - Scratch data goes under work/s83/; delete renders you no longer need.
> - Prose: no em dashes (spaced en dashes in prose, ASCII in code).
> - Commit after each step that changes tracked files.
> - Time: the user's soft deadline is 11:00 on 2026-11-08. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.

## State at the start

- `master` at `50433c0`, `git status` clean (apart from the untracked
  `yafc20261031.log`). `work/bin/s83-start/`, built from `50433c0`, is
  byte-identical to `s82-final` (`gunfinger` `f3ad06283ce8...`,
  `gunfinger-eval` `f4e86de4b796...`). The store revisions are unchanged
  (NAS `0bd3f0c23fabc698`, corpus `526a3b978eb3e411`, read from a 10 s
  `identify` with each configuration). By `docs/session-rules.md` the
  start regressions are skipped; the final ones run.
- Baselines `session-78-linux` and `session-78-linux-nas`; NAS clusters
  `c4d3da2e7e37...` (284 clusters). Digests at the start:
  `work/s83/listening-digests-start.txt`.
- `Profile::spread` is still in the core (`Option<Spread>`, `None` in
  `Profile::CURRENT`); `Profile::id` appends ` spread=SHARE:MIN..MAX`, and
  the peak picker reads each bin's half-size from
  `neighbourhood_bins_at` (the fast picker is tested against the
  definition with a spread profile, `peaks.rs`). The rounding and limits
  are 0034's (`profile.rs` unchanged since `718d861` apart from the
  power-sum channels). No harness option selects a spread profile:
  `--front-end` (`stereo::FrontEnd`) knows the downmix and the power sum
  only, and `sweep`, `calibrate` and the other library's records
  (`padding.rs`) read `Profile::CURRENT`.

## Plan

1. **Variant builds, never merged** (as session 29's `s29-span`): the
   source of `50433c0` exported with `git archive` into `work/s83/src`
   and `work/s83/src-b`, with `Profile::CURRENT.spread` set to 0034's
   variant (`work/s83/variant-a.patch`, `variant-b.patch`), built into
   `work/s83/bin-a` and `bin-b`. Every command of the harness and the
   CLI then runs unchanged on the variant's records, which carry their
   own profile identifier, so no store can mix them with today's.
   Decision (23:45): this needs no harness change in master, so step 2
   is empty and `regress` stays identical by construction. A harness
   option threading a spread profile through `FrontEnd`, `padding.rs`,
   `sweep.rs` and `recall.rs` would touch about ten call sites for a
   variant that may never be adopted; the calibration register's "To
   adopt" for this row is exactly "set `spread` in `Profile::CURRENT`".
2. **Stores** (built 23:46-23:50, `work/s83/setup.sh`, `setup-b.sh`):
   - Corpus: `gunfinger index corpus/library` with each variant build,
     `--config work/s83/empty.toml` (the configurations' limits, 1:30 to
     15:00) and `--peaks-dir work/s83/stores/{a,b}/corpus`: 265 records
     each, as `work/peaks`. Today's arm (T) reads `work/peaks` itself.
   - The other library at 262 tracks: byte copies (`cp -p`) of the NAS
     files that the development halves name (shared panel 1's 16
     scenarios, panel 2's 10, `recall`'s 80 development sources):
     `work/s83/nas-files.txt` (111 files, 1.08 GB; `work/s83/files.py`)
     in `work/s83/other/library`. Its records today's way
     (`stores/today/other`, `s83-start`) equal the NAS store's byte for
     byte for 109 files.
   - Decision (23:49): the other two (`103-makoto-jupiter_fields` and
     `112-future_cut-stealth_(domination)`, mixed-CD tracks) have no NAS
     record because the NAS's `.gunfingerignore` leaves them out; they
     are removed from the scratch library and its stores, so that, as at
     NAS scale, the panels leave them out. Both removed files are `recall` sources, so the
     reports hold 78 of the panel's 80. One recall source is named
     decomposed in the panel and listed precomposed on the Linux mount
     (ADR 0020); it is copied under its listed name. (Correction after
     the review: it is searched under that name in every arm, as at NAS
     scale, and not left out as first written here.)
   - Decision: the other library is these 109 files, not the NAS store:
     a variant record of every NAS file is the re-extraction the prompt
     excludes. The NAS map and clusters files are copied into each arm's
     work directory as the scratch store's (`reports/library-other/`),
     standing in for its own, as in session 82.
3. **The registration** (below), committed before the scored runs.
4. **The runs** (`work/s83/arm.sh ARM`), arm T with `s83-start` and arm a
   with `bin-a` side by side (16 jobs each), then arm b if time allows,
   each in its own work directory `work/s83/w-{T,a,b}`.
5. Experiments 0290 (the protocol and recall) and 0291 (the shared
   panels and the false candidates), each written before the next starts.
6. The roadmap's item, the decisions queue, the review, the wrap-up.

## Registration (committed before the scored runs)

**Variants.** 0034's, unchanged: the neighbourhood's half-size in bins at
bin `k` is `round(0.094 k)` clamped to 4-24 (a) or 4-16 (b), the same as
today's ±12 at 1 kHz (bin 128); frames stay ±12. In today's code that is
`Profile::CURRENT` with `spread: Some(Spread { share: 0.094, min_bins:
4, max_bins: 24 })` (a) or `max_bins: 16` (b); the identifier gains
` spread=0.094:4..24` or ` spread=0.094:4..16`. Everything else (hashes,
matcher, rules, ladder, set-aside share) is today's default: the fitted
matcher with link rules (ADR 0008), the set-aside lists chosen over each
arm's own index (ADR 0014), `SpansAtSpeed(0.006)` (ADR 0021),
same-title credits (ADR 0022). Variant a first; b only if time allows.

**Arms.** T: `s83-start`, `work/peaks`, `stores/today/other`. a: `bin-a`,
`stores/a/corpus`, `stores/a/other`. b likewise. Each in its own work
directory with copies of the corpus clusters and the NAS map and clusters
files; `--panels work/s33/panels`.

**Evaluations**, at 262 tracks (the corpus alone) unless marked +109 (the
corpus and the 109 NAS files):

1. The standard protocol: `sweep --seed 2026, 2027, 2028, 2029`; `scan`
   of `stakka-skynet-knowledge` (and `--leave-out 3 --seed 2026`,
   `--leave-out 11 --seed 2026`), `razorjack-2003-03-29` and
   `nocny-trans-vertical-2002-08-23`; `calibrate`; `loss --set
   stakka-skynet-knowledge` (0034's true hits and postings looked up).
2. `recall` (+109), development panel of seed 2026, query sets clean,
   damaged, brief and brief-damaged.
3. `shared` (+109) on `shared-material-seed-2026-development.json` and
   `shared-material-2-seed-2026-development.json` (the development
   halves; the validation halves are never named).
4. `robust` (seed 2026) and `grid` (seed 2026).

**Scores.** Per arm: identifications and wrong confident answers in each
evaluation; calibrate's weakest identifying, strongest false and margin;
the possible tier's false candidates (count, of 30 hits or more, the
strongest unrelated, the strongest audio not in the index); true hits per
development mix (summed hits of the identifying confident detections),
`loss`'s true hits and postings looked up; postings and index bytes
(`gunfinger stats`); on the shared panels, confident and possible
detections by role (played, related, partner, other) and index (everything,
played left out, source left out), and the strongest related and other
detections in hits; `robust`'s confident and possible-or-better by
transformation; `grid`'s confident by play length.

**Bars** for a variant against T (all must hold; each failure is reported
as a failure):

- B1, precision: no new wrong confident answer in the sweeps, the
  development scans and leave-outs, `grid`, `robust` and `recall`: every
  query with a wrong confident answer under the variant has one under T.
- B2, shared material: on each panel's development half and each index,
  the number of queries with a confident detection of role related or
  other is not above T's, and every confident (query, file) of role other
  under the variant is confident under T.
- B3, no fall: in each sweep, recall at every speed is not below T's; in
  each development scan, identified tracks are not fewer than T's.
- B4, margins: calibrate's margin (weakest identifying over strongest
  false) is not below T's, and the strongest audio not in the index and
  the strongest unrelated false candidate stay below the possible tier
  (60 hits).
- B5, a gain under damage: on `recall`'s damaged and brief-damaged sets
  pooled, the variant identifies at least 2 percentage points more of
  the searched queries than T; and `robust`'s confident total is not
  below T's.

Reported without a bar: postings, index bytes, true hits per mix, `loss`,
the counts of false candidates, recall's clean and brief sets, the
shared panels' possible tier and strongest detections.

**What passing would lead to.** Nothing in this session. A pass makes the
case for a NAS-scale measurement in another session (variant records of
the NAS files in a scratch store, never the NAS store, with the NAS-scale
protocol and the shared panels there), then a profile bump (`spread` set
in `Profile::CURRENT`, every record extracted again) with an ADR and one
test evaluation; each step is the user's decision (decisions queue). A
failure is recorded with the bars it failed, and the roadmap's item says
what a further test would need.

## Decisions during the session

- *(23:51)* Arms T and a started side by side (`arm.sh`, tmux `armT`,
  `arma`, 16 jobs each). Arm T's sweep 2026 equals `session-78-linux`'s
  query for query (720 of 720), so the scratch work directories
  reproduce the baseline.
- *(00:00)* B4's "unrelated" is not decided by the harness: calibrate
  lists every false candidate of 30 hits or more, and we read each
  (query recording, file) pair by name. A pair counts as related when it
  is a remix or version, a judgment by ear names it shared material, or
  T already lists it; a pair of different titles that none of these
  covers is reported as unconfirmed and listed for listening, not
  counted as unrelated.
- *(00:09)* Arm b started (tmux `armb`, 16 jobs) while T and a run
  `shared`, `grid` and `robust`: the protocol and recall took about 16
  minutes per arm, so b fits the time box. No time from these runs is
  reported.
- *(00:09)* Slip: `arm.sh` runs every registered evaluation in one
  queue, so the shared panels' searches (experiment 0291) started before
  0290 was written. 0290 holds the protocol, `recall`, `grid` and
  `robust`. (After the review: 0290 was committed 91 s before 0291, and
  the shared panels had been summarised before it, so only the order of
  the commits holds.)
- *(00:48)* Slips: 0290 and 0291 were each first committed over 40 lines
  (41 and 43) and corrected in the next commit.
- *(00:52)* `docs/calibration.md` is left unchanged: its row "Peak
  neighbourhoods that widen with frequency" states 0034's figures under
  the matchers of session 4, and no figure it states is contradicted by
  0290 or 0291 (the "To adopt" column still holds). The decisions queue
  gains item 10 (close the item or register a revised test), since
  neither variant passed.
- *(01:05)* A read-only review by a subagent recomputed the figures and
  found no figure error in the tables. It found: panel 1 described as 12
  scenarios of 4 families (it has 16 of 6; flags occur in 12 of 4); the
  00:00 reading of B4's "unrelated" was written after T's and a's
  calibrate had finished (23:59), so B4 is now reported as not shown for
  either variant, since the APH33 pair (two titles, unheard) reaches 83
  hits; Phantom 2018 described as unrelated (it is a remake of the played
  track, with role other in phantom-force-5, where T has it 5 times); the
  robust figure beside "equal-level blends"; the recall source with a
  decomposed name (searched, not left out); claims that read as
  measurements but are inferences from 0207; the checklist. All fixed in
  0290, 0291, the roadmap, the decisions queue and this brief.
