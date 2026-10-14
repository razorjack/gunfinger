# Matching and speed proposals from an external review (2026-10-14)

**Status: five unverified proposals.** None is an experiment, a measured
result or an adopted decision. Nothing here authorises a corpus run or a
code change: each proposal needs its own plan, session and experiment
file before anything is measured, and anything that changes output
needs our decision. Our first item, possible plays inside a
confident play of another recording (roadmap, "Reports and command
line"), came before all of them; it is done (ADR 0019, 2026-10-14).

## Provenance

- A GPT Pro model was asked, through a GPT Astra agent (T3 thread
  `12914dec-3ae7-4195-8e14-9ade15d7452e`, "Develop Matching Improvement
  Hypotheses"), for hypotheses to improve matching. Its input described
  `85baa30`: all 106 experiment records, the 18 ADRs, the brief, the
  test-set ledger, the calibration register, the references, the roadmap
  and 16 source files. It had no raw experiment JSON and none of the
  discarded implementations.
- The Astra agent relayed the report on 2026-10-14 with its own review
  (seven points, folded in below as "Review"). Neither ran a corpus
  experiment; the review checked code, dependency sources, arithmetic
  and experiment records.
- This note was written by the Claude agent that integrates them. It adds
  local checks marked "Local check": code reading and arithmetic only, no
  timing and no corpus run.
- The verbatim report, the prompt and the inputs are in
  `work/research/gpt-pro-matching/` (`gpt-pro-response.md`, `prompt.md`,
  `sources.txt`, `matching-code.txt`, `handoff-review.md`). `work/` is not
  committed, so this note is the durable record. All acceptance figures
  below (5%, 15%, 20%, 80%, 95%, 50%, 1.2 GB) are the reviewer's proposed
  choices, not measurements and not project policy.

## Protections that apply to every proposal

- The held-out test set (`ed-rush-optical-essential-mix`), its one
  remaining evaluation, and its NAS alias
  (`EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root) are not used to
  explore or tune anything here. Nor is the recall panel's validation
  half (`docs/panels/recall-seed-2026.json`), which stays unsearched.
- The five test-derived versions (Terminal 2, the Orders remixes, the
  Marky & Bungle Funktion, the Clear Skyz Break remix and 1996 VIP, the
  Black Barrel Beachball) are left out of every panel, including
  proposal 5's. The candidate mixes of experiment 0065 stay untouched.
- The NAS and the NAS peak store are read-only; listening answers and
  verdicts are given by ear; manifests do not change.
- An exact-output proposal must give `regress` identical and
  byte-identical NAS reports (pre-merge and merged lines, detections,
  evidence, float bits, order, plays and file IDs; timings excepted).
- A behaviour-changing proposal keeps the confidence rule (240 hits in 3
  windows, ADR 0008) frozen in its first experiment, gets its own matcher
  identity in reports and baselines, and reports separately: files never
  admitted, admitted files whose hits or windows change, and confident,
  possible-only and wrong confident outcomes. A change to postings
  changes `hash::design()`'s tag (AGENTS.md). Query-only changes need no
  new reference postings, but their results must name the variant.
- AGENTS.md still holds: no SIMD, no `unsafe`, optimise only from
  measurement, a losing experiment's code is deleted.

## What has been measured already

The proposals must not repeat these:

- 0001: a rung is realised by an STFT with window and hop scaled by the
  speed; transforming peak coordinates instead loses about half the
  hashes. 0004: posting precision and delta coding (3.12 bytes a posting
  for the lists alone, a size result, not a codec in the hot loop).
- 0014: triplet hashes scan 25-79 times fewer postings but keep about
  half the evidence under heavy damage (offline estimate).
- 0058: the peak verifier separates true from related detections no
  better than hits. 0076: shared passages played whole make the other
  version confident in 18 of 34 directions. 0091 and 0101: the
  containment rule, measured, not adopted.
- 0078: FFT scratch reuse; plan construction kept local per call.
- 0093: the conservative pre-filter (`may_form_line`): only 1.8% of
  assets are sorted at NAS scale, 0.3% form lines.
- 0096: parts of 4,096 assets; 1,024 and 8,192 were slower. Session 17
  also rejected two-pass grouping, run maxima and counting while taking.
- 0103: the NAS profile (pre-filter 25.6% of CPU samples, grouping 23.2%,
  taking from the lists 10.5%, sorting and lines 5.9%, STFT and peak
  picking 27.6%, second pass 3.3%) and three rejected ways of computing
  slots once (V2, V2b, V3). 0104 and 0105: the saved index read on a
  second thread, which cost record reading its inlining until records
  were parsed from memory. 0106: the index built on threads.

## Proposed measurement protocol

- A fresh baseline with the store revision, compiler, lockfile, build
  profile, worker count, track length range and both playbacks frozen.
  Session 17's baseline used 33,596 records and session 18's 33,723; they
  are not interchangeable.
- Main workload: the 2003 mix (46:10) at NAS scale, store-only, saved
  index current. Report apart: short queries, corpus scale, a stale index
  rebuilt, and rebuilt and saved. A change is not credited with a path it
  does not touch.
- At least three A B B A blocks with independent release binaries: wall,
  user and system CPU, peak memory, and where available the startup,
  first-pass and refinement times. Counters come from separate
  instrumented runs (0103's instrumented run was slower; 0104 showed an
  inlining effect).
- CPU work is not wall time. With startup S, parallel first pass P,
  later work F and overhead O, a stage that is a fraction f of P made r
  times faster saves at most P f; 0103's CPU shares do not give f.
- For a behaviour change, the diagnostic sweep criterion comes before any
  development-mix scan: other rips, damage, short plays, shared passages
  at several lengths, repeated passages and source-cluster leave-outs.
- Real-hit replay captures and fine-grained timing hooks are proposed
  harness additions; no such command exists today.

## 1. Integer phases per query anchor for the pre-filter

**Kind:** an optimisation candidate with exact output. **Status:**
unverified.

- **Measured in session 19 (experiment 0108): stopped, not kept.**
  Implemented as described (a guarded `u16` phase per query point, the
  `f64` slots outside the guard) with 11 predicate tests against
  `densest_clusters`; the same lines, the same detections, 0.017% more
  assets sorted, reports byte-identical. In split profiles of the 2003
  mix at NAS scale (38,105 records; A B B A, once under other load and
  once with less), the pre-filter, its phases and the sort they lead to fell
  from 30.1% to 27.8% of CPU samples on the quieter machine (7.6%; 13.0%
  against the unchanged analysis), below the 20% stop point. The
  counting, its stores and the hit loads cost more than the slot
  arithmetic. The code was deleted; it is kept in `work/s19/lines.rs.p1`.

- **Mechanism.** Today each hit computes `offset = r - q` in f64 (r the
  reference frame, q the anchor's frame in reference coordinates) and
  slot `floor(offset / 4) mod 4096` (`OffsetSlots::may_form_line`,
  `search/lines.rs`). The proposal computes `a = floor(q) mod 16384` once
  per query point and per hit `slot = ((r - a) mod 16384) >> 2`, with
  the same 4-frame buckets, 4,096 slots and neighbour test. Exact
  offsets, sorting, `densest_clusters`, `line_of`, chains, refinement and
  confidence stay as they are. The only new memory is one `u16` per query
  point of the block searched (2Q bytes for Q points), whatever the number
  of hits; nothing is persisted, the index format does not change and
  nothing is indexed again.
- **Why it would stay conservative** (the report's argument: plausible,
  not yet proven for the guarded float case and not yet tested; the
  predicate tests below are required whatever the performance). The
  integer proxy offset is
  `r - floor(q) = (r - q) + frac(q)`; offsets 2 frames apart give proxies
  less than 3 apart, so buckets at most one apart, and folding keeps
  neighbours. Float rounding needs an explicit guard, for example finite
  q with `|q| < 2^32`, checked once per point block, with the current f64
  filter outside it. The filter may pass different non-line assets; the
  invariant is that every asset holding an eligible line reaches the
  unchanged exact finder.
- **How it differs from 0103 V3.** V3 stored a `u16` slot per hit (a
  second array per part, growing with the hits), computed in f64 while
  taking the part; this stores one per query point (2Q bytes) and removes
  the f64 work from the per-hit path.
- **Smallest decisive test.** A pure predicate test against the exact
  cluster finder: negative offsets, 4-frame boundaries, wraparound, equal
  offsets, every order of small cases, large guarded coordinates, values
  next to float boundaries, and the existing case where rounding makes a
  2-frame bucket miss a line. Then a replay of captured grouped hits
  (native, off-speed and key-locked windows, damaged excerpts, weak other
  rips) on one worker and ten: predicate time, assets passed, sorts and
  lines.
- **Proposed criteria.** No eligible line lost; exact output. Advance from
  replay only with at least 20% less filter-plus-resulting-sort time,
  phase construction included: faster arithmetic with no net saving in
  the stage is not enough. Adopt only with at least 5% median wall time
  on the main workload beyond the spread, and no repeatable regression at
  corpus scale or on short queries.
- **Review.** Rust's float-to-integer `as` casts saturate, so the phase
  needs a signed floor and modulo before narrowing; a wrapping
  subtraction and a 14-bit mask give the per-hit modulo. Keep the float
  fallback and the exact finder unchanged. The report's first replay
  outcome ("overall filtering-stage cost unchanged") contradicts its
  20% criterion; the 20% criterion is the one to use.
- **Local check (an attribution estimate, not a ceiling).** 0103's split
  profiles put the pre-filter at 8,610 CPU samples, and at 4,995 (V2) and
  5,035 (V2b), whose slots were computed elsewhere. The difference
  suggests the per-hit slot arithmetic may account for roughly 10% of
  that profile's CPU (about 3,600 of about 33,600 samples, on a build with
  the grouping not inlined). This is an estimate to verify: V2 and V2b
  moved the work and changed the grouping, instruction paths and cache
  behaviour around it, so it is neither a controlled isolation, an upper
  bound on the integer path, nor a wall-time prediction. The counting
  remains in any case. The report's 25.6% was a deliberately loose bound
  that assumed the whole filter vanished. V3, which moved the slot work
  instead of removing it, saved 1.3% of CPU and no wall time.

## 2. FFT lengths of the turntable rungs

**Kind:** a diagnostic, then a behaviour-changing hypothesis. **Status:**
unverified.

- **Measured in session 19 (experiment 0109): the diagnostic; no length
  changed.** Plan construction is 0.04% of the analysis (1,862 plans in
  0.052 s in one `identify`), so no plan cache was tried. Weighted by the
  frames each rung analyses in the 2003 mix's `identify` at NAS scale,
  one thread: nearest-even makes the first pass's turntable analysis
  21.8% faster (18.4% with the unchanged key-locked rungs counted), the
  factor-aware rule within one sample 25.5% (21.6%), within two samples
  36.5% (30.8%), and the fastest length measured within two samples, a
  ceiling, 44.0% (37.2%). The largest prime factor predicts time poorly
  (949 = 13 x 73 is slower than 948; 1114 = 2 x 557 is twice as fast as
  1113 = 3 x 7 x 53). Builds with the factor-aware or fastest lengths in
  the first pass, for timing only (their reports differ): 6.9% and 8.7%
  less user CPU and 5.4% and 7.6% less median wall time with `--playback
  both`, 12.1% and 14.4% less CPU and 9.2% and 11.1% less wall time with
  turntable playback (timed under other load). Changing a length
  needs the retrieval study below and our decision.

- **Today.** A turntable rung analyses the query with window
  `round(1024 / s)` and hop `round(128 / s)` (`points_at_speed_in`,
  `speed.rs`; 41 rungs, 0.92 to 1.08 in steps of 0.004, experiment 0001)
  and corrects peak coordinates for the rounding. Key-locked rungs and
  reference extraction use 1024.
- **The report's proposal.** Nearest-even windows,
  `2 round(1024 / s / 2)`, for first-pass turntable analysis only: 17 of
  the 41 lengths are odd, each changes by one sample, at most 0.105%.
  RealFFT 3.5.0 runs an even length N as a complex FFT of N/2 with pre-
  and postprocessing and an odd N as a complex FFT of N. The report gives
  no other reason for nearest-even, and itself cautions that
  factorisation matters and an even length need not be faster.
- **Evidence.** 0078 measured three analyses at 0.016849 s for 1024,
  0.369398 s for 985 and 0.224277 s for 1113 (reused scratch, timed under other
  load): a length-specific cost worth measuring, not a clean estimate.
- **Local check (factorisation).** RustFFT 6.4.1 has kernels for sizes up
  to 32 (prime factors up to 31) and combines them by mixed radix; a
  larger prime factor p goes through Rader's algorithm, or Bluestein's
  (an inner FFT of at least 2p - 1) when p - 1 has a prime factor above
  23. 985 = 5 x 197 and 1113 = 3 x 7 x 53, consistent with 0078. By the
  largest prime factor of the complex length each rung runs:
  - today 25 rungs have one above 31 and 16 above 100 (1103 at 0.928,
    1049 at 0.976, 977 at 1.048);
  - nearest-even leaves 24 and 13 (largest 557). It gives 9 of the 17
    lengths it changes a larger largest prime factor (1113 to 1114 =
    2 x 557, 973 to 974 = 2 x 487, 959 to 958 = 2 x 479), which is not
    the same as measured slower, and leaves the 10 even lengths whose
    half holds a large prime (1094: 547, 1028: 2 x 257, 1004: 2 x 251);
  - choosing, within one sample of `round(1024 / s)` (the same 0.105%
    bound), the length whose complex FFT has the smallest largest prime
    factor leaves 12 rungs above 31 and none above 100 (largest 83);
    within two samples (0.21%), 6 and none (largest 71).

  The factor-aware rule is this note's extension, not the report's
  recommendation. The arithmetic ranks candidates for timing and
  establishes no winner: whether time follows the largest prime factor
  is a prediction to measure. The diagnostic should time three arms
  (today, nearest-even, a factor-aware rule), not nearest-even alone.
- **Smallest decisive test.** Enumerate the lengths in the Rust build and
  record the pinned versions and features. On fixed decoded input, time
  plan and window construction, repeated `process_with_scratch`, and
  log-power and peak picking separately, over the whole weighted ladder,
  with per-rung tails. If plan construction is a material share, a plan
  cache across calls (0078 kept plans local) is an exact-output change to
  try before any change of length; its bit-identity must be checked.
  Only then retrieval: on-rung and half-step speeds, several sample-start
  phases, other rips, blends at equal level, combined damage, content
  near Nyquist and short plays, tracing admission and first-pass lines
  before confidence. Unchanged rungs and key-lock-only runs are controls.
- **Contract.** Not bit-identical: the window, spectral grid, peaks and
  pairs change, and with them lines, chains, fitted speed and evidence.
  `points_at_speed_in` also serves refinement, so a first-pass-only
  variant needs its own call path; changing the shared function tests a
  different hypothesis. Reference postings are unchanged; results and
  baselines must name the variant.
- **Proposed criteria.** Stop before retrieval if the weighted first-pass
  analysis is not at least 20% faster, unaffected rungs included. Adopt
  only with at least 5% wall time, no new wrong confident play, no lost
  right confident or possible file on the development panels, lost
  shared-material matches reported apart, and a boundary tolerance on the
  generated mixes fixed in advance. STFT and peak picking for both
  ladders were 27.6% of 0103's CPU; this touches part of it.

## 3. Part boundaries of the posting lists, computed once

**Kind:** an optimisation candidate with exact output. **Status:**
unverified.

- **Mechanism.** `WindowHits::take_part` binary-searches what is left of
  each of a window's lists for every part of 4,096 assets
  (`partition_point`). The boundaries depend on the index only. A
  directory of each hash's posting positions at the internal part
  boundaries would replace the searches with lookups, with the same
  slices, hits, order and grouping.
- **Memory.** At 33,723 assets, 9 parts, 8 internal boundaries:
  2^21 hashes x 8 x 4 bytes = 64 MiB, about 0.071 bytes a posting at
  about 949 million postings (the saved index is 3.81 GB at 4 bytes a
  posting); at 65,536 assets, 15 boundaries, 120 MiB. Positions, not byte
  offsets; it does not lift the 65,536-asset limit.
- **Review.** `take_part`'s 10.5% includes copying hits, not only the
  searches. `WindowHits` holds slices, so the hash (or directory entry)
  must travel with each list, and that metadata counts. Indexes restricted
  by leave-outs or samples must rebuild or transform the directory; the
  full-library skip of the fullest lists keeps its meaning.
- **Smallest decisive test.** First split `take_part` into searching and
  copying. Then replay captured `(hash, anchor)` requests against the
  resident posting array (not copied lists, which change locality):
  short, long and empty lists, repeated hashes, the short last part,
  restricted indexes. Every slice compared, then lines and detections.
  Directory construction timed apart; no file format change unless the
  lookup pays.
- **Proposed criteria.** Stop if the optimistic wall-time share of the
  searches is under 5%; otherwise adopt only with at least 5% after
  building or loading the directory. Persisting it means a format
  change, digest coverage and structural validation (ADR 0011, 0014).
  In 0103's profile `take_part` as a whole was 10.5% of CPU samples;
  the searches are only part of it.

## 4. A companion byte to scout candidates, then the original matcher

**Kind:** a larger algorithmic hypothesis; discovery changes. **Status:**
unverified; the riskiest of the five.

- **Mechanism.** For each reference anchor's two pairs, store a one-byte
  selector derived from the other pair's deltas, in a parallel byte
  vector. A query anchor looks at its first three eligible targets; a
  posting becomes a hit only if its selector fits one of the other
  targets, with an explicit and measured wildcard path for missing
  companions. Files the scout admits are then searched by today's pair
  matcher over the whole query and every rung, with the original
  full-library cutoff of the fullest lists and the original asset IDs.
- **Evidence.** 0014 found triplets very selective but fragile under
  blends, clipping, echo and noise. It did not measure a byte selector or
  a two-stage scout.
- **Costs.** About 0.95 GB more memory (one byte a posting, a quarter of
  the postings), on top of a 4.3 GB peak (roadmap, "Search memory at
  scale"). Selectors are derived data from the peaks with their own
  provenance and validation; no decoding again. Without a point cache the
  query is analysed twice, so even a perfect scout saves less than
  grouping minus analysis, not 0014's 25-79 times.
- **Review.** Treat the 80% reduction in grouped hits, the 15% wall-time criterion,
  the 1.2 GB budget and two-anchor admission as proposed choices. Needs
  explicit cases for missing companions, changes of order or ties in
  target selection, selectors correlated with the primary hash (collision
  rates measured conditional on it, not assumed 1/256) and weak
  overlapping files. Replay cannot repair a file the scout missed, and a
  second analysis or point cache is measured, not omitted.
- **Smallest decisive test.** Shadow mode on development excerpts, the
  scout suppressing nothing, weak other rips and combined damage first:
  is every file today's matcher reports confident or possible admitted,
  how many more, wildcard and pass rates, the reduction in grouped hits,
  and the replay's cost. A fallback only when nothing was found is not
  enough: one strong file says nothing about a weak rip or an
  overlapping track.
- **Proposed criteria.** Advance only with complete admission on the
  diagnostic panel and at least 80% fewer grouped hits; adopt only with
  at least 15% end to end, the same required output and at most 1.2 GB
  more memory. Only if grouping still dominates after proposals 1 and 3.

## 5. Identifiability before another confidence statistic

**Kind:** a diagnostic; no change to the product. **Status:**
unverified.

- **Question.** Does a query hold distinctive evidence that today's
  representation does not use, or only material several versions share?
  An unchanged passage shared by two versions cannot say which supplied
  it, whatever the fingerprint. 0076 and 0058 show the practical side;
  the roadmap's "A confidence statistic for shared material" and the
  containment rule (0091, 0101) are where this would feed in.
- **Smallest decisive test.** A small development panel that holds a
  shared passage fixed and varies distinctive context: recording
  families with the judgements by ear (`docs/pair-verdicts.txt`) and
  constructed references sharing an exactly copied passage with
  different surroundings. The passage alone, then with 5, 10 and 20 s of
  context; a weak rip, a blend masking the context, the played cluster
  removed. Instrument where aligned evidence falls: in the shared region,
  outside it, across repeats. Outcomes: no distinguishing information in
  the observation (abstain); information in the audio that does not
  survive into peaks and pairs (a representation question); information
  that survives but the evidence ignores (only this one justifies a new
  statistic).
- **Review.** A diagnostic separation, not a deployable rule: diagnosing
  ambiguity offline does not give the product a way to recognise it when
  the played file is absent. The proposed feasibility criteria (keep at
  least 95% of true confident examples, expose a reason to reject at
  least half of the constructed wrong ones) need an exactly defined
  oracle and denominator first, and an annotation-assisted oracle is not
  a production result. Split by recording family; build any reuse map
  only from files present after a leave-out.
- **Product.** Abstaining or flagging is a change of output and our
  decision (ADR 0017). Our first item hides possible
  plays inside a confident play; it does not address a confident play of
  a related version.

## Order and stop points (the report's)

1. A frozen baseline and replay captures; proposal 1 through its tests,
   replay and the full comparison.
2. Proposal 2's computational half alongside; retrieval only if the
   weighted ladder saving is large.
3. Proposal 3's search share from the same profiling, before any
   persistence. Exact winners combined and measured again, not assumed
   additive.
4. Proposal 4 only if grouping still dominates and weak-file admission
   passes.
5. Proposal 5 before any change to confidence or new evidence for it.
   Any behaviour-changing candidate is frozen before validation on
   separate families; the last test-set evaluation is not spent on this.

**Deferred by the report:** general posting compression (0004's figure
is size, and loading the saved index already overlaps decoding),
sharding past 65,536 assets presented as less posting work (it needs the
full-library cutoff and saves none), another sort, another per-hit slot
array, peak transforms on the native grid (0001), fewer windows in the
confidence rule and summed segments (roadmap, "A minimum span instead of
3 windows" and "Evidence summed across a play").

## Integration decisions

- A note of its own, linked from the roadmap ("Where search time goes",
  "Longer hashes", "A confidence statistic for shared material") and
  from `docs/status.md`; no experiment numbers, no ADR.
- Our first item stays first.
- Two local checks change the framing without rejecting anything:
  - proposal 1: 0103's split profiles suggest the slot arithmetic may
    account for roughly 10% of CPU, an attribution estimate to verify,
    not a hard ceiling or a wall-time prediction; the counting remains;
  - proposal 2: nearest-even gives 9 of the 17 lengths it changes a
    larger largest prime factor (not measured as slower), so a
    factor-aware rule within the same bound is added as an independently
    proposed third arm of the diagnostic, with a plan cache as an
    exact-output question inside it.
- Protections added beyond the report: the five test-derived versions,
  the test set's NAS alias, the candidate mixes of 0065, the read-only
  NAS store, and the AGENTS.md limits.
