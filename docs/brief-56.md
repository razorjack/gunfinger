# Session 56 brief: the time without a confident match, printed (part 2: the estimate)

Written on 2026-10-27, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-56-checklist.md` (progress).
3. ADR 0006, 0017, 0019, 0023 and this session's ADR (0024) once
   written; `docs/research/2026-10-21-printing-gaps.md` (section 5 and
   "Decision"), `docs/research/2026-10-21-gap-measures.md` (the
   definitions).
4. `docs/session-notes.md` ("Session 34", "Session 35", "Session 55");
   experiments 0159-0166 and 0234; this session's experiment (0235).

`docs/brief-2.md` to `docs/brief-55.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 56 of Gunfinger: print the time without a confident match, part 2: an estimate of how many tracks a gap holds, with an ADR. Our decision of 2026-10-27 (docs/status.md, "session 39's listening questions, and four decisions"; `docs/research/2026-10-21-printing-gaps.md`, "Decision", item 5): each "ID - ID" gap line that session 55 added carries an estimate of how many tracks the gap holds. The note recommended no count because none met its registered bar ("about n" needs 90% exact at each k); we decided to print an estimate anyway, so its form must say how sure it is. The measured methods (experiments 0160-0165): method 1, the gap's length over the mix's median interval, needs 3 clean intervals and has no estimate otherwise; method 2, matching the gap against itself (`self_match.rs`), counts DJ-length records better but merges records that share a passage; the combined estimate (method 2 limited to method 1's range B) is the best measured, a range n-1 to n+1 (at least 1) right for 97% of the test mixes' gaps (89.6% at k = 4; 83% where records share material), 2.9% off by two. Method 2 needs the gap's audio again (about 0.05 s of wall time per gap at 16 jobs) and its count drops with pitch rides unless more rungs are matched. This session picks the estimate and its printed form, writes the ADR, and implements it in the product.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).

Time box: started about 05:30 (CET); aim to finish within about 3 hours, because another session follows you (the user's soft deadline is 15:00). If a step would run past that, scale it down (method 1's range alone, for example, if method 2 cannot be made cheap and tested in time), record the decision in the brief and continue.

Environment (sessions 33-55's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,678 records, revision `1fc6fe50293ad330`. NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `ce293fe6165e...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-54-linux-nas`. Since session 53 `SpansAtSpeed(0.006)` is the default second suppression (ADR 0021); since session 54 a detection of a recording judged to carry the same title (`docs/same-titles.txt`) is scored right (ADR 0022); since session 55 `identify` and `show` print gaps of 45 s or more as unnumbered "ID - ID" lines, with `gaps` in JSON and a reserved `tracks` field (ADR 0023, `gaps::uncovered` in the core).
- Build `work/bin/s55-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans, sweep reports of seeds other than 2026 (calibrate reads every sweep report there), and reports of opt-in variants must not sit in a reports directory during `regress`; run variants in a scratch work directory.
- The user's decisions of 2026-10-27 (docs/status.md, "session 39's listening questions, and four decisions"; the roadmap's bullets carry them): `--apart-in-speed 0.006` becomes the default; gaps are printed by default ("ID - ID", 45 s or more, the mix's start and end, possible lines kept, an estimate of the tracks a gap holds, JSON yes, CSV and cue sheet unchanged); a confident detection of a near-identical version counts as right; the minimum span stays undecided. Each is implemented with an ADR (the next number is in `docs/adr/`).
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen. Pending listening (Star Trails, Into Music) stays for later. When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-55.md and docs/brief-35.md, docs/session-notes.md ("Session 34", "Session 35" and "Session 55"), docs/status.md (the newest entries), session 55's ADR and ADR 0017, `docs/research/2026-10-21-gap-measures.md` and `docs/research/2026-10-21-printing-gaps.md` (section 5 and "Decision"), experiments 0159-0166 and session 55's, and the code of the CLI's gap listing (session 55), the harness's `gaps.rs` and `self_match.rs`, and how `identify` holds the query's decoded audio or peaks while it searches.
2. Write docs/brief-56.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-56-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s56/listening-digests-start.txt. Experiments number from 0235.
3. Build `work/bin/s56-start`; `regress session-54-linux` and `regress session-54-linux-nas` identical.

Steps:

1. The choice, in the brief before code: which estimate (the combined one unless its cost or code is out of reach; method 1 alone as the fallback; a gap with no estimate prints none), and its printed form. It must not read as a certainty: a range such as "about 1-3 tracks", or a single number only where the measured exactness supports it (k = 1: 96-99%). Say what a gap at the mix's start or end gets, where the interval cannot be measured (fewer than 3 clean intervals), and where the estimate would be 0. The JSON field reserved by session 55 carries the low and high counts and the method; CSV and the cue sheet stay unchanged.
2. The ADR (the next number in `docs/adr/`), accepted on 2026-10-27 as our decision: the estimate, its form, its measured accuracy by experiment, its run-time cost, and what it does not claim. Commit it before the code.
3. The implementation in the product: the estimate reuses the query's peaks or audio already decoded by `identify` where possible (no second decode of the whole mix); one implementation shared with the harness's measure where clean, or a test that both agree on fixtures. Tests: unit tests of the estimate on fixtures (each k, no interval, start and end gaps, a gap where the methods disagree), output tests for the human tracklist, the table and JSON, and an end-to-end test. Help texts and README updated. `scripts/check.sh` green. Measure the added wall time of `identify` on the three development mixes at NAS scale.
4. The evidence: the printed tracklists of the three development mixes at 262 tracks and at NAS scale with the estimate, each gap's estimate against what the manifest says plays there; and the estimate's accuracy on the measured test mixes of 0163-0165 through the product's code (it should reproduce 0165's numbers; a difference is a finding to explain). `regress session-54-linux` and `regress session-54-linux-nas` stay identical (detections do not change). One experiment (at most 40 lines).
5. Update the roadmap's "Time without a confident match" (done, with both ADRs), the printing note's status line, and the README's example output if it shows a gap.
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 56" section at the end of docs/session-notes.md with a handoff; update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-54-linux` and `regress session-54-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the estimate and its form, its accuracy and cost, the development mixes' gap lines with their estimates, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product changes only as decided: gap lines and the JSON gaps gain the estimate as the ADR states. The search, detections, levels and every other output stay as they are; no threshold or parameter changes. `regress` stays identical; harness additions are tested.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
- Scratch data goes under work/s56/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 15:00. Start no new run after about 14:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 56

- 2026-10-27, 05:27 by the Linux machine's clock (CET). The time box
  ends at about 08:30; no new run starts after about 08:10.
- `master` at `786d3fe`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s56/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `ce293fe6165e...`, `work/audition/answers.jsonl` `3e50aaa62181...`.
- Experiments 0001-0234; this session numbers from 0235. ADRs
  0001-0023; this session's is 0024. Test set: 4 of 5 evaluations used
  (none in this session).
- Baselines `session-54-linux` (262 tracks) and `session-54-linux-nas`.
  Build `work/bin/s55-final/` (master).
- The code today: `gunfinger_core::gaps::uncovered` is the one gap
  definition. The CLI's `gaps.rs` derives `Gap { start_seconds,
  end_seconds, possible_plays }` from a report whenever it is rendered
  (`MIN_GAP_SECONDS` 45); `export::tracklist`, `table` and
  `output::render` (JSON, through the flattened `WithGaps`) print it
  with `Gap::label`. `identify` decodes the searched part once
  (`decode_recording`, `Audio::samples` at the profile's rate, from
  `query.start_seconds`) and holds the samples until `Report::new`;
  nothing of the audio is stored in the report.
- The harness: `gaps::list` lists gaps and method 1 (the gap over the
  median of the clean start-to-start intervals between entries, at
  least 3 of them; the point estimate and range A). `self_match::blocks`
  (method 2's blocks at 5, 10, 20 and 40 hits) runs in `self-match`,
  which decodes each gap with `-ss`. Range B, the stretches (blocks of
  30 s or more at 10 hits, overlapping ones merged) and the combined
  estimate exist only in Python (`scripts/analysis/gap_tests.py`; 0165
  used its "combined without discount" column).
- Session 35's test mixes (shared 3501-3503, beatmatched 3511-3513):
  the reports are in `work/reports/`, the self-match listings in
  `work/gaps/`, the per-gap scores in `work/s35/scores/`; the renders
  were deleted.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build, both regressions and the start build's `identify` of the three development mixes at both scales (tmux `s56start`) | 05:27-05:45 |
| 1 | the estimate and its form, in this brief | 05:45-05:55 |
| 2 | ADR 0024, committed before the code | 05:55-06:10 |
| 3 | the core (`repeats`, moved from the harness; method 1 and the combined estimate in `gaps`), the CLI (identify stores the repeats, the gap lines, table and JSON print the estimate), the harness (`self-match` uses the core and writes the product's estimate), tests, help, README; `scripts/check.sh`; meanwhile session 35's six test seeds rendered again in a scratch work directory (tmux `s56renders`) | 06:10-07:20 |
| 4 | the development mixes' tracklists with the estimate, each against the manifest; the wall time added; the test seeds through the product's code against 0165; both regressions; experiment 0235 | 07:20-07:55 |
| 5 | roadmap, the printing note's status line, README example | 07:55-08:00 |
| 6-7 | review by a separate agent; notes, status, checklist, final checks | 08:00-08:30 |

## Decisions

- **(05:30) Start build and regressions** (`work/s56/start.sh`, tmux
  `s56start`, logs in `work/s56/logs/`; `run.sh`, `same.py` and
  `cli.sh` are session 55's with the paths changed). The start build's
  `identify --store-only` of the three development mixes at 262 tracks
  and at NAS scale gives the "before" reports, tracklists and wall times
  (`work/s56/cli-before/`).
- **(05:40) Step 1: the estimate and its form** (taken before the code;
  conservative where the prompt leaves a choice):
  1. *Which estimate.* The combined one of experiment 0165, as measured:
     method 2's count (the stretches: blocks of the gap matched against
     itself at speed 1.0, 10 hits per cell and lag, spanning 30 s or
     more, overlapping ones merged) limited to method 1's range B (the
     gap over the upper and the lower quartile of the clean intervals,
     each rounded and at least 1); a count of 0 stays 0; a gap whose
     mix has fewer than 3 clean intervals keeps method 2's count. Its
     cost is small (0.05 s of wall time per gap at 16 jobs in 0162) and
     `identify` already holds the decoded audio, so method 1 alone (the
     fallback) is not needed. Pitch rides: speed 1.0 only, as measured;
     the neighbouring rungs (1.7 times the time) were measured only on
     33 gaps outside the registration (0166), so they stay out.
  2. *Printed form: always a range*, `about L-H tracks` with L = n-1 (at
     least 1) and H = n+1 for the combined count n: `ID - ID (7:57
     without a confident match; about 1-3 tracks)`. Never a single
     number. The exactness at k = 1 (99% on the test) says how often a
     gap holding one track gets 1; what a reader needs is how often a
     printed 1 is right, and on 0165's test gaps the combined count 1
     held one track for 95 of 125 gaps and two for 30 (76%;
     `work/s35/scores/tests-shared-and-beatmatched.json`), so "about 1
     track" would be wrong one time in four. For n = 1 the line reads
     `about 1-2 tracks`.
  3. *A count of 0* (no repeated stretch of 30 s in the gap) prints no
     estimate: the gap line stays as in part 1, and JSON's `tracks` is
     `null`. Neither method can say that a gap holds no track (0164's
     discount failed its test), and 0 also comes from short records
     that repeat little (0162), so "0" would be a claim nobody measured
     as safe. On the development gaps, the 6 counted 0 all held no
     missed track (`dev-nas`, `dev-262`), too few to print on.
  4. *Fewer than 3 clean intervals* (method 1 has no range): method 2's
     count alone gives the range, as 0165's rule does (method 2 alone:
     within one for 92% of the test gaps, 7.6% off by two, 0163); JSON
     says which method made it.
  5. *The mix's start and end.* Their gaps get the same estimate (the
     interval comes from the entries between, and method 2 needs only
     the gap's audio). The measured gaps include such edges; an intro or
     an outro without a record was not among them (no listed gap at the
     edges of the development mixes holds one, 0234), which the ADR
     says.
  6. *JSON.* `tracks` on each gap, always written: `null` without an
     estimate, otherwise `{"low": L, "high": H, "method": M}` with M
     `repeats-within-length` (the combined estimate) or `repeats`
     (method 2 alone, fewer than 3 clean intervals). CSV, the cue sheet
     and the timeline do not change.
  7. *Where method 2's evidence lives.* `identify` matches each gap of
     the report (45 s or more) against itself while the decoded samples
     are at hand (the gap's slice of them; no second decode) and stores
     the stretches in the report as `gap_repeats` (start and end in the
     recording's time); the estimate is derived from the plays and those
     stretches whenever the report is rendered, as the gaps are, so
     `show` prints it without decoding. A report without `gap_repeats`
     (saved before this change) prints no estimate, and `identify
     --save-dir` does not search such a report again on that account
     (`--again` does): the search settings are unchanged, and searching
     again changes nothing the report already holds.
  8. *Method 1's intervals* are the product's own: the gaps listed at
     45 s, and entries as the harness makes them (confident plays, two
     over the same passage, half of the shorter, taken as one), moved to
     the core so that the CLI and the harness share them. 0165 measured
     at 30 s; the harness's `self-match` (whose `--min-length` stays 30
     by default) writes the product's estimate beside its blocks, so the
     test seeds reproduce 0165 through the same code.
- **(05:36) Start results.** The start build is byte-identical to
  `s55-final`. `regress session-54-linux` (0:50) and `regress
  session-54-linux-nas` (3:58) identical, every report identical to its
  baseline's. The "before" CLI runs took 4-6 s at 262 tracks and
  13-18 s at NAS scale. The six test seeds of 0163 (shared 3501-3503,
  beatmatched 3511-3513) are rendered and searched again with the start
  build in `work/s56/wmix` (tmux `s56renders`, `renders.sh`), session
  35's reports copied beside today's (`wmix/s35-reports/`); work/reports
  is untouched.
- **(05:33) ADR 0024 committed** (`6b6de3c`) before any code; ADR 0023's
  status line says ADR 0024 amends it. Added while writing: the printed
  range read the other way on the test (125 of 125 gaps counted 1, 88 of
  97 counted 2, 106 of 106 counted 3, 49 of 50 counted 4).
- **(05:36-05:51) The code** (`ce50a95`):
  1. `gunfinger_core::repeats` is the harness's `self_match.rs`, moved
     (`git mv`): `Repeats::find(samples, profile, ratios)` builds the hits
     once and `blocks(min_hits)` reads one threshold (the harness reads
     its four; the CLI only `RECORD_HITS` 10); `stretches` (blocks of
     `MIN_STRETCH_SECONDS` 30 or more, overlapping ones merged, as
     `gap_tests.py`) and `records` (both at speed 1.0) are new. The core
     has no serde, so the harness serializes its own `BlocksAt`.
  2. `gunfinger_core::gaps` gains `entry_starts`, `clean_intervals`,
     `median` (moved from the harness's `gaps.rs`, which now calls them),
     `length_range` (range B: Python's exclusive quartiles and its
     half-to-even rounding, both tested against values Python gives),
     `combined` and `estimate`.
  3. The harness's gap listings of six saved reports are byte-identical
     with the start build and the changed one, and its self-match
     listings (three 262-track scans, `mixes-long-seed-2026`, with
     `--ratios 0.996,1.004`) identical but for the added `tracks`
     (`work/s56/gapsame.sh`, `samelistings.py`).
  4. The CLI: `Report::gap_repeats` (optional, absent in older reports,
     validated on read); `identify` calls `gaps::find_repeats` on the
     samples it holds, one gap per worker (`--jobs`), and logs the time
     at `--verbose`; `gaps::gaps` derives `tracks` per gap (a stretch
     belongs to the gap holding its middle). The intervals use the
     confident plays' spans (`FoundPlay::start_seconds..end_seconds`), as
     the harness's labels do.
  5. `tracklist_score.py`'s `GAP_LINE` also takes `; about L-H tracks`
     (tested).
  6. `scripts/check.sh` green on the first run (6:16, beside the renders).
- **(05:52-05:54) The development mixes** with the run build
  (`work/s56/cli-after/`, `beforeafter.py`): every report's plays and
  gaps identical to the start build's, every tracklist the old one but
  for `; about L-H tracks` on its gap lines (4 at 262 tracks in the 2003
  mix, 6 in Nocny; 1 at NAS scale in Nocny; Stakka none). Against the
  manifest (`gapsvs.sh`): every range holds the number of missed tracks,
  and the count is exact for 9 of 11 (Nocny at 262 tracks counts 2 for
  The Descent VIP and for slot 17, each one track). The CLI (the gap's
  slice of the samples) and the harness's `self-match` (each gap decoded
  with `-ss`) find the same stretches and estimates for all 10 gaps at
  262 tracks.
- **(05:57-06:03) The test seeds through the product's code**
  (`seeds.sh`, `reproduce.py`): the re-rendered audio gives session 35's
  blocks for all 384 test gaps; the product's `tracks` equals
  `gap_tests.py`'s combined estimate for every gap, on session 35's
  reports and on today's (whose gaps are the same); exact 99/57/58/43%
  at k = 1-4, the range holds 100/99/100/89.6%, 2.9% off by two: 0165's
  numbers.
- **(05:57-06:12) Wall time** at NAS scale, three runs of each build
  alternately (`timing.sh`, `layout.sh`, `layout2.sh`). The gap matching
  takes under 0.05 s on the development mixes (0 or 1 gap) and 0.7 s for
  the worst case (nothing found, Nocny's 67 minutes as one gap; peak
  memory 200 against 158 MB). But the run build's search itself, whose
  code did not change, took 0.6-1.0 s longer (4-6%: 11.4 -> 11.9 s for the
  2003 mix, 17.2 -> 18.1 s for Nocny), and a build without only the call
  to the gap matching (`s56-noop`, not committed) searched as fast as the
  start build. The search is generic over its progress callback, so it is
  compiled inside the CLI crate (16 codegen units, no LTO), and the code
  added there changed how it was compiled (an inference; not examined in
  the machine code). Moving the per-gap loop into the core
  (`repeats::records_in_gaps`, `74ba54d`) halved it: the search takes
  0.3-0.5 s (2-3%) longer than the start build's. Left at that: changing
  the release profile would change every timing, and the rest is a
  compile effect, not work done.
- **(06:14-06:21) Step 4 with the core-loop build** (`step4.sh`,
  `work/bin/s56-core/`): its reports and tracklists identical to the run
  build's; `regress session-54-linux` (0:51) and `regress
  session-54-linux-nas` (4:00) identical, every report identical to its
  baseline's; the test seeds at the product's 45 s give the same figures
  as at 30 s; shared material only: exact 98/44/48/31%, the range holds
  100/100/100/83.3%, 4.2% off by two (0165's). Experiment 0235 written
  (40 lines). ADR 0024's run-time paragraph gains 0235's figure.
- **(06:21) Step 5.** The roadmap's "Time without a confident match"
  (done, with both ADRs), the printing note's status line ("decided and
  implemented") and README's example (the 2003 mix at 262 tracks with
  its estimates) updated.
