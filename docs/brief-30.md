# Session 30 brief: what exact hash quantisation loses after fitting

Written by the agent for itself on 2026-10-17, so the plan survives
context compaction. Work autonomously and never stop to ask. When
something is ambiguous, take the conservative option, write the decision
down and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-30-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 28" to the end).
4. `docs/status.md` (newest entries at the bottom).
5. The roadmap's "Evidence beyond exact pair hashes"; experiments 0054,
   0055, 0056, 0058, 0059, 0143-0145 and this session's (0149, then 0151
   and up); `crates/gunfinger-core/src/hash.rs`,
   `crates/gunfinger-core/src/search/refine.rs`,
   `crates/gunfinger-eval/src/verifier.rs`,
   `crates/gunfinger-eval/src/recall.rs`.
6. The worktree `s30-quant`
   (`/Users/razorjack/.t3/worktrees/gunfinger/s30-quant`, merged into
   master once session 29 has finished) and this session's scripts
   (`scripts/analysis/quantisation.py`, `work/s30/*.sh`).

`docs/brief-2.md` to `docs/brief-29.md` and `docs/brief-24b.md` hold
rules that still apply unless this brief changes them (among them: edit
no configuration file; test fixtures are generated, never copied from
the corpus or the NAS; heavy runs one at a time; NAS-scale harness runs
overwrite `work/reports/library-nas-dnb-peaks/`, so copy it first).

## The prompt, verbatim

```text
Start session 30 of Gunfinger, a short morning session: measure what exact hash quantisation loses after fitting (the roadmap's "Evidence beyond exact pair hashes", step 2) on tonight's hardest conditions. Session 28 found that combined damage leaves about a tenth of the hits, so almost nothing reaches 240, and that other rips have less than half the hits of the recording's own record. This session asks how much of that lost evidence lands in neighbouring hash values, where a tolerant count could recover it. It also asks whether the same tolerance raises false and related candidates as much. It runs unattended from about 07:55 with a planned end at about 10:50. The end is soft: finish a run or review in progress rather than kill it, but plan to end by then. The results are due at about 11:00. The NAS is mounted at /Volumes/atlas. Work autonomously and never stop to ask. When something is ambiguous, take the conservative option, write the decision down and continue.

Context.
- The roadmap's "Evidence beyond exact pair hashes" (docs/roadmap.md, "Matching"): step 1, the peak verifier, was done in session 7 (`gunfinger-eval --verify`, crates/gunfinger-eval/src/verifier.rs; experiments 0054, 0056, 0058). At equal hits its share separated no better than hits, and worse against related records. Step 2 has never been measured: "on the same alignments, count exact pair hashes, pair hashes that allow a neighbouring value in each component (as Panako 2.0's near-exact hashing does, Six 2021, §2), and the verifier's peaks, each correspondence once. Stay within the candidate's reference passage, so that accuracy is not mixed with the cost of tolerant lookups over the whole index." The roadmap adds: "steps 2 and 3 matter more for recall than for separation ... Measure those copies and the shared-material scenarios (experiment 0055) together, since both would gain."
- Session 28 (experiments 0143-0145): recall on the NAS recall panel's development half (docs/panels/recall-seed-2026.json) at 40,126 records. Combined damage (c): 5 of 522 confident, 428 possible or better, misses for too few hits. Other rips only (a): 175 of 180. All three conditions together: 0 of 360 confident, 107 possible. No wrong confident answer anywhere. `recall` has `--other-rips-only` and `--queries clean,damaged,brief,brief-damaged`; the renders are rebuilt from the seed.
- Session 29 (experiments 0146-0148) measured the minimum span's fair test. It is finishing in this same root checkout until about 08:30; see "Sharing the checkout" below. This session numbers its experiments from 0149, then 0151 and up if it needs more (0150 is session 24b's), unless session 29 has used 0149 by then; check `docs/experiments/` after it finishes.
- State at the start: check git once session 29 has finished (clean on master after its last commit; record the hash); the NAS store at 40,126 records, revision fed26c76b8fe89ea (record it; if it changed, record the change and carry on).

Sharing the checkout with session 29. Until the orchestrating thread sends you a message saying session 29 has finished: read (First, item 1), then work in a new worktree `s30-quant` at /Users/razorjack/.t3/worktrees/gunfinger/s30-quant, branched from master's current commit, with its own target directory. There, on branch s30-quant, write and commit the brief, the checklist and the pre-registration (so it is dated before any count), and develop and commit the harness code. Do not run `git add`, `git commit` or `git stash` in the root checkout. Do not edit any tracked file there. Do not run `baseline`, a build in the root checkout, or any harness run that writes the root's work/reports/. Never touch session 29's processes or its work/s29/, work/bin/s29-* folders and worktrees. If no message has arrived by 08:50, check yourself: session 29 has finished when the last root commit is its wrap-up, `git status` is clean, and no process runs from work/bin/s29-*. Then go on in the root checkout as First, item 3 says.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it), docs/brief-29.md and docs/session-29-checklist.md (brief-2 to brief-28 and brief-24b hold rules that still apply unless this prompt changes them), docs/session-notes.md ("Session 28" to the end), docs/status.md (newest entries at the bottom), the roadmap's "Evidence beyond exact pair hashes", experiments 0054, 0055, 0056, 0058, 0059 and 0143-0145, crates/gunfinger-core/src/hash.rs, crates/gunfinger-core/src/search/refine.rs, crates/gunfinger-eval/src/verifier.rs and crates/gunfinger-eval/src/recall.rs.
2. Write docs/brief-30.md (this prompt verbatim, the state at the start, the plan with times) and docs/session-30-checklist.md on the s30-quant branch, point CLAUDE.md's reading list at them (keep brief-29 among the older briefs), and commit there.
3. After session 29 has finished, in the root checkout and in this order: build master as session 29 left it with --release the way sessions 27-29 did (Homebrew's 1.98.0 on the default PATH), copy the binaries to work/bin/s30-start/, and run `gunfinger-eval baseline session-30-start` (record its wall time); then merge s30-quant into master (fast-forward if possible; resolve any conflict keeping both sides whole). The start build therefore has none of this session's code.

Steps:

1. Setup (about 20 min, overlapping session 29's end): the steps above.

2. Pre-registration (about 20 min), written into the brief and an experiment file before any count, and committed:
   - The counts, per detection, on the alignment the second pass fitted (the verifier's per-window alignment): E, exact pair hashes, as the second pass counts them; N, the extra pairs that match when each quantised component of the pair hash (read hash.rs and name them) may differ by one step, each query pair and each reference pair counted once, with no pair counted in both E and N; V, the verifier's reference peaks found (its ±1 bin, ±1 frame). Only within the candidate's reference passage, with no tolerant lookup over the whole index.
   - The candidate sets: (T) true detections under hard conditions: the recall panel's development half under (c) combined damage, and under (a) other rips only, at NAS scale; (F) unrelated false candidates from the same runs: wrong possible detections and the strongest candidates on held-out queries; (R) related recordings: the shared panels' development halves' wrong related detections (the Fire versions, Pyro's, and the rest), and 0055's shared-material scenarios if cheap. Say which runs supply each set. The validation halves stay unsearched.
   - The measures: N/E and V/E per set, by hit band; how many (T) detections under 240 hits would reach 240 with E + N, and how many (F) and (R) would; separation at equal E (for example the AUC of N/E between T and F, and between T and R). None of this proposes a rule.
   - Declare in advance what would count as a notable result. For example: N recovers a large share of the hits lost under (c) for T, but much less for F and R. Or: N grows R as much as T, which would make tolerant hashing a risk for shared material.
   - Shrinks: (a)'s set first, then 0055's scenarios. (c) and the shared panels do not shrink.

3. The diagnostic (about 45 min): in gunfinger-eval only, beside the verifier (for example an option that adds E, N and V to the detections it already verifies, or a subcommand that reads saved reports and re-renders the queries from the seed). Unit tests: a pair one step away in each component counts in N and not in E; no correspondence counts twice; a reference pair outside the passage never counts. The product (crates/gunfinger, gunfinger-core) does not change. Commit the code before the runs.

4. The runs and the count (about 40 min): one heavy run at a time, under `nice -n 10` , detached under `screen` for anything that may last more than about 20 minutes. An experiment file of at most 40 lines. Commit.

5. A read-only review by a subagent; fix what it finds.

6. Wrap-up (from about 10:25): a "Session 30" section at the end of docs/session-notes.md. Say in plain terms how much of the evidence lost under damage and other rips sits one step away, whether related versions gain as much, and what that means for the roadmap's next step. A tolerant second pass would be our decision and a new ADR; this session writes none. Also update docs/status.md, the checklist and the roadmap's bullet. Then run `regress session-30-start` (identical) with the final build, scripts/check.sh with rustup's 1.98.0 first on PATH and RUSTUP_TOOLCHAIN=1.98.0 (green; never install a toolchain), and record the NAS store's revision. Commit. Last, print what was done, the checks, and anything left.

Rules for this session:
- Planned end about 10:50, soft. Steps shrink as step 2 says. An unfinished step is reverted, or committed only in a working, tested state, and named in the notes.
- The product does not change: crates/gunfinger and gunfinger-core behave as at the start; `regress session-30-start` identical; scripts/check.sh green.
- Long runs: the Bash tool kills a background task at its timeout, 30 minutes by default. Every run that may last more than about 20 minutes starts detached under `screen` with its own log and an exit line, or gets an explicit timeout of up to 7,200,000 ms. After each run, check that its log ends with its exit line and holds no error.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or reports (work/reports/evaluation-*-scan-ed-rush-optical-essential-mix.json included), rescore nothing, and never run test evaluation 5. Never identify, decode or index EM_1999_02_07_-_Ed_Rush_Optical.mp3 at the NAS root. Run nothing on the candidate mixes of experiment 0065 (including the mixed CDs it lists and the NAS's sety/vertical.mp3).
- Unsearched: the recall panel's validation half and both shared panels' validation halves. The frozen panel files do not change.
- The five test-derived versions are never used to tune anything: Terminal 2, the Orders remixes, the Marky & Bungle Funktion, the Clear Skyz Break remix and 1996 VIP, and the Black Barrel Beachball.
- The NAS (/Volumes/atlas/Music/dnb) is read, never written (its .gunfingerignore included). The NAS peak store (~/.local/share/gunfinger/nas-dnb-peaks) is read-only: no `index`, `prune` or `--retry-skipped`. The duplicate clusters are not rebuilt.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt and work/audition/answers.jsonl do not change. Manifests do not change, including corpus/sets/nocny-trans-vertical-2002-08-23/tracklist.toml.
- AGENTS.md: no SIMD, no `unsafe`, optimise only from measurement, an experiment file of at most 40 lines written before the next experiment starts, evaluation JSON in docs holds file names only, a new ADR to change a decision, cross-platform code (ADR 0020), no filename- or tag-derived logic in the product.
- Out of scope (ADR 0017): report edits, review workflows, and features that fix tags or the library.
- corpus/ is private and never committed; nothing of Panako's is committed. Leave the s24b-panako, s27-span, s29-span and s29-tests worktrees and branches as they are.
- Leave alone every process this session did not start. Measure no speed.
- NAS-scale harness runs overwrite work/reports/library-nas-dnb-peaks/, so copy what you need first. Keep scratch data under a few GB; delete renders and index clones you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step. Check `git status` before every commit and never use `git add -A`. Do not push. No AI attribution in commits.
```

## State at the start of session 30

- 07:47. Session 29 still finishing in the root checkout: master at
  `36dc022` ("notes, status and roadmap ... first draft, before the
  review") with `docs/session-29-checklist.md` modified; its review was
  running (started 07:45), its `regress session-29-start` and
  `scripts/check.sh` already done (checklist).
- Experiments 0001-0148 and 0150; this session numbers 0149 (if session
  29 has not used it; checked again once it has finished) and 0151 and
  up. ADRs 0001-0020. Test set: 4 of 5 evaluations used (none in this
  session).
- The NAS peak store at 07:52: 40,126 records, revision
  `fed26c76b8fe89ea` (`work/s30/store-revision-start.txt`,
  `scripts/analysis/store_revision.py`), as the prompt says. The NAS is
  mounted at `/Volumes/atlas`.
- Listening files: `docs/pair-verdicts.txt` 82 lines, sha256
  `ad74a881ba9f...`; `work/audition/answers.jsonl` 60 records, sha256
  `1e1a98a8158d...` (`work/s30/listening-digests-start.txt`), as at session
  29's start. Both must be the same at the end.
- Processes this
  session did
  not start are
  left alone.
- Disk: 27 GiB free.
- Worktrees: `s24b-panako`, `s27-span`, `s29-span`, `s29-tests` and
  `work/probe-tree` (detached), all left alone. This session's worktree
  `s30-quant` was branched from `36dc022` at 07:50.

## Plan, with times

Start 07:47; planned end about 10:50, soft.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1a | reading; worktree `s30-quant`; brief, checklist, `CLAUDE.md` committed there; store revision, listening-file digests | 0:30 | 08:15 |
| 2 | pre-registration (this brief, experiment 0149), committed on `s30-quant` before any count | 0:20 | 08:35 |
| 3 | the diagnostic in `gunfinger-eval` (E, N and the chance levels beside the verifier's peaks; `recall` with `--verify`), unit tests, committed on `s30-quant` | 0:45 | 09:20 |
| 1b | once session 29 has finished (message, or my own check at 08:50): Homebrew release build of master to `work/bin/s30-start/`; `baseline session-30-start`; merge `s30-quant` | 0:15 | (beside 3) |
| 4 | the runs (`nice -n 10`, `screen`, one at a time): `recall --verify --queries clean,damaged`, `recall --verify --other-rips-only`, `shared` on both panels' development halves; 0055 if cheap; counts; experiment file | 0:50 | 10:10 |
| 5 | read-only review by a subagent; fixes | 0:20 | 10:25 |
| 6 | wrap-up: notes, status, checklist, roadmap; `regress session-30-start`; `scripts/check.sh`; store revision; commit; message | 0:25 | 10:50 |

Steps shrink as step 2 says: (a)'s set first, then 0055's scenarios.
(c) and the shared panels do not shrink.

## Decisions

(by time, as they are made)

- **(07:50) The worktree.** `s30-quant` at
  `/Users/razorjack/.t3/worktrees/gunfinger/s30-quant`, branched from
  master's `36dc022` (session 29's draft notes; its later commits touch
  documents only, so the merge should not conflict outside
  `CLAUDE.md`, `docs/status.md` and `docs/session-notes.md`). Its target
  directory is its own (`s30-quant/target/`).
- **(07:55) Where the counts live.** In `verifier.rs`, beside the peaks:
  each `Count` the verifier makes (aligned and at each chance shift)
  gains the pair counts, so every command that verifies (`shared`
  always, the others with `--verify`) reports them. `recall` gains
  `--verify` (it ignored it before): only detections the second pass
  counted are measured, since a detection with fewer than 10 first-pass
  hits keeps the ladder's evidence and has no fitted alignment. The
  verifier's reports go to `reports/verified/`, never over
  `reports/library-nas-dnb-peaks/`. The runs use their own work
  directory, `work/s30/vwork/`, with clones of the saved index, the
  recall excerpts and the reports the runs read.
- **(08:01) The diagnostic** (`aecdf93` on `s30-quant`):
  `verifier/pairs.rs` counts E and N; `verify` adds them to its aligned
  and shifted counts (`Count::pairs`); `recall --verify` measures the
  detections the second pass counted; `shared` reports gain a `fitted`
  flag. Eight new unit tests (one step in each component counts in N
  and not E, two steps in none; no correspondence twice; nothing outside
  the passage; the ±1 frame and nearest choice; the record's pairs are
  the index's; query pairs placed at their window's offset; the hash
  layout). `cargo clippy -D warnings` clean, 238 harness tests pass.
- **(08:03) The runs start from the worktree's build, before the
  merge.** Session 29 had committed `2c87dd6` and `18b6a95` (documents
  only: its brief, notes, checklist) and its root checkout was clean,
  with no process of its own; no message yet.
  The pre-registration names "master after the merge" as the build; the
  merged master's code will be `aecdf93`'s, since session 29 changes no
  code, so the runs use the worktree's release build (Homebrew's
  1.98.0) copied to `work/bin/s30-quant/` (`COMMIT` = `aecdf93`), checked
  after the merge with `git diff aecdf93 master -- crates/` (empty). They
  write only under `work/s30/`, never the root's `work/reports/`.
  Queue `work/s30/runs.sh` (`screen` `s30-runs`, log
  `work/s30/logs/runs.log`): R1, R3, R4, R2, R5.
- **(08:02) The run directory** `work/s30/vwork/` holds APFS clones of
  the saved index (`indexes/library-nas-dnb-peaks.index`), the corpus
  and NAS clusters and the library map (the files only, so that the test
  set's reports in `work/reports/` are not copied), and the recall
  excerpts (`recall/seed-2026/`). The damaged renders are made again
  there. Tags for `labels.py`: `work/s30/nas-tags.json`
  (`store_tags.py`, 40,126 records).
- **(08:14) One label overridden beside, not instead.** R1's clean set
  (08:03-08:11, the same detections as session 28's report apart from the
  new `verified` field) gives E/hits 1.00 for T(C), so the check holds.
  Its only unrelated wrong possible detections are 8 of Congo Natty's
  "Peace" (tagged artist "unkown") for his held-out "Revolution", which
  session 28 named as the same artist; `labels.py` cannot see it. The
  pre-registered labelling stays for the main counts; F without that
  file (and R(r) with it) is reported beside.
- **(08:33-08:36) Session 29 finished; start build, baseline, merge.**
  The orchestrating thread's message (08:34): session 29's last commit
  is `68dae5b`, its thread ended at 08:20, the tree clean; checked at
  08:33 (`git status` clean, no `work/bin/s29-*`
  process). Session 29 left experiment 0149 to this
  session (its commit `2c87dd6`). `cargo build --release` with
  Homebrew's 1.98.0 found `target/release/` up to date; copied to
  `work/bin/s30-start/` (`COMMIT` = `68dae5b`), byte-identical to
  `work/bin/s29-final/` (`gunfinger` and `gunfinger-eval`). `baseline
  session-30-start`: 0.33 s wall. Then `s30-quant` merged into master
  as `2f8f4c9` (not a fast-forward, since master had session 29's later
  commits; no conflict, `CLAUDE.md` merged with session 29's line).
- **Which code produced the counts.** Every run of this session uses
  `work/bin/s30-quant/gunfinger-eval`, built in the worktree from
  `aecdf93` before the merge (R1 started at 08:03, R3 at 08:17).
  `git diff aecdf93 2f8f4c9 -- crates/` is empty: the merged master has
  the same code, so the counts are those of master's code at
  `2f8f4c9`. From here on the work continues in the root checkout.
- **(09:23) The runs finished**, none shrunk: R1 08:03-08:17, R3
  08:17-08:51, R4 08:51-09:15, R2 09:15-09:21, R5 09:21-09:23, each exit
  0; their stderr holds only FFmpeg's "Error reading frame GEOB, skipped"
  (a tag frame), no error of the runs. R1's and R2's detections equal
  session 28's reports apart from the verifier's field. R3's report
  differs from the root's copy of 2026-10-16 08:15 by 1-2 hits on some
  strong detections and in weak detections of other files; that copy
  predates the store's and index's present state (an inference from its
  date), so R1 and R2 are the build check.
- **(09:20) The second shared panel's roles.** Its frozen file lists each
  family's recordings, not directions, so `shared_outcomes.roles_of`
  cannot read it; `quantisation.py` takes a scenario's family by name
  and counts its other recordings as `version`. The per-search view (a
  related file confident today, or with E + N at 240 or more in 3
  windows) and the counts within (c) alone are exploratory: 0149 does
  not name them.
- **(09:29-09:41) The review** (a separate agent, read-only): no error in
  the counting code, no product change, every sampled number
  recomputed. Fixed after it: two unit tests rewritten (the nearest
  choice now fails under a first-in-order rule; the record's pairs are
  compared with an `Index`'s postings); the within-(c) AUCs, the
  per-panel split, the derived recovery shares (also for other rips:
  0.74 of the exact pairs lacking, N 1.10 of the own record's) and the
  not-confident R set computed by `quantisation.py` and in the data
  file; 0151, the notes, status and roadmap reworded where they went
  beyond the data (see the notes' "What else to know").
- **(09:29-09:36) Final checks** at `ff47fec`/`315529c`: `regress
  session-30-start` identical with `work/bin/s30-final/` (156 s);
  `scripts/check.sh` green (466 Rust tests, 5 Node tests, clippy for
  macOS and Linux). After the review's test-only changes the release
  binaries are byte-identical to `s30-final`; `scripts/check.sh` ran again
  (`work/s30/logs/check-after-review.log`).
- **(09:29) The end state**: NAS store 40,126 records,
  `fed26c76b8fe89ea`; listening files unchanged.

## Step 2: pre-registration (written at 07:52-08:00, committed before any count)

Everything below is fixed before the diagnostic is written or any count
is made. Nothing may be added after a count is seen; anything computed
later and not named here is exploratory and decides nothing. Nothing
here proposes a rule; a tolerant second pass would be our
decision and a new ADR.

### The counts, per detection

On the alignment the verifier rebuilds for the detection (the straight
line through the detection's first and last aligned hits, each 10 s
window moved by the mean of its densest 2-frame run of the asset's hash
hits; `verifier.rs`, unchanged), with the query analysed at the
detection's speed and playback over its span plus 1 s either side, as
the verifier does:

- **Query pairs**: the query's pairs (`hash::for_each_pair`, fan-out 2)
  whose anchor lies within the detection's span. **Reference pairs**:
  the asset's own record's pairs, formed as the index forms them (anchor
  frame rounded), whose anchor lies within the aligned reference
  passage (the span the verifier counts peaks in). Nothing is looked up
  in the index for E and N, so no tolerant lookup touches other assets.
- The pair hash's three quantised components (`hash.rs`): the **anchor
  level** (the anchor's bin in whole bins below the knee at 50 bins,
  in 2% relative steps above; 8 bits), the **bin delta** (target bin
  minus anchor bin, rounded to whole bins; 7 bits) and the **frame
  delta** (target frame minus anchor frame, rounded to whole frames;
  6 bits).
- **E**: a query pair and an unused reference pair with the same hash,
  the reference anchor within ±1 frame of where the window's alignment
  places the query anchor (a 2-frame span, as the second pass's densest
  run). Query pairs in anchor order; each takes the nearest such
  reference pair; each query pair and each reference pair counts once.
- **N**: then, among the pairs E left unused, a query pair and a
  reference pair whose hashes differ but whose three components each
  differ by at most one step (26 neighbouring hashes), at the same
  ±1 frame, matched the same way. No pair counts in both E and N.
- **V**: the verifier's reference peaks found (±1 bin, ±1 frame), with
  the passage's reference peaks P (V/P is 0058's share).
- **Chance**: E, N and V again with the reference shifted by -6.2,
  -5.0, +5.0 and +6.2 s (the verifier's shifts); their mean is the chance
  level. At chance, N/E should be near 26, the number of neighbours.
- **Check of E**: E is counted on the verifier's analysis, not the
  second pass's own, so it is compared with the detection's hits: the
  median E/hits per set is reported; outside 0.8-1.1 it is said, and
  hits + N is reported beside E + N.

### The runs and the sets

Build: master after the merge (this session's code), Homebrew's 1.98.0,
`work/bin/s30-quant/`. Each run under `nice -n 10`, one at a time, under
`screen` with a log and an exit line, `--work work/s30/vwork
--other-peaks-dir ~/.local/share/gunfinger/nas-dnb-peaks` (40,126
records), default matcher and rule, 0128's clusters.

- **R1**: `--verify recall --seed 2026 --queries clean,damaged`: C (the
  clean 30 s excerpts) and (c) combined damage, 702 queries each.
- **R2**: `--verify recall --seed 2026 --other-rips-only --queries
  clean`: (a), 702 queries, 180 observable.
- **R3, R4**: `shared docs/panels/shared-material-seed-2026-development.json`
  and `shared docs/panels/shared-material-2-seed-2026-development.json`:
  the panels' own queries in their 2 or 3 indexes, as sessions 25 and 26.
- **R5** (if cheap): `shared work/s7/shared-material.json` without the
  NAS store (262 tracks): 0055's four scenarios.

Only detections the second pass counted are measured (the `fitted`
flag; at least 10 first-pass hits).

- **T(c)**: in R1's (c) report, for each searched query, its strongest
  fitted detection of its own cluster (522 queries). **T(a)**: the same
  in R2 for the 180 observable queries. **T(C)**: the same in R1's clean
  report, the reference for what damage and other rips lose.
- **F**: in R1 and R2, every fitted detection outside the own cluster,
  not the damage partner's, whose file `labels.py` calls unrelated to
  the source (tags from the NAS store, `store_tags.py`). Named within
  it: wrong possible detections (60 hits or more), and the strongest
  fitted detection of each held-out or unobservable query.
- **R**: in R3 and R4, every fitted detection of the related recording
  (role `related`, and `shared_outcomes.py`'s `version` files with
  0128's clusters), every query kind and index; the confident ones (the
  Fire versions, Time Is Broken, Ignorance) apart. Beside it: **R(r)**,
  the wrong detections of R1 and R2 that `labels.py` calls related
  (same name, version or same artist); **P**, the played recording's
  fitted detections in R3 and R4 (the same queries' true answers); and
  0055's related and played detections from R5.
- The blend partner's and damage partner's detections are in no set.

### The measures

- M1: per set, by hit band (10-59, 60-119, 120-239, 240 and more): the
  median N/E, (E + N)/E, V/E, V/P, and the chance-corrected N (N minus
  its chance level) over E.
- M2: what N recovers of the evidence that damage and other rips lose:
  per source and speed, with T(C)'s E as the reference: the median of
  E/E(C) and of (E + N)/E(C) for T(c) and for T(a).
- M3: T(c) and T(a) detections under 240 hits that reach 240 with E + N
  (also with hits + N, and with chance-corrected N); the same count for
  F, R(r) and the not-confident R.
- M4: separation at equal E: within E bands 10-29, 30-59, 60-119 and
  120-239, the AUC of N/E and of the chance-corrected N/E between T(c)
  and F, T(a) and F, T(c) and R, P and R; pooled over the bands (pairs
  only within a band). Beside them the same AUC of V/P (0056's measure).

### Expected, written before any count

- T(C) clean: N/E about 0.1-0.3 (peaks shifted by speed fitting and
  re-encoding); E/hits near 1.
- T(c): damage removes peaks and adds the partner's, which changes which
  targets an anchor pairs with; tolerance recovers only quantisation, so
  N/E of 0.2-0.6 and (E + N)/E(C) well under 0.5; few of the 517 reach
  240.
- F: near chance, N/E far above T's (up to about 26), so tolerance
  raises unrelated false candidates more than true ones at equal E, and
  the AUC of raw N/E between T and F below 0.5; chance-corrected, near
  0.5.
- R: shared material is the same audio in the passage, so N grows R
  about as much as P and T at equal E (AUC near 0.5).

### Notable, declared before any count

- **Recall without risk**: T(c)'s median (E + N)/E at least 1.5 while
  F's and R's chance-corrected (E + N)/E in the same E band is at most
  1 + half of T(c)'s gain.
- **A risk for shared material**: R's median (E + N)/E in an E band at
  least T(c)'s or P's in the same band.
- **Reaching the rule**: at least 10% of T(c)'s detections under 240
  reach 240 with E + N; or any F detection, or any not-confident R or
  R(r) detection, reaches 240 with E + N.
- **Other rips**: T(a)'s median (E + N)/E(C) at least 0.75.
- **Separation**: any pooled AUC of chance-corrected N/E at least 0.75
  or at most 0.25.
- **The check**: median E/hits outside 0.8-1.1 in any set.

### Shrinks

(a)'s set (R2) first, then 0055's scenarios (R5). (c) (R1) and the
shared panels (R3, R4) do not shrink.
