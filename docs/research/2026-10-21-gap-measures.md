# Gaps and their track counts: definitions, measures and criteria (2026-10-21)

**Status: pre-registration**, committed in session 34 before any gap was
counted on a search. It fixes how steps 1 and 2 (method 1) of
`docs/research/2026-10-20-unidentified-gaps.md` are measured and what
counts as good enough. It is not edited after its commit; a deviation is
written in `docs/brief-34.md` with its reason, and the experiments say
where they depart from it.

## Definitions

- **Covered time.** A report's detections (two windows or more, as
  reports keep them) are grouped into plays by `plays::group`, as
  `identify` and `seconds` group them. A play is confident when one of its
  segments is (ADR 0013). The covered time is the union of the segments of
  confident plays, each from its first to its last aligned hit. A gap
  between two segments of one play is not covered, so a gap inside a
  returning track counts. Possible plays cover nothing: a possible play
  inside a gap does not close it (ADR 0006); the listing counts the
  possible plays that overlap each gap.
- **A gap** is a stretch of the mix, from 0 to its end, that no covered
  time touches. The mix's start and end are edges like a segment's.
  At minimum length L, the listed gaps are the gaps of at least L
  seconds. L is compared at 10, 20, 30, 45, 60 and 90 s.
- **Truth of a scan** (a development mix): each manifest track from its
  start to the next track's, the last to the end of the mix, as
  `seconds` reads it. A track is indexed when it has a reference and a
  file of its cluster is in the index; otherwise it is **missed**
  (left out by the run, absent from the library, or not in it at that
  scale). A second within 30 s of a border between two tracks (the
  margin `seconds` uses, experiment 0139) is a transition second; any
  other second belongs to the one track playing then, missed or
  indexed.
- **Truth of a generated mix** (`mixes`, `shared-mixes`): each play from
  its first to its last audible sample. A held-out play, or a play whose
  files are all left out, is missed. A second where two plays are
  audible is a transition second; where one plays, it is that play's.
- **A gap holds a track** when the gap covers at least 30 s of the
  track's span or half of it, whichever is less, or when at least half
  of the gap lies in the track's span. A gap's **truth** is the number of
  missed tracks it holds. A missed track is **reached** at L when a
  listed gap holds it.

## Method 1

- **Entries.** The confident plays in order of start, two of them taken
  as one entry when they overlap for at least half of the shorter (the
  CLI's `SAME_PASSAGE_OVERLAP`; without the alignment test, which scan
  reports cannot make). An entry starts where its earliest play starts.
- **The interval.** The median of the start-to-start intervals between
  consecutive entries, leaving out each interval that holds a listed gap
  at the same L (those hold time not accounted for). With fewer than 3
  intervals left, the gap gets no estimate.
- **The estimate** n is the gap's length over the interval, rounded, at
  least 1. **The range** is n-1 to n+1, at least 1 (range A, the plan's
  illustration "about 2 tracks, 1-3"). Beside it, range B: the gap over
  the upper and the lower quartile of the same intervals, each rounded
  and at least 1 (Python's `statistics.quantiles`, n = 4, as
  `gap_count.py`). A secondary interval leaves out only the interval
  holding the gap itself, as experiment 0154 did.

## Measures

Step 1, by minimum length, for each kind of report (NAS-scale slot
leave-outs, NAS-scale scans with nothing left out, the same two at 262
tracks, generated mixes):

- seconds of missed tracks inside listed gaps, over all seconds of
  missed tracks (transition seconds left out of both);
- listed gaps, and listed gaps holding no missed track, with what they
  hold instead (indexed time, transition seconds) and the possible plays
  inside them;
- missed tracks no listed gap reaches, with their lengths.

Step 2, at each L, over listed gaps:

- estimate minus truth by truth (0, 1, 2, 3, 4, 5 or more missed
  tracks): exact, off by one, off by two or more;
- how often range A and range B miss the truth;
- gaps holding no missed track (method 1 cannot say 0, so each is a
  miss of both ranges);
- the found entries' median interval beside the manifest's median
  (all tracks) for each scan, and, in the generated mixes, how often
  missed plays are reached by their length.

**Counting.** In the tables by truth, a gap is counted once per set and
set of missed tracks it holds: the first in the order plain scan, then
the design's scans in order (Nocny's slot 17, missing in every scan,
counts once; at 262 tracks the slots not indexed there count once). Each
designed run is left out in exactly one scan, so its gap counts once
anyway. Gaps holding no missed track are counted in every scan in which
they are listed (each scan is a run a user could make), and once per set
beside it.

## Criteria (written before any count)

**The minimum length.** At L:

- C1, reach: in the NAS-scale slot leave-outs, a listed gap holds at
  least 95% of the missed tracks (all runs, Nocny's slot 17 once).
- C2, gaps without a missed track at NAS scale: at most 3 in the three
  scans with nothing left out together (one per mix), and at most 10%
  of the listed gaps of the slot leave-outs.
- C3, generated mixes: at most 10% of listed gaps hold no missed
  track.

The minimum length chosen is the smallest L that meets C1, C2 and C3.
If none does, the smallest L that meets C2 and C3 is reported with its
reach; if C2 or C3 fails at every L, none is chosen, and the trade-off
goes to the ADR. The 262-track scans are reported beside it, not used
to choose.

**Method 1 as a printed range.** At the chosen L, over the NAS-scale
slot leave-outs and the scans with nothing left out (counted as above):

- M1: range A holds the truth for at least 90% of listed gaps holding
  1-4 missed tracks, pooled, and for at least 80% at each of k = 1, 2, 3
  and 4.
- M2: gaps holding no missed track are at most 10% of listed gaps
  (C2's second part, and each is a miss of every range).
- M3: the estimate is off by two or more for at most 5% of listed gaps
  holding 1-4 missed tracks.

Method 1 is good enough to print as a range when M1, M2 and M3 hold.
Range B is reported beside A and judged by M1 and M3 alike; if both
pass, the narrower one on average is preferred. Method 1 is good enough
to print as a count ("about n") only if it is exact for at least 90% of
gaps at each k; experiment 0154's start times (75% at k = 2) already
suggest it will not be. The generated mixes of 20-60 s plays are
reported for step 2 but do not judge it: their plays are far shorter
than a DJ's.
