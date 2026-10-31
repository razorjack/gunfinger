# Roadmap

Work deliberately left out of the proof of concept, roughly in the order the
real project will need it. Each item links to the decision or measurement
that motivates it where one exists. Items marked *(larger library)* need
more real tracks; the rest can be done with the current library. Figures
marked as inferences were derived, not measured.

**Research proposals, 2026-10-29.** The
[matching and archive note](research/2026-10-29-matching-and-archive-proposals.md)
records six unmeasured ideas: stereo power fingerprints, filtering before
hit grouping, correcting the second pass's initial alignment, recovering
quiet tracks using an identified foreground track, a harmonic/percussive
verification view, and recurring unidentified passages across mixes. It
also proposes automated counterexample search. Each item relates the idea
to earlier results, describes a first experiment and names its failure
cases. None is an adopted decision or an experiment result.

## Library and storage

- **Search memory at scale.** The search now runs a block of 12 windows
  on every rung at a time and merges each block's lines once every rung
  has searched it, with identical detections (experiment 0029). The
  development scan (56 minutes) peaks at 2.9 GB at 8,122 assets and 7.7
  GB at 26,462 (skip at 240: 1.4 and 3.8 GB). What is left at 26,462: the
  index (3.0 GB), the merged lines of the whole mix (1.4 GB), the
  workers' hit buffers (about 2 GB) and the final sort of the lines.
  16-byte hits and sorting the lines without a half-size copy would take
  off about 1.2 GB, also with identical detections; not done. Since
  session 12 a hit is found as 8 bytes and grouped as 16 (experiment
  0086), 24 bytes in all as before; at 33,596 records the 2003 mix peaks
  at 4.8 GB, the index 3.8 GB of it.
- **On-disk index.** *(done in session 11, ADR 0011)* `identify`,
  `explain` and `stats` save the index they build in
  `$XDG_CACHE_HOME/gunfinger/indexes/` and load it while its header names
  exactly what they would index (format, profile, hash design, frame
  bits, track length range, asset table); a stale, truncated or damaged
  file is rebuilt and replaced. The in-memory layout is kept, 4 bytes per
  posting, so ADR 0005's delta-coded lists are not used. *(session 12)*
  The harness saves its own index of the corpus and the other store in
  `work/indexes/` and removes what a run leaves out in memory (ADR 0014;
  the NAS development scan 112 s instead of 189 s, experiment 0087), and
  `doctor` reports the store's file (size, current or not) and the
  folder's other files, deleting nothing. Still open: compression (it
  needs `u64` byte offsets past 4.29 GB, ADR 0011), which would also
  shrink the two 3.8 GB files the NAS store now has on the local disk (the
  CLI's and the harness's).
- **More than 65,536 assets.** *(first step done in session 9, ADR
  0010)* The NAS store passed 32,768 records on 2026-10-09 (32,905), so
  a posting now holds 16 frame bits and 16 asset bits: 65,536 assets,
  tracks up to 17:28 (default track limit 17:00). 32,905 is 50% of
  that; at 65,536 assets of the NAS mean length the index would hold
  about 1.84 billion postings, 7.4 GB in memory, before the search's
  own memory (4.7 GB peak at 32,905 records, experiment 0063). The next
  limit: past 65,536 assets, shard by asset range (ADR 0007, option A),
  each shard with this layout; memory may call for loading shards one at
  a time before addressing does. The collection may grow to
  about 100,000 tracks with jungle and breakbeat hardcore. Shards,
  separate libraries per genre and other layouts are compared in ADR
  0007. Shards solve addressing and can bound memory, but searching
  every shard does not reduce the work or the chances of a false
  match. *(larger library)* Decide collection boundaries from
  evidence: the same queries against drum & bass alone, hardcore alone and
  their union, measuring false candidates, missed cross-genre matches and
  cost, with one confidence rule throughout (ADR 0007, measurement 3).
  Searched shard by shard, the default matcher must still choose the
  fullest 1% of posting lists it skips over the whole searched library:
  chosen per shard, a file's candidates would depend on which shard
  holds it.
- **Library identity and relocation.** Asset identity is the path
  relative to the library root, and the peak store names its library in
  `library.txt` (the root's absolute path); every command but a
  `--store-only` search refuses a store that names another library. To move a library, put its new path in
  `library.txt`: the records stay valid while the files keep their size
  and modification time. A store moved or copied as a folder works where
  it lands, with its own saved index built once. On another machine a
  copy of the store searches with `--store-only`, with no library
  configured, or with the configuration keeping the path the store names
  even where it does not exist (ADR 0012). Open: on a NAS, indexing
  throughput, slow `stat` calls and reconnects; and two risks of reaching
  the same NAS from Linux (ADR 0020), which only the Linux machine can
  check, before any `index` from Linux (`scripts/analysis/store_identity.py`
  makes both checks, read-only):
  - *Unicode form of file names.* Through macOS's SMB client, every one
    of the 149 stored paths (of 38,105 records, store revision
    `b285da9984422fb5`) whose characters have a decomposed form is
    stored decomposed (NFD; "Łódź" as "Ło" + U+0301 + "dz" + U+0301). If
    Linux's SMB client lists them precomposed (NFC), their records are
    not found: `index` would index them again under new names, the old
    records would look like deleted files, and verdicts, manifest
    references and `.gunfingerignore` patterns naming them would not
    match. Store-only runs are not affected. Check: with the NAS mounted
    on Linux, list such files and compare their bytes with the records'
    stored paths. A fix would identify files by a normalized path; it
    needs an ADR and a way to find the existing records.
  - *Modification-time precision.* 32,000 of the 38,105 records hold
    modification times with fractions of a second, as macOS's SMB client
    reported them. If Linux's client reports another precision, those
    records look out of date and `index` would decode up to those 32,000
    files (84% of the records) again. Check: `stat` a few files on the Linux mount and compare with
    their records.
- **Files that vanish while the library is listed.** *(seen in session
  12; done in session 13, ADR 0015)* `Library::scan` stopped at its first
  read error, so a file renamed or deleted between `read_dir` and its
  `metadata` (another program may write to the library) failed the whole
  listing with "No such file or directory", and since ADR 0012 `identify`,
  `explain` and `stats` then stopped. Now a folder, or an audio file that
  would be an asset, that answers NotFound when read is left out and
  counted, the first path named in one warning (`doctor`: a note); other
  files are passed over by name without being read. Other errors stop as
  before; the root is never skipped and is read again after such a
  listing; `prune` and the harness refuse such a listing. Detections do
  not change. Open: a share that drops and comes back between a vanished
  entry and the root check gives that run a partial library (the warning
  says so); comparing the root's device before and after would catch a
  root that is itself a mount point left behind empty.
- **A share that hangs.** *(done for FFmpeg and FFprobe in session 16,
  ADR 0018)* When the NAS stopped answering, a decode waited as long as
  the share stayed silent: `index` stopped making progress on that
  worker, `identify` never returned and the listening page held the
  request open. Now a child that writes nothing for 120 s is killed and
  the file fails: `index` fails it for that run only and tries it again
  on the next, `identify` fails the recording with a message naming the
  storage, and the page answers `503`. Open: listing the library,
  reading a file's size and modification time, and opening the recording
  still block on a hung share; and if a share never answers again, every
  remaining file of an `index` run waits its two minutes on each worker
  (stopping a run after many stalls in a row would be a separate
  decision).
- **Library ignore file.** *(our proposal, 2026-10-09; done in
  session 8, ADR 0009)* A track of a DJ-mixed CD already blends into the
  next one; indexed, it causes false joins and wrong answers (the mixed
  CD's "Synthesis (Remix)", experiments 0049-0053). `.gunfingerignore` at
  the library root, a subset of `.gitignore` syntax with literal square
  brackets and no negation, leaves files and folders out of every command
  that scans the library; `doctor` counts the audio files per pattern and
  the records of ignored files apart from those of gone files, and `prune
  --yes` deletes both. Store-only runs see ignored files until they are
  pruned. The first ignore file on the NAS leaves out Dangerous Drums
  Volume 2 and The Creeps' CD2 (20 and 13 audio files); we run
  `prune --yes` with the NAS configuration, which deletes their 33 peak
  records and 33 tag notes. Open: `2002 - CPT003 - Kemal vs. Rob F &
  Impulse - 256`, the third candidate in the session notes ("Mixed-CD
  tracks in the library"), is not listed; and the clusters' verdict
  links must leave out ignored files (below, "More development mixes",
  step 2). Session 9: the NAS store was pruned, then the index run added a scene copy of
  Dangerous Drums Volume 2
  (`__full_scene/dnb_scene - 2000 - Part 2/2000-08/va-dangerous_drums_2-udfrcd002-2000-sour/`),
  which the root-anchored pattern does not cover, and three mixed CDs
  in the new folders give the 2003 mix a confident wrong title
  (experiment 0063). `scripts/analysis/folder_edges.py` scores each
  folder's track boundaries from the peak records; 33 folders have at
  least 75% joined boundaries, 30 of them new (experiment 0065,
  `work/s9/mixed-cd-candidates.tsv`), a listening list. Session 52
  pruned the five files added to the ignore file on 2026-10-25 and
  2026-10-27 (a track-fix release of a mixed CD, two uploads that hold
  part of another record): the NAS store then held 40,678 records, and
  the NAS-scale baseline was `session-52-linux-nas` (experiment 0231: no
  scored result changed). Session 64 pruned the four uploads added on
  2026-10-30 (each heard to hold more than one recording): the NAS store
  holds 40,674 records, and the NAS-scale baseline is
  `session-64-linux-nas` (experiment 0254: no scored result changed).
- **Database.** Not needed so far: the peak store is one file per asset and
  the index is rebuilt from it in 0.5 s. Revisit if detections need to be
  stored; report edits and the Track/AudioAsset model, the other reasons
  once listed here, are out of scope (ADR 0017).
- **The Track/AudioAsset model.** *(out of scope for the product, ADR
  0017, 2026-10-13)* An asset is a file. We think in tracks (a
  recording, possibly with several rips, edits and remixes). Duplicate
  clusters and related recordings (experiment 0015) are the evaluation's
  view of this. The product was to get its own, editable model, shaped by
  how `review` is used. Our principle rules that out:
  Gunfinger is fired and forgotten, the user owns the tags, and the
  tracklist groups the files found in one run into entries without a
  stored model.

## Reports and command line

- **Time without a confident match.** *(2026-10-17, a proposal from an
  external review; worth doing, our decision, 2026-10-20; the plan is
  `docs/research/2026-10-20-unidentified-gaps.md`; measured in sessions
  34 and 35 in the harness, experiments 0159-0166; the options for
  printing it are `docs/research/2026-10-21-printing-gaps.md`; how to
  print it, our decision, 2026-10-27: by default, gaps of 45 s or more
  to begin with, the mix's start and end included, possible lines kept
  inside a gap, named "ID - ID", with an estimate of how many tracks the
  gap holds; JSON gains the gaps, CSV and the cue sheet do not change;
  an ADR comes with the change; part 1 done in session 55, ADR 0023,
  experiment 0234: the tracklist, the table and JSON list gaps of 45 s
  or more as unnumbered "ID - ID (<length> without a confident match)"
  lines, rows and a `gaps` field; part 2 done in session 56, ADR 0024,
  experiment 0235: each gap whose audio repeats itself gets `; about
  L-H tracks`, the combined estimate of 0165 as a range one either
  side, and `tracks` in JSON; done)* A tracklist does not say
  how much of the mix it accounts for: eleven lines over 56 minutes read
  the same whether the rest is blends or a quarter of an hour of tracks
  not in the library. Step 1 lists the stretches with no confident
  segment, computed from segments rather than whole play spans (so a gap
  inside a returning track counts); it does not claim that a matched
  span is fully explained. At 30 s, the smallest minimum length meeting
  the pre-registered criteria, every missed DJ-length track of the
  development mixes is reached, and about 1 listed gap in 13 of the
  leave-out scans holds no missed track, always time inside a found
  track; at 60 s none does, and every missed track is still reached
  (0159). Step 2 estimates how many tracks each stretch holds. Its
  length over the median interval between the found entries gives a
  range (one track either side, or the gap over the intervals'
  quartiles) that holds the truth for 90-98% of gaps where the mix has
  enough found intervals without a gap, but not a count: exact
  100/53/37/42% at k = 1-4 on the development mixes (an interval taken
  with only the gap's run missing),
  86/53/39/26% on long generated mixes, and no estimate where a mix has
  fewer than 3 found intervals without a gap (0160, 0161). The stretch
  matched against itself (a record repeats its own material) counted
  the long generated mixes' records exactly for 95/80/69/54%, with
  settings fixed before that test (0162). Records that share a passage,
  beatmatched back to back, make one stretch for two thirds of the
  shared-material panel's pairs, and that count then undercounts (0163);
  limited to the length's range, it is the best count measured (64%
  exact, 2.9% off by two) but meets no registered bar for printing a
  count (0165). A gap inside a found track goes to 0 when the gap is
  searched against its neighbours' records, but not always (0164). A
  sustained pitch change of 1-3% within a play makes the search drop
  one side of the play in half of the ridden plays of generated mixes
  (0166), a question for the search itself. Novelty segmentation comes
  only if counting stays out of reach. It fits ADR 0017 (clear output,
  no review step), but whether it is printed by default or on request
  is our decision, and either changes the output ADR 0017 describes, so
  it needs an ADR; the proposal recommended printing the length by
  default and no count yet, and a prototype is on the branch
  `s35-gaps-output`. Since ADR 0023 the product prints the gaps by
  default; the covered time is computed once, in the core
  (`gaps::uncovered`), for the CLI and the harness. Since ADR 0024 each
  gap line carries the combined estimate as a range (n-1 to n+1, at
  least 1; none for a count of 0): `identify` matches each gap against
  itself from the samples it already holds (`gunfinger_core::repeats`,
  which the harness's `self-match` also runs) and stores the stretches
  in the report, so `show` prints the estimate without decoding again.
  What is left belongs elsewhere: records sharing material, pitch rides
  (0166) and a gap holding no missed track, which the estimate cannot
  tell from one that does.
- **Possible plays inside a confident play of another recording.**
  *(done on 2026-10-14, our decision, ADR 0019, experiment
  0107)* The default tracklist printed a possible play lying inside a
  confident play of another file as a line of its own, marked "(possible;
  shares material with X)". One was found in the README's example:
  "Clockwork (Stakka Rmx)" in a mix that plays the original, before the
  remix existed. Every possible line of the three development mixes'
  tracklists at NAS scale was of this kind (9 lines, none a played
  track). Such plays now leave the tracklist; the table, CSV and JSON
  keep them, marked. The mark tests the confident play's passages (its
  segments joined across gaps of at most 10 s) instead of its whole span,
  which also marked 6 right brief plays of the generated mixes lying
  between two parts of a returning track, and skips a play of the track
  playing (one that lines up with the confident play or carries its
  artist and title), which joins that entry. 7 of the 9 lines leave; the
  two Klute Remix lines line up with the VIP and join its entry under
  `titles disagree:`. Measured on every stored development report: no
  right play lost. The cost ADR 0019 names (a played record found only
  as possible inside a passage of another) was measured in session 39 on
  generated double drops: "The printed tracklist" below. Session 48
  (experiments 0215-0218) measured features from the library's own
  evidence: whether the marked play's file and its partner's share any
  alignment of 60 hits or more by `pair` (R1) prints every record
  playing under an unrelated partner and lets in no shared material, on
  fresh seeds at both scales and in the development mixes' reports.
  Whether the tracklist prints such plays is ours to decide, with an
  ADR.
  Session 65 (experiments 0255-0258) measured the upper bound of
  finding such a record by taking out the one over it (proposal 4 of
  `docs/research/2026-10-29-matching-and-archive-proposals.md`): with
  the generator's time map and the played audio as the reference, a
  masked residual confirmed nearly every quiet and brief partner with no
  wrong confident answer at 262 tracks; with another rip as the
  reference, the case of a real mix, fewer, and at NAS scale it once made
  a known related version of the record under the masked one confident
  (see the proposals note). A partner found only in a residual would
  still be a marked play here.
- **Report edits.** *(out of scope for the product, ADR 0017, 2026-10-13;
  kept as the history of the listening tools, which stay development
  tools for evaluation ground truth)* Confirming, rejecting or renaming
  plays in a report, and
  keeping those edits when the mix is identified again. `listen` and
  `review` work with real audio output (by ear, 2026-10-06). Judging by
  ear now settles most open questions: wrong tags, versions, cluster
  joins and useful possible plays. A first step smaller than a TUI is a
  pair review: play two library files aligned at their fitted speed
  (`gunfinger-eval pair` finds the alignment), show where they agree and
  where they differ, and write the verdict to `docs/pair-verdicts.txt`.
  Decisions stay apart from measured evidence, survive identifying a mix
  again, and keep the original asset names visible. Session 10 built the
  first step as files: `scripts/analysis/listening_pack.py` writes, per
  pair, a sheet and FFmpeg clips (each file in turn, and both aligned in
  stereo) under `work/listening/`, in listening order. Session 11 added
  `gunfinger-eval pair-review`, which steps through the pack's unjudged
  items, plays their clips and appends the chosen `same` or `different`
  line to `docs/pair-verdicts.txt`. Session 14 added `gunfinger-eval
  audition`, a local web page that plays a question's files aligned on
  one timeline (switching at the same moment, seeking, looping, speed
  matching) and takes the answer on one key: the pack's pairs (verdicts
  to `docs/pair-verdicts.txt` as `pair-review` writes them), release
  folders, and the plays of a saved report (`questions detections`,
  confirmed, wrong or unsure, to the answer log
  `work/audition/answers.jsonl` only). The planned next step, those
  detection answers turned into edits of the report, is not done: nobody
  reviews the output of a tool that is fired and forgotten (ADR 0017).
- **No record inside the track length range.** *(done in session 10)*
  With a store whose only records lie outside `--min-track` and
  `--max-track`, `stats`, `identify` and `explain` said "no indexed
  assets ... run `gunfinger index` first", though the files are indexed
  and only the range leaves them out (seen in session 9 with an
  18-minute file and the 17:00 default). They now say that the range
  leaves out every file with a peak record, how many and how long, and
  how to widen it.
- **Titles of mislabelled files.** *(first step done in session 11)* A
  play takes the title in its best file's tags, so a mislabelled file
  gives a wrong title on a right match: track 2 of the 2003 mix is printed
  as "Future Cut - Sex Drive" (experiment 0061). A confident play now
  names the confident plays of other files over the same passage whose
  tags give another title (`other_titles`; "titled X; other files here:
  Y (play N)"; "titles disagree:" in the tracklist), without choosing
  one. That is where it ends: the product flags disagreeing titles and
  never chooses one, and the user owns the tags (ADR 0017). Nothing
  further is planned here.
- **A full-screen TUI.** *(replaced in session 14, ADR 0016)* `review`
  steps through a report by ear in a line loop. Browsing detections
  against the mix's waveform is now `gunfinger-eval questions
  detections` and `audition`, a local web page, which needs no terminal
  UI or audio crate and can switch files at a sample-exact moment.

## Matching

- **A minimum span instead of 3 windows.** *(undecided, 2026-10-27: we
  come back to it later; session 29's question below stays open)*
  Whether a 15-20 s play is
  confident depends on where the 10 s windows fall: 76 of 160 grid
  positions at 15 s, 148 at 20 s; 200 hits over at least 10 s between the
  first and last hit makes all of them confident and changes nothing in
  the sweep or the development scans (experiment 0020, offline). It lowers
  the shortest identification from about 25 s to about 11 s, which is our
  call. The default matcher's link rules (experiment 0030, ADR
  0008) keep chance lines from lengthening the span; measure the minimum
  span with them (the grid's 15 and 20 s plays: 67 and 146 of 160
  confident under the default, experiment 0047). The span from the
  first to the last hit counts gaps as evidence; compare it with the
  time that hits support (for example, the seconds holding aligned
  hits), including sparse alignments and repeated breaks. `pair` now
  reports that supported time; for whole files it orders pairs
  differently from coverage (experiment 0057). Keep the hit
  threshold fixed to isolate the change. Success: fewer level changes
  caused only by where a play falls on the window grid (silence put
  before the same audio should not change its level), more brief plays
  confident, and no new confident error on the shared-material challenge
  set below, which comes first.
  *(session 27, experiments 0136-0142)* Measured under today's matcher
  with the span (A) and the supported time (B) at 10, 15 and 20 s, the
  hits fixed: no variant meets the criteria written first. A(10), B(10)
  and B(15) make 803, 947 and 800 of the grid's 960 queries confident
  (today 693) with 2, 13 and 0 flips (today 30), and change nothing in
  the sweeps, recall or the development scans at 262 or NAS scale; but
  they, A(15) and B(20) make three 20 s queries of the first shared
  panel newly wrong. A(20) adds no wrong answer and names fewer. Those
  queries are files of exactly 20 s, so they hold two windows and
  today's rule can never be confident on them; after 3, 5 or 7 s of
  silence, today's rule names the same wrong files every time, and one
  more (0142). Adoption is our decision and needs a new ADR; a
  fair shared-material test places brief passages at several offsets,
  as the grid does.
  *(session 29, experiments 0146-0148)* The fair test: every shared
  passage of 30 s or less on both panels' development halves after 0-9 s
  of silence (1,880 searches), against criteria written first (F1: no
  more wrong answers than today summed over positions; F2: none in a
  passage today's rule never gets wrong). No variant meets F and the
  grid criterion together. A(20) and B(20) meet F but name fewer brief
  plays. A(10) and B(15) fail F1 by 9 and 6 of 168. B(15)'s 6 come from
  three 20 s passages, two indexes each, placed exactly on a window
  border, where today's rule cannot be confident; today's rule names
  the same files at 8-9 of their other positions. On session 28's
  brief plays at NAS scale, which test recall only (no wrong detection
  there reaches 240 hits), 15 s plays become confident wherever they
  start: 519 (A(10), B(10)) and 504 (B(15)) of 522, against 228 today. We decide whether a wrong answer that
  today's rule avoids only on the window border counts as new; if it
  does not, B(15) is the candidate in the session's reading after the
  counts, with a new ADR and the unsearched validation halves as its
  confirmation.
- **Speed that wanders within a play.** *(done for pitch rides, session
  53: ADR 0021, experiment 0232. Our decision of 2026-10-27, since DJs
  speed a set up while a track plays, because the set is continuous:
  the default matcher's second suppression drops an overlapping detection
  of the same file only within 0.6% of the stronger one's speed, so both
  sides of a ride are kept; the old rule is the harness's
  `--overlap-by-spans`. What stays open below: wander within one side
  (`--speed-per-stretch`) and rides inside short plays, a question for
  the window condition. Session 57 ran the protocol again under it
  (0236-0240): the detections it keeps more than 0.6% apart of a
  recording not playing are weak or possible everywhere (at most 55 hits
  for a related version in the shared panels, 44 in the drops, 67 in the
  ridden mixes), so a related version's second alignment has not reached
  240 hits; the confident ones it keeps are of the recording playing (the
  other sides of rides, Into Music in the 2003 mix), and `robust`'s rides
  gain 3 and 4 confident excerpts.)* The second pass analyses a play at
  one fitted speed; Star Trails plays 0.1-0.3% above it in many windows
  and loses 15% of its hits (experiment 0024), and within-play speed
  varies by ±0.2% in the development mix (experiment 0022). The opt-in
  `--speed-per-stretch` fits each stretch of 3 windows: Star Trails +11%,
  a 2% pitch ride +9%, no measurable CPU, no level changes in any set
  (experiment 0032). Wow is too fast for it. Adopt if a real play near
  the 240-hit rule needs it. Under the default matcher (ADR 0008) it is
  one field of `Matcher::Fitted`'s options; rerun the protocol.
  *(sessions 35 and 37, experiments 0166 and 0175-0178)* A sustained
  pitch ride costs more than hits: a ride of 1% or more splits the play
  into two chains, each confident after the second pass, and the second
  pass's 10 s margin widens both into the ramp, so `strongest_per_moment`
  drops the weaker side whole (0175). Rides of 0.25%, or 0.5% over 10 s
  or more, lose nothing; at 1-2%, and 3% over 2-5 s, 21-43% of the sides
  lose 30 s or more, at 3% over 10-20 s 4% (0176). The development mixes
  hold candidates: Star Trails' slower side
  (13:13-14:37 in the Stakka mix, 306-315 hits) is dropped at both
  scales, and Into Music's middle segment in the 2003 mix (candidates,
  not checked by ear). Of the existing options only `--single-pass`
  keeps both sides (0177). The opt-in `--overlap-by-chains` judges the
  overlap on the first-pass chains: on new seeds it keeps all 183 lost
  sides at no measurable CPU, with no new wrong confident detection on
  the sweep, the development scans and leave-outs and the generated
  mixes; on the first shared panel no query gains a wrong file, but a
  wrong version already confident in two Fire queries gains a second
  segment (4 wrong confident detections more), so its registered V2
  fails by its letter (0178). Adopting it is our decision and needs an
  ADR superseding ADR 0008's consequence for overlapping parts, including
  how a play shows its overlapping segments and whether a wrong file's
  extra segment is a new wrong answer.
  *(session 38, experiments 0179-0180)* The narrower opt-in
  `--apart-in-speed 0.006` drops an overlapping detection only within
  0.6% of the stronger one's fitted speed. Under criteria committed
  first it keeps all 257 lost sides on new seeds, with no new wrong
  confident detection and no lost right one on the sweep at both scales,
  the 11 development reports at NAS scale, the generated mixes and the
  first shared panel, where nothing confident changes (0179; that panel
  was also the threshold's development data). It keeps
  Star Trails' and Into Music's dropped segments. In the sweep's 30 s
  excerpts (`robust`, 0180), a ride costs recall from 2% when it lies in
  the middle: each side spans two windows, so the 3-window rule decides
  (2% at 15 s: 43-49 of 80 confident; 3%: 3-8), and the overlap rules
  win back 1-3 excerpts; 1% in the middle and 2% at 24 s cost nothing.
  The options, the risk and a recommendation are in
  `docs/research/2026-10-22-pitch-rides-decision.md`; adopting it is our
  decision and needs an ADR.
  *(session 45, experiments 0209-0211)* The confirmation on data the
  threshold had not seen, under criteria committed first: on the second
  shared panel's development half at NAS scale no confident detection
  changes in any of 440 query and index pairs; on session 39's double
  drops (seeds 2026 and 2028) no wrong confident detection is added and
  no right play lost, and the CLI's reports and printed tracklists are
  identical to the default's at 262 tracks and at NAS scale; on three
  beatmatched seeds nothing confident changes either. The variant is
  confirmed by the registered reading (a no-harm confirmation: no ride
  in this data, so recovery rests on session 38's seeds). What it changes there is weak
  or possible, at speed differences from 0.60% up; on the drops a
  related version gains weak second alignments 0.61-1.51% apart without
  any ride, so against shared material the variant relies on the
  240-hit rule's margin, not on its speed condition (the decision note's
  "Session 45").
  *(session 53, experiment 0232)* Adopted as the default (ADR 0021).
  In the standard evaluation and the three development scans it keeps
  Star Trails' slower side and Into Music's middle segment and adds or
  removes only weak detections; calibrate and the printed tracklists
  of the three development mixes do not change.
  *(session 49, experiments 0219 and 0220)* Copies of one recording drift
  against each other as well: the CITRUS 003 upload of Falcon - The Stand
  speeds up by 0.7% over the track against the vinyl rip, so against
  another upload one fitted speed leaves its first 30 s without an
  alignment and the two files part by up to 0.5 s on the listening page
  (the listening's "they diverge over time"; trailing silence moves no
  fit). `pair --fine-rungs` searches rungs 0.1% apart within 1% of the
  fitted speed and follows such a copy.
  *(session 63, experiments 0251-0253; the decision note
  `docs/research/2026-10-30-second-pass-fit-decision.md`)* The second
  pass's speed estimate (`chain_speed`), measured against three opt-in
  fits (`--second-pass-fit`): the best of 41 speeds within ±0.4%, the
  offset refitted (the grid); the same chosen on alternate windows and
  counted on the others (the split); two rounds of the hits' drift (the
  bounded fit). On 30 s excerpts the chain's fit lies within 0.003% of
  the played speed and few answers change (1-3 per fit, all against
  other rips). On 15 s plays, whose chains give the mean of their rungs
  (0.049% off), each fit gains 18-27 confident answers in `recall` (most
  against other rips) and loses 1-3.
  The grid lifts wrong versions too (calibrate's strongest false +2% at
  NAS scale and +10% at 262 tracks; 27 related or other detections newly
  confident in the panels); the split and the bounded fit do not choose
  on what they count, but under the split a related version that shares
  the played passage crosses the rule as a true play does, and the
  bounded fit loses 4 confident answers and lifts the Clockwork remix's
  shared passage by 7% at 262 tracks. Under 0242's perturbation the fits
  hold Echos's short chain at 602-603 hits where the default moves from
  503 to 455. Not adopted; a
  bounded fit for short chains only would be the candidate to measure,
  beside the minimum-span question.
- **The edge of the ladder.** Recall stays 100% to ±8.4% and is gone at
  ±9% (experiment 0033). Three extra rungs at each end (`--extra-rungs
  3`) recover every speed to ±9% for 14% more search CPU and change
  nothing else measured. The limit applies to the mix's speed relative
  to the indexed file, not to the deck's pitch. Copies of one recording
  differ in speed: of the 872 pairs of files joined in the NAS clusters
  (experiment 0049), 180 differ by more than 2%, 17 by more than 4%,
  and one "Speed Up" upload by 7.8%. A play at +5.65% (the test mix's
  fastest) against a copy 4% slow is at +10%, out of reach when that
  copy is the only one indexed; which copy runs at the record's true
  speed is unknown. Measure before deciding: a sweep that leaves out the
  excerpt's source file and its identical copies but keeps its other
  rips, so that misses beyond the ladder and misses from lost evidence
  can be told apart. Then choose between a wider search, a targeted
  one, or none. Measured in session 7 (`sweep --other-rips`, experiment
  0059, 1,305 queries at NAS scale): recall holds to 9% of relative
  speed and is gone from 10.2%; `--extra-rungs 3` adds 9.2-9.5% only.
  Inside 9% every miss is a weak copy (lossy uploads, one other vinyl
  rip) found below the rule. Of the 233 NAS clusters, 7 span more than
  4% in speed (Falcon - The Stand 11.5%, its "Speed Up" upload against a
  slow rip). A play is lost only when the deck's speed and the
  remaining copy's offset add up past 9%, so the choice depends on how
  far the decks go beyond about ±5%. Replay speed also changes
  by accident when analogue media is digitised (Six & Leman 2014, §1).
  *(session 49, the clusters file of 2026-10-25, no new run;
  `scripts/analysis/cluster_speeds.py`)* Of 284 NAS clusters, 75 span
  more than 2% in speed, 12 more than 4%, 1 more than 6% and 8%: Falcon -
  The Stand at 11.5%, its "Speed Up" upload about 11.6% fast against the
  vinyl rips (`pair` finds 6 hits between them: beyond the ladder; the
  two join only through the CITRUS 003 upload, 7.8% from the one and
  3.5% from the other). With a
  deck at ±5% and the ladder at ±8%, 50 clusters (each spanning 2.9% or
  more) hold two copies of which one, indexed alone, would miss a play
  of the other at a deck extreme. 38 copies have no other copy within
  the ladder when played at one extreme (36 at +5%, 2 at -5%); 35 of them
  are uploads of one channel that runs 3-5% fast against the vinyl rips.
  Each of those clusters also holds a rip outside the upload folders
  that, played at either extreme, has another copy within reach. Copies differ
  in speed within a file too: the CITRUS 003 upload of The Stand runs
  3.1% fast against the vinyl rip at its start and 3.8% at its end
  (experiment 0219).
  *(session 51, experiments 0226-0230; the decision note
  `docs/research/2026-10-25-ladder-edge-decision.md`)* Measured today
  with `--extra-rungs` 1, 2, 3 and 5 (86-102 rungs against 82): each
  rung pair moves the reach by 0.4% (8.4, 8.8, 9.2, 9.6, 10.4%), the
  same against the full index and against only the other copies the NAS
  library holds, and costs 4.3-5.3% more search CPU (Stakka scan at NAS
  scale ×1.046 to ×1.229; three pairs +14%, as in 0033). Calibrate
  and the development scans do not move. Two single cases decide the
  registered criteria: one rip near -8% loses its confident answer under
  every wider ladder (fitted 0.1% further off; that the outer rung moves
  the fit is inferred), and from two extra rungs a bootleg judged a
  different mix of a held-out recording is reached at one more speed
  (seed 2027: China Cup against ƆƐI3C's upload, judged different; a
  same-title credit since ADR 0022, experiment 0233). For
  a deck at ±5%, the copies with no other copy in reach fall from 29 at
  today's reach to 19, 7, 3 and 1. A targeted
  search (extra rungs only where nothing is confident; 2% of the Stakka
  mix) is estimated at under 1% more CPU; with one extra rung pair it
  passes the registration, as a variant to build and measure.
  *(session 63, experiments 0251 and 0252)* Siren Bass's upload at -8%,
  lost under one extra rung pair (214 hits at -8.18%), reaches 241 hits
  at -8.25% under a local speed search (the grid), 237 when the speed is
  chosen on other windows (the split) and 230 under a bounded fit: a
  better speed puts it at the rule, not clearly above it.
- **Peaks across frequency bands.** Anchors at 2-4 kHz are 64% of the
  postings the development mix's plays look up and 21% of their true
  hits. A profile whose peak neighbourhoods widen with frequency (variant
  a, experiment 0034) has 9% fewer postings, gives the mix 34% more true
  hits and raises the margins under both matchers, but related records
  (remixes, the same artist) reach the possible tier more easily. We
  decide; it means extracting every peak record again, and one
  test evaluation should confirm it. Measure its search cost at scale.
  Before extracting the NAS records again, test it under the default
  matcher on the shared-material challenge set below. Wang (2003, §2.1)
  picks peaks by a density criterion so that the time-frequency plane
  is covered evenly; the band profile asks the same of each band.
- **Detection boundaries.** Boundaries are the first and last aligned hit.
  In generated mixes they lie within 0.5 s of hard cuts, and start or end
  a median 2.7 s inside crossfades and bass swaps (experiment 0020); shared
  kicks once extended a synthetic detection 26 s past its track
  (experiment 0008), and linked chance lines up to 30 s (experiment 0020).
  Trimming weak chain ends (`--trim-ends`) removed the overshoot but cut
  2.6 s of true play per second removed (experiment 0031); the link rules
  of 0030 fix the overshoot at a tenth of the cost. Crossfade edges remain
  a median 2.7 s inside the truth. Gunfinger reports the matched
  interval, not the query window: in a comparison of fingerprinters on
  DJ mixes, crediting each answer to its matched interval instead of the
  whole 20 s query raised specificity from 0.56 to 0.80 (Sonnleitner et
  al. 2016, §6). Per-second scoring (Evaluation) would measure how
  accurate the boundaries are. The second pass measures up to 10 s past
  a chain; at a pitch ride that moves both sides' facing ends a median
  4 s into the ramp, so that one side overlaps the other (experiment
  0175).
- **Where search time goes.** Measured on an idle machine (experiment
  0028): at 262 assets 80-88% of search CPU analyses the query on 82
  rungs; at 26,462 assets 77-87% sorts each window's hits by asset and
  offset (the offset histogram of Wang 2003, §2.3, found by sorting).
  Any correct sort gives identical lines, so grouping hits by
  asset before sorting each asset's few hundred would keep detections
  identical; not tried. Skip at 240 is 3.5× cheaper at 31,964 assets.
  Profiled at NAS scale in session 11 (experiment 0075): with the saved
  index, a NAS `identify` of the 2003 mix is 96.5% search, and the
  default matcher spends 82% of the search's CPU sorting hits (query
  analysis 9.5%, clustering offsets 4.5%, posting scans 2.5%). *(first
  step done, merged after session 11)* The pre-release audit sorts a
  window's hits by asset first, then each asset's offsets, when the
  window holds more than 50,000 hits (experiments 0080 and 0083), and
  extracts key-locked peaks once per rounded hop (0082). The same NAS
  `identify` then takes 181 s instead of 220 s, with the same
  detections (experiment 0084). *(session 12)* Profiled again (0085):
  sorting still took 78% of the CPU, two thirds of it grouping 0.6-2.0
  million hits a window by asset. A counting sort by asset (asset numbers
  are dense) now groups them in two passes, with hits found as 8 bytes
  (0086): the NAS `identify` of the 2003 mix takes 86-92 s instead of
  167-171 s, half the CPU, the same memory, byte-identical reports. What
  is left (0088): sorting each asset's 30-60 offsets 39%, grouping and
  building lines 32%, query analysis 19%. *(session 15)* Of the assets
  with 3 hits in a window, 1.8% can form a line at NAS scale: counting
  their offsets in 4-frame buckets first skips the other sorts exactly
  (0093). The same `identify` takes 65 s instead of 93-97 s, 37% less
  CPU, the same memory, byte-identical reports. Sorting is now 3.3% of
  the CPU; grouping and building lines 51%, query analysis 30%.
  *(session 17)* A window's hits are now taken from the posting lists
  and grouped 4,096 assets at a time, in cache (0096); peak picking
  tests its four nearest neighbours without branches (0097); the store's
  record headers are read on `--jobs` threads and the mix decoded while
  the index loads (0098). The 2003 mix at NAS scale on the idle machine:
  44.1-44.3 s instead of 61.0-62.1 s, 22% less CPU, 10% less memory,
  byte-identical reports. What is left (0096): the pre-filter about 22%,
  grouping a part 20%, reading the postings 16%, query analysis 26%. A
  first `index` is 86% FFmpeg's decoding and resampling (0099), whose
  options are fixed. This comes before any change of hash design;
  `regress` must show identical detections. *(2026-10-14, unverified
  proposals from an external review; our tracklist item, which
  came first, is done: ADR 0019)*
  Integer phases per query anchor for the pre-filter (exact; 0103's
  split profiles suggest the slot arithmetic may be roughly 10% of CPU,
  an estimate to verify, not a ceiling; the counting remains), the FFT
  lengths of the turntable rungs (a diagnostic first, nearest-even as
  proposed beside a factor-aware rule; a change of length changes
  output), and posting-list part boundaries computed once (exact; part
  of `take_part`'s 10.5%), each with its tests, stop points and proposed
  criteria in `docs/research/2026-10-14-matching-proposals.md`. *(session
  19, at 38,105 records)* Integer phases made the pre-filter and its sort
  about 13% cheaper against the unchanged analysis, below the note's 20%
  stop point, and were not kept; by inference the filter's cost is its
  counting and the hit loads, not the slot arithmetic (0108). The FFT
  lengths, measured as a diagnostic (0109): plan construction is 0.04%
  of the analysis; a window within two samples of today's chosen for
  small prime factors
  makes the first pass's turntable analysis 36.5% faster on one thread,
  and a build using such lengths, for timing only, took 5.4% less wall
  time with both playbacks and 9.2% less with turntable playback (the
  fastest lengths measured: 7.6% and 11.1%; timed under other load). A change
  of length changes peaks and output: a retrieval study first, and our
  decision; the study now includes the shared-material panel's
  development half ("A confidence statistic for shared material").
  *(session 21, 0117)* On that half, at NAS scale, session 19's
  first-pass variant (windows within two samples, small prime factors)
  adds no wrong confident query and loses no right file: one wrong title
  added (239 to 240 hits) and one removed (240 to 125), in each index.
  The rest of the retrieval study is not run.
  *(session 22, 0119)* Part 1 of the retrieval study, every panel the
  harness has: no wrong confident answer added; every confident sweep
  answer and every identified track of the development mixes kept; the
  key-lock-only control byte-identical. Answers at the rule's edge move
  both ways: `robust` loses 20 right confident answers and gains 24, a
  few files cross between possible and confident (other rips, the recall
  panel), and 9 possible answers are lost and 7 gained (grid, `robust`,
  other rips). Apart from that, two starts in the generated mixes end up
  further from the truth by more than the 1.0 s tolerance. So the criterion
  "no right confident or possible file lost" and the boundary criterion
  fail. Part 2 needs
  harness work (a list of sweep speeds, sub-hop offsets, the first
  pass traced, a first-pass call path), then timing on an idle machine;
  adoption is our decision.
  *(session 20)* Split out of `take_part`, the binary searches are
  1.3-1.4% of CPU and the copying of hits 9.1% (0113): under the note's
  5% stop point, so part boundaries computed once are not pursued.
  *(session 33, 0158)* On the Linux machine (16 cores, 32 hardware
  threads) a NAS-scale leave-out scan and a 48-query clusters sample took
  about 30% less time with 16 workers than with the default 32, with
  identical results; a full clustering run at 16 workers is not measured. Whether the default should count cores rather than
  hardware threads is our decision.
  *(2026-10-21, between sessions; `docs/research/2026-10-21-low-bandwidth-search.md`,
  experiment 0167)* Two design agents read the search for a machine
  with low memory bandwidth (this one reads 26-27 GiB/s sequentially and
  9-14 GiB/s at random, with 105 ns latency, perhaps memory at a JEDEC
  default; `sudo dmidecode -t 17` would tell). The posting reads are
  about 4 GB/s and not the limit; each hit's 40 bytes of cache traffic
  in `take_part`, `group_part` and the pre-filter are, once 16 workers
  per core complex overflow its 32 MiB L3. Two changes in `take_part`,
  measured with identical reports: finding each part's end by a doubling
  search from the front of the list, and parts of 1,024 assets. The
  Stakka scan at NAS scale searches in 38.5 s instead of 76.7 s at 32
  workers and 45.7 s instead of 49.3 s at 16; a 48-query clusters sample
  takes 121 s instead of 211 s. With them, 32 workers are faster than
  16, so the default need not change. Not in master: a session adopts
  them with tests and the regressions, then narrower hits (4 or 6
  bytes), grouping without the `part` buffer and reading repeated hashes
  once, each after a profile with `perf`. Posting compression is not the
  answer to speed here (3.06-3.12 bytes a posting in a sample), only to
  memory and disk.
  *(session 36, at 3600 MT/s since 0168)* The two changes are in master
  (0169): the Stakka scan's search 45.3 s instead of 47.2 s at 16 workers
  and 37.9-38.0 s instead of 49.0-49.2 s at 32, the full NAS clustering
  run 23:02 (52:30 in session 33 at 2133 MT/s), every report and the
  clusters file byte-identical. Timers in a diagnostic build replace the
  profile (0170): the pre-filter is 43% of the search's CPU at 16
  workers, grouping 28%, query analysis 14%, taking parts 6%, sorting and
  lines 6%, the second pass 2%. Narrower hits (4 and 6 bytes) and
  grouping straight from the lists were slower by 4.4-8.2% at 16 workers
  (0171, 0172), and repeated hashes are too few to be worth reading once
  (0173). By inference, what is left is the pre-filter's and the
  scatter's work per hit, not memory traffic.
  *(session 41)* Within the pre-filter, the slot arithmetic (a call to
  `floor` per hit on x86-64) was 37-46% of its time and the `touched`
  list 16-27% (0189). Each grouped hit now holds its integer offset,
  its frame minus its anchor's phase `floor(frame) mod 2^16`, so the
  filter reads only the grouped hit and counts without floating point,
  and it clears its counts by reading the hits again (0190, 0191; exact
  by the argument in `OffsetSlots::may_form_line`, tested against
  `densest_clusters`, and agreeing with the old filter on at least 90%
  of random sets). The Stakka scan's search at
  NAS scale takes 26.7 s instead of 37.6-37.9 s at 32 workers and 32.5 s
  instead of 45.4-45.6 s at 16, the clusters sample 1:21 instead of
  1:57, `identify` of the Stakka mix at NAS scale 19.8 s instead of
  25.1-25.2 s, every report identical (0192). Counting assets while taking a
  part was not faster (0190). Grouping a part is now the largest
  stage, 40% of the first pass's CPU at 16 workers and 35% at 32; the
  pre-filter and query analysis 20-22% each (0193).
  *(session 43)* Grouping's scatter costs about 4 ns a hit at 32
  workers whatever the part size (0199); 40% of hits have the asset of
  the hit before them, and in a microbenchmark such repeats make the
  scatter about 0.7 ns a hit dearer (0201). Taking hits straight into
  a room per asset, a staged cache line per asset, and ranks kept by
  the count pass all missed the 3% stop point at 32 workers (0200,
  0202; ranks cut 3.4-4.3% at 16 workers, where, by inference, no
  second thread fills the waits they remove). In master: the
  pre-filter counts each pair of neighbouring slots in a byte and adds
  a hit to its two pairs with one 16-bit word, exact by the argument in
  brief 43 and tested against the old filter (0203); the search 26.0 s
  instead of 27.2 s at 32 workers and 31.6 s instead of 32.6 s at 16,
  every report identical (0204). Where the CPU goes now: 0205.
  *(session 46)* Query analysis at 32 workers split FFT 37%, log power
  30%, the four-neighbour test 19% and the neighbourhood test 12%; plans,
  reused analyses and rounding were each under 1%, and an `x86-64-v2`
  build was not faster (0212). In master: the peak picker compares
  powers, which order as their levels do since `decibels` never
  decreases as power grows (tested over every non-negative `f32` where
  the tests run), takes levels only for bins that pass every test on
  powers and for refinement, and tests the four nearest neighbours in
  one loop over equal slices (0213's C4; neither half passed the 3%
  stop point alone). The search 24.1-24.3 s instead of 26.1-26.2 s at
  32 workers and 29.1 s instead of 31.8 s at 16; `identify` at 262
  tracks with both ladders 6.5-6.7 s instead of 8.5 s; every report and
  the stored peaks identical (0214). The analysis is now 13-15% of the
  first pass; grouping 40-46%.
  *(session 60)* Rejecting assets before the scatter (proposal 2 of
  the 2026-10-29 note): a table of folded pairs per asset of a part,
  counted from every hit before grouping, flags each asset the
  pre-filter would pass (proved and tested), and only flagged assets
  are grouped; lines identical in every replayed window and report.
  The fewer folded slots, the more it passes: with 64, 256 or 1,024
  folded slots it flags 91%, 54% and 12%
  of the NAS-scale assets with 3 hits (97%, 70%, 21% of their hits;
  the filter passes 2%; 0243). The 1,024-slot table (1 MiB a worker)
  made the search at NAS scale 31-32% slower at 32 workers and 12-13%
  at 16 (on the Linux machine; the Mac not measured): the sketch's own
  pass cost 300 s of CPU, more than the filter and grouping it spared
  (170-173 s), and by inference its table evicts the
  lists and parts, so taking cost 50 s more (0244). Not adopted.
- **Behaviour at scale.** *(larger library; session 50: copies of
  records do not move candidates or answers, 0222-0224; session 57:
  calibrate over seeds 2026-2029 and the Stakka scans at 40,678 records,
  weakest identifying 675, strongest false 282, 7 accepted, all Cool Rock
  Stuff, 365 false candidates at 30 hits or more, 0239)* The confidence rule (240
  hits in 3 windows for the default matcher, 200 for `--single-pass`) and the possible tier
  (60 hits) were calibrated against 262 tracks. Measured since at 1,200,
  3,200, 9,200 and 27,000 assets of the NAS collection (experiments
  0049-0052): recall holds, the confident rule holds against everything
  but a few pairs to judge by ear, and matches between different
  recordings pass the possible tier from about 9,000 assets. Every one
  checked since session 10 shares material ("The possible tier at NAS
  scale"), so chance has not yet been seen above the tier; the
  strongest match not checked has 41 hits. Wang (2003, §2.3.1) sets the
  threshold from the score distribution of the highest-scoring wrong
  track, which depends on the number of tracks. Recalibrate at each new size
  (`docs/calibration.md`): strongest false candidates, weakest true
  detections, the possible tier's use, query time and peak memory, on the
  saved sweep panels (seeds 2026-2029) and the normal protocol. At each
  stage, repeat the related-recordings census (experiment 0015; a
  self-match from stored peaks is cheap, experiment 0023) and check
  whether any two different recordings reach the confident rule.
  The reversed-copy proxy shows chance growing slowly up to 21,109 assets
  (experiment 0012), but shared breaks and remixes already give 130 hits
  between different recordings in this library, so the new tracks must
  include jungle, hardcore, remixes, versions and producer catalogues that
  reuse material.
- **The filter and famous breaks.** *(larger library)* As breaks such as
  the Amen and Think become common, their hashes should move into the
  fullest lists. Check whether skipping them then hides break-heavy tracks
  from the search for candidates; the second pass counts those hashes
  again, so a candidate's evidence does not depend on the filter, but
  whether it becomes a candidate does. The skipped set changes with the
  library; if candidates go missing, try weighting hits by the rarity of
  their hash instead. *(2026-10-17, a proposal from an external review)*
  The skipped share is ranked by posting-list length, which counts every
  copy of a recording, so a collection with many rips of the same
  releases could move a track's own hashes into the skipped lists
  without adding new music. Search fixed development queries against
  indexes padded with exact copies of existing records (the scale proxy
  of experiment 0012 adds reversed copies), and trace which files become
  candidates apart from the second pass's counts. At 40,126 records with
  real duplicate rips, other rips of the recall panel stayed confident
  (175 of 180, experiment 0144), so this checks stability rather than a
  known failure; if it fails, rank hashes by the number of distinct
  files that hold them.
  *(session 50, experiments 0222-0225)* Measured with exact copies added
  to the index (`sweep --exact-copies --copied --trace-candidates`): the
  filter is stable. At 262 tracks, nine copies of every file of the
  sweep's own recordings (a third of the index) move 27% of the lists
  set aside towards them and cost a query's first pass up to 23% of its
  own hits (99 copies: 28%), but no query stops being a candidate or
  loses its confident answer, and the second pass's counts stay within
  1% (0.995-1.009 of today's).
  At NAS scale (40,580 files) the set aside follows the library's
  proportions: nine copies of the queries' 157 files move 0.9-1.4% of
  the lists, six copies of a random tenth of the library raise the cut
  by 60% and move 2%. The one effect seen is through the chain's span:
  when a rip's first-pass chain loses a window, the second pass measures
  less (Siren Bass, other rips only: 298 hits instead of 363, still
  confident; also when only other files are copied). Ranking by distinct files or by recordings (clusters)
  costs nothing at level 0 and keeps a query's first pass within 3% of
  today's under any padding (recordings: unchanged by copies by
  construction), but with no failure to remove it is not worth an ADR.
  The drum-loop pairs judged `different` (Wikki Wikki Plate ~
  Defjammer, the Klute VIP ~ remix, the Aphrodite family) share
  1,793-3,850 hashes, of which 23-35% are set aside, against a median
  42% for random pairs: the filter keeps most of what a shared loop
  brings (0225). Still open: a famous break held by many records, and a
  real collection's duplicates beyond exact copies; the 65,536-asset
  limit capped the padding at NAS scale (ADR 0010).
  *(session 59, experiment 0242)* A candidate's evidence does depend on
  the filter through the speed and span the second pass starts from, and
  so on the rest of the library (ADR 0014). Session 52's prune of five
  files moved one list into the set aside of recall's index, a
  9,930-posting list that holds no pruned posting; by inference it was at
  a tie at the cut, and its move cost one line one hit in recall's brief
  query 911 (Echos at +1%). The query plays between two rungs, so its
  middle window has two near-equal lines, at +1.2% and +0.8%; the +1.2%
  line lost the hit, `distinct` kept the +0.8% line at a tie (by
  inference), and the chain's speed, a hit-weighted
  mean of three lines, moved from +1.147% to +0.933%. The second pass,
  which measures at that speed, counts 455 hits before and 503 after;
  both detections are confident and right. With the old index's lists
  set aside, today's postings give the old recall reports exactly
  (with `--overlap-by-spans`, the pruned files' own detections set
  aside). This
  is expected under ADR 0014, not a defect. Not measured: how often a
  short chain's speed hinges on one hit, and whether fitting the speed
  in the second pass, or weighing both rungs' lines, would make it
  steadier.
- **A confidence statistic for shared material.** *(larger library;
  session 50: the first pass's filter does not hide the drum-loop
  pairs' shared hashes more than chance ones, 0225, so their evidence
  reaches the second pass as before; session 57: both panels' development
  halves give the same confident detections by role under ADR 0021 and
  the prune, 0236)* If
  identifying and shared-material evidence overlap at scale, raising the
  hit threshold would lose brief plays without resolving the ambiguity.
  It has started: the second pass measures the passage the Clockwork remix
  shares with the original at up to 119 hits, which set its rule at 240
  (experiment 0026). The band profile of experiment 0034 makes it more
  pressing: related records (remixes, the same artist) reach up to 50
  hits from audio not in the index, near the possible tier.
  At NAS scale it is the main open accuracy question (experiments 0051,
  0053). By ear, the three pairs of the listening list are one
  recording each and The Nine's Evol Intent VIP a different recording
  (2026-10-09). The strongest false candidate is therefore The Nine
  against that VIP at 197 hits; the VIP plays the original's first 3:34
  unchanged, then a 135 bpm slowdown. The rule of 240 is only 1.22×
  above it, where the calibration register asks for about 2×. Raising the
  rule to about 400 would lose brief plays without resolving the
  ambiguity. The held-out mix shows the cost already: Sick Note, a true
  play, is possible at 233 hits, 7 below the rule (test-set evaluation
  3), so true plays and shared material now meet in the same range of
  hits. Session 11 measured shared passages played whole at NAS scale
  (experiment 0076): in 18 of 34 directions the other version's play is
  confident (The Nine's VIP 21.7× the rule; "Fire" versions 1.2-2.5×;
  Sonar's revision 1.6-2.3×), while the played file's own play over the
  same moment is 4-60 times stronger (1.7 times for the VIP), with or
  without the played file in the index. A ratio of overlapping plays
  would separate the remixes but not the VIP: copies of one recording
  over one passage of the 2003 mix range from 0.30 to 1.00 of the
  strongest copy's hits. *(session 15, experiment 0091)* A containment
  rule (a confident play inside a stronger confident play of another
  cluster becomes shared material), measured offline over 24 settings:
  no development report holds a wrong confident play, so it removes and
  costs nothing there; whole play spans and a 30 s tolerance demote
  right brief plays in the generated mixes, passages with 0-10 s do not;
  on the new shared passages it removes 5 of 6 related plays, and none
  when the played recording is not indexed. A ratio bound trades the VIP
  for weaker copies. Not adopted: a new development mix that plays
  versions whole is needed first.
  Method: a challenge set of originals, VIPs, remixes and
  tracks sharing breaks, each pair confirmed by ear;
  excerpts from shared and from distinctive passages at several
  durations, with the played recording in the index and left out.
  Compare the current rule with measures such as distinct anchors,
  support spread over time, and evidence outside passages the library
  itself shows to be shared; these are hypotheses to test. Validate on
  recordings not used to develop them. Where the played passage cannot
  tell two versions apart, report the ambiguity instead of choosing one.
  Sample identification counts finding the sampled recording as the
  right answer; for Gunfinger it is a wrong identification, on the same
  evidence. Bhattacharjee et al. (2025) annotate Sample100 per segment
  and label drum-break samples apart from riffs. Scenarios that tell
  sustained evidence from distinguishing evidence: a short shared break
  repeated for a long time (does repetition alone reach the rule?);
  excerpts wholly inside a shared passage against excerpts reaching
  into distinctive material; the played recording and all its rips left
  out while a related remix or original stays; and, as a positive
  control, the source file left out while another rip of the same
  recording stays. Repetitive material is where fingerprinters lose
  specificity (Sonnleitner & Widmer 2016, §VI; Sonnleitner et al. 2016,
  §6). Run the challenge set before adopting anything that adds
  evidence (the items below, the band profile, a minimum span).
  Session 7 ran the four scenarios on two judged pairs (`gunfinger-eval
  shared`, experiment 0055): nothing reaches the rule; looping a shared
  passage to 240 s does not add hits, because each repeat starts a new
  alignment (summing a play's segments would change that); running into
  distinctive material adds hits only to the played file. More judged
  pairs (the verdicts by ear) would make it a challenge set.
  *(2026-10-14, an unverified proposal from an external review)* Before
  designing a statistic, an identifiability diagnostic: hold a shared
  passage fixed, vary the distinctive context around it, and find
  whether distinguishing evidence is absent from the audio, lost before
  peaks and pairs, or present but unused; only the last justifies a new
  statistic (proposal 5 of
  `docs/research/2026-10-14-matching-proposals.md`).
  *(session 20, experiments 0110-0115)* The challenge set exists:
  `docs/panels/shared-material-seed-2026.json`, 12 families from the
  judgements by ear (verdicts, `shared` answers, two judgements in the
  docs), each recording with its rips, split by family with seed 2026.
  The development half runs in one command (`gunfinger-eval
  --other-peaks-dir <NAS store> shared
  docs/panels/shared-material-seed-2026-development.json`, then
  `scripts/analysis/shared_outcomes.py`); the validation half is kept
  unsearched for a frozen rule. With the played recording left out at
  NAS scale, a related version is wrong confident in 106 of 218 queries
  and 12 of 14 directions once about a minute of the passage plays
  (0111). Proposal 5's diagnostic: outcome 3 (the information survives
  into peaks; the rule ignores how sparse the evidence is) in 10 of 14
  directions, clearly where the played recording has rips of its own;
  outcome 1 once, against a poor upload (0112). But true plays in the
  development mixes are as sparse: no cut on hits per second or the
  verifier's share keeps 95% of them and rejects half of the panel's
  wrong plays (0114, which compares clean renders with mixes), and the
  mixes themselves, with every played recording left out, give no wrong
  confident play to test against (0115). Next, where a related file's
  alignment stops while the music runs on, tested against true plays
  with its rule written first; reporting the ambiguity is the fallback
  (our decision). *(session 21, experiments 0116 and 0118)*
  Like for like, rule written first (`docs/brief-21.md`): 36 rendered
  mixes play each development played file with a shared passage whole
  between six drawn corpus tracks (`gunfinger-eval shared-mixes`); with
  the played recording left out, 87 wrong related confident plays (Fire,
  Pyro, Sonar, Dozer) against 453 true ones. Hits per second, the
  verifier's share and their combinations with a map of shared passages
  pass the feasibility test in rendered mixes only: at the frozen
  thresholds the development mixes keep 45-70 of 97 true plays (57-74 at
  0118's thresholds, with drawn tracks found only through another rip). The
  file's own material beyond the play points the wrong way (true plays
  end at transitions). The map (`search_peaks` over the play's file span,
  the strongest holder outside the play's recording) marks 61 of the 87,
  all in Fire's family, where a third version is indexed, and also true
  plays of tracks with a remix, VIP or edit in the library and of
  uploads newer than the duplicate clusters. No pre-registered candidate
  passes against real true plays. An exploratory combination found
  after the run, rejecting a play the map marks with under 6.276 hits
  per second (the cut keeping 95% of the development mixes' true plays),
  rejects the 61 marked wrong plays (all Fire's, the densest at 6.01)
  and keeps every rendered true play; it is frozen in the brief for a
  test on real mixes not used so far. The map is also the basis for
  reporting the ambiguity, our decision. *(session 23,
  experiments 0120-0123)* With the NAS clusters rebuilt, the 9 wrong
  confident answers in the development mixes at NAS scale were newer
  uploads and are gone; the 87 wrong related plays remain. Reporting the
  ambiguity as a note on the tracklist line, measured against criteria
  written first, flags 61 of the 87 but also 15 of 108 true plays of the
  development mixes (13.9%, against 5%): not worth proposing. The 15
  are 6 tracks: five name a remix, edit or close version, Omni Trio's
  another title; in the tracklists 6 of 40 entries carry the note. A
  second panel
  is proposed (`docs/panels/shared-material-2-seed-2026.json`, not
  searched; its recordings need `pair` before a first run). The panel is now
  a required check: anything that adds or changes evidence (the items below, the
  band profile, a minimum span, the FFT lengths' retrieval study) runs
  its development half and may add no wrong confident query there.
  *(session 24, experiments 0124-0127)* A diagnostic version verifier
  (`gunfinger-eval versions`, after the GPT Pro report: band dynamics
  along the fitted line, EQ-group gains fitted on one part and tested on
  another, supported / contradicted / unobservable patches), threshold
  set on the panel's development half, criteria written first: E1 5 of
  108 true plays rejected (passes at the limit), E2 43 of 87 wrong
  related plays (fails by one). In the rendered mixes it rejects only
  Fire's wrong plays (the low bands under matching drums); Pyro's and
  Sonar's are never rejected (the same content along the line, an
  inference). Every rejected E1 play has its reported line 98-193 ms
  off (anchored on the opening window, or an upload drifting in speed);
  that this is the cause is an inference, since some kept plays are as
  far off. A continuation needs its own line from the audio, then fresh
  calibration; it needs the reference audio, so not for `--store-only`
  (ADR 0003). Kept in the tree as a harness diagnostic until we
  decide. *(session 25, experiments 0128, 0129 and 0132)* The second
  panel's development half (Phantom Force, Star Trails, We R The Ones;
  rips joined with `pair`, passages cut, written beside the frozen file
  as `docs/panels/shared-material-2-seed-2026-development.json`): with
  the played recording left out, 5 of 135 queries are wrong confident,
  all Phantom 2018 over Phantom Force in one direction (the first panel:
  106 of 218); the other related versions stop at possible (113-185
  hits), most likely because their passages are shorter, with fewer
  hits. The
  verifier as built rejects 4 of these 5 (too few to read by the rule
  written first) and keeps 77 of 78 positive controls; its one false
  rejection is again a reported line about 320 ms off. This suggests
  (an inference from two panels) that the version problem is
  concentrated in a few families with long, dense shared passages
  (Fire's above all), and leaves a version 2 of the verifier (its own
  line from the audio) few fresh wrong plays to be evaluated on.
  The harness's renderer for `versions` and `identifiability` now
  counts a start on the decoded timeline (`render_samples_as_indexed`),
  as the peak store was built. *(session 26, experiments 0133-0135)*
  Version 2 measures each play's line from the reference's own hits
  (stretches split at jumps, pieces for drift) and contradicts a band
  only in an EQ group with an anchor. Criteria as version 1's, on a
  declared second use: E1 9 of 108 true plays rejected (fails), E2 45 of
  87 (passes). The line keeps four of version 1's five rejected true
  plays, but on wrong plays it drops differing stretches as strays and
  can move to another passage; the group rule removes most of the wrong
  plays' contradictions, so T falls to 0.0068 and blends reject true
  plays (an inference from the timelines). A version 3 (the line kept
  near the reported one, knots chosen with continuity, another
  treatment of another record's sound) needs a first-use set: new real
  mixes with checked tracklists. *(session 40, experiment 0187)* Between
  library files, where the alignments of two files lie (their offsets,
  the longest stretch without a hit near the main one) does not tell a
  near-identical version from a damaged copy of one recording either
  (AUC 0.40-0.56 on 14 hard judged pairs chosen from the measures'
  errors, so near chance by that choice).
  *(session 48, experiments 0215-0218)* On the marked plays of
  generated double drops, evidence outside passages the library shows
  to be shared was measured: files that share no alignment of 60 hits
  or more by `pair` (F1) separate a record playing under an unrelated
  partner from shared material completely (AUC 1.00 at both scales on
  development and test seeds; at most 17 hits against at least 93);
  where the marked play falls through the alignment (F2) and its hits
  outside the shared spans (F3) separate a record inside its relative
  too, but let in 3-22 shared plays (a related version found elsewhere
  in the other file); the peak verifier's share reads the wrong way.
  The shared class rests on one relation (Clockwork) in the generated
  mixes and five plays of four families in the development mixes.
  *(session 66, experiments 0259-0261)* A harmonic view of each
  candidate (median-filter separation, FitzGerald 2010, its peaks
  counted at the search's alignment) does not tell true plays from
  related versions better than denser whole-spectrum peaks at equal
  hits (AUC 0.579 against 0.684 on eight development families, 0.667
  against 0.724 on seven frozen ones); above 240 hits every view
  separates, below it the blend partner's cases score lowest under the
  harmonic view (masking by the other record, an inference).
  Not proposed (`docs/research/2026-10-31-harmonic-view-decision.md`).
- **Evidence beyond exact pair hashes.** The second pass counts only
  exact pair-hash matches at the fitted speed (`search/refine.rs`).
  Peaks can survive where their hashes do not: a peak half a bin away
  falls into the neighbouring bin, and a changed neighbour changes which
  pairs form (Six 2021, §2). Three experiments in the harness, in this
  order, with candidate discovery and the confidence rule unchanged
  until they show separation:
  1. A peak verifier after Qfp's (Sonnleitner & Widmer 2016, §VI-C):
     for each candidate, look for its reference peaks in the query at
     the fitted alignment, within a small time and frequency tolerance,
     and record the distinct reference peaks found, their share of the
     reference peaks in the aligned passage, and how they spread over
     time. Count reference peaks found in the query, not query peaks
     matched: a blend adds query peaks. In a comparison on DJ mixes,
     Qfp's verification raised specificity on the Mixotic mixes from
     0.647 to 0.927 while accuracy went from 0.889 to 0.876 (Sonnleitner
     et al. 2016, table 2); those are its figures on its data. Test weak
     true detections, unrelated false candidates and confirmed
     shared-material pairs separately; the combined-damage condition
     offers many candidates (skip at 240: 65 of 80 possible or better, 4
     confident; experiment 0027). Success: better separation of true and
     false candidates on recordings not used to develop it. Shared
     passages may verify as strongly as the played recording.
     Done in session 7 as `gunfinger-eval --verify` (tolerance ±1 bin,
     ±1 frame; experiments 0054, 0056, 0058). At equal hits its share
     separates no better than hits (AUC 0.953 against 0.950 at 262
     tracks) and worse against related records: weak blend partners
     (0.16-0.24) verify like shared passages (0.11-0.41). Kept as a
     diagnostic; no rule. Its share could order a listening list.
  2. What exact quantisation loses after fitting: on the same
     alignments, count exact pair hashes, pair hashes that allow a
     neighbouring value in each component (as Panako 2.0's near-exact
     hashing does, Six 2021, §2), and the verifier's peaks, each
     correspondence once. Stay within the candidate's reference passage,
     so that accuracy is not mixed with the cost of tolerant lookups
     over the whole index. In Qfp, narrowing the tolerance (likened by
     its authors to quantised hashes) cut Mixotic accuracy from 0.876 to
     0.570 and raised specificity to 0.987 (Sonnleitner et al. 2016,
     §6). The comparison separates features that land in neighbouring
     bins from features or pairings that are gone; tolerant hashing
     recovers only the first.
     Done in session 30 (`verifier/pairs.rs`; experiments 0149, 0151),
     at 40,126 records: under combined damage E keeps 9.6% of the clean
     exact pairs and E + N 24.3% (the near pairs equal about a sixth of
     the exact pairs lost; with other rips only, about three quarters);
     260 of 510 damaged true detections under 240 would reach it with
     E + N. Related versions gain as much as the played
     recording (AUC of N/E 0.50 on the shared panels; within the damaged
     run, related candidates gain more than true ones): 37 related
     candidates in the recall runs would reach 240, among them the Fire
     Splash Remix on clean Fire queries, and (an exploratory count) with
     the played recording left out a related version would be confident
     in 238 of 419 shared searches instead of 112. No unrelated candidate
     reaches 240. No rule; a tolerant pass would need a test that
     separates versions.
     Not measured there *(2026-10-17, a proposal from an external
     review)*: reference pairs whose two peaks are both in the query but
     no longer pair, because another peak (from a blend, for example)
     now comes first in the anchor's target zone (each anchor pairs with
     its next 2 peaks, ADR 0001). Under combined damage the verifier
     finds a median 37% of the reference peaks while E keeps 9.6% of the
     clean exact pairs. First split the reference pairs lost under
     damage, from step 2's runs: an endpoint gone, both endpoints found
     but the target displaced, or both found with a changed quantised
     value (N). Only if displaced targets are a large share, pair the
     query with more targets (fan-out 3, 5 and 8) in the second pass
     alone, the index unchanged at fan-out 2 (experiment 0004 changed
     both), each reference pair counted once. More query pairs also
     raise chance agreement, so measure unrelated candidates, related
     versions on the shared panels as in experiment 0151, and CPU. Step
     2 predicts that related versions gain as much as the played one;
     if they do, that settles it.
     Done in session 44 (experiments 0206-0208): under combined damage
     88% of the lost reference pairs have an endpoint missing in the
     query, 4.5% a displaced target (mostly at ranks 3-5) and 6.3% a
     changed value (N); on clean audio 55%, 17% and 24%. Query fan-outs
     3, 5 and 8 (the index at 2) add a median 18%, 30% and 32% to damaged
     true detections' exact pairs, and 21 of 507 under 240 would reach
     it at fan-out 5, but related versions on the shared panels gain more
     than the played recording in every band of E (AUC 0.24-0.27); no
     unrelated candidate reaches 240. Settled: no rule. The loss under
     damage is peaks, which is step 3's question.
  3. Denser query peaks around candidate passages, the reference store
     unchanged: Qfp analyses queries denser than references, keeping its
     database small (Sonnleitner & Widmer 2016, §IV-B). Experiment 0004
     changed the density of both at once, so it does not answer this.
     Use the extra peaks in the verifier first, so that the pairs the
     first pass forms stay as they are; measure peaks recovered under EQ
     and blends, accidental agreement, and CPU.

  After step 1, most reference peaks are found in the query (median
  share 0.84 for the source file, 0.65 for another upload, experiment
  0058); how many of them the exact hashes miss is what step 2
  measures. Since more evidence did not separate true from related
  candidates at equal hits, steps 2 and 3 matter more for recall than
  for separation: the 57 misses within 9% of relative speed in
  experiment 0059 are weak copies found at 25-239 hits, below the rule.
  Measure those copies and the shared-material scenarios (experiment
  0055) together, since both would gain.
- **Longer hashes.** Triplet hashes would scan 25-79 times fewer postings
  but lose about half their evidence under heavy damage (experiment 0014,
  estimated offline, no search). Revisit if profiling on a real large
  library shows lookups still dominate after simpler changes; compare
  recall under damage, related-recording errors, memory including the
  lookup structure, and wall time. Triplets come from Panako (Six &
  Leman 2014), which makes them invariant to speed with time ratios;
  experiment 0014's keep exact coordinates. Qfp uses quads (Sonnleitner
  & Widmer 2016). On DJ mixes Panako's triplets were more specific than
  Wang-style pairs (0.746 against 0.487 on club mixes, 0.349 against
  0.255 on Mixotic), at lower accuracy (Sonnleitner et al. 2016, table
  2). Each extra peak a hash needs lowers its chance of surviving (Wang
  2003, §2.2, for pairs). *(2026-10-14, an unverified proposal from an
  external review)* A one-byte companion selector per posting, from the
  anchor's other pair, to scout candidate files before today's pair
  matcher searches them: about 0.95 GB more memory, discovery changes,
  and only if grouping still dominates after the exact changes (proposal
  4 of `docs/research/2026-10-14-matching-proposals.md`).
- **Evidence summed across a play.** Plays (ADR 0006) show Fibre Optix
  "Sin", faded out for a station insert, as one possible play of two
  segments (168 hits). Experiment 0011: summing every segment is unsafe
  (chance sums reach 125 hits), summing segments of at least 60 hits gave
  no false group, but no measured track would gain. Revisit when several
  independently reviewed missed plays have strong, compatible segments that
  together pass 200, and test a restricted rule against related recordings
  and long unknown passages. A looped shared passage is the case to beat:
  the passage Clockwork shares with its Stakka remix, looped to 240 s,
  gives the other recording 13-22 detections of up to 144 hits each,
  which summed would be confident (experiment 0055). A rule that counts
  only segments advancing through the reference would leave the repeats
  out (proposal, not measured).

## Evaluation

- **The verdicts by ear on borderline pairs.** At NAS scale 28 pairs of
  files cover 40-80% of each other (experiment 0049), among them China
  Cup against its Prototype upload, whose detections are most of the
  false confident ones left. `docs/pair-verdicts.txt` takes the verdicts
  by ear; rerun `clusters --from-peaks` and the NAS-scale protocol
  after adding them. `clusters` stops when a chain of joins through
  other files contradicts a `different` verdict, and names the chain;
  with the NAS store it checks the last run's pairs first, before an
  85-minute search. A chain only the new search finds still stops the
  run after it, and the search is lost: in session 10 the PRO012 rip of
  Sonar, indexed since session 6, joined the revision (467 hits, 1.3
  hits per second) after 2 h 16 min. `--cut-sparsest` (session 10, opt
  in) cuts each such chain's sparsest measured join and lists it for
  checking by ear instead. `--reuse-pairs` (session 10) clusters the last
  report's pairs again with new verdicts in about a second, and the
  check before a search now reads those pairs as measured, so it catches
  such a chain once a run has found it. A search that a verdict stops
  keeps its pairs in `duplicate-clusters-stopped.json`, which
  `--reuse-pairs` takes. Once a run has cut a chain with
  `--cut-sparsest`, the check before the next search stops on the same
  chain from that run's pairs (session 25), so a rebuild needs
  `--cut-sparsest` again until the cut joins are judged by ear; at
  40,126 records (0128) a second upload of Sonar links the original to
  the Mark System Revision and was cut the same way. *(2026-10-22)* Every
  borderline pair between two clusters of session 33's NAS run has a
  verdict now, both Sonar cuts among them (status, "after session 38").
  China Cup's upload is a near-identical version whose hi-hats differ
  for a few bars, so its detections name the right track in another
  version; Dilusion's upload, the same recording with a skip, joins its
  cluster. Coverage and supported time do not separate the two; how
  much of the file lies within a few seconds of the main offset might.
  *(session 40, experiments 0186-0188)* Measured on the 60 other judged
  pairs: the share of the shorter file holding hits of alignments within
  10 s of the main offset (`near_10`, `scripts/analysis/pair_offsets.py`)
  orders same above different with AUC 0.95 (coverage 0.90, supported
  time 0.94) and 6 leave-one-out errors (9 and 7), the registered bar by
  one pair. On the 14 hardest judged pairs (uploads of one recording
  that drift in speed or are edited, against versions that differ in one
  layer, such as China Cup's hi-hats or We Enter's instrumental), chosen
  from the measures' errors, the AUCs are 0.40-0.56 (coverage 0.06). ƆƐI3C's China Cup gives 18 of the 25 false
  confident detections over the NAS sweeps of seeds 2026-2028, the
  other 7 Cool Rock Stuff ~ the Mobb Deep / Urban Takover upload;
  whether a detection of a near-identical version counts as wrong is
  ours. *(Our decision, 2026-10-27: it counts as right. Gunfinger
  identifies track titles; China Cup found as another China Cup whose
  hi-hats differ for a few bars is a correct identification. The sweep
  and calibrate change their scoring with an ADR.)* *(Done in session
  54: ADR 0022; `docs/same-titles.txt` holds our same-title judgments,
  apart from the clusters; the sweep, `recall`, scans, generated mixes,
  `grid` and calibrate count a confident detection of a judged file as
  right and list it apart (`robust` lists it apart; `seconds` and `loss`
  keep the cluster alone). Over the NAS sweeps of
  seeds 2026-2028 the wrong confident answers fall from 25 to 7, all
  Cool Rock Stuff ~ the Mobb Deep / Urban Takover upload; calibrate's
  false accepted 25 -> 7, no threshold moves (experiment 0233). Whether
  other pairs judged different carry one title, among them Cool Rock,
  Tower Bass's two mixes and the Stalker versions, is for us to judge.)*
  *(2026-10-30, by ear: all four pairs carry different titles, so
  `docs/same-titles.txt` does not change and the 7 Cool Rock detections
  stay wrong confident answers.)* `listening_pack.py select --min-hits` (default 30) leaves out
  borderline pairs on few hits, and `--disagreements` picks the joins
  and apart pairs nearest `near_10`'s threshold: 12 wait in
  `work/s40/pack/`, though none crosses it.
  *(2026-10-25; session 49, experiments 0220 and 0221)* Ten of the pack
  were judged (status, "after session 46"). On these pairs, which no
  measure was fitted on, coverage and `near_10` at session 40's
  thresholds put every pair on its verdict's side and supported time
  misses one (Falcon - The Stand's drifting upload, 0.009 below); pooled
  with the 60 earlier judged pairs, leave-one-out errors 9, 9 and 6. A
  near-offset share over rungs within 0.8% of the fitted speed and lines
  within 20 s of the main one (`drift_0.8_20`, chosen on session 40's
  populations) orders those populations' judged pairs best (AUC 0.99,
  in-sample; pooled with the ten, leave-one-out errors 4) but by the
  registered bar is not better: its
  threshold puts two same pairs of the ten and one presumed join below
  it. Found after the test, not confirmed: the same share with lines
  within 10 s makes no error on the ten. More judged pairs would decide.
  *(larger library)*
- **Clustering precision.** Most of session 6's gain at NAS scale came
  from joining rips the clusters had missed, so a wrong join would now
  count a false identification as correct. Coverage counts the span
  from the first to the last hit and ignores how much evidence lies in
  it: DJ Trace - Sonar joins its "Mark System Revision" at 91% with 0.7
  hits per second of aligned span, against 18.6 for the median join and
  6.5 for the 5th percentile. `clusters` now lists joins below a tenth
  of the median to judge by ear (at NAS scale the two Sonar
  revision pairs and one Phoenix upload pair; none at 262 tracks).
  Supported time (experiment 0057): the Sonar joins hold aligned hits
  on 0.39-0.43 of the file, Phoenix's on 0.63-0.72, random joins
  0.98-1.00; China Cup and The Specialist against the uploads the
  clusters keep apart, 0.96-0.99 at two offsets. A rule on supported
  time would need the verdicts by ear to set it. Still to do: inspect
  long sparse alignments and chains through mixed-CD tracks (a mixed
  CD's track can hold the start of the next one, experiment 0058),
  and check a sample of joins by ear, not only the pairs the
  clusters keep apart. Evaluations at scale elsewhere meet the same
  effect: with 430,000 added tracks, a copy of a played song among them
  counted as a false positive (Sonnleitner et al. 2016, §6).
  *(session 40, experiments 0186 and 0187)* `pair` reports which
  seconds each alignment at the fitted speed supports. 98 joins drawn
  without a verdict, the shared panels' cross-version pairs and 60
  strongest apart pairs of different tags are separated almost
  perfectly by coverage, supported time and the near-offset share alike
  (AUC 0.99-1.00); over all 1,055 joins without a verdict the near-offset
  share is 0.63 or more and over 638 apart pairs 0.30 or less (the
  clusters file lists 313 of its 2,590 pairs in both directions). All
  three fail on uploads whose speed drifts and on edits that move the
  rest by more than 10 s; for supported time and the near-offset share,
  by inference, because one rung at the fitted speed follows one speed.
  A join rule on the near-offset share would rest on one pair more than
  supported time gets right; a change is ours, with an ADR.
  *(session 49, experiment 0220)* Gathering hits over rungs 0.1% apart
  near the fitted speed (`pair --fine-rungs`, `drift_offsets.py`) lifts
  the drifting same pairs (Fractles 0.12 to 0.65, Coma ~ Spraycan 0.55 to
  0.97) but also versions sharing most of a track (the Sonar revision
  pairs to 0.56-0.77); the registered variant fails its bar on the ten
  verdicts of 2026-10-25. No join rule changes.
- **The possible tier at NAS scale.** Matches between recordings of
  different artists and titles reach 63-70 hits over the NAS sweeps
  (experiments 0040, 0041, 0051), above the tier of 60; on nested
  subsets of the NAS records 21-34 hits up to 3,200 assets and 64 from
  9,200 (experiment 0052). *(2026-10-12)* None above the tier is
  chance: each one shares material. By ear, Clockwork and Simon
  Static - Rubba Rock (the 64 from 9,200 assets) share the same stab
  sample with its reverb tail and the hi-hat rhythm under it; The Stand
  and the untagged `1.mp3` are joined since; Mystery Machine ~ NIL -
  Infectious and Sentient ~ Shere Khan - Criminal Record repeat at every
  sweep speed and the clusters' pairs label them related. The jump at
  9,200 assets is the first subset that holds Rubba Rock, not chance
  growing with the library. *(2026-10-14)* The two strongest not
  checked before, both below the tier, share material too, by
  ear (answers 35 and 36 of the answer log, from
  `work/audition/shared/s17-shared.json`): Watermelon ~ 300 Baud -
  Monotone (Mobiuz & Cycom remix), 41 hits, and Falcon - Fractles ~ Ryme
  Tyme - We Enter (Optical remix v3.0), 40 hits. Every strongest match
  between different recordings checked so far, from 40 hits up, shares
  material, and no chance match has been heard above 40 hits. The
  earlier proposal, 60 while chance stays below 30
  hits and twice it above (130-140 at NAS scale, which would hide the
  development mix's shared-material notes), took the 64 for chance. We
  decide; 60 stays. *(larger library)* The jump to 64 hits is
  one pair that shares material, so the library's makeup matters as
  much as its size: measure at each size
  rather than build the breakpoints into a rule, and count what a
  higher tier costs (weak true plays lost, possible plays to review per
  hour of mix). Shared-material notes could keep a lower threshold than
  standalone possible plays, but only where the two library files match
  each other in that passage. Two plays overlapping in a mix, or similar
  tags, do not show shared material: chance lines overlap confident
  plays too. *(session 15, experiments 0092 and 0094)* Counted on the
  NAS development reports: the two mixes show 2.9 possible plays per
  hour at 60 hits (0.6 at 130), four of them versions of the track
  playing; no development true play depends on the tier; at 262 tracks a
  higher threshold hides only damaged excerpts (48-128 of 278 at 80-130).
  The fifth play, Rift feat. Souldrop - Meltdown at 98 hits during
  Phantom Force, is not a re-pitched copy of the passage; by ear, the
  two share the full drum break (2026-10-12), so every
  possible play of the two mixes at 60 hits is a version or shared
  material. *(session 18, experiment 0102)* The third development mix
  (`nocny-trans-vertical-2002-08-23`, 1.11 h) shows 3.6, 3.6, 2.7 and
  1.8 possible plays per hour at 60, 80, 100 and 130 hits, every one
  shared material with the confident play over it (Mampi Swift - 2000
  during Stab 2 Def, the Klute Remix in two files during the VIP, the
  untagged `1.mp3` during Cannibal); none of its true plays is found
  only as possible. The three mixes together: 3.2, 2.8, 1.8 and 1.1 per
  hour. Its strongest detection matching no track (66 hits, from 44:33)
  is Stab 2 Def itself, which is heard blended in from 44:40,
  before its slot's window.
- **Recall beyond the corpus recordings.** *(development half measured
  in session 10, experiment 0068: 540/540 at every speed and kind, no
  miss, held-out sources never confident; 30 wrong answers from a
  mixed-disc track and an upload pair the clusters keep apart. The
  harness reports why a miss failed. The hardest conditions, alone and
  combined, at 40,126 records in session 28, experiments 0143-0145:
  other rips only 175/180 confident; brief plays 228/522 (15 s) and
  493/522 (20 s), set by the window grid; combined damage 5/522
  confident, 428 possible or better, for too few hits; all three
  together 0/360 confident, 107 possible; no wrong confident answer and
  no held-out query confident in any condition. The Linux machine gave
  the same counts for clean queries, other rips only and brief plays, and
  within 2 queries without damage and 5 with it otherwise, at 40,683
  records (session 33, 0156). After session 52's prune (40,678 records,
  session 57, 0238) the counts are the same, except other rips only:
  174/180 (from 175), and 171/360 with brief plays (from 176): those 6 excerpts of
  Known Unknown - Rollers Edit were identified only through the pruned
  track-fix copy. Session 50 searched the sweep panel's
  other-rips form with exact copies of the rips (0223): 408 of 423
  confident at level 0 and padded alike; the recall panel itself was not
  searched padded. Still to do: the validation half, once the
  rules change.)* The four sweep panels draw
  320 excerpts from 208 corpus files (experiment 0025). At NAS scale
  they measure those recordings against 27,000 records, not recall on
  the rest of the collection. A panel drawn from NAS records, split into
  development and validation groups by recording family before any
  result is seen, would cover other upload sources, producer
  catalogues, break-heavy tracks, mixed-CD extracts and other masters.
  Its sources need clusters first (`clusters --from-peaks` around them,
  about 85 minutes a round), or their other copies count as wrong.
  *(done in session 28)* The hardest conditions combined at this size:
  other rips only, brief plays and damage together (combined damage at
  262 tracks under skip at 240: 4 of 80 confident, 65 possible or
  better, experiment 0027), with each miss's reason: never a candidate,
  too few hits in the second pass, or enough hits in too few windows
  (`recall --other-rips-only --queries clean,damaged,brief,brief-damaged`).

- **Stereo faults before the downmix.** *(2026-10-17, a proposal from
  an external review; measured in session 42, experiments 0194-0198)*
  FFmpeg mixes every file down to mono (`-ac 1` in `decode.rs`, each
  channel at 1/sqrt(2)) before peaks are picked. On `robust`'s 80
  indexed excerpts rendered in stereo (opt-in conditions `stereo-*`),
  at 262 tracks and at NAS scale alike, the downmix loses every excerpt
  when one channel's polarity is inverted (the downmix keeps only the
  side, 10-20 dB below the mid in most sources), 51 under noise at 0 dB
  in one channel, all under noise at -10 dB, 43 when one channel is 5
  ms late and 10 when it is 1 ms late; a silent channel costs 2, noise
  at 10 dB and a 0.2 ms delay nothing. No fault raised a wrong confident
  answer. An inverted or silent channel in the indexed rip instead of
  the query costs about as much (0197: inverted, 0 of 20 confident;
  delays there left fewer queries confident), and over the
  second half of generated mixes an inverted channel leaves 1 of 100
  plays there confident, a 5 ms delay 37 and noise at 0 dB 39 (0198). As a query-side
  diagnostic (`robust --query-channels`), the clean channel keeps 75-76
  of 80 under each fault that crosses the bar but loses 2-3 clean
  excerpts and keeps 61-64%
  of the hits; the side signal wins back every inverted excerpt and
  nothing else; a side-louder-than-mid trigger fires on one clean
  held-out excerpt and on 91% of 5 ms delays, so it does not single out
  inversion. No choice met the pre-registered bar for a fallback
  (0195). About a third of the corpus's sources are mono or nearly mono
  (0196); the store keeps no per-channel measure, so how many NAS rips
  carry a fault is not known. Still open (ours): whether any fallback
  (searching the downmix and both channels, three searches a query,
  won back 77-78 of 80 in an exploratory union) is worth its own
  experiment and an ADR, and whether a per-file check of the channels
  at indexing time is worth its cost.
  *(Session 61, experiments 0245-0247, opt-in `--front-end`)* Peaks
  picked from the sum of the two channels' STFT powers (today's power
  plus the side's) win back every inverted channel and every delay up to
  5 ms: as queries against today's records (80 of 80, and 93 of 100
  plays over the inverted half of generated mixes), and with a store
  built the same way also in the rip (20 of 20 each). Noise in one
  channel costs more (0 dB: 18-23 of 80 against 29), mono queries
  against such records lose 2-4 of 80 in 6 of 38 `robust` conditions
  (the registered bar for an ADR proposal fails), and a search costs 12%
  more at NAS scale. Not adopted; the decision note
  (`docs/research/2026-10-30-stereo-power-decision.md`) recommends a
  pre-registered test of the query side alone, which needs no rebuild.
  *(Session 62, experiments 0248-0250)* That test, at NAS scale under
  one frozen rule, fails its registered bar: the fault gains hold (80 of
  80 inverted and delayed excerpts, 93 of 100 plays in inverted halves
  of mixes), nothing new is wrong, the shared panels lose nothing (17
  fewer related versions), but `recall` loses 9 of 522 clean excerpts
  (one wide source) and 58 of 721 brief plays (51 with the side within
  15 dB of the mid): queries picked from
  mid plus side carry peaks the downmix records lack (median hits 0.39-0.49
  of the downmix's where the side is within 10 dB of the mid). A census
  of 2,000 NAS rips (0248) finds about 0.4% inverted in one channel,
  delays and missing channels rarer. No ADR; `identify` keeps the
  downmix. Open (ours): whether a combination equal to today's power
  where the mid dominates (the larger of the mid's and the side's power
  per bin, unmeasured) is worth its own test.
  *(2026-10-30, by ear)* Of the four rips flagged by the census and
  played against their downmix, Usual Suspects - Advent, Keltech - Axis
  (Low Level Mix) and Plasmic - The Weapon are inverted in one channel;
  We - Hello World is unsure.
- **More development mixes.** Mixes of existing library tracks, especially
  with brief plays and long blends, would test what the single development
  mix cannot. Agree each mix's role before looking at its results. The
  held-out test mix has two evaluations left (ADR 0004). *(larger
  library)* Each new genre needs its own development and held-out mixes;
  check the speed range those DJs used. My 2003 mix is the second
  development set (`razorjack-2003-03-29`, experiment 0060: 12/12 at NAS
  scale). To use it in the harness:
  1. *(done in session 10)* Manifests that reference NAS files
     (`second-library/<path>`, as in `docs/pair-verdicts.txt`), valid
     with `--other-peaks-dir`; the five NAS-only tracks are now
     references, set aside at 262 tracks.
  2. *(done in session 10, experiment 0066)* Clusters around its files
     (`clusters --from-peaks --manifest <set>`), so that other rips and
     uploads count as the track.
     The alien5ive INFRA012 upload of The Specialist,
     confident in the mix, was judged the same recording by ear (2026-10-09). Before the
     run: corpus-only `clusters` adds every `same` verdict as a link, so
     the NAS files in the verdicts by ear would enter the corpus
     clusters; add verdict links only between files of the libraries
     searched, and not to files their ignore files leave out. The ignore
     file exists (ADR 0009); after the NAS store is pruned, one
     85-minute run serves both.
  3. *(done in session 10, experiment 0067)* The set in the standard
     protocol at both sizes, with leave-outs: 12/12 at NAS scale, 7/7
     referenced and nothing possible over the 5 absent tracks at 262
     tracks, 0 wrong everywhere.
  Then the experiments it offers *(done in session 10, experiment
  0069)*: without Phantom Force's rips neither Phantom 2018 nor the
  Fracture edit becomes confident (the track is missed, with possible
  notes only); without the PHUD1 vinyl rip the start moves back from
  10:49 to 11:55.8; the Kinetic tease at 17:23 is the track's 1:11-1:51,
  possible on its own, and the track starts again from 0:15 at 18:35.
  *(session 18, experiment 0100)* The third development set,
  `nocny-trans-vertical-2002-08-23`: DJ Vertical on Nocny Trans (Radio
  Łódź, 2002-08-23), a YouTube upload of the broadcast, 18 tracks, one
  an ID. We fixed its role and wrote its tracklist without
  Gunfinger before anything searched it (ADR 0004). It plays VIPs and
  remixes whose other versions are indexed: 9/9 at 262 tracks, 17/17 at
  NAS scale, 14/17 in three leave-outs, 0 wrong everywhere; the other
  versions share 0-38 hits with the played one, except the Klute Remix
  of We R The Ones (166-263, possible in the mix, shared material). The
  containment rule (0101) and the possible tier (0102) were measured on
  it. The NAS's `sety/vertical.mp3`, one of 0065's candidates, was left
  alone. A real test of the containment rule still needs a mix whose
  tracks have indexed versions sharing much of their audio.
  Candidates on the NAS (experiment 0065, `work/s9/candidate-mixes.tsv`;
  no `identify` before their roles are agreed): my own
  `Razor Jack - 2003-08-27.mp3` (20.7 min); 136 sets, radio shows and
  single-file mixed CDs of 1999-2000 in the scene folders; single-file
  mixed CDs with published tracklists in folders at the root (Stakka &
  Skynet's Clockwork mix CD, Andy C's Ram Raiders, DJ TeeBee's Through
  The Eyes Of A Scorpion); 96 radio shows and sets in `sety/`. The root's
  `EM_1999_02_07_-_Ed_Rush_Optical.mp3` is by name the broadcast of the
  held-out test set: keep it out of development use.
- **Public DJ-mix datasets and per-second scoring.** Mixotic (10
  Creative Commons techno and house mixes, 723 reference tracks,
  approximate song borders) and UnmixDB (mixes generated
  beat-synchronously from the Mixotic tracks, with exact starts, cue
  points and speed factors) would give evidence without spending a test
  evaluation (`docs/references.md`). They are another genre, so they test
  how the design generalises, not the jungle calibration, and they need
  their own library and peak store. UnmixDB's time scaling comes from
  SoX; check which variants resample and which keep the pitch. UnmixDB
  reuses Mixotic's tracks, so split development from validation by
  recording family across both, and run the frozen matcher on the
  validation part. Besides track recall, score per second as Sonnleitner
  et al. (2016, §5) do: correctly labelled time, confidently mislabelled
  time, silence where the playing track is not in the library
  (specificity), and boundary errors, with both tracks correct during a
  transition and explicit uncertainty around annotated borders. Keep the
  current scoring for continuity. These measures would also judge the
  minimum span and the time that hits support (Matching). Published
  scores do not compare with Gunfinger's track recall; compare systems
  only by running them under one protocol. Session 7 could not download
  Mixotic: the three archives are Google Drive files that answer with a
  sign-in page; a browser download is needed.
  *(session 27, experiment 0139)* Per-second scoring exists:
  `gunfinger-eval seconds` scores saved `mixes`, `shared-mixes` and
  `scan` reports (correct, possible, unlabelled, mislabelled, silent
  where nothing indexed plays; a margin around approximate borders;
  `--boundaries` for start and end errors). Today's baseline: generated
  mixes 94.2% of indexed seconds correct, 100% specificity, edges 2-3 s
  inside the truth; the development mixes 86.8% (262) and 90.7% (NAS)
  at a 30 s margin, where the manifests' borders seem to account for
  most of the shortfall (an inference). Not yet unit-tested: the truth
  built from manifests and generated mixes. The datasets above remain
  to be downloaded.
- **The printed tracklist.** *(2026-10-17, a proposal from an external
  review; measured in session 39, experiments 0182-0185)* The harness
  scored detections, plays and seconds; nothing scored the entries
  `identify` prints after grouping files (ADR 0017), choosing each
  entry's name, flagging disagreeing titles and leaving out possible
  plays inside a confident play (ADR 0019). Session 39 added
  `mixes --kind drops` (double drops, a quiet or brief record inside a
  stronger one, a shared-material pair apart and together, a returning
  record, two rips of one recording, a held-out record; recordings whose
  tags agree, `scripts/analysis/tracklist_tags.py`), ran the CLI on 32
  mixes at 262 tracks and NAS scale (and 32 more with three other pairs
  at NAS scale) and scored the printed tracklist
  against the truth (`tracklist_score.py`, which rebuilds the entries
  and checks every printed line). Grouping and printing lose nothing the
  search finds confidently, apart from one merge in 512 plays: two
  unrelated records over a long drop whose tracks would have started
  2.2 s apart pass `same_recording`. No confident extra entry at 262
  tracks; at NAS scale Animation's remix (a related version by verdict)
  and, with other pairs, T-Minus and its vocal mix printed for each
  other. What leaves the
  tracklist is ADR 0019's cost: a record inside its partner found only
  as possible. In a double drop where the second record leaves again,
  19-25% of unrelated partners leave; for a record inside its relative
  it depends on the pair (Clockwork's original inside its remix 6 of 8,
  three more pairs at NAS scale 5 of 15, 0185), and two records that
  share at least 188 s (Pyro) become one entry under `titles
  disagree:`. Brief records (20-40 s) leave 19 of 32 times, while the
  search loses quiet ones (-9 dB) most. A drop through to the second
  record costs nothing. One new confident identification at NAS scale
  awaits listening: Digital & Spirit - Crash over the first 56 s of
  Ryme Tyme - T-Minus (Unreleased Vocal Mix) (0185).
  The same mark is right on shared material (64 of 105 marked plays at
  262 tracks), and no measure in the report tells the two apart (0183);
  a length bound of 40 s would print every drop partner and 19 of 64
  shared-material lines. Open, ours to decide with an ADR: whether a
  marked play may stay in the tracklist (for example by length, or
  marked as possible with its partner named), and whether
  `same_recording` needs more than a 5 s start and half the shorter
  play when two files' titles differ. Session 48 (0217) lined the drops
  up in beat phase (`mixes --kind drops --aligned-drops`, each record's
  strongest drop it can play then): 3 of 128 unrelated drops became one
  entry at 262 tracks and 4 at NAS scale (tempo-only 1 and 0), 6 of the
  7 under `titles disagree:`; a merge needs the two drops at about one
  place in their tracks. Not measured: drops of the same rank (first on
  first), and real double drops.
- **Panako as an outside benchmark.** Run in session 24b at 262 tracks
  (experiments 0130, 0131, 0150; Java 11, LMDB built from source): 59% of
  the sweep queries to Gunfinger's 100%, 26/27 development-mix tracks to
  27, 105 generated-mix plays to 106; 14 wrong answers, none at S* = 13
  (104 plays there); about 4 times the index CPU per second of audio. A
  NAS-scale run (about 72 h of indexing on one core) was to follow
  only if Panako found what Gunfinger misses; it found 7 plays that
  Gunfinger reports as possible, nothing else.

## Engineering

- **Before publishing.** Check that the history holds no audio from
  `corpus/` (`git log --all --stat`). We keep the library file
  names in `docs/` as they are (2026-10-06).
- **GitHub Actions.** Run `scripts/check.sh` on push. FFmpeg must be
  installed in the runner for the codec tests.
