# 0021: The second suppression keeps a detection at another speed

## Status

Accepted (2026-10-27), our decision. Supersedes ADR 0008's consequence
for overlapping parts: after the second pass, a detection whose measured
span overlaps a stronger one of the same asset is dropped only when their
fitted speeds are within 0.6%, no longer at any speed. ADR 0008's
matcher, its confidence rule (240 hits in 3 windows), its split
consequence and its needle-skip consequence stand. `--single-pass` and
ADR 0006 (plays, the possible tier) are unchanged.

## Context

DJs speed a set up while a track is audible, because the set is
continuous. When the pitch moves by about 1% or more within a play, the
first pass splits the play into two chains, one per speed (ADR 0008:
chains join only lines from rungs a step apart, 1.5 steps with
rounding). Each side is confident
after the second pass, but the second pass measures each chain up to
10 s beyond it, so the facing ends move into the ramp and the measured
spans overlap (median 4.7-5.2 s, experiment 0175). The second
suppression (`strongest_per_moment` on the measured spans) then dropped
the weaker side whole. The play stays found; it loses time, 30 s or more
in 21-43% of the sides of 1-2% rides (0176), and a printed gap would fall inside a
track that was playing. In the development mixes this drops Star Trails'
slower side (13:13-14:37 in the Stakka mix) and Into Music's middle
segment in the 2003 mix; nobody has listened to either.

`docs/research/2026-10-22-pitch-rides-decision.md` holds the options,
the shared-material risk and the questions this ADR answers.

## Decision

1. **The rule.** `Matcher::Fitted`'s second suppression is
   `SecondOverlap::SpansAtSpeed(SAME_SPEED_SHARE)`
   (`gunfinger-core/src/search.rs`): after the second pass, a detection
   is dropped when its measured span overlaps a stronger detection of
   the same asset and its fitted speed is within `SAME_SPEED_SHARE`
   (0.006) of the stronger one's, as a share of the stronger one's
   speed. A detection further apart in speed is kept. `identify`,
   `explain` and every harness command that searches with the default
   matcher use it. The first suppression, on first-pass chains, and the
   confidence rule do not change; no threshold of `docs/calibration.md`
   moves.
2. **The share.** 0.6% is 1.5 steps of the speed ladder (0.4% steps),
   the chain link rule's limit (`chains::NEARBY_SPEEDS`, a step and
   rounding): two alignments a chain could join are one alignment here
   too. It lies between the largest difference measured outside a ride
   among the confident detections the old rule dropped, 0.40% (repeats
   and versions within one play's alignment, on the same rung or the
   next, in every development trace of session 37), and the smallest of
   a 1% ride, 0.90% (0179). Weak alignments outside rides lie further
   apart (0.61-1.51% for a related version in 0210). `SAME_SPEED_SHARE` is defined as
   `NEARBY_SPEEDS`, so a ladder with another step moves it with the
   step; measure it again then.
3. **Overlapping segments are shown as measured.** The two sides of a
   ride overlap in reported time (median 5 s, about 20 s at most).
   Plays already group them (ADR 0006), and scoring counts either. A
   play's end is the end of its segment that ends last: with disjoint
   segments that is the last one, as before, but a weaker segment at
   another speed can now lie inside a stronger one, and the play must
   not end early (`plays.rs`; the CLI's report check rejects a segment
   that ends after its play). Clipping the weaker side at the stronger
   one's boundary was not taken: it changes the spans that scoring and
   the JSON report use.
4. **A wrong file's added segment is a wrong answer.** Scoring does not
   change: a confident detection of a file not playing counts as wrong,
   also where the default already names that file at the stronger
   speed (0178's V2). On the data measured the question did not arise
   (0179, 0209-0211).
5. **ADR 0008.** "A speed change of more than 0.6% between windows
   splits a play into segments" stays true; both segments are now kept.
   After a needle skip the parts lie at one speed, so the weaker part,
   which the margin makes overlap the stronger, is still dropped.
6. **Saved reports.** `search::design` goes from `lines-chains-v1` to
   `lines-chains-v2`, so `identify --save-dir` searches again instead of
   reusing a report saved under the old rule.
7. **The old rule stays measurable** in the harness as the opt-in
   `--overlap-by-spans` (reports in `variant-spans/`), beside
   `--overlap-by-chains`. `--apart-in-speed SHARE` varies the share.

## Evidence

- 0175-0177 (session 37): where a side is lost, the dose-response by
  ride and ramp, and what the options recover; only `--single-pass` kept
  both sides, at 3.3 times the wall time at NAS scale.
- 0178: `--overlap-by-chains` kept every side but added a wrong
  version's segments on the first shared panel. Not adopted.
- 0179 (session 38, criteria committed first): on new seeds every lost
  side kept (257 of 257, 100% of 17,487 s); no wrong confident
  detection added and no right one lost on the sweep at both scales, the
  11 development reports at NAS scale, the generated mixes and the
  first shared panel; calibrate's false candidates unchanged (18 at
  262 tracks, 95 at NAS scale); user CPU +1.3% and +0.8%.
- 0180: in 30 s excerpts a ride in the middle costs recall through the
  3-window rule; the overlap rules win back 1-3 excerpts.
- 0209-0211 (session 45, criteria committed first, data the threshold
  had not seen): the second shared panel, double drops (the CLI's 64
  reports and printed tracklists byte-identical at 262 tracks and NAS
  scale) and beatmatched long mixes: nothing confident changes.
- 0232 (session 53): the default's change in the standard evaluation,
  the development scans and the printed tracklists.

## Consequences

- Both sides of a pitch ride of 1% or more are kept as segments of one
  play; its span covers the time it played.
- Weak and possible detections more than 0.6% from a stronger one of
  their file are kept where they were dropped (sessions 38 and 45:
  0.60-12.1%, at most 61 hits). A related version has weak second
  alignments just over 0.6% apart without any ride (0210). Should one
  reach 240 hits on its own it would be a new wrong confident segment;
  none has in any run. Against shared material the protection is the
  confidence rule's margin, not the speed condition. Worth counting
  beside the shared panels and drops when they run again: the confident
  segments a file gains against `--overlap-by-spans`.
- A detection the default kept can now be dropped: it overlaps, within
  0.6%, a side that the old rule dropped (0179 found one possible
  detection in a rides seed and a weak one in the Stakka mix).
- Calibration figures before this ADR describe the old rule; under the
  new one calibrate's false candidates were the same (0179).
