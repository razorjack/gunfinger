# Version verification along a fitted alignment: an external report (2026-10-16)

**Status: unverified.** This note records an external research report
and checks what code reading and arithmetic can check. Its citations
were not fetched and its numbers from the literature are unverified.
Nothing here authorises a change to the product: a verifier that rejects
or annotates plays changes the output, which is our decision
(ADR 0017), and one that needs dense features stored beside the peaks
changes ADR 0003. Session 24 builds a diagnostic version of the first
method in the harness and measures it against criteria written before
any count (`docs/brief-24.md`, experiment 0124 and after).

## Provenance

- A GPT Pro model was asked on 2026-10-16 how to test, along a
  fingerprint alignment, whether a mix still plays a reference's musical
  content where the pair hashes are silent, and how to tell a different
  version from the same recording degraded by DJ EQ, blends and cuts.
  The Claude agent that launched session 24 drafted the prompt; the
  prompt was sent and its answer relayed by hand.
- The prompt and the verbatim answer are in
  `work/research/gpt-pro-version-verification/` (`prompt.md`,
  `gpt-pro-response.md`). `work/` is not committed, so this note is the
  durable record.
- The prompt gave the model the matcher's outline (8 kHz, STFT
  1024/128, 41 turntable rungs from 0.92 to 1.08, 240 aligned hits for
  a confident play), the panel's 106 of 218 wrong confident queries and
  the failure of hit sparseness on real mixes. It gave no code, no
  experiment records and no data.
- "Local check" marks what this note's author checked in the code or by
  arithmetic. No corpus run stands behind this note; session 24's runs
  are in its experiments.

## Protections

As in `docs/research/2026-10-14-matching-proposals.md`: the held-out
test set and its NAS alias, the recall panel's validation half, the
shared panels' validation half and the second shared panel, the five
test-derived versions and the candidate mixes of experiment 0065 are not
used to explore, tune or calibrate anything here. The NAS and its peak
store are read-only; verdicts and listening answers are given by ear.

## The report's formulation

- **Signal model.** The mix is `y(t) = g(t) [h_t * x](phi(t)) + u(t)`:
  the reference `x` through a fader gain `g`, a slowly changing EQ
  `h_t` and the time map `phi(t) = o + s t + delta(t)`, plus everything
  else `u`. Without restrictions the hypothesis cannot fail (`g = 0`
  explains any mix), and demanding that `x` explain the whole mix
  rejects every blend.
- **The question it proposes**: given the reference material that
  demonstrably survives in an interval, could a plausible common
  gain/EQ also explain what happened to the reference's other expected
  material?
- **Three local states**: *supported*, *contradicted* (the reference's
  content should be audible under a plausible gain/EQ and is not) and
  *unobservable* (killed, faded or masked). Beside them, a separate
  measure of **version distinctiveness**: a supported break that other
  versions share does not support the exact title, and hundreds of hits
  on a shared passage must not make up for no version-specific evidence.
- Local check: this addresses the failure of sessions 20-23. Hit
  sparseness (0114, 0116, 0118) and the ambiguity note (0123) count
  every stretch without hash hits, or every shared passage, against a
  play; DJ EQ, blends and cuts also silence hashes, so true plays in
  real mixes were rejected or flagged (46-72% kept; 15 of 108 flagged).
  A three-state test can in principle tell a killed band from missing
  content. It cannot tell a missing bassline from a bass kill (the
  report says so), and where two versions are identical over the
  passage the mix played, no test can (0112's outcome 1).

## The methods, as the report gives them

### A. Candidate-restricted dense descriptors

- **Report**: Ramona and Peeters (DAFx 2011) verify fingerprint
  annotations with 36-dimensional descriptors on 2 s windows every 50
  ms; 29 of 30 deliberately wrong edits detected among 100 radio
  occurrences, the miss a shared vocal passage. A small experiment with
  one constant time scale, not an exact-version precision for DJ mixes.
- **Local check**: 20 descriptors a second x 60 s x 36 dimensions =
  43,200 comparisons a minute, as stated. The report's own warning
  applies: counting where dense descriptors disagree, without
  observability, repeats the sparseness test.

### B. Band-wise spectral dynamics

- **Report**: compare log band energies `L(f, t)` and their temporal
  differences `D(f, t) = L(f, t) - L(f, t - Delta)` along the alignment,
  in 1-3 s patches every 100-250 ms. For one source under a
  time-invariant gain, the gain cancels in `D`; added music, fast EQ
  moves and near-total attenuation do not cancel. Haitsma and Kalker
  (ISMIR 2002): band-difference bits survive +-3 dB EQ (bit errors
  0.045-0.066) but not +-4% speed (0.355-0.472). Lopez-Serrano et al.
  (ISMIR 2016): on seven constructed EDM tracks, log-frequency cosine
  matching correlated 0.11 with loop activations, peaks 0.51-0.53. Keep
  several band scores: the minimum punishes bass kills, the maximum
  accepts one shared stem.
- **Local check**: the cancellation holds for `L = log |H|^2 + log |X|^2`
  when the filter is short against the 128 ms window and constant
  between the two frames; a DJ EQ knob moved over seconds changes `D`
  only by its own slope. The harness already has a related measure:
  `identifiability`'s *audio* (0112), the median correlation of
  band-centred log spectra along the line, gave the related file
  0.35-0.60 and the played recording's rips 0.72-0.88 on clean
  passages. It compares spectral shape per frame, not dynamics per
  band, and it has no observability.

### C. A constrained fader and EQ fit

- **Report**: Kim, Yang and Nam (DAFx 2022) estimate DJ fader and EQ
  gains jointly with a fixed basis of filter responses by convex
  optimisation; over 50,742 transitions of 5,040 real mixes the
  mel-spectrogram reconstruction error falls from 5.14 (fader only) to
  4.24 (joint). Proposed adaptation: a small, bounded, temporally
  regularised gain/EQ basis per candidate, fitted on some material and
  tested on other material, so that the fit cannot explain away the
  distinctive part; a known neighbouring record may join the model, an
  unknown one adds uncertainty, never a rejection. Two questions:
  *presence* (does the aligned reference explain structure the
  background cannot?) and *compatibility* (does the fitted gain/EQ
  predict reference structure that fails to appear where it would be
  observable?). Reconstruction improvement is not evidence of the right
  version.
- **Local check**: power additivity (`P_y >= g P_x` with `u` adding
  power) holds only approximately per bin; summed over a band of several
  bins and frames, cancellation between correlated sources is small but
  not zero. A three-band DJ EQ with kills can remove whole bands, so a
  contradiction must be *selective*: part of the reference present at
  the fitted gain while other content in the same band group, or the
  same band at nearby times, is missing.

### D. Complex coherence and waveform projection

- **Report**: magnitude-squared coherence over a window tests whether
  the waveform itself survives; strong positive evidence on a
  distinctive passage, fragile negative evidence (other transfers,
  nonlinear processing, residual time variation). Schwarz and Fourer
  (CMMR 2019): 28.2 ms median start error after "sample alignment" on
  444 synthetic three-track mixes, subtraction sensitive to 0.1% speed
  errors.
- **Local check**: coherence of a single observation is 1 by
  definition (`|X* Y|^2 = |X|^2 |Y|^2`), so it needs averaging, as the
  report says. At 8 kHz, 10 ms is 80 samples, far more than phase
  coherence tolerates at 1-4 kHz; vinyl rips of one record differ in
  transfer and speed drift. Not tested in session 24.

### E. Fixed-template presence tests (NMF, NMFD)

- **Report**: fixed reference templates with non-negative activations
  along the alignment; Lopez-Serrano et al. reached activation
  correlations 0.87-0.88 with known loops; Gururani and Lerch (2017)
  used partially fixed NMF with a trained classifier (79.37% / 34.60%
  location precision/recall), which this project's rules exclude
  (no trained model); Andre, Fourer and Schwarz (2024 preprint) on
  1,931 artificial mixes. A detector's "this break is present" is
  exactly why the wrong version is returned.
- **Local check**: no training is needed for a fixed-template NNLS fit,
  but its cost grows with the template count (below). Not tested in
  session 24.

### F. Chroma, CQT and local alignment

- **Report**: Serra et al. (2008) cover-song identification and Kim et
  al. (ISMIR 2020, 1,557 mixes, median cue-in error 11.40 s) work at the
  level of a composition or a segmentation, not the exact version.
  Keeping only the best local alignment rewards the shared passage;
  unrestricted DTW can jump between repeated breaks.
- **Local check**: 0.92-1.08 is `12 log2(s)` = -1.44 to +1.33
  semitones, so integer chroma rotations do not compensate a turntable
  speed, as stated. Not tested in session 24.

## Speed and alignment

- **Report**: render the reference onto the mix clock by band-limited
  resampling at the continuously fitted speed, not the ladder's rung (a
  half step, 0.002, drifts 120 ms a minute); allow a small smooth timing
  correction shared across bands, never per band. A candidate-only 16
  kHz verification is worth testing.
- **Local check**: the ladder has 41 rungs from 0.92 to 1.08 in steps
  of 0.004 (`speed.rs`: `SLOWEST`, `FASTEST`, `STEP`); 0.002 x 60 s =
  0.12 s. The fitted matcher (ADR 0008) analyses each candidate again
  at its fitted speed, and every report's detection carries that
  continuous speed and the line's offset (`track_start_seconds` at
  `start_seconds`, file time = offset + speed x t), so the rung's
  residual does not reach a verifier that uses the detection's line.
  Within a long segment the line is one straight fit; vinyl drift and
  pitch riding bend the true map, so a few frames of local lag are
  expected. The harness renders a file at a turntable speed with FFmpeg
  (`render.rs`: `asetrate` between two `aresample`s). The profile
  analyses bins 5-500 (39 Hz-3.9 kHz) of a 1,024-point STFT at 8 kHz:
  128 ms window, 16 ms hop, 7.8125 Hz bins, as stated.

## Cost

- **Report**: no comparable published CPU times. Derived budgets for
  one candidate-minute: 3,750 frames, 1.924 million one-sided STFT
  values, 0.48 million at 128 bands; a six-coefficient NNLS fit at 128
  bins and 20 sweeps per frame about 115 million multiply-accumulates; a
  64-template fit at 50 sweeps about 3.07 billion; about 192 million
  operations for the 1,024-point FFTs. A minute of 513-bin float32
  magnitudes is 7.7 MB.
- **Local check**: each figure follows from its stated assumptions
  (3,750 x 513 = 1,923,750; 6 x 128 x 20 x 2 x 3,750 = 115.2 million;
  64 x 128 x 50 x 2 x 3,750 = 3.072 billion; 5 x 1,024 x 10 x 3,750 =
  192 million; 3,750 x 513 x 4 bytes = 7.70 MB). Against the peak
  store's 187 bytes per second of audio (ADR 0003), 7.7 MB a minute is
  about 690 times as much; 20 bands of one byte at 62.5 frames a second
  would be 1,250 bytes a second, about 6.7 times. Whether anything dense
  is stored is our decision.

## Evaluation, as the report proposes it

- Define the identity boundary first (rips and transfers of one version
  are one; remixes and arrangement-changing edits are not). This project
  has it: the verdicts by ear and the duplicate clusters (ADR 0017).
- Matched hard negatives: the same DJ processing and overlapping record
  applied to the reference `R` (positive) and to a related version `V`
  with `V` left out (negative), the shared passage fixed and 0, 5, 10
  and 20 s of distinctive context added, audible, attenuated, masked and
  cut away; cross-rip positives and real mixes; run the whole retrieval
  first, so that the verifier sees the candidates the matcher chose.
- Report retained true plays at a fixed wrong-version rate, not F1;
  split by family and mix; uncertainty clustered by mix or family.
- **Report**: 218 independent zero-error queries bound the error rate
  at `1 - 0.05^(1/218)` = 1.37% (one-sided 95%); 2,995 are needed for
  0.1%.
- **Local check**: both figures are right (ln 0.05 / ln 0.999 =
  2,994.2). The 218 queries come from 14 directions of 6 families, so
  the effective count is far smaller: with the family as the unit,
  zero errors in 6 families bound the rate at `1 - 0.05^(1/6)` = 39%.

## Caveats

- **The verifier needs the reference audio.** The peak store holds
  peaks and tags, not audio or dense features (ADR 0003). A product
  verifier would decode each candidate's file at identification time,
  so it would not work for `identify --store-only` or when the library
  is unreachable; storing dense features instead changes ADR 0003 and
  the store's size. Both are our decisions, not this session's.
- **The binding test is the real true plays** of the three development
  mixes. Rendered plays were three times denser than real ones and set
  thresholds that failed on real mixes (0116, 0118).
- **Distinctiveness is only as good as the siblings indexed.** In the
  main case the played version is not indexed; where no third version
  is (Dozer, Pyro, Sonar in the panel), nothing in the library marks
  the related file's shared passage, and only the *contradicted* state
  can reject its play. Session 23's `holders` lookup names the files
  that hold a passage (at least 30 hits) but keeps no span within it;
  the regions need a search of the reference against those files.
- **Selective processing.** A DJ who kills exactly the band in which two
  versions differ makes the difference unobservable; the verifier must
  then abstain, which keeps the wrong title.
- **Cuts and divergence look alike.** When the reference stops matching
  in every band at once, a fader cut and the played version diverging
  from the reference are the same observation unless another indexed
  record explains what follows.

## What session 24 tests

- Methods B and C together, as the report's first experiment:
  band-wise log energies and their differences along each play's
  fitted lines, the reference rendered at the fitted speed, a bounded
  gain/EQ model of three DJ bands fitted on part of each patch and
  tested on the rest, the three states per patch, and distinctiveness
  from the files that hold the reference's passage. The design, the
  per-play summaries, the split and the criteria are in
  `docs/brief-24.md` (written before any count).
- Not tested: coherence (D), templates (E), chroma (F), 16 kHz.
- The report's matched hard-negative test (0, 5, 10, 20 s of
  distinctive context) on two calibration families, if time remains;
  otherwise its plan for session 25.
