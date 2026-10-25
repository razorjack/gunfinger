# 0219: Falcon - The Stand's two uploads: a speed that drifts, not trailing silence

**Question.** Brief 49, step 1 (exploratory, one pair): the "HD Audio &
Speed Up" upload (A, 485.1 s) was judged the same recording as the CITRUS
003 upload (B, 514.1 s); by ear the two part with B at +7.78%, read as one
file's longer silence at the end. Can that silence move a fit or placement?

**Commands.** FFmpeg `silencedetect`; `work/s49/falcon/drift.py`
(uncommitted): B's lag per 10 s of A against alignment 1 (B = 0.4 +
1.0779 A) from high-passed onset envelopes; `pair --fine-rungs`
(`work/bin/s49-run`) on A ~ B and on each upload ~ the corpus vinyl rip
`d. falcon - the stand.mp3`. Data: `data/0219-falcon-the-stand.json`.

**Silence.** Neither starts silent; A ends with 4.2 s below -60 dB (5.9
s below -45 dB); B has none and ends 3 s before A's music (A 476.7 s).
**The speed drifts.** Against alignment 1, B's lag goes smoothly from
-0.2 s (A 0:00) to +0.48 s (3:40), 0 (6:50) and -0.18 s (7:50) (rows on
a correlation peak a bar or two away set aside). The fine rungs agree:
A 0:00-0:57 lines up at 1.0835, 0:00-3:10 at 1.081, 2:50-7:50 at
1.0745-1.0757, all within 1.5 s of one line. Against the vinyl rip A
finds 6 hits (at about +11.6% it lies beyond the ladder); B runs at
1.031 (start) to 1.038 (end) times the rip's speed, so A against the rip
is about 1.116 at both ends: by inference B's digitisation drifts, its
speed rising 0.7%.

**A's first 30 s** hold no fitted alignment: the files run 0.5% from
the fitted speed there, where few hashes survive, and a 26-hit repeat
221 s away remains. The ladder's alignment (rungs 1.076, 1.080) spans A
0:00.5-7:48.9 at one offset, 1,423 hits.

**Length.** `pair` and `clusters` fit speed and offset from hits;
silence has no peaks. Length is only the denominator of coverage,
supported time and `near_10` (A's silence lowers each by 1%) and clips a
lane at its file's end; the page plays B at the alignment's speed, not
the length ratio (1.0597, 9 s apart by the end).

**Conclusions.** The verdict holds: one recording at one offset end to
end. The reading does not: the files part by up to 0.5 s because B's
speed drifts, which no single speed follows; cutting the silence would
change neither fit nor placement.
