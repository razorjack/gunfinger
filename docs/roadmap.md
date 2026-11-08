# Roadmap

Work deliberately left out of the proof of concept, as open items with
their state and next step. Items marked *(larger library)* need more real
tracks. Figures marked as inferences were derived, not measured.

**How sessions update this file.** A session changes an item's state and
next-step lines, its evidence list (experiment, ADR and note numbers) and
the few current numbers a session needs to act on it; when an item is
finished or closed, its line moves to "Done and closed". Results go into
the experiment files, the session notes and `docs/status.md`, not here;
an item does not gain a paragraph per session. Open decisions are in
`docs/decisions-queue.md`. The roadmap's text up to session 75, with each
item's session-by-session history, is in `docs/roadmap-history.md` under
the same headings and item names (taken from commit `009fe55`).

States: *open* (work can start; "done, open points" when an ADR settled
the main question and smaller points stay), *measured and undecided*
(evidence in, the decision is ours and not yet in the decisions queue),
*blocked on the user* (an entry in the decisions queue), *blocked on a
larger library*, *deferred* (revisit on the condition the line names),
*demoted* (until new evidence addresses its recorded failures).

**Research proposals, 2026-10-29.** The
[matching and archive note](research/2026-10-29-matching-and-archive-proposals.md)
records six ideas with their first experiments and failure cases. All six
have since been measured: stereo power fingerprints (0245-0250, not
adopted; the inverted-channel repair 0262-0263 and its two-record pilot
0288-0289 wait on the user), filtering before
hit grouping (0243-0244, not adopted), correcting the second pass's
initial alignment (0251-0253, not adopted; the short-chain variant
0286, not adopted), quiet tracks under an identified one (0255-0258,
0269-0275; `identify --under`, ADR 0025, Proposed), a harmonic/percussive view
(0259-0261, not adopted) and recurring passages across mixes (0267-0268,
a pilot). Automated counterexample search was piloted in 0264-0266.

## The active queue (2026-11-05)

The order of work agreed after the external review of 2026-11-05
(`docs/research/2026-11-05-external-review.md`). The items below hold
each item's state and next step, and `docs/roadmap-history.md` their
history; open decisions are in `docs/decisions-queue.md`.
Each session takes the first item it can do without a pending decision.

1. **The harness measures what it says.** Generated-mix scoring takes
   the highest confidence among a play's matching detections, and the
   saved reports are rescored offline; `regress` checks query identity
   and counts, records provenance and gains a strict mode; render caches
   are keyed by a recipe fingerprint; harness FFmpeg calls run under the
   core's watchdog (ADR 0018); `regress` covers the 2003 and Nocny scans
   at both scales. One session, with both regressions identical or every
   difference explained.
   *Session 77 (0279):* the scoring and `regress` parts are done: plays
   take the highest level (6 of 54,477 saved plays moved, all in 0250);
   `regress` reports incomparable pairs, compares `provenance.json` and
   has `--strict`.
   *Session 78 (0280):* done. Render receipts hold a recipe (sources'
   path, size and modification time; steps; renderer and FFmpeg's
   version) and refuse any other; every harness FFmpeg run is under the
   core's stall timeout (encodes count advancing `-progress` reports);
   `regress` scans all three development mixes, and an older baseline is
   "incomplete". Both regressions identical; baselines `session-78-linux`
   and `session-78-linux-nas`. Next step, optional: reuse of identical
   recipes across work directories.
2. **`--under` before its default.** Fix the fourth drop rule
   (`named_as_one_track` matches non-overlapping plays within 90 s;
   0274), run the mask's entries in parallel, measure 0275's table
   again, then the decision (queue item 2).
   *Session 79 (0281, 0282):* both fixes made and measured. Both lost
   partners are kept, with no new wrong find; the option takes
   1.85-2.01 times the default at NAS scale (Nocny 0.3 s over twice),
   with about 2.4 GiB more peak memory. *Session 80 (0283, 0284):*
   harder generated cases (broadcast, key lock, rides, three records,
   a mistaken foreground, a left-out partner): no unrelated title
   printed; the registered bar failed in the mistaken-foreground case
   through 2 related versions of a partner that plays, in pair drops
   with both records out of the library. Next: the decision (queue item 2), after one real double
   drop with a known tracklist.
3. **The user's decisions** in `docs/decisions-queue.md`, then one
   session that adopts the accepted ones together and reruns the
   protocol once.
4. **Real mixes with roles.** New development mixes with tracklists
   written by ear, and a new held-out set committed before any search
   (queue item 6). Then a field trial: `identify` over sets outside both
   roles, with about 20 printed entries a week checked by ear through
   `audition`.
5. **The minimum span**, once decided (queue item 1): errors scored per
   passage and as a probability over placement, the rule frozen, one
   confirmation on the shared panels' validation halves, an ADR.
   *Session 81 (0285-0287):* short-chain stability under library
   perturbations measured: no confident answer changed; short plays'
   hits and speeds move far more than long ones'; the bounded fit for
   short chains only is not a candidate (0286).
6. **Two records for a flagged inverted file**, the user's idea of
   2026-10-31 (queue item 3): a harness pilot that scores the two forms
   apart and never sums or chains their evidence; as-ripped and repaired
   queries, genuine wide stereo, the shared-material negatives.
   *Session 82 (0288, 0289):* done in the harness (`--second-records`);
   the registered bar holds. Next: the decision (queue item 3), then an
   ADR.
7. **The band profile of 0034** under the default matcher at 262 tracks
   (shared panels' development halves, `recall`, `robust`), criteria
   first; a NAS re-extraction only if it passes.
   *Session 83 (0290, 0291):* measured; neither variant passes (no
   gain under combined damage; a loses equal-level blends, b
   strengthens shared material). Next: the decision (queue item 10).
8. **A second-chance search over the gaps**: the costly searches
   (targeted extra rungs first, estimated at under 1% more CPU in
   0226-0230, session 51; then key lock) only over the stretches
   `gaps::uncovered` finds without a confident match.
   *Session 84 (0292, 0293):* built in the harness (`--gap-rungs`,
   `--gap-key-lock`). T1 equals L1's sweep recall query for query at
   under 0.3% more CPU on the development mixes at NAS scale, but fails
   its registration on generated edge mixes (a confident play of the
   same file covers a returning play's time); T2 brings the Cool Rock
   bootleg back. Key lock over the gaps of the turntable ladder passes
   its registration: 0.56-0.58 times both ladders' CPU on the
   development scans, the same answers, 97 and 94 of 109 key-locked
   plays in generated mixes against 99 (decisions queue item 9). Next:
   a revised covered time (confident segments only), registered on new
   mixes; the key-lock default is the user's decision.
9. **Hash-rarity weighting** as a diagnostic score beside the hit count
   (hits weighted by the inverse log of their posting-list length), on
   the shared panels' development halves and 0276's break lists.
10. **Break search, measured as retrieval** (queue item 4): judged
    holders as relevant, a listening pack across ranks, both playback
    ladders; a library-wide clustering first, so that a null of random
    passages is not contaminated by unclustered rips.
11. **Process.** A `gunfinger-eval protocol --variants ...` runner (one
    set of renders, one loaded index, one table against the baseline);
    the roadmap rewritten as open items with state and next step (done
    in session 76); the
    diagnostics cleanup (queue item 8); FFmpeg's version in the record
    header and perhaps an explicit resampler (a profile bump, an ADR); a
    posting-count set-aside threshold stored in the saved index instead
    of the share-based cut, so that a small prune moves nothing (an ADR
    superseding part of ADR 0014).

Demoted until new evidence addresses their recorded failures: another
general harmonic or band-dynamics verifier, a global stereo-power front
end, wider fan-out, a large Panako run, and further search-speed work
("Where search time goes": 24 s per hour-long mix at NAS scale is far
inside the brief's bound; reopen only if `--under`'s default needs it).
"A confidence statistic for shared material" waits on queue item 7.

## Library and storage

- **Search memory at scale.** *Open; not in the active queue.* Next:
  16-byte hits and sorting the lines without a half-size copy would take
  off about 1.2 GB at 26,462 assets with identical detections; not done.
  Facts: since session 12 a hit is found as 8 bytes and grouped as 16; at
  33,596 records the 2003 mix peaked at 4.8 GB, the index 3.8 GB of it.
  Evidence: 0029, 0063, 0086.
- **On-disk index.** *Done (ADR 0011; the harness's saved index, ADR
  0014); compression open.* Next: compression needs `u64` byte offsets
  past 4.29 GB (ADR 0011); it would also shrink the two NAS index files
  on the local disk (the CLI's and the harness's). Evidence: 0087.
- **More than 65,536 assets.** *First step done (ADR 0010); the rest
  blocked on a larger library.* A posting holds 16 frame bits and 16
  asset bits: 65,536 assets, tracks up to 17:28 (default limit 17:00).
  At 65,536 assets of the NAS mean length the index would hold about
  1.84 billion postings, 7.4 GB in memory, before the search's own
  memory, so memory may call for loading shards one at a time before
  addressing does. Next: past 65,536 assets, shard by asset range (ADR
  0007, option A); shards do not reduce the work or the chances of a
  false match. Decide collection boundaries from evidence (drum & bass
  alone, hardcore alone, their union: false candidates, missed
  cross-genre matches and cost, with one confidence rule; ADR 0007,
  measurement 3). Searched shard by shard, the fullest 1% of posting
  lists must still be chosen over the whole searched library. Evidence:
  0063; ADR 0007.
- **Library identity and relocation.** *Done (ADR 0012); open points on
  a NAS.* Open: indexing throughput, slow `stat` calls and reconnects on
  a NAS; the two risks of reaching the NAS from Linux (ADR 0020), checked
  with `scripts/analysis/store_identity.py`: names stored decomposed
  (NFD) by the Mac that Linux lists precomposed would be indexed again
  under new names, and verdicts, manifest references and ignore patterns
  naming them would not match (a fix needs an ADR and a way to find the
  existing records); modification times of another precision would make
  up to 32,000 records (84%) look out of date. State unclear here:
  session 32's notes (`docs/history/session-notes-01-39.md`, "Session
  32") record the Linux index run (the same 40,683 records as the Mac's,
  160 equal only after NFC; NFD paths in
  `docs/panels/recall-seed-2026.json` and
  `docs/experiments/data/0129-second-panel-development.json`, so a
  `recall` run on Linux would not find that panel file's record); the
  roadmap's text did not record whether that closes the two risks.
- **Files that vanish while the library is listed.** *Done (ADR 0015);
  one open point.* Open: a share that drops and comes back between a
  vanished entry and the root check gives that run a partial library
  (the warning says so); comparing the root's device before and after
  would catch a root that is itself a mount point left behind empty.
- **A share that hangs.** *Done for FFmpeg and FFprobe (ADR 0018); open
  for the rest.* Open: listing the library, reading a file's size and
  modification time, and opening the recording still block on a hung
  share; with a share that never answers again, every remaining file of
  an `index` run waits two minutes on each worker (stopping a run after
  many stalls in a row would be a separate decision).
- **Library ignore file.** *Done (ADR 0009); open points.* Facts: the NAS
  store holds 40,666 records (revision `0bd3f0c23fabc698`), the NAS-scale
  baseline is `session-75-linux-nas` (0278); DJ sets indexed as tracks
  are left out since 2026-10-31, while uploads cut from DJ sets stay in
  the library by the user's decision. Prunes: sessions 52 (0231), 64
  (0254) and 75 (0278), each with no scored result changed. Open in the
  item's text (not rechecked in session 76): `2002 - CPT003 - Kemal vs.
  Rob F & Impulse - 256` (notes "Mixed-CD tracks in the library") is
  not listed. The clusters' verdict links leave out ignored files since
  session 10 ("More development mixes", step 2, in the history). 0065
  listed 33 folders with at least 75% joined boundaries as a listening
  list (`work/s9/mixed-cd-candidates.tsv`); session 10's notes record
  which of them became ignore patterns.

## Reports and command line

- **Time without a confident match.** *Open, done with open points:
  printing done (ADR 0023, 0024); the recurring-passages pilot open.* `identify` prints gaps of 45 s or
  more as "ID - ID" lines with a range of tracks from the gap matched
  against itself (`gunfinger_core::repeats`); the covered time is
  computed once, in `gaps::uncovered`. What is left belongs elsewhere:
  records sharing material, pitch rides (0166) and a gap holding no
  missed track, which the estimate cannot tell from one that does. The
  pilot of recurring passages across mixes (`gunfinger-eval passages`,
  0267-0268) found 192 of 312 recurring passages (whole mixes: 266) and
  linked no unrelated records; the product's ladder reaches none beyond
  12% relative speed; not a product change; a next step would search
  whole mixes and measure archive-scale cost. Novelty segmentation comes
  only if counting stays out of reach. Evidence: 0159-0166, 0234, 0235;
  `docs/research/2026-10-20-unidentified-gaps.md`,
  `docs/research/2026-10-21-printing-gaps.md`.
- **Possible plays inside a confident play of another recording.**
  *Done (ADR 0019); `identify --under` measured and undecided (ADR 0025,
  Proposed).* Open: whether the tracklist prints a marked play is ours
  to decide, with an ADR (R1, whether the two files share an alignment
  of 60 hits or more by `pair`, printed every record playing under an
  unrelated partner and let in no shared material, 0215-0218). `--under` gains 39 right
  entries and no wrong one on generated double drops and changes nothing
  on the development mixes, at 1.85-2.01 times `identify`'s time at NAS
  scale (about 17 s per hour of mix) and about 2.4 GiB more peak memory
  since session 79's fixes (the fourth drop rule, masks on the workers).
  Caveat: in session 65, with
  another rip as the reference, the residual once made a known related
  version of the masked record confident at NAS scale; a product
  version needed a rule for related versions (session 70). Next:
  decisions queue item 2, after one real double drop with a known
  tracklist. Evidence: 0107, 0215-0218, 0255-0258, 0269-0275,
  0281-0284; notes "Session 72", "Session 73", "Session 79", "Session 80".
  Session 80's harder generated cases printed no unrelated title; the
  bar failed with a mistaken foreground through 2 related versions (0284).

## Matching

- **A minimum span instead of 3 windows.** *Blocked on the user
  (decisions queue item 1); measured.* No variant meets the fair test's
  F and the grid criterion together. On the grid, B(15) makes 800 of 960
  queries confident with 0 flips, against 693 with 30 today (0136-0142).
  Facts: on session 28's
  brief plays at NAS scale, 15 s plays are confident 504 of 522 times
  under B(15) (200 hits over at least 15 s of supported time), 519 under
  A(10) and B(10), against 228 today. Under library perturbations
  (0285, 0287) no confident answer changes today, because short plays
  near the rule fail on windows; a 10-20 s play's true detection moves
  by 5% or more 6-7 times as often as a longer play's. On the fair
  shared-material test (0146-0148) B(15) fails F1 by 6 of 168 wrong
  answers, all from three 20 s passages placed exactly on a window border, where today's rule
  names the same wrong files at 8-9 of their other positions; A(20) and
  B(20) meet F but name fewer brief plays; A(10) fails F1 by 9. Next:
  active queue item 5 (placement-aware scoring, the rule frozen, one
  confirmation on the shared panels' validation halves, a new ADR); the
  hit threshold stays fixed to isolate the change. Evidence: 0020, 0047,
  0057, 0136-0142, 0146-0148, 0285-0287; the variant worktree `s29-span`
  (never merged).
- **Speed that wanders within a play.** *Done for pitch rides (ADR 0021,
  0232); open: wander within one side and rides inside short plays.*
  `SpansAtSpeed(0.006)` (the harness's opt-in was `--apart-in-speed
  0.006`) keeps both sides of a ride (the old rule is
  `--overlap-by-spans`); against shared material it relies on the
  240-hit rule's margin, not on its speed condition (session 45), and
  related versions' second alignments reached at most 55, 44 and 67
  hits in the panels, drops and ridden mixes (session 57). Open: `--speed-per-stretch` (opt-in; Star
  Trails +11% hits, no measurable CPU, 0032) is adopted only if a real
  play near the 240-hit rule needs it; rides inside short plays are a
  question for the window condition (0180: a ride from 2% in the middle
  of a 30 s excerpt costs recall); `--overlap-by-chains` (0178) is not
  adopted and would need an ADR superseding ADR 0008's consequence for
  overlapping parts. The second pass's fits (`--second-pass-fit`,
  0251-0253) were not adopted; the bounded fit for short chains only
  (`--second-pass-fit iterated-short`, 0286) removes most of a short
  chain's speed movement under perturbations but changes no confident
  answer there and lifts calibrate's strongest false at 262 tracks from
  100 to 107: not adopted
  (`docs/research/2026-10-30-second-pass-fit-decision.md`). `pair
  --fine-rungs` follows copies that drift against each other (0219,
  0220). Evidence: 0022, 0024, 0032, 0166, 0175-0180, 0209-0211, 0219,
  0232, 0236-0240, 0251-0253, 0285-0286;
  `docs/research/2026-10-22-pitch-rides-decision.md`.
- **The edge of the ladder.** *Measured and undecided; the targeted
  variant built and measured (session 84), not passed.* Facts: recall
  holds to about 9% of relative speed against the remaining copy; each
  extra rung pair moves the reach by 0.4% for 4.3-5.3% more search CPU
  over the whole query; for a deck at ±5%, copies with no other copy in
  reach fall from 29 to 19, 7, 3 and 1 with 1, 2, 3 and 5 extra rung
  pairs. One rip near -8% (Siren Bass's upload) loses its confident
  answer under every wider ladder (0251, 0252). The same rungs searched
  only over the gaps (`--gap-rungs`, 0292) give the wider ladders'
  sweep recall query for query, keep the Siren Bass answer and cost
  ×1.000-1.003 on the development mixes at NAS scale (L1 ×1.045); T1
  fails G3 at NAS scale (generated edge mixes: 50 of 92 edge plays
  against L1's 56, the losses mostly returning plays whose time a
  confident play of the same file covers), and T2 adds the Cool Rock
  bootleg answer. Next: a covered time from confident segments only (or
  a play's own alignment), registered on new generated mixes; then the
  decision (ours). Evidence: 0033, 0059, 0219, 0226-0230, 0251, 0252,
  0292; `docs/research/2026-10-25-ladder-edge-decision.md`.
- **Peaks across frequency bands.** *Measured and undecided: neither
  variant passed its registration (session 83; decisions queue item
  10).* Under the default matcher at 262 tracks both of 0034's variants
  keep every answer with no wrong confident answer and raise the
  development mixes' true hits (a: 9% fewer postings, Stakka +32%; b: 6%
  more, +39%), but neither gains the registered 2 points on recall's
  damaged sets (a +0.38, b +1.21); a loses `robust`'s equal-level
  blends (31 → 23 and 28 → 21 confident; 1,932 overall against 1,938),
  and b strengthens shared material on the second shared panel. Next: the user's choice between
  closing it and a revised registration (variant b at NAS scale in a
  scratch store); no NAS re-extraction without that. Evidence: 0034,
  0207, 0290, 0291; brief 83.
- **Detection boundaries.** *Open; not in the active queue.* Facts:
  boundaries are the first and last aligned hit; crossfade edges lie a
  median 2.7 s inside the truth (0020); `--trim-ends` cost 2.6 s of true
  play per second removed (0031); at a pitch ride the second pass moves
  both sides' facing ends a median 4 s into the ramp (0175). Per-second
  scoring (`gunfinger-eval seconds`, 0139) measures boundary accuracy.
  Evidence: 0008, 0020, 0030, 0031, 0139, 0175.
- **Where search time goes.** *Demoted (the active queue): 24 s per
  hour-long mix at NAS scale; reopen only if `--under`'s default needs
  it.* Facts (0214): the analysis is 13-15% of the first pass,
  grouping 40-46%. Measured and not kept or not adopted: integer phases
  (0108), narrower hits and grouping straight from the lists
  (0171-0173), the scatter variants (0200, 0202), the sketch before
  grouping (0243, 0244). Open: FFT lengths with small prime factors
  (0109) change peaks and output; part 1 of their retrieval study
  failed two criteria (0119), part 2 needs harness work, then timing,
  and adoption is ours. A first `index` is 86% FFmpeg's decoding and
  resampling (0099), whose options are fixed. Workers: at 16 workers
  NAS-scale scans took about 30% less time than at 32 (0158), but with
  the two `take_part` changes in master 32 workers are faster than 16
  (0167, 0169: 37.9 s against 45.3 s), so the old text concluded the
  default need not change; decisions queue item 9 still cites 0158.
  Evidence: 0028, 0075, 0080-0088, 0093, 0096-0099, 0103, 0108-0109,
  0113, 0117, 0119, 0158, 0167-0173, 0189-0193, 0199-0205, 0212-0214,
  0243-0244; `docs/research/2026-10-14-matching-proposals.md`,
  `docs/research/2026-10-21-low-bandwidth-search.md`.
- **Behaviour at scale.** *Blocked on a larger library.* Facts (session
  57, 0239, at 40,678 records): calibrate over seeds 2026-2029 and the
  Stakka scans gives weakest identifying 675, strongest false 282, 7
  false accepted (all Cool Rock Stuff; decisions queue item 5), 365
  false candidates at 30 hits or more. Next, at each new size:
  recalibrate (`docs/calibration.md`, "After indexing more tracks"),
  repeat the related-recordings census (0015, 0023) and check whether
  two different recordings reach the confident rule. The new tracks must
  include jungle, hardcore, remixes, versions and producer catalogues
  that reuse material. Evidence: 0012, 0049-0052, 0222-0224, 0233, 0239.
- **The filter and famous breaks.** *Open; measured stable under copies.* Facts: copies of records do not move candidates or answers
  (0222-0225); a candidate's evidence depends on the filter through the
  speed and span the second pass starts from (0242, expected under ADR
  0014); a famous break lines up with up to 403 of the NAS library's
  files, and its holders lose more hits to the filter (0276-0277). How
  far a short chain's speed and hits move when the library changes:
  0285 and 0287 (up to a rung for 10-20 s plays, no confident answer
  changed). Open: how often a short chain's speed hinges on one hit
  (not counted); break
  search as a command (decisions queue item 4; active queue
  item 10); a posting-count set-aside threshold (active queue item 11);
  if candidates go missing, rank hashes by distinct files or weight hits
  by rarity (active queue item 9); a real collection's duplicates beyond
  exact copies (the 65,536-asset limit capped the padding at NAS scale).
  For `identify` the break's many holders change nothing measured; the
  passage query with every list costs about 8 thread seconds per query
  at NAS scale. Evidence: 0012, 0144, 0222-0225,
  0242, 0276, 0277; `docs/research/2026-11-02-break-search.md`.
- **A confidence statistic for shared material.** *Blocked on the user
  (decisions queue item 7; the active queue: it waits on that item).*
  Facts: with the played recording left out at NAS scale a related
  version is wrong confident in 106 of 218 queries of the first panel
  (0111) and 5 of 135 of the second (0128, 0129); no pre-registered candidate
  passes against real true plays (0116, 0118); the containment rule
  (0091) was not adopted, since a new development mix that plays
  versions whole is needed first; reporting the ambiguity
  as a note flagged 13.9% of true plays (0120-0123); the version verifier,
  versions 1 and 2, failed one criterion each (0124-0127, 0133-0135;
  `gunfinger-eval versions`, kept as a diagnostic); the harmonic view
  was not proposed (0259-0261). An exploratory rule found after session
  21's run (a play the map marks with under 6.276 hits per second
  rejected) is frozen in `docs/brief-21.md` for a test on real mixes not
  used so far. The shared panels' development halves are a required
  check for anything that adds or changes evidence; their validation
  halves stay unsearched. Next: if the user confirms decisions queue
  item 7, the item becomes presentation of shared passages and the
  statistic work stops; a version 3 of the verifier would need new real
  mixes with checked tracklists. Evidence: 0026, 0051, 0053, 0055, 0076,
  0091, 0110-0118, 0120-0129, 0132-0135, 0187, 0215-0218, 0225, 0236,
  0259-0261; `docs/panels/`;
  `docs/research/2026-10-16-version-verification.md`,
  `docs/research/2026-10-31-harmonic-view-decision.md`.
- **Evidence beyond exact pair hashes.** *Steps 1 and 2 done with no
  rule; step 3 open.* Step 1, the peak verifier (`--verify`, 0054, 0056,
  0058), separates no better than hits; kept as a diagnostic. Step 2,
  tolerant pairs and query fan-out (0149, 0151, 0206-0208): related
  versions gain as much as the played recording; settled, no rule; the
  loss under damage is peaks. Next, step 3: denser query peaks around
  candidate passages, the reference store unchanged, used in the
  verifier first; measure peaks recovered under EQ and blends,
  accidental agreement and CPU. Steps 2 and 3 matter more for recall
  than for separation: the 57 misses within 9% of relative speed in
  0059 are weak copies found at 25-239 hits; measure those copies and
  the shared-material scenarios (0055) together. Wider fan-out is demoted (the active
  queue). Evidence: 0004, 0027, 0054-0059, 0149, 0151, 0206-0208;
  `verifier.rs`, `verifier/pairs.rs`.
- **Longer hashes.** *Deferred.* Triplet hashes would scan 25-79 times
  fewer postings but lose about half their evidence under heavy damage
  (0014, offline). Revisit only if profiling on a real large library
  shows lookups still dominate after simpler changes; a one-byte
  companion selector per posting (proposal 4 of
  `docs/research/2026-10-14-matching-proposals.md`) only if grouping
  still dominates.
- **Evidence summed across a play.** *Deferred.* Summing every segment
  is unsafe (chance sums reach 125 hits, 0011); a looped shared passage
  would become confident (0055). Revisit when several independently
  reviewed missed plays have strong, compatible segments that together
  pass 200; a rule counting only segments that advance through the
  reference is a proposal, not measured.

## Evaluation

- **The verdicts by ear on borderline pairs.** *Open; more judged
  pairs needed (larger library).* Facts: every borderline pair between
  two clusters of session 33's NAS run has a verdict (2026-10-22); China
  Cup's upload counts as right since ADR 0022 (`docs/same-titles.txt`);
  the four pairs heard on 2026-10-30, Cool Rock among them, carry
  different titles, so the 7 Cool Rock detections stay wrong confident
  answers. `clusters` stops on a chain that contradicts a `different`
  verdict; `--cut-sparsest` and `--reuse-pairs` handle it. Open: on the
  14 hardest judged pairs no measure separates (AUC 0.40-0.56, coverage
  0.06; 0187); `drift_0.8_20` is not better by the registered bar, and
  the 10 s variant found after the test needs more judged pairs to
  decide (0220, 0221). Session 40's pack (`work/s40/pack/`) was heard
  on 2026-10-25 (status, "after session 46" and "after session 51");
  two of its items got no verdict, each holding a file that should not
  be in the library. Evidence: 0049, 0057, 0128, 0186-0188, 0220,
  0221, 0233.
- **Clustering precision.** *Open.* A join rule on the near-offset share
  would rest on one pair more than supported time gets right; a change
  is ours, with an ADR (0186, 0187). Coverage, supported time and the
  near-offset share all fail on uploads whose speed drifts and on edits
  that move the rest by more than 10 s. `pair --fine-rungs` lifts drifting
  same pairs and version pairs alike; the registered variant failed
  (0220). Still to do: inspect long sparse alignments and chains through
  mixed-CD tracks (0058), and check a sample of joins by ear. A
  library-wide clustering comes first for break search (active queue
  item 10). Evidence: 0057, 0058, 0186, 0187, 0220.
- **The possible tier at NAS scale.** *Blocked on a larger library; the
  value is decided: 60 stays (our decision); measure at each size.* Every strongest match between
  different recordings checked from 40 hits up shares material, by ear;
  no chance match has been heard above 40 hits. The three development
  mixes show 3.2, 2.8, 1.8 and 1.1 possible plays per hour at 60, 80, 100
  and 130 hits, every one a version or shared material (0092, 0094,
  0102); no development true play depends on the tier, and at 262 tracks
  a higher threshold hides only damaged excerpts (48-128 of 278 at
  80-130). Next, at each size: count what a higher tier costs (weak true
  plays lost, possible plays per hour of mix). Shared-material notes
  could keep a lower threshold, but only where the two library files
  match each other in that passage. Evidence: 0040, 0041,
  0051, 0052, 0092, 0094, 0102.
- **Recall beyond the corpus recordings.** *Open; the development half
  measured, the validation half kept for a frozen rule.* Facts (0238, 40,678
  records): other rips only 174/180 confident, brief plays 228/522 (15
  s) and 493/522 (20 s), combined damage 5/522 confident, all three
  together 0/360 confident (107 possible), other rips with brief plays
  171/360 (6 Known Unknown excerpts were found only through the pruned
  track-fix copy); no wrong
  confident answer and no held-out query confident in any condition.
  `recall` needs `--panels work/s33/panels`. Evidence: 0068, 0143-0145,
  0156, 0223, 0238.
- **Stereo faults before the downmix.** *Blocked on the user for the
  inverted-channel repair (decisions queue item 3; active queue item 6,
  piloted in session 82); the rest measured.* Facts: the
  downmix loses every excerpt with one channel's polarity inverted
  (0194-0198); the polarity check flags an estimated 0.08% of the NAS
  store (5 of 6,766 files sampled, about 31 records; 0262); the
  power-sum front end (`--front-end`,
  0245-0247) and power-sum queries alone (0248-0250) were not adopted;
  replacing the flagged records (`variant-store --repair-inverted`,
  0263) recovers the four plainly inverted recordings but loses every
  as-ripped query. Two records per flagged file (`--second-records`,
  0288-0289) keep both kinds of query and name nothing new; the
  registered bar holds. An ADR would decide the
  check at indexing, a second record kind the saved index can tell
  apart, and one entry per file in the output (the stronger record's
  play; `docs/research/2026-10-30-stereo-power-decision.md`, "Session
  82"). Open (ours): a file inverted only in part (We - Polynesian
  Inventor, heard inverted, not flagged; a check per window would need
  its own registration); the larger of the mid's and the side's power
  per bin (unmeasured). Evidence: 0194-0198, 0245-0250, 0262, 0263,
  0288, 0289; `docs/research/2026-10-30-stereo-power-decision.md`.
- **More development mixes.** *Blocked on the user (decisions queue item
  6; active queue item 4).* The three development mixes score 11/11,
  12/12 and 17/17 at NAS scale with 0 wrong and no longer show failures;
  the held-out test mix has one evaluation left (ADR 0004). Agree each
  mix's role before looking at its results. Candidates on the NAS (0065,
  `work/s9/candidate-mixes.tsv`): no `identify` before their roles are
  agreed; `EM_1999_02_07_-_Ed_Rush_Optical.mp3` stays out of
  development use, and `sety/vertical.mp3` was left alone (session 18).
  Each new genre needs its own development and held-out mixes; check
  the speed range those DJs used. A real test of the containment rule needs a mix whose tracks
  have indexed versions sharing much of their audio. Evidence: 0060,
  0065-0067, 0069, 0100-0102.
- **Automated counterexample search.** *Open; a pilot (0264-0266).*
  `gunfinger-eval counterexamples` found 188 failures in 17 classes at
  262 tracks and 72 in 11 at NAS scale; every play lost needs a channel
  fault or a blend. Next: run it with the same seed beside a variant, so
  that classes compare sample for sample; for new classes, axes the
  generator lacks (other EQ, beatmatched and quiet partners, key-locked
  plays, the fault's placement) and fades on the blend. Cost: about
  16-17 minutes per run. It does not replace real positive controls.
- **Public DJ-mix datasets and per-second scoring.** *Per-second scoring
  done (0139); the datasets blocked on the user (a browser download;
  decisions queue item 9).* `gunfinger-eval seconds` scores saved
  `mixes`, `shared-mixes` and `scan` reports; generated mixes 94.2% of
  indexed seconds correct, the development mixes 86.8% (262) and 90.7%
  (NAS) at a 30 s margin. Not yet unit-tested: the truth built from
  manifests and generated mixes. With Mixotic and UnmixDB (another
  genre, their own library and store, split by recording family across
  both, the frozen matcher run on the validation part), `seconds` gives
  the first external per-second score. UnmixDB's time scaling comes from
  SoX: check which variants resample and which keep the pitch. Compare
  systems only by running them under one protocol.
- **The printed tracklist.** *Measured and undecided (ours, with an
  ADR).* Grouping and printing lose nothing the search finds
  confidently, apart from one merge in 512 plays (0182-0185); what leaves
  the tracklist is ADR 0019's cost (a record inside its partner found
  only as possible). Open: whether a marked play may stay in the
  tracklist (by length, or marked as possible with its partner named),
  and whether `same_recording` needs more than a 5 s start and half the
  shorter play when two files' titles differ. The mark is right on
  shared material too (64 of 105 marked plays at 262 tracks) and no
  measure in the report tells the two apart (0183); a length bound of
  40 s would print every drop partner and 19 of 64 shared-material
  lines. Drops of the same rank and
  real double drops are not measured (0217). The T-Minus vocal mix
  upload that 0185 left for listening opens with 56 s of Digital &
  Spirit - Crash and is ignored on the NAS since 2026-10-27 (status,
  "2026-10-27: the T-Minus vocal mix upload ignored on the NAS").
  Evidence: 0182-0185, 0215-0218.

## Engineering

- **Before publishing.** *Open.* Check that the history holds no audio
  from `corpus/` (`git log --all --stat`). We keep the library file
  names in `docs/` as they are (2026-10-06).
- **GitHub Actions.** *Open.* Run `scripts/check.sh` on push. FFmpeg must
  be installed in the runner for the codec tests.

## Done and closed

One line per finished or closed item; the history is in
`docs/roadmap-history.md` under the same name.

- **Database.** Not needed: the peak store is one file per asset and the
  index is rebuilt from it in 0.5 s; revisit only if detections need to
  be stored. Report edits and the Track/AudioAsset model are out of
  scope (ADR 0017).
- **The Track/AudioAsset model.** Out of scope for the product (ADR
  0017, 2026-10-13).
- **Report edits.** Out of scope for the product (ADR 0017,
  2026-10-13); the listening tools stay development tools for
  evaluation ground truth.
- **No record inside the track length range.** Done in session 10.
- **Titles of mislabelled files.** Done (session 11, `other_titles`);
  the product flags disagreeing titles and never chooses one (ADR 0017).
- **A full-screen TUI.** Replaced in session 14 by `questions
  detections` and `audition` (ADR 0016).
- **Panako as an outside benchmark.** Run in session 24b at 262 tracks
  (0130, 0131, 0150); the NAS-scale run was not needed, since Panako
  found only 7 plays that Gunfinger reports as possible. A large Panako
  run is demoted (the active queue).
