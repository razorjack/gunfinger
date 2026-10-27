# 0023: The tracklist marks time without a confident match

## Status

Accepted (2026-10-27), our decision. Amends ADR 0017's default output:
the tracklist, the table and the JSON report list each stretch of 45 s
or more that no confident play covers. Part 1 of the decision; part 2
(an estimate of how many tracks a gap holds) follows in a later session
and amends this record. The search, the detections, the confidence rule
(ADR 0013), the possible tier (ADR 0006), the entry rule (ADR 0017),
ADR 0019's marked plays, CSV and the cue sheet do not change.

## Context

A tracklist does not say how much of the mix it accounts for: eleven
lines over 56 minutes read the same whether the rest is blends or a
quarter of an hour of tracks not in the library. Sessions 34 and 35
measured gaps in the harness (experiments 0159-0166;
`docs/research/2026-10-21-gap-measures.md` defines them):

- A gap is a stretch of the mix that no segment of a confident play
  covers. A play is as confident as its strongest segment, so its
  possible segments cover their time; a gap between two segments of one
  play counts. Possible plays cover nothing (ADR 0006).
- At 30 s and at 60 s every missed DJ-length track of the three
  development mixes at NAS scale is reached (271 of 271). At 30 s, 7.7%
  of the listed gaps in the leave-out scans hold no missed track, all of
  them time inside a found track (the end of Star Trails, 59 s; 34 s
  inside Kinetic; 43 s after Blindside's tease); at 60 s none does
  (0159).
- No estimate of the tracks a gap holds met its registered bar for
  printing (0160-0165).

`docs/research/2026-10-21-printing-gaps.md` set out the choices; our
decision of 2026-10-27 is its section "Decision". This record answers
its sections 1-7 for part 1: section 1 in item 1 below, 2 in item 2, 3
in 3, 4 in 4, 5 (a count) in item 11, 6 in items 7 and 8, and 7 (the
wording) in items 5, 6 and 9.

## Decision

1. **Where.** By default, in every format that lists entries in time
   order: the tracklist (the default format), the table (`--format
   human`) and JSON, from `identify` and `show`. Gaps are derived from
   the plays whenever a report is rendered, so `show` lists them for
   reports saved before this change, and a report read back never
   supplies its own. The timeline does not change: it draws each play
   as a bar, and time without a bar is already visible there.
2. **Minimum length.** 45 s, at first; we revisit it once real mixes
   show how the lines read. One threshold for every format, the named
   constant `MIN_GAP_SECONDS` in the CLI. A stretch is listed when its
   end minus its start is at least 45 s, the inequality of the
   harness's listings. No command-line option changes the length or
   turns gaps off: the decision is one threshold, ours to revisit, and
   an option nobody passes changes nothing for the user of ADR 0017,
   who runs the tool and reads the result. A program that wants another
   length has every play's segments in JSON.
3. **Start and end.** The time before the first confident segment and
   after the last is a gap like any other, with the same minimum. The
   edges are those of the part searched: `--start` and `--duration` move
   them.
4. **Possible lines.** A possible entry inside a gap keeps its line at
   its own time, after the gap's line; it neither closes nor splits the
   gap, since it identifies nothing. A possible play that ADR 0019
   leaves out of the tracklist stays out, and its time is part of any
   gap it meets. (Such a play lies inside a passage of a confident play,
   whose holes last at most 10 s, so it cannot meet a listed gap in
   practice; the rule holds either way.)
5. **Wording and place.** A gap is named "ID - ID", the usual name in
   DJ tracklists for a track nobody has identified, followed by its
   length, on an unnumbered line at the gap's start in the time column:

   ```text
          0:00  ID - ID (2:25 without a confident match)
    1.    2:25  Future Cut - The Specialist
          6:43  ID - ID (7:57 without a confident match)
    2.   14:40  Tee Bee - Two Faced
   ```

   DJ tracklists usually number ID lines, and the tracklist is pasted
   elsewhere. We leave the line unnumbered all the same: a number would
   count the gap as one track, but a gap can hold several missed tracks
   or none (a found track's own uncovered time), and how many is part
   2's estimate. The entries keep the numbers of the tracks found, so
   they stay put when a gap appears or the minimum length changes.
   "ID - ID" still reads as an unidentified track where the list is
   pasted. The length is cut to the second, as the tracklist's times
   are; "without a confident match" says only that nothing confident
   was found there, not that the other lines explain the rest of the
   mix (gaps under 45 s are not listed, and a confident entry can cover
   time where another record plays beside it). A gap line comes before
   an entry that starts at the same moment.
6. **The table** lists a gap as a row in time order among the plays,
   with its span in the time column and the same words in the asset
   column, `ID - ID (2:25 without a confident match)`; the other
   columns stay empty.
7. **JSON.** The report gains `gaps`, always written (empty when there
   is none): each gap has `start_seconds`, `end_seconds` and
   `possible_plays`, the positions (from 1) in `plays` of the possible
   plays with a segment overlapping it, as `shares_material_with.play`
   and `other_titles` number plays. An added field, which schema version
   3 allows, so the version stays 3. Part 2's estimate will be a field
   `tracks` on each gap; part 1 does not write it, so no reader mistakes
   a placeholder for a missing estimate.
8. **CSV and the cue sheet do not change.** Both are read by other
   programs, and neither can say "no track" in a way a consumer would
   not read as a track.
9. **Nothing found.** A tracklist without any entry still prints `no
   tracks found` and no gap line. The table and JSON list the gaps as
   they are: one over the whole part searched, when it lasts 45 s or
   more.
10. **One definition.** The covered time and the stretches it leaves
    are computed once, in the core (`gunfinger_core::gaps`), for the
    CLI and for the harness's `gaps` listings.
11. **No estimate yet.** Part 1 prints no count of the tracks a gap
    holds; our decision asks for one, and part 2 proposes which estimate
    and in what form (experiments 0160-0165) and adds it to the line and
    to JSON as `tracks`.

## Consequences

- Every default tracklist with a confident entry and a stretch of 45 s
  or more without one gains lines. The search, the detections and the
  entries are unchanged, and `regress` stays identical.
- A gap line can be noise: time inside a found track that no segment
  covers. At 45 s experiment 0159 listed one such gap in the three
  development mixes' plain scans at NAS scale (59 s at the end of Star
  Trails, measured before ADR 0021 kept that side) and 3 of 131 listed
  gaps in the leave-outs; a sustained pitch ride can leave more inside a
  found track (0166). Experiment 0234 lists every gap line the
  development mixes print now.
- The minimum length and the wording are ours to change after real
  mixes; part 2 adds the estimate to the line and `tracks` to the JSON.
- `scripts/analysis/tracklist_score.py`, which checks every printed
  line against the entries rebuilt from JSON, passes over gap lines.
