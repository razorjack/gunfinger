# Session 28 brief: the listening page's held-arrow bug, a light visual refresh, recall under the hardest conditions combined

Written by the agent for itself on 2026-10-17, so the plan survives
context compaction. Work autonomously and never stop to ask. When
something is ambiguous, take the conservative option, write the decision
down and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-28-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 23" to the end; the handoffs of
   sessions 24-27).
4. `docs/status.md` (newest entries at the bottom).
5. ADRs 0016, 0017 and 0020, `docs/audition-questions.md`, every file in
   `crates/gunfinger-eval/src/audition/web/`.
6. Experiments 0027, 0068, 0074, 0128, 0136-0142 and this session's
   (0143 and up; 0150 is reserved for session 24b); the roadmap's
   "Recall beyond the corpus recordings".
7. The code: `crates/gunfinger-eval/src/recall.rs`,
   `crates/gunfinger-eval/src/robust.rs`, and this session's scripts.

`docs/brief-2.md` to `docs/brief-27.md` and `docs/brief-24b.md` hold
rules that still apply unless this brief changes them (among them: edit
no configuration file; test fixtures are generated, never copied from
the corpus or the NAS; heavy runs one at a time; NAS-scale harness runs
overwrite `work/reports/library-nas-dnb-peaks/`, so copy it first).

## The prompt, verbatim

```text
Start session 28 of Gunfinger, an overnight session: fix a hearing-safety bug in the listening page, give the page a light visual refresh, then measure recall on the NAS recall panel's development half under the hardest conditions combined, with failure reasons. If session 24b has finished, merge its branch. It runs unattended from about 03:30 with a planned end at about 10:45. The end is soft: finish a run or review in progress rather than kill it, but plan to end by then. The results are due at 11:00. The NAS is mounted at /Volumes/atlas. Work autonomously and never stop to ask. When something is ambiguous, take the conservative option, write the decision down and continue.

Context.
- The six questions of work/audition/s23/ have been answered by ear (answer log records 55-60). The six verdict lines and a status entry with the comments were committed by the orchestrating thread before this session started (`c0b3172`; check `git log -3`). The comments are verbatim in work/audition/s23/listening-comments-2026-10-17.md. In short: the hifi-junglist PRO 012 A Sonar upload is the same recording as the corpus Sonar; that upload ~ the Mark System Revision is different (the cut `--cut-sparsest` made there agrees with the verdict by ear; 0128's other Sonar cut, the old-skool-jungle-dnb upload ~ the Mark System Revision, still has no verdict); the three Aphrodite pairs are different, sharing most material (they stay apart); Kemal - Animation ~ the Silent Extent Remix is different, sharing only the intro. Record in the notes what the verdicts imply; the frozen panel files do not change.
- A bug was reported in the listening page (`gunfinger-eval audition`, ADR 0016): "when I play the music and then hold left arrow, it plays first fragment, then shifts to second and it plays along with the first one, then third with first and second... and then accumulate in volume until it's so loud that it's painful. This only happens when i hold the right/left arrow. when i press it repeatedly, it's fine". The orchestrating thread's diagnosis, to confirm before fixing: in crates/gunfinger-eval/src/audition/web/player.js, `cut(when)` calls `source.stop(when)` for a source still sounding but leaves that entry's stored `stop` at its old value and keeps the entry; Web Audio applies only the latest `stop()` call on a source, so a later `cut()` at a later `when` (held keys repeat every ~30 ms, faster than LEAD = 0.06 s) moves the stop later again, and each seek adds a source that keeps sounding. keys.js has REPEATING = {"seek", "marker"}, so holding a marker key may do the same.
- A slightly better look for the page was also requested: "The web part looks absolutely fine but ... could it be slightly stepped up? let's prioritize functionality, as it's literally a quick eval tool, but slight improvement (without overdoing it) in visual look would be welcome".
- The roadmap's "Recall beyond the corpus recordings": experiment 0068 measured the recall panel's development half at NAS scale (540/540 at every speed and kind); 0074 at 33,596 records. Still to do there: "the hardest conditions combined": other rips only, brief plays and damage together (combined damage at 262 tracks under skip at 240: 4 of 80 confident, 65 possible or better, 0027), and the harness saying why a query failed (never a candidate, too few hits in the second pass, or enough hits in too few windows). The validation half stays unsearched.
- Session 27 (experiments 0136 to 0142) measured the minimum span and supported time in place of 3 windows, and per-second scoring; read its notes section and its "Handoff for the next session". Its release builds used Homebrew's 1.98.0 on the default PATH (rustup's gives different bytes from the same sources); build the start and final binaries the same way so they compare, and run scripts/check.sh with rustup's. Session 24b (branch s24b-panako, worktree /Users/razorjack/.t3/worktrees/gunfinger/s24b-panako) is running Panako as an outside benchmark at nice 10 until about 06:30-08:00; its results go to experiment 0150, which is reserved for it. This session numbers its experiments from 0143, below 0150. Session 27's worktree `s27-span` and its branch are kept, never merged; leave them alone.
- State at the start: check git (clean at `c0b3172`); the NAS store at 40,126 records, revision fed26c76b8fe89ea (record it; if it changed, record the change and carry on).

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it), docs/brief-27.md and docs/session-27-checklist.md (brief-2 to brief-26 and brief-24b hold rules that still apply unless this prompt changes them), docs/session-notes.md ("Session 23" to the end), docs/status.md (newest entries at the bottom), ADRs 0016, 0017 and 0020, docs/audition-questions.md, experiments 0027, 0068, 0074, 0128 and 0136-0142, the roadmap's "Recall beyond the corpus recordings", and every file in crates/gunfinger-eval/src/audition/web/.
2. Write docs/brief-28.md (this prompt verbatim, the state at the start, the plan with times) and docs/session-28-checklist.md. Point CLAUDE.md's reading list at them; keep brief-27 among the older briefs. Commit.
3. Build with --release and copy the binaries to work/bin/s28-start/. Run `gunfinger-eval baseline session-28-start` (record its wall time).

Steps:

1. Setup (until about 03:50): the steps above.

2. The held-arrow bug (about 45 min, first; it can hurt the listener's ears).
   - Confirm the cause by reading player.js and keys.js, and find every path that starts or reschedules a source (seek, marker jumps, play and pause, A and B switching, question change, the horizon refill). State the invariant the player must hold: at most the sources the current schedule intends sound at any time, so held keys at any repeat rate never stack audio.
   - Fix it minimally in player.js (and keys.js if needed). When a source is cut, its entry must record the new stop time, or be dropped and never rescheduled later.
   - A test that fails before the fix and passes after: seeks and marker jumps repeated faster than LEAD (for example every 30 ms for 2 s) leave at most the intended number of sources sounding, and the last scheduled stop of every replaced source is no later than the cut. The page's JS has no tests today. Node is on PATH (v24, via mise); use `node --test` with a small fake AudioContext and no npm packages. scripts/check.sh runs it when `node` is on PATH and prints a "skipped" line otherwise, as it does for the other target's clippy. Say in the notes that this adds Node as an optional tool of the checks. If that needs a decision beyond ADR 0016, write the smallest note the project's rules require (a new ADR only if a decision changes).
   - Check it in the real page too: run your own server on a free port with `--no-open`, with copies of docs/pair-verdicts.txt and work/audition/answers.jsonl in /tmp passed as the verdicts file and answer log, so the listening files cannot change. Drive headless Chrome (/Applications/Google Chrome.app, `--autoplay-policy=no-user-gesture-required`) to hold the arrow keys and count sounding sources, if that can be done in reasonable time; otherwise say the unit test is the only check. Never answer a question there; the copies are deleted afterwards.
   - Commit.

3. A light visual refresh of the listening page (about 1 h 30 min at most; stop there).
   - Functionality first: the layout, every key, every behaviour and every piece of information stay as they are; nothing that looks nicer but works worse. No new libraries, fonts from the network, build step or framework; the page stays offline and self-contained.
   - In scope: one coherent set of CSS custom properties (colour, spacing, radius); typography (a clear size scale, tabular numbers for times); spacing and alignment; contrast at least WCAG AA for text; the lanes, markers, spans and playhead; a clearer current question in the queue and a clearer state for answered questions; visible keyboard focus; `prefers-reduced-motion` respected if you add any transition. Dark stays the default look. A light palette under `prefers-color-scheme: light` only if it is just a second set of the same properties and both pass the contrast check.
   - Screenshots before and after with headless Chrome against your own server (as in step 2), the same questions and window size, saved under work/s28/screens/ with their paths listed in the notes. Look at them yourself before committing and fix what reads worse.
   - Run the step 2 test again. Commit.

4. Recall under the hardest conditions combined (about 3 h 30 min, from about 05:30).
   - Pre-registration, written into the brief and an experiment file before any count, and committed: the panel is docs/panels/recall-seed-2026.json's development half only (the validation half stays unsearched; the frozen file does not change). The conditions: (a) other rips only: each source's own record left out of the index, its cluster's other rips counted as correct, sources with no other rip in the index reported as unobservable for this condition, never as misses; (b) brief plays: the lengths you declare (at least 15 s and 20 s; say where in the source the excerpts start); (c) damage: the combined damage of 0027 (say exactly which `robust` conditions and settings); then (a), (b) and (c) together, each pair of them where cheap, and the clean control. Speeds: the panel's. The measures: confident and possible recall, wrong answers (confident and possible, with which records), held-out sources never confident, and for every miss its reason (never a candidate, too few hits in the second pass, enough hits in too few windows). Declare what would count as a notable result before seeing any.
   - Clusters: check the current NAS clusters (0128, at fed26c76b8fe89ea) against the six new verdicts. Rebuild them only if a disagreement touches a panel source and the time allows; otherwise say which counts it could affect.
   - Harness changes in gunfinger-eval as needed (options on `recall`, or reuse of `robust`'s damage code), with unit tests; the product (crates/gunfinger, gunfinger-core) does not change. Commit the code before the runs.
   - The runs, at NAS scale, one heavy run at a time, detached under `screen`. Experiment file(s) of at most 40 lines each, written before the next experiment starts. Commit.
   - Shrinks from the end if time runs short: the pairwise combinations first, then the possible-tier detail.

5. Session 24b's merge (about 30 min, when it is ready; check from about 08:30). 24b is ready when its thread has finished, its branch s24b-panako has its wrap-up commit (its brief, notes, status and roadmap updated, experiment 0150 written) and no process of its runs is left. Then merge s24b-panako into master with a merge commit, as session 26 did at 2eee100: first check that its diff since the last merge touches only docs, scripts/analysis and similar, with nothing of Panako's (no jar, source, LMDB, database or Panako output) and nothing under corpus/; notes, status and roadmap keep every session's sections whole, in time order; CLAUDE.md lists docs/brief-24b.md among the older briefs. If 24b is not ready by about 10:00, leave it unmerged and say so in the notes; the orchestrating thread will merge it later. Never edit the branch's files in its worktree, and never touch its work/ folder.

6. A read-only review by a subagent, starting by about 09:30 (after step 4's code and runs, before or beside step 5); fix what it finds.

7. Wrap-up (from about 10:00): a "Session 28" section at the end of docs/session-notes.md (the bug, its cause and fix and how it was checked; the visual changes, with the screenshot paths; the recall results in plain terms, what the hardest conditions cost and why queries failed; what the six verdicts imply; whether 24b was merged; the handoff), docs/status.md, the checklist, the roadmap's bullet. `regress session-28-start` identical with the final build; scripts/check.sh green with rustup's 1.98.0 first on PATH and RUSTUP_TOOLCHAIN=1.98.0 (never install a toolchain); the NAS store's revision. Leave target/release/gunfinger-eval built from the final commit, so the next `audition` run has the fix. Commit. Last, print what was done, the checks, and anything left.

Rules for this session:
- Planned end about 10:45, soft. Steps shrink from the end of step 4 (see there); steps 2, 5 and 7 do not shrink. An unfinished step is reverted, or committed only in a working, tested state, and named in the notes.
- The product does not change: crates/gunfinger and gunfinger-core behave as at the start; `regress session-28-start` identical; scripts/check.sh green.
- Long runs: the Bash tool kills a background task at its timeout, 30 minutes by default; session 24b lost a run this way tonight. Every run that may last more than about 20 minutes starts detached under `screen` with its own log and an exit line, or gets an explicit timeout of up to 7,200,000 ms; a run longer than 2 h is split. After each run, check that its log ends with its exit line.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or reports (work/reports/evaluation-*-scan-ed-rush-optical-essential-mix.json included), rescore nothing, and never run test evaluation 5. Never identify, decode or index EM_1999_02_07_-_Ed_Rush_Optical.mp3 at the NAS root. Run nothing on the candidate mixes of experiment 0065 (including the mixed CDs it lists and the NAS's sety/vertical.mp3).
- Unsearched: the recall panel's validation half and both shared panels' validation halves. The frozen panel files do not change.
- The five test-derived versions are never used to tune anything: Terminal 2, the Orders remixes, the Marky & Bungle Funktion, the Clear Skyz Break remix and 1996 VIP, and the Black Barrel Beachball.
- The NAS (/Volumes/atlas/Music/dnb) is read, never written (its .gunfingerignore included). The NAS peak store (~/.local/share/gunfinger/nas-dnb-peaks) is read-only: no `index`, `prune` or `--retry-skipped`. Cluster rebuilds write only under work/.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt and work/audition/answers.jsonl do not change (the copies in /tmp are yours and are deleted). Manifests do not change, including corpus/sets/nocny-trans-vertical-2002-08-23/tracklist.toml.
- AGENTS.md: no SIMD, no `unsafe`, optimise only from measurement, an experiment file of at most 40 lines written before the next experiment starts, evaluation JSON in docs holds file names only, a new ADR to change a decision, cross-platform code (ADR 0020), no filename- or tag-derived logic in the product.
- Out of scope (ADR 0017): report edits, review workflows, and features that fix tags or the library.
- corpus/ is private and never committed; nothing of Panako's is committed, and the s24b-panako worktree's files are left alone (step 5 merges the branch only).
- Leave alone every process this session did not start, including session 24b's Panako runs (java, ffmpeg). Session 24b's runs share the machine until about 06:30-08:00, so this session's times say nothing about speed; measure none.
- NAS-scale harness runs overwrite work/reports/library-nas-dnb-peaks/, so copy what you need first. Keep scratch data under a few GB.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step. Check `git status` before every commit and never use `git add -A`. Do not push. No AI attribution in commits.
```

## State at the start of session 28

- 03:23. `git status` clean at `c0b3172` on `master` (the six new
  verdict lines and the status entry, committed by the orchestrating
  thread). Experiments 0001-0142; this session numbers 0143-0149, and
  0150 is reserved for session 24b. ADRs 0001-0020. Test set: 4 of 5
  evaluations used (none in this session).
- The NAS peak store at 03:26: 40,126 records, revision
  `fed26c76b8fe89ea` (`work/s28/store-revision-start.txt`), as the
  prompt says. The NAS is mounted at `/Volumes/atlas`.
- Listening files: `docs/pair-verdicts.txt` 82 lines, sha256
  `ad74a881ba9f...`; `work/audition/answers.jsonl` 60 records, sha256
  `1e1a98a8158d...` (`work/s28/listening-digests-start.txt`). Both must be
  the same at the end.
- Processes this session did not start are left alone, including session 24b's
  Panako run (`run.sh stakka-out`, java at nice 10, from 03:25). No `audition` server runs.
  - Disk: 31 GiB free.
- Toolchains: Homebrew's cargo 1.98.0 first on PATH (release builds);
  rustup's 1.98.0 for `scripts/check.sh`; Node v24.18.0 (mise shim,
  also `/opt/homebrew/bin/node`).

## Plan, with times

Start 03:23; planned end about 10:45, soft.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1 | setup: brief, checklist, CLAUDE.md, build, `work/bin/s28-start/`, `baseline` | 0:30 | 03:55 |
| 2 | the held-arrow bug: cause, invariant, fix, `node --test` with a fake AudioContext, `check.sh` hook, the real page in headless Chrome | 0:45 | 04:40 |
| 3 | the visual refresh: screenshots before, CSS properties, type scale, contrast, queue states, focus, lanes' colours, screenshots after, test again | 1:20 (stop by 06:05) | 06:00 |
| 4a | pre-registration (brief and experiment 0143), the clusters against the six verdicts; committed before any count | 0:30 | 06:30 |
| 4b | harness: `recall` options for other rips only, brief lengths, the combined damage, miss reasons; unit tests; committed | 1:00 | 07:30 |
| 4c | runs at NAS scale under `screen`, one at a time; experiment files | 1:45 | 09:15 |
| 5 | session 24b's merge, from about 08:30, when ready (by 10:00 at the latest) | 0:30 | (beside 4c) |
| 6 | read-only review by a subagent, by 09:30; fixes | 0:30 | 10:00 |
| 7 | wrap-up: notes, status, checklist, roadmap, `regress`, `check.sh`, store revision, final build left in `target/release/` | 0:45 | 10:45 |

Step 4 shrinks from its end: the pairwise combinations first, then the
possible-tier detail. Steps 2, 5 and 7 do not shrink.

## Decisions

(by time, as they are made)
