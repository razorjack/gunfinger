# Session 27 brief: a minimum span instead of 3 windows, measured end to end; per-second scoring

Written by the agent for itself on 2026-10-17, so the plan survives
context compaction. Work autonomously and never stop to ask. When
something is ambiguous, take the conservative option, write the decision
down and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-27-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 23" to the end; the handoffs of
   sessions 24-26).
4. `docs/status.md` (newest entries at the bottom).
5. The roadmap's "A minimum span instead of 3 windows", "Detection
   boundaries" and "Public DJ-mix datasets and per-second scoring";
   ADRs 0006, 0008, 0013; experiments 0020, 0030, 0047, 0057, 0111,
   0122, 0129, and this session's (0136 and up).
6. The code: `crates/gunfinger-core/src/confidence.rs`,
   `crates/gunfinger-core/src/search/refine.rs`,
   `crates/gunfinger-eval/src/grid.rs`, and this session's scripts.

`docs/brief-2.md` to `docs/brief-26.md` and `docs/brief-24b.md` hold
rules that still apply unless this brief changes them (among them: edit
no configuration file; test fixtures are generated, never copied from
the corpus or the NAS; runs longer than 30 minutes start detached under
`screen`; heavy runs one at a time; NAS-scale harness runs overwrite
`work/reports/library-nas-dnb-peaks/`, so copy it first).

## The prompt, verbatim

```text
Start session 27 of Gunfinger, an overnight session: measure the roadmap's "A minimum span instead of 3 windows" (brief plays) end to end, now that the shared-material panels it was waiting for exist, and add per-second scoring to the harness to judge it. It runs unattended from about 00:35 with a planned end at about 05:30. The end is soft: finish a run or review in progress rather than kill it, but plan to end by then. Another session may follow until 11:00, so leave everything committed and a handoff in the notes. The NAS is mounted at /Volumes/atlas. Work autonomously and never stop to ask. When something is ambiguous, take the conservative option, write the decision down and continue.

Context.
- The roadmap (docs/roadmap.md, "Matching", "A minimum span instead of 3 windows"): whether a 15-20 s play is confident depends on where the 10 s windows fall (experiment 0020: 76 of 160 grid positions confident at 15 s, 148 at 20 s; 0047 under the default matcher: 67 and 146 of 160). 200 hits over at least 10 s between the first and last hit made all of them confident offline and changed nothing in the sweep or the development scans (0020), lowering the shortest identification from about 25 s to about 11 s, which is our call. The roadmap asks: measure it under today's matcher and link rules (ADR 0008, experiment 0030); compare the span from the first to the last hit with the time hits support (`pair`'s supported time, experiment 0057); keep the hit threshold fixed to isolate the change; success is fewer level changes caused only by where a play falls on the window grid, more brief plays confident, and no new confident error on the shared-material challenge set, "which comes first". That set now exists: the first shared panel's development half (0111, 0122; 218 queries, 106 already wrong confident with the played recording left out) and the second panel's development half (0129; 135 queries, 5). `gunfinger-eval grid` already has `confident_under(min_span)` (crates/gunfinger-eval/src/grid.rs).
- Today's rule is in gunfinger-core's confidence.rs (ADR 0013, play confidence; ADR 0006, the possible tier; the 240-hit rule). Read them and say exactly which rule the variant replaces.
- Sessions 24-26 measured a diagnostic version verifier (0124-0127, 0133-0135); it failed its criteria twice and is not this session's subject.
- No listening was done: the six questions in work/audition/s23/ and the cut Sonar joins stay open. Plan nothing that waits on a judgement by ear, and never answer a question.
- State at the start: git clean at 21b0220 (check it); the NAS store at 40,126 records, revision fed26c76b8fe89ea (record it; if new files are indexed during the session, record the change and carry on).

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it), docs/brief-26.md and docs/session-26-checklist.md (brief-2 to brief-25 and brief-24b hold rules that still apply unless this prompt changes them), docs/session-notes.md ("Session 23" to the end), docs/status.md (newest entries at the bottom), ADRs 0004, 0006, 0008 and 0013, experiments 0020, 0030, 0031, 0047, 0057, 0111, 0122 and 0129, the roadmap's "A minimum span instead of 3 windows", "Detection boundaries" and "Public DJ-mix datasets and per-second scoring", and crates/gunfinger-core/src/confidence.rs and crates/gunfinger-eval/src/grid.rs.
2. Write docs/brief-27.md (this prompt verbatim, the state at the start, the plan with times) and docs/session-27-checklist.md. Point CLAUDE.md's reading list at them; keep brief-26 among the older briefs. Commit.
3. Build with --release and copy the binaries to work/bin/s27-start/. Run `gunfinger-eval baseline session-27-start` (record its wall time).

Steps:

1. Setup (until about 01:00): the steps above.

2. Pre-registration (about 30 min), written into the brief and the experiment file before any count:
   - The variants: (A) the minimum span, the hit threshold unchanged, "N hits over at least S seconds between the first and last aligned hit" in place of the 3-window condition, with S = 10 s as in 0020 and one or two more values declared now; (B) the same with supported time (seconds holding aligned hits) in place of the span. Say how each applies at segment and play level (ADR 0013), and to the possible tier.
   - The panels: grid (every window offset, 0047), the sweeps 2026-2029, the generated mixes (seeds 2026 and 7), `robust` (damage), the three development mixes at 262 tracks and at NAS scale with their leave-outs, the recall panel's development half, and both shared panels' development halves.
   - The criteria, both required for calling a variant worth proposing: no new wrong confident answer on any panel (the shared panels first; a wrong possible answer becoming confident counts); and more brief plays confident on the grid, with fewer level changes caused only by the window grid. Report everything else (level changes on true plays elsewhere, the shortest identification) beside them.
   - How the counts are made: offline from saved detections where they carry what the rule needs (hits, first and last aligned hit, supported time per segment), checked first by reproducing today's levels exactly from the same reports; otherwise a variant build in a worktree that is never merged (as session 22 did), with its own work directory so it never writes work/reports/. Say which panel uses which.
   Commit before any count.

3. Per-second scoring in the harness (about 2 h, can overlap step 4's runs): for mixes with exact truth (the generated mixes; the shared-mixes renders if their plans hold the truth) score each second as correctly labelled, confidently mislabelled, possibly labelled, unlabelled while a library track plays, and silent while the playing track is not in the library (specificity), plus each play's start and end error; for the three development mixes, with their manifests' approximate borders, the same with an explicit uncertainty margin around each border, stated in advance. Follow the roadmap's sketch (Sonnleitner et al. 2016, section 5): both tracks count during a transition. Unit tests. Report today's matcher on these measures (a baseline the project has not had) and each variant beside it. Keep the current scoring unchanged for continuity. Commit.

4. The counts (about 1 h 30 min): every panel under today's rule and each variant; every changed level listed with its cause where it can be named; the shortest identification under each. Experiment files (at most 40 lines each, written before the next experiment starts).

5. A read-only review by a subagent, starting by about 04:15; fix what it finds.

6. Wrap-up (from about 04:45): a "Session 27" section at the end of docs/session-notes.md (what each variant would change, in plain terms: which brief plays it would name, the shortest identification, any new wrong answer; the per-second baseline; whether a variant meets the criteria; that adopting it is our decision and would need a new ADR, which this session does not write), docs/status.md, the checklist, the roadmap's two bullets. `regress session-27-start` identical with the final build; scripts/check.sh green with rustup's 1.98.0 first on PATH and RUSTUP_TOOLCHAIN=1.98.0 (never install a toolchain); the NAS store's revision. Commit. Last, print what was done, the checks, and anything left.

Rules for this session:
- Planned end about 05:30, soft. Steps shrink from the end: the development mixes' per-second scoring in 3, then variant (B) in 4. An unfinished step is reverted, or committed only in a working, tested state, and named in the notes.
- The product does not change: crates/gunfinger and gunfinger-core behave as at the start; any variant lives in a worktree never merged, or offline; `regress session-27-start` identical; scripts/check.sh green.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or reports, rescore nothing, and never run test evaluation 5. Never identify, decode or index EM_1999_02_07_-_Ed_Rush_Optical.mp3 at the NAS root. Run nothing on the candidate mixes of experiment 0065 (including the mixed CDs it lists and the NAS's sety/vertical.mp3).
- Unsearched: the recall panel's validation half and both shared panels' validation halves. The frozen panel files do not change.
- The five test-derived versions are never used to tune anything: Terminal 2, the Orders remixes, the Marky & Bungle Funktion, the Clear Skyz Break remix and 1996 VIP, and the Black Barrel Beachball.
- The NAS (/Volumes/atlas/Music/dnb) is read, never written (its .gunfingerignore included). The NAS peak store is read-only: no `index`, `prune` or `--retry-skipped`. The duplicate clusters are not rebuilt this session.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt and work/audition/answers.jsonl do not change. Manifests do not change, including corpus/sets/nocny-trans-vertical-2002-08-23/tracklist.toml.
- AGENTS.md: no SIMD, no `unsafe`, optimise only from measurement, an experiment file of at most 40 lines written before the next experiment starts, evaluation JSON in docs holds file names only, a new ADR to change a decision, cross-platform code (ADR 0020), no filename- or tag-derived logic in the product.
- Out of scope (ADR 0017): report edits, review workflows, and features that fix tags or the library.
- corpus/ is private and never committed; nothing of Panako's is committed, and the s24b-panako worktree and branch are left alone.
- Leave alone every process this session did not start.
- Runs longer than 30 minutes start detached under `screen`; heavy runs one at a time. NAS-scale harness runs overwrite work/reports/library-nas-dnb-peaks/, so copy what you need first. Keep scratch data under a few GB.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step. Check `git status` before every commit and never use `git add -A`. Do not push. No AI attribution in commits.
```

## State at the start of session 27

- 00:29. `git status` clean at `21b0220` on `master` (78 commits ahead
  of `origin/master`, never pushed). Experiments 0001-0135; this session
  numbers 0136 and up. ADRs 0001-0020. Test set: 4 of 5 evaluations
  used (none in this session).
- The NAS peak store at 00:30: 40,126 records, revision
  `fed26c76b8fe89ea` (`work/s27/store-revision-start.txt`,
  `scripts/analysis/store_revision.py`), as the prompt says. The NAS is
  mounted at `/Volumes/atlas`.
- Processes this session did not start are left alone. The `s24b-panako` worktree and branch are left alone.
  - Disk: 20 GiB free.
- Listening files: `docs/pair-verdicts.txt` sha256 `d48ffb313396...`,
  `work/audition/answers.jsonl` sha256 `ee2d5bd9fe0d...`, as at session
  26's end (`work/s27/listening-digests-start.txt`).
- Build: `cargo build --release` (Homebrew's 1.98.0) was current;
  `work/bin/s27-start/` holds `gunfinger` and `gunfinger-eval`
  byte-identical to `work/bin/s26-final/`.
- `baseline session-27-start` copies the last `regress`'s reports (0.21
  s wall), so, as in sessions 24-26, `regress session-27-start` is the
  speed check: identical (720/720 sweep queries, all 87, 93 and 79 scan
  detections, the calibrate line known since session 8), 154.5 s wall,
  1,308 s user (session 26: 153.9 s). The machine runs at its usual
  speed.

## Plan, with times

Start 00:29; planned end about 05:30, soft.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1 | setup: brief, checklist, build, baseline, `regress` as the speed check, store revision | 0:30 | 01:00 |
| 2 | pre-registration: the variants and which rule they replace, the panels, the criteria, which panel is counted from which data; experiment 0136's first part; committed before any count | 0:30 | 01:30 |
| 3a | the variant build in a worktree (supported time and spans in every report it writes; nothing else changes), its identity checks, the queue of its runs under `screen` | 0:30 | 02:00 |
| 3 | per-second scoring in the harness (generated mixes first, then the development mixes with their margins), unit tests; today's matcher as the baseline | 2:00 | 04:00 (overlaps 4) |
| 4 | the counts: variant A offline from master's saved reports first, then both variants from the variant build's runs as they finish; changed levels and causes; the shortest identification; experiment files | 1:30 | 04:15 |
| 5 | read-only review by a subagent from 04:15 at the latest; fixes | 0:30 | 04:45 |
| 6 | wrap-up: notes, status, checklist, roadmap, `regress`, `check.sh`, store revision, commit | 0:45 | 05:30 |

Steps shrink from the end: the development mixes' per-second scoring in
3, then variant (B) in 4. Heavy runs (`regress`, the variant build's
runs, `check.sh`) never overlap one another; builds and unit tests of
step 3 run beside the variant's runs, as the prompt allows.

## Decisions

- **(00:35) The speed check.** As in sessions 24-26, `baseline` only
  copies reports, so `regress session-27-start` with the start's
  binaries is the speed check (above).
