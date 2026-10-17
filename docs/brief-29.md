# Session 29 brief: the minimum span's fair test, brief passages at every offset of the window grid

Written by the agent for itself on 2026-10-17, so the plan survives
context compaction. Work autonomously and never stop to ask. When
something is ambiguous, take the conservative option, write the decision
down and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-29-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 27" to the end; the handoffs of
   sessions 27 and 28).
4. `docs/status.md` (newest entries at the bottom).
5. ADRs 0001, 0004, 0006, 0008 and 0013; experiments 0020, 0047,
   0136-0145, 0150 and this session's (0146 and up); the roadmap's "A
   minimum span instead of 3 windows";
   `crates/gunfinger-core/src/confidence.rs`.
6. The variant worktree `s29-span`
   (`/Users/razorjack/.t3/worktrees/gunfinger/s29-span`, never merged)
   and this session's scripts (`scripts/analysis/span_offsets.py`,
   `work/s29/*.sh`).

`docs/brief-2.md` to `docs/brief-28.md` and `docs/brief-24b.md` hold
rules that still apply unless this brief changes them (among them: edit
no configuration file; test fixtures are generated, never copied from
the corpus or the NAS; heavy runs one at a time; NAS-scale harness runs
overwrite `work/reports/library-nas-dnb-peaks/`, so copy it first).

## The prompt, verbatim

```text
Start session 29 of Gunfinger, a morning session: give a fair, decision-ready answer on the roadmap's "A minimum span instead of 3 windows". Session 27 found that the variants' only new wrong answers come from queries whose position on the window grid protects today's rule. This session counts every rule on brief passages placed at every position of the grid, and on tonight's brief plays at NAS scale. It runs unattended from about 06:10 with a planned end at about 10:45. The end is soft: finish a run or review in progress rather than kill it, but plan to end by then. The results are due at about 11:00. The NAS is mounted at /Volumes/atlas. Work autonomously and never stop to ask. When something is ambiguous, take the conservative option, write the decision down and continue.

Context.
- Session 27 (experiments 0136-0142, notes "Session 27", its handoff item 2): variants A(S) (first to last aligned hit at least S s apart) and B(S) (at least S whole seconds holding an aligned hit), S = 10, 15, 20, each replacing only "in 3 windows" of today's rule (240 hits in 3 windows; ADRs 0001, 0008, 0013). By the criteria written first, none is worth proposing: on the grid (0047's positions) A(10), B(10) and B(15) make 803, 947 and 800 of 960 brief queries confident against 693, but all variants except A(20) name a related version confidently in three 20 s shared-material queries (fire-2, fire-6, pyro-2) of the first shared panel, with the played recording left out. 0142: with 3, 5 or 7 s of digital silence before the same passages, today's rule names the same wrong versions in 18 of 18 queries and one more (pyro-1 after 5 s); A(15) and B(15) were wrong exactly as often as today's rule on those queries, A(10) and B(10) once more. Its conclusion: "A fair test of a span rule counts brief passages at several offsets." The variant code is on branch `s27-span` (worktree /Users/razorjack/.t3/worktrees/gunfinger/s27-span, tip `812342d`, never merged; env flags `S27_SHIFTED_ONLY`, `S27_SILENCE_ONLY`; binaries in work/bin/s27-span*/).
- Session 28 (experiments 0143-0145 and up, notes "Session 28" and its handoff) fixed the listening page, refreshed its look, merged session 24b's Panako branch, and measured recall on the NAS recall panel's development half under hard conditions. Brief plays (b) at NAS scale: 15 s plays 228 of 522 confident and 20 s plays 493 of 522, set by where the play starts in a 10 s window (15 s from 0-5 s into a window: 0 of 259 confident). `recall` now has `--other-rips-only` and `--queries` (clean, damaged, brief, brief-damaged).
- Session 24b (experiment 0150, Panako 2.1 at 262 tracks): Panako confidently names 7 generated-mix plays that Gunfinger reports only as possible (0150 lists them).
- Session 27 left gaps in `seconds`'s unit tests: building the truth from manifests and from generated mixes, and a border margin next to a track that is not in the library.
- State at the start: check git once session 28 has finished (clean on master after its last commit; record the hash); the NAS store at 40,126 records, revision fed26c76b8fe89ea (record it; if it changed, record the change and carry on).

Sharing the checkout with session 28. Session 28 is finishing in this same root checkout: its reproduction runs (`work/bin/s28-verify/`, `work/s28/verify/`) and its last notes update run until about 06:50. Until the orchestrating thread sends you a message saying session 28 has finished: read (First, item 1), draft the brief and the checklist under work/s29/ (ignored by git; move them to docs/ once session 28 has finished), and do step 2 in the s29-span worktree, which lies outside the root checkout and has its own target directory. Do not run `git add`, `git commit` or `git stash` in the root checkout. Do not edit any tracked file there. Do not run `baseline`, a build in the root checkout, or any harness run that writes the root's work/reports/. Never touch session 28's processes or its work/s28/ and work/bin/s28-* folders. If no message has arrived by 07:15, check yourself: session 28 has finished when the last root commit is its wrap-up, `git status` is clean, and no process runs from work/bin/s28-*. Then go on.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it), docs/brief-28.md and docs/session-28-checklist.md (brief-2 to brief-27 and brief-24b hold rules that still apply unless this prompt changes them), docs/session-notes.md ("Session 27" to the end), docs/status.md (newest entries at the bottom), ADRs 0001, 0004, 0006, 0008 and 0013, experiments 0020, 0047, 0136-0145 (and any later ones of session 28) and 0150, the roadmap's "A minimum span instead of 3 windows", crates/gunfinger-core/src/confidence.rs, and `git log master..s27-span` with its diff.
2. Write docs/brief-29.md (this prompt verbatim, the state at the start, the plan with times) and docs/session-29-checklist.md, drafted under work/s29/ until session 28 has finished. Then point CLAUDE.md's reading list at them; keep brief-28 among the older briefs.
3. After session 28 has finished: commit the brief and checklist, then build with --release the way sessions 27 and 28 did (Homebrew's 1.98.0 on the default PATH) and copy the binaries to work/bin/s29-start/. Run `gunfinger-eval baseline session-29-start` (record its wall time).

Steps:

1. Setup (about 20 min, overlapping session 28's end): the steps above.

2. The variant build (about 30 min): a new worktree `s29-span` at /Users/razorjack/.t3/worktrees/gunfinger/s29-span from master's current commit, with `s27-span` merged into it so the variants run with session 28's harness (never merged into master; leave `s27-span` itself unchanged). Add what the fair test needs (for example, a silence lead-in at any offset you declare, beside `S27_SILENCE_ONLY`), with unit tests. Its work directory is its own, never work/reports/. Check first that today's rule reproduces master's levels exactly on one panel.

3. The fair test on the shared panels (about 1 h 30 min). Pre-register first, in the brief and an experiment file, and commit before any count:
   - The queries: every passage of both shared panels' development halves of 30 s or less (the scenarios 0140-0142 used, and any longer ones up to 30 s), each after digital silence at offsets covering the whole 10 s window period in steps of 1 s or less (state them), every scenario and index as in 0142. The validation halves stay unsearched; the frozen panel files do not change.
   - The rules: today's, A(10), A(15), A(20), B(10), B(15), B(20).
   - The fair form of criterion 1, written before any count. Suggested: summed over all offsets, a variant names no more wrong files confidently than today's rule, and no passage is wrong under the variant that is never wrong under today's rule at any offset. A wrong possible answer becoming confident counts. Keep brief 27's criterion 1 as it was written, and report it beside the fair form without replacing it.
   - The measures: wrong confident answers per rule and offset; for each passage, the share of offsets at which today's rule is wrong (its dependence on the grid); right confident answers per rule; level changes.
   - The runs, then an experiment file of at most 40 lines.

4. Brief plays at NAS scale under the variants (about 45 min): session 28's (b) and (a)+(b) queries of the recall panel's development half on the variant build (or offline from saved detections, if they carry what the rules need; check by reproducing today's levels first). Count per rule: confident recall at 15 and 20 s, by where the play starts in a window, wrong answers confident and possible, and held-out sources confident. Also check offline whether 0150's 7 Panako-only plays become confident under each rule. Experiment file.

5. `seconds`'s unit-test gaps (about 30 min, can overlap step 3's or 4's runs): truth from manifests and from generated mixes, and a margin next to a track that is not in the library. Tests only; behaviour changes only where a test exposes a bug (then say so). Commit.

6. A read-only review by a subagent, starting by about 09:45; fix what it finds.

7. Wrap-up (from about 10:10): a "Session 29" section at the end of docs/session-notes.md, written for a decision. Say in plain terms what each variant would change: which brief plays it would name, the shortest identification, every new wrong answer under the fair test and under brief 27's criterion as written, and which variant, if any, the evidence favours and why. Adopting one is our decision and would need a new ADR replacing the window condition of ADRs 0001 and 0008; this session writes none. Also update docs/status.md, the checklist and the roadmap's bullet. Then run `regress session-29-start` (identical) with the final build, scripts/check.sh with rustup's 1.98.0 first on PATH and RUSTUP_TOOLCHAIN=1.98.0 (green; never install a toolchain; it also runs the listening page's `node --test` since session 28), and record the NAS store's revision. Commit. Last, print what was done, the checks, and anything left.

Rules for this session:
- Planned end about 10:45, soft. Steps shrink from the end: step 5, then step 4's (a)+(b) and Panako check. Step 3 does not shrink below offsets covering the whole window period. An unfinished step is reverted, or committed only in a working, tested state, and named in the notes.
- The product does not change: crates/gunfinger and gunfinger-core on master behave as at the start; any variant lives in the s29-span worktree, never merged; `regress session-29-start` identical; scripts/check.sh green.
- Long runs: the Bash tool kills a background task at its timeout, 30 minutes by default. Every run that may last more than about 20 minutes starts detached under `screen` with its own log and an exit line, or gets an explicit timeout of up to 7,200,000 ms; a run longer than 2 h is split. After each run, check that its log ends with its exit line and holds no error.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or reports (work/reports/evaluation-*-scan-ed-rush-optical-essential-mix.json included), rescore nothing, and never run test evaluation 5. Never identify, decode or index EM_1999_02_07_-_Ed_Rush_Optical.mp3 at the NAS root. Run nothing on the candidate mixes of experiment 0065 (including the mixed CDs it lists and the NAS's sety/vertical.mp3).
- Unsearched: the recall panel's validation half and both shared panels' validation halves. The frozen panel files do not change.
- The five test-derived versions are never used to tune anything: Terminal 2, the Orders remixes, the Marky & Bungle Funktion, the Clear Skyz Break remix and 1996 VIP, and the Black Barrel Beachball.
- The NAS (/Volumes/atlas/Music/dnb) is read, never written (its .gunfingerignore included). The NAS peak store (~/.local/share/gunfinger/nas-dnb-peaks) is read-only: no `index`, `prune` or `--retry-skipped`. The duplicate clusters are not rebuilt.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt and work/audition/answers.jsonl do not change. Manifests do not change, including corpus/sets/nocny-trans-vertical-2002-08-23/tracklist.toml.
- AGENTS.md: no SIMD, no `unsafe`, optimise only from measurement, an experiment file of at most 40 lines written before the next experiment starts, evaluation JSON in docs holds file names only, a new ADR to change a decision, cross-platform code (ADR 0020), no filename- or tag-derived logic in the product.
- Out of scope (ADR 0017): report edits, review workflows, and features that fix tags or the library.
- corpus/ is private and never committed; nothing of Panako's is committed. Leave the s24b-panako and s27-span worktrees and branches as they are.
- Leave alone every process this session did not start. Measure no speed.
- NAS-scale harness runs overwrite work/reports/library-nas-dnb-peaks/, so copy what you need first. Keep scratch data under a few GB; delete renders and index clones you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step. Check `git status` before every commit and never use `git add -A`. Do not push. No AI attribution in commits.
```

## State at the start of session 29

- 06:04. Session 28 still finishing in the root checkout: master at
  `adec35a` with `docs/session-notes.md`, `docs/session-28-checklist.md`
  and `docs/status.md` modified (its last notes update), its
  reproduction run (`work/bin/s28-verify/gunfinger-eval ... recall`)
  running. Session 28 finished
  at 07:06 (the orchestrating thread's message): its last commit
  `df2f038`, checked at 07:10: `git status` clean on `master`, no process
  from `work/bin/s28-*` (its wrap-up changed only its brief, notes,
  checklist and status since `adec35a`).
- Experiments 0001-0145 and 0150; this session numbers 0146-0149 (and
  0151 and up if it needs more). ADRs 0001-0020. Test set: 4 of 5
  evaluations used (none in this session).
- The NAS peak store at 06:07: 40,126 records, revision
  `fed26c76b8fe89ea` (`work/s29/store-revision-start.txt`,
  `scripts/analysis/store_revision.py`), as the prompt says. The NAS is
  mounted at `/Volumes/atlas`.
- Listening files: `docs/pair-verdicts.txt` 82 lines, sha256
  `ad74a881ba9f...`; `work/audition/answers.jsonl` 60 records, sha256
  `1e1a98a8158d...` (`work/s29/listening-digests-start.txt`), as at session
  28's start and end. Both must be the same at the end.
- Processes this session did not start are left alone: session 28's
  reproduction run (`work/bin/s28-verify/`).
- Disk: 30 GiB free.
- Worktrees: `s24b-panako` (`bad25f9`), `s27-span` (`812342d`),
  `work/probe-tree` (detached, an older session's), all left alone.

## Plan, with times

Start 06:04; planned end about 10:45, soft.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1a | reading; brief and checklist drafted under `work/s29/`; store revision, listening-file digests | 0:30 | 06:35 |
| 2 | worktree `s29-span` from master, `s27-span` merged in, silence lead-ins at declared offsets with unit tests, its own target directory; today's rule against master's levels on the grid (`work/s29/vwork/`) | 0:35 | 07:10 |
| 1b | once session 28 has finished (message, or my own check at 07:15): brief and checklist to `docs/`, `CLAUDE.md`, commit; Homebrew release build to `work/bin/s29-start/`; `baseline session-29-start` | 0:15 | 07:25 |
| 3a | pre-registration (this brief, experiment 0146), committed before any count | 0:15 | 07:40 |
| 3b | the runs: both shared panels' development halves, every brief passage at 10 offsets, under `screen` | 0:40 | 08:20 |
| 3c | counts (`span_offsets.py`), experiment 0147 | 0:30 | 08:50 |
| 4 | recall (b) and (a)+(b) on the variant build at NAS scale, today's levels against session 28's reports first; counts per rule; 0150's 7 plays offline; experiment 0148 | 0:45 | 09:35 |
| 5 | `seconds`'s unit-test gaps on master (beside the runs of 3b and 4) | 0:30 | (beside) |
| 6 | read-only review by a subagent, from 09:45; fixes | 0:25 | 10:10 |
| 7 | wrap-up: notes, status, checklist, roadmap; `regress session-29-start`; `scripts/check.sh`; store revision; commit; message | 0:35 | 10:45 |

Steps shrink from the end: step 5, then step 4's (a)+(b) and the
Panako check. Step 3 does not shrink below offsets covering the whole
window period.

## Decisions

(by time, as they are made)

- **(06:10) The variant worktree.** `s29-span` at
  `/Users/razorjack/.t3/worktrees/gunfinger/s29-span`, branched from
  master's `adec35a` (session 28's last commit; its uncommitted notes
  update does not touch code), with `s27-span` (`812342d`) merged in
  without conflict (`15f3240`); two of session 28's test literals in
  `recall.rs` gained the variant's three `Outcome` fields so the tests
  compile. `s27-span` itself is unchanged. Its target directory is its
  own (`s29-span/target/`); the release build (Homebrew's 1.98.0) is
  copied to `work/bin/s29-span/` with its commit in `COMMIT`.
- **(06:14) The fair test's queries in the variant** (`583bfe3`):
  `S29_OFFSETS_ONLY` makes each shared scenario's only queries its
  `alone` passages (10, 20 and 30 s, no longer than the passage, as the
  panel's `alone` queries) after 0, 1, ..., 9 s of digital silence,
  named `<scenario>-silence-<o>s-alone-<L>s` and rendered as 0142's
  (`render_samples`, zeros prepended, MP3 128 kbps). `S27_SILENCE_ONLY`
  now builds its queries with the same function (same names). Four unit
  tests: the queries of every passage and offset, passages no longer
  than the passage with distinct names, the offsets cover the 10 s
  window period a second apart (against `search::WINDOW_SECONDS`), and
  the silence comes before the passage in the samples. `cargo clippy
  -D warnings` clean; the 24 `shared` tests pass.
- **(06:16) The reproduction check** for step 2 is the grid at 262 tracks
  (`grid --seed 2026`, 960 queries; `work/s29/vgrid.sh`, `screen`
  `s29-vgrid`, work directory `work/s29/vwork/` with a clone of
  `work/grid/`): every detection's level under today's rule must equal
  master's `work/reports/grid-seed-2026.json` (`span_rule.py identity`
  and `span_rule.py grid`, which also gives criterion 2's counts again).
- **(06:30) Step 2's check passed.** The variant's grid (06:16-06:30,
  exit 0, no error) has every detection identical to master's
  `work/reports/grid-seed-2026.json` (`span_rule.py identity`), today's
  rule reproduces every stored level (0 problems), and the report is
  byte-identical to session 27's variant grid
  (`work/s27/vwork/reports/grid-seed-2026.json`). Criterion 2's counts
  are 0138's (`work/s29/counts/v-grid.json`): confident of 960 and flips,
  today 693 and 30, A(10) 803 and 2, A(15) 644 and 2, A(20) 482 and 1,
  B(10) 947 and 13, B(15) 800 and 0, B(20) 640 and 0.
- **(06:20) Commit order.** The pre-registration below is committed on
  master once session 28 has finished, and the shared runs start only
  after that commit, so no count exists before it.

- **(06:20) Step 5 drafted beside step 2**, in a second worktree from
  master (`s29-tests`, `/Users/razorjack/.t3/worktrees/gunfinger/s29-tests`,
  `adec35a`), since the root checkout's tracked files are session 28's
  until it finishes: six tests in `seconds.rs` (`7d7c6e4`), to be
  cherry-picked onto master. None exposed a bug, so no behaviour changes.
- **(06:23) The analysis scripts, tried before any count**:
  `span_offsets.py` on 0142's reports (it gives 0142's counts: A(10)
  and B(10) add pyro-1 after 3 s in 3 indexes) and `span_recall.py` on
  session 28's brief reports (today's rule gives 0144's and 0145's
  counts exactly: 228 and 493 of 522, 0 of 259 for 15 s plays starting
  0-5 s into a window; 37 and 141 of 180). Both drafted under
  `work/s29/`, moved to `scripts/analysis/` with the pre-registration.
  **What was seen early**: while locating 0150's seven plays in session
  27's mixes report and trying `span_recall.py panako`, the strongest
  detections of the seven (hits, windows, span, supported seconds) and
  the rule levels of two of them (Kosmic: confident under A(10), A(15),
  B(10), B(15); Fanfare: under A(10), B(10)) were printed before this
  pre-registration was committed. The Panako check was already written
  below (06:20) and is not changed; it is offline and deterministic.
  Nothing of the shared offsets or of the variant recall runs exists
  yet.

- **(07:00) Step 4's runs started before the commit.** Session 28's
  last run ended at 06:59 and the machine was idle while it wrote its
  notes, so the variant's two recall runs (`work/s29/vrecall.sh`,
  `screen` `s29-vrecall`) started then. The harness prints only today's
  rule's counts, which are session 28's; the variant counts are made
  offline with `span_recall.py count` only after this pre-registration
  is committed. The shared runs of step 3 start only after the commit.

- **(07:10) The root checkout is this session's** (the orchestrating
  thread's message of 07:06, checked). The brief, checklist,
  pre-registration (experiment 0146) and the two analysis scripts are
  committed on master before any count of the variants; the worktrees
  `s29-span` and `s29-tests` were branched from `adec35a`, whose code is
  `df2f038`'s.

- **(07:11) Committed** the brief, checklist, 0146 and the two scripts
  (`b61c6b5`, 07:10:55). The variant's (b) run (06:59-07:10:55, exit 0)
  had ended in the same second; its report was read only after the
  commit. Its queries and today's detections equal session 28's
  (`span_recall.py same`: 1,404 queries, 21,300 detections, 0
  differences). The shared runs are queued after the recall runs
  (`work/s29/after-recall.sh`, `screen` `s29-voffsets`), one heavy run at
  a time.
- **(07:12) Start build and baseline.** `cargo build --release` with
  Homebrew's 1.98.0 found `target/release/` up to date (session 28's
  final build of the same sources); copied to `work/bin/s29-start/`,
  byte-identical to `work/bin/s28-final/` (`gunfinger` and
  `gunfinger-eval`). `baseline session-29-start`: 0.21 s wall (it copies
  the reports of session 28's final `regress`).
- **(07:13) Step 5 on master**: `7d7c6e4` cherry-picked as `12d3243`; the
  17 `seconds` tests pass on master.

## Step 3: pre-registration (written at 06:20-06:35, committed before any count)

Everything below is fixed before any silence-led query of this session
is searched. Nothing may be added after a count is seen; anything
computed later and not named here is exploratory and decides nothing.
Nothing here changes the product; adopting a variant is our
decision and would need a new ADR replacing the window condition of
ADRs 0001 and 0008, which this session does not write.

### The queries

- Both shared panels' development halves, as 0140-0142:
  `docs/panels/shared-material-seed-2026-development.json` (16
  scenarios) and `docs/panels/shared-material-2-seed-2026-development.json`
  (10). The validation halves stay unsearched; the frozen panel files do
  not change.
- The passages: each scenario's `alone` cuts, the first 10, 20 and 30 s
  of its shared passage, each no longer than the passage: 46 cuts on the
  first panel and 29 on the second, 75 in all. These are every passage of
  30 s or less that the panels' queries hold (0140 and 0142 used those of
  20 s or less; the 30 s cuts and pyro-2's 29.9 s, watermelon-2's
  23.7 s, phantom-force-3's 27.2 s, phantom-force-4's 23.0 s and
  star-trails-2's 22.3 s are the longer ones up to 30 s). The shortest
  passages give cuts of 13.8 s (playground-shades-1), 15.4 s
  (playground-shades-2) and 14.7 s (phantom-force-2) beside their 10 s.
- The offsets: each cut after o s of digital silence, o = 0, 1, 2, ...,
  9 s: ten offsets a second apart, covering the whole 10 s window period
  (the grid's own spacing, 0047). Native speed. o = 0 is the cut alone,
  rendered the same way. 750 queries.
- The indexes, as 0142: everything; the played recording left out; the
  played file alone left out where another rip of it is indexed (the
  positive control), with the corpus plus the NAS store at 40,126
  records (`fed26c76b8fe89ea`) from a clone of the saved index, and
  0128's NAS clusters. 7 scenarios of the first panel and 6 of the
  second have the third index: 112 + 76 = 188 passages (a passage is
  one cut in one index), 1,880 results (a result is one query against
  one index).
- The runs: the variant build `work/bin/s29-span/` (`583bfe3`) with
  `S29_OFFSETS_ONLY=1`, `--work work/s29/vwork --other-peaks-dir
  ~/.local/share/gunfinger/nas-dnb-peaks shared <panel>`, one panel at
  a time under `screen` (`work/s29/voffsets.sh`, logs with exit lines in
  `work/s29/logs/`).

### The rules

Today's (a segment confident at 240 hits in 3 windows, possible at 60;
a play takes its best segment's level), A(10), A(15), A(20), B(10),
B(15) and B(20), exactly as brief 27 defines them (0136): each replaces
only `windows >= 3`; the hits, the possible tier and the grouping into
plays stay. Levels are recomputed offline from each detection's hits,
windows, first and last hit and supported seconds, after today's rule
has reproduced every stored level of the report (any mismatch is
reported and that panel's variant counts are not used).

### Scoring

As 0138-0142 (`span_rule.py shared`; `shared_outcomes.py`'s roles with
0128's clusters, so a rip of the played recording is the played
recording): a detection is wrong when it is confident and its role is
not `played`, in any of the three indexes. A **wrong answer** is a
(result, file) pair: a file named confidently, by any of its detections,
in that result. A **passage** is (scenario, cut, index); its ten results
are its offsets. Counted by `scripts/analysis/span_offsets.py` (written
and tried on 0142's reports before this pre-registration).

### Criterion 1, the fair form (F), written before any count

A variant meets F when both hold:

- **F1**: on each panel, its wrong answers summed over all passages and
  offsets are no more than today's rule's;
- **F2**: no passage has a wrong answer under the variant at any offset
  while today's rule has a wrong answer at none of that passage's ten
  offsets.

A wrong possible answer becoming confident counts in both, also beside
a wrong file already confident in the same result.

**Brief 27's criterion 1, as written (C27)**, is reported beside F and
not replaced: a new wrong answer is a (result, file) pair confident and
wrong under the variant and not under today's rule in the same result;
C27 holds when there is none.

### The measures

- M1: wrong answers per rule and offset, per panel (and per index).
- M2: for each passage with a wrong answer under any rule, the offsets
  at which today's rule is wrong (its dependence on the grid: 10 of 10
  means its position does not protect today's rule) and those at which
  each variant is.
- M3: right confident results per rule and offset: results in the
  everything and the positive-control indexes with a confident detection
  of the played recording.
- M4: level changes against today's rule per rule: detections moved
  between possible and confident, right and wrong apart.
- The build's check, beside: offsets 3, 5 and 7 s of each cut of 20 s
  (or of the whole passage when it is shorter) are 0142's queries by
  name; their renders are compared byte for byte with 0142's
  (`work/s27/vsilence/shared/`) and their results detection by detection
  with 0142's reports.

### Expected before the data, so it cannot be claimed later

- From 0142: today's rule names the wrong Fire versions and Time Is
  Broken at most offsets of the 20 s cuts and at every offset of the
  30 s cuts of fire-2, fire-6 and pyro-2, in the everything and the
  played-left-out indexes; and never anything confidently in a 10 s cut,
  which spans 2 windows at most.
- B(10) can make a 10 s cut confident (10 supported seconds fit in a span
  under 10 s); fire-2's and fire-6's wrong versions have about 320 hits
  per 10 s, so F2 may fail for B(10). A(10) cannot make a 10 s cut
  confident (its span is under 10 s); A(20) and B(20) none of 10 or 15 s.
- The wrong answers of A(15), B(15) and A(10) should be close to today's
  rule's summed over offsets (0142: equal or one more at 3, 5 and 7 s).

### What the session concludes from it

A variant is worth proposing when it meets F and brief 27's criterion 2
on the grid (0138: A(10), B(10) and B(15) do; the step 2 run counts it
again). If more than one does, the notes compare them by fewer wrong
answers summed over offsets, then more right confident results here and
more brief plays confident at NAS scale (step 4); we decide.

## Step 4: pre-registration (written with step 3's, committed before any count)

- **Queries**: session 28's brief plays of the recall panel's
  development half, (b) and (a)+(b) (0143): 58 searched and 20 held-out
  sources, 9 speeds, 15 and 20 s, 1,404 queries each; (a)+(b) in the
  index without each searched source's own record and its identical
  copies (20 sources observable, 38 unobservable). Rebuilt by the
  variant build (`recall --queries brief`, then `recall --other-rips-only
  --queries brief`) in `work/s29/vwork/` from a clone of
  `work/recall/seed-2026/` (the cached excerpts), one run at a time under
  `screen`.
- **Check first**: each variant report against session 28's
  (`work/s28/reports/recall-seed-2026-brief.json` and
  `recall-seed-2026-other-rips-brief.json`): the same queries (source,
  speed, length, where the play starts, lead-in and tail files) and,
  detection by detection, the same asset, hits, windows and level. A
  mismatch is reported and the variant counts of that report are not
  used.
- **Counts, per rule**: confident recall at 15 and 20 s (searched
  sources' queries with a confident detection of their own cluster; of
  522 under (b), of 180 observable under (a)+(b)); the same by where the
  play starts in a 10 s window (whole-second bins of `starts_at_seconds`
  mod 10); wrong answers (a detection outside the own cluster, no
  partner in brief plays) confident, and possible only; held-out and
  unobservable queries with a confident detection.
- **Panako's 7 plays (0150)**: offline from session 27's variant report
  `work/s27/vwork/reports/mixes-seed-2026.json` (identical to master's,
  0138), today's levels reproduced first: under each rule, whether each
  play becomes confident (item.code "Kosmic", mix 02; Aphrodite
  "Fanfare", 03; Friction & Nu Balance "Movement", 04; Silent Witness
  "Again and Again", 06; Gridlok "Soul Sucker", 07; Bad Company "Pulse",
  09; Dominion, 10), with its hits, windows, span and supported seconds.
- **Notable, declared before any count**: any wrong answer confident
  under a variant; any held-out or unobservable query confident under a
  variant.
