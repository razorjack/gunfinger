# Printing the time without a confident match: options (2026-10-21)

**Status: decided and implemented** (2026-10-27, "Decision" at the
end); part 1 (the gap lines, without an estimate) is implemented by ADR
0023 in session 55, with an unnumbered "ID - ID (<length> without a
confident match)" line; part 2 (the estimate) by ADR 0024 in session
56: the combined estimate of section 5 as a range, `; about 1-3
tracks`, none where it counts 0. Written as a proposal for our
decision (session 35); it is not an ADR: the choices below were ours to
make. Each lists the options and the
evidence measured so far, then a recommendation, marked as such. The
measures are the harness's (`gaps`, `self-match`): nothing here is in the
product yet. Definitions are those of
`docs/research/2026-10-21-gap-measures.md`: a gap is a stretch of the
mix that no segment of a confident play covers; a possible play does not
close it.

## What the evidence says, in short

- Gaps of 30 s or more reach every missed DJ-length track of the three
  development mixes at NAS scale (271 of 271) and every held-out play of
  the 80 long generated mixes (experiments 0159, 0161). At 30 s, 7.7% of
  the listed gaps in the leave-outs hold no missed track; each is time
  inside a found track that no segment covers (the end of Star Trails,
  59 s; 34 s inside Kinetic; 43 s after Blindside's tease). None lasts
  60 s.
- A gap's length over the mix's median interval (method 1) gives a range
  that holds the truth for 98% of development gaps but a count that is
  exact for only 100/53/37/42% at k = 1-4, when the interval is taken as
  if the gap's run were the only one missing (0160). It needs 3 clean
  intervals in the mix: with the registered interval, 81 of 115
  development gaps have no estimate.
- Matching the gap against itself (method 2) counts DJ-length records
  better: 95/80/69/54% exact on the long generated mixes (0162).
  Session 35 tested it where it should fail. Records that share a
  passage, played back to back at one tempo, usually make one stretch
  (68% of the panel's pairs), and its exact share at k = 2-4 falls from
  67% to 25% (0163). Limiting its count to method 1's range B gives the
  best count measured, 64% exact and 2.9% off by two on the test mixes,
  but not one to print as a count, and as a range of one either side it
  falls one gap short of the registered 90% at k = 4 (0165).
- A sustained pitch change of 1-3% within a play makes the search drop
  one side of it in about half of the ridden plays of generated mixes,
  which leaves gaps of up to 4:23 inside found tracks (0166). The three
  development mixes show no such gap of 60 s or more at NAS scale, so
  real rides seem rarer or briefer than the generator's; that is an
  inference, not a measurement.

## 1. By default, or on request

- **Default.** Every run shows where the tracklist accounts for nothing.
  A gap line identifies nothing, so it cannot be a wrong identification
  (ADR 0017's first concern); its risk is noise: a listed stretch with
  nothing missed in it (none at 60 s in the development mixes or in the
  generated mixes without pitch rides; 99 of 227 with them, 0166).
- **On request** (`--gaps`). The tracklist stays as it is; a user who
  does not know the option never sees the gaps. ADR 0017's user runs the
  tool once and reads the result: an option they never pass is the same
  as no feature.
- **Default in the table and JSON, on request in the tracklist.** The
  tracklist is what is pasted elsewhere; the table is read in the
  terminal.

*Recommendation:* by default in every format that lists entries in time
order (tracklist, table, JSON), not in the cue sheet (section 6). A gap
line claims only that no confident match covers a stretch, which the
report already implies by leaving it out.

## 2. The minimum length: 30 or 60 s

| At | Missed tracks reached (NAS leave-outs) | Listed gaps without a missed track (leave-outs; plain scans) | Long generated mixes |
|---|---|---|---|
| 30 s | 271 of 271 | 7.7%; 1 in the three together | every held-out play; none empty |
| 60 s | 271 of 271 | 0; 0 | 799 of 800; none empty |

Plays shorter than 40 s are often not reached even at 30 s (42 of 101 in
the short generated mixes are not, 0159), and a longer minimum cannot
reach more;
real DJ plays are mostly 2-6 minutes. In
the ridden generated mixes of 0166, 139 of 267 gaps at 30 s and 99 of 227
at 60 s lie inside found tracks: the minimum length does not remove
those, only the search can.

*Recommendation:* 60 s for what is printed. It reached the same missed
tracks in the development mixes, and listed nothing that was a found
track's own uncovered time. JSON lists the same gaps (one threshold, one
answer).

## 3. The mix's start and end

The time before the first confident segment and after the last is a gap
like any other (the registered definition). At 262 tracks my 2003 mix
starts with 2:25 of Konspiracy, not indexed there; the development mixes'
ends hold outros and the last record's tail.

*Recommendation:* list them, with the same minimum length. A mix that
starts with an unindexed record is the commonest case of a missing first
entry.

## 4. A gap with a possible line inside it

A possible play (ADR 0006) does not close a gap: it identifies nothing.
In the tracklist, a possible entry already prints with "(possible)", and
one inside a passage of a confident play of another record is left out
(ADR 0019).

- **Both, in time order:** the gap line at its start, the possible entry
  at its own time inside it.
- **The gap line names the possible entry** ("no confident match; a
  possible match at 10:47") and the entry keeps its line.
- **A possible entry splits the gap.** This would claim that the
  possible record accounts for its time, which ADR 0006 does not allow.

*Recommendation:* both, in time order, unchanged otherwise. A possible
entry left out by ADR 0019 stays out; its time is still part of the gap.

## 5. A count, or not; as a range; from which method

| Estimate | Exact, k = 1 / 2 / 3 / 4 | Within one | Off by two or more | Without an estimate |
|---|---|---|---|---|
| Method 1 (length; registered interval), test mixes of 0163 | 96 / 56 / 44 / 38% | 95% | 5.5% | a mix with fewer than 3 clean intervals (most NAS leave-outs; Nocny at 262 tracks) |
| Method 2 (self-match), same | 98 / 49 / 52 / 36% | 92% | 7.6% | never |
| Combined (method 2 within range B), same | 99 / 57 / 58 / 43% | 97% (89.6% at k = 4) | 2.9% | keeps method 2's count |
| Combined, records sharing material only | 98 / 44 / 48 / 31% | 96% (83% at k = 4) | 4.2% | |

The pre-registered bar for printing a count ("about n") is 90% exact at
each k; no estimate is near it beyond k = 1. The bar for a range of one
either side is 90% at each k and at most 5% off by two: method 1 met it
in session 34 only with an interval taken as if the gap were the mix's
only one, and the combined estimate misses it by one gap at k = 4. A
gap with no missed track cannot get 0 from either method; the
neighbour discount of 0164 gives 0 where the found track's record is
found in the gap, but failed its registered test. Method 2 also needs
the mix's audio decoded again for each gap, about 0.05 s of wall time
per gap at 16 jobs (21 s for 395 gaps), and its count drops with pitch
rides unless every rung to ±3.2% is matched (5.4 times the time).

- **No count.** The gap prints its length. The reader divides by a
  typical track length if they want to; nothing is claimed.
- **A range from the combined estimate** ("about 1-3 tracks", n-1 to
  n+1, at least 1): right for 97% of the test's gaps, but less often
  where records share material (83% at k = 4), which we expect to be
  common in mixes of one label or one break (not measured).
- **A range from method 1 alone:** needs no decoding, but has no
  estimate in mixes with many gaps, and is right for 95%.

*Recommendation:* no count for now; print the length. A count is the
part of a gap line that can be wrong, and none has met its registered
bar. If we want one anyway, the combined range is the best measured,
and a gap without a method 1 estimate keeps method 2's count.

## 6. JSON, CSV and cue sheet

- **JSON** (an added field; schema version 3 allows those): `gaps`, each with `start_seconds`,
  `end_seconds`, `possible_plays` (the positions, from 1, in `plays` of
  the possible plays that overlap it; neither array indices nor
  tracklist numbers, so worth settling in the ADR) and, if a count is printed, `tracks` as
  `{"low": n, "high": m}` or `null`, with `method`.
- **CSV:** one row per play today. Options: a second file, rows of a
  different kind (`confidence` = `none`, empty asset), or nothing. Rows
  of another kind break a consumer that reads every row as a play.
- **Cue sheet:** confident entries only today; a cue track is something
  a player can skip to. Options: an unnamed track per gap
  (`TITLE "No confident match"`), which renumbers the tracks after it,
  or nothing.

*Recommendation:* JSON yes; CSV and cue sheet unchanged for now. Both are
read by other programs, and neither has a way to say "no track" that a
consumer would not misread as a track.

## 7. Wording

The line must say that nothing confident was found there, not that the
rest of the mix is explained: a confident entry can still cover a
stretch where another record plays beside it, and gaps under the
minimum length are not listed. "Unidentified" is avoided: it suggests
that the other entries are a complete identification.

*Recommendation:* "no confident match for <length>", unnumbered, at the
gap's start, in the tracklist's time column. A report with no entry at
all keeps its line "no tracks found" without a gap line.

The prototype on the branch `s35-gaps-output` renders the tracklist and
JSON only; the table (section 1) is not done there.

## Examples from this session's reports

My 2003 mix at 262 tracks (`gunfinger identify`, 2026-10-21; 7 of its 12
tracks indexed), the recommended option at 60 s, as the prototype on the
branch `s35-gaps-output` prints it with `gunfinger show` (times are cut
to the second, as the tracklist's are):

```text
       0:00  no confident match for 2:25
 1.    2:25  Future Cut - The Specialist
       6:43  no confident match for 7:57
 2.   14:40  Tee Bee - Two Faced
      20:04  no confident match for 4:15
 3.   24:20  Carlito & Addiction - Into Music
      29:25  no confident match for 1:39
 4.   31:05  Total Science - Yvon Is On
 5.   34:54  Marcus Intalex & S.T.Files - Balderdash
 6.   39:14  Spirit - Out Of Control
 7.   42:15  DJ Hidden - Skektics
```

The four gaps hold 1, 2, 1 and 1 tracks not indexed (Konspiracy; Escape
Route and Phantom Force; Kinetic; Artificial Life). Method 1, method 2
and the combined estimate all give 1, 2, 1 and 1 here; with the combined
range the second line would read "no confident match for 7:57 (about 1-3
tracks)".

Nocny at 262 tracks has 9 entries and six gaps of 2:04 to 14:28; with
only one clean interval, method 1 gives no estimate for any of them.
Method 2 counts 1, 2, 3, 1, 2 and 2 records where the truth is 1, 1, 3,
1, 2 and 1. Stakka at 262 tracks has one gap of 0:59 (13:24-14:23), the
tail of Star Trails that no segment covers: listed at 30 s, not at 60 s.

The same gap in JSON (a field added to schema version 3, which allows
added fields):

```json
"gaps": [
  {
    "start_seconds": 403.24375390625,
    "end_seconds": 880.775357421875,
    "possible_plays": []
  }
]
```

## Decision (2026-10-27)

Ours, for the ADR that brings the change:

1. By default, in every format that lists entries in time order.
2. 45 s at first (not 60 s), to be revisited once real mixes show how
   the gap lines read.
3. The mix's start and end are listed, with the same minimum length.
4. A possible line inside a gap stays listed, in time order.
5. Print an estimate of how many tracks the gap holds; which estimate
   and its form (a range, "about n") are for the implementing session to
   propose from 0160-0165, the combined estimate being the best measured.
6. JSON gains the gaps; CSV and the cue sheet do not change.
7. A gap is named "ID - ID", the usual name in DJ tracklists for a track
   nobody has identified, instead of "no confident match".
