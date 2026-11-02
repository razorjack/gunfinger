# Session 73 brief: the cost and the effect of `identify --under`, for the default

Written on 2026-11-02, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file (its pre-registration above all), and
   `docs/session-73-checklist.md` (progress).
3. `docs/brief-72.md` (the option's design); `docs/session-notes.md`
   ("Session 65", "Session 70", "Session 72"); `docs/status.md`,
   "2026-11-02: the user's answers on a quiet track under an identified
   one"; ADR 0025, 0006, 0019, 0023; the proposals note's §4 and
   "Evaluation and adoption boundaries"; experiments 0269-0272.
4. The code: `crates/gunfinger/src/under.rs`, `export.rs` (the
   tracklist), `crates/gunfinger-core/src/residual.rs`.

`docs/brief-2.md` to `docs/brief-72.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 73 of Gunfinger: measure `identify`'s new quiet-track step to decide whether it is on by default. Session 72 built proposal 4's residual into the product behind an option, off by default, with an ADR in status Proposed (docs/session-notes.md, "Session 72"). The user decided that the step should be on by default, subject to its cost, and asked for the cost to be measured first (docs/status.md, "2026-11-02: the user's answers on a quiet track under an identified one"). Sessions 65 and 70 measured it only on generated double drops; real mixes, the time it adds and the printing in use are unmeasured. This session measures the product's option, as shipped, against the default on the same inputs, and writes the numbers the default decision needs. It is a measurement session: the default does not change; the decision is ours, in the ADR.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32). Timing comparisons need an otherwise idle machine.

Time box: started about 05:45; aim to finish within about 2.5 hours, because another session may follow you tonight. If a step would run past that, scale it down (the three development mixes and one pair of generated seeds first), record the decision in the brief and continue.

Environment (sessions 33-72's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,674 records, revision `7210fc479da58376` (session 64 pruned four ignored uploads). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-64-linux-nas` (NAS scale; compare with it, not `session-54-linux-nas`).
- Build `work/bin/s72-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
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
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-72.md and the newest brief, docs/session-notes.md ("Session 65", "Session 70", "Session 72"), docs/status.md (the newest entries), the ADR session 72 wrote, the proposals note (§4 and "Evaluation and adoption boundaries"), ADR 0019 and 0023, experiments 0269-0271 and session 72's, and the code of the option.
2. Write docs/brief-73.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-73-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s73/listening-digests-start.txt. Experiments number from 0273.
3. Build `work/bin/s73-start`; `regress session-54-linux` and `regress session-64-linux-nas` identical.

Steps:

1. Pre-registration, committed before any run with the option: the inputs (the three development mixes at 262 tracks and NAS scale, scored against their manifests; generated drops mixes on seeds no session has used, and the long generated mixes, at NAS scale; the Stakka scan's leave-outs); the scores, the option against the default on the same input (tracklist entries gained and lost against the manifests, a partner's start time against the manifest's, new wrong lines and wrong confident partners, partners dropped by the related-material rule, gap lines that change); the cost (added wall time and CPU per mix and per confident play, A B B A on the idle machine, peak memory, at 262 tracks and NAS scale); a budget variant if the cost is high (for example the step only for plays above a length, or a cap per mix), measured beside; and what would justify turning the step on by default (written as a proposal for us, with the numbers).
2. The runs, under tmux with logs where long; the development mixes are scored with today's harness; nothing is tuned on them.
3. Experiments (at most 40 lines each, written before the next starts): the development mixes; the generated mixes; the cost and any budget.
4. Update the ADR's open points with the measurements (status stays Proposed: the default is ours), and the proposals note's §4 and the roadmap. Write, for the user, a short "what the tracklist looks like" sample in the notes: one development mix's tracklist with and without the option.
5. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
6. Wrap-up: a "Session 73" section at the end of docs/session-notes.md with a handoff; update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-54-linux` and `regress session-64-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: entries gained and lost on the development mixes, wrong lines, the generated mixes, the cost, a recommendation on the default, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's behaviour does not change: no threshold, parameter, rule, default or output change; `identify --under` stays off by default (the default is ours, in ADR 0025). Harness additions are tested and keep `regress` identical.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
- Scratch data goes under work/s73/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00 on 2026-11-02. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 73

- 2026-11-02, 05:44 by the Linux machine's clock (CET). Aim to finish by
  about 08:15; no new run after 10:20 in any case.
- `master` at `31ff279`, clean apart from the untracked
  `yafc20261031.log`. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s73/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `c4d3da2e7e37...`, `work/audition/answers.jsonl` `3e99ebca589b...`;
  all three as session 72 left them.
- Experiments 0001-0272; this session numbers from 0273. ADRs
  0001-0025 (0025 Proposed). Test set: 4 of 5 evaluations used (none in
  this session).
- The NAS store: 40,674 records, revision `7210fc479da58376`. Build
  `work/bin/s72-final/` (master). Baselines `session-54-linux` and
  `session-64-linux-nas`.
- Seeds used by earlier sessions for generated mixes go up to 2041;
  2050, 2051 and 2052 appear nowhere as seeds (docs, code, scripts,
  panels, reports, renders).

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build and both regressions (tmux `s73start`) | 05:44-06:05 |
| 1 | pre-registration (below), committed before any run with the option | 06:05 |
| 2a | tools: `tracklist_score.py` reads plays found under others, tested; `under_score.py` (manifests, leave-out lists, tracklist differences), tested; a timing variant build in a worktree, never merged | 06:05-06:30 |
| 2b | renders of the generated mixes (harness, scratch work directory) | beside 2a |
| 2c | cost: A B B A on the development mixes, both scales, machine otherwise idle | 06:30-06:55 |
| 2d | effect: development mixes; generated mixes at NAS scale; Stakka leave-outs; timing variant | 06:55-07:40 |
| 3 | experiments 0273 (development mixes and leave-outs), 0274 (generated mixes), 0275 (cost and budget) | beside 2d, to 07:50 |
| 4 | ADR 0025's open points, proposals note §4, roadmap, the tracklist sample | 07:50-08:05 |
| 5 | read-only review by a subagent; fixes | 08:05-08:20 |
| 6 | notes, status, checklist; final checks | 08:20-08:45 |

## Pre-registration (step 1, committed before any run with the option)

Everything below is fixed before the first run of `identify --under` in
this session. Nothing is tuned: the option is measured as shipped
(`work/bin/s73-start/gunfinger` = `s72-final`), against the default
(no `--under`) on the same input with the same build and settings.

### Inputs

- **D, the development mixes**: `stakka-skynet-knowledge`,
  `razorjack-2003-03-29`, `nocny-trans-vertical-2002-08-23`, named
  explicitly, at 262 tracks (`~/.config/gunfinger/config.toml`) and at
  NAS scale (`--config ~/.config/gunfinger/nas-dnb.toml`), the whole
  library indexed, `identify`'s defaults otherwise (saved index on,
  default ladder and matcher, 32 workers). Scored against their
  manifests with today's harness rules (duplicate clusters of each
  scale, `docs/same-titles.txt`).
- **G, generated mixes at NAS scale**: drops mixes (`mixes --kind
  drops --tagged work/s39/tags-262.json --other-root
  /mnt/atlas/Music/dnb`, 16 mixes) on the unused seeds 2050 and 2051, and
  long mixes (`mixes --kind long`, 4 mixes) on the unused seed 2052,
  rendered by the harness in a scratch work directory (`work/s73/gen/`,
  its own copy of the panels, so the new sweep panels are written there
  and not into `work/s65/panels`). `identify` at NAS scale with
  `--exclude-from` the NAS files of every recording that holds a
  held-out file of the seed's sweep panel (`tracklist_score.py
  exclude`, as session 39), `--save-dir`, then `show`. Scored with
  `tracklist_score.py` against the generator's truth.
- **L, the Stakka scan's leave-outs**: the 14 patterns of
  `scripts/analysis/leave_out_design.py` for `stakka-skynet-knowledge`
  (30 runs of 1-4 slots), each as `--exclude-from` every file of the
  duplicate clusters of the left-out slots' references (corpus clusters
  at 262 tracks; at NAS scale the NAS files of the NAS clusters holding
  them, and the references' own NAS copies). At 262 tracks with and
  without the option. At NAS scale (the index is built for each run
  with `--exclude-from`, about a minute) only with the option; the
  default there is the same report without `under`, which holds only if
  D and G show the plays and gaps of the two runs identical (checked;
  otherwise the default is run too, time allowing).

### Scores (the option against the default, the same input)

1. **Entries gained and lost**: the printed tracklists compared line by
   line once the numbers and the `(under N.)` mark are taken off.
   Expected: no entry lost or changed, entries gained only.
2. **Each gained entry**, and each play found under another in the JSON
   (confident or possible), is right when its file (or a same-audio
   file) is in the duplicate cluster of a track playing then, or judged
   the same title (`docs/same-titles.txt`): for D and L, a manifest slot
   whose span (its start to the next slot's start) overlaps the find; a
   left-out slot's track cannot be found (its files are out of the
   index), so a find there is right only as another slot's track. For
   G, a truth play overlapping it (`tracklist_score.py`'s rules).
3. **A partner's start**: each right gained entry's start minus the
   start of the truth play (G) or manifest slot (D, L) it names.
4. **New wrong lines**: gained entries that are not right. **Wrong
   confident partners**: confident found plays that are not right
   (also those inside an entry). Wrong possible finds are listed and do
   not count as wrong (ADR 0006).
5. **Partners dropped by the related-material rule**: from the timing
   variant's lines (below), each detection the step dropped, by rule
   (the entry's own files; shared material by stored peaks; the same
   file confident within 90 s; one track with a confident play), and
   whether the dropped file was playing then (G: truth; D: manifest).
6. **Gap lines that change**: the "ID - ID" lines of the two
   tracklists compared (expected: none; a find under a confident play
   lies in covered time).
7. **G by scenario**: each truth play's outcome by default and with the
   option (printed confident, printed under another entry, possible,
   not printed), with `tracklist_score.py summary`'s rows.

### Cost

1. **A B B A**: `identify MIX -f json` of each development mix, default
   (A) then option (B), B, A, at each scale, under `/usr/bin/time -v`
   (wall time, user and system CPU, peak memory), the machine otherwise
   idle (no other run of ours; load checked before each block), after
   one untimed warm-up run per scale. Reported per mix: the means of
   the two A and two B runs, B minus A, B over A, the added seconds per
   confident entry searched under and per confident play, and per hour
   of mix; the two runs of each kind must give identical JSON.
2. **Where the time goes**: a variant build (`work/bin/s73-timing/`,
   from a worktree of master, never merged) that prints, per entry, the
   time of the decode and render, the mask, the search of the residual
   and the shared-material check, and per detection at possible or above
   its fate (kept, or which rule dropped it); its JSON reports must equal
   the product's byte for byte. Run once on D at both scales and on G.
3. **G**: wall time of each `identify` invocation, default and option
   (one run each, sequential, machine otherwise idle), as a second
   view of the cost per mix.
4. **A budget variant**, computed from 2 and the option's reports,
   offline: the step only for entries whose taken-out play spans at
   least 60, 120 or 180 s, and a cap of the K longest entries per mix (K
   8 and 16): the right and wrong finds kept on G and D and the time
   saved. Computed whether or not the cost is high; a variant worth
   proposing would need its own build and an ADR change, not made here.

### What would justify turning the step on by default (a proposal for us)

- **J1, precision**: no new wrong confident line and no wrong confident
  partner on D, L and G. A wrong one means proposing that the step stays
  off until its cause is understood.
- **J2, benefit**: right gained entries on G (quiet and brief partners
  printed that the default leaves out). Gains on D are welcome but not
  required (the manifests list one track per slot).
- **J3, cost**: at NAS scale on D, the option's mean wall time at most
  twice the default's (the step adds at most `identify`'s own time), and
  peak memory at most 1 GB more. If J3 fails, a budget variant that
  meets it while keeping at least 90% of the right gained entries on G
  would be proposed, with its own build and ADR change; if neither, the
  proposal is that the step stays off by default until it is cheaper,
  with where its time goes. The first look (0272: 2.4-2.8 times at NAS
  scale) was known when this bar was set, so J3 is expected to fail
  without a budget; the absolute cost (seconds per hour of mix) is
  reported beside it for us to weigh, as speed comes last in the
  product's order (AGENTS.md).
- **J4, reliability**: every run ends with status 0, and any "nothing is
  reported under" warning is explained.

### Scaling down (if the time box needs it)

In this order: the NAS-scale leave-outs, then the long mixes, then seed
2051. D and seed 2050 stay.

## Decisions

- **(05:47-05:53) Start build and regressions** (`work/s73/start.sh`,
  tmux `s73start`, logs in `work/s73/logs/`; `run.sh` and `same.py` are
  session 72's with the paths changed). Cargo had nothing to rebuild:
  `work/bin/s73-start/` is byte-identical to `s72-final`. `regress
  session-54-linux` (0:50) and `regress session-64-linux-nas` (4:35,
  beside the first render) identical, every report equal to its
  baseline's.
- **(05:51) Renders** (`work/s73/gen.sh`, tmux `s73gen`): the harness's
  `mixes` at 262 tracks in `work/s73/gen/` with its own copy of
  `work/s65/panels`, drops seeds 2050 and 2051 (16 mixes each), long seed
  2052 (4 mixes).
- **(05:52) The timing variant** (`work/s73/wt-timing/`, a detached
  worktree of `7653694`, never merged; `work/bin/s73-timing/gunfinger`):
  `under.rs` prints on stderr, per recording, per entry (`s73-entry`:
  file taken out, start, span, segments, decode, mask, search and
  shared-material seconds, finds kept; `s73-entry-total`), per detection
  at possible or above (`s73-detection`: its file, span, level, hits,
  `taken-out`, `named-nearby` or `candidate`), the files held back as
  sharing material (`s73-shared`) and the found plays dropped as one
  track with a confident play (`s73-one-track`). No other change.
- **(05:53) Seed 2051 replaced by 2053.** `mixes --kind drops --seed
  2051` stopped before drawing: with that seed's held-out recordings no
  pair of the shared-material panel has both files indexed. The next
  unused seed, 2053 (named nowhere), takes its place; nothing else
  changes.
- **(05:54-05:58) Seed 2053's render run again.** A waiting script
  (`gen2.sh`, tmux `s73gen2`) tested for the session `s73gen`, which
  tmux matched as a prefix of its own name, and started beside a second
  start of the same render; both were stopped, the half-written
  `drops-seed-2053` renders and sweep panel deleted, and the render run
  once alone (tmux `s73gen4`, 1:12). Renders: `work/s73/gen/mixes/`
  (drops 2050 and 2053, 16 mixes each; long 2052, 4 mixes of about 83
  min).
- **(05:58) Exclusion lists.** G: `tracklist_score.py exclude` with each
  seed's sweep panel and the NAS clusters (136, 128 and 130 NAS files for
  2050, 2053, 2052); the held-out files' NAS copies in the library map
  were checked and are all in those lists already
  (`work/s73/exclude_copies.py`). L: `under_score.py exclude` for each
  of the 14 patterns, corpus clusters at 262 tracks, NAS clusters and the
  library map at NAS scale (`work/s73/leave/`).
- **(05:59) The queue** (`work/s73/queue.sh`, tmux `s73queue`):
  `cost.sh` (A B B A on D, both scales), then `effect.sh` (the timing
  variant on D; G default with the product and `--under` with the
  timing variant; L at 262 tracks, default with the product and
  `--under` with the variant), one run at a time. The option's runs on
  G and L use the timing variant instead of a second run of the product,
  to save about 40 minutes; this holds only if the variant's JSON equals
  the product's on D at both scales (checked when D is done; otherwise
  G's and L's option runs are made again with the product). The NAS
  leave-outs (`leave-nas.sh`) run after the queue if time allows.
- **(06:13) The timing variant's JSON equals the product's** on all six
  D runs (`cmp`), so G's and L's option runs with the variant stand for
  the product's, as planned at 05:59.
- **(06:25) A partner lost by the fourth drop rule** (seed 2050,
  mix-14): Chris SU & SKC - Black Out, quiet under Aphrodite - Cool Rock
  Stuff, was found confidently (333 hits, three files) and dropped as
  "one track with a confident play": its line puts the file's start at
  325.3 s in the mix, and the line of the confident Tee Bee - Two Faced
  play (421-553 s, ending 36 s before the find) puts that file's start at
  325.8 s. The start test alone (5 s, without the overlap condition the
  review removed in session 72) matched two different tracks. A miss,
  not a wrong line; recorded for ADR 0025's open points, not changed
  (measurement only). The review found a second such partner (seed
  2053, below).
- **(06:25) Repeated entry** (seed 2050, mix-10): a YouTube upload of
  Jo-S - Afterburner was found confidently under Aphrodite - Wikki Wikki
  Plate while the default prints the rip of the same track over the same
  passage; the two lines are 6.7 s apart in start and tagged differently,
  so ADR 0017's rule keeps them apart and the tracklist lists the track
  twice, a second apart. Counted as right and as a repeated entry.
- **(06:27) The NAS leave-outs run** (`work/s73/after.sh`, tmux
  `s73after`, after the queue): with `--exclude-from` the NAS index is
  built in about 18 s, not a minute, so the 14 patterns take about 15
  minutes and fit the time box.
- **(06:09-06:53) The runs** (logs `work/s73/logs/`, every exit status 0
  apart from seed 2051's render): A B B A on D (06:00-06:09); the
  variant on D (06:09-06:13); G (06:13-06:36); L at 262 tracks
  (06:36-06:42) and at NAS scale (06:42-06:53). Nothing was scaled down.
  Results in experiments 0273 (D, L), 0274 (G) and 0275 (cost, budget).
- **(06:50) Verdict against the criteria**: J1 met (no wrong line, no
  wrong confident find), J2 met (37 partners printed that the default
  leaves out), J3 not met (2.8-2.9 times at NAS scale; no budget
  reaches it while keeping 90% of the gains), J4 met. By the
  registered rule the proposal is that the step stays off by default
  until it is faster; ADR 0025's open points give the numbers for our
  decision. No budget variant was built.
- **(06:55-07:12) The review's findings** (a separate agent,
  read-only). Taken: the fourth drop rule dropped two right partners,
  not one (also seed 2053 mix-00's quiet partner, found only as
  possible, 111 hits, 3.5 s from Into Music's file start); the start
  test is described as the lines' file starts, not the plays' starts;
  the long mixes are 72-84 min; the shared-material rule held back 11
  partners (22 detections), the default leaving 9 of them out; 0273's
  find is of the same file as the confident Into Music play, kept
  because that play starts 92 s after it; the proposals note no longer
  states that find is the next track in the blend; 0275's memory in GiB
  and "0.1-0.2 GB" everywhere; the notes no longer quote the user's
  decision; `under-fates` counted one-track drops twice and without
  their level (fixed, tested: `fates` marks the detections of a play
  dropped as one track); `diff` counted a repeated line once (fixed,
  tested). Recorded deviations from the pre-registration: the budget on
  D was first reported for G only (now one line in 0275: on D at NAS
  scale the length budgets keep 94-100% of the step's time, the 8
  longest 57-79%); seed 2053's default run shared the machine with the
  scoring of seed 2050, and the variant's NAS run of Nocny was slowed by
  something else (neither enters 0275; noted in the times table); the
  NAS leave-outs' default reports (`work/s73/leave/out/nas-*-a.json`)
  are the option's reports less `under`, not runs, which the plays'
  identity in all 42 paired runs of D and G allowed; experiments 0274
  and 0275 were written while 0273's leave-out runs went on (the queue
  ran every pre-registered run at once). Not taken: tests for
  `under_summary`, `manifest_score` and `cost`'s aggregation beyond its
  parsing (time); `Truth.judge` reading the library map's copies (no
  effect here: no find needed it). Also noted: the default itself has
  one wrong confident play in the drops mixes (seed 2053 mix-08, Cool
  Rock Shook Ones) and one in the long mixes (seed 2052 mix-00, The
  Nine (Evol Intent VIP)); out of this session's scope, unchanged by the
  option.
- **(06:54-07:07) Final checks** (`work/s73/final.sh`, tmux
  `s73final`, beside the review): `work/bin/s73-final/` byte-identical
  to `s72-final`; both regressions identical (0:51; 3:55);
  `scripts/check.sh` green (7:37); the digests unchanged. The review's
  fixes touched only the analysis scripts and documents; the analysis
  tests pass after them (68).
