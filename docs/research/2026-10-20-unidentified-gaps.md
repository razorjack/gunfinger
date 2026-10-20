# Marking unidentified time in a tracklist (2026-10-20)

**Status: a plan.** By our decision (2026-10-20), marking the time a
tracklist does not account for is worth doing. Whether it prints by
default or on request is still ours to decide, and either changes the
default output ADR 0017 describes, so it needs an ADR. Nothing here
changes the product yet. The roadmap's entry is "Time without a
confident match" (Reports and command line); experiment 0154 is the
first measurement.

## The problem

A tracklist does not say how much of the mix it accounts for. When
Gunfinger finds A, B, C, F, G and H but not D and E, C is followed by F
and nothing marks the minutes between them: the output reads the same as
a long play of C followed by a blend. Eleven lines over 56 minutes look
the same whether the rest is transitions or a quarter of an hour of
tracks not in the library.

## Step 1: the gaps

- A gap is a stretch with no confident segment, computed from segments
  rather than whole play spans, so a gap inside a returning track counts.
- A minimum length leaves ordinary transitions out. It is chosen against
  the generated mixes, whose transitions have exact lengths, not fixed
  here.
- A gap does not claim that the matched time around it is fully
  explained, and a possible play inside it does not close it: a possible
  play never identifies a track (ADR 0006). How a gap and a possible
  line inside it print together is part of the ADR.
- An illustration of the tracklist, not a decided format:

  ```console
   3.    9:45  Artist C - Title C
        14:10  6:30 without a confident match (about 2 tracks, 1-3)
   4.   20:40  Artist F - Title F
  ```

## Step 2: how many tracks a gap holds

Three methods, cheapest first.

1. **The gap's length.** The gap over the median start-to-start interval
   of the mix's found tracks, rounded, given as a range. On the
   development manifests' start times it gives the count of k missed
   tracks within one in every case, exactly for 75% of gaps at k = 2 and
   42-48% at k = 3-4 (experiment 0154). It cannot say 0, and it
   undercounts if the found tracks' intervals are longer than the missed
   ones'.
2. **The gap against itself.** Search the gap's peaks against the gap at
   speed 1.0 and the neighbouring rungs (the DJ may ride the pitch). A
   drum & bass record repeats its own material (the second drop repeats
   the first, loops reuse their samples), so lines away from the main
   diagonal join the moments where one record plays. Moments joined this
   way form blocks; the number of blocks is a count, and their edges are
   rough boundaries that overlap during blends. It needs no new features:
   the mix's peaks, pair hashes and line search exist. Risks: beatmatched
   records that share a famous break (Amen, Think) line up with each
   other and merge two blocks, the problem of the roadmap's "The filter
   and famous breaks"; a short play may repeat nothing; a double drop
   puts two blocks at one time. Glazyrin (2014) argues that
   repetition-based methods do not suit splitting a mix into tracks and
   that novelty-based ones do; that is about timbral features, and
   whether exact landmark repetition behaves better is to be measured.
3. **Novelty segmentation** (Foote 2000): a self-similarity matrix of
   spectral features, with change points where a checkerboard kernel
   along its diagonal peaks; Glazyrin (2014) and Scarfe et al. (2013)
   split DJ mixes this way. In jungle mixes consecutive records sound
   alike and blend for long, and a track's own drops and breakdowns are
   change points too. It needs feature code beside the peaks, computed
   from the mix only, so the peak store does not change. Tried only if
   the self-match fails.

The length gives the range; structure evidence from method 2 or 3 would
choose within it.

## Measuring

- Known counts come from leaving files out of the index. `scan
  --leave-out` picks the tracks by seed; gaps of consecutive tracks need
  a way to name the run. The generated mixes give exact truth, with
  crossfades, cuts and short plays.
- Step 1 is scored with `seconds`: the unlabelled seconds that listed
  gaps cover, and listed gaps where an indexed track plays.
- Step 2 is scored per gap: estimate minus truth by number of missed
  tracks, how often the range misses the truth, and gaps holding no
  missed track.
- Criteria are written before any count. The held-out test set is not
  used for development.
