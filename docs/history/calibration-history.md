# Calibration register: history

Two sections of `docs/calibration.md`, moved here verbatim in session 76
(from commit `009fe55`) because they record past measurements and changes
rather than today's values. Their headings are unchanged.

## Measured at NAS scale (session 5)

The NAS collection from its peak store alone: 26,890 records,
2,806 hours; the harness index is the corpus plus those records less
the 238 that copy corpus files, 26,914 assets (experiments 0035-0044).
These are measurements; no rule was changed. "Today's matcher" is
`--single-pass` since session 6, "skip at 240" the default. Under the rules every
detection of a record the clusters do not join is false, so the figures
are given with and without the groups still to confirm by ear.

| Measurement | Today's matcher | Skip at 240 | At 262 tracks (today's; skip) |
|---|---|---|---|
| Sweeps | seeds 2026-2027: 1,080/1,080 | seeds 2026-2029: 2,160/2,160 | the same |
| Weakest identifying | 403 | 661 | 403; 658 |
| Strongest false, under the rules | 5,785 (another upload), 0.07× | 5,845, 0.11× | 97; 119 |
| Without other uploads with the same artist and title | 342 (Coma ~ Spraycan), 1.18× | 511 (Synthesis VIP ~ "Synthesis (Remix)"), 1.29× | |
| Without those two pairs as well | 200 (The Nine ~ Evol Intent VIP), 2.02×; without it 126, 3.20× | 220 (The Nine ~ Evol Intent VIP), 3.0× | 4.15×; 5.53× |
| Strongest unrelated (different artist and title) | 78 | 67-70 | 28; 21-24 in the mix |
| Development scan | 11/11, 6 wrong (faster uploads), unrelated 40 | 11/11, the same 6, unrelated 48 | 11/11, 0 wrong |
| Duplicate clusters | same recording down to 0.825, different up to 0.737; uploads 2.9-4.7% fast never join (ladder 0.98-1.02). Session 6 (±8%, experiment 0049): 0.840 and 0.766, 216 further rips, 28 borderline pairs | | ≥ 0.984; ≤ 0.39 (session 6: ≤ 0.31) |
| Development mix: wall; CPU; peak | 848 s; 6,106 s; 8.42 GB | 259 s; 1,958 s; 4.01 GB | 34 s; 264 s; 399 MB / 31 s; 238 s; 293 MB |
| `stats` | 755.9 M postings, 4.01 bytes per posting, buckets p99 4,538 | | 5.07 bytes, p99 55 |

Session 6, under the default matcher and the clusters on the ±8%
ladder (experiments 0049, 0051, 0053; store revision
`d428ee9585936326`): development scan 11/11 with 0 wrong; sweeps
2026-2029 540/540 each, 0, 24, 8 and 8 wrong (three pairs to
check by ear); over the four seeds weakest identifying 661, strongest false
511 (Synthesis VIP ~ "Synthesis (Remix)" on a mixed CD), 1.29×; without
the three pairs 197 (The Nine ~ Evol Intent VIP), 3.36×; different
recordings 63-70 hits, all shared material (by ear, 2026-10-12).
The margin compares the weakest identifying
detection with the strongest false one; the rule's own headroom is
smaller. Without the three pairs the rule of 240 is 1.22× the strongest
false candidate (197), where the "If it moves" column asks for about
2× (about 400, still below the weakest identifying 661). That
candidate is a VIP of the played track, so this is the question of a
statistic for shared material (roadmap), not of the threshold alone.
By ear, the three pairs have since been judged one recording each and The
Nine's VIP a different recording (`docs/pair-verdicts.txt`,
2026-10-09); these figures change at the next clusters run.

What the "If it moves" column would say (not applied): the confident
rule collides with the weakest identifying detection only through other
uploads of the same track, a question for the clusters, not the
statistic; without them, twice the strongest false candidate is 400
under today's matcher (403 weakest) and 440 under skip at 240 (661). The
possible tier's condition is met: twice the strongest unrelated false
candidate is 140-156 hits, still below 200. The clusters' gap narrowed
and the ladder misses fast uploads; widening it to ±8% is proposed in
`docs/session-notes.md`.

Session 9, costs only (store revision `b8bb402f0ba23761`, 32,905
records, 3,433 h; experiments 0062-0064; no rule measured): 922.0 M
postings, 3.70 GB, 4.01 bytes per posting, buckets p99 5,515 and max
123,575, the fullest 1% holding 26.0%; the index builds in 65-72 s.
Store-only `identify`: the development mix 315 s, 2,344 s CPU, 4.76 GB;
the 2003 mix 266 s, 1,854 s CPU, 4.66 GB, both about 20% above 27,042
records, in line with the postings (+21%). `index` over Ethernet: 1.91
s per hour of audio, CPU-bound, so a full re-index takes about 1.8 h.
32,905 is 50% of the 65,536 assets a posting now addresses (ADR 0010).
Rules wait for the next clusters run: the new folders hold rips and
mixed-CD tracks the clusters do not know yet.

Session 10, the NAS baseline (store `5701f221f7b48ee1`, 32,441 records
after the prune; the clusters of experiment 0066, with the
verdicts by ear and one join cut by `--cut-sparsest`; experiment
0067): development scan 11/11, 0 wrong; leave-outs 3 and 11 8/11 and
0/11, 0 wrong; sweeps 2026-2029 540/540 each, 4, 0, 8 and 12 wrong (40
before), all from one mixed disc the ignore file does not list
(Essential Rewindz's Funktion and The Nine, borderline at 58-78%); over
the four seeds weakest identifying 661, strongest false 382 (The Nine on
that disc), 1.73×; without same-name pairs 220 (The Nine ~ Evol Intent
VIP, judged different), 3.00×; different recordings 63-70 hits (shared
material) outside that disc's blends (142-177). My 2003 mix
(`razorjack-2003-03-29`):
12/12 at NAS scale and 7/7 referenced at 262 tracks, 0 wrong, leave-outs
0 wrong; it leaves both margins unchanged (1.73×, 6.65×). The rule stays
240 hits in 3 windows: what passes it falsely is the unlisted mixed
disc, a question for the ignore file and for judging by ear, not for the
threshold. Costs: sweeps 1,535-1,584 s, 14,012-14,535 s CPU, 6.3-6.7 GB;
scans 307-315 s, 2,281-2,328 s CPU, 4.8 GB (+19-20% CPU for +20%
records).

Session 11, the NAS baseline after the prune (store
`f9f9bc706195a71b`, 33,596 records: eight patterns for mixed discs beside
unmixed ones, 1,252 new uploads; the clusters of experiment 0071;
experiment 0073): development scan 11/11, 0 wrong; leave-outs 3 and 11
8/11 and 0/11, 0 wrong; sweeps 2026-2029 540/540 each and 0 wrong (24
before, all from Essential Rewindz's mixed disc 2, now pruned); over the
four seeds weakest identifying 661, strongest false 220 (The Nine ~ Evol
Intent VIP, judged different), 3.00× with and without same-name pairs;
on audio not in the index 197 (the same VIP), 3.36×; different
recordings 63-70 hits (shared material). The 2003 mix: 12/12 at NAS
scale and 7/7 at 262 tracks, 0
wrong; margins 3.00× and 6.65×. The rule stays 240 hits in 3 windows:
nothing false reaches it, and the nearest candidate (1.09× below) is a
VIP of the played track, the question of a statistic for shared
material. Costs: sweeps 1,666-1,724 s, 14,619-15,129 s CPU, 6.8-6.9 GB;
scans 2,337-2,368 s CPU, 4.9 GB (wall times under other load).

Session 11, research (experiment 0076, measurement only): played whole
from one file, the shared passages of 17 pairs of related versions give
the other version a confident play in 18 of 34 directions (The Nine's
Evol Intent VIP 21.7× the rule, "Fire" versions 1.2-2.5×, Sonar's
revision 1.6-2.3×), while the sweeps' 30 s excerpts give at most 220
hits. The sweep margins above therefore say nothing about a version over
a long shared passage. Test evaluation 4 (21/23, 2 wrong: Mind
Overload's Molten Beats edit and The Nine's VIP) is not an input here.

## Changes since `poc-freeze-1`

| Change | Effect on results | To undo |
|--------|-------------------|---------|
| Elegance pass (commit 83754c9) | none: identical reports, except speeds that differ by at most 4.4e-16 because the speed fit was rewritten (experiment 0007) | nothing to undo |
| Test track 16 corrected to "Funktion (Remix)", absent from the library | test score 15/16 instead of 15/17 | revert the manifest |
| Plays and the possible tier (ADR 0006, commit a58e7a9) | `identify` output and JSON schema 2; scoring unchanged | the tier: delete `MIN_POSSIBLE_HITS` and `Confidence::Possible`; plays: delete `plays.rs` and restore the detection list in `identify.rs` |
| Session 2 core changes: rungs as `Rung::{Turntable, KeyLocked}`, search progress, track positions, `hash::targets` and `hash::pair_hash` | none: `regress session-2-start` and the development scan give identical detections after each | nothing to undo |
| Key-locked rungs, opt-in (`identify --playback both` or `key-lock`, `playback` in the configuration file; experiments 0010, 0016) | default unchanged. With both ladders the full protocol passes with the same thresholds: sweep 100%, development 11/11, leave-outs 0 wrong, margin 5.16× (strongest false 97), audio not in the index 19 | delete `Rung::KeyLocked` and `--playback` |
| Both playbacks by default, our decision of 2026-10-06 (`identify`, `explain` and the `gunfinger-eval` ladder; experiment 0018) | the protocol results of experiment 0016 become the baseline. On the development mix the same 14 plays, none with key lock; Side Effects starts 23 s later, Bios-Fear and the Clockwork remix gain 3 hits each. Search costs 1.57 times the CPU time | the default in `config.rs` (`PlaybackChoice::Both`) and the `--ladder` default in `gunfinger-eval`; move `work/reports/ladder-turntable/` back to `work/reports/` |
| Session 3 changes without effect on detections: the index built in two passes, reports that record their settings, atomic report writes, `explain --windows`, `Evidence::pass`, the opt-in `search::search_twice` and `search::search_peaks`, sweep panels, `Index::skipping_fullest` | none: `regress session-3-start` gives identical detections after each | nothing to undo |
| The opt-in second pass's rule set from its own calibration: `FITTED_RULE` 240 hits in 3 windows, possible 60 (experiment 0026) | default unchanged (`regress session-3-start`: identical); applies only to `--second-pass` in the harness | set `FITTED_RULE.min_hits` back to `MIN_HITS` |
| The fitted matcher by default, our decision of session 6 (`search::Matcher::Fitted`: the fullest 1% of posting lists skipped for candidates, the link rules, the second pass, `FITTED_RULE`; ADR 0008, experiment 0047) | the candidate baseline reproduced exactly; sweeps 540/540 for seeds 2026-2029, development 11/11, leave-outs 0 wrong; margin 4.15× → 6.65×; true plays 0.84-1.16 times the hits; development detections matching no track 6,096 → 69; mix 10's Dominion and the grid's 10 s plays confident → possible; Star Trails split where its speed changes | `Matcher::default()` back to `SinglePass` (or `--single-pass`); the single pass's reports are in `work/reports/variant-single-pass/` |
| Store fixes (session 6): MP3 in WAV behind an ID3v2 tag, hidden folders skipped, truncation tolerance of the larger of 1 s and 1% | none: `regress session-6-start` identical; three corpus files would now pass the tolerance and were not indexed | revert commit 53c6990 |
| The second suppression keeps a detection at another speed, our decision of 2026-10-27 (`search::SAME_SPEED_SHARE` 0.006 in `SecondOverlap::SpansAtSpeed`, the default of `Matcher::Fitted`; ADR 0021, experiments 0179, 0209-0211 and 0232) | both sides of a pitch ride of 1% or more kept (all 257 sides the old rule dropped on rides seed 3801); Star Trails' slower side (Stakka mix, 315 hits) and Into Music's middle (2003 mix, 1,392 hits) kept; weak detections more than 0.6% from a stronger one of their file kept; no confident detection lost or added elsewhere; calibrate unchanged at both scales; the printed tracklists of the three development mixes unchanged | `--overlap-by-spans` in the harness; in the core, `SecondOverlap::default()` back to `Spans` and `search::design` back to `lines-chains-v1` |
