# References

Papers and datasets behind Gunfinger's design and its open questions.
Each entry says what the source shows that matters here, with section
or table numbers, and where the project uses it. Figures quoted from a
paper are that paper's results on its own data, not predictions for
Gunfinger. Entries marked "not read" are leads only.

Short citations in other documents use the keys in bold.

## Fingerprinting

**Wang 2003.** A. L.-C. Wang, "An Industrial-Strength Audio Search
Algorithm," Proc. 4th International Society for Music Information
Retrieval Conference (ISMIR), Baltimore, 2003, pp. 7-13.
<https://www.ee.columbia.edu/~dpwe/papers/Wang03-shazam.pdf>

- §2.1: spectrogram peaks are local energy maxima in a neighbourhood,
  chosen by a density criterion so that the time-frequency plane is
  covered evenly. Gunfinger's peaks (`peaks.rs`, ADR 0003) and the check
  that no band takes most of them (experiment 0034).
- §2.2: hashes from an anchor peak paired with peaks in its target zone;
  the fan-out factor F; a hash survives with probability about
  p·[1-(1-p)^F] when each peak survives with probability p. Gunfinger's
  pair hashes and `FAN_OUT` (`hash.rs`, ADR 0001, experiment 0004).
- §2.3: hits are grouped by track, and a match is a diagonal line of
  time pairs, found by sorting the offset differences per track and
  scanning for a cluster; the score is the number of hits in the
  cluster. Gunfinger's lines (`search/lines.rs`), which sort each
  window's hits by asset and offset, generalised to a speed per rung.
- §2.3.1: the threshold comes from the score distribution of the
  highest-scoring wrong track, which depends on the number of tracks in
  the database. Gunfinger's confidence rule against a measured null
  (`confidence.rs`, ADR 0001) and why it must be measured again as the
  library grows (`docs/calibration.md`).

**Six & Leman 2014.** J. Six and M. Leman, "Panako - A Scalable Acoustic
Fingerprinting System Handling Time-Scale and Pitch Modification," Proc.
15th ISMIR, Taipei, 2014.
<https://archives.ismir.net/ismir2014/paper/000122.pdf>

- §1: replay speed changes by accident when analogue media is digitised,
  and media may have been recorded at an undocumented speed. The NAS
  clusters show this between rips of one recording (roadmap, "The edge
  of the ladder").
- §2.2-2.5: triplets of peaks hashed by frequency differences on a
  constant-Q scale and the ratio of time differences, invariant to time
  stretch and pitch shift up to about 10%. The brief's candidate B (ADR
  0001); experiment 0014 measured exact triplets, not these invariant
  ones.

**Six 2021.** J. Six, "Panako 2.0 - Updates for an Acoustic
Fingerprinting System," Extended Abstracts for the Late-Breaking Demo
Session of the 22nd ISMIR, online, 2021. This is the "Six 2021" of the
brief. <https://backoffice.biblio.ugent.be/download/8726851/8726856>

- §2: peaks that differ by about half a bin between query and reference
  spread over neighbouring bins, so exact hashes have off-by-one errors in
  time and frequency. Panako orders the hash components so that the last
  bits can be ignored in a range query (near-exact hashing), and accepts
  matches within a margin around a regression line from the first match
  to the last, which also allows linear speed changes.
- §3, figure 2: the top-1 true positive rate for 20 s queries sped up by
  10% rose from 18% (2014 version) to 83%.
- Gunfinger: roadmap, "Evidence beyond exact pair hashes".

**Sonnleitner & Widmer 2016** (Qfp). R. Sonnleitner and G. Widmer,
"Robust Quad-Based Audio Fingerprinting," IEEE/ACM Transactions on
Audio, Speech, and Language Processing, 2016,
doi:10.1109/TASLP.2015.2509248.
<https://www.cp.jku.at/research/papers/preprint_TASLP2509248.pdf>

- Quads of four peaks give hashes invariant to time and pitch scaling,
  searched by range queries; robust to ±30% in its evaluation.
- §IV-B: queries are analysed denser than references (smaller max
  filters, wider grouping regions, more quads per second), so the
  reference database stays small while the query has more chances to
  reproduce the stored features.
- §VI-B: per track, a histogram of offsets as in Wang 2003, with query
  time scaled by the fitted time-scale factor; outliers removed by the
  variance of their scale estimates.
- §VI-C: each match is verified by aligning the reference peaks near it
  with the query, using the fitted scale factors, and counting those found
  within a tolerance box (12 frequency bins by 18 frames there). It looks
  for reference peaks in the query, not the reverse, and accepts a share
  of them, so extra and missing peaks are tolerated. The paper calls this
  essential for precision on large collections with repetitive material.
  The idea is adapted from Astrometry.net (Lang et al. 2010).
- Gunfinger: roadmap, "Evidence beyond exact pair hashes" and "Longer
  hashes".

**Sonnleitner et al. 2016** (DJ-mix monitoring). R. Sonnleitner, A.
Arzt and G. Widmer, "Landmark-Based Audio Fingerprinting for DJ Mix
Monitoring," Proc. 17th ISMIR, New York, 2016, pp. 185-191.
<https://archives.ismir.net/ismir2016/paper/000187.pdf>

- Compares Audfprint (Wang-style pairs), Panako 1.4 (triplets) and Qfp
  on 8 club mixes and the 10 Mixotic mixes, scored per second: accuracy
  and precision where the playing track is in the database, specificity
  (no answer) where it is not (§5).
- Table 2 (Mixotic): Qfp has specificity 0.927 and accuracy 0.876; with
  its verification relaxed so that no candidate is rejected, 0.647 and
  0.889 (club mixes: 0.942 and 0.741 against 0.888 and 0.765). Panako's
  triplets were more specific than Audfprint's pairs (0.349 against
  0.255; club mixes 0.746 against 0.487), at lower accuracy.
- §6: narrowing Qfp's range queries, which the authors liken to
  quantised hashes, cut Mixotic accuracy from 0.876 to 0.570 and raised
  specificity to 0.987.
- §6: with 430,000 extra tracks, Mixotic specificity fell to 0.56, partly
  because the extra tracks held a copy of a played song that the ground
  truth called absent. Crediting each answer to its matched interval
  rather than the whole 20 s query raised specificity to 0.80 (club
  mixes 0.71 to 0.89).
- Lower specificity on Mixotic is attributed to its larger reference set
  and highly repetitive tracks. Its song borders are approximate: the
  point where the previous track has faded out.
- Gunfinger: roadmap, "Evidence beyond exact pair hashes", "Public DJ-mix
  datasets and per-second scoring", "Clustering precision", "Detection
  boundaries", "Longer hashes".

**Smith & Serra 1987.** J. O. Smith and X. Serra, "PARSHL: An
Analysis/Synthesis Program for Non-Harmonic Sounds Based on a Sinusoidal
Representation," Proc. International Computer Music Conference, 1987.
Quadratic interpolation of spectral peaks, used for the fractional bin
of each peak (`peaks.rs`). Cited since session 1; not read again here.

## Sample identification

**Bhattacharjee et al. 2025.** A. Bhattacharjee, I. Meresman Higgs, M.
Sandler and E. Benetos, "Refining Music Sample Identification with a
Self-Supervised Graph Neural Network," Proc. 26th ISMIR, 2025,
arXiv:2506.14684. <https://arxiv.org/abs/2506.14684>

- In sample identification, finding the recording a track sampled is
  the right answer. For Gunfinger the same answer is a wrong
  identification: the task differs, the evidence is the same.
- §4.2: extends Sample100 (75 hip-hop queries, 68 sampled references,
  320 unrelated tracks) with sample boundaries to ±250 ms, time-stretch
  ratios and stems, and labels samples as riffs, drum breaks ("beats")
  or single notes.
- §6.3, table 4: drum-break samples were harder to retrieve than riffs
  (mAP 0.391 against 0.471), because drums are layered and buried in the
  mix.
- Gunfinger: roadmap, "A confidence statistic for shared material".

## Datasets

**Mixotic** (Sonnleitner et al. 2016). 10 Creative Commons DJ mixes of
techno and house (11 h 23 min), 723 reference tracks of which 118 are
played, with song-border annotations.
<https://www.cp.jku.at/datasets/fingerprinting/> (about 1 GB of mixes,
9.4 GB of references).

**UnmixDB.** D. Schwarz and D. Fourer, "UnmixDB: A Dataset for DJ-Mix
Information Retrieval," 2018, doi:10.5281/zenodo.1422385 (CC BY-NC-ND
4.0, about 4.2 GB). Beat-synchronous mixes generated from the Mixotic
tracks: each mixes three excerpts, the middle one between linear
crossfades, and is rendered 12 times (4 variants of effects, 3 of time
scaling, made with SoX). The ground truth gives each track's start, end
and cue points, BPM and speed factor. The venue of the paper (probably
an ISMIR 2018 late-breaking demo) was not confirmed.

## Not read

- T. Kim, M. Choi, E. Sacks, Y.-H. Yang and J. Nam, "A Computational
  Analysis of Real-World DJ Mixes Using Mix-To-Track Subsequence
  Alignment," ISMIR 2020 (cited by Six 2021).
- D. Schwarz and D. Fourer, "Methods and Datasets for DJ-Mix Reverse
  Engineering," CMMR 2019 (evaluated on UnmixDB).
- T. Kim, J. Kim, H. Kim and J. Nam, "Raveform: A Dataset of Metrical and
  Functional Structure Annotations for EDM Tracks in DJ Mixes,"
  Transactions of the ISMIR 9(1), 2026.
- D. Lang, D. W. Hogg, K. Mierle, M. Blanton and S. Roweis,
  "Astrometry.net: Blind Astrometric Calibration of Arbitrary
  Astronomical Images," The Astronomical Journal, 2010 (the source of
  Qfp's verification).
