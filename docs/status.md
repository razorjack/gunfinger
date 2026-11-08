# Status

Running log, newest entry at the bottom: one short paragraph per
session with pointers (`docs/session-rules.md`). This file keeps the
entries from 2026-11-02 on; the older ones are in `docs/history/`
verbatim, under the same headings.

## Index

- 2026-10-05 to 2026-10-23: the seed, the proof of concept and its "Final report", and sessions 2-39. `docs/history/status-2026-10-05-to-10-23.md`
- 2026-10-23 to 2026-10-31: sessions 40-70 and the listening and decisions between them. `docs/history/status-2026-10-23-to-10-31.md`
- From 2026-11-02: sessions 72 on, below.

The older entries that record the user's listening answers, verdicts
and decisions, which later sessions cite:

- "2026-10-06: checks by ear". `docs/history/status-2026-10-05-to-10-23.md`
- "2026-10-09: the verdicts by ear". `docs/history/status-2026-10-05-to-10-23.md`
- "2026-10-09: the mixed-CD judgments; the NAS store pruned". `docs/history/status-2026-10-05-to-10-23.md`
- "2026-10-10: session 10, the verdicts by ear at NAS scale". `docs/history/status-2026-10-05-to-10-23.md`
- "2026-10-12: the first listening with the tool". `docs/history/status-2026-10-05-to-10-23.md`
- "2026-10-12: session 15, the verdicts by ear applied, the containment rule and the possible tier measured". `docs/history/status-2026-10-05-to-10-23.md`
- "2026-10-12: after session 15, the answer by ear on Meltdown". `docs/history/status-2026-10-05-to-10-23.md`
- "2026-10-16: after session 22, the listening; `1.mp3` left out of the NAS library". `docs/history/status-2026-10-05-to-10-23.md`
- "2026-10-17: after session 27, the listening of session 23's six questions". `docs/history/status-2026-10-05-to-10-23.md`
- "2026-10-22: after session 38, the listening of the accumulated pack". `docs/history/status-2026-10-05-to-10-23.md`
- "2026-10-25: after session 46, the listening of session 40's pack". `docs/history/status-2026-10-23-to-10-31.md`
- "2026-10-27: session 39's listening questions, and four decisions". `docs/history/status-2026-10-23-to-10-31.md`
- "2026-10-27: the T-Minus vocal mix upload ignored on the NAS". `docs/history/status-2026-10-23-to-10-31.md`
- "2026-10-30: the listening of session 58's pack". `docs/history/status-2026-10-23-to-10-31.md`
- "2026-10-30: four uploads ignored on the NAS, and a listening pack". `docs/history/status-2026-10-23-to-10-31.md`
- "2026-10-30: the listening of the 2026-10-30 pack". `docs/history/status-2026-10-23-to-10-31.md`
- "2026-10-30: session 64, the four ignored NAS uploads pruned, and the evening's answers carried". `docs/history/status-2026-10-23-to-10-31.md`
- "2026-10-31: a listening pack of what is still open". `docs/history/status-2026-10-23-to-10-31.md`
- "2026-10-31: the listening of the 2026-10-31 packs". `docs/history/status-2026-10-23-to-10-31.md`
- "2026-10-31: the last answer, and the user's decisions and views". `docs/history/status-2026-10-23-to-10-31.md`


## 2026-11-02: the user's answers on a quiet track under an identified one

The user answered the six questions an ADR on proposal 4 needs
(sessions 65 and 70):

1. On by default in `identify`, subject to point 5.
2. The same acceptance rule as today's search (ADR 0006).
3. How a partner is printed: open; try an option that seems right and
   judge it in use.
4. Related versions found this way: not printed at first. Gunfinger is
   for learning a title; related versions matter less.
5. The cost: measure it first, then decide whether the step is on by
   default.
6. The reference: the best-aligned rip of the louder track.

## 2026-11-02: session 72, `identify --under`, a quiet track under an identified one, off by default

Brief `docs/brief-72.md` (its design written before code), checklist
`docs/session-72-checklist.md`; notes "Session 72"; ADR 0025
(Proposed); experiment 0272. Without the option Gunfinger's output is
unchanged, byte for byte.

- 04:25 start at `2701507`, clean. Start build `work/bin/s72-start/`
  (= `s70-final`); both regressions identical (0:51; 4:10).
- `gunfinger identify --under`: for each confident tracklist entry, the
  strongest play's file is decoded from the library at each segment's
  line (`decode::Decoder::render`), aligned on the recording and masked
  with the core's `residual` (moved from the harness; session 70's
  *Align*, `over` 2); what is left is searched under today's rule. Not
  reported: finds of the entry's files, of files sharing material with
  the file taken out by stored peaks, and of tracks confident within
  90 s (the same file, or one lining up or tagged alike). A confident
  find is a tracklist entry marked `(under N.)`; the table, CSV (`play`,
  `under_play`) and JSON (`under`) list every find; cue sheet and
  timeline leave them out. Not run on the peak store's own records.
- The user's answers of 2026-11-02 are followed: today's acceptance
  rule, the best-aligned rip as the reference, related versions held
  back, the printing to be judged in use; the default waits for the
  cost (session 73).
- First look (0272): the Stakka mix gains nothing at either scale;
  generated mix-00 gains its quiet partner, right, under entry 5. The
  step takes 1-3 s per confident entry: `identify` 5.8 s becomes 29.3 s
  at 262 tracks, 18.4 s becomes 51.4 s at NAS scale (Stakka).
- The harness's residual calls the core's code and reproduces session
  70's report exactly.
- Final checks with `work/bin/s72-final/`: `scripts/check.sh` green (723
  tests); `regress session-54-linux` and `regress session-64-linux-nas`
  identical; with `--under` off every output of `identify` and `show`
  identical to `s70-final`'s on the three development mixes at both
  scales; `docs/pair-verdicts.txt` and the NAS clusters file
  (`c4d3da2e7e37...`) unchanged.

## 2026-11-02: session 73, the cost and the effect of `identify --under`, measured

Brief `docs/brief-73.md` (its pre-registration committed before the
first run), checklist `docs/session-73-checklist.md`; notes "Session
73"; experiments 0273-0275; ADR 0025 stays Proposed, with its measured
open points. The product did not change.

- 05:44 start at `31ff279`, clean. Start build `work/bin/s73-start/`
  (= `s72-final`); both regressions identical (0:50; 4:35).
- Development mixes at 262 tracks and NAS scale, and Stakka's 14
  leave-out patterns at both scales: with `--under` no tracklist entry
  is gained or lost, no gap line changes, nothing wrong (0273).
- Generated mixes at NAS scale (drops seeds 2050 and 2053, long 2052):
  38 entries gained, all right; quiet partners printed 18 of 32 (default
  2), brief 18 of 32 (6), unrelated partners inside the louder record 29
  of 32 (20); none of 56 confident finds wrong (0274).
- Cost: at NAS scale `identify` takes 2.8-2.9 times as long (about 35 s
  more per hour of mix, 0.1-0.2 GB more memory); the mask on one thread
  is 40-70% of the step; no budget halves it at 90% of the gains (0275).
- By the registered criteria (J1, J2, J4 met; J3, at most twice the
  default's time, not met) the proposal is to keep the step off by
  default until the mask is faster; turning it on as it is costs about
  35 s per hour of mix. The decision is ours (ADR 0025).
- Final checks with `work/bin/s73-final/` (= `s72-final`): both
  regressions identical; `scripts/check.sh` green; the analysis tests
  green after the review's fixes; `docs/pair-verdicts.txt` and the NAS
  clusters file (`c4d3da2e7e37...`) unchanged.

## 2026-11-02: session 74, which tracks use this break? A passage query, measured

Brief `docs/brief-74.md` (its pre-registration committed before the
first scored run), checklist `docs/session-74-checklist.md`; notes
"Session 74"; experiments 0276-0277; research note
`docs/research/2026-11-02-break-search.md`. The product did not change.

- 07:40 start at `8c404d1`, clean. Start build `work/bin/s74-start/`
  (= `s73-final`); both regressions identical (0:51; 3:57).
- `gunfinger-eval break-search` (harness only, tested): a file and a
  span of 20 s, searched against the whole index on the turntable
  ladder with every posting list and with the fullest 1% set aside;
  each file once at its strongest detection of 20 hits or more, the
  query's own cluster apart, the reach of the strongest files.
- At NAS scale it finds 59 of 64 judged shared passages, Special
  Branch's partners and `parts` edges (51 in the first five, 25 first);
  random passages list a median 0 other files (0276).
- Famous breaks line up with up to 403 files (1% of the index) at
  similar strength; the files above an expected one are mostly other
  rips, versions and holders of the same break. The filter hides most
  of them (Special Branch: 124 other files with every list, 25 with
  it); every list costs a median 5.5 thread seconds per query alone
  (0277).
- By the registered bar, not yet worth a design note and an ADR
  (ranking first 39%, bar 50%; the null was dominated by rips outside
  the NAS clusters). The note proposes listening to the lists of three
  famous breaks first; the decision is ours.
- Final checks with `work/bin/s74-final/` (= `s73-final` for
  `gunfinger`): both regressions identical (0:51; 3:47); the NAS-scale
  queries give the same report again; `scripts/check.sh` green (726
  Rust tests); `docs/pair-verdicts.txt` and the NAS clusters file
  (`c4d3da2e7e37...`) unchanged.

## 2026-11-02: session 75, the records of DJ sets and a mixed CD's track fix pruned from the NAS store, new NAS-scale baseline

Brief `docs/brief-75.md`, checklist `docs/session-75-checklist.md`;
notes "Session 75"; experiment 0278. The product did not change.

- 08:35 start at `793d413`, clean. Start build `work/bin/s75-start/`
  (= `s74-final`); both regressions identical (0:51; 3:47).
- With the user's approval of 2026-10-31, `prune --yes` with the NAS
  configuration deleted the 8 peak records, 99 skip notes and 8 tag
  notes (107 files) that the three ignore entries of 2026-10-31 leave
  out (`/sety/`, the Virus on Kiss FM show of 3 May 2000, the Lowdown
  Vol 6 track fix), after `doctor` and `prune` listed exactly those. The
  NAS store: 40,666 records, revision `0bd3f0c23fabc698` (was 40,674,
  `7210fc479da58376`); 1,827 skip notes; no records of ignored files.
- `clusters --from-peaks --reuse-pairs`: nothing dropped; the NAS
  clusters file is unchanged, SHA-256 `c4d3da2e7e37...`, 284 clusters,
  2,875 pair entries.
- At NAS scale no scored result changed (0278): the sweep, the Stakka
  scan and its leave-outs, and calibrate score as before. The pruned files'
  six weak sweep candidates and their weak detection in each Stakka
  scan are gone. 25 other sweep queries changed (21 refits of confident
  right detections by at most 4e-4 percentage points, one of them 14
  hits stronger; four weak candidates) because lists crossed the
  set-aside cut (0242's mechanism): under the old index's lists all 720
  sweep queries equal the old baseline less the pruned files'
  candidates. The leave-out-3 scan's Clockwork speeds moved by less
  than 1e-5 percentage points, by inference for the same reason.
- Baselines: `session-54-linux` (262 tracks, unchanged) and
  `session-75-linux-nas` (NAS scale); compare NAS-scale runs with
  `session-75-linux-nas`, not `session-64-linux-nas`.
- The roadmap's "Library ignore file" records that DJ sets indexed as
  tracks are now left out, while uploads cut from DJ sets stay in the
  library by the user's decision.
- Final checks with `work/bin/s75-final/` (= `s74-final`):
  `scripts/check.sh` green (726 Rust tests); `regress session-54-linux`
  and `regress session-75-linux-nas` identical (0:51; 3:48);
  `docs/pair-verdicts.txt` and the NAS clusters file unchanged.

## 2026-11-05: an external review, the standing rules and a decisions queue

- Two independent read-only reviewers (one GPT model, one Claude model)
  reviewed the roadmap, the experiments and the harness. What we
  checked and adopted is in
  `docs/research/2026-11-05-external-review.md`; the reports are under
  `work/review-2026-11-05/`.
- New: `docs/session-rules.md` collects the standing rules that briefs
  2-75 and the prompts repeated, and `CLAUDE.md` no longer names the
  older briefs as live rules. `docs/decisions-queue.md` lists nine open
  decisions with evidence and a recommendation (the minimum span,
  `--under`'s default, the inverted-channel repair, break search, the
  zero-wrong bar against the seven Cool Rock detections at NAS scale,
  real mixes with roles and a new held-out set, presentation of related
  versions, kept diagnostics, smaller ones). The roadmap starts with an
  ordered active queue.
- Harness defects confirmed in the code, first on the queue: generated
  mixes score a play at the level of its strongest detection by hits
  (`mixes.rs`, `score_play`), not at its highest confidence; `regress`
  compares queries by position without checking counts or identity;
  render caches are keyed by name and size; harness FFmpeg calls run
  without the stall watchdog; `regress` covers only the Stakka mix.
- Stale counts corrected: the test set has one evaluation left
  (`docs/calibration.md`, the roadmap's "More development mixes").
  Nothing was run; no code changed.

## 2026-11-07: session 76, the documents every session reads, made small and current

Brief `docs/brief-76.md`, checklist `docs/session-76-checklist.md`;
notes "Session 76". No code changed and nothing was measured.

- The roadmap is rewritten as items with state, next step and evidence
  (33 KB, was 119 KB); its old text is in `docs/roadmap-history.md`.
- The notes keep sessions 70-76 and the status the entries from
  2026-11-02 on, each with an index; older sections are verbatim in
  `docs/history/`. The calibration register's history sections moved
  there too.
- `CLAUDE.md`, `AGENTS.md`'s module map, `docs/session-rules.md` and
  `README.md` follow the moves; two points about `AGENTS.md` went to the
  decisions queue (item 9).

## 2026-11-07: session 77, generated-mix scoring and `regress` measure what they say

Brief `docs/brief-77.md`, checklist `docs/session-77-checklist.md`;
notes "Session 77"; experiment 0279. The product did not change.

- A generated-mix play now takes the highest level among its matching
  detections (ADR 0013's rule); the strongest by hits still places it.
  `gunfinger-eval rescore` rescored the 205 saved reports offline: 6 of
  54,477 plays moved, all possible to confident and all in 0250
  (amended; its bars and conclusion stand). No other figure moved.
- `regress` checks that the sweep's queries and the scans match before
  comparing and prints "incomparable" for a mismatch; `baseline` writes
  `provenance.json`, which `regress` compares; `regress --strict` exits
  with an error on any difference. Both baselines gained a provenance
  file beside their unchanged reports.
- Final checks with `work/bin/s77-final/`: `scripts/check.sh` green;
  `regress session-54-linux` and `regress session-75-linux-nas`
  identical, also with `--strict`; `docs/pair-verdicts.txt` and the NAS
  clusters file unchanged. Left for session 78: the render cache, the
  harness's FFmpeg watchdog and `regress`'s coverage.

## 2026-11-07: session 78, render receipts, the harness's FFmpeg watchdog and `regress` over all three development mixes

Brief `docs/brief-78.md`, checklist `docs/session-78-checklist.md`;
notes "Session 78"; experiment 0280. The product's behaviour did not
change (the core's watcher gained a mode the harness uses).

- A cached render is reused only when its receipt holds the same recipe:
  the sources' path, size and modification time, the caller's steps
  (excerpt, playback, filters, partner, fault, seed, encoding) and the
  renderer with FFmpeg's version. Older receipts are refused once: the
  sweep's 720 renders were rendered again (about 18 s) and gave identical
  results (0280).
- Every harness FFmpeg run is under the core's stall timeout (120 s,
  ADR 0018). Encodes to a file report their progress, and only a report
  that advanced counts, because FFmpeg repeats its last report while its
  input stalls.
- `regress` also scans the 2003 and Nocny mixes (7/7 and 9/9 at 262
  tracks, 12/12 and 17/17 at NAS scale, 0 wrong); a baseline saved
  without them is reported as incomplete. Baselines `session-78-linux`
  and `session-78-linux-nas`.
- Final checks with `work/bin/s78-final/` (`9b0ac59`): `scripts/check.sh`
  green (on a rerun after the known "Text file busy" test race);
  `regress session-78-linux` and `regress session-78-linux-nas`
  identical, also with `--strict`; `docs/pair-verdicts.txt` and the NAS
  clusters file unchanged. A review fixed `shared --harmonic`'s
  reference, which this session's receipts had broken before any run.

## 2026-11-07: session 79, the fourth drop rule of `identify --under` and its masks on the workers

Brief `docs/brief-79.md`, checklist `docs/session-79-checklist.md`;
notes "Session 79"; experiments 0281-0282. `identify --under` changed;
it stays off by default and ADR 0025 stays Proposed. Without the option
every output is unchanged.

- The fourth drop rule drops a play found under an entry for lining up
  with a confident play only when the two are one passage (they overlap
  or lie at most 10 s apart); tags alike still count within 90 s. Both
  partners session 73 lost are kept, and on 64 generated drops mixes
  (two seeds of 0274, two fresh) the fix released 9 plays, all right
  partners, with no wrong line, lost entry or repeated entry (0282).
- Each confident entry's decode and mask run on its own worker; the
  output is byte-identical to the serial step's. At NAS scale the option
  takes 1.85-2.01 times the default's time (before 2.85-2.94), about 17 s
  more per hour of mix, and 2.0-2.8 GiB more peak memory. The registered
  bar of twice fails on Nocny by 0.3 s (0281).
- Decisions queue item 2 holds the evidence for the default; real
  annotated double drops, key lock, broadcast compression and a mistaken
  foreground are unmeasured.
- Final checks with `work/bin/s79-final/`: `scripts/check.sh` green;
  `regress session-78-linux` and `regress session-78-linux-nas`
  identical with `--strict`; `docs/pair-verdicts.txt` and the NAS
  clusters file unchanged. A review by a subagent found no bug in the
  code and corrected several figures in the documents.

## 2026-11-07: session 80, the precision of `identify --under` on harder generated cases

Brief `docs/brief-80.md`, checklist `docs/session-80-checklist.md`;
notes "Session 80"; experiments 0283-0284. A measurement session: the
product is unchanged, the option stays off by default and ADR 0025
stays Proposed.

- The harness's drops kind gained `--case` (key lock under or over, a
  ride of the louder record, a third record, a pair's record left out of
  the library as the louder or the quieter one); broadcast processing
  was applied to finished renders. 12 mixes per case at NAS scale, with
  and without the option.
- Broadcast, key lock, rides, three records and long mixes with no
  partner: 86 entries gained, all right, and no wrong confident find
  among 145 (0283).
- A mistaken foreground: the registered bar failed through 2 entries,
  in the pair drops of two mixes whose records share material and are
  both out of the library; each prints a related version (a VIP) of the
  partner that plays. A left-out quiet partner: no related version
  printed (0284).
- Decisions queue item 2 and ADR 0025's open points hold the evidence;
  real annotated double drops remain unmeasured.
- Final checks with `work/bin/s80-final/`: `scripts/check.sh` green;
  `regress session-78-linux` and `regress session-78-linux-nas`
  identical with `--strict`; `docs/pair-verdicts.txt` and the NAS
  clusters file unchanged. A review by a subagent found no change to the
  default path and corrected the documents' readings of 0284.


## 2026-11-07: session 81, short chains under library perturbations, and a bounded fit for short chains only

Brief `docs/brief-81.md`, checklist `docs/session-81-checklist.md`;
notes "Session 81"; experiments 0285-0287. A measurement session: the
product and its defaults are unchanged.

- A registered perturbation panel (files added and removed, duplicate
  copies, the set-aside cut moved by `--set-aside-share`) over the grid's
  10-40 s plays and the sweep, at 262 tracks and NAS scale: no confident
  answer changed. Short plays' detections move far more than long ones'
  (up to a rung of speed; a 20 s play lost a third of its hits, a 10 s
  play nine tenths), but at today's rule
  short plays near it fail on windows, not hits (0285, 0287).
- The bounded fit for short chains only (`--second-pass-fit
  iterated-short`) removes most of that movement but lifts calibrate's
  strongest false at 262 tracks from 100 to 107; it fails its registered
  bar and is not proposed (0286).
- For the minimum span (decisions queue item 1): under B(15) hits would
  decide 15-20 s plays; recall's brief plays near 200 hits were not
  perturbed and are the check to run before the rule is frozen.
- Final checks with `work/bin/s81-final/`: `scripts/check.sh` green
  after a test-only fix of `decode::watch`'s "Text file busy" race;
  `regress session-78-linux` and `regress session-78-linux-nas`
  identical with `--strict`; `docs/pair-verdicts.txt` and the NAS
  clusters file unchanged. A review by a subagent found no code error
  and corrected several readings in the documents.

## 2026-11-07: session 82, two records for a file with an inverted channel, a harness pilot

Brief `docs/brief-82.md`, checklist `docs/session-82-checklist.md`;
notes "Session 82"; experiments 0288-0289. A measurement session: the
product and its defaults are unchanged.

- The user's idea of 2026-10-31, piloted in the harness
  (`--second-records`): a flagged file keeps its record as ripped and
  gains the repaired one as a second asset; at the file only the
  stronger play is kept, and hits are never added.
- Both kinds of query now find the four plainly inverted recordings in
  full (48 of 48 excerpts and 4 of 4 mixes each), where today's index
  misses the correct-polarity kind and replaced records (0263) the
  as-ripped kind; nothing new is named on the plan's queries, the sweeps
  or the shared panels' development halves (0288, 0289).
- Every move checked is the set-aside cut's; the second records add
  0.011% of the NAS index's postings. The registered bar holds; the ADR
  is the user's decision (decisions queue item 3).
- We - Polynesian Inventor, heard inverted in part, is not flagged and
  stays unrecovered in its inverted windows.
- Final checks with `work/bin/s82-final/`: `scripts/check.sh` green;
  `regress session-78-linux` and `regress session-78-linux-nas`
  identical with `--strict`; `docs/pair-verdicts.txt` and the NAS
  clusters file unchanged. A review by a subagent found one code error
  in the file-level merge (no effect on this session's reports) and
  smaller points; all fixed.

## 2026-11-08: session 83, the band profile of 0034 under the default matcher

Brief `docs/brief-83.md`, checklist `docs/session-83-checklist.md`;
notes "Session 83"; experiments 0290-0291. A measurement session: the
product, its profile and its defaults are unchanged.

- Both of 0034's band profiles, run as variant builds on their own
  stores at 262 tracks under today's default matcher, keep every answer
  of the protocol, `recall`, `grid` and `robust` with no wrong confident
  answer, and raise the development mixes' true hits by 16-32% (a) and
  24-39% (b) (0290).
- Neither passes its registration: neither gains the registered 2
  points on recall's damaged sets (+0.38, +1.21); a loses `robust`'s
  equal-level blends; b strengthens shared material on the second
  shared panel (0291); B4's "unrelated" is not shown for either.
- The choice between closing the item and a revised test is the user's
  (decisions queue item 10).
- Final checks with `work/bin/s83-final/`: `scripts/check.sh` green;
  `regress session-78-linux` and `regress session-78-linux-nas`
  identical with `--strict`; `docs/pair-verdicts.txt` and the NAS
  clusters file unchanged. A review by a subagent found no figure error
  and corrected several descriptions and B4's verdict.

## 2026-11-08: session 84, a second-chance search over the gaps

Brief `docs/brief-84.md`, checklist `docs/session-84-checklist.md`;
notes "Session 84"; experiments 0292-0293. A measurement session: no
default of the product changed.

- The harness can search extra rungs or the key-locked ladder only over
  the stretches without a confident match (`--gap-rungs`,
  `--gap-key-lock`; `gap_search.rs`).
- Targeted extra rungs give the whole-query ladders' sweep recall query
  for query at ×1.000-1.003 of the default's CPU on the development
  mixes at NAS scale (one extra rung pair over the whole mix ×1.045),
  but T1 fails its registration on generated edge mixes (returning
  plays count as covered) and T2 brings back the Cool Rock bootleg
  answer (0292).
- Key lock over the turntable ladder's gaps passes its registration:
  the same answers for 0.56-0.58 times both ladders' CPU (0293;
  decisions queue item 9).
- Final checks: see the notes' "Final checks".
