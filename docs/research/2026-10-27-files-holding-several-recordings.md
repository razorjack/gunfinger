# Pre-registration: library files that hold more than one recording

Session 58, 2026-10-27. Written and committed before any candidate is
listed. It fixes the signal, its thresholds, the files searched, the
check against the known cases and what counts as a candidate worth the
user's ear. The harness lists candidates; the user judges them and
decides what to ignore. Nothing is ignored or pruned by this session.

## What we look for

Three kinds of file were found by ear in the NAS collection and are now
ignored and pruned (session 52): the tracks of a scene release that fixes
tracks of a mixed CD (each blends into the next), an upload cut from a
mix (`Optical & Ryme Tyme - Space Cake [CUT].m4a`: Bacteria, then Space
Cake) and an upload whose first 56 s are another record (`Ryme Tyme -
T-Minus (Unreleased Vocal Mix).m4a`: Digital & Spirit - Crash). In each,
a stretch at the file's start or end lines up with another recording,
somewhere inside that recording, and the rest of the file is a different
recording.

## The signal

For a query file A of length L, from its stored peaks:

1. **Search.** A's peaks are searched against an index of every current
   NAS record, with the fullest posting lists set aside and on the
   turntable ladder, as `clusters` searches (`SKIPPED_SHARE`,
   `speed::ladder()`). By default only A's first and last 60 s are
   searched (A whole when L < 150 s); the `[CUT]` files are searched
   whole (below). Every detection of another file with at least 30 hits
   is a candidate part (`clusters`' `MIN_HITS_TO_MEASURE`).
2. **Measure again.** Each candidate is measured again over the whole of
   A against the found file B alone, at the detection's speed (as
   `clusters` measures a pair at its fitted speed), so that a part's span
   in A is not cut at the searched window. Of that search's alignments,
   the strongest that overlaps the window's detection in A is kept; if
   none does, the window's detection is.
3. **An edge part** is a part B with all of:
   - at A's edge: it starts within 15 s of A's start, or ends within
     15 s of A's end;
   - long and strong enough to be a play: at least 20 s in A and at
     least 60 hits (the least a possible play has,
     `FITTED_RULE.min_possible_hits`);
   - not a copy of A: it covers less than 80% of A (the clusters'
     `MIN_COVERAGE`) and leaves at least 30 s of A outside it;
   - inside B: for a part at A's start, at least 30 s of B lie before its
     span in B; for a part at A's end, at least 30 s of B lie after it. A
     transition plays the outgoing record past its start and the
     incoming one before its end; a remix that shares the original's
     intro or outro lines up start to start or end to end, and is left
     out by this rule.
4. **The whole file.** A file with an edge part is searched again whole
   (step 1 on all of A, then step 2), so that the report lists every part
   of it, not only those in the windows.
5. **A second part elsewhere**: a part C (from step 4) of at least 60
   hits and 20 s, of a file that is neither B nor in B's duplicate
   cluster, that reaches at least 20 s further into A than the edge part
   (past its end for a part at A's start, before its start for a part at
   A's end). The rest of A is then another recording found in the
   library; without one, the rest is unmatched (no other copy indexed)
   or A's own.

A **candidate** is a file with at least one edge part. Its **strength**
is the hits of its strongest edge part; the report lists candidates by
strength, strongest first, each with every part (A's times, the matched
file and its cluster, B's times, hits, speed) and which rule each part
meets.

A candidate is **worth the user's ear** when it has an edge part and a
second part elsewhere, or an edge part of at least 100 hits.

The thresholds are ours and fixed here; none comes from the run.

## Files searched, and what it costs

A whole-file search costs about 117 s of CPU per file (session 10's
NAS clusters run: 639 files, 75,059 s); all 40,678 records whole would
take about 41 hours on 32 threads, and their first and last 60 s about
13 hours at that rate. The search has become cheaper since (sessions
41, 43 and 46), by a factor we measure before the run on the first 64
files of the second tier (timing only: the output is not read before
the tiers are fixed). The tiers, in this order:

1. the 46 records of `[CUT]` files, searched whole;
2. the first and last 60 s of the other 6,293 records of the
   `i-witness-dnb` channel (the channel of the `[CUT]` uploads and of the
   Space Cake file);
3. the first and last 60 s of the 6,103 records outside
   `__youtube_archivists/` (release folders: where mixed CDs are), 6,088
   after the leave-outs below; among them the controls' 11 and 75
   records;
4. the first and last 60 s of the other 28,236 uploads, in path order.

Tiers run in order until the run's budget, about 90 minutes of wall
time, is spent; a tier that does not fit whole is cut at a path-order
prefix, recorded in brief 58. The tier lists are written by
`work/s58/tiers.py` from the store's record headers.

Left out of the queries: every record the NAS `.gunfingerignore`
matches (none since session 52's prune; the harness checks it), and the
15 records of experiment 0065's candidate folders that the ignore file
does not leave out (`shy_fx-just_an_example-1995-sour`,
`mampi_swift-blade_runners_ep-bld003-2002-sour`). Every record stays in
the index, as in every NAS run. A candidate with a part in a 0065 folder
is listed but kept out of the listening pack.

## The check against the known cases

The five pruned files have no record, so they cannot be searched. Their
saved alignments are in the clusters file before session 51's ignore run
(`work/listening-2026-10-25/duplicate-clusters-before.json`), whose
queries were the corpus recordings' rips; steps 3 and 5 are applied to
those pairs by hand in the experiment:

- the track-fix Drumtrip: Propaganda at its first 40 s (the 3zb upload,
  207 hits; the track-fix Propaganda, 56), Known Unknown (Chaos Edit,
  Rollers Edit) over 10-175 s. Expected: a candidate worth the ear, with
  a second part elsewhere.
- the Space Cake `[CUT]` file: Bacteria over its first 65 s (three
  files, 118-149 hits). Expected: a candidate worth the ear (an edge part
  of 100 hits or more), the rest unmatched in those pairs.
- the T-Minus vocal mix upload: only the corpus T-Minus over 68-234 s is
  saved (Crash was found by session 39's drops mix, not in a saved
  pair). Expected: not a candidate from the saved pairs alone, which
  shows what the saved pairs miss.
- the track-fix Kraken and Propaganda: no saved pair of the Kraken;
  the Propaganda's end (186-226 s) at the Drumtrip's start, from the
  Drumtrip's side only.

Two live controls in today's store: the tracks of Hard Leaders - Listen
Up (`va-listen_up-hlcd9-2000-sour`, 11 records), which the user judged to stay (full
tracks with short crossfades), and the unmixed discs of the releases
whose mixed disc is ignored by a `2*` or `1*` pattern (75 records in
seven folders). Each that comes
out a candidate is counted in the experiment as a candidate the user has
already judged a keep.

## Kinds, for the experiment's tally

A guess from each candidate's parts and its folder, for counting only;
the listener decides what a file is.

- **A mixed CD's track**: a file outside `__youtube_archivists/` with an
  edge part.
- **Cut from a mix**: an upload named `[CUT]`, or an upload with edge
  parts at both ends.
- **An upload opening with another record**: any other upload with an
  edge part at its start only.
- **Other**: an upload with an edge part at its end only.

## The listening pack

At most 20 questions from the candidates worth the ear (none with a part
in a 0065 folder): strongest first, taking each kind in turn so that the
pack is varied, `[CUT]` uploads first among cuts. Each question has the
file as its reference lane and the strongest file of each part's cluster
as a lane placed by that part's alignment, and asks whether the file is
one recording throughout or changes record. Agents answer nothing.
