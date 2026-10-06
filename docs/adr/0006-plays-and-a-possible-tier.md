# 0006: Plays and a possible tier

## Status

Accepted (2026-10-06). Supplements ADR 0001; the confident rule is
unchanged. The possible threshold was calibrated at 262 tracks and must be
measured again when the library grows (see "When the library grows").

## Context

The owner reviewed the first test-set evaluation (ledger):

- The Pulse was found as two confident detections because of a needle skip
  at 36:17. A skip moves the position in the track, so the hits move to a
  new line and a new chain starts. `identify` listed the two as unrelated
  rows.
- Fibre Optix "Sin" was faded out for a station insert and brought back.
  Both pieces were found at the same speed with 84 hits each, below the
  rule, and `identify` showed them only in a flat list of the ten strongest
  sub-threshold candidates, apart from the timeline.

The frozen rule answers "is this track playing?" and has produced no wrong
answer. The owner wants the tool to help where the evidence is real but
short, without weakening that answer.

## Decision

1. **Plays** (`gunfinger-core/src/plays.rs`). Detections of one asset with
   gaps of at most 90 s between them are one play, listed with its segments.
   Only detections that are at least possible take part, so chance-level
   alignments never stretch a play. A play's confidence and speed are those
   of its strongest segment. Evidence is not summed into a stronger claim:
   summed evidence across gaps has no measured null.
2. **A possible tier** (`MIN_POSSIBLE_HITS = 60` in
   `gunfinger-core/src/confidence.rs`). A detection below the rule with at
   least 60 hits is `possible`: this recording, or one sharing material with
   it (a remix, a VIP), probably plays here. The calibration rule: the
   threshold is at least twice every false candidate that is not a remix or
   version of the played recording, over the sweep and the development set
   with its leave-outs. Measured (experiment 0006): unrelated recordings
   reach at most 28 hits and audio not in the index at most 19; all 18 false
   candidates at 30 hits or more are the Stakka remix of Clockwork against
   the original.
3. **Output.** `identify` prints one table of plays in time order, labelled
   `confident` or `possible`, with the segments of a multi-segment play
   underneath. JSON is schema version 2: `plays`, each with `segments`. The
   separate list of sub-threshold candidates is gone; weak detections are
   not shown.
4. **Scoring is unchanged.** Only confident detections identify a track or
   count as wrong. The harness reports the possible tier apart: referenced
   tracks found only as possible, and possible plays that match no track.

## Consequences

- A remix of the playing record can appear as `possible` (the Stakka remix
  of Clockwork, 91 hits, while the original plays in the development mix).
  That follows from the claim the tier makes.
- Duplicate rips of one recording still get one row each; collapsing them
  needs the Track/AudioAsset model (roadmap).
- The development set has no brief plays, so what the tier adds can only be
  seen on the test set (evaluation 2 in the ledger). That evaluation is not
  blind: the evaluation-1 report, with Sin's 84-hit chains, was inspected
  before this decision. The threshold follows the rule above, not those
  numbers.

## When the library grows

Chance alignments get stronger with more postings. After indexing more
tracks, follow `docs/calibration.md`:

1. Rerun the sweep, the development scan and its leave-outs, and
   `gunfinger-eval calibrate`.
2. In calibrate's possible-tier block, every false candidate at half the
   threshold or more must be a remix or version of the played recording. If
   an unrelated recording appears there, raise `MIN_POSSIBLE_HITS` to at
   least twice its hits.
3. If the threshold would reach the confident rule (200 hits), the tier no
   longer adds anything: delete `MIN_POSSIBLE_HITS` and
   `Confidence::Possible`. Keep plays; grouping the segments of confident
   detections does not depend on library size.
