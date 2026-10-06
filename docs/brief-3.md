# Session 3 brief: memory, reports, generated mixes, evidence

Written by the agent for itself on 2026-10-06, so the plan survives context
compaction. The owner is away. Work autonomously and never stop to ask. When
something is ambiguous, take the conservative option, write the decision
down and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-3-checklist.md` (progress).
3. `docs/notes-for-owner.md` (findings to report).
4. `docs/status.md` (newest entries at the bottom).
5. The newest file in `docs/experiments/`.

`docs/brief-2.md` holds session 2's rules, which still apply.

## The owner's prompt, verbatim

```text
Start session 3 of Gunfinger: autonomous work on the items from the Astra and
Fable reviews that were recommended for immediate work. The owner is away;
work autonomously and never stop to ask. When something is ambiguous, take the
conservative option, write the decision down and continue.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it), docs/brief-2.md,
   docs/notes-for-owner.md, docs/status.md (newest entries at the bottom),
   docs/roadmap.md, docs/calibration.md and the newest experiment
   (docs/experiments/0018-key-lock-default.md).
2. Write docs/brief-3.md: this prompt, the state at the start and the plan.
   Write docs/session-3-checklist.md with the items below. Point CLAUDE.md's
   reading list at brief-3 and the session-3 checklist. Commit.
3. Build with --release and run `gunfinger-eval baseline session-3-start`.

Rules (brief.md sections 3-5 and brief-2's rules still hold):
- corpus/ is private, copyrighted audio: read-only, never commit any of it.
  Only file names may appear in docs. Never edit a manifest.
- Commit every standalone piece as soon as scripts/check.sh is green, and tick
  it in the checklist. Never `git add -A` without checking. No remotes, no
  pushing, nothing published. No AI attribution in commits. No em dashes
  anywhere.
- Save every experiment immediately as docs/experiments/NNNN-slug.md (at most
  40 lines; numbering continues at 0019), with data in docs/experiments/data/,
  before starting the next. Put findings into docs/notes-for-owner.md as soon
  as you find them, and add entries to docs/status.md.
- Default detection must not change without the full protocol: sweep (seed
  2026), development scan, leave-outs 3 and 11 (seed 2026), calibrate, regress
  against the baseline, and an entry in docs/calibration.md. Until then, new
  algorithms are opt-in (a flag or a harness option). Changes meant to keep
  results identical need `regress` to show identical detections.
- Test set: 3 evaluations left. Do not spend any; the owner decides. Where
  one would help, say so in notes-for-owner.
- Both playbacks (turntable and key lock) are the default in identify, explain
  and the gunfinger-eval ladder. Default reports are in work/reports/,
  turntable-only reports in work/reports/ladder-turntable/.
- The owner's ~/.config/gunfinger/config.toml sets playback = "turntable",
  library and peaks_dir, and the gunfinger CLI reads it from any directory.
  For measurements, pass every setting as a flag or use --config /dev/null.
  Do not edit that file.
- Other sessions load this machine's CPU, with load averages up to about 280.
  Compare CPU time (/usr/bin/time -l) and deterministic counts (detections,
  hits, postings scanned), interleave variants, and treat wall times as
  noise. Watch memory: the scale proxy at 26,000 assets swapped (experiment
  0012). Give long background jobs time limits and log to work/logs/.

Work through these in order; if one is blocked, record why and move on:

1. Memory by phase (roadmap). Measure peak memory while loading, building
   and searching, at several worker counts, on the synthetic scale proxy.
   If the struct-size inference holds, build the index in two passes that
   read one peak record at a time (count, then fill). Drop the peak records
   after the build in identify and explain; only stats reads them later.
   Detections must stay identical (regress).
2. Reports and explain:
   - Write reports atomically (temporary file, then rename, as the peak
     store does).
   - Record the remaining search settings in each report: profile and hash
     design, the confidence rule, the library's revision. Playback is
     already recorded. `--save-dir` must search a recording again when any
     of them differ.
   - Show the evidence per window in `explain`.
3. Generated mixes plus the window-grid sweep. Add a seeded harness renderer
   of library-track mixes with exact truth: speeds, crossfades, bass-swap EQ,
   plays of 20-60 s, cuts, a returning track, held-out tracks. Use it to
   slide brief plays across the 10 s window grid in 1 s steps and vary their
   length and source position. Try a minimum aligned span in seconds instead
   of 3 windows (opt-in). It also gives boundary truth.
4. Where real mixes lose evidence, on the development mix only. Attribute the
   per-play loss: solo against blended passages, frequency band, distance
   from the nearest rung, speed variation within a window. Add robust
   conditions: wow (0.55 and 0.75 Hz), broadcast compression, beatmatched
   blends, combined damage.
5. A second pass at the fitted speed (opt-in): re-analyse each candidate's
   span with one STFT at its fitted speed and count hits against that asset
   alone. It changes the evidence scale, so measure its own null and
   calibration, using items 3 and 4.
6. One default-change package, kept opt-in: the second pass plus the
   common-hash filter (drop the fullest 1% of posting lists).
   - Set the threshold from calibration data alone, never from the test set.
     Note that the filter would push Sick Note (209 hits on the test set)
     below 200.
   - Run every robust condition with the filter on and off, also with both
     ladders, plus brief excerpts and combined damage.
   - Report confident recall, possible recall and false candidates
     separately, then run the full protocol, including on the padded index.
   - Write the case for adoption in notes-for-owner. Adoption and any
     test-set confirmation are the owner's call.
7. Before tracks are added:
   - A fixed query panel (queries, source positions, held-out clusters kept
     across library growth) with more sweep seeds than 2026.
   - Duplicate clustering from stored peaks (no decoding), checked against
     the exhaustive `clusters` with controlled cases first: different rips,
     cropped recordings, edits, and A-B-C chains.

At the end: update README, roadmap, calibration and status. Put a summary at
the top of docs/notes-for-owner.md: what was done, what was learned, and the
decisions left for the owner.
```

## State at the start of session 3

- Commit `0fdf730` on `master`. Experiments 0001-0018; ADRs 0001-0007.
- Candidate A: speed-ladder pair hashes (21 bits, fan-out 2), lines per
  10 s window, chains, plays (ADR 0006). Confident rule: 200 hits and 3
  windows; possible tier 60 hits. Calibrated at 262 tracks
  (`docs/calibration.md`).
- Both playbacks are the default (82 rungs: 41 turntable, 41 key-locked);
  1.57 times the CPU time of turntable alone (experiment 0018). The owner's
  configuration file sets `playback = "turntable"`.
- Development set 11/11, 0 wrong; leave-outs 3 and 11: 0 wrong. `calibrate`
  with both playbacks: weakest identifying 501 hits, strongest false 97,
  margin 5.16×. Test set 15/16, 0 wrong (turntable alone); 2 of 5
  evaluations used, 3 left.
- Measured, not adopted: emptying the fullest 1% of posting lists
  (experiments 0013, 0017; harness option `--drop-fullest 0.01`).
- Open: memory at scale (the scan at 26,462 assets swapped, 0012); ADR 0007
  (beyond 32,768 assets).

## Plan

Items in the order of the prompt. For each: read the code involved, make
the change opt-in or prove it identical, write the experiment before the
next one starts, commit with the gate green, tick the checklist, add a
status entry and any finding to the notes.

1. **Memory by phase.** Add a harness or `--verbose` measurement of
   resident memory after loading, after the build and after the search
   (`/usr/bin/time -l` for the peak, plus a phase log). Run the scale proxy
   at a few copy counts and worker counts, under a time limit. If the
   peak-record and `Point` arrays dominate, build the index in two passes
   over the store (count postings per hash, then fill), one record at a
   time, and drop the records after the build where only `stats` needs
   them. `regress session-3-start` must show identical detections.
2. **Reports.** Atomic writes through a temporary file and rename. A
   `search` block in the report: profile, hash design, confidence rule and
   a library revision (a digest of the indexed assets' paths, sizes and
   mtimes, or of the peak records); `--save-dir` compares all of them.
   `explain --windows` (or a section in `explain`) shows each window's
   lines for the chosen asset.
3. **Generated mixes.** A seeded `gunfinger-eval mix` command: renders a
   mix of library tracks through FFmpeg with exact truth, scores it, and a
   window-grid sweep that slides brief plays in 1 s steps. An opt-in
   minimum aligned span in seconds, compared with the 3-window rule on the
   same data. Boundary error from the truth.
4. **Evidence loss.** On the development mix only: per-play attribution
   using the library file at the fitted speed. New `robust` conditions:
   wow at 0.55 and 0.75 Hz, broadcast compression, beatmatched blends,
   combined damage.
5. **Second pass** at the fitted speed, opt-in, with its own null and
   calibration from items 3 and 4.
6. **Package**: second pass plus the common-hash filter, opt-in, through
   every robust condition (filter on and off, both ladders), brief
   excerpts, combined damage, the full protocol and the padded index.
   Case for adoption in the notes; no test-set evaluation.
7. **Before tracks are added**: a fixed query panel with several seeds;
   duplicate clustering from stored peaks, checked against `clusters` on
   controlled cases first.

Conservative defaults for this session: anything that could change default
detection stays behind a flag or a harness option; the test set is not
run; the owner's configuration file is never edited, and measurements use
`--config /dev/null` or explicit flags.
