# Session 2 brief: command line, test infrastructure, transformed-library experiments

Written by the agent for itself on 2026-10-06, at the owner's request, so
the plan survives context compaction. The owner is away overnight. Work
autonomously; never stop to ask. Take the conservative option when something
is ambiguous, write the decision down, and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it). Its repository,
   corpus and evaluation rules still hold unless this brief says otherwise.
2. This file.
3. `docs/session-2-checklist.md`: what is done and what is next.
4. `docs/notes-for-owner.md`: findings to report in the morning.
5. `docs/status.md` (newest entries at the bottom) and the newest file in
   `docs/experiments/`.

## State at the start of session 2

- Candidate A works: speed-ladder pair hashes, lines per 10 s window,
  chains, plays (ADR 0006). Confident rule 200 hits and 3 windows; possible
  tier at 60 hits. Calibrated at 262 tracks (`docs/calibration.md`).
- Development set 11/11, 0 wrong. Test set (owner-corrected manifest):
  15/16, 0 wrong; 2 of 5 test evaluations used (ledger).
- Commands: `gunfinger index|identify|stats`; `gunfinger-eval
  validate|survival|clusters|sweep|scan|calibrate`.
- Open decision: library size beyond 32,768 assets (ADR 0007).

## Rules for this session

- Everything from `docs/brief.md` sections 3 to 5 still applies: corpus
  read-only, derived data in `work/`, logs to `work/logs/`, `--release` for
  every experiment, `scripts/check.sh` green before every commit, commit at
  every milestone, never `git add -A` without checking, no remotes, no
  pushing, nothing published anywhere.
- House style from `AGENTS.md`. New dependencies must earn their place;
  record why in the commit message.
- **Save every experiment immediately**: `docs/experiments/NNNN-slug.md`
  (at most 40 lines; numbering continues from 0007) plus data in
  `docs/experiments/data/` (file names only, no audio). Write the note before
  starting the next experiment. Assume context can be lost at any moment.
- **Anything worth telling the owner** goes into `docs/notes-for-owner.md`
  as soon as it is found, not at the end.
- **Default detection behaviour does not change without the full check**:
  sweep, development scan with both leave-outs, `calibrate`, and an entry in
  `docs/calibration.md`. Experimental algorithms are opt-in (a flag or a
  harness command) until they pass that check.
- **Test set**: 3 evaluations left. Do not spend one on display-only
  features. Spend one only if default detection behaviour changes, and log
  it in the ledger. When in doubt, leave it for the owner.
- No GitHub Actions (cannot be tested here).

## Scope, in priority order

### 1. Test infrastructure

- Synthetic end-to-end test that needs no private corpus: deterministic
  synthetic tracks and a synthetic mix (speed changes, crossfade, an
  unindexed insert, a skip), run through the real binaries in `cargo test`;
  skip with a message when FFmpeg is missing.
- `gunfinger-eval` regression command: save baseline reports, rerun sweep
  and development scans, and report what changed.
- Check whether search is deterministic (two identical runs, byte for
  byte); fix it if not.
- Property tests where real invariants exist (plays, timecode, hashing).

### 2. Algorithm experiments on transformed copies of the library

Each one is a harness command or an opt-in path, with results saved.

- Scale proxy: pad the index with time-reversed (and speed-scaled) copies of
  the library's peaks; measure query time and the strongest false candidate
  against index size (ADR 0007, `docs/calibration.md`).
- Lookup-cost levers: skipping the most common hashes; longer hashes.
- Robustness: EQ (bass boost and cut, low-pass, telephone band), noise, low
  bitrates, crossfaded blends of two library tracks, radio inserts (speech
  from macOS `say`), needle skips, gradual pitch changes, speeds beyond ±8%.
- Key-locked mixes (FFmpeg `atempo`): measure the failure, prototype a
  tempo-only ladder (hop scaled, window and bins not).
- Related recordings (remixes) from the library self-match, so `calibrate`
  can separate remixes from unrelated records.

### 3. Command line

- Colour (`--color auto|always|never`, `NO_COLOR`), never in JSON.
- Position in the track for each segment.
- Rows for files with the same audio at the same time collapsed into one.
- Timeline view.
- `explain <mix> <asset>`: hits per window along the best alignment.
- `listen`: render the mix excerpt and the library track at the detected
  speed, and play them.
- Export: cue sheet, CSV, plain tracklist.
- Batch identification of several mixes with one index load.
- Shell completions and a man page; `--quiet`; progress on a terminal.
- XDG/TOML configuration; `doctor`; `prune` with safety checks; remembered
  failed files.
- TUI last, only if everything else is done.

## Definition of done for the session

As much of the scope as time allows, each item committed with the gate
green, experiments written up, `README.md`, `AGENTS.md`, `docs/roadmap.md`,
`docs/calibration.md` and `docs/status.md` true, and a morning summary at
the top of `docs/notes-for-owner.md`.
