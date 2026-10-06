# Calibration register

Gunfinger's thresholds and design choices were measured on one library: 262
tracks, 29.3 hours. Chance alignments get stronger and posting lists longer
as the library grows, so some of these numbers will move. This page lists
what was chosen against this library, what each choice rests on, how to
measure it again, and what to change or undo if the measurement moves.

## Library-dependent choices

| Choice | Value | Code | Rests on (at 262 tracks) | If it moves |
|--------|-------|------|--------------------------|-------------|
| Confident rule, hits | 200 | `confidence.rs` `MIN_HITS` | strongest false candidate 95 hits (a remix), weakest identifying 501 (experiment 0004) | Keep it about twice the strongest false candidate. If that collides with the weakest identifying detection, hits alone no longer separate them; the statistic needs rethinking (ADR 0001). |
| Possible tier | 60 hits | `confidence.rs` `MIN_POSSIBLE_HITS` | strongest unrelated false candidate 28 hits; audio not in the index 19 (experiment 0006); strongest chance alignment with 30 reversed copies per record (8,122 assets) 30 hits (experiment 0012) | Raise it to at least twice the strongest unrelated false candidate. If that reaches 200, delete the tier (ADR 0006). |
| Pairs per anchor and peak density | fan-out 2, ±12 × ±12 | `hash.rs` `FAN_OUT`, `profile.rs` | leanest variant with the best margins (experiment 0004) | If margins shrink at scale, rerun the density variants of experiment 0004 on the larger library. |
| Duplicate clusters | one alignment covers ≥ 80% of the shorter file | `gunfinger-eval` `clusters.rs` | same-recording pairs ≥ 0.984, all others ≤ 0.39 (experiment 0002) | Rerun `clusters` after adding tracks; check that the gap holds. |
| Posting layout | 15 asset bits | `index.rs` `Posting` | 32,768 assets at most (ADR 0005) | A hard limit; the options for going past it are in ADR 0007 (open). |

Not library-dependent: the speed ladder (0.4% steps, experiment 0001), the
3-window minimum (a 30 s excerpt spans 3 windows), chain linking, the 90 s
gap between segments of a play, and the 90 s scoring tolerance.

## Baseline to compare against

| Measurement | At 262 tracks |
|-------------|---------------|
| Sweep (seed 2026) | 100% recall at every speed, 0 wrong, speed error ≤ 0.016% |
| Development set | 11/11, 0 wrong; leave-out 3: 8/11, 0 wrong; leave-out 11: 0/11, 0 wrong |
| Test set (owner-corrected manifest) | 15/16, 0 wrong; Sin found as possible; no possible play matches no track (ledger, evaluation 2) |
| `calibrate`, confident rule | weakest identifying 501 hits, strongest false 95, margin 5.27× |
| `calibrate`, possible tier | false candidates ≥ 30 hits: only the Stakka remix of Clockwork; strongest unrelated 28; audio not in the index 19 |
| Query time, development mix | 22 s, of which 2.6 s lookups, lines and chains (experiment 0005) |
| `gunfinger stats` | 74.4 postings/s, 5.07 bytes per posting, buckets p99 55 |

## After indexing more tracks

0. Before indexing, `gunfinger-eval baseline <name>` keeps the reports at
   the old size; `gunfinger-eval regress <name> --no-rerun` after step 4
   shows what moved.
1. `gunfinger index <library>`, then `gunfinger-eval clusters` (new rips
   join clusters).
2. `gunfinger-eval sweep --seed 2026`. The draw depends on the library, so
   compare recall and wrong answers, not individual excerpts.
3. `gunfinger-eval scan stakka-skynet-knowledge`, with `--leave-out 3
   --seed 2026` and `--leave-out 11 --seed 2026`.
4. `gunfinger-eval calibrate`: compare both blocks with the baseline and
   apply the "If it moves" column.
5. `gunfinger stats`, and query time as in experiment 0005.
6. Record the run as an experiment. A test-set scan counts as an evaluation
   in the ledger (two of five used so far).

## Changes since `poc-freeze-1`

| Change | Effect on results | To undo |
|--------|-------------------|---------|
| Elegance pass (commit 5559c99) | none: identical reports, except speeds that differ by at most 4.4e-16 because the speed fit was rewritten (experiment 0007) | nothing to undo |
| Owner corrected test track 16 to "Funktion (Remix)", absent from the library | test score 15/16 instead of 15/17 | revert the manifest |
| Plays and the possible tier (ADR 0006, commit e87b79f) | `identify` output and JSON schema 2; scoring unchanged | the tier: delete `MIN_POSSIBLE_HITS` and `Confidence::Possible`; plays: delete `plays.rs` and restore the detection list in `identify.rs` |
| Session 2 core changes: rungs as `Rung::{Turntable, KeyLocked}`, search progress, track positions, `hash::targets` and `hash::pair_hash` | none: `regress session-2-start` and the development scan give identical detections after each | nothing to undo |
| Key-locked rungs, opt-in (`identify --playback both` or `key-lock`, `playback` in the configuration file; experiments 0010, 0016) | default unchanged. With both ladders the full protocol passes with the same thresholds: sweep 100%, development 11/11, leave-outs 0 wrong, margin 5.16× (strongest false 97), audio not in the index 19 | making `both` the default needs no recalibration at this library size; measure search time first |

## Measured, not adopted

| Candidate | Evidence | To adopt |
|-----------|----------|----------|
| Empty the fullest 1% of posting lists (`Index::without_fullest(0.01)`) | passes the full protocol (experiment 0017): sweep 100%, development 11/11, leave-outs 0 wrong; margin 5.86× (weakest identifying 445, strongest false 76); audio not in the index 15; 62% fewer postings scanned, no speed change at 262 tracks (0013) | call it in `catalog.rs` after `Index::build`, rerun the protocol and one test evaluation; every hit count drops by about 17% |
