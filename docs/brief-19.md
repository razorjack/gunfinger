# Session 19 brief: search speed, proposals 1 and 2 of the external review

Written by the agent for itself on 2026-10-14, so the plan survives
context compaction. Work autonomously and never stop to ask. When
something is ambiguous, take the conservative option, write the decision
down and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-19-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 17" to the end).
4. `docs/status.md` (newest entries at the bottom).
5. `docs/research/2026-10-14-matching-proposals.md` (the whole note);
   the roadmap's "Where search time goes"; ADRs 0008, 0011 and 0017;
   experiments 0078, 0088, 0093, 0096, 0097 and 0103.
6. `crates/gunfinger-core/src/search/lines.rs` (`OffsetSlots`),
   `spectrogram.rs` and `speed.rs`.
7. The newest file in `docs/experiments/`.

`docs/brief-2.md` to `docs/brief-18.md` hold rules that still apply
unless this brief changes them (among them: edit no configuration file;
test fixtures are generated, never copied from the corpus or the NAS;
runs longer than 30 minutes start detached under `screen`; heavy runs one
at a time; NAS-scale harness runs overwrite
`work/reports/library-nas-dnb-peaks/`, so copy it first; each timing
series records the load and the busiest processes).

## The prompt, verbatim

```text
Start session 19 of Gunfinger, a 3-hour session on search speed: proposals 1 and 2 of docs/research/2026-10-14-matching-proposals.md. It starts at about 17:30 and must finish by 20:30. Work autonomously and never stop to ask. When something is ambiguous, take the conservative option, write the decision down and continue.

Context.
- The research note records five unverified proposals from an external review (GPT Pro, relayed and reviewed by a GPT Astra agent, integrated by a Claude agent, 2026-10-14). We chose two for this session: proposal 1 (integer phases per query anchor for the pre-filter, exact output) and proposal 2 (FFT lengths of the turntable rungs, the diagnostic only). Nothing in the note has been measured yet; its local checks are code reading and arithmetic. Its protections, contracts, stop points and proposed criteria apply as written unless this prompt changes them.
- The tracklist item came first and is done (ADR 0019, experiment 0107, commit d867557). It changes no detection.
- The NAS peak store changed after session 18: it now holds 38,105 records, revision b285da9984422fb5 (checked at 17:20 with scripts/analysis/store_revision.py ~/.local/share/gunfinger/nas-dnb-peaks), against session 18's 33,723 (c835c5531c325274); new files were indexed. Session 17's and 18's NAS timings are not comparable with this session's. The CLI's saved index (~/.cache/gunfinger/indexes/nas-dnb-peaks-23935cfc56936b67.index, 4.3 GB) was written at 15:16; check with `doctor` that it is current for this revision, and if not, let one `identify` rebuild and save it before any timing. The harness's saved index (work/indexes/library-nas-dnb-peaks.index, 3.8 GB) was made at 33,723 records and is stale; you may delete it if you need the space. The disk has about 18 GB free.
- The NAS is mounted at /Volumes/atlas. This session should not need it: store-only searches read the local peak store, and the development mixes are in corpus/.
- Timings need an idle machine: record the load average and the busiest processes during every timing series, alternate the builds A B B A, and if the machine is busy, label the series as contended, do not adopt on it, and repeat it later if the load drops.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it), docs/brief-18.md and docs/session-18-checklist.md (brief-2 to brief-17 hold rules that still apply unless this prompt changes them), docs/session-notes.md (from "Session 17" to the end), docs/status.md (newest entries at the bottom), the whole research note, the roadmap's "Where search time goes", ADRs 0008, 0011 and 0017, experiments 0078, 0088, 0093, 0096, 0097 and 0103, crates/gunfinger-core/src/search/lines.rs (`OffsetSlots::may_form_line`), crates/gunfinger-core/src/spectrogram.rs and crates/gunfinger-core/src/speed.rs.
2. `git status` should be clean at d867557. Write docs/brief-19.md (this prompt verbatim, the state at the start, the plan with times) and docs/session-19-checklist.md. Point CLAUDE.md's reading list at them; keep brief-18 among the older briefs. Commit.
3. Build with --release and copy the binaries to work/bin/s19-start/. Run `gunfinger-eval baseline session-19-start`. Record the NAS peak store's state (38,105 records and revision b285da9984422fb5 expected), the compiler, the lockfile's RealFFT and RustFFT versions, and the machine's load.

Steps:

1. Setup (15 min): the steps above.

2. Proposal 1, integer phases for the pre-filter (about 85 min).
   a. Implement it as the note describes: `a = floor(q) mod 16384` once per query point, per hit `slot = ((r - a) mod 16384) >> 2`, the same 4-frame buckets, 4,096 slots and neighbour test. One u16 per query point of the block searched; nothing persisted, no format change. Follow the review's points: a signed floor and modulo before narrowing (Rust's `as` saturates), a wrapping subtraction and a 14-bit mask per hit, a guard (finite q with |q| < 2^32, checked once per point block) with today's f64 filter outside it. Exact offsets, sorting, `densest_clusters`, `line_of`, chains, refinement and confidence stay as they are.
   b. Tests first, against the exact cluster finder: negative offsets, 4-frame boundaries, wraparound, equal offsets, every order of small cases, large guarded coordinates, values next to float boundaries, and the existing case where rounding makes a 2-frame bucket miss a line. The invariant: every asset holding an eligible line reaches the unchanged exact finder; the filter may pass different non-line assets.
   c. Measure on the 2003 mix at NAS scale (`--store-only --playback both -f json`, saved index current): instrumented counts from a separate run (assets passed, sorts, lines, old against new), then the filter-plus-resulting-sort time by 0103's split-profile method. There is no replay harness and no time to build one: the profile stands in for the note's replay; say so in the experiment. Stop if the stage is not at least 20% cheaper, phase construction included.
   d. If it passes: at least three A B B A blocks with independent release binaries (wall, user and system CPU, peak memory). Keep it only with: `regress session-19-start` identical; the 2003 mix's report at NAS scale byte-identical between the old and new builds, and the stakka-skynet-knowledge and nocny-trans-vertical-2002-08-23 mixes' too; at least 5% less median wall time beyond the spread; no repeatable regression at 262 tracks (the stakka mix) or on a short query (a 30 s excerpt of the 2003 mix); peak memory at most 3% higher. Otherwise delete the code (AGENTS.md). An experiment file either way.

3. Proposal 2, FFT lengths of the turntable rungs: the diagnostic only (about 45 min).
   a. Enumerate the lengths in the Rust build: every turntable rung's `round(1024 / s)`, nearest-even `2 round(1024 / s / 2)`, and the note's factor-aware rule (within one sample, the length whose complex FFT has the smallest largest prime factor; within two samples as a fourth row if cheap). Record the factorisations.
   b. On fixed decoded input (for example 60 s of the 2003 mix, decoded once), time plan and window construction, repeated `process_with_scratch`, log power, and peak picking separately, for each rung and arm, with per-rung medians and tails. Weight the ladder by how often each rung is analysed in the 2003 mix's `identify` at NAS scale (first pass and refinement apart; `points_at_speed_in` serves both), counted in an instrumented run. Also count how many plans one `identify` builds and what they cost.
   c. Report the weighted first-pass analysis time of each arm against today's, and whether the note's stop point (at least 20% faster, unaffected rungs included) is passed. Do not change the product's lengths: that changes peaks and output, needs a retrieval study and is our decision. Say what the retrieval study would need and how long it would take.
   d. If plan construction is a material share of `identify`'s CPU, try a plan cache across calls (exact output): points bit-identical on every rung of both ladders, then step 2d's criteria. Keep it only if it passes them.
   e. Diagnostic code lives under work/perf/ like 0078's (record its command lines), unless a reusable `gunfinger-eval` command is clearly worth keeping. An experiment file.

4. A read-only review of the session's code and experiments by a subagent; fix what it finds (15 min).

5. Wrap-up (20 min): a "Session 19" section at the end of docs/session-notes.md (what was kept or rejected with numbers, the FFT diagnostic and whether a retrieval session is worth running, the timing conditions); update docs/status.md, the checklist, the roadmap's "Where search time goes", and the research note (proposals 1 and 2: measured, with experiment numbers; the note's other text stays). `regress session-19-start` identical with the final build; scripts/check.sh green; the NAS store's revision. Commit. Last, print what was done, the checks, and anything left.

Rules for this session:
- Hard end 20:30. If step 2 overruns by more than 20 minutes, step 3 shrinks to 3a-3c on fewer rungs. The review starts by 19:55 and the wrap-up by 20:10 whatever is left. An unfinished step is reverted, or committed only in a working, tested state, and named in the notes.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or reports, rescore nothing, and never run test evaluation 5. Never identify, decode or index EM_1999_02_07_-_Ed_Rush_Optical.mp3 at the NAS root. Run nothing on the candidate mixes of experiment 0065, including the NAS's sety/vertical.mp3. The recall panel's validation half stays unsearched.
- The five test-derived versions are never used to tune anything: Terminal 2, the Orders remixes, the Marky & Bungle Funktion, the Clear Skyz Break remix and 1996 VIP, and the Black Barrel Beachball.
- The NAS (/Volumes/atlas/Music/dnb) is read, never written; its .gunfingerignore is edited only by hand. The NAS peak store is read-only: no `index`, `prune` or `--retry-skipped` against it. Index only into temporary stores. Check its revision at the end; if it changes during the session (new files being indexed), write it down and compare only runs made at the same revision.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt and work/audition/answers.jsonl do not change. Manifests do not change.
- The matcher's results, the confidence rule, the possible tier's threshold, the peak records and the detections do not change: every kept change has exact output; `regress session-19-start` identical at the end; scripts/check.sh green.
- AGENTS.md: no SIMD, no `unsafe`, optimise only from measurement, an experiment file of at most 40 lines, a losing experiment's code deleted.
- Out of scope (ADR 0017): report edits, review workflows, and features that fix tags or the library.
- corpus/ is private and never committed.
- Leave alone every process this session did not start.
- Runs longer than 30 minutes start detached under `screen`; heavy runs one at a time. Keep scratch data under a few GB.
- Commit after each step. Check `git status` before every commit and never use `git add -A`. Do not push. No AI attribution in commits.
```

## State at the start of session 19

- 17:23. `git status` clean at `d867557` on `master`. Experiments
  0001-0107 (the next is 0108); ADRs 0001-0019. Test set: 4 of 5
  evaluations used (none in this session).
- `/Volumes/atlas`
  mounted
  over
  SMB.
- The NAS peak store (`~/.local/share/gunfinger/nas-dnb-peaks`): 38,105
  records, revision `b285da9984422fb5`, as expected (17:23). The CLI's
  saved index `nas-dnb-peaks-23935cfc56936b67.index` (4.29 GB, 15:16);
  the harness's `work/indexes/library-nas-dnb-peaks.index` (3.81 GB,
  03:41, 33,723 records, stale).
- Toolchain: rustc 1.98.0 (Homebrew); `rust-version = "1.89"`. Lockfile:
  RealFFT 3.5.0, RustFFT 6.4.1.
- Listening files: `docs/pair-verdicts.txt` sha256 `d9af100a…9586`,
  `work/audition/answers.jsonl` sha256 `dff04586…09be`
  (`work/s19/listening-digests-start.txt`).

## Plan, with times

Start 17:30; hard end 20:30.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1 | setup: brief, checklist, build, baseline, store revision, `doctor` | 0:15 | 17:45 |
| 2a-b | integer phases in `OffsetSlots`, predicate tests against `densest_clusters` | 0:30 | 18:15 |
| 2c | counts (instrumented, uncommitted), split profiles old and new | 0:20 | 18:35 |
| 2d | A B B A x3 at NAS scale, stakka at 262, 30 s excerpt, three NAS reports, `regress`; experiment 0108 | 0:35 | 19:10 |
| 3 | FFT lengths: enumeration, `work/perf/` timing, instrumented counts, weighted result; experiment 0109 | 0:45 | 19:55 |
| 4 | read-only review by a subagent; fixes | 0:15 | 20:10 |
| 5 | wrap-up: notes, status, checklist, roadmap, research note, `regress`, checks, store revision | 0:20 | 20:30 |

If step 2 overruns by more than 20 minutes, step 3 shrinks to 3a-3c on
fewer rungs. The review starts by 19:55 and the wrap-up by 20:10
whatever is left.

## Conservative decisions taken up front

- Every timing
  series records `uptime` and the top processes (`work/s19/load.sh`);
  series taken under load are labelled contended and nothing is adopted
  on them alone. If the load does not drop, proposal 1 is not adopted
  this session whatever the contended numbers say.
- The split profile (0103's method, `sample` every 10 ms on builds with
  the grouping's functions kept out of line) stands in for the note's
  replay of captured hits, which does not exist; the experiment says so.
- Instrumented counting builds are separate binaries from timing builds,
  never committed.
- The harness's stale saved index is deleted only if space runs short;
  no step of this plan needs the harness at NAS scale.
- The FFT diagnostic changes no product length; a plan cache is tried
  only if plan construction is a material share (taken as at least 3%
  of `identify`'s CPU).
