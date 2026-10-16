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
