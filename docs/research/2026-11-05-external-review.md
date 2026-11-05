# External review of the roadmap, experiments and harness (2026-11-05)

On 2026-11-05 two reviewers worked independently and read-only: one
GPT model and one Claude model, neither launching agents of its own.
Each read `AGENTS.md`, the briefs, the roadmap, the ADRs, the research
notes, recent sessions and a sample of experiments across the whole
range, and traced parts of the harness code. Their reports are kept
outside the repository (`work/review-2026-11-05/`). This note records
the findings we checked and what was done with each. The code
references were confirmed against the tree at `81b83c2`. Neither
reviewer ran a harness command, so the code findings come from reading
the code, not from reproducing them.

## The common conclusion

Both reviewers found the evidence discipline sound: criteria are
registered first, failed bars are reported as failures, fresh seeds
confirm, credits are counted apart, the validation halves and the test
set are untouched, and ADR 0008, 0021, 0022, 0023 and 0024 rest on
enough evidence. What they challenged is the inputs and the process,
not the method. The three development mixes are saturated (all plays
found at both scales), so recent decisions rest on generated mixes and
panels built from a few familiar recordings. Several harness checks
guarantee less than the sessions assume. The session ritual and the
documents repeat themselves, which costs time that could go to new
evidence.

## Harness findings, confirmed in the code

1. **Generated-mix scoring takes the level of the strongest detection
   by hits.** In `crates/gunfinger-eval/src/mixes.rs`, `score_play`
   picks the matching detection with the most hits and uses its
   confidence for the whole play. A play with a confident detection can
   therefore score as possible when a stronger-by-hits detection covers
   fewer windows. `counterexamples.rs` works around this, and 0266
   records a changed failure after that runner was corrected. How many
   conclusions about generated mixes would change has not been
   measured. Fix: take the highest confidence of all matching
   detections, then rescore the saved reports offline.
2. **`regress` compares by position.** `compare_sweeps` in `regress.rs`
   zips the queries and speed rows without checking counts or query
   identity, so a shorter report can print "N of N queries with
   identical detections". The baselines carry no provenance (build,
   panel, cluster and same-titles digests). Fix: check the identity and
   count of every query, record provenance, and add a strict mode that
   fails on any difference, for refactors.
3. **Render caches are keyed by name and size.** `render.rs` accepts a
   cached render whose size matches its completion receipt; sweep
   render names hold only the draw and speed. After a change of panel
   or source audio at the same path, a cached render can be reused for
   different truth. No published result is known to be affected. Fix: a
   recipe fingerprint (source identity, excerpt, playback, encoding,
   faults, renderer version) in the receipt.
4. **Harness FFmpeg calls have no stall timeout.** `render.rs` calls
   FFmpeg with `.status()` and `.output()` and not through the core's
   `decode::watch` (ADR 0018). A stalled reference render from the NAS
   could take a whole unattended session. Fix: the watched child with a
   progress heartbeat, since encodes to file are silent.
5. **`regress` covers one development mix.** `report_names` builds the
   Stakka sweep and scans only. The 2003 and Nocny mixes, which hold most
   of the development tracks and the only real version plays, run
   outside it. Fix: add both scans at both scales.
6. **`--under`'s fourth drop rule.** `named_as_one_track` in
   `crates/gunfinger/src/under.rs` accepts `lines_up_with` within a
   90 s neighbourhood without requiring overlap, and dropped two right
   partners in 0274.

## Decisions and experiments

- **Saturated development data and a spent test set.** See
  `docs/decisions-queue.md`, item 6. Both reviewers rank new real,
  annotated mixes as the most valuable missing evidence: for the
  minimum span, `--under`'s precision and the shared-material question.
- **Fresh seeds are not independent families.** 0124 and 0216 rest
  mostly on one relationship (Clockwork). `recall`'s split key is
  normalised artist and title, not an audio family, so copies tagged
  differently may fall on both sides (not shown to happen). Results
  should state the number of distinct families beside the query count
  (now a standing rule).
- **Registered criteria that tested the wrong property.** 0263's
  exact-regression item failed on set-aside moves that 0242 had
  explained; 0148's F1 counts window-border placements. The records
  stay failures; a revised candidate needs a new registration that
  separates invariants (unchanged records identical) from intended
  changes, and scores errors by placement.
- **The zero-wrong bar against the seven Cool Rock detections at NAS
  scale**, and the calibration row's "twice the strongest false" (282
  hits) against the threshold of 240: decisions queue, item 5.
- **The minimum span** is the largest measured recall lever and has
  waited since 2026-10-27: decisions queue, item 1. The reviewers
  disagree on how to read B(15)'s six border errors; both views are
  recorded there.
- **The band profile of 0034** (+34% true hits, 9% fewer postings) was
  never retested under the default matcher, although 0207 found that
  lost evidence is in the peaks, not in the pairing.
- **Targeted extra rungs** were estimated at under 1% more CPU and
  registered as a variant to build (roadmap, under 0251-0252), and were
  never built.
- **The decoder's resampler is FFmpeg's default** (`-ar` in
  `decode.rs`, no explicit resampler options). Peaks differ between
  FFmpeg 8.1.2 and 8.1.3 and between the machines (0153; `grid` 65/144
  against 67/146), and where the difference comes from is not traced.
  Recording FFmpeg's version in the record header is cheap and lets a
  store built elsewhere be recognised. Pinning the resampler explicitly
  may reduce the differences (an inference). Either change alters the
  records, so it needs a profile bump and an ADR.

## Ideas not explored

- **Hash-rarity weighting** as a diagnostic score: count the second
  pass's hits weighted by the inverse log of their posting-list length.
  Famous breaks' evidence is made of common hashes (0277). This is
  untested in the project; the roadmap mentions it once.
- **A second-chance search over the gaps**: the turntable ladder over
  the whole mix, then the costly searches (extra rungs, key lock,
  perhaps `--under`) only where `gaps::uncovered` finds no confident
  match. This turns three cost questions into one.
- **Two records for a flagged inverted file**, as the user proposed:
  decisions queue, item 3.
- **Short-chain fit stability** under library perturbations, before the
  minimum span or wider ladders depend on short chains.
- **Break search as retrieval with many relevant results**, with a null
  that is valid: today's null is contaminated by unclustered rips
  (0276), which a library-wide clustering would remove.
- **A per-query chance model** (expected chance hits from the
  posting-list lengths) to replace recalibration at each library size.
  Low priority: it does not help with versions.

## Process

Adopted on 2026-11-05:

- `docs/session-rules.md` collects the standing rules. `CLAUDE.md` no
  longer names 74 briefs as live rules; they are history.
- `docs/decisions-queue.md` lists the open decisions, so that they can
  be cleared together and adopted in one session with one protocol
  rerun.
- One home per fact: results in the experiment file, short status
  entries, handoff-only notes (in the session rules).
- Start-of-session regressions are skipped when the start build is
  byte-identical to the last final build; a prune's set-aside moves are
  attributed by one check.
- `docs/roadmap.md` starts with an ordered active queue.

Proposed, not yet done:

- A `gunfinger-eval protocol --variants ...` runner that evaluates named
  variants from one set of renders and one loaded index, and writes one
  comparison table. Sessions 60-67 each measured one variant with
  15-25 minutes of start and end ritual.
- A rewrite of the roadmap into open items with state and next step,
  with the history moved to the experiments it cites.
- A field trial: `identify` over real sets that are neither development
  nor held-out, with the user checking about 20 printed entries a week
  through `audition`.
- Replacing the share-based set-aside cut with a posting-count threshold
  stored in the saved index, so that a small prune moves nothing (an ADR
  superseding part of ADR 0014).

Not adopted: moving the briefs and checklists into `docs/sessions/`,
because many documents link to them by path, and the change to the
reading list removes the cost without moving files.
