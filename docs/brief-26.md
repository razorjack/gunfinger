# Session 26 brief: session 24b merged; the version verifier's version 2, with its own line from the audio

Written by the agent for itself on 2026-10-16, so the plan survives
context compaction. Work autonomously and never stop to ask. When
something is ambiguous, take the conservative option, write the decision
down and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-26-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 23" to the end; the handoffs of
   sessions 24, 24b and 25).
4. `docs/status.md` (newest entries at the bottom).
5. `docs/research/2026-10-16-version-verification.md`; experiments
   0124-0129 and 0132, and this session's (0133 and up).
6. `docs/brief-24.md` (version 1 as pre-registered, T = 0.121, the
   split) and `docs/brief-25.md`.
7. The code: `crates/gunfinger-eval/src/versions.rs`,
   `versions/compare.rs`, `scripts/analysis/version_plays.py`,
   `version_lines.py`.

`docs/brief-2.md` to `docs/brief-25.md` and `docs/brief-24b.md` hold
rules that still apply unless this brief changes them (among them: edit
no configuration file; test fixtures are generated, never copied from
the corpus or the NAS; runs longer than 30 minutes start detached under
`screen`; heavy runs one at a time; NAS-scale harness runs overwrite
`work/reports/library-nas-dnb-peaks/`, so copy it first).

## The prompt, verbatim

```text
Start session 26 of Gunfinger, an overnight session: merge session 24b's Panako branch into master, then build and measure version 2 of the diagnostic version verifier, which takes each play's line from the audio instead of the reported one. It runs unattended from about 23:25 with a planned end at about 06:00. The end is soft: finish a run or review in progress rather than kill it, but plan to end by then. Another session follows until 11:00, so leave everything committed and a handoff in the notes. The NAS is mounted at /Volumes/atlas. Work autonomously and never stop to ask. When something is ambiguous, take the conservative option, write the decision down and continue.

Context.
- Session 24 (0124-0127) built `gunfinger-eval versions`, a diagnostic version verifier; with T = 0.121 set on the first shared panel's development half it rejected 5 of 108 true plays in the three development mixes (E1, limit 5) and 43 of 87 wrong related plays in rendered mixes (E2, 44 required): failed by one. All five rejected true plays have reported lines 98-193 ms off (the opening-window anchor in refine.rs; alien5ive uploads drifting 0.2-0.5% in speed). Its reach is Fire's family: Dozer's, Pyro's and Sonar's wrong versions play the same content along the matched stretch.
- Session 25 (0128, 0129, 0132): three harness fixes (the `decode::watch` tests robust under load; `holders` refuses a left-out file that is not indexed; `render_samples_as_indexed`, the reference on the store's timeline); the NAS clusters rebuilt at 40,126 records (run `clusters` with `--cut-sparsest` from the start until the cut Sonar joins are judged); the second shared panel's development half at NAS scale: 5 of 135 queries wrong confident with the played recording left out (all Phantom 2018 over Phantom Force), against the first panel's 106 of 218; session 24's verifier, frozen, rejected 4 of those 5 and kept 77 of 78 positives (too few wrong plays to read). Read docs/session-notes.md "Session 24" and "Session 25" with their handoffs.
- Session 24b ran in parallel in the worktree /Users/razorjack/.t3/worktrees/gunfinger/s24b-panako on branch `s24b-panako`. Panako could not be built (it needs a JDK 11-16; only Temurin 21 is installed), so its part is written up as blocked; Gunfinger's side of the comparison is measured (0130 pre-registration, 0131). Its commits are on the branch only; its notes say what the merge needs.
- No listening was done: the six questions in work/audition/s23/ and the cut Sonar joins stay open. Plan nothing that waits on a judgement by ear, and never answer a question.
- State at the start: git clean at 0aa1dea (check it); the NAS store at 40,126 records, revision fed26c76b8fe89ea (record it; if new files are indexed during the session, record the change and carry on).

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it), docs/brief-25.md and docs/session-25-checklist.md (brief-2 to brief-24 hold rules that still apply unless this prompt changes them; brief-24 and brief-25 matter most), docs/session-notes.md ("Session 23" to the end), docs/status.md (newest entries at the bottom), docs/research/2026-10-16-version-verification.md, experiments 0124-0129 and 0132, and the code of `gunfinger-eval versions`, scripts/analysis/version_plays.py and version_lines.py.
2. Write docs/brief-26.md (this prompt verbatim, the state at the start, the plan with times) and docs/session-26-checklist.md. Point CLAUDE.md's reading list at them; keep brief-25 among the older briefs. Commit.
3. Build with --release and copy the binaries to work/bin/s26-start/. Run `gunfinger-eval baseline session-26-start` (record its wall time).

Steps:

1. Setup (until about 23:50): the steps above.

2. Merge `s24b-panako` (about 30 min): read its notes ("Session 24b", "what session 25 needs to merge") and `git log master..s24b-panako`. Merge it with a merge commit (no AI attribution), resolving the conflicts so that every session's sections in docs/session-notes.md, docs/status.md and docs/roadmap.md are kept whole and in time order. Its experiments keep their numbers (0130, 0131; master has 0128, 0129, 0132 around them). Add docs/brief-24b.md to CLAUDE.md's list of older briefs. After the merge `regress session-26-start` must be identical (24b changed no product code; check). Leave the worktree and its work/ (the Panako clone and run scripts, kept for a later run if Java 11 is installed) and the branch in place.

3. Verifier version 2: design and pre-registration (about 45 min), written into the brief and the experiment file before any count on an evaluation set:
   - The line: each play's line measured from the audio over its body, not taken from the report: the reference's own hash hits along the play (as refine.rs's per-stretch measurement and version_lines.py's excerpt lines do), a piecewise or smoothly varying speed for uploads that drift, and stretches split where the material jumps (a loop, 0132's looped query). Say how a play with no measurable line is treated (it decides nothing: neither rejected nor kept as supported).
   - The kill rule in blends (session 24's Astro-Sine case): the outgoing record's low end masking part of the low group should read as masked or killed, not contradicted; say how.
   - Everything else as version 1 unless a change is written down here with its reason.
   - Calibration: the first panel's development half only, with version 1's rule for T (the most lenient value that rejects half of the wrong plays there).
   - Evaluation, declared as a second use: E1 (the 108 true plays; their version-1 count, 5, is known) and E2 (the 87 wrong related plays; 43 known). The criteria as version 1's (E1 at most 5 rejected, E2 at least 44). Beside them, deciding nothing: the second panel's development half (5 wrong, 78 positives) and E2's true drawn plays. Write down that a pass on a second use is weaker evidence than session 24's first use, and what a first use would need (new real mixes with checked tracklists, which no session can add).
   Commit before any evaluation run.

4. Implementation (about 2 h 30 min): in gunfinger-eval, with unit tests (a drifting copy, a line off by 150 ms, a jump, a blend with a bass cut). No change to crates/gunfinger or gunfinger-core behaviour; no SIMD, no `unsafe`; cross-platform. Smoke runs on calibration items only. Commit.

5. Calibration, then evaluation (about 1 h 30 min): T from the calibration set, committed before step 5's evaluation runs; then E1, E2 and the side sets against the criteria. For every true play still rejected, say why, as session 24 did. Experiment files (at most 40 lines each, written before the next experiment starts).

6. A read-only review by a subagent, starting by about 04:45; fix what it finds.

7. Wrap-up (from about 05:15): a "Session 26" section at the end of docs/session-notes.md (the merge; version 2's line and kill rule; the numbers against the criteria beside version 1's; what a pass would and would not show; the handoff for the next session), docs/status.md, the checklist, the roadmap. `regress session-26-start` identical with the final build; scripts/check.sh green with rustup's 1.98.0 first on PATH and RUSTUP_TOOLCHAIN=1.98.0 (never install a toolchain); the NAS store's revision. Commit. Last, print what was done, the checks, and anything left.

Rules for this session:
- Planned end about 06:00, soft. Steps shrink from the end: the side sets in 5, then the unit tests' extras in 4. An unfinished step is reverted, or committed only in a working, tested state, and named in the notes.
- The product does not change: crates/gunfinger and gunfinger-core behave as at the start; `regress session-26-start` identical; scripts/check.sh green.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or reports, rescore nothing, and never run test evaluation 5. Never identify, decode or index EM_1999_02_07_-_Ed_Rush_Optical.mp3 at the NAS root. Run nothing on the candidate mixes of experiment 0065 (including the mixed CDs it lists and the NAS's sety/vertical.mp3).
- Unsearched: the recall panel's validation half and both shared panels' validation halves. The frozen panel files do not change.
- The five test-derived versions are never used to tune or calibrate anything: Terminal 2, the Orders remixes, the Marky & Bungle Funktion, the Clear Skyz Break remix and 1996 VIP, and the Black Barrel Beachball.
- The NAS (/Volumes/atlas/Music/dnb) is read, never written (its .gunfingerignore included). The NAS peak store is read-only: no `index`, `prune` or `--retry-skipped`. The duplicate clusters are not rebuilt this session.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt and work/audition/answers.jsonl do not change. Manifests do not change, including corpus/sets/nocny-trans-vertical-2002-08-23/tracklist.toml.
- AGENTS.md: no SIMD, no `unsafe`, optimise only from measurement, an experiment file of at most 40 lines written before the next experiment starts, evaluation JSON in docs holds file names only, a new ADR to change a decision, cross-platform code (ADR 0020), no filename- or tag-derived logic in the product.
- Out of scope (ADR 0017): report edits, review workflows, and features that fix tags or the library.
- corpus/ is private and never committed; nothing of Panako's (source, jars, databases) is committed.
- Leave alone every process this session did not start.
- Runs longer than 30 minutes start detached under `screen`; heavy runs one at a time. NAS-scale harness runs overwrite work/reports/library-nas-dnb-peaks/, so copy what you need first. Keep scratch data under a few GB.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step. Check `git status` before every commit and never use `git add -A`. Do not push. No AI attribution in commits.
```

## State at the start of session 26

- 23:22. `git status` clean at `0aa1dea` on `master` (55 commits ahead
  of `origin/master`, never pushed). Experiments 0001-0129 and 0132 on
  `master`; `s24b-panako` holds 0130 and 0131, so this session numbers
  0133 and up. ADRs 0001-0020. Test set: 4 of 5 evaluations used (none
  in this session).
- Branch `s24b-panako` at `3a1420a` (session 24b's wrap-up), worktree
  `/Users/razorjack/.t3/worktrees/gunfinger/s24b-panako`, clean; no
  process of it runs.
- The NAS peak store at 23:23: 40,126 records, revision
  `fed26c76b8fe89ea` (`work/s26/store-revision-start.txt`), as the
  prompt says; the newest record written at 17:53. The NAS is mounted at
  `/Volumes/atlas`.
- Processes this session did not start are left alone.
  - Disk: 12 GiB free (19 GiB
  at session 25's start), so scratch data stays small and the verifier's
  timelines are deleted once summarised if space runs short.
- Listening files: `docs/pair-verdicts.txt` sha256 `d48ffb313396...`,
  `work/audition/answers.jsonl` sha256 `ee2d5bd9fe0d...`, as at session
  25's end (`work/s26/listening-digests-start.txt`).
- Build: `cargo build --release` (Homebrew's 1.98.0) was current;
  `work/bin/s26-start/` holds `gunfinger` and `gunfinger-eval`
  byte-identical to `work/bin/s25-final/`.
- `baseline session-26-start` copies the last `regress`'s reports (0.24
  s wall), so, as in sessions 24 and 25, `regress session-26-start` is
  the speed check: identical (720/720 sweep queries, all 87, 93 and 79
  scan detections, the calibrate line known since session 8), 153.9 s
  wall, 1,300 s user (session 25: 153.8 s). The machine runs at its
  usual speed.

## Plan, with times

Start 23:22; planned end about 06:00, soft.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1 | setup: brief, checklist, build, baseline, `regress` as the speed check, store revision | 0:30 | 23:50 |
| 2 | merge `s24b-panako` (merge commit; notes, status and roadmap kept whole and in time order; brief-24b in CLAUDE.md); `regress` identical after it | 0:30 | 00:20 |
| 3 | version 2's design and pre-registration: the measured line, the kill rule in blends, the split, T's rule, the criteria, the second use; experiment 0133's first part; committed before any run | 0:45 | 01:05 |
| 4 | implementation in `gunfinger-eval versions` with unit tests (a drifting copy, a line off by 150 ms, a jump, a blend with a bass cut); smoke runs on calibration items only; committed | 2:30 | 03:35 |
| 5 | calibration (T committed), then E1, E2 and the side sets; the rejected true plays and why; experiment files | 1:10 | 04:45 |
| 6 | read-only review by a subagent from 04:45 at the latest; fixes | 0:30 | 05:15 |
| 7 | wrap-up: notes, status, checklist, roadmap, `regress`, `check.sh`, store revision, commit | 0:45 | 06:00 |

Steps shrink from the end: the side sets in 5, then the unit tests'
extras in 4. Heavy runs (`regress`, the verifier's runs, `check.sh`)
never overlap.

## Decisions

- **(23:30) The speed check.** As in sessions 24 and 25, `baseline`
  only copies reports, so `regress session-26-start` with the start's
  binaries is the speed check (above).
