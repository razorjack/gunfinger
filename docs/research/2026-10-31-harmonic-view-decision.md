# The harmonic view: a decision note (2026-10-31)

**Status: measured, not proposed.** Session 66 tested proposal 5 of
[the proposals of 2026-10-29](2026-10-29-matching-and-archive-proposals.md)
as a candidate-only diagnostic (brief 66, experiments 0259-0261). The
index, the search and the product did not change.

## What was measured

For every candidate of 20 hits or more that the unchanged search found in
the shared-material scenarios (the two panels' development halves and
seven judged pairs, at NAS scale), the candidate's file was rendered along
the line through the detection's ends and both it and the query were
split into a harmonic and a percussive part by median filtering
(FitzGerald 2010, §2: 25 frames across time, 17 bins across frequency, a
soft mask). The score was the share of the reference's peaks found in the
query within a bin and a frame, minus the share at four chance shifts.
The comparison arm used the same denser picker (8 x 8) on the whole
spectrum. Positives were the played recording and the blend partner;
negatives, the related recording; true plays were compared at the
negatives' hit counts. Thresholds came from eight development families
and were applied once to seven frozen ones.

## What the view separates and what it does not

- **Above 240 hits it separates true plays from related versions**
  (AUC 0.913 development, 0.896 frozen), but no better than denser peaks
  (0.882, 0.903), the percussive part alone (0.864, 0.903) or the peak
  verifier on today's peaks (0.914, 0.954).
- **Below 240 hits it is weak or reads the wrong way** (AUC at 20-59
  and 60-239 hits: 0.257 and 0.403 in development, 0.602 and 0.184
  frozen; the denser arm 0.468 and 0.591, 0.673 and 0.414). A true play
  under another record (the blend partner, 347 of the 870 development
  positives at equal hits) keeps its drums but finds few of its harmonic
  peaks (median excess 0.113, against 0.292 in the denser arm): the
  louder record's sustained material covers them. That the related
  versions keep sustained material where they share more than drums is
  an inference from their scores, not checked by ear. Pooled at equal hits, the harmonic view's AUC is
  below the denser arm's in every development family and in all but one
  frozen family.
- **The bass kill** did not read as a contradiction: the harmonic view
  kept 32 of 46 and 43 of 49 killed true plays at the rate-0 threshold,
  the denser arm 26 and 37.
- **The Cool Rock pair** (calibrate's strongest false at NAS scale,
  confident up to 1,288 hits here): at the frozen threshold the harmonic
  view keeps 6 of its 105 negative cases (two detections), the denser
  arm 4, the percussive part 0, hits 68. That threshold also rejects
  about half of the frozen true plays (the harmonic view keeps 483 of
  944).
- **Identical audible passages** could not be marked: by the registered
  rule (the denser arm's share as close as another rip's), nearly every
  negative is as close as the weakest other rips, which are many at NAS
  scale. Telling such cases apart stays a question for listening.

By the pre-registered reading, neither rule held: on the development
part the view kept 3.6 points more true plays than denser peaks at the
rate-0 threshold (10 needed) with a lower AUC; on the frozen part its
wrong-answer rate (0.5%) was above the denser arm's (0.3%).

## The cost

About 3.2 s of CPU per candidate in the harness, of which decoding the
reference is 0.4 s; memory rises to 4.6 GiB at 262 tracks and 8.6-9.8
GiB per NAS process (0261). A product use would need the reference audio
at identification time, which `identify --store-only` or an unreachable
library cannot provide and which changes how identification reads files
(ADR 0003, ADR 0017), or a new stored field of harmonic peaks, about
2.2 times the stored peaks again. Each needs its own ADR.

## Risks of taking it further

- The view rejects true plays where a DJ blends, the case real mixes
  hold most often; the generated renders were three times denser in
  evidence than real plays before (0116, 0118).
- As with tolerant pairs and wider fan-outs before (0151, 0208), the
  separation measured here comes with hits: above 240 every view
  separates, below it none does reliably.
- The judged negatives rest on a few families; fresh seeds of one family
  are not independent.

## Recommendation

We do not propose the harmonic view as evidence, a rule or a stored
field. If shared material is taken up again, the measure to start from
is the percussive part or the existing verifier above 240 hits, where
all views separate, with real mixes' true plays as the binding control;
the harness option (`shared --harmonic`) stays for that.
