# 0175: where the search loses a side of a pitch ride

**Question.** Session 37, step 1: 0166 found 192 of 382 ridden indexed
plays with a side of 30 s or more and no confident detection. Which stage
of the search drops it, by which rule?

**Method.** Session 35's rides seeds 3521 and 3522 rendered again; both
reports equal 0166's apart from `wall_seconds`. A diagnostic build (never
committed) wrote each mix's detections of 30 hits or more at four stages:
the chains, after the first `strongest_per_moment`, after the second pass
(`refine`) and after the second `strongest_per_moment`.
`scripts/analysis/ride_stages.py` names, for each lost side, the furthest
stage the side's own chain reached (the chain overlapping it more than the
other side). Data `data/0175-where-a-ride-loses-a-side.json`.

| Seeds 3521 / 3522 | |
|---|--:|
| Ridden indexed plays; sides of 30 s or more | 191 / 191; 329 / 327 |
| Lost sides (240 hits in 3 windows); with 30 s or more uncovered | 93 / 99; 101 / 120 |
| Lost in lines or chains; at the first suppression; in the second pass | 0; 0; 0 |
| Confident after the second pass, dropped at the second suppression | all, either count |
| Overlap of the two sides' chains | 0 s in all 192 |
| Overlap after the second pass: median; 90%; max | 4.7-5.2 s; 10.4-10.5 s; 15.8 s |
| Facing end moved by the second pass: dropped side; kept side (medians) | 3.8-4.0 s; 3.6-4.1 s |
| Dropped side's fitted speed against its rendered speed: median; max | 0.002%; 0.028% |

- 379 of the 382 ridden plays split into two chains at the ride, each
  confident after the second pass (1,539-6,863 hits for the dropped
  sides); 303 then lose one (192 a side of 30 s or more), 76 keep both
  as segments of one play. One dropped side only touches the kept one.
- The generator renders a ride as a turntable would: the native render
  read at the integrated position (`source_at`, continuous, tested), so
  pitch and tempo change together; the fitted speeds match the rendered.

**Conclusion.** The side is lost by `strongest_per_moment` after the
second pass: the chains of the two sides never overlap, but measuring
each at its fitted speed up to 10 s beyond its chain (`MARGIN_SECONDS`)
moves their facing ends a few seconds into the ramp, so the spans
overlap and the weaker side, confident on its own, is dropped whole.
Plays and same-audio merging come after and only group what is left.
