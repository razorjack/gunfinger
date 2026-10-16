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
