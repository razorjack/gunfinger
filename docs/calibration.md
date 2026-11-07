# Calibration register

Gunfinger's thresholds and design choices were measured on one library: 262
tracks, 29.3 hours. Chance alignments get stronger and posting lists longer
as the library grows, so some of these numbers will move: a threshold
follows the score of the highest-scoring wrong track, which depends on
the number of tracks (Wang 2003, §2.3.1; `docs/references.md`). This page
lists what was chosen against this library, what each choice rests on,
how to measure it again, and what to change or undo if the measurement
moves.

## Library-dependent choices

| Choice | Value | Code | Rests on (at 262 tracks) | If it moves |
|--------|-------|------|--------------------------|-------------|
| Confident rule of the default (fitted) matcher | 240 hits, 3 windows | `confidence.rs` `FITTED_RULE` | over four sweep draws and the development scans, with the link rules: strongest false 99 (the passage the Clockwork remix shares with the original), weakest identifying 658 (Illuminati - Melange at -3%), margin 6.65× (experiments 0026, 0030, 0047). Since ADR 0022 a detection of a file judged to carry the truth's title (`docs/same-titles.txt`) is right and no false candidate: over the NAS sweeps of seeds 2026-2028 with the Stakka scans, false accepted 25 → 7 (all Cool Rock Stuff ~ the Mobb Deep / Urban Takover upload), strongest false 282 and weakest identifying 675 unchanged (experiment 0233); with seed 2029 as well, the same (7 accepted, 282, 675; 18 same-title credits; experiment 0239). On 2026-10-30 the Cool Rock pair was judged by ear to carry different titles, so the strongest false (282) is a confirmed different version | Keep it about twice the strongest false candidate and well below the weakest identifying one, as for `MIN_HITS`. |
| Confident rule of `--single-pass`, hits | 200 | `confidence.rs` `MIN_HITS` | strongest false candidate 95 hits (a remix; 97 with both playbacks, experiment 0016), weakest identifying 501 (experiment 0004); over four sweep draws 97 and 403 (experiment 0025) | Keep it about twice the strongest false candidate. If that collides with the weakest identifying detection, hits alone no longer separate them; the statistic needs rethinking (ADR 0001). |
| Possible tier | 60 hits | `confidence.rs` `MIN_POSSIBLE_HITS` | strongest unrelated false candidate 28 hits; audio not in the index 19 (experiment 0006), 28 over four sweep draws (experiment 0025), 22 under the fitted matcher (experiment 0047); strongest chance alignment with 30 reversed copies per record (8,122 assets) 30 hits (experiment 0012). On real records under the default matcher: 21, 27, 34, 64 and 64 hits at 209, 1,205, 3,202, 9,196 and 26,969 assets (sweep 2026, experiment 0052) | Raise it to at least twice the strongest unrelated false candidate. If that reaches 200, delete the tier (ADR 0006). Proposed, not applied: 60 while that chance stays below 30 hits (to about 3,000 assets), twice it above (130-140 at NAS scale). The jump to 64 hits is one pair, Clockwork against Simon Static - Rubba Rock, and it is not chance: by ear they share a stab sample (2026-10-12), as every strongest match between different recordings checked since session 10 shares material. The two strongest not checked before (41 and 40 hits, both below the tier) share material too, by ear (2026-10-14): every match checked from 40 hits up shares material, and no chance match has been heard above 40 hits. The third development mix's possible plays (96 to 197 hits) all share material with the track playing (experiment 0102). The proposal took the 64 for chance; we decide, and 60 stays. These breakpoints are measurements of this collection, not a rule for any library. |
| Pairs per anchor and peak density | fan-out 2 (Wang 2003, §2.2), ±12 × ±12 | `hash.rs` `FAN_OUT`, `profile.rs` | leanest variant with the best margins (experiment 0004) | If margins shrink at scale, rerun the density variants of experiment 0004 on the larger library. |
| Duplicate clusters | one alignment covers ≥ 80% of the shorter file, in its own seconds; candidates on the turntable ladder (±8%), the better of the ladder's alignment and the pair's at its fitted speed; the verdicts by ear override, and a `different` verdict that a chain of joins contradicts stops `clusters` | `gunfinger-eval` `clusters.rs`, `docs/pair-verdicts.txt` | same-recording pairs ≥ 0.984, all others ≤ 0.31 (experiments 0002, 0049); the same clusters from stored peaks (experiment 0023). At NAS scale 0.840 against 0.766, 28 pairs between 40% and 80% (experiment 0049); joins have a median 18.6 hits per second of aligned span, the sparsest 0.7 (DJ Trace - Sonar against its "Mark System Revision", judged a different recording by ear) | Rerun `clusters` after adding tracks (`clusters --from-peaks` first, to compare cheaply); check that the gap holds. An edit with a cut in the middle is never a duplicate under this criterion; the verdicts by ear decide such pairs. Listen to the joins `clusters` lists as sparse (under a tenth of the median hits per second): coverage alone does not show how much evidence an alignment has. `pair` reports supported time, the seconds holding aligned hits: the Sonar joins 0.39-0.43, random joins 0.98-1.00, China Cup against its Prototype upload 0.96-0.98 at two offsets (experiment 0057). |
| Posting layout | 16 asset bits, 16 frame bits (17 and 15 until session 9) | `index.rs` `Posting` | 65,536 assets and tracks up to 17:28 at most (ADR 0010); the NAS store passed 32,768 records on 2026-10-09 (32,905) | A hard limit. Past 65,536 assets shard by asset range (ADR 0007, option A). The default track limit, 17:00 (`config.rs` `DEFAULT_MAX_TRACK`), stays below 17:28. |

Not library-dependent: the speed ladder (0.4% steps, experiment 0001), the
3-window minimum (a 30 s excerpt spans 3 windows; a 15-20 s play only at
some places on the window grid, experiment 0020), chain linking, the
second suppression's share (0.6%, the chain link rule's 1.5 steps of the
ladder: overlapping detections within it are one alignment; the
confident repeats and versions the old rule dropped lie at most 0.40%
apart, the sides of a 1% ride at least 0.90%;
ADR 0021, experiment 0179; a ladder with another step moves it, so
measure it again with `--apart-in-speed`), the 90 s gap between segments
of a play, and the 90 s scoring tolerance.

## Baseline to compare against

| Measurement | At 262 tracks |
|-------------|---------------|
| Sweep (seed 2026) | 100% recall at every speed, 0 wrong, speed error ≤ 0.016% |
| Development set | 11/11, 0 wrong; leave-out 3: 8/11, 0 wrong; leave-out 11: 0/11, 0 wrong |
| Second development set (`razorjack-2003-03-29`) | not yet scanned by the harness. `identify` against the NAS store: 12/12 confident, 0 wrong (checked by ear), weakest track 2,233 hits (experiment 0060). At 32,905 records: all 12, Phantom Force from a PHUD1 rip at 6,513 hits, and one confident title not played, a mixed-CD track whose last 1:11 already plays Phantom Force (1,364 hits; experiment 0063) |
| Test set (corrected manifest) | 14/16, 0 wrong; Sin (123 hits) and Sick Note (233 hits in 4 windows) found as possible; no possible play matches no track (ledger, evaluation 3: the default matcher of ADR 0008 with both playbacks). Evaluation 2, the single pass with turntable alone: 15/16 with Sick Note confident at 209 hits under its rule of 200 |
| `calibrate`, confident rule | weakest identifying 501 hits, strongest false 97, margin 5.16× (both playbacks; turntable alone: 95, 5.27×) |
| `calibrate`, possible tier | false candidates ≥ 30 hits: only the Stakka remix of Clockwork; strongest unrelated 28; audio not in the index 19 |
| `calibrate` over sweep seeds 2026-2029, default matcher (ADR 0008) | weakest identifying 658 (Illuminati - Melange at -3%), strongest false 99, margin 6.65×; audio not in the index 22 (experiment 0047). On the Linux machine with the Stakka scans (session 57): 675, 100, 6.75×; 21 (experiment 0239) |
| The same, `--single-pass` | weakest identifying 403 (Fibre Optix - Sin at -3%), strongest false 97, margin 4.15×; audio not in the index 28 (experiment 0025) |
| `mixes --count 12`; `grid`, default matcher | 106 confident, 9 possible, 0 wrong; confident at 10, 15, 20 s: 0, 67, 146 of 160, 0 wrong (single pass: 107 and 8; 8, 76, 148; experiment 0047). `grid` on the Linux machine: 0, 65, 144, as with session 32's first build there, so the Linux machine's store and peaks (session 32) and not a rule move it (experiment 0239) |
| Query time, development mix | turntable alone 22 s, of which 2.6 s lookups, lines and chains (experiment 0005); both playbacks 261 CPU seconds against 166 for turntable alone (experiment 0018) |
| `gunfinger stats` | 74.4 postings/s, 5.07 bytes per posting, buckets p99 55 |

## Measured at NAS scale (session 5)

Moved verbatim to `docs/history/calibration-history.md` in session 76,
with the paragraphs of sessions 6, 9, 10 and 11 that followed it: NAS-scale
measurements at earlier store revisions, under which no rule changed.
Today's NAS-scale values are in the first table's "Rests on" column
and the roadmap's "Behaviour at scale".

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
   in the ledger (four of five used so far).

## Changes since `poc-freeze-1`

Moved verbatim to `docs/history/calibration-history.md` in session 76: the
table of changes since the proof of concept, with each change's effect on
results and how to undo it, up to ADR 0021.

## Measured, not adopted

| Candidate | Evidence | To adopt |
|-----------|----------|----------|
| Empty the fullest 1% of posting lists (`Index::without_fullest(0.01)`) | passes the full protocol (experiment 0017): sweep 100%, development 11/11, leave-outs 0 wrong; margin 5.86× (weakest identifying 445, strongest false 76); audio not in the index 15; 62% fewer postings scanned, no speed change at 262 tracks (0013) | call it in `catalog.rs` after `Index::build`, rerun the protocol and one test evaluation; every hit count drops by about 17% |
| The second pass at the fitted speed without skipping the fullest lists | at 240 hits: sweeps 540/540 for seeds 2026-2029, development 11/11, leave-outs 0 wrong; weakest identifying 660, strongest false 97, margin 6.80×; audio not in the index 24; robust, both ladders, 1,997 confident against 1,959 today, no wrong answer (experiments 0024, 0026, 0027) | superseded by the default (ADR 0008), which keeps its evidence at lower cost |
| The second pass with the fullest 1% emptied (`--drop-fullest 0.01`) | at 240 hits passes the protocol (weakest identifying 520, strongest false 91, margin 5.71×) but keeps 72-96% of today's own-track evidence; robust 1,884 confident against 1,959 today (experiments 0026, 0027) | not recommended; the skip variant is better |
| Weak chain ends left out of the boundaries (`Options::trim_weak_ends`; harness `--trim-ends`) | detection levels unchanged under both matchers; mixes' overshoot 86.0 → 1.3 s under today's matcher but 218.8 s of true play lost on 72 of 115 plays; under skip at 240 overshoot 7.6 → 7.1 s for 221 s lost (experiment 0031) | not recommended; the link rules fix the overshoot at a tenth of the cost |
| The second pass fitting the speed per stretch of 3 windows (`Options::speed_per_stretch`; harness `--speed-per-stretch`) | under skip at 240 without the link rules: sweeps 540/540 for seeds 2026-2029, margin 5.53× unchanged, development 11/11, leave-outs 0 wrong; Star Trails 1,558 → 1,733 hits, robust pitch ride +9.1% hits, wow unchanged, Dominion -3.2%; strongest false in the mixes 42 → 50; no level changes; no measurable CPU (experiment 0032) | set `speed_per_stretch: true` in `Matcher::Fitted`'s options; rerun the protocol |
| A rule on the peak verifier's share of reference peaks found (`gunfinger-eval --verify`, tolerance ±1 bin, ±1 frame; Sonnleitner & Widmer 2016, §VI-C) | identifying detections find a median 0.84-0.85 of their reference peaks, chance 0.004-0.010. At equal hits it separates no better than hits: AUC 0.953 against 0.950 over 6-100 hits at 262 tracks, 0.827 against 0.865 against related records; weak blend partners (excess 0.16-0.24) verify like shared passages (0.11-0.41). At NAS scale another upload of the played recording finds a median 0.65 (experiments 0054, 0056, 0058) | not recommended as a rule; the share can order a listening list |
| Three extra rungs at each end of both ladders, to ±9.2% (`speed::ladder_with_extra_rungs`; harness `--extra-rungs 3`) | robust speeds ±8.2% to ±9%: 40/40 confident at every speed under both matchers, against 0-11 at ±9% today; sweeps 2026-2029, margins (4.15×, 5.53×), development 11/11, leave-outs 0 wrong, mixes and grid unchanged; false candidates +11% (today's) and +8% (skip); 14% more search CPU (experiment 0033). On other rips alone at NAS scale: 3 of 4 queries at 9.2-10% speed relative to the remaining copy recovered, none of 31 at 10.2-12.1%; 2 more wrong answers from China Cup's Prototype upload (experiment 0059; that upload, PRO 001, joined China Cup's cluster by a verdict on 2026-10-09) | build the ladders with 3 extra rungs in `identify` and `explain`; rerun the protocol |
| Peak neighbourhoods that widen with frequency, 9.4% of the bin within 4-24 bins (`Profile::spread`, variant a; a store of its own) | full protocol under both matchers: sweeps 2,160/2,160, development 11/11, leave-outs 0 wrong, mixes' levels unchanged; 9% fewer postings; development mix +34% true hits with 36% fewer postings looked up; margin 4.15× → 4.96× (today's), 5.53× → 5.82× (skip); robust level (1,955 against 1,959; skip 1,993 against 1,995); false candidates of 30 hits or more 55 → 94 and 41 → 65, nearly all related records; audio not in the index 28 → 43 and 24 → 50 hits. Variant b (4-16 bins): +41% true hits, robust 1,991, margins 4.31× and 5.64× (experiment 0034) | set `spread` in `Profile::CURRENT`, extract every peak record again, rerun the protocol and one test evaluation |
