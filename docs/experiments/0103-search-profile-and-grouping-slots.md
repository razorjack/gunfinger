# 0103: the NAS search's profile, and the pre-filter's slots computed once

**Question.** Session 18, step 6: where does the 2003 mix at NAS scale
(`--store-only --playback both -f json`, 33,723 records) spend its CPU
after 0098, and can the pre-filter and grouping cost less, identically?

**Profile** (`work/s18/profile.sh`, `sample` every 10 ms; a build with the
grouping's functions not inlined: 47.9 s, 378 s CPU, 4.31 GB): the
pre-filter (`may_form_line`) 25.6% of CPU samples, grouping a part by
asset 23.2%, taking a part from the lists 10.5%, the counting sort and
lines 5.9%; the ladder's STFT and peak picking 27.6% and its second pass
3.3%; records 2.2%, the saved index 0.8%. The first 5 s run at 100-260%
(headers, the index load beside decoding), the search at about 960%.
Analysis has no identical-output headroom left: the picker reads every
bin, and FFT and `log10` must stay bit-identical.

**Variants**, each giving the same report byte for byte (A B B A or
pairs against `s18-start`, which ran 375.9-385.9 s CPU, 43.8-45.5 s):

| Variant | CPU (user) | Wall | Memory |
|---|--:|--:|--:|
| V2: slots counted and scattered straight from the lists, candidates gathered and sorted | 402.9, 403.6 s | 47.0, 47.7 s | -0.8% |
| V2b: slots computed while taking the part, scattered, candidates placed by counting | 377.2, 377.1 s | 45.1, 44.8 s | -0.2% |
| V3: 0098's structure, each hit's slot computed once while taking the part (`OffsetSlots::slot_of`), the pre-filter reading `u16` slots | 371.2, 371.6, 371.5, 372.0 s | 43.9, 43.7, 44.2, 44.0 s | +0.5% |
| `s18-start` beside V3 | 377.2, 376.5, 376.1, 376.3 s | 44.2, 44.0, 43.8, 44.7 s | |

Load average 4.7-27 when a series started (the previous run's threads
still counted); other processes at 0-5.8% of a core.

**Conclusion.** None kept. In the split profiles, slots computed once
cut the pre-filter from 8,610 CPU samples to 4,995 (V2) and 5,035 (V2b),
but V2's grouping straight from the lists takes 18,743 samples where the
start build's taking, grouping and lines take 13,307, and V2b's taking
and grouping take 2,933 more. V3 saves 1.3% of CPU with no change in
wall time (the search runs on every core either way), for a second
array per part and a second code path; the user would not see it. Code
in `work/s18/lines.rs.v2`, `.v2b`, `.v3`; data
`data/0103-search-profile-and-grouping-slots.json`.
