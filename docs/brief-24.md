# Session 24 brief: a diagnostic version verifier for shared material

Written by the agent for itself on 2026-10-16, so the plan survives
context compaction. Work autonomously and never stop to ask. When
something is ambiguous, take the conservative option, write the decision
down and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-24-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 20" to the end).
4. `docs/status.md` (newest entries at the bottom).
5. `docs/research/2026-10-16-version-verification.md` (this session's
   research note, from step 2 on); ADRs 0003, 0006, 0013, 0017, 0019;
   experiments 0110-0123 and this session's; `docs/panels/`.
6. `docs/brief-20.md` (the panel's oracle), `docs/brief-21.md` (the
   rendered mixes' oracle and denominators) and `docs/brief-23.md` (the
   true plays of the development mixes under the fresh clusters).
7. `work/research/gpt-pro-version-verification/` (`prompt.md`,
   `gpt-pro-response.md`; not committed).

`docs/brief-2.md` to `docs/brief-23.md` hold rules that still apply
unless this brief changes them (among them: edit no configuration file;
test fixtures are generated, never copied from the corpus or the NAS;
runs longer than 30 minutes start detached under `screen`; heavy runs one
at a time; NAS-scale harness runs overwrite
`work/reports/library-nas-dnb-peaks/`, so copy it first).

## The prompt, verbatim

```text
Start session 24 of Gunfinger, an overnight session: build and measure a diagnostic version verifier for shared material, as an external research report proposes. It runs unattended from about 18:50 with a planned end at 04:30. The end is soft: if a run or a review is in progress at 04:30, finish it rather than kill it, but plan the work to end by then. A second session (25) continues from about 04:30 to 11:00 on what this one leaves, so leave everything committed and a handoff list in the notes. The NAS is mounted at /Volumes/atlas. Work autonomously and never stop to ask. When something is ambiguous, take the conservative option, write the decision down and continue.

Context.
- Shared material (sessions 20-23, experiments 0110-0123). When the version played is not indexed, a related version (remix, VIP, edit) becomes a confident wrong title in 106 of 218 queries of the shared panel's development half (0111, unchanged at the new revision in 0122). Rendered mixes: 87 wrong related plays (0116, 0121). Every candidate rule so far failed on the true plays of the three real development mixes: hit sparseness (0116, 0118) rejected too many, and the ambiguity note (0123) flags 15 of 108 true plays (13.9%) against a 5% limit. Both treat every stretch without hash hits as evidence against the reference, but DJ EQ (bass kills), blends and short cuts also silence hashes.
- GPT Pro was asked how to test, along a fitted alignment, whether the mix still plays the reference's content where hashes are silent. The prompt and the verbatim response are in work/research/gpt-pro-version-verification/ (`prompt.md`, `gpt-pro-response.md`; work/ is not committed). Its recommendation: a reference-conditioned compatibility test with three local states, supported, contradicted (the reference's content should be audible here under a plausible gain/EQ, and is not) and unobservable (masked, killed or faded), plus a separate measure of version distinctiveness (a supported shared break does not support the exact title). First method: dense band-wise spectral dynamics along the alignment (log band energies, their temporal differences) with a constrained gain/EQ fit estimated on some material and tested on other material; the reference rendered onto the mix clock with the continuously fitted speed; phase coherence only as positive evidence; NMF and chroma as optional. Evaluation: precision first, retained true plays at a fixed wrong-version rate; 218 queries cannot show error rates below about 1.4%. Its citations and numbers are unverified.
- The launching agent's assessment, for you to check: the idea addresses why sparseness failed, and is worth a diagnostic session. Two caveats the report underplays: (1) the verifier needs the reference audio, which the peak store does not hold (ADR 0003), so in the product it would not work for --store-only searches; that, and any storage of dense features, is our decision later, not this session's; (2) the binding test remains the real true plays of the development mixes, not rendered ones. Tonight's `holders` lookup (session 23) already says which passages of a reference other indexed files hold; that is a basis for distinctiveness.
- No listening was done since session 23: the six questions in work/audition/s23/ and the Sonar chain (0120's cut join) stay open. Plan nothing that waits on a judgement by ear, and never answer a question.
- Leave alone every process this session did not start.
- State at the start: git clean at 53d4136; the NAS peak store at 39,354 records, revision d71ba5333cff9a34 (check it); the rebuilt NAS clusters of 0120 are current.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it), docs/brief-23.md and docs/session-23-checklist.md (brief-2 to brief-22 hold rules that still apply unless this prompt changes them; brief-20, brief-21 and brief-23 matter most), docs/session-notes.md ("Session 20" to the end), docs/status.md (newest entries at the bottom), docs/research/2026-10-14-matching-proposals.md (the format of a research note), ADRs 0003, 0006, 0013, 0017 and 0019, experiments 0110-0123, docs/panels/, and both files in work/research/gpt-pro-version-verification/.
2. `git status` should be clean. Write docs/brief-24.md (this prompt verbatim, the state at the start, the plan with times) and docs/session-24-checklist.md. Point CLAUDE.md's reading list at them; keep brief-23 among the older briefs. Commit.
3. Build with --release and copy the binaries to work/bin/s24-start/. Run `gunfinger-eval baseline session-24-start` and record its wall time against session 22's `regress` (153 s) as a check of the machine's speed. If it is more than twice as slow, spend at most 20 minutes looking for a cause without touching processes you did not start, write down what you found, and continue.

Steps:

1. Setup (until about 19:30): the steps above.

2. The research note (about 30 min): docs/research/2026-10-16-version-verification.md in the format of the 2026-10-14 note: status (unverified; nothing here authorises a product change), provenance (the GPT Pro request, the files in work/), each method in a few lines with what the report claims, "Local check" where code reading or arithmetic can check it, the caveats above, and which parts this session tests. Do not fetch the web; the citations stay unverified. Commit.

3. Design and pre-registration (about 1 h), written into the brief and the experiment file before any count on the evaluation sets:
   - The verifier: its inputs (mix, reference, fitted offset and speed, the play's span and segments); how the reference is put onto the mix clock (resampling with the fitted speed; a small smooth timing correction only if needed, shared across bands); the bands, hop, patch length (about 1-3 s) and step (about 100-250 ms); the band-dynamics comparison; the constrained gain/EQ model (a small smooth basis across bands, bounded, slowly varying in time, fitted on one part of the material and tested on another, so it cannot explain away every mismatch); the rule for each patch's state (supported, contradicted, unobservable); distinctiveness (the reference's regions other indexed files hold, from `holders` and `pair` alignments, so that evidence in shared regions is kept apart).
   - At most three per-play summaries, declared before any count.
   - The split. Calibration, the only data used to choose any threshold: the shared panel's development half (the `shared` runs of 0111 and 0122: the wrong confident queries and the positive controls), and sweep renders if needed. Evaluation, never used to choose anything: E1, the 108 true plays of the three real development mixes (session 23, step 4c, new clusters); E2, the rendered mixes' 87 wrong related plays and their true drawn plays (0116 under 0121's clusters). Say where each set's plays and audio come from.
   - The criteria, fixed before evaluation: E1, at most 5 of the 108 true plays rejected; E2, at least half of the 87 wrong related plays rejected. Report everything else (the calibration set, the true drawn plays of E2) beside them, deciding nothing.
   Commit before step 5's runs.

4. Implementation (until about 00:30): the verifier in gunfinger-eval (a command reading a list of plays and writing a timeline per play: per patch the band scores, the fitted gain/EQ, the state and whether the region is distinctive, plus the summaries). Unit tests on synthetic signals: the same recording, an EQ'd copy, a bass-killed copy, a copy blended with other audio, a different arrangement sharing one part. No change to crates/gunfinger or gunfinger-core behaviour (a public helper in core only if needed, with no behaviour change); no SIMD, no `unsafe`; cross-platform (ADR 0020). Smoke runs only on calibration items (a positive control and one of Fire's wrong plays).

5. Calibration (about 1 h): run on the calibration set, choose the thresholds there, write them down and commit them before step 6.

6. Evaluation (until about 03:00): E1 and E2 against the criteria. List every rejected true play of E1 with why (bass kill, blend, cut, another rip, a real version difference) and the timelines of a few examples of each kind. Experiment file(s), at most 40 lines each, written before the next experiment starts.

7. If time remains before the review: the report's controlled test on two calibration families: the same shared passage with 0, 5, 10 and 20 s of the version's distinctive material, audible, attenuated and cut away, rendered with the harness. Otherwise leave it to session 25 with its plan written down.

8. A read-only review by a subagent, starting by 03:30; fix what it finds.

9. Wrap-up (from about 04:00): a "Session 24" section at the end of docs/session-notes.md (what the verifier does, the criteria and the numbers, the rejected true plays and why, what it would mean for the product and what we would have to decide, the handoff for session 25); update docs/status.md, the checklist and the roadmap. `regress session-24-start` identical with the final build. scripts/check.sh green with rustup's 1.98.0 first on PATH so the Linux clippy pass runs, as session 23 ran it; never install a toolchain (session 23's first attempt without RUSTUP_TOOLCHAIN began installing stable). The `decode::watch` tests are known to fail under heavy load (session 23): if only they fail, run that module again once the load is lower and record both; their fix is session 25's. The NAS store's revision unchanged. Commit. Last, print what was done, the checks, and anything left.

Rules for this session:
- Planned end 04:30, soft (see above). Steps shrink from the end: 7, then the examples in 6. An unfinished step is reverted, or committed only in a working, tested state, and named in the notes.
- The product does not change: crates/gunfinger and gunfinger-core behave as at the start; `regress session-24-start` identical; scripts/check.sh green.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or reports, rescore nothing, and never run test evaluation 5. Never identify, decode or index EM_1999_02_07_-_Ed_Rush_Optical.mp3 at the NAS root. Run nothing on the candidate mixes of experiment 0065 (including the mixed CDs it lists and the NAS's sety/vertical.mp3).
- The recall panel's validation half, the shared panel's validation half and the second shared panel (docs/panels/shared-material-2-seed-2026.json, both halves) stay unsearched: no run, score or look at their detections. The frozen panels do not change.
- The five test-derived versions are never used to tune or calibrate anything: Terminal 2, the Orders remixes, the Marky & Bungle Funktion, the Clear Skyz Break remix and 1996 VIP, and the Black Barrel Beachball.
- The NAS (/Volumes/atlas/Music/dnb) is read, never written (its .gunfingerignore included). The NAS peak store is read-only: no `index`, `prune` or `--retry-skipped`. Decoding NAS reference files for the verifier is reading. Check the store's revision at the start and the end.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt and work/audition/answers.jsonl do not change. Manifests do not change, including corpus/sets/nocny-trans-vertical-2002-08-23/tracklist.toml. The duplicate clusters are not rebuilt.
- AGENTS.md: no SIMD, no `unsafe`, optimise only from measurement, an experiment file of at most 40 lines written before the next experiment starts, evaluation JSON in docs holds file names only, a new ADR to change a decision, cross-platform code (ADR 0020), no filename- or tag-derived logic in the product.
- Out of scope (ADR 0017): report edits, review workflows, and features that fix tags or the library.
- corpus/ is private and never committed.
- Leave alone every process this session did not start, including system services.
- Runs longer than 30 minutes start detached under `screen`; heavy runs one at a time. NAS-scale harness runs overwrite work/reports/library-nas-dnb-peaks/, so copy what you need first. Keep scratch data under a few GB.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step. Check `git status` before every commit and never use `git add -A`. Do not push. No AI attribution in commits.
```

## State at the start of session 24

- 18:40. `git status` clean at `53d4136` on `master`. Experiments
  0001-0123 (the next is 0124); ADRs 0001-0020. Test set: 4 of 5
  evaluations used (none in this session).
- **The NAS peak store is not at the prompt's revision.** It holds
  40,126 records, revision `fed26c76b8fe89ea`
  (`work/s24/store-revision-start.txt`), against 39,354 and
  `d71ba5333cff9a34` at session 23's end check (11:38). 772 records are
  newer than that check, the newest written at 17:53 today; no
  `gunfinger index` process runs now, so I take it for an index run
  after session 23. 0120's clusters do not cover those 772
  files. This session's revision is the one above; the end check
  compares with it. The decision this leads to is under "Decisions"
  below.
- Processes this
  session
  did not start
  are left
alone, including another session's (a
  resumed session).
- Listening files: `docs/pair-verdicts.txt` sha256 `d48ffb313396...`,
  `work/audition/answers.jsonl` sha256 `ee2d5bd9fe0d...`, as at session
  23's end (`work/s24/listening-digests-start.txt`).
- Build: `cargo build --release` was current; `work/bin/s24-start/`
  holds `gunfinger` and `gunfinger-eval` byte-identical to
  `work/bin/s23-final/`.
- **`baseline session-24-start` copies the reports of the last `regress`
  (0.36 s); it searches nothing**, so its wall time says nothing about
  the machine. As the speed check I run `regress session-24-start`
  instead, the command whose 153 s session 22 measured (result under
  "Setup" below).

## Plan, with times

Start 18:40; planned end 04:30, soft.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1 | setup: brief, checklist, build, baseline, `regress` as the speed check, store revision | 0:50 | 19:30 |
| 2 | the research note `docs/research/2026-10-16-version-verification.md` | 0:30 | 20:00 |
| 3 | design and pre-registration: the verifier, summaries, split, criteria; experiment 0124's first part; committed | 1:00 | 21:00 |
| 4 | implementation: `gunfinger-eval versions` (name fixed in step 3), synthetic tests, smoke runs on two calibration items | 3:30 | 00:30 |
| 5 | calibration on the panel's development half; thresholds written and committed | 1:00 | 01:30 |
| 6 | evaluation: E1 (108 true plays), E2 (87 wrong related plays and the drawn true plays); rejected true plays with causes; experiment files | 1:30 | 03:00 |
| 7 | if time remains: the controlled test on two calibration families; otherwise its plan for session 25 | 0:30 | 03:30 |
| 8 | read-only review by a subagent, from 03:30 at the latest; fixes; `check.sh` during the review | 0:30 | 04:00 |
| 9 | wrap-up: notes, status, checklist, roadmap, `regress`, `check.sh`, store revision, commit | 0:30 | 04:30 |

Steps shrink from the end: 7, then the examples in 6.

## Setup (done at 18:46)

- `regress session-24-start` with `work/bin/s24-start/` (18:42-18:45):
  identical (720/720 sweep queries, all 87, 93 and 79 scan detections);
  153.45 s wall, 1,293 s user, load 30-34 at the end. Session 22's took
  153 s, so the machine runs at its usual speed; no search for a cause.

## Decisions (setup and step 3, written before any run of the verifier)

- **The store's revision.** The session takes the store as it found it
  (40,126 records, `fed26c76b8fe89ea`) and checks it at the end against
  that. No step searches the whole store: the plays come from saved
  reports (each scored as published), and the verifier reads only the
  reference files' audio and the peak records of a play's reference and
  of the files its run's report names as holders. So the 772 records
  newer than 0120's clusters reach no count. The clusters are 0120's
  (`work/s23/clusters-new.json`), not rebuilt.
- **One calibration run.** 0111's and 0122's `shared` runs are the same
  rendered queries with the same outcomes (0122); the calibration plays
  come from 0122's report (`work/s23/after/shared-material-seed-2026-development.json`,
  scored under 0120's clusters as session 23 scored it), so no play is
  counted twice.
- **Playback.** The saved reports do not record whether a detection came
  from a turntable or a key-locked rung. Every evaluation mix is a
  turntable mix, and the panel's queries play their files at native
  speed, where the two are the same; the verifier renders every
  reference as turntable playback (resampling).

## Step 3: the design and the pre-registration (written at 18:56, before any run of the verifier)

Everything below is fixed before the verifier runs on any calibration or
evaluation play. Nothing may be added after a count is seen; anything
computed later and not named here is exploratory and decides nothing.
Adopting anything is our decision; nothing changes in the
product.

### The verifier (`gunfinger-eval versions`)

- **Input**: a JSON list of plays, each with an id, the mix's audio
  file, the reference (a harness file name), the files of the
  reference's own recording, its *siblings* (below) and its segments
  (each: `start_seconds`, `end_seconds` in the mix, `track_start_seconds`,
  `speed`; file time = `track_start + speed x (t - start)`). A play's
  segments are the product's (ADR 0006): one file's detections at least
  possible, joined across gaps of at most 90 s. Only the segments' spans
  are tested; the gaps between them have no line.
- **The reference on the mix clock**: for each segment, the mix decoded
  from 1 s before to 1 s after the segment (8 kHz mono), and the
  reference rendered with FFmpeg as turntable playback at the segment's
  fitted speed (`render::render_samples`, as `identifiability` renders
  a line), starting at the file time of the decoded mix's first sample,
  silent where the file has no audio. The timing correction: each patch
  takes the lag within +-3 frames (+-48 ms), one lag for all bands, that
  gives the highest mean dynamics correlation over its informative
  bands; no other warping.
- **Bands**: the profile's STFT (1,024 points, hop 128, Hann; 16 ms
  frames); 20 bands with log-spaced edges over bins 5-500 (39 Hz-3.9
  kHz; edge i at bin `round(5 x 100^(i/20))`); a band's level `L` is 10
  log10 of its bins' summed power. Three EQ groups, as a DJ mixer's
  three bands: low = bands 0-7 (39-246 Hz), mid = 8-17 (246-2,465 Hz),
  high = 18-19 (2,465-3,906 Hz).
- **Patches**: 125 frames (2.0 s), every 16 frames (0.256 s), over each
  segment's span.
- **Band dynamics**: `D(b, i) = L(b, i) - L(b, i - 3)` (48 ms); `r(b)` is
  the Pearson correlation of the reference's and the mix's `D` over the
  patch. A band is *informative* when the reference's `D` has a standard
  deviation of at least 1 dB over the patch and the band's mean level is
  within 40 dB of the reference's loudest band in the patch. A cell
  (band, frame) is *salient* when the reference's level there is within
  15 dB of the band's maximum in the patch.
- **The gain/EQ model**: one gain per EQ group per patch (a piecewise
  constant curve across bands), never per band. *Anchors* are the
  informative bands with `r >= 0.6`: material that demonstrably
  survives. The patch's frames are cut into 0.256 s blocks, alternating
  between two folds. On each fold the gain of group j is the median of
  `L_mix - L_ref` over the salient cells of j's anchor bands; a group
  without anchors takes the patch's broadband gain (the same median over
  every anchor band) plus the play's typical offset of that group (the
  median of group minus broadband gain over the play's patches where the
  group has anchors; 0 if never). Bounded: each gain within -40 and +12
  dB of the play's typical broadband gain (the median over the play's
  patches with anchors). A band's residual `e(b)` is the median over its
  salient cells of `L_mix - L_ref - gain`, each cell with the gain
  fitted on the *other* fold (fitted on one part, tested on the other).
- **Band states** (informative bands of a patch with at least one
  anchor), tested in this order:
  1. *killed* (unobservable): its group has no anchor and every
     informative band of the group has `e < -10` dB: the whole group is
     absent uniformly, as under a kill or a fade;
  2. *supported*: `r >= 0.5`;
  3. *contradicted*: `e < -10` dB: the reference predicts at least ten
     times the power the mix holds in this band, while its group is not
     uniformly absent (a selective absence no DJ EQ makes);
  4. *masked* (unobservable): `e > +6` dB: the mix holds at least four
     times the reference's predicted power;
  5. *contradicted*: `r < 0.2`: the reference would be at least a fifth
     of the band's power and its dynamics do not appear;
  6. otherwise *undecided* (no evidence).
- **Patch states**: *unobservable* when the patch has no anchor (no
  gain can be established: a cut, a fade, total masking, or a reference
  that matches nothing there); otherwise with `s` supported and `c`
  contradicted bands, *contradicted* when `c >= 2` and `c >= s / 2`,
  *supported* when not contradicted and `s >= 3`, else *unobservable*.
- **Distinctiveness**: a play's *siblings* are the files outside the
  reference's own recording (0120's clusters, and the panel's recording
  list for panel files) that its run's report names as holders of its
  segments with at least 60 hits (session 21's (c) and the possible
  tier). The reference's stored peaks over the play's file span (10 s
  wider on each side) are searched on the turntable ladder against an
  index of the siblings alone (`search_peaks`, as `holders` searches);
  each detection of at least 30 hits (`LISTED_HOLDER_HITS`) holds the
  reference's span it covers. A patch is *distinctive* when the file
  time at its centre lies outside every held span. Where a report keeps
  only the strongest holder (0116), the siblings are that file alone.
- **Timeline**: per patch its time, file time, lag, the 20 band
  correlations and residuals, the three group gains, the band states,
  the patch state and whether it is distinctive; per play the
  summaries.

### The per-play summaries (three, declared now)

- **S1, the contradicted share**: contradicted patches / (supported +
  contradicted patches) over all the play's patches; undefined when no
  patch is decided. **The decision statistic**: a play is *rejected* when
  S1 is at or above the threshold T; a play with S1 undefined is kept.
- **S2, the distinctive contradicted share**: the same over distinctive
  patches only; reported, deciding nothing.
- **S3, distinctive support**: supported distinctive patches x 0.256 s;
  reported, deciding nothing (zero: no evidence for the exact title).

### The split

- **Calibration** (the only data used to choose T): the shared panel's
  development half, 0122's run. Negatives: the wrong confident plays of
  the *played left out* index (106 wrong confident queries; per play).
  Positives: the right confident plays of the *source left out* index
  (the played recording's other rip, the positive control; per play).
  Plays from the report's detections at least possible, joined across
  90 s, confident when a segment is, scored by session 20's oracle under
  0120's clusters (`shared_outcomes.py`, as session 23). Audio: the
  rendered queries in
  `work/shared/shared-material-seed-2026-development-83361e93ae0502ae/`.
  Siblings from session 23's holder lookups of these plays
  (`work/s23/after/holders-lookups-shared.json`). No sweep renders.
- **E1** (evaluation): the 108 true plays of the three development mixes
  (`second` in `work/s23/after/score/0116-criteria.json`; plays of
  `work/s23/after/shared-mixes-dev.json`, step 4c of session 23, credited
  by the verified scans under 0120's clusters). Audio:
  `corpus/sets/<set>/` (the manifests' `audio` files, read only).
  Siblings from the report's `holders` lists.
- **E2** (evaluation): session 21's rendered mixes (0116,
  `work/s21/shared-mixes-s21.json`) scored under 0120's clusters
  (`rows` of the same `0116-criteria.json`): the 87 wrong related plays
  of the *played left out* runs, and beside them the 480 right drawn
  plays of the same runs. Audio: `work/shared-mixes/shared-mixes-s21/`.
  Siblings: the report's strongest holder, where outside the reference's
  recording under 0120's clusters and at least 60 hits.

### The threshold and the criteria (fixed now)

- **T** is the largest value that rejects at least half of the
  calibration's negatives: with the negatives' S1 sorted from highest
  (undefined counted lowest), T is the `ceil(n/2)`-th value. If that
  value is 0 or undefined (fewer than half of the negatives have a
  contradicted patch), T is the smallest S1 above 0 among the negatives,
  and the calibration is said not to reach half. Reported beside it: the
  calibration positives kept at T. If they are under 95%, the
  calibration does not separate, and that is said; E1 and E2 run at T
  anyway. Why this choice: thresholds set to keep 95% of clean
  calibration plays failed on the real mixes (0116, 0118); the most
  lenient threshold that still rejects half of the wrong plays costs
  real true plays the least.
- **E1**: at most 5 of the 108 true plays rejected. **E2**: at least 44
  of the 87 wrong related plays rejected. Both are required to call the
  verifier worth pursuing; either way it is our decision what
  to do next.
- Reported beside, deciding nothing: S2, S3, patch-state shares, the
  calibration's counts at T, E2's drawn true plays rejected at T, the
  positive control of E2 (15 plays of the *source left out* runs). Every
  rejected E1 play is listed with the probable cause read from its
  timeline (bass kill, blend, cut, another rip, a real version
  difference), marked as an inference.
- Expected before the data, written so it cannot be claimed later: the
  panel's queries are clean renders, so calibration positives will have
  low S1 and T may be lenient for clean plays and strict for real ones;
  DnB's shared breaks carry most of a patch's dynamics, so a related
  version's patches will often be supported in drum-heavy bands, and the
  rejections must come from selective absences (basslines, stabs, pads).

### Amendments from the synthetic tests (written at 19:05, before any run on calibration or evaluation data)

Step 4's unit tests on synthetic tracks (drums shared, bassline and lead
per arrangement) showed two properties of the design that contradict
its stated intent. Both are changed now, before the verifier has run on
any calibration or evaluation play; nothing else changes.

1. **Kills are EQ-shaped, not whole-group.** With the bassline and kick
   removed (a bass kill), the top low band (156-188 Hz) still holds the
   snare's spectral leakage, follows the reference's dynamics and so is
   an anchor; the rule "killed only when the group has no anchor" then
   made every other low band contradicted (84 of 86 patches). A real
   DJ low kill leaves the bands near the crossover partly present in the
   same way. New rule: an informative band with a deficit (`e < -10` dB)
   and `r < 0.5` is *killed* (unobservable) when the deficit has the
   shape of a DJ EQ cut of its group: in the low group every informative
   low band below it also has a deficit (a low shelf); in the high group
   every informative high band above it does (a high shelf); in the mid
   group every informative mid band does, except possibly the group's
   lowest and highest informative bands (a mid cut leaking past the
   crossovers). Any other deficit is *contradicted* (a notch no DJ EQ
   makes). The order becomes: supported (`r >= 0.5`); a deficit, killed
   or contradicted by its shape; masked; `r < 0.2` contradicted;
   undecided. The patch rule is unchanged.
2. **Masked from +3 dB, not +6 dB.** In a blend at equal level, bands
   where the other record held up to three quarters of the power had
   low correlations and counted as contradicted (S1 0.29). The dynamics
   test can only speak where the reference is at least half of the
   band's power, so a band is masked when the mix holds at least twice
   the reference's predicted power (`e > +3` dB).

## During the session

- **19:10, from the launching agent**: a parallel session (24b) runs a
  Panako benchmark at 262 tracks in its own git worktree on branch
  `s24b-panako`, reading `corpus/` and `work/` by absolute path and never
  writing to this checkout. Its heavy runs (nice 10) end by 00:30, so
  extra load before then is expected; this session's calibration and
  evaluation runs after 00:30 have the machine. Its worktree and branch
  are not touched; session 25 merges them, so `master` stays linear and
  committed.
- **19:09, smoke runs** (`work/s24/smoke.json`, two calibration items,
  3.9 s): a positive-control play of Fire's original (S1 0, 108 patches
  supported) and the Rogue Unit Remix wrong over the original's passage
  (S1 0.083: 388 supported, 35 contradicted, 3 unobservable). In the
  wrong play the low bands are often contradicted (its bassline), but
  the patch rule keeps most such patches supported because the drum-led
  mid and high bands agree, the "single shared stem" case the report
  warns about; its lag sits at -2 to -3 frames (the bound) in most
  patches. Seen on calibration data after the pre-registration, so the
  design does not change; it is reported with the results.
