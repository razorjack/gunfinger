# 0024: Each gap line estimates how many tracks the gap holds

## Status

Accepted (2026-10-27), our decision. Part 2 of the decision that ADR
0023 implements in part; it amends ADR 0023's items 5, 6, 7 and 11: a gap
line, a gap's table row and a gap in JSON gain an estimate of how many
tracks the gap holds. The gaps themselves, the search, the detections,
the confidence rule (ADR 0013), the entries (ADR 0017, ADR 0019), CSV,
the cue sheet and the timeline do not change.

## Context

Our decision of 2026-10-27 (`docs/research/2026-10-21-printing-gaps.md`,
"Decision", item 5) asks for an estimate of how many tracks a gap holds,
although none of the measured estimates met the bar registered for a
count ("about n" needs 90% exact at each k, the number of missed tracks
a gap holds): the form must say how sure it is. Sessions 34 and 35
measured three estimates (experiments 0160-0166):

- **Method 1**, the gap's length over the median start-to-start interval
  between the mix's entries, leaving out intervals that hold a listed
  gap; it needs 3 such clean intervals, and most gaps of a mix missing a
  third of its tracks have none (0160). Range B is the gap over the
  upper and the lower quartile of the same intervals.
- **Method 2**, the gap matched against itself: a drum & bass record
  repeats its own material, so stretches of the gap joined by exact
  repeats count its records (0162). It counts DJ-length records better
  than the length does, but two records that share a passage, played
  back to back, usually make one stretch (0163), and a sustained pitch
  ride lowers its count (0166).
- **The combined estimate**, method 2's count limited to method 1's
  range B (0165): the best measured.

## Decision

1. **The estimate** is the combined one, as 0165 measured it. Method 2:
   the gap's pair hashes, at speed 1.0, matched against its own at lags
   of 8 s or more; 5 s cells joined by 10 hits or more at one lag; the
   groups of joined cells spanning 30 s or more, overlapping ones
   merged, are the stretches, and their number is the count. Method 1's
   range B limits it: raised to its low end or lowered to its high end.
   A count of 0 stays 0. A gap in a mix with fewer than 3 clean
   intervals keeps method 2's count.
2. **The printed form is a range**, one either side of the count n, at
   least 1: `about L-H tracks` with L = n-1 (at least 1) and H = n+1,
   after the length on the gap line and in the table's gap row:

   ```text
          6:43  ID - ID (7:57 without a confident match; about 1-3 tracks)
   ```

   Never a single number. A count of 1 holds one track for 99% of the
   test's gaps that hold one, but a printed 1 was right for 95 of 125
   test gaps (76%); the other 30 held two. A count of 1 prints `about 1-2
   tracks`.
3. **No estimate** is printed, and the line stays as ADR 0023 has it,
   when the count is 0 (no repeated stretch of 30 s in the gap), and for
   a report saved before this change. Neither method can say that a gap
   holds no track: the neighbour discount meant to do so failed its test
   (0164), and short records that repeat little give 0 as well (0162).
4. **The mix's start and end.** Their gaps get the same estimate: method
   2 needs only the gap's audio, and the intervals come from the entries
   between. The measured gaps include such edges; none of them was an
   intro or an outro without a record.
5. **JSON.** Each gap gains `tracks`, always written: `null` without an
   estimate, otherwise `{"low": L, "high": H, "method": M}`, with M
   `repeats-within-length` for the combined estimate and `repeats` for
   method 2's count alone. The report gains `gap_repeats`, the stretches
   found in its gaps (start and end in the recording's time): the
   evidence the estimate is derived from whenever the report is
   rendered, as the gaps are, so `gunfinger show` prints it without
   decoding the recording again. An added field; schema version 3 allows
   it.
6. **Where it runs.** `identify` matches each gap of 45 s or more against
   itself while it holds the searched part's decoded samples (no second
   decode), one gap per worker. Method 1's entries and intervals are the
   harness's (confident plays, two over the same passage taken as one;
   the gaps listed at 45 s), and the core computes method 2, method 1's
   range B and the combination for the CLI and the harness alike
   (`gunfinger_core::repeats`, `gunfinger_core::gaps`).
7. **Reports saved before this change** print their gaps without an
   estimate. `identify --save-dir` does not search them again on that
   account (the search settings are unchanged); `--again` does.

## Accuracy, measured

| Gaps, k = 1 / 2 / 3 / 4 | Exact | Range holds | Off by two or more |
|---|---|---|---|
| Test mixes of 0163, shared material and control, 384 (0165) | 99 / 57 / 58 / 43% | 100 / 99 / 100 / 89.6% | 2.9% |
| Their records sharing a passage only, 192 | 98 / 44 / 48 / 31% | 96% (83% at k = 4) | 4.2% |
| Long generated mixes, development, 320 | 98 / 86 / 75 / 60% | | 0 |
| Method 2 alone (fewer than 3 clean intervals), the test of 0163 | 98 / 49 / 52 / 36% | 92% | 7.6% |

The range of one either side falls one gap short of the registered 90%
at k = 4 on the test, mostly where records share a passage. Read the
other way, on the test the printed range held the truth for 125 of 125
gaps counted 1, 88 of 97 counted 2, 106 of 106 counted 3, 49 of 50
counted 4, 5 of 5 counted 5 and 0 of 1 counted 6
(`work/s35/scores/tests-shared-and-beatmatched.json`).

Run time: method 2 took about 0.05 s of wall time per gap at 16 jobs on
the Linux machine (320 gaps in 21 s, 0162), from audio already decoded.
In `identify` (experiment 0235, NAS scale, 32 workers) the matching takes
under 0.05 s for the development mixes and 0.7 s for one 67-minute gap,
and the run takes 0.3-0.5 s longer, most of it in the unchanged search,
compiled differently beside the new code.

## What the estimate does not claim

- It is not a count, and it does not say that the rest of the mix is
  explained.
- It never says 0. A gap that holds no missed track (time inside a found
  track that no segment covers) gets at least `about 1-2 tracks` when the
  found track repeats itself there (0162). At 45 s the development mixes
  list no such gap (0234); a sustained pitch ride can leave one (0166).
- Records sharing a passage, played back to back, are often counted as
  one, so the range runs low in mixes of one label or one break (0163;
  how common that is in real mixes is not measured).
- A sustained pitch change within a record lowers method 2's count;
  only speed 1.0 is matched (0166 measured more rungs on 33 gaps,
  outside its registration).
- The accuracy was measured on generated mixes and the development
  mixes' gaps, not on other real mixes.

## Consequences

- Gap lines with a repeated stretch in them gain `; about L-H tracks`;
  JSON gains `tracks` on each gap and `gap_repeats` in the report.
  Detections, entries and `regress` are unchanged.
- `identify` takes a little longer per gap; experiment 0235 has the
  figure.
- The harness's `self-match` uses the core's code and writes the
  product's estimate beside its blocks, so the estimate is measured by
  the code that prints it.
