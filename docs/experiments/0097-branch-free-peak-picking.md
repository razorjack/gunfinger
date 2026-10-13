# 0097: peak picking without unpredictable branches

**Question.** Step 2 of session 17: `PeakPicker::pick` alone was 11.6% of
the NAS search's CPU (0096's profile, 4,962 samples), and peak picking
also runs in every `index`. Can it be faster with the same peaks?

**Variants**, the 2003 mix at NAS scale, `--duration 15:00`, A B B A,
user CPU, reports byte-identical:

| Variant | Against | CPU |
|---|---|--:|
| run maxima: each frame's maximum over 25 bins (a sparse table), checked over the 25 frames before the full scan | `s17-a3` | +5.7%, rejected |
| the four nearest neighbours compared without branches into a list of candidates | `s17-a3` | -7% |
| the neighbourhood scanned nearest frames first, each frame's bins without branches | above | -3.8% |

The run maxima cost more to compute for every frame than the scans they
save: the cost was not in the scans but in the four-neighbour test,
whose outcome changes from bin to bin and was mispredicted.

**Change** (`peaks::PeakPicker`): the four-neighbour test uses `|` instead
of `||` and writes candidate bins to a buffer, a missing frame standing in
as silence (`-inf`, which rejects nothing a finite floor does not,
NaN included); `dominates_neighbourhood` tries frames nearest first and
folds each frame's bins with `|`. Both are the same predicates in
another order. A test checks the peaks against the definition over a whole
spectrogram, on a chord with noise, on the spread neighbourhood, and on
noise repeating every 4 hops, where every local maximum ties and none is
a peak; it passes on the old picker too.

**Result.** `pick` 4,962 -> 1,537 samples. With 0096's first steps, the
2003 mix at NAS scale: 51.7-51.9 s and 388 s CPU against 58.9-62.1 s and
480-483 s. `regress session-17-start` identical at 262 tracks, where
analysis dominates: 151.7 s and 1,318 s CPU with the session's build
(`s17-step2`) against 200.8 s and 1,796 s with the start build. Peak
records stay the same; step 3 checks that on scratch stores. **Kept.**
Data `data/0097-branch-free-peak-picking.json`.
