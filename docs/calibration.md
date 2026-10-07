# Calibration register

Gunfinger's thresholds and design choices were measured on one library: 262
tracks, 29.3 hours. Chance alignments get stronger and posting lists longer
as the library grows, so some of these numbers will move. This page lists
what was chosen against this library, what each choice rests on, how to
measure it again, and what to change or undo if the measurement moves.

## Library-dependent choices

| Choice | Value | Code | Rests on (at 262 tracks) | If it moves |
|--------|-------|------|--------------------------|-------------|
| Confident rule, hits | 200 | `confidence.rs` `MIN_HITS` | strongest false candidate 95 hits (a remix; 97 with both playbacks, experiment 0016), weakest identifying 501 (experiment 0004); over four sweep draws 97 and 403 (experiment 0025) | Keep it about twice the strongest false candidate. If that collides with the weakest identifying detection, hits alone no longer separate them; the statistic needs rethinking (ADR 0001). |
| Second pass's rule (opt-in) | 240 hits, 3 windows | `confidence.rs` `FITTED_RULE` | over four sweep draws and the development scans: strongest false 119 (the passage the Clockwork remix shares with the original, with the fullest lists skipped), 97 without the filter, 91 with them dropped; weakest identifying 520-660 (experiment 0026) | Keep it about twice the strongest false candidate and well below the weakest identifying one, as for `MIN_HITS`. |
| Possible tier | 60 hits | `confidence.rs` `MIN_POSSIBLE_HITS` | strongest unrelated false candidate 28 hits; audio not in the index 19 (experiment 0006), 28 over four sweep draws (experiment 0025); strongest chance alignment with 30 reversed copies per record (8,122 assets) 30 hits (experiment 0012) | Raise it to at least twice the strongest unrelated false candidate. If that reaches 200, delete the tier (ADR 0006). |
| Pairs per anchor and peak density | fan-out 2, ±12 × ±12 | `hash.rs` `FAN_OUT`, `profile.rs` | leanest variant with the best margins (experiment 0004) | If margins shrink at scale, rerun the density variants of experiment 0004 on the larger library. |
| Duplicate clusters | one alignment covers ≥ 80% of the shorter file | `gunfinger-eval` `clusters.rs` | same-recording pairs ≥ 0.984, all others ≤ 0.39 (experiment 0002); the same clusters from stored peaks (experiment 0023) | Rerun `clusters` after adding tracks (`clusters --from-peaks` first, to compare cheaply); check that the gap holds. An edit with a cut in the middle is never a duplicate under this criterion. |
| Posting layout | 15 asset bits | `index.rs` `Posting` | 32,768 assets at most (ADR 0005) | A hard limit; the options for going past it are in ADR 0007 (open). |

Not library-dependent: the speed ladder (0.4% steps, experiment 0001), the
3-window minimum (a 30 s excerpt spans 3 windows; a 15-20 s play only at
some places on the window grid, experiment 0020), chain linking, the 90 s
gap between segments of a play, and the 90 s scoring tolerance.

## Baseline to compare against

| Measurement | At 262 tracks |
|-------------|---------------|
| Sweep (seed 2026) | 100% recall at every speed, 0 wrong, speed error ≤ 0.016% |
| Development set | 11/11, 0 wrong; leave-out 3: 8/11, 0 wrong; leave-out 11: 0/11, 0 wrong |
| Test set (owner-corrected manifest) | 15/16, 0 wrong; Sin found as possible; no possible play matches no track (ledger, evaluation 2; turntable alone, not yet run with both playbacks) |
| `calibrate`, confident rule | weakest identifying 501 hits, strongest false 97, margin 5.16× (both playbacks; turntable alone: 95, 5.27×) |
| `calibrate`, possible tier | false candidates ≥ 30 hits: only the Stakka remix of Clockwork; strongest unrelated 28; audio not in the index 19 |
| `calibrate` over sweep seeds 2026-2029 | weakest identifying 403 (Fibre Optix - Sin at -3%), strongest false 97, margin 4.15×; audio not in the index 28 (experiment 0025) |
| Query time, development mix | turntable alone 22 s, of which 2.6 s lookups, lines and chains (experiment 0005); both playbacks 261 CPU seconds against 166 for turntable alone (experiment 0018) |
| `gunfinger stats` | 74.4 postings/s, 5.07 bytes per posting, buckets p99 55 |

## After indexing more tracks

0. Before indexing, `gunfinger-eval baseline <name>` keeps the reports at
   the old size; `gunfinger-eval regress <name> --no-rerun` after step 4
   shows what moved.
1. `gunfinger index <library>`, then `gunfinger-eval clusters` (new rips
   join clusters; `clusters --from-peaks` gives the same result at a
   seventh of the CPU and compares).
2. `gunfinger-eval sweep --seed N` for 2026 to 2029. Each seed's held-out
   recordings and excerpts are kept in `docs/panels/`, so individual
   excerpts compare across sizes; new tracks are indexed, and new rips of
   a held-out recording stay held out once step 1 has clustered them. A
   panel whose excerpt's file is gone is an error.
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
| Key-locked rungs, opt-in (`identify --playback both` or `key-lock`, `playback` in the configuration file; experiments 0010, 0016) | default unchanged. With both ladders the full protocol passes with the same thresholds: sweep 100%, development 11/11, leave-outs 0 wrong, margin 5.16× (strongest false 97), audio not in the index 19 | delete `Rung::KeyLocked` and `--playback` |
| Both playbacks by default, the owner's decision of 2026-10-06 (`identify`, `explain` and the `gunfinger-eval` ladder; experiment 0018) | the protocol results of experiment 0016 become the baseline. On the development mix the same 14 plays, none with key lock; Side Effects starts 23 s later, Bios-Fear and the Clockwork remix gain 3 hits each. Search costs 1.57 times the CPU time | the default in `config.rs` (`PlaybackChoice::Both`) and the `--ladder` default in `gunfinger-eval`; move `work/reports/ladder-turntable/` back to `work/reports/` |
| Session 3 changes without effect on detections: the index built in two passes, reports that record their settings, atomic report writes, `explain --windows`, `Evidence::pass`, the opt-in `search::search_twice` and `search::search_peaks`, sweep panels, `Index::skipping_fullest` | none: `regress session-3-start` gives identical detections after each | nothing to undo |
| The opt-in second pass's rule set from its own calibration: `FITTED_RULE` 240 hits in 3 windows, possible 60 (experiment 0026) | default unchanged (`regress session-3-start`: identical); applies only to `--second-pass` in the harness | set `FITTED_RULE.min_hits` back to `MIN_HITS` |

## Measured, not adopted

| Candidate | Evidence | To adopt |
|-----------|----------|----------|
| Empty the fullest 1% of posting lists (`Index::without_fullest(0.01)`) | passes the full protocol (experiment 0017): sweep 100%, development 11/11, leave-outs 0 wrong; margin 5.86× (weakest identifying 445, strongest false 76); audio not in the index 15; 62% fewer postings scanned, no speed change at 262 tracks (0013) | call it in `catalog.rs` after `Index::build`, rerun the protocol and one test evaluation; every hit count drops by about 17% |
| The second pass at the fitted speed (`search::search_twice`; harness `--second-pass`) | at 240 hits: sweeps 540/540 for seeds 2026-2029, development 11/11, leave-outs 0 wrong; weakest identifying 660, strongest false 97, margin 6.80×; audio not in the index 24; robust, both ladders, 1,997 confident against 1,959 today, no wrong answer (experiments 0024, 0026, 0027) | superseded by the row below, which keeps its evidence at lower cost |
| The second pass with the fullest 1% of posting lists skipped for candidates (`Index::skipping_fullest(0.01)`, `--second-pass --skip-fullest 0.01`) | at 240 hits: sweeps 540/540 for seeds 2026-2029, development 11/11, leave-outs 0 wrong; weakest identifying 658, strongest false 119 (the Clockwork remix's shared passage), margin 5.53×; audio not in the index 24; 94% fewer false candidates; robust, both ladders, 1,995 confident against 1,959 today (combined damage 7 → 4 of 80), no wrong answer; 57-63% less search CPU at 8,122 and 26,462 assets (experiments 0026, 0027) | in `identify` and `explain`, build the index with `skipping_fullest(0.01)` and search with `search_twice`; make both the `gunfinger-eval` default; rerun the protocol and one test evaluation (Sick Note, 209 hits today, would most likely become possible) |
| The second pass with the fullest 1% emptied (`--second-pass --drop-fullest 0.01`) | at 240 hits passes the protocol (weakest identifying 520, strongest false 91, margin 5.71×) but keeps 72-96% of today's own-track evidence; robust 1,884 confident against 1,959 today (experiments 0026, 0027) | not recommended; the skip variant is better |
| Link rules for chains (`search::Links`; harness `--nearby-rungs --strong-gaps`) | today's matcher: sweeps 540/540 for seeds 2026-2029 unchanged, margin 4.15× unchanged, development 11/11, leave-outs 0 wrong; 68% fewer false candidates; mixes' overshoot 86.0 → 7.6 s, mix 10's Dominion (21.5 s) confident → possible, the grid's 10 s plays 8 → 0 confident (all through chance windows). Skip at 240: margin 5.53× → 6.65× (strongest false 119 → 99), nothing else changes (experiment 0030) | pass `Links { nearby_rungs: true, strong_across_gaps: true }` in `identify` and `explain` (`search::Options`); rerun the protocol |
| Weak chain ends left out of the boundaries (`Options::trim_weak_ends`; harness `--trim-ends`) | detection levels unchanged under both matchers; mixes' overshoot 86.0 → 1.3 s under today's matcher but 218.8 s of true play lost on 72 of 115 plays; under skip at 240 overshoot 7.6 → 7.1 s for 221 s lost (experiment 0031) | not recommended; the link rules fix the overshoot at a tenth of the cost |
| The second pass fitting the speed per stretch of 3 windows (`Options::speed_per_stretch`; harness `--speed-per-stretch`, with `--second-pass`) | under skip at 240: sweeps 540/540 for seeds 2026-2029, margin 5.53× unchanged, development 11/11, leave-outs 0 wrong; Star Trails 1,558 → 1,733 hits, robust pitch ride +9.1% hits, wow unchanged, Dominion -3.2%; strongest false in the mixes 42 → 50; no level changes; no measurable CPU (experiment 0032) | pass `speed_per_stretch: true` with the second pass in `identify` and `explain`; rerun the protocol |
| Three extra rungs at each end of both ladders, to ±9.2% (`speed::ladder_with_extra_rungs`; harness `--extra-rungs 3`) | robust speeds ±8.2% to ±9%: 40/40 confident at every speed under both matchers, against 0-11 at ±9% today; sweeps 2026-2029, margins (4.15×, 5.53×), development 11/11, leave-outs 0 wrong, mixes and grid unchanged; false candidates +11% (today's) and +8% (skip); 14% more search CPU (experiment 0033) | build the ladders with 3 extra rungs in `identify` and `explain`; rerun the protocol |
| Peak neighbourhoods that widen with frequency, 9.4% of the bin within 4-24 bins (`Profile::spread`, variant a; a store of its own) | full protocol under both matchers: sweeps 2,160/2,160, development 11/11, leave-outs 0 wrong, mixes' levels unchanged; 9% fewer postings; development mix +34% true hits with 36% fewer postings looked up; margin 4.15× → 4.96× (today's), 5.53× → 5.82× (skip); robust level (1,955 against 1,959; skip 1,993 against 1,995); false candidates of 30 hits or more 55 → 94 and 41 → 65, nearly all related records; audio not in the index 28 → 43 and 24 → 50 hits. Variant b (4-16 bins): +41% true hits, robust 1,991, margins 4.31× and 5.64× (experiment 0034) | set `spread` in `Profile::CURRENT`, extract every peak record again, rerun the protocol and one test evaluation |
