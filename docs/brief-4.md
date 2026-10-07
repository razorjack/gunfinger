# Session 4 brief: idle-machine measurements, search memory, matching

Written by the agent for itself on 2026-10-07, so the plan survives context
compaction. The owner is away overnight and has left the machine idle.
Work autonomously and never stop to ask. When something is ambiguous, take
the conservative option, write the decision down and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-4-checklist.md` (progress).
3. `docs/notes-for-owner.md` (findings to report).
4. `docs/status.md` (newest entries at the bottom).
5. The newest file in `docs/experiments/`.

`docs/brief-2.md` and `docs/brief-3.md` hold rules that still apply.

## The owner's prompt, verbatim

```text
Start session 4 of Gunfinger: autonomous work on roadmap items that need no
decision from the owner and no larger library. The owner is away overnight
and has left the machine idle for this session. Work autonomously and never
stop to ask. When something is ambiguous, take the conservative option, write
the decision down and continue.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it), docs/brief-3.md
   and docs/brief-2.md (their rules still hold), docs/notes-for-owner.md,
   docs/status.md (newest entries at the bottom), docs/roadmap.md,
   docs/calibration.md and the newest experiment.
2. Write docs/brief-4.md: this prompt, the state at the start and the plan.
   Write docs/session-4-checklist.md with the items below. Point CLAUDE.md's
   reading list at brief-4 and the session-4 checklist. Commit.
3. Build with --release and run `gunfinger-eval baseline session-4-start`.

Rules (brief-2's and brief-3's rules still hold):
- corpus/ is private, copyrighted audio: read-only, never commit any of it.
  Never edit a manifest.
- Commit each standalone piece once scripts/check.sh is green, and tick it in
  the checklist. Never `git add -A` without checking. No remotes, no pushing,
  nothing published. No AI attribution in commits. No em dashes anywhere.
- Save every experiment immediately as docs/experiments/NNNN-slug.md (at most
  40 lines; numbering continues after the newest), with data in
  docs/experiments/data/. Put findings into docs/notes-for-owner.md as soon as
  you find them, and add entries to docs/status.md.
- Default detection must not change. Everything that would change detections
  or boundaries is opt-in (a flag or a harness option) and gets the full
  protocol: sweeps for seeds 2026-2029 on the saved panels, the development
  scan, leave-outs 3 and 11, calibrate, regress. Measure it under both today's
  matcher and the candidate (`--second-pass --skip-fullest 0.01`, rule 240),
  because the owner has not chosen between them. Then write the case for
  adoption in notes-for-owner. Changes meant to keep results identical need
  `regress` to show identical detections.
- Test set: spend no evaluations (3 left).
- Never change or re-extract work/peaks. A changed peak profile goes into a
  separate store under work/variants/.
- The owner's ~/.config/gunfinger/config.toml sets playback = "turntable"
  and the library. For CLI measurements, pass every setting as a flag or use
  --config /dev/null. Do not edit that file.
- The machine is idle tonight, so wall time and peak memory are meaningful.
  Keep them meaningful:
  - Run timed and memory measurements one at a time, never beside another
    heavy job of yours. Record `sysctl -n vm.loadavg` before and after each
    one. If the load average stays above 2 before a run, someone is using
    the machine: wait, or fall back to CPU time and counts and say so.
  - Run every long job under `caffeinate -i` so the machine does not sleep,
    with a time limit, logging to work/logs/.
  - Check `sysctl vm.swapusage` before and during scale runs. Stop a run
    whose swap use grows by more than 1 GB, record it, and go on with a
    smaller size. Do not let the machine swap heavily.
  - Report wall time, CPU time (/usr/bin/time -l) and peak resident memory
    together. Repeat timings at least three times and give the median and
    the spread.

Work through these in order; if one is blocked, record why and move on:

1. Idle-machine measurements of today's code, as the reference for
   everything below:
   - Where search time goes (roadmap): profile `gunfinger identify` on the
     development mix with /usr/bin/sample (or xctrace) and attribute the
     time to decoding, the index build, posting scans, sorting hits,
     clustering offsets, chaining and the second pass. Do this for today's
     matcher and for skip at 240.
   - Peak memory and time at scale: the development scan at 262, 8,122 and
     26,462 assets (synthetic copies), and as close to the 32,768-asset
     limit as memory allows, for today's matcher and skip at 240. This
     replaces the noisy figures of experiments 0021 and 0027 and the
     inference of about 11 GB for an hour-long mix.
   - End-to-end `identify` on the development mix at 262 assets: turntable
     only, both playbacks, and skip at 240 with both.
   - Indexing throughput: extract the 262 library tracks into a scratch
     store under work/variants/ (never work/peaks). Report the time per
     track and per hour of audio, and the projected time for 20,000 tracks.
2. Search memory at scale (roadmap): merge neighbouring rungs' lines as each
   rung finishes instead of keeping every rung's lines until chains are
   built. Detections must stay identical. Measure lines held with
   `memory --count-lines`, and peak memory against item 1, at 8,122 and
   26,462 assets.
3. Chance lines in chains (roadmap; experiment 0020): require linked lines to
   come from nearby rungs, or a stronger line across a gap. Use
   `explain --windows` on the Clockwork remix's third window and the window
   grid as checks.
4. Speed that wanders within a play (roadmap; experiments 0022, 0024): in the
   second pass, fit the speed per stretch of a few windows. Check Star Trails
   and the generated mixes, including wow, and measure the extra analysis
   cost.
5. Peaks across frequency bands (roadmap; experiment 0022):
   - Measure true hits per 1,000 postings scanned by anchor band. This is
     offline and needs no profile change.
   - Then try peak budgets spread more evenly across bands, as variant
     profiles in their own stores. Re-extract the 262 tracks into each.
   - Report recall under the robust conditions, the development mix's kept
     hashes, postings, margins and index size.
   - The owner decides adoption; it would mean re-extracting every record.
6. Detection boundaries (roadmap; experiment 0020): trim weak chain ends.
   Measure against the generated mixes' exact boundaries, reporting true
   coverage lost beside the gain.
7. The edge of the ladder (roadmap): measure recall from ±8.2% to ±9% and
   what extra rungs would cost.
8. Small engineering, each its own commit:
   - A library identifier in the peak store, so two libraries cannot share
     one by accident. Records stay readable, and an existing store adopts
     its library without re-extraction.
   - A harness option that adds a second library's peak records to the
     index, the way --synthetic-copies adds reversed copies. It is for
     measuring a larger real library later; test it with a scratch library.
   - `gunfinger doctor` counting only library files within the track length
     range against the 32,768-asset limit.
   - Shared material in the report (roadmap): a possible play that lies
     entirely inside a confident play of another recording is shown as
     "shares material with <recording> (play N)". This is display only:
     detection and scoring stay unchanged. A remix played in its own right
     must not be folded in.

At the end: update README, roadmap, calibration and status. Put a session 4
summary at the top of docs/notes-for-owner.md: what was done, what was
learned (item 1's measurements first, as they bear on the matcher decision),
and each opt-in result with its case for or against adoption.
```

## State at the start of session 4

- Commit `9d86926` on `master`. Experiments 0001-0027; ADRs 0001-0007.
- Default matcher: speed-ladder pair hashes, lines per 10 s window, chains,
  plays. Confident 200 hits in 3 windows, possible 60. Both playbacks (82
  rungs) by default; the owner's configuration file sets turntable.
- Opt-in candidate, not adopted: the second pass with the fullest 1% of
  posting lists skipped for candidates (`--second-pass --skip-fullest
  0.01`), rule 240 hits in 3 windows (`FITTED_RULE`). Case for adoption in
  `docs/notes-for-owner.md`.
- Development set 11/11, 0 wrong; leave-outs 0 wrong; four sweep seeds
  540/540 each. Test set 15/16, 0 wrong; 2 of 5 evaluations used.
- Memory: the index is built in two passes (3.1 GB at 26,462 assets). The
  search keeps every rung's lines until chains are built; experiment 0021
  measured about 180 MB per minute of query at 26,462 assets on a loaded
  machine, and 0027 could not reproduce its peaks.
- Track length range flags (`--min-track`, `--max-track`) landed after
  session 3.
- Machine at the start: Apple M1 Pro, 10 cores, 32 GB; load average 2.7
  and falling; swap 2.4 GB used of 4 GB (left over from earlier).

## Plan

Items in the order of the prompt. For each: read the code involved, make
the change opt-in or prove it identical, write the experiment before the
next one starts, commit with the gate green, tick the checklist, add a
status entry and any finding to the notes.

Measurement discipline for the whole session: one heavy job at a time;
`sysctl -n vm.loadavg` and `sysctl vm.swapusage` before and after each
timed run, recorded in the run's log; `caffeinate -i` and `timeout` around
long jobs; `/usr/bin/time -l` for wall, user and system CPU and the peak
footprint; three repeats, median and spread. A helper script under
`work/scripts/` wraps this so every timed run logs the same fields.

1. **Reference measurements.** `sample` the `identify` process during a
   run (both matchers) and attribute the stacks to the pipeline stages by
   function name. Scale runs with `gunfinger-eval memory` (and `scan
   --synthetic-copies`) at 0, 30, 100 and 121 copies (31,964 assets: the
   synthetic stretches allow at most 121 copies; 32,768 would need 124).
   End-to-end `identify` with `--config /dev/null` and explicit flags.
   Indexing into `work/variants/index-throughput/`.
2. **Merge lines per rung.** Fold each finished rung's lines into a running
   set of distinct lines so neighbouring rungs' duplicates are dropped as
   soon as both are in. `distinct` must give the same result whatever the
   order of arrival; if exact equality cannot be kept with an incremental
   merge, merge per window in rung order at the end of each rung and prove
   identity with `regress`.
3. **Chance lines in chains.** Opt-in harness option; candidates: linked
   lines within a rung reach of each other, or across an empty window only
   when the line is strong. Full protocol under both matchers.
4. **Speed per stretch** in the second pass, opt-in. Full protocol under
   the candidate matcher (the second pass exists only there and in the
   plain `--second-pass`).
5. **Bands.** Offline hits per 1,000 postings by anchor band first; then
   variant profiles in `work/variants/<name>/peaks`, recall under the
   robust conditions, margins, index size.
6. **Boundaries.** Opt-in trimming of weak chain ends, scored on the
   generated mixes' exact boundaries.
7. **Ladder edge.** Robust-style speeds from ±8.2% to ±9%, and the cost of
   extra rungs.
8. **Engineering.** Library identifier in the store; a second library's
   records in the harness index; `doctor` counting within the length
   range; shared material in the report.

Conservative defaults: anything that could change default detection stays
behind a flag or a harness option; the test set is not run; the owner's
configuration file is never edited; `work/peaks` is never written.
