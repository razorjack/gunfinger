# Pitch rides and the second overlap rule: what an ADR would decide (2026-10-22)

**Status: the evidence and a recommendation** (session 45 added its
confirmation at the end). Nothing here changes the product. Whether to
change how the search judges overlap after the second pass is our
decision and needs an ADR that supersedes ADR 0008's consequence for
overlapping parts. Experiments 0166 and 0175-0178 (sessions 35 and 37),
0179-0180 (session 38) and 0209-0210 (session 45); the roadmap's "Speed
that wanders within a play".

## The rule today

The default matcher (ADR 0008) finds lines per 10 s window on the speed
ladder (0.4% steps), joins lines of successive windows into chains only
from rungs at most 1.5 steps (0.6%) apart, and drops every chain that
overlaps a stronger chain of the same asset (`strongest_per_moment`, the
first suppression). The second pass then measures each chain again at
one fitted speed over the chain and 10 s either side (`MARGIN_SECONDS`),
and `strongest_per_moment` runs again on the measured spans (the second
suppression). ADR 0008 already names both consequences that matter
here: a speed change of more than 0.6% between windows splits a play
into segments, and after a needle skip the weaker part, which the margin
makes overlap the stronger, is dropped.

## The failure and its size

When a DJ rides the pitch by about 1% or more, the play splits into two
chains at the ride, one per speed. Each side is confident after the
second pass, but the margin moves both facing ends a few seconds into
the ramp, the measured spans overlap (median 4.7-5.2 s, 0175), and the
second suppression drops the weaker side whole. The play is still
found; it loses time. Session 35's rides mixes (0166) showed the effect
on gaps: 139 of 267 gaps of 30 s or more inside the mixes hold no missed
track.

0176's dose-response (seed 3701, the same 194 ridden plays under every
condition; sides of 30 s or more with 30 s or more uncovered, of
248 for the drift to 347):

| Ride | 2-5 s ramp | 10-20 s ramp |
|---|--:|--:|
| 0% (control), 0.25% | 0 | 0 |
| 0.5% | 2 | 0 (0.5% over 60 s: 0) |
| 1% | 133 | 70 |
| 2% | 150 | 102 |
| 3% | 101 | 14 |

No play lost every confident detection and none was wrong. Within-play
wander measured in the development mix (0022, ±0.2%) is below the
smallest ride that costs anything. The development mixes hold two
candidates, which nobody has listened to: Star Trails in the Stakka mix
(a side at 13:13-14:37, 1.3-1.5% slower than the kept 9:45-13:24, 306-315
hits, dropped at 262 tracks and at NAS scale; the manifest has Star
Trails until 15:31) and Into Music in the 2003 mix (a middle segment at
1.064 between 1.040 and 1.053, 1,392 hits, dropped, leaving a 15-41 s
hole). Nocny shows none.

What it costs the user: the tracklist still lists the track, but the
play's span is shorter than the time it played, and a gap marker
(`docs/research/2026-10-21-printing-gaps.md`) would mark a gap inside a
found track. It never costs a confident identification in the long
mixes measured. In 30 s excerpts (0180) a ride in the middle costs
recall from 2% (43-49 of 80 confident at 2%, 3-8 at 3%; 1% in the middle
and 2% at 24 s cost nothing), but there the 3-window rule decides: each
side spans two windows. Neither overlap rule changes that (1-3 excerpts
won back); it belongs to the window condition (the roadmap's "A minimum
span instead of 3 windows").

## The options

Each is reachable from the harness today and changes nothing by default.

1. **Keep today's rule.** Rides of 1% or more keep costing time (above);
   nothing else changes.
2. **`--single-pass`** keeps both sides (0177: 0 and 8 sides left) because
   it has no second pass, but it is the matcher ADR 0008 replaced: 3.3
   times the wall time at NAS scale and many more false candidates. Not a
   candidate.
3. **`--overlap-by-chains`** (0178): the second suppression is skipped,
   since the first already left no two chains of one asset overlapping.
   It keeps every lost side (183 of 183 on seeds 3741-3742), never
   removes a detection, and names no new wrong file anywhere it ran. On
   the first shared panel it keeps 56 more confident segments: 52 of the
   played recording and 4 of the Rogue Unit remix of Fire, a wrong
   version the default already names confidently in fire-6's two
   "beyond" queries (indexed and played left out). A real default run
   (session 38) gives the same comparison as session 37's trace. Its
   registered V2 failed by its letter.
4. **`--apart-in-speed 0.006`** (0179): the second suppression drops a
   detection only when it overlaps a stronger one of the same asset and
   their fitted speeds differ by at most 0.6%. A ride's sides differ by
   the ride; repeats and versions within one play's alignment lie on the
   same rung or the next (0-0.40% in every development trace). Under
   criteria committed before it ran (brief 38, 08:26) it keeps all 257
   sides the default loses on new seeds (3% and 1-2%, short and long
   ramps; 100% of their seconds), adds no wrong confident detection and
   loses no right one on the sweep at both scales, the 11 development
   reports at NAS scale, the generated mixes and the new seeds, and on
   the first shared panel changes no confident detection in any of its
   622 queries. It keeps Star Trails' and Into Music's dropped segments.
   Calibrate's false candidates are unchanged at both scales (0178: 18
   to 19 at 262 tracks). User CPU +1.3% at 262 tracks, within the
   default's own runs, and +0.8% at NAS scale, 0.6% above the highest of
   four default runs.

Not measured: clipping the weaker side's span at the stronger one's
boundary instead of dropping it, and a second pass without its margin
where a chain ends at a change of speed. Both change more code than a
predicate in `strongest_per_moment`.

## The shared-material risk

Precision comes first (ADR 0017), and shared material is where a
change to overlap could add wrong answers: a remix, VIP or bootleg
shares passages with the played record, and the default may already name
it confidently (0178's Rogue Unit remix of Fire). `--overlap-by-chains`
keeps every second alignment of such a file; on the shared panel the
default dropped them because their spans overlapped the stronger one at
the same speed or one rung away, and the variant kept them as extra
segments. `--apart-in-speed 0.006` keeps a second alignment only at a
speed more than 0.6% away from every stronger overlapping one. By
argument, a file playing at one speed does not give that (every
development trace has such second alignments at 0-0.40%); a wrong file can only gain a
segment where it is itself found on both sides of a ride. That needs a
version sharing the ridden passage and confident on both sides, which no
run has shown, and the default would already name that file wrongly on
the stronger side. The shared panel was also the threshold's development
data, so its clean result is consistent with the rule on data it was chosen
from, not a confirmation;
the second shared panel (`shared-material-2-seed-2026-development.json`)
has not been run with either variant.

## What an ADR would decide

1. The second suppression's rule: today's spans, `Chains`, or
   `SpansAtSpeed(0.006)`. The first suppression and the confidence rule
   stay as they are; no threshold of `docs/calibration.md` moves, since
   calibrate's false candidates do not change under `SpansAtSpeed`.
2. The share, and what it rests on: 1.5 rung steps, the chain link
   rule's limit (ADR 0008), between the largest difference measured
   outside a ride (0.40%) and the smallest of a 1% ride (0.90%). A ladder
   with another step would move it with the step.
3. How a play shows two sides that overlap in reported time (median 5 s,
   up to about 20 s): as measured, or clipped at the stronger one's
   boundary for display. Plays already group them (ADR 0006); scoring
   counts either.
4. Whether a wrong file's added segment, where the default already names
   that file there, is a new wrong answer (0178's V2 said yes;
   `SpansAtSpeed` made the question moot on the measured data).
5. ADR 0008's consequence "a speed change of more than 0.6% between
   windows splits a play into segments" stays true; the change is that
   both segments are kept. The needle-skip consequence (a weaker part at
   the same speed dropped) is unchanged.

## Recommendation (ours to decide)

Adopt `SecondOverlap::SpansAtSpeed(0.006)` as `Matcher::Fitted`'s second
suppression, through an ADR that supersedes ADR 0008's consequence for
overlapping parts, and show a play's overlapping segments as measured.
It is the only option measured that keeps the sides of 1-3% rides (all
257 that today's rule drops on seed 3801's four conditions) while
leaving every confident detection of the first shared panel, the sweep
and the development scans as it was, apart from the two candidates; it is one
predicate in `strongest_per_moment`, and its CPU cost is under 1%. Do not
adopt `--overlap-by-chains`: it keeps the same sides but also a wrong
version's extra segments.

Before the ADR, one confirmation on data the threshold has not seen: the
second shared panel's development half (`shared-material-2-seed-2026-
development.json`) at NAS scale, default and variant, with S2 and S3 as
in brief 38. Listening to Star Trails at 13:13-14:37 in the Stakka mix
and Into Music at 25:57-26:56 in the 2003 mix would show whether the
kept segments are real rides; adoption does not depend on it, since
the generated rides measure the mechanism and the scans score neither
segment as wrong (the manifest has Star Trails until 15:31). Rides inside short plays are a separate
question for the window condition.

## Session 45: the confirmation on data the threshold has not seen

Criteria S2 and S3 of brief 38, word for word, with a tracklist criterion
T, were committed before any run (brief 45, 21:20, `3bb80ef`); the
variant and its 0.6% are session 38's, unchanged.

- **The second shared panel** (0209): its development half at NAS
  scale, a default run and a variant run, 440 query and index pairs over
  10 scenarios. No confident detection changes in any pair (played 545,
  related 12, other 3, partner 0, in both runs). The variant adds 83
  weak or possible detections (at most 61 hits) whose speeds differ from
  the stronger overlapping detection of their file by 0.60-12.1%, 41 of
  them under 0.9%. User CPU 5,140 s and 5,145 s. The validation half was
  not opened.
- **Double drops** (0210): session 39's mixes, seeds 2026 and 2028. In
  the harness at 262 tracks, wrong confident detections 0 -> 0 and right
  plays 221 -> 221 and 220 -> 220. The CLI's reports and printed
  tracklists, from a diagnostic build whose `Matcher::Fitted` uses the
  variant (never committed; checked against the harness and on the
  Stakka mix's Star Trails side), are byte-identical to the default's in
  all 64 reports at 262 tracks and at NAS scale: no tracklist line, level
  or wrong play changes.
- So S2, S3 and T hold on every registered run: the variant is
  confirmed on data its threshold has not seen, by the registered
  reading.

Two findings add to the argument above; neither changes the result:

1. **The 0.6% does not separate weak alignments by speed.** Session 38
   measured the speed differences only for detections confident after
   the second pass (0-0.40% outside rides, 0.90% and up for 1% rides).
   On session 45's data no such detection is dropped at more than 0.6%,
   since the variant adds no confident one. The weak and possible
   detections it keeps lie at every difference from 0.60% up, the
   smallest it can keep (41 under 0.9% on the panel, 14 of the 20 on the
   drops); at most 61 hits, they stay far below the 240-hit rule.

2. **A related version gains segments without a ride.** On the drops, 19
   of the 20 detections the variant adds belong to the Clockwork pair's
   file that is not playing while its related version plays, at
   0.61-1.51% from a stronger alignment of the same file (7-44 hits).
   The section "The shared-material risk" argues that a wrong file gains
   a segment only where it is found on both sides of a ride; on these
   data that holds only at the confident level. A related version whose
   second alignment reaches 240 hits at another speed would (by
   inference) become a new wrong confident segment under the variant
   where the default drops it. None did in any run (sessions 38 and 45),
   but the protection is the confidence rule's margin, not the rule's
   speed condition.

For an ADR, these add:

- The evidence base: S1-S3 on seeds 3801 (rides), the sweep at both
  scales, the 11 development reports at NAS scale, the generated mixes,
  both shared panels' development halves (the second unseen by the
  threshold) and the double drops at both scales, with their tracklists.
- The residual risk, stated as above: a related version's second
  alignment at more than 0.6% that is itself confident. A check worth
  keeping beside any adoption: the count of confident segments a file
  gains under the variant on the shared panels and drops (0 so far).
- The recommendation above is unchanged (ours to decide): adopt
  `SpansAtSpeed(0.006)` through an ADR. The second panel's validation
  half stays unseen for a later test, should a rule change need one.
