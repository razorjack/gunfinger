# 0127: where the verifier's lines are off (exploratory)

**Question.** Session 24, after 0125, deciding nothing: the rejected
true plays had their lags pinned at +-48 ms. Is the reported line off?

**Method.** `scripts/analysis/version_lines.py`: per E1 play, 25 s mix
excerpts at three places per segment, identified against a store of the
reference's record alone; each excerpt's line against the reported one
(points of >= 50 hits, within 1 s). Ad hoc: earlier excerpt starts, E2's
render truth, a sequential decode against FFmpeg's seek. `data/0127-*`.

| E1, 104 plays with a local line | Off > 24 ms | > 48 ms | > 96 ms |
|---|--:|--:|--:|
| Plays (of them rejected at T) | 20 (5) | 9 (5) | 6 (5) |

1. **The opening window sets the line.** `refine.rs` takes the track
   start from the opening window's deviation, the speed from the chain.
   Phantom Force: the mix holds the record at another alignment from 653
   to 723 s; the line from the window at 715.8 s lies 137 ms from the
   body's, constant over 8 excerpts. Kontempt.mp3: the anchor moves with
   the excerpt's start (0, +67, +130 ms from 1800, 1880, 1900 s); the
   body lies 160 ms off.
2. **Uploads whose speed drifts.** alien5ive's ABPR011, udfr017 and ABPR
   014 change local speed by 0.2-0.5% along the play (ABPR011 1.0006 to
   0.9952), up to 350 ms off; the corpus Auto Ignition and Pathogen at
   the same mix times keep one speed, within 22 ms. E2's positive
   control, the PRO 012 A Sonar upload, drifts too (1.0034 to 1.0069).
3. **A damaged reference.** Do Drop In (E2, drawn) has damaged MP3
   frames before 427 s: the sequential decode the store was built from
   runs 235 ms ahead of FFmpeg's seek, which `render_samples` uses. The
   development mixes and the rejected E1 references decode cleanly.

- E2's two Astro-Sine rejections are not timing (18 ms from the truth):
  the render's bass cut in a blend reads as contradicted in the six
  lowest bands (the outgoing track masks two, so the kill rule does not
  apply): 8 of 65 patches of an 18.5 s play.

**Conclusion.** All 5 rejected E1 plays and 4 of E2's 6 rejected true or
control plays fail on the line, not DJ processing: a verifier needs its
own line from the audio. Not rerun on corrected lines (E1's second use).
