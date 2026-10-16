# 0127: where the verifier's lines are off (exploratory)

**Question.** Session 24, after 0125, deciding nothing: the rejected
true plays had their lags pinned at +-48 ms. Is the reported line off?

**Method.** `scripts/analysis/version_lines.py`: per E1 play, 25 s mix
excerpts at three places per segment, identified against a store of the
reference's record alone; each excerpt's line against the reported one;
a play's offset is the median of its excerpts (>= 50 hits, within 1 s).
Ad hoc runs, E2's render truth and a sequential decode: `data/0127-*`.

| E1, 104 plays with a local line | Off > 24 ms | > 48 ms | > 96 ms |
|---|--:|--:|--:|
| By the median (of them rejected at T) | 20 (5) | 9 (5) | 6 (5) |
| By the worst excerpt (rejected) | 27 (5) | 15 (5) | 10 (5) |

1. **The opening window sets the line.** `refine.rs` takes the track
   start from the opening window's deviation, the speed from the chain.
   Phantom Force: the mix holds the record at another alignment from 653
   to 723 s; the line from the window at 715.8 s lies 134-139 ms from 8
   excerpts' lines. Kontempt.mp3: 158-165 ms off; excerpts whose play
   opens 18 or 37 s later move the anchor by 67 or 130 ms.
2. **Uploads whose speed drifts.** alien5ive's ABPR011, udfr017 and ABPR
   014 change local speed by 0.3-0.6% along the play, up to 350 ms off;
   the corpus Auto Ignition and Pathogen at the same mix times keep one
   speed, within 22 ms. E2's positive control, the PRO 012 A Sonar
   upload, drifts too (1.0034 to 1.0069).
3. **A damaged reference.** Do Drop In (E2, drawn) has damaged MP3
   frames before 427 s: the sequential decode the store was built from
   runs 235 ms ahead of FFmpeg's seek, which `render_samples` uses.

- Off is not always rejected: Kraken - Dominion (opus) is 122 ms off at
  its one excerpt, S1 0.027; four more kept plays have an excerpt over
  96 ms. Astro-Sine (E2, 18 ms from the truth) is not timing: under the
  render's bass cut the outgoing bass sits near the predicted level, so
  r < 0.2 contradicts the six lowest bands (8 of 65 patches).

**Conclusion.** Every rejected E1 play, and 4 of E2's 6 rejected true or
control plays, has a line this far off; that this causes them is an
inference (not rerun on corrected lines). A verifier needs its own line.
