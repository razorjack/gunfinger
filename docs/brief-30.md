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
