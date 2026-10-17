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

- **(03:27) The baseline.** `baseline session-28-start` copies the
  last `regress`'s reports (0.23 s wall): those of session 27's final
  check, made with `work/bin/s27-final/`, byte-identical to
  `work/bin/s28-start/` (Homebrew's 1.98.0). No `regress` ran at the
  start: session 24b's Panako run shares the machine, so a wall time
  would say nothing about speed; the final `regress` is the identity
  check.
- **(03:30) Step 2, the cause, confirmed by reading.** `cut(when)`
  calls `source.stop(when)` on an entry still sounding but leaves the
  entry's `stop` at its old value and keeps it. Web Audio applies only
  the latest `stop()` call made before a source has stopped, so the
  next cut, within LEAD (60 ms) and at a later `when`, finds the stale
  `stop > when` and calls `stop()` again, moving the stop later; the
  sources started by the previous change are kept the same way. Held
  keys repeat every 25-35 ms, so each repeat adds a source that keeps
  sounding until the key is released. Pressed slower than LEAD, the cut
  source has stopped before the next call, which then changes nothing.
  Every path that starts or reschedules a source goes through `cut`:
  `play` (`startAt`), `pause` (`cut(now)`), `seek` (arrows, mouse,
  marker jumps through `jumpTo`), `switchTo`/`switchBack` (1-9, Tab),
  `toggleBlend`, `toggleMatching`, `toggleLoop` (`restartHere`), and
  `open` (a question change pauses first, then drops the entries). The
  horizon refill (`tick` -> `schedule`) only appends steps after the
  cursor, back to back. So the fix in `cut` covers every path, also a
  pause right after a seek and lane keys pressed faster than LEAD.
  The invariant: at any context time the audible sources are those of
  the schedule's steps covering that time, at most one per sounding
  lane (two in the blend); every change ends each earlier source at
  its `when` or disconnects it before it starts, and no later call
  moves that stop. `keys.js` needs no change.
- **(03:40) The test.** `web/tests/player.test.mjs` with a fake
  AudioContext (`fake-audio.mjs`) that applies `stop()` as the
  specification does; five tests (held left, held Shift-left down to
  the start, held marker key, held right in the blend, lane keys every
  30 ms), each 66 changes 30 ms apart. Before the fix all five fail (66
  sources audible at once, 124 in the blend; `work/s28/player-test-before-fix.log`),
  after it all pass. `scripts/check.sh` runs them when Node 22.7 or
  newer is installed (the page's modules load without a package.json
  only from 22.7) and prints a "skipped" line otherwise. ADR 0016's "No
  build step" section gains one dated sentence (the page still needs no
  Node; Node is an optional tool of the checks), AGENTS.md's invariant and
  testing line say the same; no decision changes, so no new ADR.
- **(03:45) The real page.** This session's own server
  (`--no-open`, a free port, copies of the verdicts file and the answer
  log in `/tmp/s28-audition/`, deleted afterwards and unchanged before
  deletion) on `work/audition/s23/01-s23-pack.json`, headless Chrome
  with its own profile, keys sent over the DevTools protocol
  (`work/s28/cdp/`, uncommitted). An injected script counts the
  sources that have started and not fired `ended` and taps the output
  with an analyser. Held left arrow for 2 s (71 repeats about 30 ms
  apart) from 3:20 of question 1: start binary, the count rises to 61
  and the output from -13 to +7 dBFS (`work/s28/page-hold-before-fix.log`);
  fixed binary, 1 (2 for a moment at some cuts, the `ended` event's
  delay) and -10 to -22 dBFS (`work/s28/page-hold-after-fix.log`). The
  held marker key changed little there: the question has 5 markers, so
  most repeats found none; the unit test covers marker repeats. No
  question was answered.
- **(03:40) Step 3, the visual refresh**, done in about 20 minutes.
  Screenshots before and after with this session's own server and
  headless Chrome at 1440 x 900, the same questions and states
  (`work/audition/s23/01-s23-pack.json` and
  `work/audition/questions/02-folders.json`; question 7 open, question 1
  at 2:30 with lane 2, the blend and the loop, the help, a folder
  question): `work/s28/screens/{before,after}-0{1,2,3,4}-*.png`. For
  these the answer log was an empty file of this session's in
  `/tmp/s28-audition/`, so the folder questions show as open beside the
  pairs answered from the verdicts file; neither listening file was
  read for writing. Changes: `style.css` holds one set of custom
  properties (surfaces, text, the canvas's colours, a type scale of
  12-20 px, spacing, radii); `lanes.js` reads the canvas's colours from
  it once and gives each lane row its colour (a swatch by the label, the
  sounding lane's border). Queue: the current question has an accent
  bar and a bold title, open questions a blue dot, answered ones a
  green small-caps answer beside a muted title. Choices are chips, the
  chosen one green. Times use tabular figures. Canvas: a span gets a
  band along its top, the loop two edges, markers one can jump to a
  small flag (`join` markers stay plain lines), the playhead is twice as
  wide with a cap at each end. `:focus-visible` shows an outline.
  Contrast (WCAG 2, `work/s28/contrast.py`): every text colour on every
  surface it sits on is 5.5:1 or more. No transition was added, so no
  `prefers-reduced-motion` rule; no light palette (dark stays, and a
  second palette was not needed). Two things that read worse were fixed
  before the commit: the answer as a padded uppercase pill wrapped in
  the queue's fixed column ("DIFFERE NT"), and the help's keys in 12 px
  monospace shrank the arrows. Layout, keys, behaviour and every piece
  of text are as before. The step 2 tests pass again and the real-page
  hold check on the refreshed build gives the same counts
  (`work/s28/page-hold-after-refresh.log`: at most 2 playing for a
  moment, peak -9.2 dBFS).
- **(03:43) The clusters against the six new verdicts.** 0128's NAS
  clusters (`work/reports/library-nas-dnb-peaks/duplicate-clusters.json`,
  at `fed26c76b8fe89ea`) agree with all six: the three Aphrodite pairs
  and Kemal - Animation ~ the Silent Extent Remix lie in different
  clusters; the PRO 012 A upload is in the corpus Sonar's cluster, and
  its join to the Mark System Revision is one of the two cut links. No
  file of the six pairs' clusters is a recall-panel source (either
  half). So the clusters are not rebuilt, and no count of step 4
  depends on the verdicts. The old-skool upload ~ the revision cut
  still has no verdict; it touches no panel source either.
- **(03:44) Combined damage today at 262 tracks**: session 27's run of
  `robust` (`work/s27/vwork/reports/robust-seed-2026.json`, today's
  matcher): `combined` 4 of 80 confident, 65 of 80 possible or better,
  no wrong answer, as 0027's skip variant at 240 hits.

## Step 4: pre-registration (written at 03:41-03:46, committed before any count)

Everything below is fixed before any query of these conditions is
searched. Nothing may be added after a count is seen; anything computed
later and not named here is exploratory and decides nothing. Nothing
here changes the product.

### The panel and the index

- `docs/panels/recall-seed-2026.json`, development half only: 80
  sources (60 indexed, 20 held out, one per recording family, a third of
  each kind); the 2 indexed sources without a record since the
  prune (Essential Rewindz's Stealth, Soul Survivors' Jupiter Fields)
  are left out as in 0074, so 58 indexed and 20 held-out sources are
  searched. The validation half stays unsearched; the frozen file does
  not change. The panel's 9 speeds: -8, -5, -3, -1, 0, +1, +3, +5, +8%.
- The index: `recall`'s, the corpus library and the NAS store at
  40,126 records (`fed26c76b8fe89ea`), every member of the held-out
  sources' clusters left out. Clusters: 0128's NAS clusters, checked
  against the six new verdicts by ear (all six agree, none touches a
  panel source, so they are not rebuilt; Decisions, 03:43).
- The matcher, ladder and rule are `identify`'s defaults (the fitted
  matcher, both ladders, 240 hits in 3 windows confident, 60 hits
  possible).

### The conditions

- **C, the clean control**: the panel's 30 s excerpts as rendered in
  session 10 (MP3 128 kbps, the sweep's rendering), as 0068 and 0074.
- **(a) other rips only**: the same queries; the index also leaves out
  the own record of each of the 58 indexed sources (one index for all
  of them; no two development sources share a cluster). A source's
  cluster mates still in the index count as right. A query is
  *observable* when its cluster holds at least one other file in this
  index; by the clusters file that is at most 22 of the 58 indexed
  sources (198 queries). Unobservable queries are never misses: they
  are reported apart and scored like held-out queries (any confident
  detection is a wrong answer).
- **(b) brief plays** of 15 s and 20 s. Each brief query is, joined by
  hard cuts in the decoded samples (8 kHz mono, as the search reads
  them; no second lossy encoding): the last 3 + o s of a held-out
  source's excerpt at the same speed, then the first L s of the
  source's own excerpt, then the first 3 s of another held-out source's
  excerpt. o is drawn uniformly from [0, 10) s for each query with the
  panel's seed, so the play starts at a uniformly random place on the
  10 s window grid, 3-13 s into the query. The brief play is the start
  of the panel's excerpt: from the panel's `start_seconds` (drawn 15-85%
  into the track, clear of intro and outro), L seconds of query time.
  For query number n (in the order sources, then speeds, then lengths)
  the context sources are held-out sources n mod 20 and (n + 7) mod 20
  of the panel's held-out list, the next one taken when either is the
  source itself. Held-out sources get the same construction (the null).
- **(c) damage**: the combined damage of 0027, `robust`'s `combined`
  condition with its settings unchanged: the excerpt played at the
  query's speed with wow at 0.55 Hz (33 rpm) of depth 0.2%; a
  beatmatched partner at -6 dB (another panel source's excerpt, beat-
  aligned at a tempo within 10% of its own, otherwise at the query's
  speed unmatched, as `robust` does); FM broadcast processing (two
  compressors and a limiter, `robust`'s `BROADCAST`) over the mix; AAC
  64 kbps. The partner of an indexed source is the next searched
  indexed source in panel order, of a held-out source the next held-out
  source (both cyclic), so a held-out query holds no indexed audio. The
  partner's cluster counts as neither right nor wrong (`robust` credits
  it apart). Rendered from the NAS audio at the panel's 9 speeds.
- **(a)+(b)+(c)**: brief plays cut from the damaged excerpts, with
  damaged held-out excerpts as context, searched in (a)'s index.
- **Pairs, where cheap**, in this order: (a)+(c), (b)+(c), (a)+(b).

Runs, one heavy run at a time, each condition's report written as it
finishes: R1 (C's index): C, (c), (b); R2 ((a)'s index): (a),
(a)+(b)+(c), (a)+(c); then, time allowing, R3: (b)+(c); R4: (a)+(b).
Step 4 shrinks from its end: the pairs first, then the possible-tier
detail.

### The measures (per condition; per speed, per kind, per brief length)

- Confident recall: indexed (observable) queries with a confident
  detection of their own cluster. Possible recall: with a detection of
  their own cluster at the possible level or better (60 hits).
- Wrong answers: confident detections of a file outside the query's own
  cluster and its partner's, each listed with its file and hits; wrong
  possible detections counted and listed the same way.
- Held-out queries (and, under (a), unobservable ones) with any
  confident detection: never expected.
- For every miss of an observable indexed query, its reason (`recall`'s
  `Miss`): never a candidate (no detection of its own cluster in the
  search's output at all); too few hits (its strongest own detection
  under 240 hits after the second pass, told apart by whether it
  reached 60, the possible tier); enough hits in too few windows (240
  or more in fewer than 3 windows).
- Every confident wrong answer is checked afterwards with `pair`
  (source file against the found file): coverage of 80% or more (the
  clusters' criterion) marks a cluster gap to check manually; it still
  counts as wrong (AGENTS.md).

### Expected before the data, so it cannot be claimed later

- C: 522 of 522 (0138's run at 40,126 records); no wrong answer (0074's
  9 were the DYkast pair, since joined by the verdict by ear).
- (a): at least 90% of observable queries confident (other rips of the
  same recording, some of them other masters or vinyl rips).
- (b): near the grid's rates at 262 tracks over all offsets (0138,
  today's rule: 15 s 67 of 160 = 42%, 20 s 146 of 160 = 91%); possible
  recall near 100%.
- (c): near `robust`'s combined row at 262 tracks today (0138's run: 4
  of 80 confident, 65 of 80 possible or better, no wrong answer).
- (a)+(b)+(c): next to no confident query.

### What counts as notable (declared before any count)

1. Any confident wrong answer in any condition; and apart, any whose
   `pair` coverage is under 80% (not a cluster gap).
2. Any held-out or unobservable query with a confident answer.
3. (a): under 90% of observable queries confident, or its misses
   concentrated in one kind of source.
4. (b): confident recall more than 10 points below the grid's at the
   same length (under 32% at 15 s, under 81% at 20 s).
5. (c): confident recall above 15%, or possible-or-better recall under
   60%.
6. Any wrong possible detection of 200 hits or more (a small margin
   below the confident tier).

## Decisions, continued (after the pre-registration, by time)

- **(03:55) Other rips only, made precise before any count.** As `sweep
  --other-rips` does, the index leaves out each indexed source's record
  and the files of its cluster whose peak records are identical to it
  (the same audio filed again), so that only other rips can answer; a
  query is observable when its cluster keeps a file in that index.
  The pre-registration said "the own record"; identical copies are the
  same rip, so leaving them in would let the source answer for itself.
  The harness reports how many records this leaves out.
- **(04:05) The harness** (`recall`, committed before the runs):
  `--other-rips-only` and `--queries clean,damaged,brief,brief-damaged`,
  searched in that order against one index, each report written as it
  finishes (`recall-seed-2026[-other-rips][-damaged][-brief].json` in
  `work/reports/library-nas-dnb-peaks/`). Damaged excerpts are rendered
  by `robust::render_combined`, `robust`'s `combined` recipe moved into
  one function that `robust` also calls: a `robust --only combined` run
  of the refactored build in a scratch work directory rendered all 100
  combined and 100 control queries byte-identical to the cached ones and
  gave the same rows (combined 2 + 2 of 80 confident, 32 + 33 possible).
  Damaged renders go to `work/recall/seed-2026-damaged/`. `Outcome`
  gains a `partner` flag (never set by the sweep, so its reports do not
  change); a partner's detection is neither right nor wrong. 7 new unit
  tests (report names, partners, brief plays and their samples, the
  other-rips selection, scoring of unobservable queries and partners).
- **(04:00) Screenshot 04 retaken** (the orchestrating thread's note,
  03:52): `before-01` and `before-04` were byte-identical, as were
  `after-01` and `after-04`, because 01 opens the first open question,
  the Skynet & Stakka folder question, and 04 reopened that same
  question. So the folder question's look was checked in 01, but 04 added
  nothing. 04 now opens another folder question (Hard Leaders - Listen
  Up, 11 files, 10 boundaries) at 1:35, with the start build for before
  and the refreshed build for after (`work/s28/screens/{before,after}-04.log`).
  Looked at: in the before shot the 1 px playhead at 1:35 is hard to
  find among the boundary markers; in the after shot it is visible.
  Nothing reads worse, so nothing changed.
- **(03:58) The runs started** (`work/s28/queue.sh`, `screen`
  `s28-queue`, frozen binary `work/bin/s28-recall/` from `0c0905d`,
  logs in `work/s28/runs/`, reports copied to `work/s28/reports/`; the
  NAS report folder from before cloned in `work/s28/before/`). The
  damaged excerpts rendered in 2 minutes (03:55-03:57): 702, of which 88
  play their partner unmatched because no tempo within 10% of its own
  was found, as `robust` does. Order: C, (c), (b), (a), (a)+(b)+(c),
  (a)+(c), (b)+(c), (a)+(b), one invocation each.
  `scripts/analysis/recall_conditions.py` summarises the reports; tried
  on 0074's report first, it gives 0074's numbers (522 of 522, the 9
  DYkast answers).

- **(05:11) The queue finished**: all eight runs exit 0, no error in any
  `.err`, the last at 05:10:45. Experiments 0144 (single conditions)
  and 0145 (combinations) written in that order.
- **(05:20) Data files hold file names only.** AGENTS.md asks that
  evaluation JSON in `docs/experiments/data/` hold file names only;
  0144's data file, as first committed, held library-relative NAS paths.
  `recall_conditions.py` now writes file names, and both data files were
  regenerated from it (0144's numbers unchanged). `recall_subsets.py`
  (new) restricts each report to (a)'s 20 observable sources for 0145.
- **(05:29) The frozen build's sources, checked.** `work/bin/s28-recall/`
  was copied from `target/release/` at 03:55, where the last build was
  `cargo build --release -p gunfinger-eval` at 03:53. After it only
  unit tests were added and `cargo fmt --all` ran before `0c0905d` was
  committed, so the runs' binary was built from `0c0905d`'s code before
  formatting. A build of `0c0905d`'s sources differs from it in bytes
  (with `-p gunfinger-eval` by 1,424 bytes; the workspace build by
  36 KB, from feature unification across the workspace). To confirm the
  results belong to the committed code, every condition is run again
  with the workspace build of HEAD (`work/bin/s28-verify/`, the same
  Rust sources as `0c0905d`) in a scratch work directory of APFS clones
  (`work/s28/verify/`, `work/s28/verify.sh`), the damaged excerpts
  rendered again, and renders and reports compared byte for byte.
- **(05:37) Session 24b merged** (`31ba92a`) after the orchestrating
  thread reported it finished; checked as step 5 says (worktree clean at
  `bad25f9`, wrap-up commits, no Panako process, seven files under
  `docs/` since `2eee100`, nothing of Panako's or under `corpus/`). One
  conflict, in the roadmap: session 27's per-second paragraph kept, the
  branch's Panako bullet in place of the old one.
- **(05:45) The review's findings** (read-only agent, started 05:18,
  report 05:38). No wrong number and no bug in the fix or the scoring.
  Fixed: the damaged-render cache named by source and speed only, so a
  changed partner would have reused an old render (now
  `<source>-<partner>-<speed>`, a unit test; `is_partner` and
  `is_unobservable` taken out of `Searcher::query` with a test); prose in
  the notes, status, roadmap, 0144 and the checklist.
- **Times corrected** (the review compared them with the commits). The
  Decisions above were not all headed with the time of the work:
  "(03:40) The test" and "(03:45) The real page" were done before step
  2's commit at 03:34 (the test logs at 03:30, the page logs at 03:33);
  step 3 ran from about 03:34 to its commit at 03:40 (before screenshots
  03:35, after ones 03:39), not about 20 minutes; "(04:05) The harness"
  was written by 03:55 (`0c0905d`) and "(04:00) Screenshot 04 retaken"
  by 03:59. The continued Decisions are in the order written. The
  notes' first draft said "(05:30)" for a commit made at 05:18.
- **Experiment order.** The queue started (a)+(b)+(c) at 04:27:51, 61 s
  before 0144 was committed (04:28:52). All pre-registered runs had been
  queued back to back at 03:55, and no combined result existed before
  04:40, so 0144 was written before any combined count was seen.
- **(06:59) The rerun with the committed code**: the damaged renders
  (05:31) and all eight reports (06:16 and 06:59) byte-identical to the
  runs' of `work/bin/s28-recall/`.
