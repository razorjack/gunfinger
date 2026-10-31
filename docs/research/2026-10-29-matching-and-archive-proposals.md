# Matching and archive proposals (2026-10-29)

**Status: six unmeasured proposals and a proposed evaluation method.**
This note records the ideas and observations from a review of the roadmap,
experiment history through 0242, relevant ADRs, calibration, research
notes and matching code. No experiment, timing or corpus search was run
for the review. No decision, confidence rule or product behaviour changes.
Each proposal needs a registered experiment before implementation is
judged; adoption that changes an existing decision needs a new ADR.

## Priorities and observations

The first experiments worth trying are stereo power fingerprints (§1) and
filtering before hit grouping (§2): each addresses a measured problem
with a contained change. Correcting the second pass's initial alignment
(§3) is another contained accuracy experiment. Recovering quiet tracks
using an identified foreground track (§4) has the largest potential
recall benefit, but also more uncertainty. A separated harmonic view
(§5) is a higher-risk precision experiment. Matching unnamed passages
across mixes (§6) is a possible new capability for an archive.

These priorities follow from the measurements, not from predicted gains:

- More evidence has repeatedly helped related versions as well as true
  plays. Tolerant pairs (0151), wider query fan-outs (0208) and the peak
  verifier (0056, 0058) provide no safe general replacement for the
  confidence rule. Raising a threshold also leaves the underlying
  ambiguity unresolved and costs weak true plays.
- Under combined damage, 88.3% of lost reference pairs have a missing
  endpoint, 4.5% a displaced target and 6.3% a changed quantised value
  (0207). A missing endpoint means that the query's peak representation
  lost it, not necessarily that its signal is absent from the audio.
- Grouping still costs 40.2% of first-pass CPU at 32 workers and 45.9%
  at 16 in the latest stage measurement (0214, on Linux). Narrower hits,
  grouping directly from lists and several scatter layouts lost their
  experiments (0171, 0172, 0200-0202). A new speed proposal should avoid
  work that those variants still performed.
- The second pass counts all posting lists, but its speed and span still
  depend on the first pass's filtered evidence (0223, 0242). Restoring
  the lists does not make the final measurement independent of discovery.
- Near-identical versions judged to carry the same title are already
  credited under ADR 0022. Precision experiments must retain that
  distinction and must not count such credits as new identification
  gains or errors.
- When the heard passage is identical in two versions, the audio cannot
  establish which version supplied it. Different processing may expose
  obscured evidence; it cannot create distinguishing evidence that was
  never present.

The proposals use local signal processing and the existing file model.
They require no neural model, network service, tag inference, report
editing or stored model of tracks above files (brief, ADR 0017).

## 1. Combine stereo channels after measuring spectral power

**Evidence.** One inverted channel lost all 80 indexed excerpts under the
current downmix (0194). Choosing one channel lost clean matches; searching
the downmix and both channels kept the clean controls and recovered most
faulted excerpts, at three searches per query (0195). Faults in the
reference also cost recall (0197), and faults over half a generated mix
lost whole plays (0198). The measured alternatives were waveform channel
choices, not combined channel powers.

**Proposal.** Compute an STFT for each channel, then pick peaks from
their combined power, with a fixed normalization:

```text
P(f, t) = |X_L(f, t)|^2 + |X_R(f, t)|^2
```

For fixed decoded samples, reversing either channel's polarity leaves
this power unchanged. Combining waveforms first includes an interference
term that can cancel centred sound or comb-filter delayed channels.
Combining powers removes that term. A channel delay still shifts content
within the finite analysis windows, so delay invariance is not claimed.
Lossy encoding after a fault may also change the decoded signal.

This gives one peak stream and one search, with an additional channel's
STFT work. It does not require a fault detector or a choice of the clean
channel. It does not remove noise in a noisy channel.

**First experiment.** Compare against the current downmix on the stereo
fault panel, clean stereo, other rips and shared-material cases. First
measure query-only compatibility with today's reference peaks. Separately
build a small variant reference store and test faults on either side,
including polarity changes partway through a mix. Report recall, wrong
confident answers, peak and posting density, and full search cost.

**What decides it.** Does fault recovery survive clean and cross-rip
controls without adding wrong answers? Polarity invariance of the formula
does not establish compatibility with today's mono fingerprints. A
replacement front end changes the profile and needs new reference peaks;
query processing cannot restore information already cancelled in an old
reference record. An additional search view also needs its combined
false-answer rate measured, not just each view's rate alone.

**Measured (session 61, experiments 0245-0247).** Query-only, against
today's records: every inverted channel and delay up to 5 ms recovered,
mono queries' counts unchanged, noise in one channel worse, 12% more
search time at NAS scale. With a variant store: faults in the rip recovered too, but
mono queries lose 2-4 of 80 in 6 of 38 conditions, so the registered bar
fails. Not adopted; see
[the decision note](2026-10-30-stereo-power-decision.md).

**Measured (session 62, experiments 0248-0250).** The query side alone
at NAS scale under one frozen rule: fault gains, the panels, the mixes
and the scans hold with no new wrong file, but `recall` loses 9 of 522
clean excerpts (one wide source) and 58 of 721 brief plays (51 with the
side within 15 dB of the mid), so its bar
fails; no ADR. About 0.4% of the NAS rips have an inverted channel
(0248). See the decision note's "Session 62".

**Measured (session 67, experiments 0262-0263).** Not this proposal but
the record side it cannot reach: a check at indexing that repairs only
records of files whose channels are inverted (the downmix with one
channel's polarity reversed). It flags about 0.08% of the NAS store
(5 of 6,766 files), recovers the four plainly inverted recordings in
full (0 to 48 of 48 excerpts, 0 to 4 of 4 mix plays) with no new wrong
answer, and leaves every other record byte-identical; a regression
still moves sub-threshold candidates and one fitted speed through the
lists set aside, so its registered bar fails on that item. No ADR; see the decision note's "Session 67".

## 2. Reject impossible alignments before scattering hits

**Evidence.** `WindowHits::lines` in `search/lines.rs` takes a part's
hits, groups them by asset, then runs `OffsetSlots::may_form_line` on
each asset. Grouping is the largest measured stage (0214). An earlier
NAS trace passed only 2.2% of the assets with at least three hits to
sorting (the low-bandwidth search note, session 36); that percentage is
not a fresh measurement of today's code.

**Proposal.** Before grouping, build a small conservative counting sketch
over asset and folded offset. Scatter hits only for assets that might
contain a line. Keep today's precise filter and line construction for
those assets. Several compact counter or bitset layouts are candidates;
no layout or size is chosen here.

The required property is one-sided: folding or collisions may admit extra
assets but must never exclude an asset with a real line. The existing
filter already uses folded neighbouring offset buckets with this
property. Extending it across assets could avoid most scatters, whereas
the failed narrower-hit and direct-grouping variants still scattered
essentially every hit.

**First experiment.** Replay representative real hit buffers through a
small set of sketch sizes and layouts. Prove the conservative rejection
condition, including neighbouring buckets across wraparound, saturation
and the existing fallback when phases cannot be represented exactly.
Compare lines and their ordering against the current path. Then measure
full release searches on macOS and Linux, at corpus and NAS scale, with
different worker counts. Include clearing and counter updates in the
cost, and skewed or dense buffers that make the sketch pass most assets.

**What decides it.** A substantial end-to-end saving with identical
reports, including evidence and float bits, would justify adoption.
Rejecting most assets is insufficient if maintaining the sketch costs
as much as scattering them. The index and its 4-byte postings would stay
unchanged; report scratch bytes per worker and peak memory. Any persisted
alternative would need a separate bytes-per-posting justification.

*(Session 60, measured and rejected.)* Dense tables of folded pairs per
asset, at 64, 256 and 1,024 slots, kept every line in the replays (and,
at 1,024 slots, every report compared), but passed 97%, 70% and 21% of
the NAS-scale hits; the largest made the search at NAS scale 31-32%
slower at 32 workers and 12-13% at 16, on the Linux machine (the Mac
was not measured; experiments 0243 and 0244). Hashed layouts were not tried: by a uniform estimate (not
measured), a table shared by a part's assets needs many more cells than
the part's 45,000 hits to reject an asset of 44 hits.

## 3. Correct the second pass's initial speed estimate

**Evidence.** In 0242, a first-pass line losing one hit changed the kept
rung, moving the chain's fitted speed from +1.147% to +0.933% and its
second-pass count from 455 to 503. Both answers were right. In 0226,
widening the ladder moved Siren Bass's fitted speed from -8.09% to
-8.18%, reducing its second-pass count from 305 to 214 and losing a
confident answer. These are measured cases, not an estimate of frequency.

`chain_speed` fits through line centres when there is enough span; short
chains instead use a hit-weighted mean of rung speeds. `refine` uses
that estimate to analyse the candidate again. The optional
`speed_per_stretch` fits local stretches afterward (0032), but is not a
general search for a better initial alignment.

**Proposal.** Use the ladder's alignment as an initial estimate. Fit speed
and offset from nearby individual correspondences, reanalyse, and repeat
a small fixed number of times. Keep the fit near the original passage
and constrain continuity, so it cannot jump to another repeated break.
The search remains restricted to one candidate; it adds no global rungs.

**First experiment.** Measure the available gain with a fine local speed
search on weak true candidates and shared-material negatives, without
changing confidence. Then compare a bounded iterative fit with that
local-search reference. Perturb the lists set aside and library contents
as in 0242, and test between-rung speeds, other rips and short chains.
Measure how often a better fit changes an answer, not only its hit count.

**What decides it.** Recovery of weak true plays and steadier measurements
must survive the negative controls. Maximizing over several alignments
can increase wrong-version scores too. Register the fit budget and
measure the selection effect; fitting on one portion of the evidence
and testing on another is a diagnostic worth comparing. It is not safe
to assume today's threshold has the same meaning after this selection.
This proposal does not itself solve the three-window condition or join
short segments across a pitch ride.

**Measured (session 63, experiments 0251-0253).** The local search (41
speeds within ±0.4%, offset refitted), a split that chooses on alternate
windows and counts on the others, and a bounded fit (two rounds of the
hits' drift), each opt-in (`--second-pass-fit`). Few answers change on
30 s excerpts (1-3 per fit, against other rips), where the chain's fit is
within 0.003%; on 15 s plays (the mean of the rungs, 0.049% off) each fit
gains 18-27 confident answers in `recall` and loses 1-3. The grid lifts
wrong versions as well; the split suggests that most of the gain on short
chains is not selection, but a related
version sharing the played passage gains with it. The bounded fit fails
its registered bar on lost answers (4) and on the margin at 262 tracks;
it costs 1.6% more search time, and under 0242's perturbation it holds
Echos at 602 hits where the default moves from 503 to 455. Not adopted;
see
[the decision note](2026-10-30-second-pass-fit-decision.md).

## 4. Recover a quiet track using an identified foreground track

**Evidence.** Lost peak endpoints dominate combined damage (0207), and
wider query pairing helped related versions more than true plays (0208).
Quiet double-drop partners were lost mainly by the search; brief partners
were more often found as possible and then omitted by ADR 0019 (0184).
Those are different failures and need separate scores.

**Proposal.** Once track A is confidently identified, estimate its local
time alignment and slowly varying gains across frequency bands.
Attenuate the spectrogram regions that A explains, then search the
remaining signal for a quieter track B. A spectral mask or conservative
power residual is a first candidate; exact waveform subtraction would
require much tighter timing and transfer matching.

Run this within covered passages too: a confident foreground track can
cover the entire quieter play, so searching only the reported gaps would
miss the intended cases. An audio match to A supplies its contribution;
neither its tags nor a guessed musical arrangement enters the model.

Sub-band gain fitting and known-source time-warp estimation have research
precedent (Kim, Yang & Nam 2021, "Transition Analysis Methods"; André et
al. 2024, §§III-IV). Those studies do not establish that a residual
improves fingerprint retrieval. The earlier version-verification note
also considered signal models and fixed templates; this proposal uses
them to discover a buried second track, rather than to reject a version
from missing expected content.

**First experiment.** Generate double drops with known truth and index a
different rip of the foreground track. Supply the true time map first
to test whether masking has useful potential, while estimating gains
from the mixture. Then use estimated timing. Compare against denser
query peaks without masking. Include A alone, a held-out B, related
versions, shared breaks, bass swaps and broadcast processing. Separate
candidate discovery, confirmation and printed-tracklist recall.

**What decides it.** The residual must recover actual quiet partners,
including through other rips, without creating confident absent ones.
Confirmation needs evidence in the original mixture, not only artifacts
in a processed signal. That may need a new evidence rule, which this
proposal does not supply. Reference audio is needed initially, so this
would not work with `--store-only`; a compact stored representation is
a later question, with its own size measurement and ADR. Power
additivity is approximate, and alignment errors can leave A in the
residual or remove B. These are central failure cases for the experiment.

**Measured, the oracle only (session 65, experiments 0255-0258).** A
harness option (`mixes --kind drops --residual`) takes every play found
confidently in generated double drops as A, masks its span with a render
of A placed by the generator's time map (per-band gains fitted from the
mixture over blocks of about 1 s, each bin losing twice A's estimated
power), turns the masked spectrum back into audio and searches it with
the unchanged search. With the played file itself as A's reference (62
and 60 of the 64 cases at 262 tracks), the residual confirmed (possible or confident) 62 of 64 quiet and brief
partners on the development seeds (plain excerpt 41, denser query peaks
39; confident 47 against 8) and all 64 on two test seeds (plain 36;
confident 48 against 12). At 262 tracks no masked search gave a
confident answer that was not playing, also with the partners'
recordings left out of the index. With another rip of A as the
reference, the case a real mix presents, the gain shrinks: at NAS scale,
where the NAS holds another rip for most As, the mask left A confident
in about a quarter of the cases (a third with NAS uploads), and the
quiet and brief partners confirmed were 50 of 64 (plain 40), confident
32 (plain 8), on the development seeds and 55 (plain 35), confident 34
(plain 12), on the test seeds. On a test seed the residual also made one
confident answer that was not playing: with a record masked, a known
related version (the Cool Rock pair) of the record playing under it
reached 265 hits: taking A out can lift a relative of the record under
A as well as that record. Of the partners the residual newly confirmed,
6 of 21 had a detection in the plain excerpt on the development seeds,
16 of 28 on the test seeds, and 5 of 12 and 14 of 20 at NAS scale (the
same mixes, so the counts overlap): a rule that needs a detection in
the original mixture would keep a third to two thirds of them. The idea is worth a next step, in this order: why another
rip leaves A behind (a different master or lossy upload, the render's
seek against the stored peaks' timeline, the rip's alignment), then
estimated timing, then an evidence rule tested against real
shared-material relations. Not measured: broadcast processing,
shared-break panels, printed-tracklist recall (a partner under a
confident play stays marked by ADR 0019 whatever its level), and the
cost in a product (here every confident play's span is searched three
or four more times).

## 5. Measure evidence outside the dominant drum texture

**Evidence.** Related versions gain alongside true plays under tolerant
pairs, wider fan-out and aggregate peak verification (0151, 0208, 0056,
0058). Band-energy version verification also lost true plays: its first
version suffered alignment errors, and its second rejected blends at
the recalibrated threshold (0125-0127, 0133-0135). Another experiment
needs a reason to measure different information.

**Proposal.** Add a candidate-only harmonic/percussive view. Median
filtering across time and frequency separates sustained spectral
structure from transients without a neural model (FitzGerald 2010,
§2). Extract features from the separated reference and query, and ask
whether a candidate found through drums also matches sustained material
at the same alignment. Bass, pads and other sustained sounds are
possible additional evidence, not automatically distinctive evidence.

Start with a diagnostic and no change to the main index. This changes
the signal presented to the feature extractor, unlike the earlier
verifier's correlations of energies in fixed frequency bands. It also
differs from spreading the ordinary peaks more evenly across bands
(0034).

**First experiment.** Compare true damaged rips with wrong related
versions at similar existing hit counts. Use the same mixing transforms
for positive and negative cases. Include bass kills, strong blends,
little sustained content, shared riffs and different rips. Compare
against simply increasing query peak density, so any gain can be
attributed to separation. Measure retained true plays at a fixed
wrong-answer rate, split by recording family.

**What decides it.** Does the extra view distinguish cases that current
evidence cannot? Shared riffs can still fool it, and a DJ's bass kill
must not become a contradiction merely because the harmonic view finds
nothing. Combining views is not automatically independent evidence.
Identical audible passages remain ambiguous. Adoption would need an
acceptance rule measured on fresh families and a decision on reference
audio or supplementary stored features; no blanket rejection rule is
proposed here.

**Measured (session 66, experiments 0259-0261).** As a candidate-only
diagnostic on the shared-material panels' development halves and seven
judged pairs at NAS scale, the harmonic view separated no better than
denser peaks without separation at equal hits; below 240 hits the blend
partner's cases scored lowest (masking by the other record, an
inference), while the played file's scored as well as under denser peaks.
The bass kill was not read as a contradiction. Not proposed; the
decision note is
[2026-10-31-harmonic-view-decision.md](2026-10-31-harmonic-view-decision.md).

## 6. Find recurring unidentified passages across a batch of mixes

**Potential value.** An archive could report a direct correspondence:

```text
Unidentified passage at 23:14 also matches audio at 51:06 in another mix.
```

This is useful even when no clean reference recording is indexed.
Different mixes can expose different parts of the material, and one
occurrence may be much clearer than another. This benefit has not been
measured on the archive.

**Proposal.** Match passages across a batch of mixes and report the files,
times and directly supported spans. Keep the operation within one run;
it needs no editable identity model, saved decisions, guessed titles or
library-cleaning workflow. The initial result is an audio correspondence,
not an assertion that two whole tracks have the same identity.

**First experiment.** Generate independent occurrences of held-out
recordings with different partners, cue points, EQ and speeds. Search
the mixes against each other and score passage retrieval against the
known construction. Include non-overlapping excerpts of the same source,
unrelated records sharing breaks, related versions and repeated radio
inserts. Score a shared snippet separately from a whole-track claim;
the latter is not an output of this prototype.

Two details matter. First, opposite deck settings can exceed the current
ladder's relative reach: 1.08 / 0.92 is about 1.174, or +17.4%, before
differences between rips. A prototype needs an explicit relative-speed
range and cost measurement. Second, direct passage matches must not
propagate titles through chains of partial overlap. Mixed-CD tracks and
uploads holding parts of other records show why such propagation would
be unsafe (0063, 0185's later listening verdict, 0241).

**What decides it.** Can the batch find useful repeated passages under
different mixes while avoiding misleading whole-track links? The
existing gap self-match is useful precedent for working without an
indexed title, but shared material already makes it merge records
(0163). A graph of passage matches cannot treat every connected
component as one track. Cost, temporary indexing and long-file handling
also need measurement; the present reference-length limit is not a
solution for indexing whole mixes as ordinary tracks.

**Session 69's pilot (experiments 0267-0268).** `gunfinger-eval
passages` draws batches of 12 generated mixes in which 16 recordings
left out of the index occur three times each, with different partners,
cue points, bass swaps and speeds (occurrences at 1.04-1.08 and
0.92-0.96, so opposite deck settings), with three controls:
non-overlapping excerpts of one source, the shared-material panel's
development pairs, and a 12 s insert. Each mix's unidentified spans (30
s or more without a confident detection at 262 tracks), or the whole
mix, are indexed in memory as pieces of at most 600 s and searched
against each other; a link is a confident detection under the frozen
rule, reported pairwise with its spans, and only the two directions of
one match are joined. Over ten batches nothing linked unrelated
records, and the links between records sharing a passage (Fire's remix
and Pyro's two tunes, nearly every batch) reach at most 0.2 s past that
passage from the unidentified spans and 12.3 s from whole mixes. The
insert, at 12 s, is shorter than the rule's windows and never made a
link, so it could not test whether an insert bridges two records. Recall is the weak side: from the
unidentified spans, 192 of 312 recurring passages are found at ±18%
(50% on the two registered batches; 80% was registered), because a
held-out record's time in a gap is short (13% of the plays have none:
28 lie inside their neighbours' crossfades, and 49 leave at most 21 s
between them, under the 30 s minimum); whole mixes give 266 of 312 (85%).
The product's ladder finds none of the 112 beyond 12% relative speed;
±18% (91 rungs) costs 2.33 times its thread seconds and finds 62 and 98
of them. Links within one recording but at another place in it (10-178
s apart in the track; by inference its own repeated material) are the
wrong links that remain:
15 and 37 at ±18%, 9 and 11 of them between excerpts with no source
time in common. A batch takes 75-82 s and 2.3-2.6 GiB on the Linux
machine; how the pairwise search grows with an archive's size is not
measured. By the registered bar the pilot is not worth a design note
yet. A next step would search whole mixes (or unidentified spans with
a shorter minimum, widened over the crossfades around them), report a correspondence only with
its passage's spans, and measure the cost at archive scale and on real
mixes, where shared material is more common than in these batches.

## Automated counterexample search

The main opportunity for agentic work is a constrained search for
failures, not unattended listening judgments. For each candidate, search
bounded, realistic combinations of pitch motion, EQ, blend level,
channel faults and placement. Find a small transformation that loses a
true answer or creates a wrong one, then minimize it while preserving
the failure. Save the rendering recipe, seed, source identities,
expected answer and variant settings under `work/`.

Synthetic construction supplies the truth. Fresh seeds of the same
recording family are not independent validation: the marked-play
experiments obtained many examples of one shared-material relationship
(0215-0218). Freeze a candidate before testing fresh families. Real
positive controls remain necessary because statistics that separated
clean renders rejected real true plays (0116, 0118).

**Session 68's pilot (experiments 0264-0266).** `gunfinger-eval
counterexamples` draws small generated mixes (a held-out lead-in, the
target, a partner), adds transformations within fixed bounds (speed
±6%, a 1-3% ride, a bass kill and a blend of 10-60 s, brief 42's
channel faults), and minimizes each failure greedily; `--replay` renders
and searches a saved recipe again. Of 2,000 samples at 262 tracks, 188
failed (17 classes); of the same first 500 at NAS scale, 72 (11
classes). Every play lost or long gap needs a channel fault or a blend,
alone or with up to three more transformations; speed, bass kills and
rides alone never failed. The wrong answers (NAS scale only) need no
transformation: three related-version families already in the roadmap.
All 12 replayed recipes reproduced their failure. Each class is a known
failure or two in one play; the pairs no roadmap entry names are rides
of 0.5-3%, harmless alone under ADR 0021, whose later side a blend or a
fault loses. The blend is harsher than the generator's crossfade (no
fades, up to the whole target). Cost: 5.3 s and 157 CPU s per failure
at 262 tracks (16:30 for the run), 14.3 s and 389 CPU s at NAS scale
(17:10 for 500 samples). Worth running beside a later session as a
paired check of a variant (the same seed draws the same samples, so
classes gained or lost compare directly), not as a search for new
classes with these axes; that needs axes the generator lacks (other
EQ, beatmatched or quiet partners, key-locked plays, the fault's
placement), and real recordings stay the positive controls.

## Evaluation and adoption boundaries

- Register each experiment's hypotheses, development data, frozen
  validation data and criteria before its runs. These proposals set no
  numeric adoption thresholds and promise no gain. No experiment number
  is allocated by this note.
- Do not use the held-out test mix, its NAS alias, reserved panel halves
  or unassigned real mixes to explore or tune proposals. A frozen
  validation run needs its agreed role and protocol first. Agents never
  answer listening questions, edit manifests or add title judgments.
- Run corpus experiments with `--release`. Keep generated audio,
  temporary stores, hit buffers and reports in `work/`. Use distinct
  profiles and variant identities where the evidence changes, and keep
  the existing reference store intact during pilots.
- For §2, require identical search output and the normal regressions.
  For behaviour changes, report discovery, hits, windows, confidence,
  wrong answers and output effects separately. More candidates or more
  hits alone are not an identification improvement.
- Test shared-material cases, played-recording leave-outs, other-rip
  positives and damaged true plays together. Measure the result after
  any union or selection across views, speeds or masks. One frozen
  acceptance rule must eventually apply to all inputs; extra attempts
  do not inherit the old rule's measured false-answer rate for free.
- Report elapsed time, CPU and peak memory, including preprocessing,
  decoding and temporary representations. Any stored-field change needs
  its size and fidelity justification. A front-end or posting change
  must follow the profile and `hash::design()` invalidation rules.
- Product output, decoding, reference representation or confidence
  changes require the relevant ADR. The initial aim is a credible
  measured result, including a negative result, before that decision.

## Related notes and sources

- [Roadmap](../roadmap.md), especially matching, stereo faults and the
  printed tracklist.
- [Calibration register](../calibration.md).
- [Matching proposals of 2026-10-14](2026-10-14-matching-proposals.md).
- [Version verification](2026-10-16-version-verification.md).
- [Low-bandwidth search and subsequent measurements](2026-10-21-low-bandwidth-search.md).
- [Ladder-edge decision note](2026-10-25-ladder-edge-decision.md).
- [References](../references.md), including FitzGerald 2010, Kim, Yang &
  Nam 2021 and André et al. 2024. Their methods motivate experiments;
  their results are not predictions for Gunfinger.
