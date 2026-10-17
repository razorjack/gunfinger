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

## Step 2: pre-registration (written at 00:37-00:41, committed before any count)

Everything below is fixed before any variant is counted on any panel.
Nothing may be added after a count is seen; anything computed later and
not named here is exploratory and decides nothing. Nothing here changes
the product; adopting a variant is our decision and would need
a new ADR, which this session does not write.

### Today's rule, and what a variant replaces

`identify` and the harness search with `Matcher::Fitted` (ADR 0008).
Every detection with 10 first-pass hits or more is measured again at its
fitted speed (`search/refine.rs`): its hits are the densest run of
offsets in each 10 s window of query time (windows counted from the
query's start) that holds at least 3 hits (`MIN_LINE_HITS`); `windows`
is the number of such windows, `hits` their sum, `start_seconds` and
`end_seconds` the query times of the first and the last of those hits.
`Evidence::confidence` (`confidence.rs`, `FITTED_RULE`) makes a
detection (a play's segment):

- **confident** when `hits >= 240` **and `windows >= 3`** (`MIN_WINDOWS`);
- **possible** otherwise when `hits >= 60` (`MIN_POSSIBLE_HITS`, ADR 0006;
  no window condition);
- **weak** otherwise.

A play is the segments of one asset at least possible with gaps of 90 s
at most (`plays::group`, ADR 0006); its level is the highest any of its
segments reaches, with hits and windows never pooled (ADR 0013).

Each variant replaces exactly one condition: `windows >= 3` in the
confident rule. Everything else stays: 240 hits; the possible tier (60
hits, no window or span condition); the search and its detections; the
grouping into plays (it depends only on the possible tier, so plays and
their segments are identical under every variant); the play level as
the highest segment level (ADR 0013); no pooling across segments. The
single pass (`--single-pass`, 200 hits) is not measured. So a variant
can change a segment only between possible and confident, and only one
with 240 hits or more.

- **Variant A(S), the minimum span**: confident when `hits >= 240` and
  `end_seconds - start_seconds >= S`, S = **10 s** (0020), **15 s** and
  **20 s** (`grid::SPANS`). A(10) only promotes: hits in 3 windows lie
  more than 10 s apart (the first before the end of the first window,
  the last at or after the start of the third), so every segment
  confident today is confident under A(10). A(15) and A(20) can also
  demote a segment of 3 windows or more whose span is shorter.
- **Variant B(S), supported time**: confident when `hits >= 240` and
  `supported_seconds >= S`, S = **10, 15 and 20 s**, where
  `supported_seconds` is the number of distinct whole seconds after the
  segment's first hit, `floor(t - start_seconds)`, holding at least one
  of the hits counted in `hits` (the anchors' query times `t` of each
  window's densest run). This is `pair`'s supported time (0057:
  distinct seconds holding a hit of a window's densest run) on the
  query side, counted from the first hit so that it does not depend on
  where whole seconds fall either. Gaps without hits do not count, as
  they do in the span.

Six variants in all. Each is judged on its own by the criteria below.

### The panels, and where each count comes from

Two sources of detections. **M**: master's saved reports, read offline;
variant A only, since no report holds supported time. **V**: one run of
each panel with a variant build in a worktree that is never merged
(branch `s27-span`, `/Users/razorjack/.t3/worktrees/gunfinger/s27-span`),
whose only change is that every report it writes also holds each
detection's `supported_seconds` and, in the sweep's, recall's and
`robust`'s reports, each detection's first and last hit (and `robust`
keeps its queries' detections); its search is today's. V writes only to
its own work directory, `work/s27/vwork/` (APFS clones of the render
caches and the saved NAS index, so nothing is rendered or indexed again),
never to `work/reports/`. V runs one panel at a time under `screen`.

Before any variant count from a report, today's rule is applied to the
same report and must reproduce every stored level (each detection's
`confident` flag or `level`) and the stored score (identified, wrong,
recalled, outcomes) exactly; a report that fails is not used and the
failure is reported. Before V's numbers are used, V's detections are
compared with M's for every report both hold at the same library state
(asset, first and last hit, track position, speed, windows, hits,
fitted): they must be identical.

| Panel | M (A only) | V (A and B) |
|---|---|---|
| grid, seed 2026, 16 briefs x 6 lengths x 10 offsets | `grid-seed-2026.json` | rerun |
| sweeps 2026-2029 (262 tracks) | no first/last hit: hits-only bounds | rerun |
| generated mixes, seeds 2026 and 7 | `mixes-seed-2026.json`, `mixes-seed-7.json` | rerun |
| `robust`, seed 2026, every condition (the damage) | stale (single pass, 2026-10-06), not used | rerun |
| development mixes at 262 tracks: Knowledge, its leave-outs 3 and 11; 2003-03-29, its leave-out 3; Vertical | `scan-*.json` in `work/reports/` | rerun (and Vertical's leave-out 3) |
| development mixes at NAS scale (40,126 records): the three, and every track left out (11, 12, 17) as 0122 | session 23's (39,354 records), beside only | rerun |
| recall panel, development half, seed 2026 (NAS) | no first/last hit: hits-only bounds | rerun |
| first shared panel, development half (NAS) | 0122's run (39,354 records; 0129: scores the same under 0128's clusters) | rerun at 40,126 |
| second shared panel, development half (NAS) | 0129's run (40,126) | rerun |

Where M and V both exist, a new wrong confident answer in either counts.
"Hits-only bounds": a variant cannot make a detection confident below
240 hits, so where no false detection reaches 240 hits, no variant can
add a wrong answer there, whatever its span.

V's runs, in this order (criteria first): grid; second shared panel;
first shared panel; sweeps; mixes; scans at 262; `robust`; NAS scans;
recall. If time runs short, the runs not finished are named, and B
shrinks first (the prompt's order).

### The criteria (both required for a variant to be worth proposing)

1. **No new wrong confident answer on any panel**, the shared panels
   first. A new wrong answer is a detection, play, track slot or query
   that is wrong by the panel's own scoring and confident under the
   variant but not under today's rule; a wrong possible answer that
   becomes confident counts, also in a query that is already wrong
   confident today (a second wrong title is a new wrong answer). The
   panels' scoring: shared, `shared_outcomes.py`'s oracle (a confident
   detection of a role other than `played`; the blend partner set
   aside), in all three indexes and the controls; scans, `scoring.rs`
   (a confident detection outside every slot it could credit);
   generated mixes and grid, `mixes::score` (a confident detection
   matching no play); sweeps and recall, a confident detection not
   `correct`; `robust`, a confident detection of another cluster.
2. **More brief plays confident on the grid, with fewer level changes
   caused only by the window grid.** Over the grid's 960 queries, a
   query is confident when any detection of the brief play's cluster
   overlapping it is confident (`grid::confident_under`). Required, both
   strictly: more confident queries than today's rule in total, and
   fewer *flips* in total, a flip being a (brief, length) pair whose
   verdict differs between its 10 offsets (`grid::print_summary`).

Reported beside, deciding nothing: per panel, every true segment or play
whose level changes (promotions and demotions apart), with its windows,
span and supported time as the cause; the shortest identification under
each rule (on the grid, the shortest length with any confident query
and the shortest with all of them; on every panel, the shortest span and
supported time of a confident true segment); the per-second measures
below.

### Per-second scoring (step 3), defined now

Scored on detections as reports hold them, at a play level: detections
are grouped into plays as `plays::group` does, each play takes the
highest level of its segments under the rule being scored, and a second
is labelled with a play's file at the play's level when it lies inside
one of the play's segments (first to last hit), not in the gaps between
segments. Each whole second of the mix `[k, k+1)` is judged at `k +
0.5`.

- **Generated mixes** (exact truth: each play's start, end and whether
  its track is indexed). A label is right when its file is in the
  cluster of a track playing at that second (during a crossfade both
  tracks play, so either is right). Seconds while an indexed track
  plays: *mislabelled* if any confident label there is not right; else
  *correct* if a confident label is right; else *possible* if a
  possible label is right; else *unlabelled*. Seconds while no indexed
  track plays (held-out tracks, silence): *silent* if no confident
  label, else *mislabelled*; specificity is silent over these seconds.
  For each indexed play credited by a confident play of its cluster
  overlapping it, the start error is that play's first segment start
  minus the truth start, the end error its last segment end minus the
  truth end (median, mean absolute, 90th percentile of the absolute).
- **The development mixes**, borders from the manifests (a track starts
  at its `start`, ends at the next track's start, the last at the end of
  the file); a track is in the library when it is referenced and not
  left out. A margin of **M = 30 s** either side of each border, fixed
  now, is a transition: there both neighbouring tracks are right, and a
  second there is never counted unlabelled (it is reported apart as
  "in a margin, unlabelled"). Otherwise the same categories. Beside
  M = 30 s, M = 15 s and 60 s are reported, deciding nothing. Start and
  end errors only for Knowledge and Vertical, whose starts were not
  written from Gunfinger's output, and 2003-03-29's track 4 (AGENTS.md);
  against approximate starts they measure agreement, not accuracy.
- The shared-mixes renders (session 21) only if their reports hold each
  play's truth and time is left; the scoring is the generated mixes'.
- The existing scoring and every report stay as they are; the
  per-second scorer is a separate command that reads saved reports.

### Expected before the data, so it cannot be claimed later

A(10) makes the grid's 15 and 20 s plays confident wherever they have
240 hits (0020: all 160 of each, single pass) and cannot demote; under
A(15) and A(20) the 15 and 20 s plays mostly drop (0020: 12 and 9 of
160). B(S) behaves like A(S) on dense plays and is stricter on sparse
ones. The shared panels' related versions with 240 hits or more in 2
windows, if any, would be the new wrong answers; 0111 and 0129 list
possible plays up to 227 hits, and whether any reaches 240 in 2 windows
is not known.

## Decisions, continued (after the pre-registration, by time)

- **(00:43) Step 3a, the variant build**: worktree
  `/Users/razorjack/.t3/worktrees/gunfinger/s27-span`, branch `s27-span`
  at `d6a8f09` (never merged), binaries in `work/bin/s27-span/`.
  `Detection` gains `supported_seconds` (refine.rs: distinct
  `floor(t - first hit)` over every hit of the measured runs; 0 on the
  ladder), and the reports of `scan`, `mixes`/`grid`, `sweep`/`recall`,
  `robust` and `shared` carry it (`sweep`'s and `recall`'s also the
  first and last hit; `robust` keeps every detection at least possible
  in `found`). The worktree's unit tests were not updated (some test
  literals lack the new field), so its `cargo test` does not compile;
  the release build does, and V's identity with M's reports is the
  check. (Fixed at 02:48, `3589aa8`: the tests compile and pass, 443.) Its work directory `work/s27/vwork/` holds APFS clones of
  `work/{grid,mixes,sweep,robust,recall,shared,indexes}` and copies of
  the two clusters files and the library map. The queue
  (`work/s27/vqueue.sh`, `screen` `s27-vqueue`) started at 00:44.
- **(00:47) M's shared reports keep no `fitted` flag** (`SharedFound`);
  `span_rule.py` takes every shared detection as fitted (the default
  matcher keeps nothing above weak that it did not refine). With that,
  today's rule reproduces every stored level and the published outcomes
  (106 of 218, 5 of 135). A first version without it did not reproduce
  them and was not used.
- **(00:49) Session 24b resumed beside this session**, in its own
  worktree (`/Users/razorjack/.t3/worktrees/gunfinger/s24b-panako`,
  branch `s24b-panako`), to run Panako now that Java 11 is installed
  (the launching agent's message). It never writes to this checkout;
  its heavy runs are at nice 10 and end by about 06:30. Experiment 0150
  is reserved for it; this session numbers from 0136 and skips 0150.
  Its branch is not merged. Wall times measured tonight run beside its
  load, so they are not compared with earlier sessions'.
- **(00:55) Step 3, per-second scoring** (`gunfinger-eval seconds`,
  `crates/gunfinger-eval/src/seconds.rs`, 10 unit tests): reads saved
  `mixes` and `scan` reports, groups detections with the core's
  `plays::group` (each segment's file mapped to an id), levels each play
  by its best segment under `3 windows` or `--min-span S`, judges each
  whole second at its middle, and writes `work/seconds/<stem>.json`
  (`-margin-M` for scans). As pre-registered, with three details fixed
  while writing it, before any per-second count: a label is right when
  its file is in the cluster of a track whose interval (widened by the
  margin) holds the second; a second inside a margin counts as indexed
  when an indexed track's widened interval holds it; a label running
  past its own track's end (overshoot) is mislabelled time, as in the
  roadmap's sketch. Start and end errors for scans only with
  `--boundaries`, which this session passes for Knowledge and Vertical
  only; 2003-03-29's track 4 is left out too (conservative: one track,
  and the command has no per-track option). Supported time (B) needs the
  variant build's field, so B's per-second scores come from the variant
  worktree's own `seconds` (master merged into `s27-span`, a
  `--min-supported` rule added there), never from master.
- **(01:05) The shared-mixes renders**: session 21's report
  (`work/reports/library-nas-dnb-peaks/shared-mixes-s21.json`, 36
  rendered mixes, 87 runs, searched at 38,105 records) holds each mix's
  plan, so `seconds` reads it too (`shared_mixes`'s report structs now
  also deserialise; the runs on the three development mixes, which have
  no planned truth, are skipped). Scored under today's clusters (0128),
  variant A only: B would need a rerun, which the plan does not have.
- **(01:44) V's first runs.** Grid (00:44-00:52) and the second shared
  panel (00:52-01:14) are identical to master's reports apart from the
  added field (`span_rule.py identity`). The first shared panel
  (01:14-01:43) ran at 40,126 records, master's at 39,354, so they are
  not compared detection by detection (28,035 differences, mostly
  detections of files stored since); today's rule gives the same
  outcomes on V's run (106 of 218 wrong with the played recording left
  out). On it every variant but A(20) adds wrong confident answers:
  the 20 s `alone` queries of fire-2, fire-6 and pyro-2 (642, 703 and
  270 hits in 2 windows, spans 19.6-19.8 s, supported 20 s), so B(20),
  whose 20 whole seconds a 19.8 s span can hold, fails too. With A(20)
  and B(20) failing criterion 2 on the grid, no variant meets both
  criteria; the remaining runs are counted as planned and decide
  nothing further.
- **(01:57) A stale report among M's.** `work/reports/mixes-seed-7.json`
  was written on 2026-10-06 by the single pass (no detection is
  fitted), before ADR 0008 made the fitted matcher the default. Today's
  rule reproduced it because `span_rule.py` levels unfitted detections
  by the single pass's rule, so the reproduction check could not catch
  it. It is dropped from M: 0137's mixes row now counts seed 2026 alone
  (115 plays), and seed 7 comes from V (a fresh draw by today's
  `mixes`, 2 mixes, which differs from the old report's). Every other M
  report is today's matcher: every scan detection with 10 hits or more
  is fitted, the grid's are all fitted, and V reproduced the second
  shared panel's bit for bit. The per-second scores of the generated
  mixes are taken from V's reports (seed 2026 identical to M's).
- **(02:01) The listening files may change during the session**: the six
  questions of `work/audition/s23/` may be answered (`gunfinger-eval audition
  work/audition/s23/01-s23-pack.json`, from the
  main checkout; the launching agent's message). Their `same` and
  `different` answers append lines to `docs/pair-verdicts.txt` and
  records to `work/audition/answers.jsonl`. Those are the listening files: this
  session never commits, reverts, edits or stages either file, and
  leaves alone every process it did not start. The end check of the verdicts
  file now reads "changed only by the listening answers", with the diff's
  line count recorded. No count of this session depends on them: the
  clusters are not rebuilt and nothing is rescored with new verdicts.
- **(02:10) `robust`'s detections keep no `fitted` flag** either
  (`RobustFound`); the first count levelled them by the single pass's
  rule and failed reproduction (121 problems), so it was not used. Taken
  as fitted, like the shared panels', today's rule reproduces every
  stored level and count of V's `robust` run (0 problems).
- **(02:22) V's queue finished** at 02:21 with the recall panel's
  development half (522 of 522 indexed queries confident under every
  rule, no change). Master's NAS scans and recall report were written
  by older stores (recall: 33,587 indexed files against V's 40,114), so
  they are not compared detection by detection; V's own reports are
  levelled exactly by today's rule. Experiments 0138 (V's counts) and
  0139 (per-second scoring) close steps 3 and 4.
- **(02:28) The shared panels' changes by query length (0140).** Every
  outcome the variants change on both shared panels, but one under
  B(20), is in a query file of 20 s or less, which holds 2 windows and
  so is never confident under today's rule. Whether today's rule would
  name the same related versions confidently with the passage shifted
  off the window grid was not tested: `identify --store-only` builds
  its index from every record in the NAS store, and whether the store
  holds a record of the held-out test broadcast was not checked, as
  that would mean looking for it. The conservative option is to leave
  the direct test (a harness change in the variant worktree that pads
  the 20 s queries) to a later session. Criterion 1's result stands as
  written.
- **(02:30) Experiment 0141, pre-registered before its run.** 0140's
  inference is testable inside the harness, which searches the saved
  NAS index of the store's vetted records. The variant worktree gains,
  never merged, an `S27_SHIFTED_ONLY` switch: each scenario of a shared
  panel's development half is searched with only up to 20 s of its
  passage after 3, 5 and 7 s of the played file's own material (kind
  `before`, scenarios whose passage starts at least that far in), in
  the same three indexes, under `--work work/s27/vshift` so V's reports
  stay as they are. Readout, per panel and rule: outcomes with the
  played recording left out and in the everything index, and for the
  six queries of 0140, whether today's rule names the same related
  file confidently. Prediction: today's rule names it in most of the
  shifted queries, and the variants change little against today. It
  decides nothing about criterion 1, which stands; it shows
  how much of the protection on the 20 s queries is the file length.
- **(02:40) 0141's runs** took 02:30-02:38 (both panels, 63 shifted
  queries). The prediction held: for each of 0140's six wrong answers,
  today's rule names the same related file confidently in all three
  shifted queries; A(10)'s outcome on the short file equals today's
  rule's on the shifted queries in 74 and 70 of 78 pairs, today's own in
  29. Criterion 1's result is unchanged.
- **(03:01) The review's first finding, and experiment 0142,
  pre-registered before its run.** In 0141 the lead-in is the played
  file's own material, which the wrong versions of fire-2 and fire-6
  also match (their detections start at query time 0 and gain hits), so
  only pyro-2 tests the offset alone. 0142 repeats 0141 with digital
  silence of 3, 5 and 7 s before up to 20 s of each passage (the
  roadmap's "silence put before the same audio should not change its
  level"): worktree switch `S27_SILENCE_ONLY` (kind `silent`), `--work
  work/s27/vsilence`, both panels' development halves, every scenario
  (silence needs no material before the passage), the same indexes and
  readout as 0141. Prediction: today's rule names the same wrong file
  confidently in all three silence-led queries of each of 0140's six
  (642, 703 and 270 hits over 19.6-19.8 s reach a third window at these
  offsets), and on the short `alone` files A(10)'s outcome equals today's
  rule's on the silence-led queries in nearly every pair.
- **(03:06) The orchestrating thread**: session 28 waits on a clean root
  checkout. Finish 0142 and the review's fixes, then wrap up as planned;
  no new experiment after 0142; `docs/pair-verdicts.txt` stays unstaged
  (the orchestrator commits the six new lines afterwards).
- **(03:12) 0142's runs** took 03:01-03:09 (48 and 30 silence-led
  queries). The prediction held: today's rule names the same wrong file
  in all three silence-led queries of each of 0140's six (18 of 18), and
  also names Pyro's Ignorance for pyro-1 after 5 s, which the 20 s file
  never shows; A(10) and B(10) add pyro-1 after 3 s. Criterion 1's
  result is unchanged.
- **(03:12) The review's other fixes**: per-second tables give
  mislabelled seconds where nothing indexed plays (0139); 7 of the
  generated mixes' 8 mislabelled seconds are labels starting early;
  0141's conclusion limited to pyro-2; 0137's loop detection named as a
  third segment; the development mixes' border reading marked as an
  inference, with 2003-03-29's own-boundary caveat. Not changed, and
  said in the notes: `seconds`' boundary uses every confident segment of
  the track's plays that overlaps its slot, not only the first; its
  median takes the upper middle of an even count (`seconds_summary.py`,
  which the experiments use, averages the two); no unit test covers
  `set_truth`, `mix_truth` or a margin beside an unindexed neighbour.
  The queue scripts logged `date`'s exit status, not the run's; every
  report reproduces and no `.err` shows a failure.
