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
- **(23:31) The merge** (`2eee100`, a merge commit of `3a1420a`): two
  conflicts, `docs/session-notes.md` and `docs/status.md`, both
  sections appended at the end on each side; resolved by keeping every
  section whole in the order the sessions started (24 at 18:40, 24b at
  19:06, 25 at 20:44). The merged files hold every line of both sides
  (the merge adds exactly the branch's 1,346 lines, 2 changed). The
  roadmap and `devscore.py` (a `corpus` parameter with the old default)
  merged without conflict. 0130 and 0131 keep their numbers, so 24b's
  status note on renumbering no longer applies. No file under `crates/`,
  `Cargo.toml` or `Cargo.lock` changed; the release build after the
  merge is byte-identical to the start's (`work/bin/s26-merge/`).
  `regress session-26-start` with it (23:30-23:32): identical (720/720
  sweep queries, all 87, 93 and 79 scan detections, the calibrate line
  known since session 8), 155.2 s.

## Step 3: version 2's design and pre-registration (written at 23:35, before any run of version 2)

Everything below is fixed before version 2 runs on any calibration,
evaluation or side-set play. Nothing may be added after a count is seen;
anything computed later and not named here is exploratory and decides
nothing. Nothing here changes the product; adopting anything is our
decision.

### What changes from version 1, and why

Version 1 is `docs/brief-24.md`, step 3 with its two amendments.
Version 2 changes two things and nothing else.

**1. The line is measured from the audio.** Version 1 compares each
play along its reported segments' lines. 0127 found that line 98-193 ms
from the audio for all five rejected E1 plays (the opening-window
anchor; uploads whose speed drifts within a play), and 0132's rejected
positive 313-327 ms off where a play opens just before a loop point.
Version 2 measures each reported segment's line from the reference's
own hash hits over the segment's body:

- *Hits.* The mix excerpt the segment is compared on (its span and 1 s
  either side, as version 1 decodes it) is analysed as turntable
  playback at the segment's reported speed (`Rung::points`), and its
  hashes are looked up in an index of the reference's peak record alone
  (from the store, as `held_spans` loads records). Each posting is a hit:
  a mix time and a file time. Only hits inside the segment's span count.
- *Knots.* The segment is cut into 10 s windows of mix time from its
  start (the matcher's window). In each, the densest run of hits whose
  deviations from the reported line lie within 2 frames (32 ms; the
  matcher's line span), at any deviation, ties to the smaller absolute
  deviation, is a *knot* when it holds at least 10 hits (the matcher's
  `STRONG_LINE_HITS`): its mix time is the mean of its hits' mix times,
  its file time the reported line there plus the run's mean deviation;
  it keeps its first and last hit's mix times.
- *Stretches and jumps.* Knots in time order form stretches: a knot
  joins the current stretch when its deviation is within `0.02 x dt +
  2 frames` of the previous knot's (dt: their mix-time distance; 2% is a
  generous pitch ride, 2 frames the measurement's spread); otherwise the
  material jumped (a loop, a needle drop, the other alignment of 0127's
  Phantom Force, 0.4 s away) and a new stretch starts. A stretch of one
  knot in a segment with more knots is a *stray* and is dropped: one
  window cannot tell a short jump from a chance or repeated-section
  alignment. A segment's only knot is kept.
- *Pieces.* A stretch's line is the fewest straight pieces through its
  knots that keep every knot within 1 frame (16 ms) of its piece: the
  chord from the first to the last knot, split at the knot farthest
  from it while any knot is farther than 1 frame, each part fitted the
  same way (Ramer-Douglas-Peucker). A piece's speed is its chord's
  slope; a lone knot's piece takes the reported speed. This follows a
  speed that drifts (piecewise constant speed, continuous line) without
  a model of the drift.
- *Span.* A stretch is tested from its first knot's first hit to its
  last knot's last hit; the stretch's first piece starts there and its
  last piece ends there. Between stretches (around a jump) and outside
  the knots' hits nothing is tested: a line is never extrapolated past
  the reference's own hits. Between knots of a stretch the pieces
  interpolate, so stretches where hashes are silent (a blend, a kill)
  are tested, which is the verifier's purpose.
- *Comparison.* Each piece is verified exactly as version 1 verifies a
  segment: the reference rendered at the piece's speed from the store's
  timeline (`render_samples_as_indexed`), the mix's excerpt (cut from
  the segment's decoded excerpt), patches wholly inside the piece, one
  lag within +-48 ms per patch. The typical gains are the play's, over
  all its pieces.
- *No measurable line.* A segment without a knot, or with strays only,
  is not tested. A play with no tested patch has S1 undefined and is
  *unmeasured*: it decides nothing. It is not rejected, and it is
  counted apart from the plays kept with an S1, so an unmeasured true
  play is not claimed as supported and an unmeasured wrong play does not
  count as rejected.
- *Held spans* (S2, S3; deciding nothing) are searched over the tested
  pieces' file span, 10 s wider each side, as version 1 over its
  segments'.

**2. A band is contradicted only where its EQ group demonstrably passes
the reference.** In E2's Astro-Sine plays (0127) the incoming record's
bass was cut while the outgoing record's bass filled the six lowest
bands near the level the gain fit predicted; their dynamics were not the
reference's, so `r < 0.2` made them contradicted. Those patches had no
anchor in the low group: its gain was borrowed (the patch's broadband
gain plus the play's typical offset of the group), so nothing showed the
group open. Version 2: when a band's EQ group has no anchor in the patch
(no band of the group with `r >= 0.6`), a band of that group that is not
supported is *killed* when its residual is under -10 dB and *masked*
otherwise, never contradicted. In a group with an anchor every rule is
version 1's. So another record's material under a cut reads as masked,
a cut group as killed, and a contradiction needs the reference's own
material passing the same group of the mixer: a missing bassline under a
kick that plays is still contradicted.

Reported beside, deciding nothing: each patch's state as version 1's
band rules would judge it on the same line (`state_v1_rules`), so the
two changes' shares can be told apart without another run.

Everything else is version 1's: bands, groups, patches, dynamics,
informative and salient, anchors, folds, gains and bounds, the other
band rules and their order, the patch rule, silence, S1, S2, S3,
siblings, the renderer, the mix decoder.

### The split, T and the criteria

- **The plays** are session 24's and 25's files unchanged, siblings
  included, so version 1 and version 2 are measured on identical plays:
  calibration `work/s24/plays/calibration.json` (312 negatives, 75
  positives), E1 `work/s24/plays/e1.json` (108), E2
  `work/s24/plays/e2.json` (87 wrong related, 480 true drawn, 15
  positive control), the second panel `work/s25/plays/panel-2.json` (5
  negatives, 78 positives). The calibration negatives come from 0122's
  run under 0120's clusters, which score it identically under 0128's.
- **Calibration**: the first panel's development half only. T by
  version 1's rule: the negatives' S1 sorted from highest, undefined
  lowest, T is the `ceil(n/2)`-th; if that is 0 or undefined, the
  smallest S1 above 0 and "does not reach half". Reported beside: the
  positives kept at T (under 95%: "does not separate"; E1 and E2 run at
  T anyway). T is committed before any evaluation run.
- **Evaluation, a second use, declared now.** E1 and E2 were used for
  version 1 (session 24: 5 of 108 and 43 of 87), and their failures
  shaped version 2: the line answers the five E1 rejections that 0127
  traced on E1's own audio, the group rule answers an E2 drawn play's
  rejection (Astro-Sine). The criteria are version 1's, both required:
  **E1**, at most 5 of the 108 true plays rejected (S1 >= T); **E2**, at
  least 44 of the 87 wrong related plays rejected. Unmeasured plays are
  not rejected in either set.
- **What a pass would and would not show.** A pass on this second use is
  weaker evidence than a pass on session 24's first use would have been:
  version 2 was designed knowing which E1 plays failed and why, so a
  pass shows that it removes those failures without losing the wrong
  plays' rejections, not that it keeps true plays it has never seen. A
  first use needs real mixes no verifier has seen, with tracklists
  checked by ear (they cannot be added by a session), and wrong
  related plays from families never used to design or calibrate.
- **Beside, deciding nothing**: E2's true drawn plays and positive
  control rejected at T; the second panel's development half (5
  negatives, 78 positives) at T; per set, S1 quantiles, unmeasured plays,
  S1 with version 1's band rules on version 2's line; the measured
  lines' offsets from the reported ones (median of each play's knots),
  stretches, strays and pieces; every play whose decision differs from
  version 1's (session 24's and 25's runs at their T) and, for every E1
  play still rejected, why, read from its timeline and line (as session
  24 did), marked as an inference.
- **One run of version 2 per set.** No variant runs on E1, E2 or the
  second panel. A defect that stops plays from being read (as session
  24's render defect) is fixed and every set rerun, both runs reported.
- **Expected before the data**, written so it cannot be claimed later:
  the five E1 plays and the second panel's rejected positive have lines
  off by 98-327 ms and should be kept; Phantom Force's first window may
  form a stray. The group rule lowers contradiction where a low group
  has no anchor, in wrong plays too, so T may fall and E2's count may
  fall with it; Fire's wrong plays keep the kick in the low group, so
  most of their contradictions should stay. Dozer's, Pyro's and Sonar's
  wrong plays should stay unrejected (the same content along the line).
