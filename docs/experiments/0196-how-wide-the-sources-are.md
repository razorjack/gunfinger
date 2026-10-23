# 0196: how many corpus sources are mono or nearly mono

**Question.** How many of the corpus's and the NAS store's sources are
mono or nearly mono (brief 42's pre-registration)?

**Change.** Harness only: `gunfinger-eval stereo-width` (corpus only)
decodes a seeded sample of the library files with a current peak record
whole, as two channels at 8 kHz (`-ac 2`; a one-channel file gives two
identical ones), and reports the side's energy against the mid's
(`stereo::side_to_mid_db`, S = (L - R) / 2, M = (L + R) / 2); `robust`
keeps the same ratio for each stereo query.

**Command.** `stereo-width --seed 2026 --sample 100` (5 s); the 50
excerpts' widths from `robust --only stereo-control,...` at 262 tracks
(the mean of each excerpt's two speeds). Data:
`data/0196-how-wide-the-sources-are.json`.

| Side against mid | Sample, 100 of 265 files | `robust`'s 50 excerpts (40 indexed, 10 held out) |
|---|--:|--:|
| Mono (-40 dB or less) | 5 | 2 (1, 1) |
| Nearly mono (-40 to -20 dB) | 29 | 15 (15, 0) |
| Narrow (-20 to -10 dB) | 62 | 28 (20, 8) |
| Wide (above -10 dB) | 4 | 5 (4, 1) |
| Median | -17.7 dB | -17.6 dB |
| Side louder than mid | 0 | 1 (held out; +0.3 and +0.7 dB) |

The 5 mono files lie at -104 to -52 dB; the widest file at -7.5 dB.

**NAS store.** Not measured. A peak record holds one peak list, picked
from FFmpeg's downmix, and no measure per channel (`store.rs`), so the
stored peaks cannot tell a mono source from a stereo one; the brief
rules out decoding NAS files for it. The corpus was drawn from the same
collection, so the sample is the nearest estimate, by inference only.

**Conclusion.** About a third of the corpus's sources are mono or nearly
mono (34 of 100 files, 17 of 50 excerpts); almost all the rest are
narrow, the side 10-20 dB below the mid. No sampled file has its side
louder than its mid; over 30 s one held-out excerpt has, so (by
inference from one excerpt) a test on short stretches can fire where a
test on whole files does not.
