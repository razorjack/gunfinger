# 0023: duplicate clusters from stored peaks

**Question.** `clusters` decodes every library file and searches it
against the library: too slow for 25,000 tracks. Does searching each
file's stored peaks, rescaled to each speed, find the same clusters?

**Change.** `search::search_peaks` (core) searches peaks instead of audio;
`gunfinger-eval clusters --from-peaks` uses it, writes
`duplicate-clusters-from-peaks.json` and compares with the clusters in
use. Same criterion: one alignment at 0.98-1.02 covering 80% of the
shorter file. Data: `data/0023-duplicate-clusters-from-peaks.json`.

**Controlled cases first** (`work/scripts/controlled-library.sh`, 10 files
from three corpus tracks): a rip at +0.6% in AAC, a crop, an edit
removing a minute in the middle and one removing 40 s near the end, an
A-B-C chain (first 65%, the whole at +0.3%, last 65%; first and last
overlap 46%), and a mash of 30% of one track and 70% of another.

| Method | Clusters | User CPU |
|--------|----------|---------:|
| Decoded audio | {original, rip, crop, late edit}, {first, whole, last} | 47 s |
| Stored peaks | the same | 2.7 s |

Both match the design. The middle edit stays alone (best alignment 51%),
the mash too (70%). Every pair's coverage agrees to 0.001; rescaled peaks
keep 88-95% of the hits where speeds differ, all of them where not.

**Library** (262 assets; the exhaustive run repeated with current code,
which reproduced the 17 clusters in use). Stored peaks find the same 17
clusters. No pair disagrees on the 80% criterion and none lies between
40% and 80% under either method; the 23 same-recording pairs keep 98-100%
of their hits. User CPU 231 s against 1,559 s.

**Conclusions.** (1) Stored peaks give the same clusters at a seventh of
the CPU, without decoding. (2) The criterion has a wide gap here, but an
edit with a cut in the middle is never a duplicate under it; that is by
design (one alignment), and worth knowing when edits are added.
(3) `--from-peaks` compares and does not replace the clusters in use.
