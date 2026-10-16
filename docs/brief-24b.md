# Session 24b brief: Panako as an outside benchmark

Written by the agent for itself on 2026-10-16, so the plan survives
context compaction. Work autonomously and never stop to ask. When
something is ambiguous, take the conservative option, write the decision
down and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-24b-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 24b" at the end, once written).
4. `docs/status.md` (the "Session 24b" entry at the bottom, once written).
5. ADRs 0001, 0004, 0006; experiments 0001, 0014, 0047 and this
   session's (0130 and up); the roadmap's "Longer hashes" bullet.

The session runs in the worktree
`/Users/razorjack/.t3/worktrees/gunfinger/s24b-panako` on branch
`s24b-panako`. Session 24 runs at the same time in the main checkout
(`/Users/razorjack/Projects/OpenSource/gunfinger`, `master`): never
write there, never run git there, never touch its processes. Read only
its `corpus/` and `work/`, by absolute path.

## The prompt, verbatim

```text
Start session 24b of Gunfinger: run Panako (Joren Six's acoustic fingerprinter, the "candidate B" the brief names) as an outside benchmark on the 262-track development corpus, and compare it with Gunfinger on the same queries. It runs unattended from about 19:15. Heavy runs must end by 00:30 (see "The machine"); light work (scoring, writing) may continue until about 02:00. Work autonomously and never stop to ask. When something is ambiguous, take the conservative option, write the decision down and continue.

Where you run.
- You are in a separate git worktree on the local branch `s24b-panako`, created from master. Session 24 runs at the same time in the main checkout, /Users/razorjack/Projects/OpenSource/gunfinger, on master: never write anything there, never run git commands against it, and never touch its processes. Read its files by absolute path only where you need them: corpus/ (private; the 262-track library and the development sets) and work/ (the harness's renders and reports). Your own scratch goes to work/ in your worktree.
- The branch is merged into master later by session 25, after session 24 ends. To keep that merge simple: number your experiment files 0130 and up (session 25 renumbers them after session 24's last); write your brief and checklist as docs/brief-24b.md and docs/session-24b-checklist.md; do not edit CLAUDE.md; append a "Session 24b" section at the end of docs/session-notes.md and an entry at the end of docs/status.md; at most one short bullet in docs/roadmap.md. Commit on your branch after each step.

Why.
- ADR 0001: Gunfinger's matcher is Wang-style pair hashes searched on a ladder of 41 assumed turntable speeds (0.92-1.08). The brief named a second candidate, hashes invariant to speed after Panako (Six & Leman 2014; Six 2021), to be built only if the ladder failed; it did not fail, so B was never built. Experiment 0014 estimated triplet hashes offline. Sonnleitner et al. (2016) found Panako's triplets more specific but less accurate than pairs on real DJ mixes. Nobody has run Panako itself on this material.
- The question: on the same queries at 262 tracks, how do Panako's answers compare with Gunfinger's (right and wrong answers, recall by speed, boundaries, the three development mixes), and at what cost (index time, query time, memory, index size)? It is a reference point. The product does not change, and nothing from Panako goes into Gunfinger: the brief requires Gunfinger's own fingerprinter, with no third-party fingerprinting code and no copied code. Do not copy Panako's code into the repository.

The machine.
- Java was installed for this: Eclipse Temurin 21.0.12.1 LTS (arm64), /Library/Java/JavaVirtualMachines/temurin-21.jdk, `java` on PATH. FFmpeg is installed (Gunfinger uses it).
- Session 24's heavy runs start after about 00:30. Your heavy runs (Panako indexing and queries, Gunfinger runs) must end by then, one at a time, at `nice -n 10`. Light work afterwards.
- Leave alone every process you did not start, including session 24's.

Getting Panako.
- Clone https://github.com/JorenSix/Panako into a new, empty directory under your worktree's work/ (for example work/panako/src), and build it with its own Gradle wrapper, which downloads its dependencies. Nothing else may be downloaded or installed: no Homebrew packages, no system changes. If Panako needs something more (a native library, another Java version), write down exactly what and stop the Panako part; we decide.
- Record the commit you built, its licence (from the repository) and its version. Read its README and configuration for how to run it; reading is fine, copying is not.
- Keep Panako's state (its database, cache, configuration) under your worktree's work/ where its configuration allows. If it writes anywhere else (for example under ~/.panako), record where, and delete it at the end.
- Use its defaults first. Any change to its configuration is chosen on the sweep only (below), written down with the reason, and reported beside the defaults.

Steps (times are a plan):

1. Setup (until about 19:45): read CLAUDE.md and AGENTS.md (in your worktree), docs/brief.md (never edit it), ADRs 0001, 0004 and 0006, experiments 0001, 0014 and 0047, the roadmap's "Longer hashes" bullet, README's bibliography, and how the harness renders and scores the sweep and the generated mixes (crates/gunfinger-eval, docs/adr/0004-evaluation-protocol.md). Write docs/brief-24b.md (this prompt verbatim, the plan with times) and docs/session-24b-checklist.md. Commit.

2. Panako built and checked (about 45 min): clone, build, run its own self-check if it has one, index a few corpus files and query one of their sweep renders. Commit nothing from Panako; record the commands in the brief.

3. Write down, before any count, in the experiment file: the queries; how a Panako answer is scored against the truth (match the harness's rules: the right recording counts through the 262-track duplicate clusters, a speed or start outside the harness's tolerances does not; state what Panako's output gives and how its time and frequency factors map to Gunfinger's speed); which Panako score counts as a confident answer (its default acceptance, and nothing tuned on the development mixes); and what is compared with Gunfinger. Commit.

4. Runs (until 00:30 at the latest, one at a time):
   a. Index the 262 corpus library files (the harness's library: what `gunfinger-eval` indexes by default; read it from the main checkout's corpus/). Record time, memory and index size.
   b. The sweep renders, seed 2026 first (work/sweep/seed-2026 in the main checkout; the harness's plan says which track, speed and start each query is), then 2027-2029 if time allows: right and wrong answers, recall by speed, start error. Gunfinger's answers on the same queries come from the saved reports (session 24's baseline `session-24-start`, or work/reports in the main checkout; read only).
   c. The three development mixes (razorjack-2003-03-29, stakka-skynet-knowledge, nocny-trans-vertical-2002-08-23), each whole: tracks found, wrong answers and boundaries against their manifests, beside Gunfinger's scans.
   d. If time allows: the generated mixes (work/mixes) and the `robust` renders (damage).
   Record CPU and wall time per run and the load average beside it; times are rough.

5. Scoring and the experiment file(s) (at most 40 lines each): Panako against Gunfinger per panel; where Panako finds what Gunfinger misses or the reverse, name the cases; any wrong answer of either with its cause where it can be named. Conclusions only as far as 262 tracks support them; say what a NAS-scale comparison would need and cost (not run tonight).

6. A read-only review by a subagent; fix what it finds.

7. Wrap-up (by about 02:00): the "Session 24b" notes section (what Panako is, how it was run, the numbers beside Gunfinger's, what it would mean, and that nothing changed in the product), the status entry, the checklist, the roadmap bullet. Run scripts/check.sh in your worktree if you changed any code there (with rustup's 1.98.0 first on PATH and RUSTUP_TOOLCHAIN=1.98.0; never install a toolchain). Delete Panako's state outside work/ if any. Commit on your branch. Last, print what was done, the numbers, and what session 25 needs to merge the branch (files that will conflict, experiment numbers to change).

Rules:
- Never write to the main checkout or run git there; never push; no AI attribution in commits; check `git status` before every commit and never use `git add -A`.
- The held-out test set (corpus/sets/ed-rush-optical-essential-mix and every report named after it, such as work/reports/evaluation-*-scan-ed-rush-optical-essential-mix.json) is out of every step: never open, query or score it, and never run test evaluation 5. Never touch the NAS: no file under /Volumes/atlas is read, indexed or queried tonight (that rules out EM_1999_02_07_-_Ed_Rush_Optical.mp3 and the candidate mixes of experiment 0065 too).
- The validation halves of the recall panel and of both shared-material panels stay unsearched.
- The five test-derived versions are never used to tune anything: Terminal 2, the Orders remixes, the Marky & Bungle Funktion, the Clear Skyz Break remix and 1996 VIP, and the Black Barrel Beachball.
- Agents never write verdicts or listening answers; manifests do not change; the product (crates/gunfinger, gunfinger-core) does not change.
- AGENTS.md: an experiment file of at most 40 lines, written before the next experiment starts; evaluation JSON in docs holds file names only; Python analysis scripts in scripts/analysis/; cross-platform code.
- corpus/ is private and never committed; nothing of Panako's (source, jars, databases) is committed.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
```

## State at the start

- 19:06. `git status` clean at `81e16e0` on `s24b-panako` (master's
  head when the worktree was made: session 24's step 3). Experiments
  0001-0125 on master; this branch numbers from 0130.
- Only one JDK on the machine:
  Temurin 21.0.12.1 (`/usr/libexec/java_home -V`); no `~/.gradle`, no
  `~/.panako`.
- Gunfinger's saved reports in the main checkout (read only):
  `work/baselines/session-24-start/` (sweep seed 2026 and the stakka
  scans with leave-outs, as of the last `regress`, identical at 18:45
  today); `work/reports/sweep-seed-2027..2029.json` (2026-10-08),
  `scan-razorjack-2003-03-29.json` (10-11),
  `scan-nocny-trans-vertical-2002-08-23.json` (10-14),
  `mixes-seed-2026.json` (10-08).

## Plan, with times (as written at 19:30, after step 2's result)

| Step | What | Planned end |
|---|---|--:|
| 1 | setup: reading, brief, checklist; commit | 19:45 |
| 2 | Panako: clone, build; **blocked** (below); record what it needs, clean up | 20:00 |
| 3 | pre-registration in experiment 0130, for a later run once we decide; commit | 21:15 |
| 4 | Gunfinger's side only: its answers on the comparison's queries from the saved reports; a check that the saved reports older than today's `regress` still match the current build (reruns in this worktree, `nice -n 10`, one at a time, ending by 00:30) | 00:30 |
| 5 | experiment files 0130 (the build, the pre-registration) and 0131 (Gunfinger's reference columns) | 01:00 |
| 6 | read-only review by a subagent; fixes | 01:30 |
| 7 | wrap-up: notes, status, checklist, roadmap bullet; `check.sh` only if code changed; commit; the final message | 02:00 |

## Step 2: Panako's build (19:10-19:25)

Commands (from the worktree root; nothing of this is committed):

```sh
mkdir -p work/panako/src-clone
git clone https://github.com/JorenSix/Panako work/panako/src-clone/Panako
cd work/panako/src-clone/Panako
GRADLE_USER_HOME=<worktree>/work/panako/gradle-home ./gradlew --no-daemon shadowJar
```

- Commit `f1248f7a35a06af449f02f7df33c4cfdc1aeedc1` (2026-09-17, "Merge
  pull request #57", a README change; the last code change is from
  2024-05-29). Version 2.1 (`build.gradle`, `codemeta.json`). Licence:
  GNU Affero General Public License v3 (`license.txt`).
- The wrapper downloaded Gradle 7.2 into `work/panako/gradle-home`
  (235 MB) and failed before reading the build:
  `Could not compile settings file ... Unsupported class file major
  version 65`. Class file version 65 is Java 21. Gradle 7.2 runs on Java
  8 to 16; Java 21 needs Gradle 8.5 or later. Panako's CI builds with
  JDK 11 (`.github/workflows/gradle.yml`), and its README's "JDK 11 or
  later" does not hold for its own wrapper.
- **Decision (our rule):** Panako needs something more, so the
  Panako part stops here. Exactly what it needs, either of:
  1. a JDK the wrapper's Gradle 7.2 runs on (11 to 16; its CI uses
     Temurin 11), used only for the build (`JAVA_HOME` for `./gradlew`);
     whether the built jar then runs on Temurin 21 (with the startup
     script's `--add-opens` options) is untested; or
  2. permission to change the local clone's wrapper to Gradle 8.5 or
     later (`gradle/wrapper/gradle-wrapper.properties`), which very
     likely also needs the Shadow plugin's newer release
     (`com.github.johnrengelman.shadow` 8.1.1 in `build.gradle`);
     both downloaded by the wrapper into `work/panako/gradle-home`, no
     system change. Not tried: it downloads something other than
     Panako's own wrapper's Gradle.
  Either way the next possible obstacle is native code: Panako stores
  fingerprints in LMDB through `org.mlmdbjava:lmdbjava:0.9.1` and its
  PANAKO strategy computes its transform through JGaborator
  (`be.ugent.jgaborator:jgaborator:0.7`, JNI). The README says "The
  current release does not support Apple M1 out of the box" for LMDB,
  while its 2.1 changelog says "Panako now also supports Apple M1
  targets". Whether those jars carry macOS arm64 libraries is unknown
  until a build resolves them.
- Nothing ran of Panako, so it wrote nothing outside `work/`: no
  `~/.panako`; `~/.gradle` was not created (Gradle's home was set to
  `work/panako/gradle-home`). The clone and Gradle's home stay under
  `work/panako/` for a later attempt.

## Decisions

- **The Panako part stops at the build** (step 2 above). Steps 4a-d and
  the comparison in 5 are not run. What the session does instead stays
  within the prompt: the pre-registration (step 3), so that a later run
  is mechanical, and Gunfinger's side of the comparison.

## Step 3: the pre-registration (written at 19:50, before any count)

Fixed now, for the run that follows once we decide on step 2.
Nothing may be added after a Panako count is seen; anything computed
later and not named here is exploratory and decides nothing. Nothing of
Panako's goes into the repository or the product.

### Panako's configuration

- The defaults are those of `resources/defaults/config.properties` at
  `f1248f7` (the file `./gradlew install` copies to `~/.panako`), with
  one choice the prompt makes: `STRATEGY=PANAKO`, the speed-invariant
  algorithm (that file's own default is `OLAF`, its Wang-style one).
  Its acceptance, unchanged: `PANAKO_MIN_HITS_UNFILTERED=10`,
  `PANAKO_MIN_HITS_FILTERED=5`, `PANAKO_MIN_SEC_WITH_MATCH=0.2`,
  `PANAKO_MIN_MATCH_DURATION=3`, time and frequency factors within
  0.8-1.2, `NUMBER_OF_QUERY_RESULTS=1000`.
- Only paths change: `PANAKO_LMDB_FOLDER` and `PANAKO_CACHE_FOLDER`
  under `work/panako/` (one LMDB folder per index below; one shared
  cache of prints), given as `KEY=VALUE` arguments, and Panako's
  `config.properties`, if it must exist, in a `work/` folder if Panako
  can be pointed there; otherwise `~/.panako` is recorded and deleted at
  the end. JVM options as in its startup script
  (`resources/defaults/panako`: the two `--add-opens`, `-Xms256m
  -Xmx256m`); `AVAILABLE_PROCESSORS` at its default 1. If a run fails
  for lack of heap, it is rerun once with `-Xmx2g` and that is written
  down. Neither changes what is matched.
- Files are stored and queried by absolute path; a match path is mapped
  back to the library-relative name by removing the library root. After
  the first index: 262 distinct identifiers (`panako stats`), or the
  collisions are listed and their files' answers excluded from every
  count, for both systems.

### The indexes

- **Library**: the 262 files `gunfinger-eval` indexes by default (the
  main checkout's `corpus/library`, the default extensions, the peak
  store's records); the list is written to `work/panako/library.txt`
  and checked against the clusters file's members before any query.
- **I-all**: all 262 (step 4a; its time, memory and size are the cost
  measurements). For the three development mixes.
- **I-seed-N**, N = 2026-2029: the library minus the seed's held-out
  recordings (`docs/panels/sweep-seed-N.json`'s `held_out`, each
  extended to its duplicate cluster, as `Plan::for_seed` does), stored
  from the cached prints into its own LMDB folder. For the sweep and,
  for seed 2026, the generated mixes (as `mixes::sweep_index`).
- **I-stakka-out** (optional, after the rest): the library minus the
  clusters of every `stakka-skynet-knowledge` reference, the equivalent
  of Gunfinger's `--leave-out 11` (a run with no right answer: every
  confident answer in it is wrong).
- Clusters: `work/reports/duplicate-clusters.json` in the main checkout
  (the 262-track clusters, 17 clusters of 37 files), the harness's.

### The queries

- **Sweep**, seed 2026 first, then 2027-2029: the 720 renders in the
  main checkout's `work/sweep/seed-N/` (`NNN-+S.mp3`: draw NNN of the
  panel, indexed draws first, at S% speed; 30 s; 128 kbit/s MP3), with
  `panako query`. Truth from the panel: render time t holds file time
  `start_seconds + (1 + S/100) t` (`render::render_excerpt`).
- **Development mixes**, each whole, against I-all:
  `stakka-skynet-knowledge`, `razorjack-2003-03-29`,
  `nocny-trans-vertical-2002-08-23` (the manifests' `audio` files in the
  main checkout's `corpus/sets/`), with `panako monitor` at its default
  step and overlap, the documented mode for long audio. Beside it,
  deciding nothing: `panako query` on the whole file.
- **If time allows**: the 12 generated mixes of seed 2026 (the main
  checkout's `work/mixes/seed-2026/`, truth in `mix-NN.truth.json`)
  against I-seed-2026, with `monitor`; then I-stakka-out. The `robust`
  renders only after their conditions and report are named here first.

### What a Panako answer is, and how it is scored

- Panako prints a result only when it passes its acceptance; with the
  defaults above, **every printed result is a confident answer**.
  Panako has no possible tier, so Gunfinger's possible plays stay out
  of the comparison (scored apart, as ADR 0006 does).
- Its fields (README): query start and stop, match path, match start
  and stop (seconds in the reference), score (matching fingerprints),
  time factor, frequency factor, share of seconds with a match. The
  **time factor is Gunfinger's speed**: the query plays the reference
  at that ratio, so query time q holds reference time
  `match_start + time_factor (q - query_start)`, Gunfinger's
  `track_start + speed (t - start)`. On turntable renders the frequency
  factor should equal it; it is reported, never used to score. A
  "No match" line, or a query with no line, is no answer.
- **Sweep** (`sweep::speed_row`, `devscore.rescore_queries`): an answer
  is right when its file is in the query's own cluster, wrong
  otherwise, held-out queries included; recall per speed is the share of
  the 60 indexed queries with at least one right answer; wrong answers
  are counted per answer, as the harness counts them. The harness checks
  neither speed nor start in the sweep. Beside it, deciding nothing,
  per right answer: the speed error `time_factor - (1 + S/100)` in
  percentage points (the harness's `mean/max_speed_error_percent`), and
  the start error `match_start - (start_seconds + (1 + S/100)
  query_start)` in seconds. A right answer with a start error beyond
  +-3 s or a speed error beyond +-1 point is listed as "right file,
  other position" and counted again without it, as a stricter recall.
- **Development mixes** (`scoring::score`, `devscore.score_scan`):
  each monitor result is a detection over its span of mix time (its
  window's offset plus its query start and stop, unless the output
  already gives mix time: checked on the first result of the first
  mix, a format check, not a tuning step). A referenced track is
  identified when a detection of a reference or a cluster member
  overlaps the window from 90 s before its start to 90 s after the next
  track's start; every other detection is wrong. References under
  `second-library/` are set aside, as at 262 tracks. **Boundaries**:
  per identified track, the earliest start of its credited detections
  minus the manifest's start, for `stakka-skynet-knowledge` and
  `nocny-trans-vertical-2002-08-23`, and for `razorjack-2003-03-29`
  track 4 only (AGENTS.md).
- **Generated mixes** (`mixes::score`): a detection matches a play when
  its file is in the play's cluster and it overlaps the play's audible
  span; a play is found when a detection matches it; detections
  matching no play are wrong; position error as `position_error_seconds`.

### One change of configuration, chosen on the sweep only

- If Panako's defaults give wrong answers on seed 2026's sweep, one
  stricter acceptance is chosen there and nowhere else: S* = 1 + the
  highest score of any wrong answer of seed 2026 (indexed and held-out
  queries). Every panel is then reported at the defaults and at S*
  (results with score >= S*), side by side. A filter on the printed
  score equals raising `PANAKO_MIN_HITS_FILTERED` if the score is the
  filtered count (unverified: read in the output, not the code). No
  other knob; nothing is chosen on the development or generated mixes.

### What is compared with Gunfinger

- Gunfinger's answers on the same queries from its saved reports, read
  only (the main checkout's `work/baselines/session-24-start/` and
  `work/reports/`), checked in step 4 against the current build where
  older than today's `regress`. Gunfinger's confident detections only.
- Per panel, side by side: recall per speed, wrong answers, speed error
  (and Panako's start error; the sweep reports hold no start for
  Gunfinger); tracks identified, wrong and boundaries per mix; the
  generated mixes' plays found and wrong; the queries one system answers
  rightly and the other does not, by name; every wrong answer with its
  cause where it can be named (a duplicate rip outside the clusters, a
  remix or VIP sharing material, an unrelated file).
- Cost, each run alone at `nice -n 10`, with the load average beside
  it: wall, user and system CPU and maximum resident memory from
  `/usr/bin/time -l`; index size as `du -sk` of the LMDB folder and of
  the cache of prints, apart. Gunfinger's beside them: the peak store's
  size and `gunfinger stats` for the index; its index and query times
  from a run on the same machine where one exists. Times are rough
  and only differences of several times are read.

## 19:24: the launching agent's instruction (verbatim)

```text
Panako 2.1 compiles for Java 11 and its wrapper's Gradle 7.2 runs only on Java 16 or older, so Eclipse Temurin 11 is being installed (`brew install --cask temurin@11`). Do not change the wrapper, download another Gradle, or install anything yourself. Every few minutes check `/usr/libexec/java_home -v 11`; once it succeeds, build with `JAVA_HOME="$(/usr/libexec/java_home -v 11)" ./gradlew ...` (run Panako itself with the same Java 11 unless its README says otherwise) and record it in the brief. Until then, carry on with the Gunfinger side (checking the saved reports, the scoring script). If Java 11 has not appeared by 22:00, write the Panako part up as blocked, with what it needs, and wrap up.
```

- **The check.** At 19:25 `/usr/libexec/java_home -v 11` exits 0 and
  prints Temurin 21's home: it treats the version as a minimum when no
  11 is installed. So the wait checks the JDK's own version instead
  (`work/s24b/wait-java11.sh`: `java_home -v 11`, then that JDK's
  `java -version` must start with `11.`), every 2 minutes, until 22:00.
- **Runtime.** Panako runs with the same Java 11 (its startup script's
  options otherwise, as pre-registered). Times: a later start shortens
  step 4; the order of the pre-registration decides what runs first,
  and heavy runs still end by 00:30.

## Step 4, Gunfinger's side (19:16-19:31)

- **The saved reports match the current build.** `work/bin/s24-start/`
  in the main checkout (the code of `53d4136`, unchanged on this branch)
  run from this worktree with `--work work/s24b` (copies of the peak
  store, the clusters and the renders, with receipts written for the
  renders made before receipts existed, so nothing was rendered again):
  scans of `razorjack-2003-03-29` (15 s wall) and
  `nocny-trans-vertical-2002-08-23` (30 s), sweeps 2027 (113 s), 2028
  (152 s), 2029 (166 s) and the 12 mixes of seed 2026 (42 s), load
  10-37. Every report is identical to the saved one
  (`scripts/analysis/report_identity.py`); the mix renders are
  byte-identical to the main checkout's. Seed 2026's sweep and the
  stakka scans were checked by session 24's `regress` at 18:45.
- **Gunfinger's columns** (`scripts/analysis/panako_compare.py
  gunfinger`, `work/s24b/gunfinger-columns.json`): sweeps 2026-2029,
  60/60 at every speed, 0 wrong, speed error at most 0.026 points;
  stakka 11/11, razorjack 7/7, nocny 9/9 referenced at 262 tracks, 0
  wrong; generated mixes 106 of 115 indexed plays confident, 0 wrong.
- **Cost at 262 tracks.** `gunfinger stats`: 105,303 s of audio,
  peak store 19.7 MB (187 B/s), index 39.7 MB (7.84 M postings, 5.07
  B each). A fresh `gunfinger index` into `work/s24b/peaks-fresh`: 50.9
  s wall, 290 s user, 181 s system, 938 MB maximum resident, load about
  15. It extracted all 265 audio files: the 3 the working store keeps as
  skip notes (decoded short, "probably damaged", from an earlier run)
  decoded now. The harness's library stays the store's 262 records.

## Preparing Panako's run (19:30)

- **The library list**: the 262 records' sources
  (`scripts/analysis/store_records.py`), all present, covering every
  cluster member and every panel draw; per-seed lists
  (`panako_compare.py plan`): 209, 209, 207, 210 files, as the saved
  sweep reports' `indexed_assets`.
- **Panako's configuration file** is read from beside its jar
  (`Config.java`); when missing, Panako writes its built-in defaults
  there. The installed file (`resources/defaults/config.properties`,
  the pre-registered defaults) differs from the built-in values in
  `MONITOR_STEP_SIZE` (10 against 25), `PANAKO_MIN_MATCH_DURATION` (3
  against 5) and the decoder command; three keys exist only built in
  (`PANAKO_HIT_PART_DIVIDER` 5, `PANAKO_HIT_PART_MAX_SIZE` 250,
  `PANAKO_USE_GPU_EP_EXTRACTOR` false). The run copies the jar and that
  file, with the strategy and the paths changed, into `work/panako/run/`,
  as `./gradlew install` would lay out `~/.panako`.
- **Outside `work/`**: `Config` also opens a Java Preferences node,
  which macOS keeps in `~/Library/Preferences/com.apple.java.util.prefs.plist`
  (an existing file, from 2026-07-25, no Panako key). Its copy and
  digest are in `work/panako/state-before/`; it is compared after
  Panako's first run. The default logging configuration writes to
  `~/.panako/log`; the run does not use it (Java's default logging, to
  the run's log).
- `work/panako/run.sh` holds the stages: `build`, `test` (its unit tests,
  the self-check), `install`, `smoke` (three files, one of them seed
  2026's draw 000, and that draw's render at +3%), `index-all`,
  `index-seed N`, `sweep N`, `dev SET`, `dev-query SET`, `mix NN`.

## Appendix: the run scripts (in `work/`, not committed; kept here so they outlive the worktree)

`work/panako/run.sh`:

```bash
#!/bin/bash
# Session 24b: Panako's build and runs, one stage at a time, as pre-registered
# in docs/brief-24b.md (step 3). Not committed (work/).
# usage: work/panako/run.sh STAGE [ARG]
set -u
WT=/Users/razorjack/.t3/worktrees/gunfinger/s24b-panako
M=/Users/razorjack/Projects/OpenSource/gunfinger
P=$WT/work/panako
SRC=$P/src-clone/Panako
RUN=$P/run
DB=$P/db
LOGS=$P/logs
mkdir -p "$LOGS"

java11() {
  local home
  home=$(/usr/libexec/java_home -v 11 2>/dev/null) || return 1
  "$home/bin/java" -version 2>&1 | grep -q 'version "11\.' || return 1
  echo "$home"
}
J11=$(java11) || { echo "no Java 11"; exit 1; }

# One timed run at nice 10 under caffeinate, with the load average around it.
timed() {
  local name=$1; shift
  local log=$LOGS/$name.log
  echo "start $(date '+%F %T') load $(sysctl -n vm.loadavg)" > "$log"
  echo "args $*" >> "$log"
  caffeinate -i nice -n 10 /usr/bin/time -l "$@" > "$LOGS/$name.out" 2>> "$log"
  echo "exit $? end $(date '+%F %T') load $(sysctl -n vm.loadavg)" >> "$log"
}

# Panako as its startup script runs it (resources/defaults/panako), from RUN;
# an executable, so that /usr/bin/time can run it.
panako=$RUN/panako

paths_for() {
  echo "STRATEGY=PANAKO" "PANAKO_LMDB_FOLDER=$DB/$1" "PANAKO_CACHE_FOLDER=$DB/cache"
}

case $1 in
  build)
    cd "$SRC" && JAVA_HOME=$J11 GRADLE_USER_HOME=$P/gradle-home \
      timed build ./gradlew --no-daemon shadowJar ;;
  test)
    cd "$SRC" && JAVA_HOME=$J11 GRADLE_USER_HOME=$P/gradle-home \
      timed self-check ./gradlew --no-daemon test ;;
  install)
    # As `./gradlew install` lays out ~/.panako, but in RUN: the jar and the
    # documented default configuration, with the strategy and paths changed.
    mkdir -p "$RUN" "$DB"
    cp "$SRC/build/libs/panako-2.1-all.jar" "$RUN/panako.jar"
    sed -e "s|^STRATEGY=.*|STRATEGY=PANAKO|" \
        -e "s|^PANAKO_LMDB_FOLDER=.*|PANAKO_LMDB_FOLDER=$DB/all|" \
        -e "s|^PANAKO_CACHE_FOLDER=.*|PANAKO_CACHE_FOLDER=$DB/cache|" \
        -e "s|^OLAF_LMDB_FOLDER=.*|OLAF_LMDB_FOLDER=$DB/olaf_db|" \
        -e "s|^OLAF_CACHE_FOLDER=.*|OLAF_CACHE_FOLDER=$DB/olaf_cache|" \
        "$SRC/resources/defaults/config.properties" > "$RUN/config.properties"
    cat > "$RUN/panako" <<SCRIPT
#!/bin/bash
exec "$J11/bin/java" --add-opens=java.base/java.nio=ALL-UNNAMED --add-opens=java.base/sun.nio.ch=ALL-UNNAMED \\
  -server -Xms256m -Xmx\${PANAKO_HEAP:-256m} -jar "$RUN/panako.jar" "\$@"
SCRIPT
    chmod +x "$RUN/panako"
    cat "$RUN/panako"
    diff "$SRC/resources/defaults/config.properties" "$RUN/config.properties" ;;
  smoke)
    # Three library files, one of them seed 2026's draw 000; its render at +3%.
    head -n 3 "$WT/work/s24b/smoke-files.txt" > "$P/smoke-list.txt"
    timed smoke-store "$panako" store "$P/smoke-list.txt" $(paths_for smoke)
    timed smoke-query "$panako" query "$M/work/sweep/seed-2026/000-+3.mp3" $(paths_for smoke)
    timed smoke-stats "$panako" stats $(paths_for smoke) ;;
  index-all)
    sed "s|^|$M/corpus/library/|" "$WT/work/s24b/library.txt" > "$P/lists/index-all.txt"
    timed index-all "$panako" store "$P/lists/index-all.txt" $(paths_for all)
    timed stats-all "$panako" stats $(paths_for all) ;;
  index-seed)
    timed "index-seed-$2" "$panako" store "$P/lists/index-seed-$2.txt" $(paths_for "seed-$2") ;;
  sweep)
    timed "sweep-$2" "$panako" query "$M"/work/sweep/seed-"$2"/*.mp3 $(paths_for "seed-$2") ;;
  dev)
    # The development mix named by its directory, whole, with `monitor`.
    case $2 in stakka-skynet-knowledge|razorjack-2003-03-29|nocny-trans-vertical-2002-08-23) ;; *) exit 1 ;; esac
    audio=$(ls "$M/corpus/sets/$2"/*.{mp3,m4a} 2>/dev/null | head -n 1)
    timed "monitor-$2" "$panako" monitor "$audio" $(paths_for all) ;;
  dev-query)
    case $2 in stakka-skynet-knowledge|razorjack-2003-03-29|nocny-trans-vertical-2002-08-23) ;; *) exit 1 ;; esac
    audio=$(ls "$M/corpus/sets/$2"/*.{mp3,m4a} 2>/dev/null | head -n 1)
    timed "query-$2" "$panako" query "$audio" $(paths_for all) ;;
  mix)
    timed "mix-$2" "$panako" monitor "$M/work/mixes/seed-2026/mix-$2.mp3" $(paths_for seed-2026) ;;
  *)
    echo "unknown stage $1"; exit 1 ;;
esac
```

`work/s24b/wait-java11.sh`:

```bash
#!/bin/bash
# Waits until a JDK whose version is 11 is installed, or until the given HHMM.
# `java_home -v 11` alone also returns newer JDKs, so the version is read from the JDK itself.
deadline=$1
while true; do
  home=$(/usr/libexec/java_home -v 11 2>/dev/null)
  if [ -n "$home" ] && "$home/bin/java" -version 2>&1 | grep -q 'version "11\.'; then
    echo "java 11 at $home ($(date '+%T'))"; exit 0
  fi
  if [ "$(date +%H%M)" -ge "$deadline" ]; then
    echo "no java 11 by $deadline ($(date '+%T'))"; exit 1
  fi
  sleep 120
done
```

`work/s24b/run.sh` (the Gunfinger runs of step 4):

```bash
#!/bin/bash
# One harness run at nice 10 under caffeinate, timed, with the load average before and after.
# usage: work/s24b/run.sh <log name> <gunfinger-eval arguments...>
set -u
M=/Users/razorjack/Projects/OpenSource/gunfinger
B=$M/work/bin/s24-start/gunfinger-eval
W=/Users/razorjack/.t3/worktrees/gunfinger/s24b-panako/work/s24b
name=$1; shift
log=$W/logs/$name.log
{
  echo "start $(date '+%F %T') load $(sysctl -n vm.loadavg)"
  echo "args $*"
} > "$log"
caffeinate -i nice -n 10 /usr/bin/time -l "$B" --corpus "$M/corpus" --work "$W" --peaks-dir "$W/peaks" \
  --panels /Users/razorjack/.t3/worktrees/gunfinger/s24b-panako/docs/panels \
  --verdicts /Users/razorjack/.t3/worktrees/gunfinger/s24b-panako/docs/pair-verdicts.txt "$@" >> "$log" 2>&1
echo "exit $? end $(date '+%F %T') load $(sysctl -n vm.loadavg)" >> "$log"
```
