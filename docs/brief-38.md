# Session 38 brief: a narrower variant for pitch rides

Written by the agent for itself on 2026-10-22, so the plan survives
context compaction. Work autonomously and never stop to ask: where a
decision is needed, take the conservative option, write it down here and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-38-checklist.md` (progress).
3. Experiments 0166 and 0175-0178, and this session's (0179 and up); the
   roadmap's "Speed that wanders within a play"; ADR 0006 and ADR 0008.
4. `docs/session-notes.md` ("Session 36" and "Session 37"),
   `docs/status.md` (newest entries), `crates/gunfinger-core/src/search.rs`
   (`strongest_per_moment`, the variants), `search/refine.rs`,
   `search/chains.rs`, `crates/gunfinger-eval/src/robust.rs`.

`docs/brief-2.md` to `docs/brief-37.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 38 of Gunfinger: finish session 37's pitch-ride work so that we can decide on it. Session 37 (experiments 0175-0178) found that a ride of about 1% or more splits a play into two chains, each confident after the second pass, and that `strongest_per_moment` after the second pass then drops the weaker side whole, because measuring each side up to 10 s past its chain makes the two overlap (median 5 s). The opt-in `--overlap-by-chains` kept all 183 lost sides on new seeds and lost no right answer, but on the first shared-material panel it gave a wrong version the default already names (the Rogue Unit remix on two Fire queries) a second confident segment, so its pre-registered V2 failed by its letter. Session 37 proposed a narrower variant: keep a dropped side only when its fitted speed differs from the kept detection's by more than about 0.6% (the ridden sides differ by 1% or more; the Fire segments by 0.4% and 0%). This session tests that variant against criteria written first, adds a ride to `robust`'s excerpts, and writes down what an ADR would decide. The product's rules and defaults do not change.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600. Use the default jobs (32; faster since 0169). One heavy job at a time.

Environment (sessions 33-37's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- NAS store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records), `--other-peaks-dir`; NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`; saved index `work/indexes/library-nas-dnb-peaks.index`. Baselines `session-32-linux` and `session-33-linux-nas`. Build `work/bin/s37-final/` (master). `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- Session 37's renders (16 GB in `work/mixes/`), scripts (`scripts/analysis/ride_*.py`), queues and logs in `work/s37/`; its variant `--overlap-by-chains` and its registered criteria in docs/brief-37.md and experiment 0178.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans must not sit in a reports directory during `regress`.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-37.md, docs/session-notes.md ("Session 36" and "Session 37"), docs/status.md (newest entries), experiments 0166 and 0175-0178, the roadmap's "Speed that wanders within a play", ADR 0006 and ADR 0008, and the code of `search.rs` (`strongest_per_moment`, the variant), `search/refine.rs`, `search/chains.rs` and `robust.rs`.
2. Write docs/brief-38.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-38-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s38/listening-digests-start.txt; it must be the same at the end. Experiments number from 0179.
3. Build `work/bin/s38-start`; `regress session-32-linux` and `regress session-33-linux-nas` identical.

Steps:

1. Pre-registration, committed before the variant runs on any test data: the variant's rule (the speed difference, how it is measured, and where it applies), its threshold chosen from development data only (0175's and 0176's seeds and the shared panel's Fire queries, which session 37 has already seen; say so), and the criteria, at least: lost sides kept on new rides seeds (3% and 1-2% rides, short and long ramps); no new wrong confident detection, counting a wrong file's added segment as wrong (the reading under which 0178's V2 failed), and no lost right one, on the sweep (seed 2026) at both scales, the three development scans and leave-outs at NAS scale, the long and short generated mixes, and the first shared-material panel's development half at NAS scale with a separate default run beside the variant's (session 37 took the default from the variant run's trace); CPU beside.

2. The variant, opt-in (a matcher option reachable from the harness; off by default; `regress` identical with it off), tested, then measured against step 1's criteria. Also report the two development candidates (Star Trails in the Stakka mix at 13:13-14:37, Into Music's middle segment in my 2003 mix) under it, as candidates nobody has listened to. An experiment file.

3. A ride inside `robust`'s excerpts (session 37's step 5): a condition whose 30 s excerpt contains a 1-3% ride, at 262 tracks and NAS scale, under the default and both variants: does a ride cost recall in excerpts, not only time in long plays? An experiment file.

4. What an ADR would decide, in docs/research/ (not an ADR; the decision is ours): the rule today, the failure and its size (0176's dose-response, the development candidates), each variant's evidence, the shared-material risk, and a recommendation marked as such.

5. Ahead, only when 1-4 are done: if the narrower variant passes, its effect on per-second scores (`seconds`) of the rides mixes and the development scans.

6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.

7. Wrap-up: a "Session 38" section at the end of docs/session-notes.md with a handoff; update docs/status.md, the checklist and the roadmap's "Speed that wanders within a play". Final checks: `scripts/check.sh` green, `regress session-32-linux` and `regress session-33-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, `git status` clean. Commit. Your final message: the variant's result against each criterion, the `robust` result, the recommendation, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's behaviour does not change: no threshold, parameter, rule or default changes; variants are opt-in, tested, and keep `regress` identical when off. Harness additions are tested and keep `regress` identical.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other (session 37 lost time to that).
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine".
- Leave alone every process this session did not start.
- Scratch data goes under work/s38/ and renders under `work/mixes/`; delete renders you no longer need and keep the total under about 40 GB.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00. Start no new run after about 10:45; finish what is running, then do steps 6-7, and list what is left in the handoff.
```

## State at the start of session 38

- 2026-10-22, 08:12. `master` at `f7e4819`, clean. Git identity `Jacek
  Galanciak <jacek.galanciak@gmail.com>`. No tmux server running.
- `docs/pair-verdicts.txt`: SHA-256 `ad0fc785d26a...`
  (`work/s38/listening-digests-start.txt`), equal to session 37's. No
  answer log in this clone.
- Experiments 0001-0178; this session numbers from 0179. Test set: 4 of
  5 evaluations used (none in this session).
- `work/mixes/`: session 37's renders, 16 GB (seed 3701's 12 conditions,
  rides 3741-3742, seed 2026's short and long mixes). Disk: 809 GB free.
- No default report of the first shared panel's development half exists
  on the Linux machine (session 37 took the default from its trace).

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digest; start build; both regressions; then the default shared run (38 min) in the same queue, since it needs no new code | 08:12-09:05 |
| 1 | threshold from session 37's traces; the variant's code and test; criteria; commit before any variant run | 08:20-08:50 |
| 2 | the variant's runs (queue after the default shared run): rides seed 3801 (4 conditions; default, variant, `--overlap-by-chains`), both regressions with it, 8 development scans, short and long mixes of seed 2026, the shared run; experiment 0179 | 09:05-10:20 |
| 3 | `robust` ride conditions (code and test while step 2 runs); runs at 262 tracks and NAS scale under the default and both variants; experiment 0180 | 10:20-10:45 |
| 4 | the research note | beside the runs |
| 5 | only if 1-4 are done | |
| 6-7 | review, notes, final checks | 10:45-11:00 |

## Decisions

- **(08:14) The default shared run in the start queue.** It needs no new
  code, takes about 38 minutes and is the comparison step 1 asks for, so
  it runs with the start build right after both regressions
  (`work/s38/start.sh`, tmux `s38start`), while the variant is written.
  No other heavy job runs beside it.
- **(08:22) Start build and regressions.** `work/bin/s38-start/` is
  byte-identical to `s37-final/`. `regress session-32-linux` identical
  (1:09) and `regress session-33-linux-nas` identical (6:42, beside the
  variant's first build); every report they write is identical to its
  baseline's apart from `wall_seconds`. The default shared run started at
  08:22 (start build).
- **(08:26) Step 1, the pre-registration** (committed with the variant's
  code, before it runs on any test data).

  *The rule* (`--apart-in-speed 0.006` in the harness;
  `Options::second_overlap = SecondOverlap::SpansAtSpeed(0.006)` in the
  core; off by default). Only the second suppression changes, the
  `strongest_per_moment` after the second pass: a detection is dropped
  for overlapping a stronger kept detection of the same asset only when
  their spans, as the second pass measured them, overlap and their fitted
  speeds (the second pass's `Detection::speed`; for a key-locked
  detection, the tempo) differ by at most 0.6% of the stronger one's:
  `|weaker / stronger - 1| <= 0.006`. The first suppression, on the
  first-pass chains, is unchanged, and so is `--single-pass`. Unlike
  `--overlap-by-chains`, a detection the variant keeps can itself drop a
  weaker one at its speed that the default keeps, so the variant's list
  is not a superset of the default's; S3 checks what that costs.
  `--overlap-by-chains` becomes `SecondOverlap::Chains` (same behaviour).

  *The threshold, from development data only.* Session 37's traces, all
  seen by session 37: seeds 3521-3522 (0175), seed 3701's conditions
  (0176), the three development scans at NAS scale, and the first shared
  panel's development half (all its queries, Fire's among them). For each
  detection confident after the second pass and dropped by the default's
  second suppression, the speed difference to the stronger overlapping
  kept one (`work/s38/dev/speed_gaps.py`, `shared_gaps.py`, `edges.py`):
  rides of 1%: 0.90% at least (seed 3701, 10-20 s), 0.95% (2-5 s), 0.98%
  (seeds 3521-3522, 1-3%); 2%: 1.66%; 3%: 2.87%, apart from one 308-hit
  repeat at 0.00%; 0.5% over 2-5 s: four at 0.50%; the shared panel: 59,
  all at 0-0.005% or 0.378-0.400% (one rung step), Fire's Rogue Unit
  segments among them at 0.00% (627 hits) and 0.40% (366 hits); the
  development scans: Star Trails 1.34%, Into Music 1.05%, Phantom Force
  0.005%. 0.6% lies between the largest difference outside a ride
  (0.40%) and the smallest of a 1% ride (0.90%), and equals 1.5 rung
  steps, the chain link rule's limit (ADR 0008) and the second pass's
  `RUNG_REACH`. It leaves the sides of 0.5% rides dropped (0176: 2 sides
  of 30 s or more at 0.5% over 2-5 s). The shared panel is not new data
  for this threshold: it is the same development half session 37 ran, so
  its result below tests the rule on queries whose differences were read
  to choose it.

  *Test data.* Rides seed 3801 (never rendered), 16 mixes per condition,
  four conditions: `--ride-change 0.03 --ride-seconds 2,5`, `0.03` over
  `10,20`, `--ride-change 0.01,0.02` over `2,5` and over `10,20`; each
  searched by default and with the variant. The other runs below, by
  default (existing reports where they exist; the shared panel's in a
  separate run, 08:22) and with the variant.

  *Criteria.*
  - S1, recovery: of the sides of 30 s or more that the default loses on
    seed 3801 (no confident detection of the play's own file, 240 hits in
    3 windows; `ride_variant.py recovery`), the variant keeps at least
    90% over the four conditions together, and covers at least 90% of
    their seconds; in each condition at least 75% of the sides. The
    partial count (30 s or more uncovered) is reported beside.
  - S2, no new wrong confident detection: in every run below, the count
    of wrong confident detections does not rise, and no confident
    detection wrong at its time appears that the default's run lacks; a
    wrong file's added segment counts as wrong even where the default
    already names that file confidently there (the reading under which
    0178's V2 failed). Wrong is: in the sweep a wrong answer, in a scan
    a wrong identification or a confident detection matching no slot's
    files, in generated mixes a confident detection of a file not
    playing then, in the shared panel a confident detection of role
    `other`. Runs: the sweep (seed 2026) at 262 tracks and at NAS scale
    (`regress` with the variant), the three development scans and their
    eight leave-outs at NAS scale (Stakka's three in `regress`), the
    short and long generated mixes of seed 2026 at 262 tracks, the first
    shared panel's development half at NAS scale (a default run and a
    variant run, both reports compared query by query), and seed 3801.
  - S3, no right answer lost: in the same runs, every confident
    identification of the default stays: sweep queries confident on
    their cluster, scan tracks identified, generated and rides mixes'
    indexed plays with a confident detection, and shared queries with a
    confident detection of the played recording (and of the partner,
    for blends).
  - Beside, not criteria: user CPU (`regress` at both scales against the
    start build's, seed 3801's searches, the shared runs, with what ran
    beside), right segments added, possible detections added, calibrate's
    false candidates; Star Trails (13:13-14:37) and Into Music's middle
    segment under the variant, as candidates nobody has listened to;
    `--overlap-by-chains` on seed 3801 if time allows.
  - The variant passes when S1, S2 and S3 all hold. Adoption stays our
    decision and needs an ADR.
- **(08:26) Step 3's design** (written before its runs). `robust` gains
  four ride conditions, run only when named in `--only` (the default
  `robust` run is unchanged): the speed rises by 1%, 2% or 3% over 3 s
  centred 15 s into the 30 s excerpt (`ride-1pct-at-15s` and so on: each
  side spans two 10 s windows), and by 2% centred at 24 s
  (`ride-2pct-at-24s`: the side before spans three). Rides go up from
  the base speeds 0.95 and 1.03, as the existing 2% ramp over the whole
  excerpt (`pitch-ride-2pct`, run beside). At 262 tracks and NAS scale,
  under the default, `--overlap-by-chains` and `--apart-in-speed 0.006`.
- **(08:27) Committed before any variant run** (`393469d`, the variant,
  the `robust` conditions and the criteria above; its message says
  08:40, the time planned, and the entries were dated 08:40 there: both
  were written at 08:20-08:26). Checks before the commit: `check.sh`
  green on the variant's code (4:44, beside the NAS regression's end and
  the default shared run), then clippy and `robust`'s tests on the
  added conditions; `check.sh` again on the commit (tmux `s38chk2`). Run
  build `work/bin/s38-run/` from `393469d`. The step 2 queue
  (`work/s38/step2.sh`, tmux `s38queue`) waits for `s38start` by exact
  name (`=s38start`).
- **(08:27) `--overlap-by-chains` on seed 3801 last**, after the shared
  variant run, as a comparison only (not a criterion); dropped first if
  time runs short.
- **(09:02) The default shared run** (start build, 38:31, 6.2 GiB; it
  ran beside `check.sh` twice, 08:22-08:31). Compared query by query with
  session 37's `--overlap-by-chains` report (`variant_compare.py shared`):
  confident detections of role `played` 516 -> 568, `other` 475 -> 479
  (the Rogue Unit remix's second segment in fire-6's two "beyond"
  queries, indexed and played left out), `related` 430 and `partner` 8
  unchanged, no query loses a played or partner detection. So session
  37's trace-based default (`default_final`) gave the same outcome as a
  real default run.
- **(09:27) S1 on seed 3801** (renders 3:32-3:34 and 14.4-15.1 GiB each,
  variant searches on the cached renders 2:37-2:39). Sides of 30 s or
  more with no confident detection by default, kept by the variant: 3%
  over 2-5 s 94 of 94, over 10-20 s 10 of 10; 1-2% over 2-5 s 44 of 44,
  over 10-20 s 109 of 109: 257 of 257, 100% of their 17,487 s (3 s
  short). By the partial count 376 of 380 (the four left at 1-2% over
  10-20 s). Wrong confident 0 -> 0 and right 191 -> 191 in every
  condition, no play lost or gained (`variant_compare.py answers`). S1
  holds.
- **(09:44) S2 and S3 so far** (all but the shared panel). `regress
  session-32-linux` with the variant: sweep identical, the Stakka scan
  and leave-out 3 gain Star Trails' 13:13-14:37 side (315 hits),
  calibrate unchanged (false candidates 18 -> 18). `regress
  session-33-linux-nas`: 716 of 720 sweep queries identical (the same 4
  weak additions as 0178), wrong 7 -> 7, right 540 -> 540; Stakka gains
  the side in three files (306-315 hits); calibrate unchanged (95 ->
  95). The 8 other development reports at NAS scale: wrong 0 -> 0, right
  unchanged; the 2003 mix and its leave-out 3 gain Into Music's
  25:57-26:56 segment (1,392 hits); weak detections otherwise; Phantom
  Force's 0.005% segment, which 0178 added, stays dropped. Short mixes
  of seed 2026 identical; long: 14 weak added, wrong 0 -> 0, right 191 ->
  191. Nothing removed except one possible detection in seed 3801's 1-2%
  over 10-20 s (no play changes).
- **(09:42) Queue reordered.** The step 2 queue had reached `robust`
  (step 3, experiment 0180) before the shared variant run, the last run
  of 0179, which AGENTS.md's order forbids (an experiment file before
  the next experiment's runs). Its first `robust` run was stopped after
  a minute (partial renders deleted; the cache reuses only completed
  renders) and a new queue (`work/s38/step2b.sh`, tmux `s38shared`) runs
  the shared variant run, then waits for `work/s38/0179-written` before
  `robust`; `--overlap-by-chains` on seed 3801 last, skipped if
  `work/s38/stop-chains` exists. zsh expands `=s38queue`, so the first
  attempt to stop the tmux session by exact name failed; the processes
  were stopped by PID.
- **(10:20) The shared variant run** (35:27). Against the default run
  (`variant_compare.py shared`): confident detections by role identical
  in every query (played 516, other 475, related 430, partner 8), no
  query loses a played or partner detection; 185 weak detections added
  in 59 queries. User CPU 11,478 s against the default's 11,841 s, which
  ran beside `check.sh`. S1, S2 and S3 all hold: the variant passes its
  registered criteria (0179). `robust` released (`work/s38/0179-written`)
  after 0179 was committed.
