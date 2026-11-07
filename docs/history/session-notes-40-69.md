# Session notes, sessions 40-69

The sections of sessions 40-69 moved verbatim from `docs/session-notes.md` at commit
`009fe55` in session 76, with their headings unchanged. The index of
every session is at the top of `docs/session-notes.md`.

## Session 40

### What changed

The product, the clusters' join rule and the clusters file did not
change, and `regress session-32-linux` and `regress session-33-linux-nas`
stay identical with the new harness code. Added, harness only and
tested:

- `pair` reports, for each alignment at the fitted speed, the seconds of
  the shorter file it supports (`supported_seconds`, beside the
  unchanged `supported` share), and searches its pairs one per worker
  thread (`--jobs`; the report keeps the listed order). Test
  `pair::tests::supported_seconds_leave_out_a_stretch_the_query_replaces`.
- `scripts/analysis/pair_offsets.py`: `populations` (the four labelled
  populations, pair files and labels), `measures` (nine measures per
  `pair` report) and `scores` (AUC per comparison, best threshold with
  leave-one-out errors, the pairs each measure gets wrong).
- `listening_pack.py select --min-hits N` (default 30) leaves out
  borderline pairs on fewer hits; `--disagreements N --measures FILE`
  picks the N joins holding the least near the main offset and the N
  apart pairs holding the most; sheets print the near-offset share when
  an item has it.
- Python unit tests on fixtures (`scripts/analysis/test_pair_offsets.py`,
  `test_listening_pack.py`, Python's `unittest`), run by
  `scripts/check.sh` when Python 3 is installed.

In short: the share of the shorter file held by alignments near the
main offset (`near_10`) orders the 60 judged pairs slightly better than
coverage and supported time (AUC 0.95 against 0.90 and 0.94; 6
leave-one-out errors against 9 and 7), which meets the registered bar by
one pair. It does not separate the two pairs it came from at the
threshold the judged pairs fit, and on the 14 hardest judged pairs
(drifting or edited uploads of one recording against versions differing
in one layer, chosen from the measures' errors) the AUCs are 0.40-0.56
(coverage 0.06), near chance by that choice. 18 of the 25 false confident
detections over three NAS sweeps are China Cup's near-identical upload.

### Step 1: the pre-registration (brief 02:20, `dd7ed06`)

Populations: (a) 62 of 64 verdicts have records (`1.mp3` is left out of
the NAS library; the Synthesis (Remix) track sits on Dangerous Drums 2,
a mixed CD the `.gunfingerignore` lists), 60 without the two source
pairs (22 same, 38 different); (b) 100 joins drawn (98 distinct pairs:
the clusters file lists 313 of its 2,590 pairs in both directions, and
two were drawn both ways) at 80% or more that no
verdict touches, seed 4001; (c) 51 cross-version pairs of 16 panel
families, at most 4 per two recordings, seed 4002; (d) the 60 strongest
apart pairs whose files differ in both compared artist and title.
Pairs with a file in one of experiment 0065's 33 mixed-CD candidate
folders are left out everywhere (one (d) pair had one). Measures:
coverage, supported time, hits per second, `near_T` (T = 2, 5, 10, 20),
`longest_gap`, `far_jumps`. Bar: leave-one-out errors on the judged
pairs fewer than both coverage's and supported time's, and no (b) join
below the threshold.

### Steps 2-4 and experiments 0186-0187

`pair` on the four populations took 31-52 s each (32 jobs). 0186:
`near_10` meets the bar (6 errors against 9 and 7, no (b) join below
0.552); `longest_gap` ties supported time and puts two (b) joins below
its threshold; `far_jumps` and `hits_per_second` do worse. Coverage,
supported time and `near_T` separate (b) from (a)'s different, (c) and
(d) almost perfectly (AUC 0.99-1.00); counting (b)'s 98 distinct pairs
once moves its AUCs by 0.001 at most. 0187:
Fractles ~ the CITRUS 003 upload (same; the upload drifts from 0.9688
to 0.9657, and one rung at the fitted speed keeps 68 s near the main
offset) is wrong under every measure; Mind Overload's RAMM 23 rip ~ the
Molten Beats track, We Enter ~ its full instrumental and Sonar's Mark
System Revision ~ the PRO012 rip (all different) under 6-7 of 9. An
exploratory variant over every search's spans, which follow drift, does
worse (AUC 0.94).

### Step 5a and experiment 0188

The saved NAS-scale reports hold 7 false confident detections, all
Cool Rock Stuff ~ the Mobb Deep / Urban Takover upload in sweep 2026;
the 11 development scans at NAS scale none. Seed 2026 has no China Cup
query, so seeds 2027 and 2028 ran at NAS scale: 11 and 7 false
confident, all China Cup's three corpus rips found as ƆƐI3C's upload.
Of the 73 distinct pairs behind false candidates of 30 hits or more
(84 rows of seed and pair), only China
Cup's three lie above `near_10`'s threshold (0.77).

### Step 5b: the pack

`work/s40/pack/`: 12 disagreements and 2 control joins, from 1,837 more
entries measured (1,595 distinct pairs; 22:51). No join without a verdict
lies below `near_10`'s threshold (1,055 distinct joins, lowest 0.63, 9
below 0.8) and no apart pair above it (638, highest 0.30), so the pack
holds the pairs nearest the threshold on each side. The review found
that the first pack held the Phoenix join whose TECH012 file is an
identical copy of a judged file; `--disagreements` now counts such
copies as judged, and the pack was built again.

### Times on the Linux machine (32 workers)

| Run | Time | Peak memory |
|---|--:|--:|
| `regress session-32-linux` (start; step 2; final) | 1:10; 1:09; 1:09 | 0.4 GiB |
| `regress session-33-linux-nas` (start; step 2; final, again) | 6:15; 5:56; 5:56, 5:58 | 5.3 GiB |
| `pair`, the two source pairs | 0:45 | 0.15 GiB |
| `pair`, populations a-d (51-100 pairs each) | 0:31-0:52 | 0.6-1.1 GiB |
| `pair`, the pack's 1,837 entries (beside the sweeps) | 22:51 | 1.5 GiB |
| `sweep --seed 2027` and `2028` at NAS scale (beside `pair`) | 8:34; 8:19 | 5.2; 4.9 GiB |
| `listening_pack.py render`, 14 items | 2:01 | |
| `scripts/check.sh` (beside `pair`, failed; again; final; after the review) | 4:01; 3:47; 3:45; 3:45 | |

### Decided this session, open to change

- The two source pairs are left out of every score and reported apart;
  one comparison repeats the judged scores with them.
- Pairs with a file in one of experiment 0065's mixed-CD candidate
  folders (joined boundaries 0.75 or more) are left out of every
  population.
- "No verdict touches" means: for (b) neither file is named in any
  verdict; for (d) and the pack's apart pairs no verdict names a file of
  each of the two clusters.
- (d) takes the strongest pair of each two clusters, by hits.
- The measures use the fitted-speed search only (one rung), as the
  sheets do; by inference from their alignments, that is why drifting
  uploads fail (0187).
- Two NAS-scale sweeps (2027, 2028) were added to step 5a because the
  saved sweep has no China Cup query.
- The pack holds the 6 joins and 6 apart pairs nearest the threshold,
  though none crosses it.

### Decisions waiting (ours)

- Whether the join rule should use the near-offset share (alone, or
  beside coverage), with an ADR. The evidence: one pair better than
  supported time on 60 judged pairs, no help on the hard ones, and no
  join or apart pair without a verdict that it would change at 0186's
  threshold.
- Whether a confident detection of a near-identical version (China Cup
  ~ ƆƐI3C, 18 of 25 false confident detections at NAS scale) counts as
  wrong in the sweep and calibrate; today it does.
- Whether a near-offset measure should follow drift (the ladder's
  chains or the audio second pass instead of one rung) before any rule
  is built on it.

### Questions for listening (none answered by agents)

- The pack `work/s40/pack/` (README in the order to listen): six joins
  holding the least near the main offset (0.68-0.81 of the file;
  lowest Alien Girl's upload ~ the PRO014 rip) and six apart pairs
  holding the most (0.16-0.30), and two control joins. We R The Ones
  VIP ~ the CERT1853 Klute remix is in it although a verdict judged the
  VIP against another file of the remix (not an identical copy, and in
  no cluster with it).
- Session 39's: Digital & Spirit - Crash over the first 56 s of the
  T-Minus vocal mix; Techno Funk over Alien Girl and NIL - Infectious
  over Mystery Machine.

### Handoff for the next session

1. Reports: `work/reports/library-nas-dnb-peaks/pairs-s40-{source,a,b,c,d,false,false-2,pack}.json`
   (`pairs-pairs.json`, the 2026-10-22 pack's, untouched),
   NAS-scale sweeps of seeds 2027 and 2028 in `work/s40/sweeps-nas/`
   (moved out of the reports directory, see below).
2. Measures and scores: `work/s40/measures.json`,
   `measures-source.json`, `measures-pack.json`, `scores.json`,
   `false/measures-all.json` and `false/candidates-all.json`; the
   populations in `work/s40/populations/` (`populations.json`, pair
   files).
3. Builds: `work/bin/s40-start/` (= `s39-final`), `s40-run/` (the
   `pair` change), `s40-final/` (master).
4. Commands (`sh` scripts in `work/s40/`: `start.sh`, `pairs.sh`,
   `step2.sh`, `sweeps.sh`, `packpairs.sh`, `check2.sh`, `final.sh`):

   ```sh
   N=~/.local/share/gunfinger/nas-dnb-peaks; R=work/reports/library-nas-dnb-peaks
   python3 -I scripts/analysis/pair_offsets.py populations $R/duplicate-clusters.json docs/pair-verdicts.txt $N work/peaks $R/library-map.json docs/experiments/data/0065-nas-candidates.json work/s40/populations docs/panels/shared-material-seed-2026.json docs/panels/shared-material-2-seed-2026.json
   work/bin/s40-final/gunfinger-eval --other-peaks-dir $N pair work/s40/populations/s40-a.txt
   python3 -I scripts/analysis/pair_offsets.py measures work/s40/measures.json $R/pairs-s40-a.json $R/pairs-s40-b.json $R/pairs-s40-c.json $R/pairs-s40-d.json
   python3 -I scripts/analysis/pair_offsets.py scores work/s40/populations/populations.json work/s40/measures.json work/s40/scores.json
   python3 -I scripts/analysis/listening_pack.py select $R/duplicate-clusters.json docs/pair-verdicts.txt work/listening/store-paths.txt work/s40/pack/pairs.txt work/s40/pack/items.json --controls 2 --seed 2026 --map $R/library-map.json --disagreements 6 --measures work/s40/measures-for-pack.json
   ```

5. Next, if we continue: the decisions above; a near-offset share that
   follows drift (alignments of the ladder's chains, or the audio
   second pass, with their own supported seconds), measured on the same
   populations against the same bar, as a second use of the judged
   pairs; the pack's verdicts.

### What else to know

- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. No mix was searched (the sweeps
  search excerpts of library files). Nothing ran on the candidate mixes
  of 0065 or on files of its mixed-CD candidate folders, nor on the NAS
  root's broadcast file. The NAS was only read (records, tags and, for
  the pack's clips, FFmpeg decoding); the NAS store only read;
  `work/peaks` only read.
- `docs/pair-verdicts.txt` is unchanged (SHA-256 in
  `work/s40/listening-digests-start.txt`), and so is the clusters file
  (`work/s40/clusters-digest-start.txt`); no answer log; no manifest or
  committed panel changed (`work/s40/panels` is a copy of
  `work/s33/panels`).
- The first `check.sh` failed one load-sensitive test
  (`decode::watch::tests::a_probe_that_writes_nothing_is_an_error`)
  while `pair` used every thread beside it; the rerun on a quiet machine
  was green.
- The experiments were committed before the `pair` code that produced
  them (`71f227e`, `1e5af33` before `f00f2a0`), because the code commit
  waited for `check.sh`'s rerun; the code ran unchanged as
  `work/bin/s40-run`.
- `calibrate` (in `regress`) reads every `sweep-seed-*.json` in the
  reports directory: with the two new NAS sweeps there, the final NAS
  regression's calibrate lines changed (false accepted 7 -> 25) while
  every report was identical. With the sweeps moved to
  `work/s40/sweeps-nas/` it is identical. Keep extra sweeps out of
  `work/reports/library-nas-dnb-peaks/` before `regress`, as with slot
  leave-out scans.
- Final checks (03:05-03:26) with `work/bin/s40-final/` (built from
  `c583a34`; its `gunfinger-eval` is byte-identical to `s40-run`, its
  `gunfinger` to the start build's): `scripts/check.sh` green, after
  the review's script fixes too; `regress session-32-linux` and
  `regress session-33-linux-nas` identical (the second once the extra
  sweeps had moved), every report identical to its baseline's apart from
  `wall_seconds`; `docs/pair-verdicts.txt` and the clusters file at their
  start digests.
- The read-only review (03:05-03:13) found counts of list entries where
  distinct pairs were meant (the clusters file lists 313 pairs both
  ways), a judged pair in the first pack through an identical copy, a
  contradiction between 0186 and 0187, wrong run times and inferences
  stated as facts; all fixed (brief, "03:05-03:13").

## Session 41

### What changed

One exact change to the search is in master (`7269498`, experiments
0189-0192): the pre-filter (`OffsetSlots::may_form_line`) counts
integer offsets that `take_part` stores in the grouped hits, instead of
`f64` offsets turned into slots per hit, and clears its counts by
reading the hits again instead of keeping a `touched` list. Each point
of a searched block gets a phase, `floor(frame) mod 2^16` (`Phases`); a
grouped hit (`Hit`) keeps its posting's asset and, in the frame bits,
the frame minus its anchor's phase modulo 2^16, from which `Hit::slot`
folds the slot in integers and `Hit::frame` gives the frame back
exactly for the 2% of assets that are sorted. The integer offsets lie
within one frame of the offsets, so a cluster's offsets still lie
in one slot or two neighbouring ones (the argument is in the method's
documentation); a block with a point 2^31 frames or more from frame 0
sorts every asset. The filter may pass some assets the old one
rejected; those hold no cluster, so sorting them gives no line. Nothing else changed: lines,
detections, evidence, boundaries, speeds and orderings are identical in
every report compared.

In short: on the Linux machine the Stakka scan's search at NAS scale
takes 26.7 s instead of 37.6-37.9 s at 32 workers (29-30% less) and
32.5 s instead of 45.4-45.6 s at 16 (28-29%), with 31-32% less CPU at 32 workers and 30% at 16;
`identify` of the Stakka mix through the CLI at NAS scale 19.8 s
instead of 25.1-25.2 s; the clusters sample 1:21 instead of 1:57;
`regress session-33-linux-nas` 4:12 instead of 6:06; the full NAS
clustering 14:36 instead of 23:02, byte-identical to the start build's
own run.

### Step 1: where the pre-filter's time went (experiment 0189)

A diagnostic build (`work/s41/diag.diff`) timed seven passes per part,
each adding a piece of the filter. At 16 (32) workers: loads 14% (15%),
the slot arithmetic 37% (46%), the `touched` list and its clearing 27%
(16%), counting and the test 23% (24%). On the baseline x86-64 target
`f64::floor` is an indirect call (no SSE4.1 `roundsd`); on the Mac it is
one instruction, by inference why 0108's integer phases saved only 13%
there. A filtered asset holds 44 hits on average and nearly all are
read: the early exit spares 1.4% of hits, and a passing asset passes
late in its hits. So a cheap first test cannot help.

### Steps 2 and 3: the candidates (0190, 0191)

Pre-registered (brief 41, 03:57, `b2c019f`) before any timing; the stop
point a cut of 3% or more at 32 workers against the start build in
every pair. One round, A B B A at 32 and 16 workers (search time, cut
at 32 workers):

| Candidate | 32 workers | Cut |
|---|--:|--:|
| Start | 37.60, 38.05 s | |
| C1 integer slots from per-point phases | 30.76, 30.85 s | 18.2-18.9% |
| C2 integer offsets in the grouped hit | 29.13, 29.28 s | 22.5-23.0% |
| C3 stamps instead of `touched` | 35.85, 35.92 s | 4.6-5.6% |
| C4 C2, cleared by re-reading the hits | 27.14, 27.38 s | 27.8-28.0% |
| C5 one 4 KiB fill instead of `touched` | 35.32, 35.35 s | 6.1-7.1% |
| G1 counting assets while taking a part | 38.59, 38.39 s | -2.6, -0.9% |

C1-C5 passed; by the registered rule the combination was C4. An
exploratory round (0191, its rule set before it ran) found one fill on
integer slots 0.7-0.9% faster than C4 at 32 workers and slower at 16,
and stamps slower than both: C4 was kept. G1 was dropped; no other
grouping candidate was tried.

### Steps 4 and 5: adoption and its times (0192)

`scripts/check.sh` green; both regressions identical, each report too
apart from `wall_seconds`; the 11 development scans at NAS scale and the
11 at corpus scale (262 tracks) and the clusters sample byte-identical
to the start build's (`work/s41/validate.sh`); the full NAS clustering
(session 33's run 2 command) in scratch copies with the start build
(23:02) and the adopted build (14:36), byte-identical (SHA-256
`be08df5b1948...`). The clusters file in use was written by earlier
builds and the listening's verdicts: its clusters are the same as the
runs', its pair entries are not, so it was not the reference.

| Run (Linux machine, 3600 MT/s, 32 workers unless stated) | Start (`s41-start` = `s40-final`) | Now (`7269498`) |
|---|--:|--:|
| Stakka scan at NAS scale, search | 37.64, 37.92 s | 26.66, 26.69 s |
| The same, user CPU | 1,088, 1,096 s | 747, 748 s |
| The same, 16 workers | 45.63, 45.41 s | 32.51, 32.53 s |
| Clusters sample of 48 | 1:56, 1:58 | 1:22, 1:21 |
| `identify` of the Stakka mix (NAS configuration) | 25.1, 25.2 s | 19.8, 19.8 s |
| `identify`'s user CPU | 595, 597 s | 427, 427 s |
| `regress session-33-linux-nas` | 6:06 | 4:12 |
| `regress session-32-linux` | 1:09 | 1:09 |
| Full NAS clustering (scratch copies) | 23:02 | 14:36 |

### What the search's CPU goes to now (0193)

With 0193's timers (exploratory, 16 and 32 workers) the first pass,
96-97% of the scan's CPU, now goes to grouping a part by asset 40% and
35%, the pre-filter 20% and 22% (93 s instead of 288 s at 16 workers,
1.9 ns a hit instead of 5.8), query analysis 20% and 22%, taking parts
10% and 11% (by inference from 0170's timers, about 5 s more for the
integer offsets),
aligning, sorting and lines 9%, phases, hashing and lookups under 1%.

### Decided this session, open to change

- Candidates were tested with the core crate's tests before timing;
  `scripts/check.sh` ran in full for the adopted code (a scaling down
  under the time box, brief 41).
- The 262-track scans were read as the 11 development scans at corpus
  scale, against the start build's reports.
- The fill (0.7-0.9% faster than C4 at 32 workers, slower at 16) was not
  taken: below the 3% rule set before its round.
- Grouping: only G1 was tried.

### Handoff for the next session

1. Master's search is 0192's. Builds: `work/bin/s41-start/` (=
   `s40-final`), `s41-adopt/` (validated), `s41-final/` (master at the
   end), the candidates `s41-c1` to `s41-c5`, `s41-c2fill`,
   `s41-c2stamp`, `s41-g1`, and the diagnostics `s41-diag`,
   `s41-timers` (sources `work/s41/lines.rs.*`: `lines.rs.adopt` is
   `7269498`, `lines.rs.final` its draft before rustfmt and the clippy
   fix; `diag.diff`, `timers.diff`).
2. Scripts (`sh`, from the repository root): `work/s41/ab.sh <tag>
   <base> <builds...>` (A B ... B A on the Stakka scan in the scratch
   work directory `work/s41/w`; `ab.py` summarises), `validate.sh`,
   `devscans.sh` and `corpusscans.sh` (11 scans at each scale against
   the start build's reports in `work/s41/dev-start/` and
   `corpus-start/`), `timing.sh`, `full.sh`, `diag.sh`, `timers.sh`.
3. What is left: grouping a part (40% of the first pass's CPU at 16
   workers, 35% at 32) is the largest stage; narrower hits (0171),
   grouping from the lists (0172) and counting while taking (G1, 0190)
   were all slower, so it needs another idea. Query analysis (20-22%)
   and the pre-filter (20-22%) follow; the filter's remaining cost is
   its loads, counts and the re-read. The fill (C2 with one fill, 0191)
   was within 1% of C4.
4. All times are on the Linux machine at 3600 MT/s. On the Mac the
   saving should be smaller, since its `floor` is one instruction (0108
   measured 13% of the stage there with phases alone); not measured.

### What else to know

- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. The development sets were named
  one by one. Nothing ran on the candidate mixes of 0065 or on the NAS
  root's broadcast file.
- The NAS was only read (the library listing of `identify`); the NAS
  store only searched; no `index` or `prune`. The full clustering run
  wrote only its scratch directory; the clusters file in use is
  unchanged (SHA-256 `d9c85c4586ad...`).
- `docs/pair-verdicts.txt` is unchanged (SHA-256 in
  `work/s41/listening-digests-start.txt`); no answer log; no manifest or
  committed panel changed. `work/s41/panels` is a copy of
  `work/s33/panels`.
- A read-only review by a separate agent (05:47-05:55) found the code
  correct (the exactness argument, the guard, the round trip, the
  clearing on every path) and 14 points in the documents: four wrong
  figures, two claims that were inferences, a pass missing from 0189's
  table, the tests' description in the roadmap, wrapping and the
  handoff's sources. All fixed (brief 06:00), with two comments in
  `lines.rs`, so the final build was made again.
- Final checks (05:56-06:10) with `work/bin/s41-final/` built from
  `7dbaa8a`: `scripts/check.sh` failed once with ADR 0020's "Text file
  busy" race in `decode::watch`'s tests, then green on the idle machine
  (3:44); `regress session-32-linux` (1:08) and `regress
  session-33-linux-nas` (4:11) identical, every sweep query, scan
  detection and calibrate line, and each report identical to its
  baseline's apart from `wall_seconds`; `docs/pair-verdicts.txt` and the
  clusters file in use unchanged.

## Session 42

### What changed

The decoder, the product and every default are unchanged; `regress
session-32-linux` and `regress session-33-linux-nas` stay identical.
Added, harness only and tested (`b99f59e`):

- `robust`'s nine stereo conditions, run only when named in `--only`
  (`stereo-control`, `stereo-inverted`, `stereo-silent`,
  `stereo-noise-snr-10db`, `-0db`, `--10db`, `stereo-delay-0.2ms`,
  `-1ms`, `-5ms`): the excerpt rendered with both channels
  (`render::render_stereo_samples`), the right channel faulted in the
  harness, encoded as a stereo MP3 at 128 kbit/s (`encode_stereo`). Each
  stereo query's report keeps its side-to-mid ratio and its wrong
  detections by name (`QueryResult::stereo`, absent elsewhere, so the
  default report is unchanged).
- `robust --query-channels downmix|left|right|louder|side`
  (`stereo::QueryChannels`): `downmix`, the default, is the core's
  `decode`; the others decode two channels and pick. Report
  `robust-seed-<seed>-channels-<choice>.json`.
- `gunfinger-eval stereo-width` (corpus only): a seeded sample of
  library files, each decoded whole as two channels.
- `mixes --stereo-fault <condition>` (short mixes only): stereo renders
  with the right channel faulted over the second half, in their own
  render directory and report; the mono path renders byte for byte as
  before (checked on the 16 mixes of seed 2026).
- `scripts/analysis/robust_stereo.py` (with `test_robust_stereo.py`).

FFmpeg's `-ac 1` adds the two channels at 1/sqrt(2) each (measured in
`stereo`'s decoding test).

In short: an inverted channel costs everything (0 of 80 excerpts
confident at both scales, 0 of 20 when the rip carries it, 1 of 100
plays in a faulted half mix); noise at -10 dB in one channel costs
everything, noise at 0 dB and a 5 ms delay about half; a 1 ms delay
costs 10 of 80; a silent channel 2.
No fault raises a wrong confident answer. No channel choice meets the
pre-registered bar for a fallback.

### Step 1: the pre-registration (brief 42, 06:16, `e129851`)

Faults on the right channel of `robust`'s excerpts (seed 2026, 40
indexed and 10 held-out draws at speeds 0.95 and 1.03), against a
stereo control; channel choices `downmix`, `left`, `right`, `louder`,
`side`, and `either` as the union of `left` and `right`; widths mono
(-40 dB or less), nearly mono (to -20 dB), narrow, wide; a trigger (side
louder than mid). A fault is worth its own roadmap entry when it loses 8
or more of 80 confident excerpts at either scale or raises a wrong
confident answer; a fallback is worth an experiment when a choice made
without knowing the faulted channel wins back half the loss at both
scales while staying within 1 of 80 on the clean control and adding no
wrong confident answer (the side only behind a trigger that fires on no
clean query and on at most 1 of 100 sampled corpus files).

### Steps 2-4: experiments 0194-0196

| Fault, confident of 80 (both scales) | Downmix | Left | Right | Louder | Side | Either |
|---|--:|--:|--:|--:|--:|--:|
| None (`stereo-control`) | 80 | 77 | 78 | 77 | 0 | 78 |
| Inverted | 0 | 75 | 77 | 76 | 80 | 77 |
| Silent | 78 | 78 | 0 | 78 | 78 | 78 |
| Noise 10, 0, -10 dB | 80, 29, 0 | 74, 76, 75 | 53, 0, 0 | 62, 2, 0 | 0, 0, 0 | 74, 76, 75 |
| Delay 0.2, 1, 5 ms | 80, 70, 37 | 75, 76, 75 | 76, 78 (77), 76 | 75, 76, 75 | 78, 69, 41 | 76, 78 (77), 76 |

(In brackets where NAS scale differs.) Wrong confident answers: none at
262 tracks; at NAS scale only the held-out Cool Rock Stuff excerpt as
the Mobb Deep / Urban Takover upload (session 40's pair), in the
control as in the mono `control`, and in more fault rows when one
channel is searched. One possible answer is raised at NAS scale:
Cause 4 Concern - Abduction as Cause 4 Concern's remix of Phuture
Assassins' Future Sound (60-68 hits), in `stereo-control` under left,
louder and the unions with them, and in `stereo-silent` under every
choice.

- 0194: the faults worth a roadmap entry by the registered bar: inverted
  (80 lost), noise at 0 dB (51) and -10 dB (80), 5 ms (43) and 1 ms (10).
- 0195: no fallback qualifies: `louder` loses 3 clean excerpts and
  `either` 2 (bar 1); the side wins back every inverted excerpt but
  finds nothing in clean audio, and its trigger fires on 2 clean queries
  (one held-out excerpt, a wide passage) and on 91 of 100 queries
  delayed by 5 ms. Exploratory: the downmix with both channels (three
  searches) keeps all 80 clean excerpts and wins back 77-78 under each
  fault but noise at -10 dB (75).
- 0196: of 100 sampled corpus files (seed 2026, of 265) 5 are mono, 29
  nearly mono, 62 narrow and 4 wide (median -17.7 dB); none has its
  side louder than its mid. The NAS store keeps one peak list per file
  from the downmix, so its sources' width cannot be measured from it.

### Step 5: ahead (0197, 0198)

- 0197: the fault in the indexed rip, each copy in its own scratch
  store: unchanged 20 of 20 confident, inverted 0, silent 19, 1 ms 18,
  5 ms 4 (hits 0.20 of the unchanged copy's, as in the query).
- 0198: 24 short mixes (seeds 2026-2028) at 262 tracks, the fault over
  the second half: confident plays in that half 95/100 (control), 1/100
  (inverted), 37/100 (5 ms), 39/100 (noise at 0 dB); correct seconds
  94.1%, 48.1%, 77.3%, 75.2%; no wrong confident play.

### Times on the Linux machine (32 workers)

| Run | Time | Peak memory |
|---|--:|--:|
| `regress session-32-linux` (start; run build; mix build) | 1:08; 1:08; 1:08 | 0.4 GiB |
| `regress session-33-linux-nas` (start; run build) | 4:13; 4:12 | 5.3 GiB |
| `stereo-width`, 100 files | 0:05 | 1.8 GiB |
| `robust`, 10 conditions, 1,000 queries, 262 tracks (first run renders) | 1:36; 1:04-1:08 | 1.2; 0.3 GiB |
| The same at NAS scale | 3:32-3:57 | 5.0 GiB |
| `mixes`, 8 stereo mixes, 262 tracks | 0:23-0:26 | 3.5-3.7 GiB |
| `scripts/check.sh` | 3:58 | |

### Decided this session, open to change

- The fault is always on the right channel; `left` is the clean channel
  and `right` the faulted one. By symmetry a fault on the left would
  swap them (an inference, not measured).
- Faults are applied in the harness at 44.1 kHz, after the playback and
  before encoding; queries are stereo MP3s at 128 kbit/s like most
  `robust` conditions. The reverse case (0197) used FFmpeg filters on
  whole files.
- An excerpt's width is its `stereo-control` ratio over 30 s; the
  corpus sample measures whole files.
- The two unions with the downmix were added while summarising and are
  labelled exploratory; they decide nothing.
- Step 5b gave each faulted copy its own store, because the CLI merges
  same-audio plays.

### Questions for us

- Whether a decoder fallback is worth its own experiment and an ADR.
  The measured case for one: an inverted channel, in the mix or in the
  rip, loses everything it covers, and nothing in today's output says
  why. Against it: no single choice was free on clean audio (one
  channel loses 2-3 of 80 and a third of the hits), the obvious trigger
  (side louder than mid) also fires on 5 ms delays and on one wide
  clean passage, and searching the downmix and both channels triples
  the search. How often such faults occur in the mixes and the NAS rips
  is not known.
- Whether a per-file check of the two channels while indexing (one
  ratio per record) is worth adding to the store, so that rips with an
  inverted channel could at least be listed; it would change the record
  format.

### Questions for listening (none answered by agents)

- Cause 4 Concern - Abduction (excerpt 9 at +3%) against the SUBBASS
  AUDIO upload of Phuture Assassins - Future Sound (Cause 4 Concern
  remix): do they share material in the left channel?
- The held-out David Carbone - Innocence (Cause 4 Concern remix) excerpt
  whose side is louder than its mid over 30 s: a wide passage or a
  channel fault in the rip?

### Handoff for the next session

1. Builds: `work/bin/s42-start/` (= `s41-final`), `s42-run/` (the
   `robust` harness, used for 0194-0196), `s42-mix/` (master at
   `b99f59e` and after, used for 0197-0198), `s42-final/` (master at
   the end).
2. Reports: `work/s42/corpus/` and `work/s42/nas/` (the ten `robust`
   reports and `summary.json`/`.txt`), `work/s42/w/reports/stereo-width-seed-2026.json`,
   `work/s42/ref/` (0197's copies, stores and reports), `work/s42/mixes/`
   (0198's reports and summary; `seconds` scores in `work/seconds/`).
   Renders: `work/s42/w/robust/seed-2026/stereo-*` (466 MB),
   `work/mixes/stereo-*-seed-202[678]/` (597 MB).
3. Scripts (`sh`, from the repository root): `work/s42/runs.sh` (both
   regressions, the width sample, the ten `robust` runs), `ref.sh` and
   `step5b.sh`, `mixes.sh`, `final.sh`; Python `matrix.py`, `ref.py`,
   `mixes.py`.
4. Commands:

   ```sh
   N=~/.local/share/gunfinger/nas-dnb-peaks; W=work/s42/w
   ONLY=stereo-control,stereo-inverted,stereo-silent,stereo-noise-snr-10db,stereo-noise-snr-0db,stereo-noise-snr--10db,stereo-delay-0.2ms,stereo-delay-1ms,stereo-delay-5ms
   work/bin/s42-final/gunfinger-eval --work $W robust --seed 2026 --only $ONLY --query-channels left
   work/bin/s42-final/gunfinger-eval --other-peaks-dir $N --work $W robust --seed 2026 --only $ONLY
   work/bin/s42-final/gunfinger-eval --work $W stereo-width --seed 2026 --sample 100
   work/bin/s42-final/gunfinger-eval mixes --seed 2026 --count 8 --stereo-fault stereo-inverted
   python3 -I scripts/analysis/robust_stereo.py OUT.json downmix=R.json left=R-left.json right=R-right.json louder=R-louder.json side=R-side.json
   ```

5. Next, if we continue: the questions for us above; a fallback, if
   wanted, needs its own pre-registration (which trigger, which search,
   its cost) and an ADR.

### What else to know

- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. No development mix was searched;
  only `robust`'s excerpts of corpus files and generated mixes. Nothing
  ran on the candidate mixes of 0065 or on the NAS root's broadcast
  file. No NAS audio file was decoded; the NAS store was only searched;
  `work/peaks` only read. 0197's
  scratch stores are under `work/s42/ref/`.
- `docs/pair-verdicts.txt` and the clusters file are unchanged (SHA-256
  in `work/s42/listening-digests-start.txt` and
  `work/s42/clusters-digest-start.txt`); no answer log; no manifest or
  committed panel changed.
- `robust` and `stereo-width` ran in the scratch work directory
  `work/s42/w` (copies of both clusters files and the library map, the
  saved NAS index hard-linked), so no report in `work/reports/` was
  replaced; `mixes` wrote four new report names there, moved to
  `work/s42/mixes/` after each run.
- A read-only review by a separate agent (07:12-07:20) found every
  confident count matching the data and the default paths unchanged,
  and wrong figures, wording and two unrecorded departures from the
  pre-registration; all fixed (brief 42, 07:21), with small code
  changes (the channel choices pick in `decode`; `robust::chosen` tests
  the `--only` filter).
- `robust --only control` with the mix build, rendering afresh in a
  scratch work directory, gave queries and rows identical to
  `work/reports/robust-seed-2026.json` apart from search times.
- Final checks (07:21-07:31) with `work/bin/s42-final/` built from
  `986c3f0`: `scripts/check.sh` green (3:48); `regress
  session-32-linux` (1:08) and `regress session-33-linux-nas` (4:15)
  identical, each report identical to its baseline's apart from
  `wall_seconds`; `docs/pair-verdicts.txt` and the clusters file
  unchanged.

## Session 43

### What changed

One exact change to the search is in master (`fd86305`, experiment
0203): the pre-filter (`OffsetSlots::may_form_line`) counts each pair of
neighbouring slots in a byte (4,097 bytes; the pair of slot 4,095 and
slot 0 in both byte 0 and byte 4,096), so a hit adds to its two pairs
with one 16-bit word, and the test of both is an addition and a mask.
Before, a hit added to its slot's count and read both neighbours' counts
apart. An asset passes exactly when some pair of neighbouring slots
holds 3 of its hits, as before, so every asset gets the same answer;
a test checks the two filters agree on 300,000 random sets, many near
the wrap. Grouping is unchanged: none of the four grouping candidates
passed the stop point. Lines, detections, evidence, boundaries, speeds
and orderings are identical in every report compared.

In short: on the Linux machine the Stakka scan's search at NAS scale
takes 26.0 s instead of 27.2 s at 32 workers (4.2-4.3% less) and 31.6 s
instead of 32.6 s at 16 (3.0-3.1%), with 4.6-4.8% less CPU at 32 workers;
the clusters sample 1:20 instead of 1:23; `identify` of the Stakka mix
through the CLI at NAS scale 19.7 s instead of 20.0 s; the full NAS
clustering 14:12 instead of 15:03, byte-identical to the start build's
own run.

### Step 1: where grouping's time goes (experiment 0199)

Timers per part in a diagnostic build (`work/s43/diag.diff`): at 1,024
assets and 32 (16) workers, the scatter is 27.6% (34.0%) of the first
pass's CPU, about 4.2 (3.2) ns a hit; the count pass 7%, about 1 ns a
hit; the prefix sum 0.1%; taking parts 11% (10%). The scatter costs the
same at parts of 256, 512 and 2,048 assets, and smaller parts make taking
dearer: 1,024 has the lowest first-pass CPU at both counts. A part holds
45,060 hits on average, 44 per asset, in 607 list runs of 74.

### Steps 2 and 3: the candidates (0200-0203)

Three rounds, each pre-registered before it was timed (brief 43:
07:59 `c11f284`, 08:17 `3e2db3c`, 08:29 `1336fde`); the stop point a cut
of 3% at 32 workers against the start build in every pair.

| Candidate | 32 workers | 16 workers |
|---|--:|--:|
| C1 rooms: hits taken straight into a room of 120 per asset | 1.9%, 2.6% | -2.8%, -3.2% |
| C2 rooms with parts of 512 | 0.5%, 1.6% | -3.9%, -3.9% |
| C3 the scatter through a staged cache line per asset | -17.0%, -15.2% | -19.7%, -19.6% |
| C4 ranks: the count pass keeps each hit's rank, the scatter writes start + rank | -0.1% to 0.6% (4 pairs) | 3.4-4.3% |
| C5 pairs of slots in the pre-filter | 4.1%, 4.4% | 3.1%, 3.3% |
| C6 C5 with C4's ranks | 1.9%, 2.8% | 6.2%, 6.8% |

Between rounds 1 and 2, 0201 measured why the scatter is slow: 40.3% of
hits have the asset of the hit before them in a part (a hash repeating in
a track, by inference), and in a microbenchmark such repeats make the
scatter about 0.7 ns a hit dearer, by inference because each such hit
reads the cursor the hit before has just written. Ranks remove that wait
and help at 16 workers, but at 32 workers the second thread of each core
fills it (an inference from C4's times), so only fewer operations per hit
help there; that led to C5.

### Steps 4 and 5: adoption and its times (0204)

`scripts/check.sh` green (after one clippy fix in the new test); both
regressions identical (1:13; 4:05), each report too apart from
`wall_seconds`; the 11 development scans at NAS scale and the 11 at
corpus scale (262 tracks) and the clusters sample byte-identical to the
start build's (`work/s43/validate.sh`); the full NAS clustering (session
33's run 2 command) in scratch copies with the start build (15:03) and
the adopted build (14:12), byte-identical (SHA-256 `be08df5b1948...`, the
same file as session 41's two runs). The clusters file in use was not
the reference (session 41: its pair entries come from earlier builds and
the listening).

| Run (Linux machine, 3600 MT/s, 32 workers unless stated) | Start (`s43-start` = `s42-final`) | Now (`fd86305`) |
|---|--:|--:|
| Stakka scan at NAS scale, search | 27.18, 27.18 s | 26.02, 26.01 s |
| The same, user CPU | 762, 764 s | 727, 727 s |
| The same, 16 workers | 32.59, 32.63 s | 31.60, 31.65 s |
| Clusters sample of 48 | 1:23.0, 1:23.4 | 1:20.1, 1:19.4 |
| Its user CPU | 1,677, 1,672 s | 1,593, 1,591 s |
| `identify` of the Stakka mix (NAS configuration) | 20.0, 20.0 s | 19.7, 19.7 s |
| `identify`'s user CPU | 427, 427 s | 416, 416 s |
| `regress session-33-linux-nas` | 4:20 (beside compiles) | 4:05 |
| `regress session-32-linux` | 1:08 | 1:13 |
| Full NAS clustering (scratch copies) | 15:03 | 14:12 |

### Where the search's CPU goes now (0205)

With 0193's timers on master's code (exploratory, 16 and 32 workers)
the first pass, 96-97% of the scan's CPU, goes to grouping a part by
asset 43% and 36%, query analysis 20% and 24%, the pre-filter 16% and
19% (72 s instead of 93 s at 16 workers, 1.45 ns a hit instead of 1.9),
taking parts 11% and 12%, aligning, sorting and lines 10%, phases,
hashing and lookups under 1%. Of grouping, about four fifths is the
scatter (0199).

### Decided this session, open to change

- Parts of 256 assets were measured in step 1 beside the three sizes the
  prompt named. A part size alone was not made a candidate: 1,024 had
  the lowest first-pass CPU at both job counts (0199).
- After round 1, a microbenchmark (0201, exploratory) chose the next
  candidates; rounds 2 and 3 were pre-registered then, each before its
  timing.
- C6 (fastest at 16 workers, 6.2-6.8%) was not adopted: it missed the
  stop point at 32 workers, which is the default on this machine.
- No fourth round: the time box favoured the adoption checks.
- Candidates were tested with the core crate's tests before timing;
  `scripts/check.sh` ran in full for the adopted code, as in session 41.

### Handoff for the next session

1. Master's search is 0203's. Builds: `work/bin/s43-start/` (=
   `s42-final`), `s43-adopt/` (validated, master at `fd86305`),
   `s43-final/` (master at the end), the candidates `s43-c1` to
   `s43-c6`, and the diagnostics `s43-diag-256` to `-2048`, `s43-diag2`,
   `s43-timers` (sources `work/s43/lines.rs.*`, generators in
   `work/s43/py/`; `lines.rs.adopt` is master's file; `diag.diff`,
   `timers.diff`). The microbenchmark: `work/s43/bench/scatter.rs`.
2. Scripts (`sh`, from the repository root): `work/s43/ab.sh <tag>
   <base> <builds...>` (`ROUNDS="1 2"` for more pairs; `ab.py`
   summarises), `validate.sh`, `devscans.sh`, `corpusscans.sh`,
   `timing.sh`, `full.sh`, `diag.sh`, `timers.sh`, `build-candidates.sh`.
   References: `work/s43/dev-start/`, `corpus-start/`.
3. What is left: grouping's scatter (0199, before C5: about 28% of the
   first pass at 32 workers, 34% at 16). Its cost per hit does not
   depend on the part size (0199) and, by inference from rooms (0200),
   hardly on where it writes from; repeated assets slow it with one
   thread a core, and ranks (C4) cut 3.4-4.3% at 16 workers but not at
   32, where, by inference, the second thread fills those waits. If a
   machine runs one worker per core, C4 or C6 is worth taking again
   (`work/s43/lines.rs.c4`, `.c6`; 0202, 0203). What the filter still
   costs is not split; by inference, its word load and store per hit,
   the wrap test and the second pass that clears. Query analysis
   (20-24%) is, by inference, bound by computation, and must stay
   bit-identical.
5. `work/s43/ab.sh` writes no `start` line per run to `queue.log`; the
   runs' start times are known from their logs.
4. All times are on the Linux machine at 3600 MT/s; the Mac was not
   measured.

### What else to know

- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. The development sets were named
  one by one. Nothing ran on the candidate mixes of 0065 or on the NAS
  root's broadcast file.
- The NAS was only read (the library listing of `identify`); the NAS
  store only searched; no `index` or `prune`. The full clustering runs
  wrote only their scratch directories; the clusters file in use is
  unchanged (SHA-256 `d9c85c4586ad...`).
- `docs/pair-verdicts.txt` is unchanged (SHA-256 in
  `work/s43/listening-digests-start.txt`); no answer log; no manifest or
  committed panel changed. `work/s43/panels` is a copy of
  `work/s33/panels`.
- A read-only review by a separate agent (09:42-09:49) found the code
  correct (the pairs give the old answer for every asset, the wrap
  pair, no carry, the clearing on both paths, the bounds) and points in
  the documents: three placeholders not yet filled, six figures off by
  rounding or arithmetic (0199's asset shares now over assets with a
  hit, 0200's CPU saving, 0202's CPU range, 0204's two cuts, 0205's
  analysis), uncited data (0204's samples and full runs now in
  `data/0204-samples-and-full.tsv`), inferences not marked (0203, the
  handoff), the checklist's combination, wrapping, and two comments
  and a missing assertion in the new tests. All fixed (brief 43,
  09:53); the comments change the source, so the final build was made
  again.
- Final checks (09:53-10:03) with `work/bin/s43-final/` built from the
  tree after the review's fixes: `scripts/check.sh` green (3:52);
  `regress session-32-linux` (1:09) and `regress session-33-linux-nas`
  (4:06) identical, every sweep query, scan detection and calibrate
  line, and each report identical to its baseline's apart from
  `wall_seconds`; `docs/pair-verdicts.txt` and the clusters file
  unchanged. The same checks had passed before the review's fixes
  (09:42-09:51).

## Session 44

### What changed

Nothing in Gunfinger changed: `identify` finds the same plays and prints
the same reports, and the CLI binary is byte-identical to the start
build's. The harness's pair count (`verifier/pairs.rs`, behind
`--verify`) now follows each reference pair to its two peaks and says
why the exact count lost it, and counts the exact pairs the query would
add if each anchor were paired with 3, 5 or 8 targets instead of 2, the
record and the index unchanged. This session measured the open part of
step 2 of the roadmap's "Evidence beyond exact pair hashes".

In short: under combined damage, reference pairs are lost because their
peaks are lost. 88% of the lost pairs have an endpoint missing in the
query; a displaced target is 4.5% and a changed quantised value 6.3%.
Displaced targets are few against what damage removes but equal 0.89 of
the exact pairs that remain, so the fan-out step ran. Pairing the query
with 5 targets adds a median 30% to a damaged true detection's exact
pairs, but only 21 of 507 damaged true detections under 240 would reach
it, and related versions on the shared panels gain more than the played
recording at equal evidence (AUC 0.24-0.27). Step 2's prediction holds;
no rule follows.

### The split (experiments 0206, 0207)

On the verifier's alignment, each reference pair in the passage (the
record's pairs at fan-out 2, as the index holds them) gets session 30's
status (exact, near or unmatched) and a kind from its peaks, each peak
found as the verifier finds peaks (±1 frame, ±1 bin at its place on
the alignment): an endpoint gone (anchor, target, both); both found and
the query pairs them (the target among the query anchor's first two
targets) with the same hash, a near hash or a far one; both found and
the target displaced (rank 3, 4-5, 6-8, 9 or later in the anchor's
zone); both found and outside the zone. Lost means not in E.

| Set (pooled) | n | E of the reference pairs | Endpoint gone | Displaced | Changed value |
|---|--:|--:|--:|--:|--:|
| Clean, own record | 522 | 0.460 | 0.550 | 0.171 | 0.242 |
| Combined damage | 511 | 0.048 | 0.883 | 0.045 | 0.063 |
| Other rips only | 178 | 0.248 | 0.681 | 0.108 | 0.183 |
| Played recording, shared panels | 1,297 | 0.311 | 0.689 | 0.102 | 0.186 |
| Related recordings, shared panels | 5,356 | 0.075 | 0.883 | 0.040 | 0.070 |
| Unrelated candidates | 2,926 | 0.008 | 0.993 | 0.003 | 0.004 |

The last three columns are shares of the lost pairs.

- Under damage an endpoint is gone for 84% of all reference pairs (30% on
  clean audio): the anchor alone, the target alone and both account for
  22%, 22% and 44% of the lost pairs. Damage also changes the pairs that
  keep both peaks (70% of the reference pairs on clean audio, 16% under
  damage): among them the exact share falls from 0.65 to 0.30, while
  displaced pairs rise from 0.13 to 0.27 and changed ones from 0.19 to
  0.38.
- Displaced targets sit just past the fan-out: 55% at rank 3, 37% at 4-5,
  8% at 6-8.
- N recovers the changed values: 93% of the damaged changed pairs
  are in N, and 79% of N is such a pair (21% of N has an endpoint gone, a
  near hash from other peaks).
- On clean audio 17% of the lost pairs are displaced (expected under
  5%): re-encoding and the fitted speed add or move enough peaks that an
  extra query peak comes first in the anchor's zone (an inference; not
  followed peak by peak).

### The fan-outs (experiment 0208)

| Median gain over E | Fan-out 3 | 5 | 8 |
|---|--:|--:|--:|
| Clean, own record | 0.11 | 0.14 | 0.14 |
| Combined damage | 0.18 | 0.30 | 0.32 |
| Played recording, shared panels | 0.12 | 0.16 | 0.16 |
| Related recordings, shared panels | 0.16 | 0.23 | 0.24 |
| Unrelated candidates | 0.00 | 0.09 | 0.10 |

- Related versions gain more than the played recording in every band of
  E, at every fan-out (pooled AUC of the gain, played against related,
  0.24-0.27). By inference, a related version's own material adds peaks
  to the shared passage, as damage does to the played recording; they
  take the first two targets, and the wider fan-out finds the shared
  pairs behind them.
- Damaged true detections under 240 that would reach 240 over 3 windows:
  12, 21 and 23 of 507; related recordings on the shared panels 233, 384
  and 401 of 4,428; unrelated candidates and related wrong candidates in
  the recall runs none. With the played recording left out, a related
  file would be confident in 124-129 of 419 shared searches instead of
  114.
- CPU (an estimate, not timed): the second pass's lookups grow 1.5, 2.4
  and 3.6 times; the second pass is at most 3-4% of a scan's CPU (0205),
  so at most about 2%, 6% and 11% more.
- The registered criterion for a later experiment on the rules is not
  met: (a) fails on its 10% clause, (b) fails, (c) holds.

### What it means for the roadmap

The external review's question is answered: displaced targets exist,
mostly one to three ranks past the fan-out, but under damage they are
a small part of the loss, and a wider query fan-out raises related
versions at least as much as the played recording. Neither a tolerant
count (session 30) nor a wider fan-out separates versions. The loss
under damage is peaks: for step 3 (denser query peaks) the question is
whether the reference peaks the query misses are still present as weaker
maxima in the damaged audio, or gone. Step 3 would most likely raise
related versions too, as the pair counts here suggest, since shared
material holds the same peaks; it needs the same three-way measurement
(played, related, unrelated).

### Decided this session, open to change

- The split and the fan-out counts are part of `--verify`'s pair count,
  with no flag of their own, as session 30's N; the fan-out counts were
  made in the same runs as the split and opened only after 0207 was
  written.
- The registered share was reached through its second clause (displaced
  pairs at ranks 3-8 against E), not through the share of the lost
  pairs.
- 0055's scenario file is on the Mac only, so session 30's R5 was not run;
  at 262 tracks `robust --only combined` stood in (T(0), W(0): its
  combined queries only, since the run also holds the control). T(0)
  agrees with the NAS-scale damaged set: endpoint gone 0.875, displaced
  0.049, changed value 0.066 of the lost pairs.
- The shared panels' roles come from the full panel files.

### Questions for us

- Whether step 3 (denser query peaks) is still worth a session, given
  that both tolerant hashing and a wider fan-out raise related versions
  as much as the played recording. A first measure would be cheap: for
  the reference peaks the verifier misses under damage, whether a local
  maximum exists in the query at a lower threshold.
- Nothing needs listening.

### Handoff for the next session

1. Builds: `work/bin/s44-start/` (= `s43-final`), `s44-harness/` (the
   harness at `816fd57`, which ran every count) and `s44-final/` (the
   same binaries).
2. Scripts (`sh`, from the repository root): `work/s44/runs.sh` (the
   NAS-scale runs), `check.sh` (the counts check and regressions),
   `analyse.sh <out.json> [--fan-outs]` (the analysis), `final.sh`.
   Reports: `work/s44/w/reports/library-nas-dnb-peaks/` (`verified/` for
   recall), `work/s44/w0/reports/verified/` (262 tracks). `work/s44/w/`
   holds hard links to the saved index and to session 33's renders; it
   can be deleted.
3. `lost_pairs.py` uses `quantisation.py`'s set builders; the shared
   panels' roles need the full panel files.
4. R3 took 27 minutes and R4 20, mostly reading NAS store records for the
   verifier (iowait near 20%, about 5 cores busy).

### What else to know

- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. No mix was searched; the runs
  searched recall excerpts, the shared panels' queries and `robust`'s
  excerpts. Nothing ran on the candidate mixes of 0065 or on the NAS
  root's broadcast file.
- The NAS was only read (the second shared panel's audio for its
  renders); the NAS store only read (tags for `labels.py`, records for
  the verifier). No `index` or `prune`; the clusters file in use is
  unchanged (SHA-256 `d9c85c4586ad...`).
- `docs/pair-verdicts.txt` is unchanged (SHA-256 in
  `work/s44/listening-digests-start.txt`); no answer log; no manifest or
  committed panel changed.
- A read-only review by a separate agent (11:22-11:29) found the code
  correct (the split's rules, the found peaks, E and N unchanged, the
  fan-out matching) and recomputed the experiments' numbers from the
  data files. It found T(0) and W(0) mixing `robust`'s control and
  combined queries, a sentence in 0207 that its data contradicted,
  expectations not all marked wrong, the CPU estimate rounded down, an
  unmarked inference, the AUCs' pooling range not stated, two analysis
  choices not recorded, and tests weak at ranks 5 and 9 and for near
  hashes at an extra fan-out. All fixed (brief 44, 11:40).
- Final checks (11:31-11:40) with `work/bin/s44-final/`, built from the
  tree after the review's fixes: `scripts/check.sh` green (3:53, 527 Rust
  tests and the page's 5 Node tests); `regress session-32-linux` (1:09)
  and `regress session-33-linux-nas` (4:06) identical, every sweep
  query, scan detection and calibrate line, and each report identical
  to its baseline's apart from `wall_seconds`; the verdicts file and the
  clusters file unchanged. The same checks had passed with the build
  before the review's test changes (11:21-11:30).

## Session 45

### What changed

Nothing in Gunfinger changed: no threshold, rule, default or output; the
final build is byte-identical to the start build. Added, harness
only: `variant_compare.py answers` also compares a `mixes` report's wrong
confident detections by mix and file, `shared` lists each confident
detection added or removed with its speed gap, and a new `tracklists`
command compares two `tracklist_score.py score` outputs. This session
ran the confirmation the pitch-rides decision note asked for, and
updated that note.

In short: `--apart-in-speed 0.006` passes S2 and S3 of brief 38, and a
tracklist criterion T, on data its threshold had not seen. On the second
shared panel's development half at NAS scale no confident detection
changes in any of 440 query and index pairs; on session 39's double
drops nothing confident changes in the harness at 262 tracks, and the
CLI's reports and printed tracklists are byte-identical to the default's
at 262 tracks and at NAS scale. On three beatmatched seeds nothing
confident changes either. What the variant does change is weak or
possible, and on the drops it includes weak second alignments of a
related version 0.61-1.51% apart without any ride: against shared
material the variant relies on the 240-hit rule's margin, not on its
speed condition. It is a confirmation of no harm: none of this data
holds a ride, so recovery (S1) rests on session 38's seed 3801. Adoption
is ours, with an ADR; the decision note's "Session 45" holds what an ADR
would add.

### Step 1: the pre-registration (brief 21:20, `3bb80ef`)

S2 and S3 quoted from brief 38; runs P (the second panel's development
half, NAS scale), D1 (drops, harness, 262 tracks), D2 and D3 (the CLI's
reports and tracklists at 262 tracks and NAS scale); T: no confident
extra entry added and no truth play printed by default left unprinted.
Confirmed when S2, S3 and T hold on every run made; any violation
fails it. The validation half stays unseen; nothing is tuned.

### Run P: the second shared panel (experiment 0209)

10 scenarios, 169 queries, 440 query and index pairs, on session 44's
renders (no NAS audio read). Confident played 545, related 12, other 3,
partner 0, the same in both runs; no `other` segment added, no played
detection lost. The three `other` ones are Phantom 2018 in
phantom-force-5's 50 s "before" query, in both. Beside: 83 weak or
possible detections added (at most 61 hits, 0.60-12.1% from the stronger
detection of their file, 41 under 0.9%), 2 weak removed. User CPU 5,140
s and 5,145 s; the default's detections equal session 44's run of the
panel.

### Runs D1-D3: double drops and their tracklists (experiment 0210)

- D1 (harness, 262 tracks, ladder `both` and, for the check, `turntable`):
  right plays 221 -> 221 and 220 -> 220, wrong confident 0 -> 0; 20 weak
  detections added (7-44 hits), 19 of them second alignments of the
  Clockwork pair's file not playing while its related version plays,
  0.61-1.51% from a stronger alignment of the same file.
- D2 and D3: the CLI has no variant, so the tracklists under it come from
  a diagnostic build (master's tree with `Matcher::Fitted`'s
  `second_overlap` set to `SpansAtSpeed(0.006)`, never committed;
  `work/s45/variant-cli.diff`). All 64 reports are byte-identical
  between the builds; T, S2 and S3 hold (entries 222, 223, 230, 230;
  wrong confident plays at NAS scale 2 -> 2, session 39's Animation
  remix). NAS scale used the NAS store's own records (`--store-only`,
  `--exclude-from`), not a listing of the NAS; the default's plays equal
  session 39's in all 64 reports.
- The diagnostic CLI runs the variant: its diff is one line, and on the
  Stakka development mix it keeps Star Trails' 793-877 s side (315 hits),
  which the default drops (the printed line is the same). Its segments at
  262 tracks are all in the turntable harness report with the variant,
  which shows consistency only, since the default's are too.

### Step 5, run B: beatmatched long mixes (experiment 0211)

Registered before its runs (brief 22:30). Session 35's seeds 3511-3513,
rendered again from the seeds (corpus files only), 262 tracks: right
plays 186, 195 and 188 and wrong confident detections 0, unchanged; 21
weak detections added (at most 27 hits), none of a file playing then,
0.60-2.49% from a stronger detection of the same file. The default's
detections equal session 35's reports in all 48 mixes.

### Times on the Linux machine (32 workers)

| Run | Time | Peak memory |
|---|--:|--:|
| `regress session-32-linux` (start) | 1:07 | 0.4 GiB |
| `regress session-33-linux-nas` (start) | 4:12 | 5.4 GiB |
| `shared`, second panel's development half, NAS scale (default; variant) | 18:38; 18:36 | 5.7 GiB |
| `mixes --kind drops --count 16`, cached renders (`both`; `turntable`) | 1:14-1:25; 0:59-1:00 | 1.5 GiB |
| `identify --save-dir` of 16 drop mixes, 262 tracks; NAS store's records | 0:54-0:55; 2:37-2:43 | 0.17; 5.2 GiB |
| `mixes --kind beatmatched --count 16` (rendering and searching; cached) | 3:24-3:30; 2:31-2:39 | 13.4-14.6; 2.6-2.7 GiB |
| The diagnostic build (`cargo build --release`, from scratch) | 0:59 | |

The `shared` runs read the NAS store's records from the local disk and
used 467% CPU.

### Decided this session, open to change

- The tracklists under the variant come from a diagnostic build, not a
  CLI option, so that the product does not change (brief 21:18).
- The registered check of the diagnostic build (its harness against
  master's variant) could not work, because the harness sets
  `second_overlap` itself; it was replaced by a check on the CLI's own
  segments, which shows consistency only, and a positive control on the
  Stakka mix, which with the one-line diff is the evidence (brief
  22:10-22:15, 23:00). D2's result was read before that check ran; the
  check did not change it.
- NAS scale for the drops used the NAS store's own records instead of
  the NAS configuration, which lists the NAS over a slow link tonight;
  it gives the same `library_revision` and the same plays as session 39.
- The drops ran on copies of session 39's renders; the panel on hard
  links to session 44's renders, which `shared` never rewrites.
- The variant's weak detections were compared at first with session 38's
  confident-only speed differences; corrected in the decision note
  (brief 22:31).

### Decisions waiting (ours)

- Whether to adopt `SecondOverlap::SpansAtSpeed(0.006)` through an ADR
  superseding ADR 0008's consequence for overlapping parts. The
  registered confirmation holds; the residual risk is a related
  version's second alignment more than 0.6% apart that reaches 240 hits
  on its own (none in any run of sessions 38 and 45).
- How a play shows its overlapping segments, and whether the 0.6% goes
  into `docs/calibration.md` beside the ladder's step (the decision
  note's questions 1-5).

### Questions for listening (none answered by agents)

- Unchanged from earlier sessions: Star Trails at 13:13-14:37 in the
  Stakka mix and Into Music at 25:57-26:56 in the 2003 mix (rides or
  not), session 39's three questions, session 40's pack.

### Handoff for the next session

1. Reports: the panel's in `work/s45/w/reports/library-nas-dnb-peaks/`
   and its `variant-apart-0.006/`; the drops' in `work/s45/w0/reports/`
   (and `ladder-turntable/`, each with `variant-apart-0.006/`); the CLI's
   reports and printed tracklists in `work/s45/cli-{262,nas}-{default,variant}/seed-*/`,
   their scores `work/s45/scores-*.json` and comparisons
   `work/s45/tracklists-*.txt`, `check-cli-262-*.txt`, `d1-compare.txt`,
   `d1-beside.txt`, `panel-compare.txt`, `panel-beside.txt`;
   the beatmatched reports in `work/s45/w5/reports/` (renders deleted).
2. Builds: `work/bin/s45-start/` (= `s44-final`), the diagnostic
   `work/bin/s45-variant-cli/` (never committed; its source in
   `work/s45/variant-src/`, its diff `work/s45/variant-cli.diff`), and
   `s45-final/`.
3. Scripts (`sh`, from the repository root): `work/s45/start.sh`,
   `variant-build.sh`, `panel.sh`, `drops.sh`, `turntable.sh`,
   `stakka.sh`, `beatmatched.sh`, `final.sh`; analyses
   `work/s45/panel_beside.py`, `drops_beside.py`, `check_cli.py`,
   `same_detections.py`; logs in `work/s45/logs/`.
4. `work/s45/w/` holds hard links to the saved index and to session 44's
   renders of the second panel; `work/s45/w0/mixes/` copies of session
   39's drop renders (950 MB); both can be deleted.
5. Next, if we continue: the ADR (ours); a test on the second panel's
   validation half only if a rule change needs one.

### What else to know

- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. The searched mixes were generated
  mixes and the Stakka development mix (the positive control). Nothing
  ran on the candidate mixes of 0065 or on the NAS root's broadcast
  file; the NAS store holds no record of it.
- The NAS was read only for FFprobe header reads of the first panel's
  pair files, some on the NAS (`mixes --kind drops`); no NAS audio was
  decoded.
  The NAS store and `work/peaks` were only read. No `index` or `prune`;
  the clusters file is unchanged.
- `docs/pair-verdicts.txt` is unchanged (SHA-256 in
  `work/s45/listening-digests-start.txt`); no answer log; no manifest or
  committed panel changed (`work/s45/panels` is a copy of
  `work/s33/panels`).
- A read-only review by a separate agent (22:50-22:59) recomputed every
  criterion count and found none wrong. It found the CLI segment check
  unable to tell the builds apart, 0211's Aphrodite count, speed gaps and
  memory wrong, an unmarked inference, imprecise phrases in the decision
  note, the missing no-harm caveat (no ride in the new data), the
  roadmap without the beatmatched result, stamps set ahead of the clock
  (each corrected), and a truncating `zip` in `variant_compare.py
  tracklists`. All fixed or recorded (brief 23:00).
- Final checks (22:49-22:58) with `work/bin/s45-final/`, byte-identical
  to `s45-start` (no Rust change this session): `scripts/check.sh` green
  (3:48); `regress session-32-linux` (1:09) and `regress
  session-33-linux-nas` (4:07) identical, every sweep query, scan
  detection and calibrate line, and each report identical to its
  baseline's apart from `wall_seconds`; the verdicts file and the
  clusters file unchanged. The later commits change documents and an
  analysis script only.
- Stamps in the brief were set ahead of the clock five times and
  corrected each time; commit times in `git log` are the reference.

## Session 46

### What changed

One exact change to the query analysis is in master (`245212a`,
experiments 0212-0214): the peak picker compares powers instead of
levels in decibels. `spectrogram::for_each_power_frame` gives each
bin's power (`|X|^2 + 1e-20`, the same operations as before), and
`decibels(p) = 10 log10 p` is taken only where a level is needed. Since
`decibels` never decreases as power grows, a bin with no more power
than another has no higher level, the loudest power of a set has its
loudest level, and a power below `floor_power` (the lowest power whose
level reaches the floor) is exactly a level below the floor. So the
four-neighbour test and the neighbourhood test run on powers, and for a
bin that passes them all (about 0.6 a frame) one comparison of its
level with that of the loudest other point settles a tie in decibels;
refinement and magnitudes use the levels of the same powers. The
four-neighbour test is also one loop over slices of the same length,
then the passing bins are gathered. About 4 logarithms a frame are
taken instead of 475-557 (the commit message's "513-557" has the
wrong lower end: turntable FFTs of 948 samples have 475 bins). The peaks are the same to the bit:
`decibels_never_decrease_as_power_grows` checks every non-negative
`f32` on the machine running the tests (monotonicity belongs to the
platform's maths library, so it runs on every platform; 4 s in a debug
build), a test makes two different powers round to the same decibels
as a nearest neighbour and farther away, and the tests against the
definition on levels pass. Lines, detections, evidence, boundaries,
speeds and orderings are identical in every report compared, and
`index` stores the same peaks (four corpus files checked).

In short: on the Linux machine the Stakka scan's search at NAS scale
takes 24.1-24.3 s instead of 26.1-26.2 s at 32 workers (7.2-7.5% less)
and 29.1 s instead of 31.8 s at 16 (8.6-8.7%), with 9.0-9.2% less CPU
at 32 workers; `identify` of the Stakka mix through the CLI at 262
tracks with both ladders 6.5-6.7 s instead of 8.5 s (34% less CPU);
`regress session-32-linux` 0:52 instead of 1:08 (single runs). The clusters sample
searches stored peaks and does not analyse audio: 1:19-1:20 either way.

### Step 1: where the analysis's time goes (experiment 0212)

Timers in a diagnostic build (`work/s46/diag.diff`): at 32 workers the
first pass's analysis split FFT 37%, log power 30%, the four-neighbour
test 19%, the neighbourhood test 12% (timers included), windowing 2%,
the row copy under 1%, plans 0.05%. The FFT was 44% of the turntable
analysis (0109 measured 75% on one thread, with `identify` and older
code; the thread count barely moves it here: 45% at 16 workers). A frame has 17.6 candidates and 0.61 peaks.
Repeated analysis is small: block edges about 1% of frames, the
neighbourhoods' edges 0.3%, and key-locked rungs already reuse their
hop's analysis (609 made, 580 reused).

The `x86-64-v2` build (SSE4.1, `roundss`/`roundsd`) gave byte-identical
reports and was not faster: 0.8-1.9% slower at 32 workers, 1.6% slower
and 0.2% faster at 16. By inference (v2 changes more than rounding,
so it bounds only the net effect), the rounding calls left since
session 41 removed the pre-filter's `floor` (about two `round` calls a
frame in the analysis, a few in hashing) cost nothing measurable. For a
platform decision (ADR 0020): a v2 release target would buy no speed
for the search as it is.

### Steps 2 and 3: the candidates (0213)

Pre-registered (brief 46, 23:30 `59452fd`; addenda 23:31 `2aeb90c` and
23:41 `f743ce0`), each before it was timed (rounds started 23:30:16,
23:39:57 and 23:46:07); the stop point a cut of 3%
at 32 workers against the start build in every pair.

| Candidate | 32 workers | 16 workers |
|---|--:|--:|
| C1 `round` without the library call | -1.1%, -0.2% | 0.0%, 0.1% |
| C2 the peak test on powers, a `max` fold | -0.9%, -0.5% | 2.4%, 1.9% |
| C2b C2 with the old fold of comparisons | 2.6%, 2.1% | 3.8%, 3.9% |
| C2c decibels only for bins that pass on powers | 2.4-3.2% | 4.4-5.1% |
| C3 the four-neighbour test over equal slices | 1.7%, 2.5% | 2.9%, 3.0% |
| C4 C2c with C3 | 7.2-7.8% | 8.4-8.7% |

C2 saved the logarithms but its `max` fold over every neighbourhood
cost more at 32 workers (C2b against C2); C2c then took levels only for
the bins that pass every test on powers. Neither C2c nor C3 passed
alone; together (C4) they cut more than the sum of their cuts, by
inference because after C2c the four-neighbour loop is most of what is
left of the picking.

### Steps 4 and 5: adoption and its times (0214)

Validation of `s46-adopt` (C4 in master's tree): both regressions
identical (0:52; 4:02), each report too apart from `wall_seconds`; the
11 development scans at NAS scale and the 11 at corpus scale (262
tracks) and the clusters sample byte-identical to the start build's;
the stored peaks of four corpus library files indexed into scratch
stores byte-identical between the builds and to those files' records
in `work/peaks` (read only). Clippy then asked for two rewrites (the
last level comparison through `partial_cmp`, the same predicate; a
closure in a test), so the final build ran the same validation again
(below).

| Run (Linux machine, 3600 MT/s, 32 workers unless stated) | Start (`s46-start` = `s45-final`) | Now (`s46-adopt`; `245212a` adds two clippy rewrites) |
|---|--:|--:|
| Stakka scan at NAS scale, search | 26.20, 26.08 s | 24.32, 24.12 s |
| The same, user CPU | 724, 729 s | 659, 662 s |
| The same, 16 workers | 31.83, 31.84 s | 29.10, 29.06 s |
| `identify` of the Stakka mix (corpus configuration, both ladders) | 8.47, 8.50 s | 6.50, 6.66 s |
| `identify`'s user CPU | 179, 179 s | 119, 118 s |
| Clusters sample of 48 | 1:20.4, 1:19.8 | 1:18.6, 1:19.4 |
| `regress session-32-linux` (single runs) | 1:08 | 0:52 |
| `regress session-33-linux-nas` (single runs) | 4:21 | 4:02 |

### Where the search's CPU goes now (0214)

With session 41's stage timers on the adopted code (exploratory, 16 and
32 workers), the first pass goes to grouping a part by asset 46% and
40%, the pre-filter 18% and 21%, query analysis 13% and 15% (54 s
instead of 91 s at 16 workers, 99 s instead of 167 s at 32), taking
parts 12% and 13%, aligning, sorting and lines 10.5%. Within the
analysis the FFT is now the largest part, by inference from 0212's
split with the logarithms and the four-neighbour loop made cheaper (not
measured again).

### Decided this session, open to change

- Rounds 1-3 form one experiment (0213) under one pre-registration and
  two addenda, each addendum committed before its candidates were
  timed; 0213 was written after round 3 and before any adoption run
  finished.
- C2b was registered while round 1 ran, C2c and C4 after its results;
  each before it was timed. The rule for the combination in the first
  registration (only candidates that pass) would have timed no
  combination, so C4 was registered as a candidate of its own.
- The adopted code changed after its validation (two clippy rewrites,
  one a different form of the same comparison), so the validation ran
  again on the final build instead of resting on the earlier one.
- `diag-32` ran beside a compile; it was repeated on the idle machine
  (`diag-32b`), which 0212 uses.
- Monotonicity of `decibels` is a property of the platform's maths
  library (`log10f`); the test checks it over every non-negative `f32`
  wherever the tests run, so the Mac's library is checked when its tests
  run. It has not been run on the Mac yet.

### Handoff for the next session

1. Master's analysis is 0213's C4. Builds: `work/bin/s46-start/` (=
   `s45-final`), `s46-adopt/` (validated), `s46-final/` (master after
   the clippy rewrites, validated again), the candidates `s46-c1`,
   `-c2`, `-c2b`, `-c2c`, `-c3`, `-c4`, `s46-final2/` (the final
   tree, used for the final checks), the diagnostics `s46-diag`
   (`work/s46/diag.diff`, `diag.rs`), `s46-timers` (`timers.diff`) and
   `s46-v2`. Candidate diffs `work/s46/c*.diff` (the scratch worktrees
   they were made in were removed at the end).
2. Scripts (`sh`, from the repository root): `work/s46/ab.sh`,
   `ab.py`, `validate.sh`, `devscans.sh`, `corpusscans.sh`,
   `timing.sh` (`identify` with the corpus configuration and
   `--playback both`), `indexcheck.sh` (stored peaks in scratch
   stores), `diag.sh`, `diag.py`, `timers.sh`. References:
   `work/s46/dev-start/`, `corpus-start/`.
3. What is left: grouping's scatter (session 43's handoff) is now
   about three times the analysis. Within the analysis the FFT is the
   largest part and stays fixed unless the FFT lengths change (0109's
   retrieval study, our decision); windowing and the row copy were
   2.3% of it in 0212, before the change. The neighbourhood test on powers keeps its old fold;
   whether it, or the gathering of candidates, can be cheaper was not
   measured after C4.
4. The Mac has not run the monotonicity test or been timed; its
   `log10f` is Apple's, so `scripts/check.sh` there is the first check.
   A profile with `min_bin > max_bin` (none exists) now panics when
   slicing in `pick`, where the old loop found no peaks; not changed.
5. All times are on the Linux machine at 3600 MT/s.

### What else to know

- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. The development sets were named
  one by one. Nothing ran on the candidate mixes of 0065 or on the NAS
  root's broadcast file.
- NAS-scale runs read the NAS store's records and the saved index, both
  local; no command named a path on the NAS, and `identify` used the
  corpus configuration (the slow link tonight). No `index` or `prune`
  with the NAS configuration; `index` ran only on
  copies of four corpus files into scratch stores under `work/s46/`.
  The clusters file in use is unchanged (SHA-256 `d9c85c4586ad...`).
- `docs/pair-verdicts.txt` is unchanged (SHA-256 in
  `work/s46/listening-digests-start.txt`); no answer log; no manifest or
  committed panel changed. `work/s46/panels` is a copy of
  `work/s33/panels`.
- A read-only review by a separate agent (00:34-00:42) found no input,
  profile or edge where the new picker gives other peaks than the old
  (NaN, infinities, silence rows, missing frames, the floor's
  bisection, narrow neighbourhoods, the overlapping ranges of the
  monotonicity test). It found 21 points: a field comment covering two
  fields; the four-neighbour terms of the final comparison untested
  (now a case with a neighbourhood one bin wide, which fails when they
  are removed); magnitudes not compared with the definition (now
  asserted to the bit); the lower end of "513-557"; the
  pre-registration's stamps, written from a misread clock (23:30,
  23:31 and 23:41 by the commits, each before its round started); C2b's
  timing in "Decided"; an unsupported cause for 0109's FFT share; the
  signs of the `x86-64-v2` cuts at 16 workers and inferences not marked;
  wording in the low-bandwidth note; the adoption table's build; 0213's
  sum (4.2-5.7%); 0212's plan shares; block edges as an estimate;
  regress times as single runs and in 0214's data; 0214's inference and
  the stored-peaks check; 0205's ranges; the handoff's windowing share;
  "four files checked". All fixed (brief 46, 00:44); the invalid-profile
  panic it noted is listed in the handoff. The fixes change only
  comments and tests, so the final build was made again.
- Final checks (00:44-00:53) with `work/bin/s46-final2/`, built from the
  tree after the review's fixes: `scripts/check.sh` green (3:46);
  `regress session-32-linux` (0:51) and `regress session-33-linux-nas`
  (3:47) identical, every sweep query, scan detection and calibrate
  line, and each report identical to its baseline's apart from
  `wall_seconds`; `docs/pair-verdicts.txt` and the clusters file
  unchanged. `s46-final` (before the review's comment and test fixes)
  had passed the full validation (00:29-00:42): both regressions, the
  11 development scans at both scales and the clusters sample.

## Session 47

### What changed

The listening page can play an aligned lane without following every
alignment. Gunfinger itself did not change: the CLI binary is
byte-identical to the start build's, and both regressions are
identical.

- **`O` (or the button after the transport line)** goes through up to
  three placements of the aligned lanes, and back:
  - *placed by every alignment* (the default, as before): where the
    main alignment ends, the strongest other alignment covering the
    next stretch takes over;
  - *placed by the main alignment throughout*: one offset and speed,
    those of the alignment with the most hits, over the whole
    timeline, so two rips of one recording play the same moment end to
    end;
  - *placed from the files' starts*: the lane's file from its start at
    the main alignment's speed, with no offset.

  Every question opens with the first, as it opens with speed matching
  on; the button names the placement in force, a re-placed lane's label
  says "main alignment throughout" or "from its start", and its info
  line shows the playing piece's offset, speed and label. Lane A (the
  reference) never moves, so the playhead's time is A's in every
  placement. Markers and shading still show the alignments. An
  answer's record is the same in every placement. On a question with
  one placement (folders, detections, a pair with no alignment) the
  button is hidden and `O` says so.
- **The question format** gained two optional lane fields, `main` and
  `from_start` (lists of pieces under the rules of `pieces`;
  `docs/audition-questions.md`, "Other placements"). The producers
  compute them (`placement::aligned_lane`, `throughout`, `from_start`,
  `main`), so the page still only evaluates `offset + speed × t` and
  picks a list. `questions pack` writes them for lane B and the plan's
  extra lanes, `questions shared` for lane B. Files without them load
  as before; files with them do not load in older builds.
  `schema::CONTROL_KEYS` gained `o` (no producer or loaded file uses it
  for an answer).
- **`listening_pack.py render`** resolves `--corpus` and `--nas` to
  absolute paths, so `questions pack` accepts the sheets of a render
  run with `--corpus corpus/library` (it refused them on 2026-10-22 and
  2026-10-25).

### Why two rips were placed minutes apart (brief 47, 02:04)

In session 40's pack (`work/audition/questions/05-pack-s40.json`) lane B
has a piece more than 10 s from the main alignment's offset in 5 of 14
questions: Dead Air (alignment 2 at -235.2 s against +0.4 s, A
5:20-6:27; answered `same`), Alien Girl (alignments 4-7 at -175.5 to
-340.7 s against -0.2 s, A 4:50-7:02; `same`), Fallout (-11.9 s at A
2:10-2:30 and -300.3 s at A 6:30-6:52; no answer yet), Tower Bass
(-59.6 s against -81.8 s, A 0:59-1:30; `different`) and the Klute VIP
(-132.4 s and -21.9 s against +0.4 s, A 5:10-7:28; `different`). None is
90 s away. `placement::tile` lets each moment follow the strongest
alignment covering it; in the two `same` pairs the main alignment ends
although B under it still has audio to A's end, and an alignment of the
outro against a section of B that repeats it (361 and 155-233 hits, above
the 60 of a possible play) takes over. Why the `pair` search's main
alignment ends there was not settled; by inference the repeated
material matches at both offsets and the repeat's line is the denser in
those windows.

### Checked by hand

Headless Chrome over the DevTools protocol (`work/s47/cdp/check.mjs`,
uncommitted; Node's own WebSocket) against a scratch server on the
question file made again, with `--answers` and `--verdicts` on copies in
`work/s47/scratch/`. Dead Air, lane 2, playing from A 5:38: in each
placement the page's file time for B equalled the one computed from the
question's pieces (91.6 s under alignment 2, 330.0 s under the main
alignment, 332.4 s from the start), the sources played at the piece's
speed (0.9601, 0.9594; 1 with `M` off, when the playhead moved 3.13 s in
3 s, B's own clock), and the windows fetched for B were those of the
new file times (11 and 12 after the first `O`). Brief 47's table has
the numbers; `work/s47/cdp/check.json` and two screenshots the rest.
No answer was sent.

### Decided this session, open to change

- The placement applies per question and resets when a question opens
  (conservative: no placement carries into the next pair unnoticed).
  For the session instead, keep `this.placement` across
  `Player.open`.
- The main alignment is the one with the most hits, the first of
  equals; the pack's sheets list alignments by hits, so it is always
  alignment 1 there.
- "From the start" means file time 0 at the timeline's start; for a
  pair that is A's start.
- `questions detections` and `folders` write no other placements.
- The user's `work/audition/questions/05-pack-s40.json` is left alone;
  the file made again is `work/s47/questions/05-pack-s40.json`, the
  same apart from the two fields and `made_by`'s `--out`.

### Questions for listening (none answered by agents)

- Session 40's pack items 4 (Known Unknown's Rollers Edit), 5
  (Fallout), 7 (Konflict - Pendulum ~ the Paragliders remix) and 9
  (Bacteria ~ Space Cake); session 39's three questions, Star Trails,
  Into Music (unchanged).
- Worth hearing with `O`: Dead Air and Alien Girl, answered `same` while
  lane B jumped, to confirm the verdicts under the main alignment
  throughout (an answer again with the same verdict is logged without a
  second verdict line; another answer is refused while the verdict
  stands).

### Handoff for the next session

1. The next listening, with the pending items, from the repository
   (stop a running `audition` on the same answer log first; a second
   one says so and exits):

   ```sh
   work/bin/s47-final/gunfinger-eval audition work/s47/questions/05-pack-s40.json
   ```

   It loads session 40's pack only and opens item 4, the first without
   an answer. To have the new file in the default queue instead, copy
   it over the old one (the ids are the same, so the answers stay
   attached), or make it again in place:

   ```sh
   work/bin/s47-final/gunfinger-eval questions pack --pack work/listening-s40 --pair-report work/s40/pack/pairs-report.json --second-library-root /mnt/atlas/Music/dnb --out work/audition/questions/05-pack-s40.json
   ```

   `01-pack.json` (2026-10-22) can be made again the same way from its
   pack, if wanted; without it, it plays with one placement.
2. Builds: `work/bin/s47-start/` (`dc4f7dc`), `s47-run/` (the change,
   used for the check by hand), `s47-final/` (master). Scripts:
   `work/s47/start.sh`, `final.sh`, `run.sh`, `same.py`,
   `compare-questions.py`; the check `work/s47/cdp/check.mjs`
   (`node check.mjs URL DEVTOOLS_PORT OUTDIR`, Chrome started with
   `--headless=new --remote-debugging-port=PORT
   --autoplay-policy=no-user-gesture-required`).
3. If the cause in the pair search matters later (a measure of where
   two recordings stop lining up, session 40's 0186-0187): the main
   alignment's end in Dead Air (A 5:19.9) and Alien Girl (A 4:49.8)
   while both files go on.

### What else to know

- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. No mix was searched; the only
  searches were those of `regress`. Nothing ran on the candidate mixes
  of 0065 or on the NAS root's broadcast file. The NAS was only read
  (the page's windows of the Dead Air pair's two files, decoded by the
  scratch server); the NAS store and `work/peaks` only read.
- `docs/pair-verdicts.txt`, `work/audition/answers.jsonl` and the
  clusters file are at their start digests
  (`work/s47/listening-digests-start.txt`). The scratch server answered
  into copies in `work/s47/scratch/`, and no answer was sent: both
  copies are unchanged too. No manifest or committed panel changed. The
  user's question files in `work/audition/questions/` are untouched.
- A read-only review by a separate agent (02:18-02:25) found no defect
  in the page or the Rust code and three in the documents (README's key
  table without `O`; item 11's far-piece span; the pair example's pieces
  outside its clipped file), and suggested tests; all taken (brief 47,
  02:25, `2c40547`).
- The brief's time stamps from 02:07 on were first written ahead of
  the clock and corrected from the files' and commits' times (brief 47,
  02:26).
- Final checks (02:25-02:34) with `work/bin/s47-final/` (built from
  `2c40547`; its `gunfinger` byte-identical to the start build's):
  `scripts/check.sh` green (3:47; Node 14 tests, the Python tests
  included); `regress session-32-linux` (0:50) and `regress
  session-33-linux-nas` (3:45) identical, every report identical to its
  baseline apart from `wall_seconds`; `work/s47/questions/05-pack-s40.json`
  made again with the final build, the same as the user's file apart
  from the two fields and `made_by`.

## Session 48

### What changed

The product did not change: no threshold, rule, default or output, and
the final CLI binary is byte-identical to the start build's. Added,
harness only and tested:

- `mixes --kind drops --aligned-drops`: in both unrelated double drops
  of a mix, each record's strongest drop it can play then (the largest
  rise of loudness over 8 s, moved to the beat whose next 16 beats hold
  the most onsets; `tempo::Structure`) is placed at one mix time, on the
  beat (`drops::line_up_drops`). Records, overlays, speeds and the other
  blocks are those of the tempo-only mixes of the same seed; plays carry
  `drop_seconds`, left out of the JSON when unset; report and renders
  labelled `aligned-drops`.
- `scripts/analysis/marked_features.py` (`pairs`, `features`,
  `summary`, `known`, `merges`) with `test_marked_features.py`.

In short: whether the marked play's file and its partner's share any
alignment a search would call possible (60 hits or more, by `pair` on
stored peaks) tells a record playing under an unrelated partner from
shared material completely, on fresh seeds at both scales and in the
development mixes' reports. It does not tell a record inside its
relative from shared material; position and evidence outside the shared
spans do that better but let shared material in. Lining up drops merges
few entries.

### Step 1: the pre-registration (brief 48, 02:43, `d7b17e9`)

Features F1 (the pair's strongest alignment), F2 (where the marked
file time maps through the nearest alignment against the partner's
file time), F3 (hits outside the shared spans), F4 (the peak verifier's
share), F5 (the speed ratio against the alignment's); rules R1 F1 < 60,
R2 F2 > 10 s or none, R3 F3 >= 60 fixed in advance, F4 and F5's bars
from the development seeds (brief 03:05: R4 F4 < 0.172, R5 F5 >
0.00344); classes playing, shared and other as 0183; development seeds
2026 and 2028 (session 39's reports), test seeds the first two from
2030 up keeping both Clockwork records (2032, 2034). Worth an ADR
proposal: at least 80% of the unrelated playing records printed with no
shared play let in, at every scale, and none of the development mixes'
five marked plays let in.

### Experiments 0215-0218

| Rule | Unrelated playing printed (dev; test), 262 \| NAS | Shared let in (dev; test) | Development mixes' 5 |
|---|--|--|--|
| R1 F1 < 60 | 35/35, 110/110; 25/25, 101/101 | 0/64, 0/102; 0/69, 0/117 | 0 |
| R2 F2 > 10 s or none | all | 3, 6; 7, 21 | 2 |
| R3 F3 >= 60 | all | 10, 12; 15, 22 | 1 |
| R5 F5 > 0.00344 (bar from dev) | test 20/25, 86/101 | test 2, 0 (dev 1, 0) | 0 |

- R1 prints 0-3 of 6-19 pair drops (a record inside its relative; the
  exception is the Monotone remix inside Watermelon, whose files share
  13 hits by `pair`), and 4 of 29-31 "other" plays at NAS scale
  (possible plays matching no known relation, among them NIL -
  Infectious over Mystery Machine).
- R5 is "promising, more data first" by the registered criterion (at
  least 80% printed, at most 2 shared let in per scale). Its bar and
  R4's were rounded the wrong way (0.00344 and 0.172 against the exact
  0.0034421 and 0.171746), so each lets in 1 development shared play at
  262 tracks; with the exact bars the test results are the same.
- F4 reads the wrong way: shared material shares more of the marked
  file's peaks than a record under a partner does.
- 0217: aligned, 127 of 128 drops lined up; 3 of 128 one entry at 262
  tracks, 4 at NAS scale (tempo-only 1 and 0); 6 of the 7 merged
  entries flag `titles disagree:`. One NAS merge is a chain: a Hord 39
  upload with a 12 s longer lead-in, tagged as the other rips, lines up
  with Synergy.

### Times on the Linux machine (32 workers)

| Run | Time | Peak memory |
|---|--:|--:|
| `regress session-32-linux` (start; step 2) | 0:51; 0:50 | 0.4 GiB |
| `regress session-33-linux-nas` (start; step 2) | 3:45; 3:45 | 5.3 GiB |
| `mixes --kind drops --count 16`, rendering and searching (`--verify`) | 1:04-2:02 | 6.8-7.2 GiB |
| `mixes --verify` on cached renders, 262 tracks | 1:13-1:15 | 1.5 GiB |
| `identify --save-dir`, 16 mixes, 262 tracks; NAS store | 0:42-1:03; 2:19-2:24 | 0.17; 5.1-5.3 GiB |
| `pair`, 34-37 corpus pairs (audio searched too); 110-117 NAS pairs (peaks only) | 0:26-0:55; 0:02-0:03 | 1.0; 0.7 GiB |
| `scripts/check.sh` | 3:50 | |

### Decided this session, open to change

- The features read `pair`'s "peaks, turntable ladder" search at both
  scales, so 262 tracks and NAS scale use the same evidence.
- The CLI at NAS scale read the NAS store's own records (`--peaks-dir`
  the NAS store, `--store-only --saved-index off --exclude-from`), as
  session 45 did, not a listing of the NAS.
- Aligned drops apply to the unrelated double drops only; each record
  takes its strongest drop within the file times it can be at, and the
  snap can land one beat from the rise's true beat.

### The decision this leaves for us

Whether a marked play (ADR 0019) whose file shares no possible
alignment with its partner's file by the library's own evidence (R1)
stays in the tracklist, as a possible line. On these data it would print
every left-out record under an unrelated partner (brief and quiet ones
included) and no shared material, and leave records inside their
relatives out as today. What it would need, if adopted: the pair search
(or a stored map of related files) available to `identify`, which today
sees no `pair` evidence; an ADR superseding ADR 0019's decision 2 for
that case; a check of its cost per marked play. The shared class here
is narrow (Clockwork in the generated mixes, four families in the
development mixes); a related version whose files share under 60 hits
by `pair` (the Monotone remix and Watermelon) would be printed.

### Questions for listening (none answered by agents)

- NIL - Infectious over Ed Rush & Optical - Mystery Machine (seed 2032,
  mix-07, `work/s48/cli-nas/seed-2032/mix-07.json`, plays 9 and 31; the
  two files share 41 hits by `pair`): R1 would print it as possible.
  Also session 39's (Techno Funk over Alien Girl, the same NIL play,
  Crash over the T-Minus vocal mix) and the pending ones (items 4, 5, 7
  and 9 of session 40's pack, Star Trails, Into Music).

### Handoff for the next session

1. Data: features `work/s48/features-{dev,test}-*.json` (copied to
   `docs/experiments/data/0215-*.json`, `0216-*.json`), summaries
   `work/s48/summary-*.txt`, `work/s48/known.txt`; pair reports
   `work/s48/w/reports/pairs-pairs-*.json` and
   `work/s48/w/reports/library-nas-dnb-peaks/pairs-pairs-*.json`;
   harness reports `work/s48/w/reports/ladder-turntable/` (`verified/`
   for the verifier runs); the CLI's reports `work/s48/cli-262/`,
   `cli-nas/`, `cli-nas-other/`, `cli-262-aligned/`, `cli-nas-aligned/`;
   renders `work/s48/w/mixes/` (the development seeds' tempo-only
   renders are links to session 39's in `work/mixes/`); store copies
   `work/s48/store-{2032,2034}/`, exclude lists
   `work/s48/nas-exclude-{2032,2034}.txt`; panels for seeds 2030-2035 in
   `work/s48/panels/`.
2. Builds: `work/bin/s48-start/` (= `s47-final`), `s48-run/` (the
   runs), `s48-final/` (master).
3. Scripts: `work/s48/start.sh`, `step2.sh`, `check2.sh`, `dev.sh`,
   `test.sh`, `aligned.sh`, `aligned-nas.sh`, `final.sh`. For example:

   ```sh
   E="work/bin/s48-final/gunfinger-eval --work work/s48/w --panels work/s48/panels"
   $E --ladder turntable mixes --kind drops --aligned-drops --seed 2032 --count 16 --other-root /mnt/atlas/Music/dnb --tagged work/s39/tags-262.json
   python3 -I scripts/analysis/marked_features.py pairs work/s48/pairs-test-262.txt work/s48/cli-262/seed-2032
   $E pair work/s48/pairs-test-262.txt
   python3 -I scripts/analysis/marked_features.py summary work/s48/features-test-262-2032.json work/s48/features-test-262-2034.json
   ```

4. Next, if we continue: an ADR proposal for R1 (the decision above),
   with a test on related versions beyond Clockwork (the second shared
   panel's families played inside each other); drops of the same rank
   for the merge measure.

### What else to know

- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. No development mix was searched;
  session 36's saved reports of the three development mixes were read
  (0218). Nothing ran on the candidate mixes of 0065 or on the NAS
  root's broadcast file. The NAS store was only read (`identify
  --store-only --saved-index off`, `pair` and `--other-peaks-dir`); the
  NAS library was not listed; `work/peaks` was only read (copied).
- `docs/pair-verdicts.txt`, the clusters file and the answer log are at
  their start digests (`work/s48/listening-digests-start.txt`); no
  answer log was written; no manifest or committed panel changed (the
  new seeds' panels are in `work/s48/panels/`).
- The renders of this session (the test seeds' tempo-only and
  other-pairs mixes, all aligned mixes) are kept in `work/s48/w/mixes/`
  (3.8 GB) for the handoff; delete them when no longer needed.
- A read-only review by a separate agent (03:37-03:44) found no defect
  in the drop line-up or the features' definitions, and six in the
  numbers, the rules' bars and the script; all fixed (brief 48, 03:52,
  `f285532`).
- Final checks (03:42-03:50) with `work/bin/s48-final/` (built from
  `f285532`; its `gunfinger` byte-identical to the start build's):
  `scripts/check.sh` green (3:40); `regress session-32-linux` (0:50) and
  `regress session-33-linux-nas` (3:44) identical, every report
  identical to its baseline apart from `wall_seconds`; the digests
  unchanged.

## Session 49

### What changed

The product did not change: no threshold, rule, default or output, and
the CLI binary is byte-identical to the start build's. Added, harness
only and tested:

- `pair --fine-rungs` (`pair::FineRungs`): beside its four searches of
  stored peaks, `pair` searches 20 more rungs, the fitted speed times
  1 + k × 0.1% for k = ±1 to ±10 (`peaks, fine rung -1.0%` to `+1.0%`),
  each alignment with its supported seconds. Without the option the
  report is unchanged (session 40's source pairs give an identical
  report), and with it the other searches are unchanged (the populations'
  reports less the fine rungs equal session 40's).
- `scripts/analysis/drift_offsets.py` (`measures`, `choose`, `scores`)
  and `cluster_speeds.py`, each with tests.

In short: the Falcon - The Stand pair the listening judged `same` parts
over time because one upload's speed drifts, not because of trailing
silence, which moves no fit or placement. A near-offset share gathered
over rungs near the fitted speed orders the 60 judged pairs it was
chosen on better than any earlier measure but did not carry its
threshold to the ten new verdicts; `near_10` stays the best measure
by the registered scores.

### Experiments 0219-0221

- 0219 (exploratory): A, the "HD Audio & Speed Up" upload, ends with
  4.2 s of silence (5.9 s below -45 dB) and 3 s of music B lacks; B, the
  CITRUS 003 upload, has none. Against alignment 1, B wanders from -0.2 s
  to +0.48 s and back (envelope lags and the fine rungs agree: about
  1.083 at the start, 1.075 at the end). Against the vinyl rip B runs
  1.031 to 1.038 times its speed, while A is about 1.116 throughout:
  B's digitisation drifts. A's first 30 s hold no alignment at the
  fitted speed because the files run 0.5% from it there. File length
  enters only as the shares' denominator and a lane's clip; the page
  plays B at the alignment's speed, not the length ratio.
- 0220: the registered choice `drift_0.8_20` (rungs within 0.8%, lines
  within 20 s) has AUC 0.989 on the 60 judged pairs (a) it was chosen
  on and 3 leave-one-out errors (`near_10` 0.952 and 6, supported time
  0.935 and 7; the best earlier AUC is `near_20`'s 0.955), one presumed
  join below its threshold; on the ten it orders every
  pair right but its threshold (0.843) puts two same pairs (Pyro, Alien
  Girl) below; pooled leave-one-out errors 4 (6, 9). Not better by the
  registered bar. Found after the test: `drift_0.8_10` (4 leave-one-out
  errors, no presumed join below) makes no error on the ten.
- 0221: on the ten, coverage and `near_10` at session 40's thresholds
  make no error; supported time misses Falcon by 0.009.
- Step 5 (roadmap, "The edge of the ladder"): of 284 NAS clusters 75,
  12, 1 and 1 span more than 2, 4, 6 and 8% in speed (Falcon - The Stand,
  11.5%). With a deck at ±5%, 50 clusters hold two copies of which one,
  indexed alone, would miss a play of the other; 38 copies (35 uploads of
  one channel, 3-5% fast) have no other copy in reach at one extreme,
  and each of their clusters holds a rip that has.

### Times on the Linux machine (32 workers)

| Run | Time | Peak memory |
|---|--:|--:|
| `regress session-32-linux` (start; run build) | 0:50; 0:51 | 0.4 GiB |
| `regress session-33-linux-nas` (start; run build) | 3:46; 3:45 | 5.3 GiB |
| `pair --fine-rungs`, populations a-d (62, 100, 51, 60 pairs) | 0:19-0:40 | 0.6-1.1 GiB |
| `pair --fine-rungs`, the ten verdicts | 0:13 | 0.3 GiB |
| `scripts/check.sh` | 3:55 | |

### Decided this session, open to change

- `--speed-per-stretch` was not run on the Falcon pair: it works on
  decoded audio in the second pass, and `pair` decodes only corpus
  queries. The fine rungs on stored peaks, and an onset-envelope lag on
  the decoded audio, gave the per-stretch speeds instead.
- The three verdicts of 2026-10-25 that sat in session 40's presumed
  populations (Dead Air and Pyro in (b), Drumtrip ~ Propaganda in (d))
  were left out of them for the parameter choice.
- Falcon was looked at in step 1 before the registration, so the test
  scores are also given without it (the same errors).

### Questions this leaves for us

- Whether to give the drift-following share a second registered test,
  with `drift_0.8_10` (or the order of the choice changed to put "no
  presumed join below" first) on judged pairs not yet seen; ten pairs
  chosen near `near_10`'s threshold cannot separate the measures, and
  the pooled leave-one-out margin is two pairs.
- Whether a join rule should read evidence from several rungs at all:
  the fine rungs lift versions that share most of a track (the Sonar
  revision pairs) as well as drifting copies.
- Whether the listening page should offer a placement that follows
  drift (the fine rungs' alignments as pieces), so that copies like
  Falcon's do not sound as if they part; today it plays one speed.

### Questions for listening (none answered by agents)

- None new. Pending: items 4, 5, 7 and 9 of session 40's pack
  (`work/s47/questions/05-pack-s40.json`), session 39's questions, Star
  Trails, Into Music.

### Handoff for the next session

1. Data: `pair --fine-rungs` reports
   `work/reports/library-nas-dnb-peaks/pairs-s49-{a,b,c,d,new,falcon,falcon-third,source-default}.json`;
   measures `work/s49/measures-{dev,new}.json`, the choice
   `work/s49/choice.json`, the scores `work/s49/scores.json` (copied to
   `docs/experiments/data/0220-*.json`), the exploratory table
   `work/s49/explore-candidates-test.json`; the cluster spans
   `work/s49/cluster-speeds.json`; Falcon's silence and envelope lags in
   `work/s49/falcon/`.
2. Builds: `work/bin/s49-start/` (= `s48-final`), `s49-run/` (the runs),
   `s49-final/` (master).
3. Scripts: `work/s49/start.sh`, `step3a.sh`, `step3b.sh`, `step4.sh`,
   `final.sh`. For example:

   ```sh
   work/bin/s49-final/gunfinger-eval --other-peaks-dir ~/.local/share/gunfinger/nas-dnb-peaks pair --fine-rungs work/s49/pairs/s49-new.txt
   python3 -I scripts/analysis/drift_offsets.py measures work/s49/measures-new.json work/reports/library-nas-dnb-peaks/pairs-s49-new.json
   python3 -I scripts/analysis/cluster_speeds.py work/reports/library-nas-dnb-peaks/duplicate-clusters.json work/s49/cluster-speeds.json
   ```

### What else to know

- The held-out test set was not touched; no mix was searched; nothing
  ran on the candidate mixes of 0065 or on the NAS root's broadcast
  file. The NAS store was only read (`pair --other-peaks-dir`); two NAS
  files were decoded (read) for step 1; `work/peaks` was only read.
- `docs/pair-verdicts.txt`, the clusters file and the answer log are at
  their start digests (`work/s49/listening-digests-start.txt`); no answer
  log was written; no manifest or committed panel changed.
- A read-only review by a separate agent (04:17-04:25) found no defect
  in `pair --fine-rungs`, the measure's code, the choice's order or the
  commit order, and ten points in the documents and the script; all
  taken: the source pairs' sides added to the scores (both fall on the
  same side under every measure, as in 0186), the AUC of 0220 marked as
  the choice's own data (best earlier `near_20` 0.955), experiment 0001's
  figure cited as measured (0.1% off keeps about 70%), the four Sonar
  revision pairs (0.36-0.66 to 0.56-0.77), 0220's "before the test
  pairs' measures were computed" (`pair` on the ten ran at 04:13, just
  before the choice was committed), 0219's lag rows on a neighbouring
  bar set aside, the brief's time stamps (some were later than their
  commits) and the brief's list wrap.
- Final checks with `work/bin/s49-final/` (built from `a9cc7e1`; its
  `gunfinger` byte-identical to the start build's; its `gunfinger-eval`
  differs from the run build's only by a comment's line numbers):
  `scripts/check.sh` green;
  `regress session-32-linux` (0:50) and `regress session-33-linux-nas`
  (3:45) identical, every report identical to its baseline apart from
  `wall_seconds`; the digests unchanged.

## Session 50

### What changed

The product did not change: no threshold, rule, default or output. The
core gained one opt-in method the CLI never calls
(`Index::skipping_most_held`), and `scanned_postings` now asks a
`SetAside` value (lists longer than a length, as before, or a set of
hashes), so the CLI binary is not byte-identical to the start build's;
both regressions are identical and the NAS-scale development scan
searches in 24.7 s as before. `search::MIN_HITS_TO_REFINE` is public.
Added, harness only and tested:

- `sweep --exact-copies K --copied FILE` (`padding::ExactCopies`):
  exact copies of the listed records, built into the index in memory
  after every other record (`padding-copy/<i>/<path>`), counted as
  their original in scoring; a left-out file has no copies.
- `sweep --trace-candidates`: each query's first-pass candidates (the
  detections of 10 hits or more the second pass measures, from the same
  search with the second pass off, `Matching::first_pass`) and the
  report's `set_aside` (the census of the searched index and its
  set-aside hashes).
- `sweep --rank-fullest-by files|clusters` (`fullest::Ranking`): the
  same 1% of lists, ranked by distinct files or recordings.
- `fullest --shared-by PAIRS` (with `--exact-copies`/`--copied`): the
  distinct hashes two files share and how many are set aside.
- `scripts/analysis/filter_padding.py` (`copied`, `compare`) with tests.

In short: the fullest-list filter is stable under copies. Up to ten
files of every recording the sweep plays, or six copies of a random
tenth of the NAS library, no query stops being a candidate or loses its
confident answer, and no new wrong answer appears. At 262 tracks copies move
a quarter of the lists set aside and cost the copied recordings up to
23% of their first-pass hits; at NAS scale the set aside follows the
library's proportions and moves by 1-2%. Ranking by distinct files or
recordings keeps the first pass within 3% under copies at no measured
cost (by recordings it is unchanged by them), but there is no failure
for it to remove.

### Experiments 0222-0225

| Experiment | Result |
|---|---|
| 0222, 262 tracks | panel (67 files) K = 1, 3, 9, 99 and share (26 files) K = 1, 3, 9: 540/540 confident, no lost candidate, the second pass's own counts 0.995-1.009 of level 0; own first-pass hits at least 0.768 of level 0 at K = 9 (0.717 at 99); the cut 101 → 410 postings (3,620 at 99), 27% of the set changed at panel 9 |
| 0223, NAS scale | panel (157 indexed files) K = 9 and share (4,070 files) K = 3 and 6 (K = 9 exceeds 65,536 assets): 540/540 and 408/423 (other rips) as at level 0; 0.9-2.0% of the lists moved; the share selection raises the cut 60% but leaves copied and uncopied files at 18.7% set aside; Siren Bass (other rips, +3%) 363 → 298 second-pass hits through a shorter first-pass chain, also when only other files are copied |
| 0224, ranking | files or recordings: 14.2-14.4% set aside at 262 tracks, 18.4% at NAS scale; 540/540; first pass within 3% of level 0 under every padding (postings ranking: down to 0.717); recordings unchanged by copies by construction; not worth an ADR (no U1-U3 to remove) |
| 0225, drum-loop pairs | the 7 related pairs share 1,793-3,850 hashes, 23.6-35.1% set aside (median 28.8%), against 42.4% for 20 random pairs; copies move them by at most 0.2 points at NAS scale |

### Times on the Linux machine (32 workers)

| Run | Time | Peak memory |
|---|--:|--:|
| `regress session-32-linux` (start; run build) | 0:51; 0:51 | 0.4 GiB |
| `regress session-33-linux-nas` (start; run build) | 3:45; 3:52 | 5.3; 5.4 GiB |
| `sweep --trace-candidates`, 262 tracks (K = 99) | 0:52-1:06 (2:48-3:13) | 0.24-0.53 (1.4-1.9) GiB |
| the same, NAS scale: level 0 (saved index); panel 9; share 6 | 4:50-5:04; 6:58-7:17; 9:54-10:22 | 4.9; 5.1; 7.6-7.8 GiB |
| the same, other rips: level 0; panel 9; share 6 | 2:52; 4:50; 7:05 | 4.9; 4.9; 7.6 GiB |
| `fullest --shared-by`, NAS level 0; padded | 0:06; 2:01-3:02 | 4.4; 4.5-6.9 GiB |
| `scripts/check.sh` | 4:00 | |

A padded NAS run builds its index from the stores (the saved index
holds no copies); its trace searches the first pass a second time.

### Decided this session, open to change

- Padding and the trace are `sweep` options only: the recall panel's
  other-rips queries and the shared panels were not searched padded;
  the sweep's `--other-rips` form stood for the other-rips condition.
- The share selection at NAS scale used K = 6 and 3 instead of 9 (the
  65,536-asset limit, ADR 0010).
- Wrong candidates and wrong possible answers are counted by distinct
  original file: counted by detection, K copies of a related record give
  K entries.
- Step 5 added a control (20 seeded random pairs) after the first
  readings; exploratory.

### Questions this leaves for us

- Whether to keep ranking the fullest lists by postings. The evidence
  here does not ask for a change: nothing failed. Ranking by distinct
  files is open to the product (a count per list at build or load
  time), costs nothing measured here and makes the first pass
  insensitive to repeats within a file and keeps the first pass within
  3% under copies (only ranking by recordings is unchanged by them);
  it changes about 11% of the lists set aside at NAS scale, so its
  adoption would need the full protocol and an ADR.
- Whether a famous break held by many different records behaves like
  these two-file families; no such family was built or measured.
- Whether the first-pass chain's span should bound the second pass
  (Siren Bass loses 18% of its second-pass hits when its first-pass
  chain loses a window; in the share selection its rip is not copied, so
  lists moved by other files' copies are enough).

### Questions for listening (none answered by agents)

- None new. Pending: items 4, 5, 7 and 9 of session 40's pack
  (`work/s47/questions/05-pack-s40.json`), session 39's questions, Star
  Trails, Into Music.

### Handoff for the next session

1. Data: traced sweep reports `work/s50/w262/reports/` and
   `work/s50/wnas/reports/library-nas-dnb-peaks/`
   (`sweep-seed-2026[-other-rips][-exact-<K>-<list>][-rank-<ranking>]-trace.json`);
   comparisons `work/s50/compare/` (copied to
   `docs/experiments/data/0222-*.json` to `0224-*.json`); the copied
   lists `work/s50/copied/`; the case-study reports
   `work/s50/{w262,wnas,wctl}/reports/**/fullest-0.01*.json` and pairs
   `work/s50/case/`; the level-0 census `work/s50/w0/`.
2. Builds: `work/bin/s50-start/` (= `s49-final`), `s50-run/` (the
   sweeps), `s50-case/` (`fullest --shared-by`), `s50-final/` (master).
3. Scripts: `work/s50/start.sh`, `step2.sh`, `step3-262.sh`,
   `step3-nas.sh`, `step3-nas2.sh`, `step5.sh`, `step5b.sh`,
   `compare.sh`, `table.py`, `answers_min.py`, `final.sh`. For example:

   ```sh
   E="work/bin/s50-final/gunfinger-eval --work work/s50/wnas --other-peaks-dir ~/.local/share/gunfinger/nas-dnb-peaks"
   $E sweep --trace-candidates --exact-copies 9 --copied work/s50/copied/panel-nas.txt
   $E sweep --trace-candidates --rank-fullest-by files
   python3 -I scripts/analysis/filter_padding.py compare BASE.json PADDED.json work/s50/copied/panel-nas.txt
   ```

   A scratch work directory needs the clusters files and the library map
   copied into its `reports/`, and links to `work/sweep/` and the saved
   index.

### What else to know

- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. No mix was searched. Nothing ran
  on the candidate mixes of 0065 or on the NAS root's broadcast file.
  The NAS store was only read (`--other-peaks-dir`); `work/peaks` was
  only read. No renders were made (the sweep's are reused).
- `docs/pair-verdicts.txt`, the clusters file and the answer log are at
  their start digests (`work/s50/listening-digests-start.txt`); no
  answer log was written; no manifest or committed panel changed.
- A read-only review by a separate agent (07:05-07:14) found no defect
  in the code and points in the documents and `filter_padding.py`; all
  but four small items taken (brief 50, 07:20; `13f44db`).
- Final checks with `work/bin/s50-final/` (built from `13f44db`):
  `scripts/check.sh` green (3:44, on an idle machine; a first run at
  07:05 failed only in the load-sensitive `decode::watch`, cause of the
  load unknown);
  `regress session-32-linux` (0:51) and `regress session-33-linux-nas`
  (3:52) identical, every report identical to its baseline apart from
  `wall_seconds`; the digests unchanged.

## Session 51

### What changed

The product did not change: no threshold, rule, default or output, and
the CLI binary is byte-identical to the start build's. Added, harness
only and tested:

- `sweep --speeds edge` (`sweep::Speeds`): the excerpts played at 18
  speeds at and past the ladder's ends (±8.4, ±8.8, ±9.2, ±9.6, ±10.0,
  ±10.4, ±10.8, ±11.2, ±12.0%) instead of the sweep's 9; `-edge` in the
  report's name. Renders of speeds that are not whole percents are named
  to a tenth (`000-+8.4.mp3`); the sweep's and `recall`'s names are
  unchanged.
- `scripts/analysis/ladder_edge.py` (`recall`, `uncovered`) with tests:
  recall and wrong answers by played and relative speed per ladder, new
  wrong answers against the default, the targeted estimate, and the
  share of a scan no confident detection covers.
- `cluster_speeds.py` takes the ladder's reach as a fourth argument
  (default 0.08, output unchanged).

In short: each extra rung pair moves the ladder's reach by 0.4% and
costs 4.3-5.3% more search CPU, at both scales and against the copies
the NAS library holds; calibrate and the development scans do not move.
By the registered criteria no wider ladder is worth an ADR (one rip near
-8% loses its confident answer under every wider ladder; from two extra
rung pairs a known bootleg is reached at one more speed). A targeted
search with one extra rung pair passes, by estimate, as a variant to
build and measure.

### Experiments 0226-0230

| Experiment | Result |
|---|---|
| 0226, recall by relative speed (seed 2026) | reach R_k = 8.4, 8.8, 9.2, 9.6, 10.4% for k = 0, 1, 2, 3, 5 (≥ 95% at both signs, 262 tracks), the same steps at NAS scale and against other copies only; other copies in [8.4%, R_k + 0.4%): 62% → 96% (L1) to 21% → 96% (L5); within 8.4% every L_k loses one query (Siren Bass's upload near -8%, fitted 0.1% off: 305 → 214 hits); no new wrong confident answer; wrong possible answers 24 → 29-47 (other copies) |
| 0227, standard protocol and calibrate | development scans keep their plays; calibrate unchanged at both scales (262: 690, 100, 6.90×; NAS: 704, 282, 2.50×); false candidates +3-11%; L2-L5 add one wrong confident answer at NAS scale (a held-out excerpt against a bootleg judged a different mix, which L0 answers wrongly at 7 other speeds) |
| 0228, CPU (Stakka scan, A B B A) | process CPU at NAS scale ×1.046, ×1.094, ×1.141, ×1.229 (rungs ×1.049-1.244); at 262 tracks ×1.043-1.255; T_k estimated ×1.001-1.005 (2.0% of the mix without a confident detection) |
| 0229, the NAS clusters | copies alone for a deck at ±5%: 38 (session 49, at 8.0%) → 29, 19, 7, 3, 1; for ±8%: 88 → 57 at L5 |
| 0230, seeds 2027 and 2028 (other copies) | gains hold (L1 +35 and +27 points); no loss within 8.4%; seed 2027 adds 1 (L3) and 5 (L5) wrong confident answers, all China Cup against ƆƐI3C's upload (judged different); T1 none |

Registered verdict: no wider ladder passes (G2's within-reach clause
for every L_k; W for L2-L5); C holds for L1-L3 and fails for L5. T1 (the
targeted estimate with one extra rung pair) passes G1, G2, W and its
cost bound. The decision note,
`docs/research/2026-10-25-ladder-edge-decision.md`, recommends keeping
the default and, if the copies at risk matter, building T1 as an opt-in
variant to measure on mixes before any ADR.

### Times on the Linux machine (32 workers)

| Run | Time | Peak memory |
|---|--:|--:|
| `regress session-32-linux` (start; run build) | 0:50; 0:50 | 0.4 GiB |
| `regress session-33-linux-nas` (start; run build) | 3:52; 3:45 | 5.3 GiB |
| `sweep --speeds edge`, 262 tracks, L0-L5 | 0:59-1:10 (1:34 with renders) | 0.2 GiB |
| the same, NAS scale | 4:27-5:34 | 4.9-5.0 GiB |
| `sweep --other-rips`, NAS, 9 speeds; edge speeds (three seeds) | 1:26-2:01; 2:40-3:47 | 4.9-5.1 GiB |
| `regress` with L0-L5: 262 tracks; NAS scale | 0:51-1:00; 3:47-4:34 | 0.4; 5.3-5.5 GiB |
| Stakka scan, L0 to L5: 262 tracks; NAS scale | 7.3-8.3 s; 27.3-32.1 s | |
| `scripts/check.sh` | 3:54 | |

### Decided this session, open to change

- The targeted ladder T_k was estimated from the wide-ladder runs (each
  query keeps the default's answers when it has anything confident),
  not built; its cost from the share of the Stakka mix without a
  confident detection.
- Extra rungs extend both ladders (turntable and key-locked), as
  `--extra-rungs` does and as `identify` searches both.
- L1 was added to seeds 2027 and 2028 after 0226-0229 (exploratory).
- The runs went in four queues so that each experiment was written
  before the next one's runs started.

### Questions this leaves for us

- Whether the copies at risk (29 for a deck at ±5% today, most of them
  one channel's uploads 3-5% fast) are worth a session to build and
  measure the targeted search (T1, or T2/T3 against the registered bar).
- Whether a wrong answer from a version judged different that the
  default already gives at other speeds should count against a wider
  ladder as a new failure (the registration said yes).
- Whether the second pass's fitted speed near the ladder's end should be
  looked at first (Siren Bass, here and in session 50).

### Questions for listening (none answered by agents)

- None new. Pending: items 4, 5, 7 and 9 of session 40's pack
  (`work/s47/questions/05-pack-s40.json`), session 39's questions, Star
  Trails, Into Music.

### Handoff for the next session

1. Data: summaries `work/s51/summary/` (`c1-262.json`, `c1-nas.json`,
   `c2-nas.json`, `c2-nas-2027.json`, `c2-nas-2028.json`, `c4-cpu.json`,
   `c5-clusters.json`, `0227.txt`; copied to
   `docs/experiments/data/0226-*` to `0230-*`); the sweep reports in
   `work/s51/w262e/reports/` and
   `work/s51/wnase/reports/library-nas-dnb-peaks/` (`extra-rungs-N/`
   for each ladder); the standard protocol's reports in `work/s51/w262/`
   and `work/s51/wnas/`; the timing runs' scan reports `work/s51/cpu/`.
2. Renders: `work/s51/renders/seed-2026..2028/` (2.0 GB of edge renders,
   links to the sweep's own renders); kept for a targeted-search test.
3. Builds: `work/bin/s51-start/` (= `s50-final`), `s51-run/` (the runs),
   `s51-final/` (master).
4. Scripts: `work/s51/setup.sh` (scratch directories), `start.sh`,
   `step2.sh`, `step3a.sh`-`step3d.sh`, `summary-0226.sh`,
   `summary-0227.sh`, `summary-0230.sh`, `table0226.py`,
   `table0230.py`, `cpu.py`, `cpu-copy.sh`, `reach.py`, `final.sh`. For
   example:

   ```sh
   E="work/bin/s51-final/gunfinger-eval --work work/s51/wnase --other-peaks-dir ~/.local/share/gunfinger/nas-dnb-peaks"
   $E --extra-rungs 3 sweep --other-rips --speeds edge
   python3 -I scripts/analysis/ladder_edge.py recall OUT.json CLUSTERS,CORPUS_CLUSTERS MAP docs/panels L0=A.json+B.json L3=C.json+D.json
   python3 -I scripts/analysis/cluster_speeds.py work/reports/library-nas-dnb-peaks/duplicate-clusters.json OUT.json 0.05 0.096
   ```

### What else to know

- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. The only mix searched was the
  Stakka development mix. Nothing ran on the candidate mixes of 0065 or
  on the NAS root's broadcast file. The NAS store was only read
  (`--other-peaks-dir`); `work/peaks` was only read.
- `docs/pair-verdicts.txt`, the clusters file and the answer log are at
  their start digests (`work/s51/listening-digests-start.txt`); no answer
  log was written; no manifest or committed panel changed.
- A read-only review by a separate agent (10:10-10:24) found no defect
  in the harness or the scripts and confirmed the numbers and the order
  of commits and runs; its points in the documents were all taken (brief
  51, 10:27; `3d28e70`): among them the upload behind seed 2027's wrong
  answers (ƆƐI3C's, not the Prototype upload, which is judged the same
  recording; 0059 used the same mistaken name), the bootleg's speed
  direction, and four deviations recorded in the brief.
- Final checks with `work/bin/s51-final/` (the Rust code last changed
  in `f85505a`; byte-identical to the run build, its `gunfinger` to the
  start build's): `scripts/check.sh` green (3:41); `regress
  session-32-linux` (0:50) and `regress session-33-linux-nas` (3:46)
  identical, every report identical to its baseline apart from
  `wall_seconds`; the digests unchanged.

## Session 52

### What changed

The product did not change: the CLI binary is byte-identical to
`s51-final`'s. One harness fix, after the review:
`identifiability` lists a panel file with no current peak record in the
direction's `without_record` instead of stopping the run (below). The NAS store lost the peak records and
tag notes of the five files that the NAS `.gunfingerignore` has left out
since 2026-10-25 and 2026-10-27 (approved by the user on 2026-10-27):
the three tracks of `knowledge-the_lowdown_vol_7-track_fix-1999-sour/`,
`__youtube_archivists/i-witness-dnb/Optical & Ryme Tyme - Space Cake
[CUT].m4a` and `__youtube_archivists/cryptic-jungle/Ryme Tyme - T-Minus
(Unreleased Vocal Mix).m4a`. Store-only runs and the harness's
`--other-peaks-dir` no longer search them.

In short: the prune changed nothing that is scored. Two sub-threshold
sweep candidates (7 hits each) and one clusters pair entry, all of
pruned files, are gone; the scans, calibrate and the clusters are as
before. `session-52-linux-nas` is the NAS-scale baseline from now on.

### Experiment 0231

| | Before (session 33's baseline) | After |
|---|--:|--:|
| NAS store records, revision | 40,683, `56433d74d0919a08` | 40,678, `1fc6fe50293ad330` |
| Sweep 2026: indexed assets; queries with identical detections | 40,580 | 40,575; 718 of 720 |
| Stakka scan; leave-outs 3, 11 | 11/11; 8/11; 0/11, 0 wrong | identical |
| Calibrate: weakest identifying, strongest false, false accepted | 704, 282, 7 | the same |
| Clusters; pair entries; file | 284; 2,877; `ac53014b8a87...` | 284; 2,876; `ce293fe6165e...` |

The two sweep queries (Yvon Is On at 0%, The Reprise at +1%) each lose
one candidate, the Space Cake upload and the track-fix Kraken, and
gain none. The pair entry is the corpus T-Minus against the vocal mix
upload (356 hits, `different` by ear).

### Panel entries that name a pruned file

Only the T-Minus upload is named: in `shared-material-seed-2026.json`
(family `t-minus`, validation half: a recording, the related file of
direction `t-minus-1` and the played file of `t-minus-2`) and in `shared-mixes-s21.json` and
`shared-mixes-s21-rips.json` (an `exclude` and a `recordings` entry),
in `docs/panels/` and `work/s33/panels/` alike. No sweep or recall panel
names any of the five.

- `shared-mixes`: a one-mix plan cut from `shared-mixes-s21.json` (seed
  2101, `dozer-1`, the 93 `exclude` and 27 `recordings` entries kept)
  ran without error under its three indexes. The draw comes from the
  corpus records alone, and the upload is never a played file, so the
  full plans run as before, less the upload's detections.
- `shared`: not run (the validation half stays unsearched). By the code
  a run would not fail and keeps its query count: queries are rendered
  from the NAS audio, still there, and a left-out file the index does
  not hold is passed over. But both directions change without a word:
  `t-minus-1` can no longer find its related file, so its test of a
  wrong related detection is empty, and `t-minus-2` searches its queries
  against indexes that no longer hold the played file (its "everything"
  and "played left out" indexes are the same). No fix, since a run
  neither fails nor changes its query count; a question below.
- `identifiability --half validation` (found by the review, not run):
  it stopped at `t-minus-1` with "no current record of ... in the other
  store", because it loads the record of every compared file. Fixed in
  the harness (`identifiability::with_records`, `pair::Records::holds`,
  tested): a file with no current record is listed in the direction's
  `without_record` (and on stderr and in the summary) instead of
  compared, and the run goes on. Without `--other-peaks-dir` the run
  still stops with the message that asks for it. The field is left out
  of the JSON when empty, so other reports do not change.
- `mixes --kind shared|beatmatched|drops` read the same panel; their
  draws do not change (lengths from FFprobe on the NAS audio, the
  families left out of a corpus-only index). Experiment 0185's `drops
  --other-pairs` mixes play the vocal mix upload, so scoring them again
  at NAS scale would no longer find its plays (0215's Kridian case is
  affected the same way).

### Times on the Linux machine (32 workers)

| Run | Time | Peak memory |
|---|--:|--:|
| `regress session-32-linux` (start; after) | 0:51; 0:59 | 0.4 GiB |
| `regress session-33-linux-nas` (start) | 3:47 | 5.3 GiB |
| `doctor` with the NAS configuration (before; after the prune) | 0:30; 0:04 | |
| `prune`; `prune --yes` | 0:05; 0:04 | |
| `clusters --from-peaks --reuse-pairs` | 0.2 s | |
| `regress session-33-linux-nas` on the pruned store (the saved index built again in 112 s) | 5:38 | 5.4 GiB |
| `shared-mixes`, one mix, NAS scale | 1:10 | 4.8 GiB |
| `scripts/check.sh` (before the fix; after) | 3:42; 3:47 | |
| `regress session-32-linux`, final (before the fix; after) | 0:51; 0:50 | 0.4 GiB |
| `regress session-52-linux-nas`, final, the saved index loaded (before the fix; after) | 3:47; 3:57 | 5.3 GiB |

### Decided this session, open to change

- `doctor` and `prune` without `--yes` ran beside the start regressions
  (all only read; no timing compared).
- `baseline session-52-linux-nas` first copied the reports of step 3's
  run (made with the start build); after the review's fix it was saved
  again from the final build's `regress` (identical; the first copy in
  `work/s52/baseline-first/`).
- The shared-material panel's validation half stayed unsearched, so its
  T-Minus direction was checked by the code only.

### Questions this leaves for us

- Whether `shared` (and `shared-mixes`) should report a panel file that
  the indexed store no longer holds, as `identifiability` now does,
  before the validation half is searched: directions `t-minus-1` and
  `t-minus-2` name the pruned upload.
- Whether the T-Minus family stays in the shared-material panel at all,
  since its related file is now outside the library (the panel is
  frozen; a change would be a new panel).

### Questions for listening (none answered by agents)

- None new. Pending: Star Trails, Into Music; the 58 `[CUT]` files of
  the i-witness-dnb channel (whether they are cuts from mixes is for the
  user to judge).

### Handoff for the next session

1. Baselines: `session-32-linux` (262 tracks, unchanged) and
   `session-52-linux-nas` (NAS scale, the pruned store); compare NAS-scale
   runs with `session-52-linux-nas`, not `session-33-linux-nas`.
2. The NAS store: 40,678 records, revision `1fc6fe50293ad330`; no
   records of ignored files. The harness's saved index
   `work/indexes/library-nas-dnb-peaks.index` is current for it; the
   CLI's in `~/.cache/gunfinger/indexes/` is built again by the next
   `identify` with the NAS configuration.
3. The NAS clusters file `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`:
   SHA-256 `ce293fe6165e31009ef0b0411e77836aced21716b00771ad07bef7099c2a177a`,
   284 clusters, 2,876 pair entries.
4. Builds: `work/bin/s52-start/` and `s52-run/` (both = `s51-final`;
   every run of steps 1-4), `s52-final/` (master: the `gunfinger`
   binary the same, `gunfinger-eval` with the identifiability fix).
5. Data: `work/s52/` (listings and logs in `logs/`, `where-before.tsv`,
   `diff-pruned.txt`, `pruned-reports/`, the clusters file before and
   after, the shared-mixes check in `wsm/`); scripts `run.sh`,
   `start.sh`, `step1-list.sh`, `step2.sh`, `step3.sh`, `step3b.sh`,
   `step4.sh`, `final.sh`, `final2.sh`, `where.py`, `diff.py`, `same.py`.
6. NAS-scale reports in `work/reports/library-nas-dnb-peaks/` outside
   the baseline predate the prune and name pruned files: the Vertical
   scans (the track-fix Propaganda, 6-7 hits), the recall reports (the
   track-fix Drumtrip, confident and correct in 8 Known Unknown queries),
   `shared-material-seed-2026-development.json` (Drumtrip, weak) and
   the `pairs-s40-*` and `pairs-s49-*` lists. They change when run again;
   `pair` on those lists stops at a pruned file.

### What else to know

- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. The only mix searched was the
  Stakka development mix (and one mix rendered for the `shared-mixes`
  check). Nothing ran on the candidate mixes of 0065 or on the NAS
  root's broadcast file. The NAS was only read; `prune --yes` ran once
  with the NAS configuration, after the listing of exactly the five
  files; `work/peaks` was only read.
- `docs/pair-verdicts.txt` and the answer log are at their start
  digests (`work/s52/listening-digests-start.txt`); no answer log was
  written; no manifest or committed panel changed. The clusters file
  changed only by the dropped pair entry.
- A read-only review by a separate agent (01:51-02:04) confirmed every
  number and found three errors in what the documents said about the
  harness; all were fixed (brief 52, 02:05), one of them in the harness.
- Final checks with `work/bin/s52-final/` (`d57ae16`; `gunfinger`
  byte-identical to `s51-final`'s): `scripts/check.sh` green (3:47);
  `regress session-32-linux` (0:50) and `regress session-52-linux-nas`
  (3:57) identical, every report identical to its baseline's apart
  from `wall_seconds`; the verdicts file and the answer log unchanged.

## Session 53

### What changed

Gunfinger changed as we decided on 2026-10-27 (ADR 0021): the default
matcher's second suppression is `SecondOverlap::SpansAtSpeed(0.006)`.
After the second pass, a detection whose measured span overlaps a
stronger one of the same file is dropped only when their fitted speeds
are within 0.6% (`search::SAME_SPEED_SHARE`, defined as the chain link
rule's limit, 1.5 ladder steps), so both sides of a pitch ride are kept.
`identify`, `explain` and every harness command with the default
matcher use it; `--single-pass` is unchanged. Two changes follow from
it:

- A play ends where its segment that ends last ends (`plays.rs`).
  Overlapping segments of one file were impossible before; a weaker one
  at another speed can now lie inside a stronger one, and the play would
  have ended early, its saved report rejected by `Report::validate` when
  read (`show`, `review`, `listen`, `--save-dir`).
- `search::design` is `lines-chains-v2`, so `identify --save-dir`
  searches again instead of reusing a report made under the old rule.
- `--format timeline` draws a play's segments weakest first, so a
  possible segment inside a confident one does not cover its mark (the
  review's finding; tested).

The harness keeps the old rule as `--overlap-by-spans` (reports in
`variant-spans/`); `--apart-in-speed SHARE` stays to vary the share and
refuses 0.006, the default. ADR 0008's status names what ADR 0021
supersedes; README, `docs/calibration.md`, the roadmap and the decision
note are updated.

In short: Star Trails' slower side in the Stakka mix and Into Music's
middle segment in the 2003 mix are kept; otherwise only weak detections
change. Calibrate, every identification and the printed tracklists of
the three development mixes are as before. `session-53-linux` and
`session-53-linux-nas` are the baselines from now on.

### Experiment 0232

| Before -> after | 262 tracks | NAS scale |
|---|--:|--:|
| Sweep 2026 queries whose detections changed | 0 of 720 | 4, one weak detection each |
| Stakka, leave-outs 3 and 11: identified, wrong | 11, 8, 0; 0 | 11, 8, 0; 0 |
| 2003 mix, Nocny (NAS scale only): identified, wrong | | 12/12, 17/17; 0 |
| Confident detections added | Star Trails' side (scan, leave-out 3) | the same on its 3 files; Into Music's middle |
| Calibrate | unchanged (690, 100, 18) | unchanged (704, 282, 7, 95) |
| CLI's printed tracklists, 3 mixes | identical | identical |

The added weak detections (6-20 hits) each lie 0.61-5.8% in speed from
a stronger overlapping one of their file; the removed ones are weak
Star Trails detections (9-18 hits) within 0.22-0.52% of the newly kept
side. User CPU: the rule costs +0.2% with one binary (the final build
with `--overlap-by-spans` against its default, A B B A on the Stakka scan
at NAS scale); the start build used 5% more than the final build, a
difference between the binaries, not the rule's (not traced).

### Times on the Linux machine (32 workers)

| Run | Time | Peak memory |
|---|--:|--:|
| `regress session-32-linux` (start; changed build) | 0:50; 0:51 | 0.4 GiB |
| `regress session-52-linux-nas` (start; changed build) | 3:58; 3:46 | 5.4 GiB |
| `scan` of the 2003 mix; Nocny, NAS scale (start; changed build) | 0:24, 0:34; 0:22, 0:33 | 5.0 GiB |
| `identify --store-only`, a development mix, 262 tracks; NAS scale (the first with the CLI's saved NAS index built again, 0:33) | 0:04-0:06; 0:13-0:19 | 0.2; 4.9 GiB |
| `scan stakka-skynet-knowledge`, NAS scale, for CPU | 0:27-0:29 | 5.0 GiB |
| `scripts/check.sh` | 3:58 | |
| `scripts/check.sh`, final (before the review's fix; after) | 3:43; 3:47 | |
| `regress session-53-linux`; `regress session-53-linux-nas`, final (both times) | 0:51; 3:47 | 0.4; 5.4 GiB |

### Decided this session, open to change

- How a play shows two overlapping segments: as measured, with the
  play's end at its segment that ends last (ADR 0021, decision 3).
  Clipping was not taken; it would change the spans that scoring and the
  JSON report use.
- A wrong file's added confident segment counts as a wrong answer, also
  where the default already names that file (the conservative reading,
  0178's V2; ADR 0021, decision 4).
- The share is defined as `chains::NEARBY_SPEEDS` (equal to 0.006 in
  f64; a test pins it), so it moves with the ladder's step.
- `search::design`'s version went to `lines-chains-v2`.
- The old rule is the harness's `--overlap-by-spans`; `--apart-in-speed`
  stays to vary the share and refuses the default's 0.006. The CLI has
  no option for either.
- The other two development mixes were scanned at NAS scale only, in
  scratch work directories, since `regress` runs only the Stakka scans.
- The CPU comparison added a same-binary A B B A after the first one
  showed a 5% difference between the builds.

### Questions this leaves for us

- Whether to count, when the shared panels or the drops run again, the
  confident segments a file gains against `--overlap-by-spans` (the
  residual risk of ADR 0021: a related version's second alignment more
  than 0.6% apart that reaches 240 hits on its own; none so far).
- Why the start build (`s52-final`'s binary) uses 5% more user CPU than
  the final build on the same scan, with identical reports (not traced).

### Questions for listening (none answered by agents)

- Star Trails at 13:13-14:37 in the Stakka mix and Into Music at
  25:57-26:56 in the 2003 mix: real pitch rides or not. Both segments
  are now kept by default, so listening checks the new default's two
  changes in the development mixes. Session 39's and session 40's
  remaining questions as before.

### Handoff for the next session

1. Baselines: `session-53-linux` (262 tracks) and `session-53-linux-nas`
   (NAS scale); compare with these, not `session-32-linux` or
   `session-52-linux-nas`, which hold the old second suppression.
2. Builds: `work/bin/s53-start/` (= `s52-final`); `s53-run/` and
   `s53-final1/` (identical, the code commit `3264ccb`: every run of
   steps 3 and 4); `s53-final/` (master, with the review's timeline
   fix; the same reports).
3. The NAS store (40,678 records, revision `1fc6fe50293ad330`) and the
   clusters file (`ce293fe6165e...`) did not change. The CLI's saved
   index of the NAS store (`~/.cache/gunfinger/indexes/`) was built
   again for 40,678 records by the first `identify --store-only`.
4. Reports made under the old rule outside the baselines (older scans,
   panels, mixes) differ from what the default gives now; run them
   again, or with `--overlap-by-spans` to compare like with like.
5. Data: `work/s53/` (logs in `logs/`; the CLI's reports and printed
   tracklists in `cli-before/` and `cli-after/`; the 2003 and Nocny
   scans in `wdev-start/` and `wdev-run/`, the latter also holding the
   `--overlap-by-spans` Stakka scan in `variant-spans/`); scripts
   `run.sh`, `start.sh`, `before.sh`, `cli.sh`, `devscans.sh`,
   `step3.sh`, `step4.sh`, `cpu2.sh`, `check.sh`, `final.sh`,
   `data0232.py`, `same.py`; `scripts/analysis/overlap_changes.py`
   lists what a change of the second suppression adds or removes.
6. Variants: `work/reports/variant-apart-0.006/` (both scales) holds
   reports equal to the default's, which the option no longer makes
   (it refuses 0.006); `variant-trim/`, `variant-stretch/` and the other
   variant reports were made under the old second suppression, while
   those variants now inherit the new one (harness reports do not
   record `search::design`). `pair` and `identifiability` call
   `Matcher::Fitted.options()` and also use the new rule; no `regress`
   covers them, and nothing was run with them this session.
7. The remaining decisions of 2026-10-27, each with its own ADR: gaps
   printed by default, and a near-identical version's confident
   detection counted as right; the minimum span stays undecided.

### What else to know

- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. The mixes searched were the
  three development mixes. Nothing ran on the candidate mixes of 0065
  or on the NAS root's broadcast file. No command was given the NAS
  configuration or library; NAS-scale runs used the NAS peak store on
  the local disk, and nothing was written to the NAS or to `work/peaks`.
- `docs/pair-verdicts.txt`, the answer log and the NAS clusters file
  are at their start digests (`work/s53/listening-digests-start.txt`);
  no answer log was written; no manifest or committed panel changed.
- A read-only review by a separate agent (02:45-02:55) confirmed the
  code, the tests and every number, and found a display bug
  (`--format timeline`, fixed and tested) and four inaccuracies in the
  documents (corrected; brief 53, 03:00).
- Final checks with `work/bin/s53-final/`: `scripts/check.sh` green;
  `regress session-53-linux` and `regress session-53-linux-nas`
  identical; the CLI's printed tracklists unchanged; both baselines
  saved again from this build (identical).

## Session 54

### What changed

The harness scores as we decided on 2026-10-27 (ADR 0022): a confident
detection of a file judged to carry the truth's title is right, and
every report counts such credits apart. Gunfinger itself, the clusters,
the verdicts and every search are unchanged (the `gunfinger` binary is
byte-identical to the start build's).

- `docs/same-titles.txt` holds our same-title judgments, one per line:
  `same-title` and two files, tab-separated, named as in the clusters
  files. It has one entry, our decision of 2026-10-27:
  `extra/b-bad_company-china_cup.mp3` against ƆƐI3C's upload (the pair
  of `docs/pair-verdicts.txt` line 84). A judgment relates the two
  files' clusters (so all three corpus China Cup rips and the Prototype
  upload are credited), is not followed through chains, and is read by
  scoring only. `titles::SameTitles` parses it; errors name the file and
  the line; a missing file holds no judgments; every scoring run warns
  of a judged file it neither indexes nor leaves out (at 262 tracks,
  ƆƐI3C's upload). Harness option `--same-titles FILE`.
- Scoring: the sweep and `recall` (`Outcome.same_title`,
  `Outcome::is_right`, `is_wrong`; `SpeedRow.same_title` and a `title`
  column), the scan (`credited_through_title` per track,
  `Score.same_title`, the count on the summary line), generated mixes,
  `grid` and the drops (`MixScore.same_title`, `GridQuery.same_title`),
  `robust` (its own
  counts stay the recording's; `same_title` apart) and calibrate (no
  false candidate; may identify; `same-title credits` line; `regress`
  prints it and the per-speed and per-scan counts). The counts and flags
  are written only when not zero, so earlier reports read the same.
- Unchanged, by design: `seconds`, `loss`, and the commands that measure
  shared material or decide recordings (`shared`, `shared-mixes`,
  `identifiability`, `holders`, `related`, `pair`, `clusters`); the
  possible tier's null on held-out excerpts keeps every detection.
- ADR 0004's status names the amendment; ADR 0022 records that it
  changes the brief's section 7.6 by our decision and that a later test
  evaluation would be scored by the same code. AGENTS.md names `titles`
  and the rule; `docs/calibration.md` and the roadmap's China Cup
  bullets are marked.

In short: the baselines hold no China Cup query, so `regress` is
identical at both scales; over the NAS sweeps of seeds 2026-2028 the
wrong confident answers fall from 25 to 7 (all Cool Rock Stuff), and
calibrate's false accepted from 25 to 7, with no threshold moving.

### Experiment 0233

| Scale, seed | Recalled | Wrong confident, before -> after | Same-title credits |
|---|--:|--:|--:|
| 262 tracks, 2026-2028 | 540 each | 0 -> 0 | 0 |
| NAS, 2026 | 540 | 7 -> 7 (Cool Rock Stuff) | 0 |
| NAS, 2027 (China Cup indexed) | 540 | 11 -> 0 | 11 |
| NAS, 2028 (China Cup held out) | 540 | 7 -> 0 | 7 |

Calibrate over seeds 2026-2028 and the Stakka scans at NAS scale: false
accepted 25 -> 7, false candidates 130,663 -> 130,639 (308 -> 284 at 30
hits or more), strongest false 282 (Cool Rock) and weakest identifying
675 unchanged; at 262 tracks nothing changes. In the baselines (seed
2026) calibrate is unchanged: 690, 100, 0 false accepted at 262 tracks;
704, 282, 7 at NAS scale.

### Times on the Linux machine (32 workers)

| Run | Time | Peak memory |
|---|--:|--:|
| `regress session-53-linux` (start; changed build) | 0:51; 0:51 | 0.4 GiB |
| `regress session-53-linux-nas` (start; changed build) | 3:46; 3:53 | 5.3-5.4 GiB |
| `sweep --seed 2027`, `2028`, 262 tracks (each build) | 0:48 | 0.25 GiB |
| `sweep --seed 2027`, `2028`, NAS scale (each build) | 2:26-2:31 | 5.0-5.2 GiB |
| `calibrate`, seeds 2026-2028, NAS scale | 0.1 s | 0.08 GiB |
| `scripts/check.sh` (stopped at clippy; green) | 0:04; 3:57 | |

### Decided this session, open to change

- The relation is a committed file of judgments between files, read as
  a relation between their clusters, and not followed through chains
  (A ~ B and B ~ C do not credit C for A).
- A same-title detection counts toward recall of an indexed sweep
  excerpt, and may be calibrate's identifying detection of a query, as
  a scan's strongest candidate in a track's window may be.
- On a held-out excerpt or in a left-out scan slot it is right too
  (the hold-out removes a recording; the version stays indexed with the
  title). Leaving judged files out with their partners would change
  what the harness searches.
- `not_indexed` (calibrate's possible-tier null on held-out excerpts)
  keeps every detection, a same-title one included.
- `robust` keeps its own-cluster counts and counts same-title ones
  apart; `recall`'s and `robust`'s partners accept the partner's
  same-title files.
- `seconds` and `loss` keep the cluster alone (see the handoff).
- Counts and flags are written only when not zero or true.

### Questions this leaves for us

- Same-title judgments are ours to write; none was added beyond China
  Cup. The judged `different` pairs we described as the same tune in
  another mix are the candidates:
  - Aphrodite - Cool Rock Stuff (APH027 B1) ~ Mobb Deep Vs Urban
    Takover - Cool Rock Shook Ones (2026-10-22: a different mix, one
    drum loop replaced by a similar one). Its detections are the 7 wrong
    confident answers left at NAS scale (seed 2026) and calibrate's
    strongest false (282).
  - Aphrodite - Tower Bass (Millennium Mix) (APH31 B) ~ `a - aphrodite -
    tower bass.mp3` (2026-10-25: the same tune in another mix, a
    slightly different bass and hi-hats), and the Millennium Mix ~ the
    JumpUpThrowDown version (Lacerba CERBAL6 D2), judged different
    earlier with no comment recorded in `work/audition/`.
  - Aphrodite - Stalker (APH029 B) ~ Stalker (Album Version), judged
    different, named in session 54's prompt as a candidate.
  Remixes and VIPs among the judged pairs (Sonar's Mark System
  Revision, the Hide U remix, We R The Ones VIP, Fire's Rogue Unit VIP)
  are hard negatives by the brief and are not proposed.
  Closed on 2026-10-30: by ear, all four pairs carry different titles
  (status, "the listening of the 2026-10-30 pack"), so
  `docs/same-titles.txt` does not change and the Cool Rock detections
  stay wrong answers.
- Whether `seconds` and `loss` should follow the judgments as well.
- Whether a hold-out or leave-out should also leave out the files judged
  to carry the recording's title, so that the null keeps meaning "this
  title is not in the index".

### Questions for listening (none answered by agents)

- None new. Session 53's (Star Trails at 13:13-14:37 in the Stakka mix,
  Into Music at 25:57-26:56 in the 2003 mix) and the earlier sessions'
  remain.

### Handoff for the next session

1. Baselines: `session-54-linux` (262 tracks) and `session-54-linux-nas`
   (NAS scale), identical to session 53's; compare with these.
2. The file: `docs/same-titles.txt`, ours, written by hand; one entry
   (China Cup ~ ƆƐI3C's upload). Agents never add entries; the
   candidates are listed under "Questions this leaves for us". After a
   new entry, run the sweeps and scans whose reports should credit it;
   saved reports keep the scoring they were written with (calibrate and
   `regress` read the stored `correct` and `same_title` flags of
   sweep reports, and the stored score of scans).
3. Builds: `work/bin/s54-start/` (= `s53-final`); `s54-run/` (the
   harness commit `91d5e80`: step 4's regressions, the seed sweeps and
   the baselines); `s54-final/` (master).
4. Data: `work/s54/` (logs in `logs/`; the seed 2027 and 2028 sweeps of
   both builds at both scales in `wseeds-start/` and `wseeds-run/`,
   with copies of the session-53 baseline reports for calibrate);
   scripts `run.sh`, `start.sh`, `seeds.sh`, `step4.sh`, `step5.sh`,
   `calib.sh`, `check.sh`, `final.sh`, `sweepdiff.py`, `data0233.py`,
   `same.py`.
5. Left for later: `seconds` and `loss` score by cluster alone;
   hold-outs and leave-outs remove recordings, not titles (questions
   above). Saved reports of `mixes`, `grid`, `robust` and `recall`
   were written before the change and were not run again.

### What else to know

- Experiment 0059's 16 (18 with extra rungs) wrong answers were China
  Cup's Prototype upload (PRO 001, its data), which the `same` verdict
  of 2026-10-09 joined into China Cup's cluster; experiment 0230's
  remark that 0059 meant ƆƐI3C's upload is mistaken (0230 stays as
  written; ADR 0022 and `docs/calibration.md` say it correctly).
- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. No mix was searched (the sweeps
  search excerpts of library files; `regress` scans the Stakka mix).
  Nothing ran on the candidate mixes of 0065 or on the NAS root's
  broadcast file. No command was given the NAS configuration or
  library; NAS-scale runs used the NAS peak store on the local disk,
  and nothing was written to the NAS or to `work/peaks`.
- `docs/pair-verdicts.txt`, the answer log and the NAS clusters file
  are at their start digests (`work/s54/listening-digests-start.txt`,
  `-end.txt`); no answer log was written; no manifest or committed
  panel changed. `docs/same-titles.txt` holds exactly the one entry of
  our decision.
- A read-only review by a separate agent (03:42-03:50) confirmed every
  number of 0233 and the rules, and found eleven points, all fixed
  (brief 54, 03:42-03:50): among them the 0059 attribution above, the
  6 ƆƐI3C detections below the rule, `robust` described as crediting,
  `grid` without a count, two missing tests, silent misspelt names and
  calibrate's stored confidence flag.
- Final checks (03:58-04:06) with `work/bin/s54-final/` (`ab3c3d0`; its
  `gunfinger` byte-identical to the start build's): `scripts/check.sh`
  green (3:42); `regress session-54-linux` (0:50) and `regress
  session-54-linux-nas` (3:45) identical, every report identical to
  its baseline's apart from `wall_seconds`; the digests unchanged. At
  262 tracks each run warns that ƆƐI3C's upload is not indexed, as
  designed; at NAS scale it is silent.

## Session 55

### What changed

`identify` and `show` print the time without a confident match, part 1
of our decision of 2026-10-27 (ADR 0023; the printing note's
"Decision"). The search, the detections, the entries, CSV, the cue
sheet and the timeline do not change; `regress` is identical at both
scales.

- The tracklist (the default) gets an unnumbered line at the start of
  each stretch of 45 s or more that no segment of a confident play
  covers, the mix's start and end included:
  `      6:43  ID - ID (7:57 without a confident match)`. A gap line comes
  before an entry starting at the same moment; possible entries inside a
  gap keep their lines; a possible play ADR 0019 leaves out stays out. A
  tracklist without any entry still prints `no tracks found`.
- The table (`--format human`) gets a row per gap, in time order among
  the plays: the span, and the same words in the asset column.
- JSON gets `gaps` (always written): `start_seconds`, `end_seconds`,
  `possible_plays` (positions from 1 in `plays` of the possible plays
  with a segment overlapping the gap). Schema version 3 (added field).
  The gaps are derived from the plays whenever a report is rendered, so
  `show` lists them for older reports; a saved report's own `gaps` are
  passed over when read.
- No option changes the 45 s (`MIN_GAP_SECONDS` in the CLI's `gaps.rs`)
  or turns gaps off (ADR 0023, decision 2).
- One definition: `gunfinger_core::gaps::uncovered(covered, start, end,
  min_seconds)`, called by the CLI and by the harness's `gaps::list`
  (whose listings stay byte-identical, `work/s55/gapsame.sh`).
- `tracklist_score.py` passes over gap lines (`GAP_LINE`, tested in
  `test_tracklist_score.py`). README describes the lines with the 2003
  mix's tracklist at 262 tracks; AGENTS.md names the modules.

### Experiment 0234

Every gap line the development mixes print is a track not in the
library at that scale: at 262 tracks 4 lines in the 2003 mix
(Konspiracy at the mix's start; Escape Route and Phantom Force;
Kinetic; Artificial Life) and 6 in Nocny (Mutated; The Descent VIP;
Invid, Mistaken Identity and Nephelim in one 14:27 line; Stab 2 Def;
We R The Ones VIP and Cannibal; slot 17, which the manifest itself names
"ID - ID"); at NAS scale only Nocny's slot 17 (3:10). Stakka prints none
at either scale. No line is a found track's uncovered time, an intro or
an outro, and no gap holds a possible play. All gaps last 1:39 or more,
so 30, 45 and 60 s print the same lines. In session 34's 42 NAS-scale
slot leave-outs, run again with today's search, 45 s and 60 s list the
same 128 gaps (none without a missed track, every missed track
reached); 30 s adds 12, 8 of them inside found tracks (43 s in
Blindside, 34 s in Kinetic). Where 45 s lists more than 60 s is brief
plays (0159's generated mixes).

### Times on the Linux machine (32 workers)

| Run | Time | Peak memory |
|---|--:|--:|
| `regress session-54-linux` (start; run build) | 0:50; 0:51 | 0.4 GiB |
| `regress session-54-linux-nas` (start; run build) | 3:49; 3:59 | 5.3-5.4 GiB |
| `identify --store-only`, a development mix, 262 tracks; NAS scale | 0:04-0:06; 0:13-0:19 | 0.2; 4.9 GiB |
| `scan`, NAS scale, 45 development scans (plain and leave-outs) | 0:23-0:35 each, 22 min | 5.1 GiB |
| `gaps` on 45 scan reports; on 6 saved reports | 0.5 s; 0.04 s | |
| `scripts/check.sh` (stopped at fmt; `decode::watch` under load; green) | 0:00; 4:13; 4:19 | |

### Decided this session, open to change

- Unnumbered "ID - ID" lines: a number would count the gap as one
  track. The length follows in brackets, cut to the second.
- No command-line option for the length or for turning gaps off.
- The timeline does not change (its bars already show uncovered time).
- The table's gap row is plain, not dimmed; the other columns stay
  empty.
- `possible_plays` counts a possible play by its segments (as the
  harness counts), not by its whole span (as the prototype did).
- JSON always writes `gaps`, empty when there is none; part 2's
  estimate is reserved as `tracks` on each gap and not written now.
- The table and JSON list a gap over the whole searched part when
  nothing was found; only the tracklist keeps `no tracks found` alone.
- Gaps are computed at render time and never stored or read back.

### Questions this leaves for us

- Whether 45 s is right once real mixes are read (on the development
  mixes 45 s and 60 s print the same lines).
- Part 2: which estimate, in what form, and the wording on the line (the
  combined estimate of 0165 is the best measured; none met its
  registered bar for a count).

### Questions for listening (none answered by agents)

- None new. Session 53's (Star Trails at 13:13-14:37 in the Stakka mix,
  Into Music at 25:57-26:56 in the 2003 mix) and the earlier sessions'
  remain.

### Handoff for part 2 (the estimate)

1. Where it goes: the CLI's `gaps.rs` (`Gap` gains `tracks`, and
   `Gap::label` adds the estimate after the length, for example `ID - ID
   (7:57 without a confident match; about 1-3 tracks)`, wording ours to
   decide); `output.rs`'s `WithGaps` writes it into JSON as `tracks` on
   each gap (an added field; schema version 3 allows it); the table's
   gap row uses the same label. Unit tests beside `gaps.rs`'s.
2. Method 1 (0160) needs only the plays: the entry starts
   (`export::entries`, or the harness's `entry_starts`), the median of
   the start-to-start intervals without a listed gap, and at least 3
   such intervals; it can be derived at render time as the gaps are,
   and `show` keeps working on saved reports.
3. Method 2 (self-match, 0162) and the combined estimate (0165) need
   the gap's audio: the mix decoded again for each gap, pair hashes
   matched against the gap's own at speed 1.0 (about 0.05 s of wall time
   per gap at 16 jobs; every rung to ±3.2% for pitch rides costs 5.4
   times as much, 0166). It runs in `identify` while the audio is at
   hand, so its result must be stored in the report and read back by
   `show` (unlike the gaps themselves, which are derived on render), or
   `show` must decode the mix again. The harness's
   `self_match::blocks(samples, profile, ratios)` is the measured code;
   moving it into the core would let the CLI and the harness share it as
   they now share `gaps::uncovered`.
4. A gap with no missed track gets no 0 from either method; at 45 s none
   was listed in the development mixes (0234), so the estimate's errors
   there come from the count itself.

### What else to know

- Data: `work/s55/` (logs in `logs/`; the CLI's reports and printed
  tracklists in `cli-before/` and `cli-after/`; the leave-out scans and
  their listings in `wleave/`; the gap listing comparison in
  `wgaps-start/` and `wgaps-dev/`); scripts `run.sh`, `start.sh`,
  `cli.sh`, `step3.sh`, `leave.sh`, `gapsame.sh`, `check.sh`,
  `beforeafter.py`, `gaplines.py`, `thresholds.py`, `leavecount.py`,
  `checklines.py`. Builds `work/bin/s55-start/` (= `s54-final`),
  `s55-dev/` (before `cargo fmt`; the listing comparison), `s55-run/`
  (the code commit `012ad23`: step 3) and `s55-final/` (master).
- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. The three development mixes were
  named explicitly; nothing ran on the candidate mixes of 0065 or on the
  NAS root's broadcast file. No command was given the NAS configuration
  or library; NAS-scale runs used the NAS peak store on the local disk
  (the CLI's `--store-only`, the harness's `--other-peaks-dir`), and
  nothing was written to the NAS or to `work/peaks`. The leave-out
  scans ran in a scratch work directory, so `work/reports/` holds no
  leave-out report.
- Baselines stay `session-54-linux` and `session-54-linux-nas`: the
  harness's detections did not change, so no new baseline was recorded.
- A read-only review by a separate agent (05:02-05:10) found no
  correctness bug and ten points on tests and wording, all fixed (brief
  55, 05:02-05:10): among them a test of an excerpt's edges and one of
  nested coverage that fails under a wrong union.
- Final checks (05:16-05:26) with `work/bin/s55-final/` (`4b18a9c`):
  `scripts/check.sh` green (4:21); `regress session-54-linux` (0:51)
  and `regress session-54-linux-nas` (3:58) identical, every report
  identical to its baseline's apart from `wall_seconds`;
  `docs/pair-verdicts.txt`, the answer log and the NAS clusters file
  at their start digests; no answer log written, no manifest or panel
  changed.

## Session 56

### What changed

Each gap line now estimates how many tracks the gap holds, part 2 of our
decision of 2026-10-27 (ADR 0024, amending ADR 0023; the printing note's
"Decision", item 5). The search, the detections, the entries, the gaps
themselves, CSV, the cue sheet and the timeline do not change; `regress`
is identical at both scales.

- The estimate is 0165's combined one: method 2's count (the gap matched
  against itself at speed 1.0; blocks at 10 hits spanning 30 s or more,
  overlapping ones merged) limited to method 1's range B (the gap over the
  quartiles of the mix's clean intervals between entries); 0 stays 0; a
  mix with fewer than 3 clean intervals keeps method 2's count.
- Printed always as a range one either side, at least 1:
  `ID - ID (7:57 without a confident match; about 1-3 tracks)`, in the
  tracklist and the table's gap row. A count of 0 prints no estimate.
  Never a single number: on the test, a printed 1 held one track for 95
  of 125 gaps.
- JSON: `tracks` on each gap (`null`, or `low`, `high`, `method`:
  `repeats-within-length` or `repeats`); the report gains `gap_repeats`,
  the repeated stretches in recording time, from which the estimate is
  derived whenever the report is rendered, so `show` prints it. A report
  saved before this change shows no estimate; `identify --save-dir` does
  not search it again on that account.
- `identify` matches each gap from the samples it already holds (no
  second decode): `gunfinger_core::repeats` (the harness's
  `self_match.rs`, moved; `records_in_gaps` runs one gap per worker).
  Method 1's entries, clean intervals and median moved from the harness
  to `gunfinger_core::gaps` beside `length_range` and `estimate`; the
  harness's listings are unchanged and its `self-match` writes the
  product's `tracks` beside the blocks.
- `tracklist_score.py` passes over gap lines with an estimate (tested).
  README's example shows the 2003 mix with its estimates; AGENTS.md
  names `repeats`.

### Experiment 0235

On the development mixes every printed range holds the missed tracks:
the 2003 mix at 262 tracks 1-2, 1-3, 1-2, 1-2 (holding 1, 2, 1, 1, all
exact); Nocny at 262 tracks 1-2, 1-3, 2-4, 1-2, 1-3, 1-3 (holding 1, 1,
3, 1, 2, 1; The Descent VIP and slot 17 counted 2 each; one clean
interval, so method 2 alone); Nocny at NAS scale 1-2 for slot 17 (exact).
Stakka prints no gap. The CLI and the harness's `self-match` agree on
all 10 gaps at 262 tracks. On 0163's six test seeds, rendered again, the
product's code gives 0165's numbers exactly for all 384 gaps: exact
99/57/58/43% at k = 1-4, the range holds 100/99/100/89.6%, 2.9% off by
two (shared material only: 98/44/48/31%, 100/100/100/83.3%, 4.2%), at 30
and at 45 s, on session 35's and today's reports.

Cost: the gap matching takes under 0.05 s on the development mixes and
0.7 s for a 67-minute gap; `identify` at NAS scale takes 0.3-0.5 s more
(12.8 -> 13.1 s, 18.1 -> 18.6 s), most of it in the unchanged search.

### Times on the Linux machine (32 workers)

| Run | Time | Peak memory |
|---|--:|--:|
| `regress session-54-linux` (start; core-loop build) | 0:50; 0:51 | 0.4 GiB |
| `regress session-54-linux-nas` (start; core-loop build) | 3:58; 4:00 | 5.3-5.6 GiB |
| `identify --store-only`, NAS scale, the 2003 mix; Nocny (start; first build; core-loop build) | 12.8; 13.4; 13.1 s / 18.1; 19.1; 18.6 s | 4.9-5.0 GiB |
| `identify`, Nocny with nothing found (one 67-minute gap; start; first build) | 6.6; 7.1 s | 158; 200 MB |
| `mixes --count 16`, shared and beatmatched, 16 jobs | 2:33-5:46 per seed | 14-15 GiB |
| `self-match`, 384 test gaps, 16 jobs | 0:22-0:23 | 1.2 GiB |
| `scripts/check.sh` (beside the renders; final) | 6:16; 4:38 | |

### Decided this session, open to change

- Always a range, never "about 1 track": exactness at k = 1 says how
  often a gap of one track is counted 1, not how often a printed 1 is
  right (76% on the test).
- A count of 0 prints nothing, and JSON's `tracks` is `null`.
- Speed 1.0 only; the neighbouring rungs of 0166 stay out.
- Method 1's intervals use the gaps listed at 45 s (0165 used 30 s; on
  the test seeds both give the same figures).
- Old reports print no estimate and are not searched again for it.
- The per-gap loop lives in the core (`repeats::records_in_gaps`): with it
  in the CLI crate the unchanged search ran 4-6% slower, and a build
  without the call searched as fast as the start build; in the core the
  cost halves. That this is a compile effect is an inference, not
  examined in the machine code; the release profile was not changed.

### Questions this leaves for us

- Whether the line should say anything when the count is 0 (today it
  prints the length alone, as a gap without repeats).
- Whether the remaining 2-3% of search time is worth examining (a
  profile setting such as LTO or one codegen unit would change every
  timing, so it needs its own measurement).
- The limits ADR 0024 names: records sharing a passage (83% at k = 4),
  pitch rides (0166) and a gap holding no missed track, which still gets
  at least `about 1-2 tracks` when the found track repeats there.

### Questions for listening (none answered by agents)

- None new. Session 53's (Star Trails at 13:13-14:37 in the Stakka mix,
  Into Music at 25:57-26:56 in the 2003 mix) and the earlier sessions'
  remain.

### What else to know

- Data: `work/s56/` (logs in `logs/`; the CLI's reports and tracklists in
  `cli-before/`, `cli-after/` and `cli-core/`; the listing comparison in
  `wgaps-start/` and `wgaps-dev/`; the test seeds' reports and listings
  in `wmix/` (`s35-reports/`, `reports/`, `listings-s35/`,
  `listings-today/`, `listings-45/`; renders deleted at the end); the
  one-file noise library in `tiny/`); scripts `run.sh`, `start.sh`,
  `cli.sh`, `renders.sh`, `gapsame.sh`, `samelistings.py`, `seeds.sh`,
  `reproduce.py`, `timing.sh`, `layout.sh`, `layout2.sh`, `step4.sh`,
  `beforeafter.py`, `gaplines.py`, `gapsvs.sh`, `check.sh`, `final.sh`.
  Builds `work/bin/s56-start/` (= `s55-final`), `s56-dev/` (the harness
  for the listing comparison), `s56-run/` (`ce50a95`), `s56-noop/` (the
  call removed, never committed), `s56-core/` (`74ba54d`) and
  `s56-final/`.
- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. The three development mixes were
  named explicitly; nothing ran on the candidate mixes of 0065 or on the
  NAS root's broadcast file. No command was given the NAS configuration
  or library; NAS-scale runs used the NAS peak store on the local disk,
  and the shared seeds read panel files under the NAS root (read only).
  Nothing was written to the NAS or to `work/peaks`; the test seeds ran
  in a scratch work directory, so `work/reports/` and `work/gaps/` hold
  nothing new.
- Baselines stay `session-54-linux` and `session-54-linux-nas`.
- A read-only review by a separate agent found no correctness bug and
  eleven points on tests and wording, all fixed (brief 56, 06:23-06:31).
- Final checks (06:33-06:43) with `work/bin/s56-final/` (`4d145e9`):
  `scripts/check.sh` green (4:38); `regress session-54-linux` (0:51)
  and `regress session-54-linux-nas` (3:59) identical, every report
  identical to its baseline's apart from `wall_seconds`; the final
  build's `show` prints the core-loop build's tracklists byte for byte;
  `docs/pair-verdicts.txt`, the answer log and the NAS clusters file at
  their start digests (`work/s56/listening-digests-end.txt`); no answer
  log written, no manifest or panel changed. Done at 06:43, within the
  time box.

## Session 57

### What changed

Nothing in the product or the harness: the session reran the saved
evaluations outside `regress` under tonight's changes (session 52's
prune, ADR 0021, ADR 0022, ADR 0023-0024) and compared each with its old
report, as the pre-registration
`docs/research/2026-10-27-rerun-under-tonights-changes.md` fixed before
any run. Experiments 0236-0240; the roadmap's "Speed that wanders
within a play", "Recall beyond the corpus recordings", "A confidence
statistic for shared material" and "Behaviour at scale" and two rows of
`docs/calibration.md` carry the new numbers.

In short: no finding. Every difference is the prune's, ADR 0021's, ADR
0022's or a gap line. Differences outside those definitions were traced
with an old binary on today's store or today's build under
`--overlap-by-spans` (all but one weak recall candidate, and the drops
seeds other than 2032, inferred alike); detections of unpruned files that
old binaries reproduce on today's store are attributed to the prune,
which widens the pre-registered definition (brief 57, 10:15).
ADR 0021's residual risk did not occur: the strongest detection kept
more than 0.6% apart of a recording not playing has 67 hits (the
Clockwork remix, possible).

### What each group of runs showed

| Group (experiment) | Before -> after | Explained by |
|---|---|---|
| Shared panels' development halves, NAS scale (0236) | confident detections by role identical in all 622 and 440 queries; 185 and 83 weak or possible added | ADR 0021 (each equals its session 38 or 45 variant run); 24 and 3 weak detections of pruned files gone |
| Drops of sessions 39 and 48: harness at 262 tracks, CLI at both scales (0237) | harness scores identical in 12 reports; CLI tracklists identical apart from 337 gap lines (each over a missed track); at NAS scale the T-Minus vocal mix upload's 38 entries gone | the prune (the upload; 20 re-measured segments, which session 48's binary on today's store reproduces), ADR 0021, ADR 0023-0024 |
| `recall`, the 2003 mix and Nocny at NAS scale (0238) | recall identical except other rips only 175 -> 174 of 180 and, with brief plays, 176 -> 171 of 360; scans identical in every score | the prune: the 6 excerpts' only identifying rip was the pruned Drumtrip copy; it also moves one confident right detection (455 -> 503 hits), reproduced by session 33's binary on today's store |
| Sweeps 2027-2029, calibrate, `robust`, `grid` (0239) | NAS wrong answers 11 -> 0 and 7 -> 0 (same-title credits); 2029 540/540, 0 wrong at both scales; robust's rides +3 and +4 confident; `grid` 0, 65, 144 of 160 under both rules | ADR 0022, ADR 0021, the prune (session 40's binary on today's store reproduces every move); `grid`'s difference from the Mac's 0047 is the machine's (session 32's build gives the same) |
| Short, long and ridden mixes at 262 tracks (0240) | right plays and wrong confident unchanged; the rides keep both sides (911 confident detections added); one wrong possible added (8 -> 9) | ADR 0021 (each equals its session 38 variant run) |

### ADR 0021's residual risk, counted

| Runs | Kept more than 0.6% apart | Of a recording not playing, at most | Of any, at most |
|---|--:|--:|--:|
| Shared panels (0236) | 268 | 55 hits (a related version, role `related`) | 61 (possible, the played file) |
| Drops (0237) | 66 | 44 | 109 (the played T-Minus at its own speed) |
| Generated mixes (0240) | 983 | 67 (the Clockwork remix, possible) | 7,537 (a side of a ride) |

None of a recording not playing reached 60 hits outside the Clockwork
pair, and none reached 240.

### Times on the Linux machine (32 workers)

| Run | Time | Peak memory |
|---|--:|--:|
| `regress session-54-linux`; `-nas` (start) | 0:51; 3:59 | 0.4; 5.6 GiB |
| `shared`, panels 1 and 2, at once | 25:59; 19:07 | 6.6; 5.9 GiB |
| `mixes --kind drops` (12 reports, cached renders) | 1:25-1:53 each | 1.5-2.8 GiB |
| `identify`, 16 drops mixes: 262 tracks; NAS scale (two queues at once) | 1:33-1:52; 2:25-3:39 | 0.2; 5.5 GiB |
| `recall`, 8 conditions (two queues at once) | 3:59-18:23 | 5.0-5.5 GiB |
| `scan`, NAS scale, 8 reports; 262 tracks, 2 | 0:49-1:30; 0:20-0:28 | 5.3; 0.2 GiB |
| `sweep`, 262 tracks; NAS scale (three queues at once) | 0:47-2:11; 2:31-7:54 | 0.2; 5.4 GiB |
| `robust --only` five rides: 262 tracks; NAS scale | 0:50; 4:26 | 0.2; 5.1 GiB |
| `grid` (today's; `--overlap-by-spans`; session 32's build) | 6:32; 4:35; 6:44 | 1.4 GiB |
| `mixes`, short; long and rides (cached renders) | 0:16; 1:42-2:58 | 0.5; 2.6 GiB |
| Old binaries on today's store: session 40's sweeps; session 33's recall clean, brief; session 48's `identify` | 13:51-18:05; 23:50, 12:15; 2:23 | 5.1-5.5 GiB |

Wall times ran beside other queues and are not comparable with the old
runs'.

### Decided this session, open to change

- Old reports: the newest default report before tonight's changes, and
  session 38's or 45's `--apart-in-speed 0.006` report where one exists
  (the variant ADR 0021 adopted) as a second comparison.
- Session 48's turntable `--verify` reruns of the drops seeds 2026 and
  2028 were not run again: they play session 39's mixes, which ran.
- Session 39's NAS-scale other pairs had used the NAS configuration;
  they ran with `--peaks-dir --store-only`, as sessions 45 and 48 did.
- Four traces were added during the runs (brief 57): session 40's and
  session 33's harness binaries and session 48's CLI binary on today's
  store with `--saved-index off`, and `grid` with session 32's build.
  What an old binary on today's store shares with today's build is the
  prune's effect.
- `diffs.py` counts an overlapping detection of equal hits as the
  stronger one's partner (the old rule ranked one of a tie first), and
  keys sweep and recall queries by their position (two brief queries
  share asset, start and speed).

### Questions this leaves for us

- The prune moves detections of files it did not touch: confident
  detections re-measured by a hit or two (and one by 48 hits, recall's
  brief query 911) and weak 6-hit candidates. This is outside the
  pre-registered definition of the prune; old binaries reproduce each
  traced move on today's store, so we attribute them to the store's
  content, but how (by inference, through which posting lists the first
  pass sets aside) was not traced. Open item; the smallest reproduction
  is `work/s57/trace-brief33.sh` (session 33's binary, recall's brief
  queries on today's store) against the old report
  `work/reports/library-nas-dnb-peaks/recall-seed-2026-brief.json`,
  query 911 (Organikismness - Echos at +1%: 455 hits at +1.147% before,
  503 at +0.933% now).
- The recall panel's other-rips-only form lost 6 excerpts whose only
  identifying rip was the pruned track-fix Drumtrip copy. Whether that
  copy was a rip of Known Unknown - Rollers Edit or a different
  recording that the clusters join is for us to judge; the panel and
  the clusters stay as they are.
- `docs/calibration.md`'s baseline rows are the Mac's (0047); the Linux
  machine's figures are now noted beside them.

### Questions for listening (none answered by agents)

- None new. Session 53's (Star Trails at 13:13-14:37 in the Stakka mix,
  Into Music at 25:57-26:56 in the 2003 mix) and the earlier sessions'
  remain.

### What was not run

- `shared-mixes` (no full report on this machine; session 21 ran it on
  the Mac), `robust` without `--only` (no saved full report here), the
  generated mixes whose renders were deleted (plain and long 2027-2029
  and 7, long 3531-3532, rides 3521-3522), seed 3701's ride dose (made
  by trace builds), and NAS-scale generated mixes (none saved). The
  shared and beatmatched seeds 3501-3513 were run by session 56 after
  every change.

### Handoff for the next session

1. No finding. Open item, first: the prune moves detections of files it
   did not touch (above, "Questions this leaves for us"); reproduce with
   `sh work/s57/trace-brief33.sh` and query 911 of the brief recall
   report. Nothing in the product depends on it.
2. Baselines stay `session-54-linux` and `session-54-linux-nas`; the
   saved reports outside `regress` named in the pre-registration now have
   current counterparts in `work/s57/w*/reports/` (compare with those).
3. Not run (above): `shared-mixes`, full `robust`, mixes with deleted
   renders, seed 3701's ride dose.

### What else to know

- A read-only review by a separate agent (10:00-10:15) found no missed
  regression and eight points on numbers and wording, all fixed (brief
  57, 10:15).
- Data: `work/s57/` (logs in `logs/`, comparison outputs in `out/`;
  scratch work directories `wshared`, `wdrops`, `wrecall`, `wscans`,
  `wsweeps`, `wrobust`, `wmixes` and the traces' `wsweeps40`,
  `wrecall33`, `wgrid32`; the CLI's reports in `cli/`, the old ones
  copied to `cli-old/`, session 48's binary's in `cli-s48bin/`). The
  reflink copies of the renders were deleted at the end; the reports
  stay. Scripts: `run.sh`, `same.py`, `scratch.sh`, `start.sh`,
  `shared.sh`, `drops-setup.sh`, `drops-harness.sh`, `drops-nas.sh`,
  `recall.sh`, `scans.sh`, `group3-setup.sh`, `group4.sh`,
  `group4-setup.sh`, `group5.sh`, `group5-setup.sh`, `trace-*.sh`,
  `compare-*.sh`, `final.sh`, `diffs.py`, `residual.py`,
  `residual_mixes.py`, `mixes_score.py`, `cli_compare.py`, `counts.py`,
  `grid_counts.py`.
- Builds: `work/bin/s57-start/` and `s57-final/`, both `cmp`-equal to
  `s56-final`.
- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. The mixes searched were the
  three development mixes (named explicitly) and generated mixes.
  Nothing ran on the candidate mixes of 0065 or on the NAS root's
  broadcast file. No command was given the NAS configuration or
  library; NAS-scale runs used the NAS peak store on the local disk and
  read NAS audio only to render (`--other-root`, recall's sources).
  Nothing was written to the NAS or to `work/peaks`, and `work/reports/`
  received only `regress`'s own reports.
- Baselines stay `session-54-linux` and `session-54-linux-nas`.
- Final checks (10:01-10:11) with `work/bin/s57-final/`:
  `scripts/check.sh` green (4:29); `regress session-54-linux` (0:52)
  and `regress session-54-linux-nas` (4:00) identical, every report
  identical to its baseline's apart from `wall_seconds`;
  `docs/pair-verdicts.txt`, the answer log and the NAS clusters file at
  their start digests (`work/s57/listening-digests-end.txt`); no answer
  log written, no manifest or panel changed.

## Session 58

### What changed

The product did not change: the `gunfinger` binary is byte-identical to
`s57-final`'s. The harness gained `gunfinger-eval parts`
(`parts.rs`), which lists the other library's files whose start or end
lines up inside another recording while the rest of the file is a
different one, with every part of each, and `questions parts`
(`audition/parts.rs`), which asks of each candidate whether the file is
one recording throughout or changes record (`docs/audition-questions.md`).
The rules and thresholds were pre-registered
(`docs/research/2026-10-27-files-holding-several-recordings.md`) before
any search. Experiment 0241. Nothing was ignored, pruned or written to
the NAS.

In short: 15,627 of the NAS store's 40,678 files searched (all `[CUT]`
uploads, the whole `i-witness-dnb` channel, every release folder and
3,200 other uploads), 205 candidates, 186 worth the ear. The signal finds
transitions (cuts, mixes indexed as tracks, a disc that seems mixed), but
most strong candidates are another version of the same tune (an edit,
VIP, remix or bootleg that starts inside the original). The pack of 20
questions is ready; only the user's ear can tell the two apart.

### Experiment 0241

| Kind (a guess by folder and parts) | Candidates | Worth the ear | Edge file shares a title word (post hoc) |
|---|--:|--:|--:|
| Cut from a mix | 12 | 12 | 5 |
| A mixed CD's track | 117 | 106 | 67 |
| An upload opening with another record | 37 | 31 | 29 |
| Other (an upload ending with another record) | 39 | 37 | 27 |

- `[CUT]` uploads: 54 audio files in the channel today (the ignored
  Space Cake file, 46 with a record, 7 with a skip note). 3 of the 46
  are candidates: Spider (Optiv & BTK Remix) opens with 28 s of Nero -
  Satisfy (Audio Bootleg); Break - Wine ends with 41 s of All Thieves -
  Stars (Zero T Remix); Planet Dust (DC Breaks Remix) lines up with the
  original over 0:11-0:47, more likely shared material.
- Known cases, from saved pairs (the pruned files have no record): the
  Space Cake cut and the track-fix Drumtrip come out worth the ear; the
  T-Minus vocal mix upload does not (only its T-Minus part is in a saved
  pair), nor the Kraken (no saved pair) or the track-fix Propaganda (its
  part ends 25 s before its end, the rule allows 15 s).
- Controls: no track of Hard Leaders - Listen Up; 7 tracks of the unmixed
  discs, 5 of them Ray Keith presents Vintage Dread 2000's first disc,
  whose edge parts are other records.
- Long records among the candidates: `sety/Weekenders_1_-_Nocny_Trans.mp3`
  (12 min) and the Virus on Kiss FM show of 3 May 2000 (15 min), mixes
  indexed as tracks.

### Times on the Linux machine (32 workers)

| Run | Time | Peak memory |
|---|--:|--:|
| `regress session-54-linux`; `-nas` (start) | 0:51; 3:59 | 0.4; 5.3 GiB |
| `parts`, timing sample of 64 files (index built in about 23 s) | 1:02 | 5.0 GiB |
| `parts`, tier 1 (46 files whole) | 0:48 | 5.1 GiB |
| `parts`, tier 2 (6,293 files) | 37:11 | 6.4 GiB |
| `parts`, tier 3 (6,088 files) | 34:25 | 6.6 GiB |
| `parts`, tier 4, two chunks of 1,600 files | 9:08; 8:50 | 5.9 GiB |
| `regress session-54-linux`; `-nas` (the run's build) | 0:51; 4:02 | 0.4; 5.3 GiB |
| `scripts/check.sh`; `regress session-54-linux`; `-nas` (final) | 4:44; 0:51; 3:47 | 1.3; 0.4; 5.3 GiB |

### Decided this session, open to change

- The queries were tiers in a fixed order (the `[CUT]` files whole, then
  the first and last 60 s of `i-witness-dnb`, release folders, other
  uploads) under a 90-minute budget; tier 4 was cut after 3,200 files.
- Experiment 0065's candidate mixes were left out of the queries (the
  15 records of its two unignored mixed-CD folders); a candidate with a
  part in a 0065 folder (one) is kept out of the pack.
- The pack keeps the pre-registered rule, although 14 of its 20 are
  likely versions of one tune; the controls are left out of it.
- The name split in 0241 is post hoc and crude, for reading only.
- Three differences from the pre-registration's wording, without effect
  on this run (brief 58, 12:15): an edge part is never also a second
  part; the pack leaves out candidates with an edge or second part in a
  0065 folder; lanes go to edge and second parts only, at most eight.

### Questions this leaves for us

- Whether the signal needs a rule against versions of one tune: an edit
  or VIP that starts inside the original passes the "inside the found
  file" rule, and hits do not tell it from a transition. The pack's
  answers would show how often it is wrong.
- Whether a copy of the file itself should count as a second part. The
  NAS clusters hold only the corpus recordings' rips, so most files have
  no cluster and a full copy counts: in 41 of the 133 candidates with a
  second part it is the only one (20 are worth the ear only so).
- Whether to search the 25,036 uploads not searched (about 2.2 hours at
  the measured rate), and the 7 `[CUT]` files with a skip note (no
  record, so `parts` cannot search them).

### Questions for listening (none answered by agents)

- The pack: `work/s58/questions/06-parts-s58.json`, 20 questions, each
  "is A one recording throughout, or does it change record?" (below).
- Ray Keith presents Vintage Dread 2000's first disc: 5 tracks have an
  edge part of another record (Sing opens with Londons Most Wanted -
  Girls Dem Want It, Chopper with LTJ Bukem - Horizons, Chasing Shadows
  with Volcov - Oblique Strategies). The ignore file leaves out its second disc only;
  whether the first is mixed too is for the user. Answered on
  2026-10-30: separate tracks, as the release notes say. Its five edge
  parts are therefore false transitions of `parts` (shared material
  between separate tracks), a negative for any later rule.
- The 65 candidates worth the ear whose strongest edge part has another
  name (`work/s58/different-name-files.txt`), for a later pack.
- Pending from before: Star Trails, Into Music.

### Handoff for the next session

1. The pack, for the user (Linux, the NAS at `/mnt/atlas/Music/dnb`):

   ```sh
   work/bin/s58-final/gunfinger-eval audition work/s58/questions/06-parts-s58.json
   ```

   On the Mac, make it again with the Mac's NAS root first (the same
   question ids):

   ```sh
   gunfinger-eval questions parts work/s58/out/tier-1.json work/s58/out/tier-2.json \
     work/s58/out/tier-3.json work/s58/out/tier-4-00.json work/s58/out/tier-4-01.json \
     --files work/s58/pack-files.txt --out work/s58/questions/06-parts-s58.json
   ```

   Each answer goes to the answer log only. `several` (it changes
   record): the user may add the file to the NAS `.gunfingerignore`, by
   hand, with a comment, as for the Space Cake file; a later session
   then prunes it with the user's approval and records new NAS-scale
   baselines. `one` or `unsure`: nothing changes.
2. A later pack from the post-hoc list: the same command with `--files
   work/s58/different-name-files.txt` (65 candidates; at most 20 per
   pack by the rule of this session, so cut the list first).
3. Baselines stay `session-54-linux` and `session-54-linux-nas`; the
   store, the clusters file and the verdicts file are as at the start.

### What else to know

- Scripts: `scripts/analysis/parts_tiers.py`, `parts_known.py`,
  `parts_tally.py` (also writes 0241's data) and `store_paths.py`.
- Data: `work/s58/` (`store-paths.tsv` and `store_paths.py`, the tier
  lists and `tiers.py`, `leave-out-0065.txt`, `known.py` and
  `known-cases.txt`, `pruned-durations.tsv`, `tally.py` and `tally.txt`,
  `pack-files.txt`, `different-name-files.txt`, reports in `out/`,
  questions in `questions/`, logs in `logs/`; scripts `run.sh`,
  `same.py`, `start.sh`, `timing.sh`, `main-run.sh`, `regress-run.sh`,
  `regress-nas.sh`, `final.sh`).
- Builds: `work/bin/s58-start/` (= `s57-final`), `s58-run/` (every
  search), `s58-final/`.
- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. No mix was searched; the queries
  were library records, without 0065's candidates. The NAS was only
  read (FFprobe on the five pruned files for their lengths, the ignore
  file, the audio for the pack's check); nothing was written to the NAS
  or to `work/peaks`.
- Final checks (12:27-12:36) with `work/bin/s58-final/`:
  `scripts/check.sh` green (4:44); `regress session-54-linux` (0:51) and
  `regress session-54-linux-nas` (3:47) identical, every report equal to
  its baseline's apart from `wall_seconds`; `docs/pair-verdicts.txt`, the
  answer log and the NAS clusters file at their start digests
  (`work/s58/listening-digests-end.txt`); no answer log written, no
  manifest or panel changed.

## Session 59

### What changed

The product did not change. The core gained two functions that only the
harness calls: `Index::skipping_hashes` (set aside given lists) and
`saved::provenance` (read a saved index's header), so the CLI binary is
not byte-identical to `s58-final`'s; its JSON report of the Stakka mix
is (final checks). The harness gained `recall --set-aside-like FILE`
(`fullest::setting_aside_like`), which searches today's postings with
the lists a saved index of an earlier store sets aside and records how
the two sets differ, and `recall --trace-candidates`, whose candidates
(also `sweep`'s) now keep their speed, span and chain of lines.
`robust`'s and `grid`'s same-title branches are now small functions
(`Role::of` and `count`; `matching_the_play`) with unit tests (session
54's handoff). Experiment 0242; a dated note in ADR 0014's
consequences; the roadmap's "The filter and famous breaks".

In short: the prune moved one posting list across the cut of the fullest
1% in recall's index, at a tie, and that one list moved query 911.
Expected under ADR 0014, not a defect.

### Experiment 0242

The pre-prune saved index (2026-10-21) survives as hard links in older
work directories (`work/s36/w/indexes/library-nas-dnb-peaks.index`), so
the old postings were read in memory; nothing was written to it or to a
store.

| Recall's index, held-out files left out | Old index's lists set aside | Today's |
|---|--:|--:|
| Lists set aside; shortest | 10,863; 9,931 postings | 10,864; 9,930 |
| Clean queries unlike the old report (of 702) | 0 | 3 (session 57's moves) |
| Brief queries unlike the old report (of 1,404) | 0 | 2 (910, 911) |
| Query 911: first pass; second pass | 370 hits at +1.147%; 455 | 369 at +0.933%; 503 |

- The old index less the five pruned files has today's postings, posting
  for posting: the asset numbering and the rest of the store are not a
  cause.
- The moved list (hash 1,286,800) holds 9,930 postings in both indexes
  and none of a pruned file. By inference, a list tied with it at the
  cut lost a pruned posting, so the cut fell by one and it crossed. The
  whole index's cut does not move (1,086,463 lists, 10,865 set aside,
  longest kept 9,938, before and after), so runs of the whole index
  (the development scan) do not move; runs that leave files out (recall,
  sweeps, the leave-out scans of `regress`) have cuts of their own,
  measured this session for recall only.
- Query 911 plays between the +0.8% and +1.2% rungs. Its middle window's
  line is the +1.2% rung's (198 hits) before and the +0.8% rung's (197)
  after: `distinct` keeps the strongest view of an alignment, and by
  inference the moved list cost the +1.2% line one hit and the +0.8%
  line, tied, was kept (at a tie the lower rung). The chain is
  short, so its speed is the hit-weighted mean of its lines' rungs:
  +1.147% before, +0.933% after. The second pass measures at that speed:
  455 and 503 hits. Query 910 loses one hit the same way (386 hits at
  +1.0565% and +1.056%).

### Decided this session, open to change

- The judgment: a detection's hits and fitted speed depending on the
  rest of the library is expected under ADR 0014 (the lists set aside are
  a share of the index each run searches). No ADR changes; ADR 0014 has a
  dated note under its consequences.
- The traced runs used `--overlap-by-spans`, the rule of the old
  reports (session 33's); session 57 showed today's default differs from
  it only by ADR 0021's kept detections.
- Clean queries were not traced again with the chain's lines: their three
  moves are a refit and two weak candidates.
- The traces of other moves session 57 attributed to the prune (sweeps
  of seeds 2027-2028 at NAS scale, the CLI's re-measured drops segments,
  recall's other-rips weak candidate) were not run; by the same
  mechanism they need a list set aside by one of their indexes only.

### Questions this leaves for us

- How often a short chain's speed hinges on one hit (a play between two
  rungs whose views of a window are nearly equal), and whether fitting
  the speed in the second pass, or weighing both rungs' lines, would
  steady it. Not measured; nothing in the product depends on it.

### Questions for listening (none answered by agents)

- None new. Pending: Star Trails, Into Music, session 58's pack.

### Handoff for the next session

1. No finding. The open item of session 57 is closed by 0242; reproduce
   with `sh work/s59/trace-lines.sh` (both conditions; the pre-prune
   index is read from `work/s36/w/indexes/`; keep one of its links).
2. Baselines stay `session-54-linux` and `session-54-linux-nas`.
3. `recall --set-aside-like FILE --trace-candidates` works with any saved
   index of the harness; the report records `set_aside_like` and each
   candidate's chain.

### What else to know

- Scripts: `scripts/analysis/recall_by_position.py`, `chain_lines.py`,
  `saved_index_cut.py`. Data: `work/s59/` (logs in `logs/`, the scratch
  work directory `wrecall/` with the reports, `pruned.txt`, `cli/`;
  scripts `run.sh`, `same.py`, `start.sh`, `scratch.sh`,
  `trace-setup.sh`, `trace-like.sh`, `trace-today.sh`, `trace-lines.sh`,
  `final.sh`, `compare.py`, `chain_lines.py`, `py/cut.py`).
- Builds: `work/bin/s59-start/` (= `s58-final`), `s59-run/` (the first
  traced runs), `s59-run2/` (the line trace), `s59-final/`.
- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. No mix was searched except the
  Stakka development mix (`regress`, and the CLI's report in the final
  checks); recall searched its excerpts of library files. Nothing ran on
  the candidate mixes of 0065 or on the NAS root's broadcast file. The
  NAS was only read (recall's renders were reflink copies of earlier
  ones); nothing was written to the NAS, to `work/peaks` or to the NAS
  store.
- A read-only review by a separate agent (13:44-13:52) confirmed every
  number, the comparisons, the refactors' behaviour and the judgment, and
  found six points on wording, all fixed (brief 59, 13:55): the claim
  that `regress` is untouched (its leave-out scans and sweep leave files
  out, so they have cuts of their own, not measured), two inferences in
  the roadmap marked, the conditions of "the old reports exactly", two
  memory figures, the checklist, one sentence. Left as notes:
  `--trace-candidates`'s help text does not name the chain's lines, and
  0242 gives the method instead of a literal command (the commands are
  in `work/s59/trace-*.sh`).
- Final checks (13:43-13:53) with `work/bin/s59-final/` (the same
  binaries as `s59-run2/`): `scripts/check.sh` green (4:50); `regress
  session-54-linux` (0:52) and `regress session-54-linux-nas` (3:49)
  identical, every report equal to its baseline's apart from
  `wall_seconds`; the CLI's JSON report of the Stakka mix
  (`--saved-index off`) identical from `s58-final` and `s59-final`;
  `docs/pair-verdicts.txt`, the answer log and the NAS clusters file at
  their start digests (`work/s59/listening-digests-end.txt`); no answer
  log written, no manifest or panel changed.

## Session 60

### What changed

The product did not change: master's search is the one the session
started with, and no code was committed. Proposal 2 of the 2026-10-29
note (reject impossible alignments before scattering hits) was
pre-registered, prototyped, replayed and timed, and rejected by the
registered bar. Experiments 0243 and 0244; the roadmap's "Where search
time goes", the low-bandwidth note ("Session 60") and the proposals
note (§2) record the result.

In short: a sketch counted from every hit of a part (a row of folded
pairs per asset) never rejects an asset the pre-filter would pass, and
the lines and reports stay identical, but at sizes that fit the cache it
passes most hits, and at 1,024 slots (1 MiB a worker) it passes 21% of
them and makes the NAS-scale search 31-32% slower at 32 workers.

### The sketch

Before a part is grouped, `PartSketch::flag` counts each hit in its
asset's row of `M + 1` pair bytes, its slot folded mod `M` (`M` divides
4,096), with `OffsetSlots`' word add, wrap bytes and test; an asset is
flagged when one of its pair bytes reaches 3. `group_flagged` counts and
scatters only flagged assets' hits, in today's order; unflagged assets
have no grouped hits. It is conservative: folding only adds counts,
rows are per asset, bytes are exact until their asset is flagged and the
flag is sticky, and with inexact phases the search takes today's path
(the brief's proof obligation; property tests; a read-only review found
no input that breaks it).

### Experiments 0243 and 0244

| Stakka scan, the Linux machine | Today | F64 | F256 | F1024 |
|---|--:|--:|--:|--:|
| Assets with 3 hits flagged, NAS scale (replays) | 2.05% pass the filter | 91.3% | 54.2% | 12.4% |
| Their hits flagged, NAS scale | 4.3% | 97.1% | 70.2% | 21.3% |
| `lines` on one thread, NAS replays | 0.822 s | +33.7% | +35.2% | +13.1% |
| Search at NAS scale, 32 jobs | 23.60, 24.16 s | | | 30.85, 31.97 s |
| The same, 16 jobs | 29.20, 29.33 s | | | 32.97, 32.93 s |
| 262 tracks, 32 jobs | 6.53, 6.50 s | | | 6.74, 6.53 s |

Lines were identical, bit for bit and in order, in all 787 replayed
windows under every layout, and every Stakka report of the timed runs
equalled its baseline's. The stage timers (NAS scale, 32 jobs): the
sketch's own pass 300 s of CPU; the pre-filter 136-137 s down to 33 s;
grouping 262-265 s down to 195-196 s (it still reads every hit twice to
test its flag); taking 85-87 s up to 135 s (by inference, the 1 MiB
table evicts the lists and parts).

### Decided this session, open to change

- Three layouts were registered and replayed; only the replay winner
  (F1024, lowest replay CPU) was timed in full searches, as registered.
  No fourth size or hashed layout was tried: by inference from 0243 (the
  fewer slots, the more hits pass) and by a uniform estimate for a shared
  table, neither would both fit the cache and reject most assets.
- The equality checks registered for adoption beyond the Stakka scans
  (the prototype's `regress`, the other two development scans, the CLI's
  JSON) were not run once the speed bar failed.
- The prototype is not kept in the tree (no dead code); its diff, with
  the tests and the replay harness, is `work/s60/sketch.diff`.
- The Mac was not available: no macOS timings (the proposal asks for
  them).

### Questions this leaves for us

- Grouping's scatter remains the largest stage (40% of the first pass at
  32 workers, 0214; 40.1% again in this session's timers). Proposals that
  avoid it must avoid reading every hit a second time, which this sketch
  did not.

### Questions for listening (none answered by agents)

- None new. Pending: Star Trails, Into Music.

### Handoff for the next session

1. No change to master's search. Baselines stay `session-54-linux` and
   `session-54-linux-nas`; the start regressions with today's master
   were identical, so the restructuring of 2026-10-28 changed no report.
2. Replay data: `work/s60/replay/nas/` (80 windows, 544 MB) and
   `replay/corpus/` (707 windows, 87 MB), real windows of the Stakka scan
   in the capture format of `work/s60/capture.diff`; the replay harness
   (`replay_captured_windows`, `replay_dense_windows`, ignored tests) is
   in `work/s60/sketch.diff` and can replay any other change to `lines`.
3. Test gaps the review found in the prototype (left, since it is not
   merged): the random test's phases do not change the offsets it
   builds; the wrap test's "assets 0-6 unflagged" holds trivially; no
   test compares a flagged asset's grouped slice with `group_part`'s.

### What else to know

- Scripts (`sh`, from the repository root): `work/s60/start.sh`,
  `capture.sh`, `ab.sh TAG SKETCH`, `timers.sh`, `final.sh`; `run.sh`,
  `same.py`. Builds: `work/bin/s60-start/` (master), `s60-capture/`,
  `s60-sketch/` (`GUNFINGER_S60_SKETCH` = 0, 64, 256 or 1024 picks the
  layout; 0 is off), `s60-tbase/`, `s60-tsketch/` (timers),
  `s60-final/` (= `s60-start`). Worktrees `work/s60/wt`, `proto`,
  `tbase`, `tsketch` (detached; scratch). Scratch work directory
  `work/s60/w` (the saved NAS index hard-linked).
- Protocol slips, recorded in the brief: the pre-registration's stamp
  read 01:30 (committed 01:23:31, before any timing); the A B B A run
  was first started before 0243 was written, stopped after 13 s before
  any scan finished, and started again after 0243 was committed.
- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. Only the Stakka development mix
  was searched (and `regress`'s scans). Nothing ran on the candidate
  mixes of 0065 or on the NAS root's broadcast file. Nothing was written
  to the NAS, to `work/peaks` or to the NAS store.
- A read-only review by a separate agent (01:41-01:48) confirmed the
  proof and every number, and found wording and precision points, all
  fixed (brief 60, 01:50): the slip's description, the claim that a
  table with fewer slots than the filter cannot reject (F1024 rejects
  87.6% of the assets), the bar's four failed counts, the unrun equality checks, the
  machine and scale beside each time, "every report" where only F1024
  ran reports, two ranges (100-101 s, 67-69 s), the stage timers' runs.
- Final checks (01:40-01:50) with `work/bin/s60-final/` (built from
  master, `cmp`-equal to `s60-start`): `scripts/check.sh` green (4:51);
  `regress session-54-linux` (0:50) and `regress session-54-linux-nas`
  (3:47) identical, 8 of 8 reports equal to their baselines' apart from
  `wall_seconds`; `docs/pair-verdicts.txt`, the answer log and the NAS
  clusters file at their start digests
  (`work/s60/listening-digests-end.txt`); no answer log written, no
  manifest or panel changed.

## Session 61

### What changed

The product, its front end and every default are unchanged; both
regressions stay identical. Proposal 1 of the 2026-10-29 note (combine
stereo channels after measuring spectral power) was pre-registered
(`3220fff`), built as an opt-in harness front end and measured in two
phases; experiments 0245-0247 and the decision note
`docs/research/2026-10-30-stereo-power-decision.md`. Added, tested
(`db502a3`):

- Core: `spectrogram::Signal` (`Mono`, today's; `PowerSum { left,
  right }`), analysed by `for_each_power_frame`, with
  `peaks::extract_signal_peaks(_in)`, `Rung::signal_points` and
  `search::search_signal_with`; every existing entry point taking
  `&[f32]` wraps it in `Signal::Mono`. `Profile::POWER_SUM` (field
  `channels`) names the variant's records (`... channels=power-sum`);
  today's identifier is unchanged.
- Harness: `--front-end downmix|power-sum-queries|power-sum` (a
  `Matching` option, so its reports go to `reports/variant-<name>/`),
  taken by `robust`, `scan`, `mixes` and `variant-store`, refused with
  `--verify`, and `power-sum` refused with `--other-peaks-dir` (the
  other library's records are the downmix's); `variant-store --out DIR
  [--replace TSV]` builds a store of the library's files under the front
  end's profile, never into the library's store.
  `scripts/analysis/front_end_summary.py`.
- Tests: inverting either channel leaves the power-sum peaks unchanged
  to the bit; equal channels give their downmix's peaks; a one-channel
  file decoded by FFmpeg gives at least 99% of the downmix's peaks; the
  default path gives the peaks of fixed noise that `af32482` gave (a
  pinned digest); the option checks.

In short: power-sum queries against today's records win back every
inverted channel and every delay up to 5 ms, keep mono queries' counts
and cost 12% more search time at NAS scale; a store built
the same way also wins back faults in the rip, but mono queries against
it lose 2-4 of 80 in 6 of 38 `robust` conditions, which fails the
registered bar. Noise in one channel costs more under the variant. Not
adopted.

### The variant

`P = |X_L|^2 + |X_R|^2` with no further scaling. With FFmpeg's downmix
`M = (L + R) / sqrt(2)` and `S = (L - R) / sqrt(2)`, `P = |X_M|^2 +
|X_S|^2`: today's power plus the side's, so a mono source gives today's
power (FFmpeg's `-ac 2` gives each channel of a one-channel file the
sample at 1/sqrt(2), measured), and inverting a channel changes nothing
(to the bit, tested).

### Experiments 0245-0247 (262 tracks unless named)

| Confident | Downmix | Power-sum queries, today's records | Power sum both sides |
|---|--:|--:|--:|
| `stereo-control`, inverted, silent (of 80) | 80, 0, 78 | 80, 80, 78 | 80, 80, 80 |
| Noise 10, 0, -10 dB | 80, 29, 0 | 80, 18, 0 | 80, 23, 0 |
| Delay 0.2, 1, 5 ms | 80, 70, 37 | 80, 80, 80 | 80, 80, 80 |
| NAS scale, the same rows | as at 262 | as at 262 but noise 0 dB 19 | not run |
| Inverted half of 24 mixes (of 100 plays) | 1 | 93 | 94 |
| Rip inverted, silent, 1 ms, 5 ms (of 20) | 0, 19, 18, 4 | not measured | 20, 20, 20, 20 |
| 38 default conditions (mono renders) | | every row the same | 6 rows 2-4 lower |
| Development scans (both scales; 262 for the last column) | all tracks | all tracks, same files | all tracks, same files |

Wrong confident answers: none at 262 tracks or in any mix; at NAS scale
only session 40's known pair (the held-out Cool Rock Stuff excerpt as
the Mobb Deep / Urban Takover upload), under both front ends, in more
fault rows under the variant only because it keeps that excerpt there.
Cost (0247): the Stakka scan at NAS scale 27.6-27.7 s against 24.6-24.8
s (A B B A, 32 jobs), 13% more CPU, 2% more memory; peaks and postings
0.1% fewer; a 262-track build as fast, with 3 times the peak memory.

### Decided this session, open to change

- No normalisation: the power sum equals today's power for mono sources.
- `robust` ran its 38 default conditions only at 262 tracks; at NAS
  scale only the stereo set (time box). Shared-material panels,
  `recall`'s other-rip panels and NAS-scale mixes were not run.
- The bar's "no wrong confident answer that the downmix does not have"
  was read as "no new wrong file for any draw" (brief 61, 02:35): at NAS
  scale the known pair appears in 16 held-out queries against 7.
- Hits are reported as the variant's against the downmix's for the same
  query, not against each front end's control (a departure, recorded).
- The faulted rips are session 42's re-encoded copies, put in place of
  their files in a full 262-track store per copy and front end.

### Questions this leaves for us

- Whether to pre-register the query side alone (no rebuild; the decision
  note's recommendation), with the noise conditions, the shared
  panels, `recall`'s other rips and NAS-scale mixes, and the proposals
  note's frozen acceptance rule.
- Why mono queries lose marginal excerpts against power-sum records
  (by inference, the side's power in the records), and whether a
  different combination for records (the downmix's power where the
  channels agree) would keep both.

### Questions for listening (none answered by agents)

- None new. Pending: Star Trails, Into Music.

### Handoff for the next session

1. Builds: `work/bin/s61-start/` (= `s60-final`), `s61-run/` (the
   variant, used for every run; its code before `cargo fmt`),
   `s61-final/` (master at the end).
2. Reports (scratch work directories, never `work/reports`):
   `work/s61/w/reports/` (downmix), `.../variant-power-sum-queries/`,
   `.../variant-power-sum/`, and the same below
   `library-nas-dnb-peaks/` for NAS scale (`robust-default.json`,
   `robust-stereo.json`, `mixes-stereo-*-seed-*.json`, `scan-*.json`);
   the faulted rips' runs in `work/s61/ref/w-<copy>/`; the cost runs'
   reports in `work/s61/cost/`; summaries `work/s61/phase-a-262.json`,
   `phase-a-nas.json`, `phase-b-262.json`, `phase-b-ref.json`.
3. Stores (scratch, under `work/s61/` only): `peaks-power-sum/` and
   `peaks-downmix/` (265 records each; the second equals `work/peaks`
   byte for byte), `ref/peaks-<copy>-<front end>/` with session 42's
   faulted copies in place of 10 files (lists `ref/<copy>.tsv`).
4. Renders: `work/s61/w/robust/seed-2026/` holds the 38 default
   conditions rendered afresh (1.3 GB) beside hard links to
   session 42's stereo renders; kept for the query-side test the
   decision note recommends.
5. Scripts (`sh`, from the repository root): `work/s61/start.sh`,
   `variant.sh`, `phase-a.sh`, `phase-b.sh`, `cost.sh`, `final.sh`;
   `run.sh`, `same.py`. Commands:

   ```sh
   E=work/bin/s61-final/gunfinger-eval; W=work/s61/w; N=~/.local/share/gunfinger/nas-dnb-peaks
   $E --work $W --front-end power-sum-queries robust --seed 2026 --only stereo-control,stereo-inverted
   $E --work $W --other-peaks-dir $N --front-end power-sum-queries scan stakka-skynet-knowledge
   $E --front-end power-sum variant-store --out work/s61/peaks-power-sum
   $E --work $W --peaks-dir work/s61/peaks-power-sum --front-end power-sum mixes --seed 2026 --count 8 --stereo-fault stereo-inverted
   python3 -I scripts/analysis/front_end_summary.py OUT.json downmix=$W/reports psq=$W/reports/variant-power-sum-queries
   ```

6. Next, if we continue: the decision note's recommendation, a
   pre-registered test of power-sum queries alone at NAS scale (shared
   panels, `recall`'s other rips, NAS-scale mixes, noise conditions, the
   known pair's rise), then an ADR for the CLI's decoding if it passes.
   A two-channel decoder in the core would need the stall watch (ADR
   0018) that the harness's `decode_stereo` lacks.

### What else to know

- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. Only the three development mixes
  (named explicitly) and generated mixes were searched. Nothing ran on
  the candidate mixes of 0065 or on the NAS root's broadcast file. No NAS
  audio file was decoded; the NAS store was only searched; `work/peaks`
  only read.
- The pre-registration was committed (`3220fff`, 01:58) before any code
  or run; phase A ran from the run build at 02:08, its code committed at
  02:15 after the regressions; each experiment was committed before the
  next phase started.
- A read-only review by a separate agent (02:53-03:02) confirmed the
  default path's arithmetic, the code and the tables; its points on
  wording, risks, the rebuild estimate, the build memory, a test that
  compared the code with itself, stale doc comments and one unrefused
  option combination are fixed (brief 61, 03:05).
- Final checks (03:06-03:16) with `work/bin/s61-final/` (master at
  `81981c8`): `scripts/check.sh` green (4:45); `regress
  session-54-linux` (0:51) and `regress session-54-linux-nas` (3:46)
  identical, 8 of 8 reports equal to their baselines' apart from
  `wall_seconds`; `docs/pair-verdicts.txt`, the answer log and the NAS
  clusters file at their start digests
  (`work/s61/listening-digests-end.txt`); no answer log written, no
  manifest or panel changed.

## Session 62

### What changed

The product, its front end and every default are unchanged; both
regressions stay identical. Session 61's recommendation, power-sum
queries against today's records, was pre-registered with one frozen rule
and a six-item bar (`de3be59`) and run at NAS scale beside the downmix
on the same rendered queries; a census measured channel faults in the
NAS rips for the first time. Experiments 0248-0250; the decision note's
"Session 62". Added, harness only and tested (`fcf385d`, `911df16`,
`a40e5e1`):

- `shared` and `recall` take `--front-end` (decoding and searching
  through `Matching::decode_query`/`search_audio`; a brief play is cut
  alike from each channel, `recall::brief_audio`); the peak verifier
  runs only under the downmix front end, and `recall --trace-candidates`
  refuses another.
- `mixes` takes `--other-peaks-dir`: `mixes::sweep_index` builds through
  `Padding::index` (the held-out clusters' NAS files left out, as
  `sweep` does) and takes the loaded records; at 262 tracks its reports
  are unchanged.
- `stereo-width` measures each file's balance, uncentred channel
  correlation and strongest lag within ±80 samples, with a class (one
  channel, inverted, delayed, else its width); `--other` samples the
  other library by kind (`--exclude` a list), `--files DIR` every MP3
  under a folder.
- `scripts/analysis/stereo_census.py`, `query_side_bar.py` (the bar,
  query by query).

In short: the query side keeps every fault gain at NAS scale and adds no
wrong file, but it fails its registered bar: `recall` loses confident
identifications on wide sources, because a query picked from mid plus
side carries peaks a downmix record lacks. Channel faults are rare in
the NAS rips (about 0.4% inverted). No ADR proposal.

### Experiments 0248-0250 (NAS scale)

| Run | Downmix | Power-sum queries |
|---|--:|--:|
| Census of 2,000 rips: inverted, delayed, one channel (0248) | 5, 1, 0 | |
| Shared panels, played confident; related versions named (0249) | 269 and 208; 326 and 9 | the same; 309 and 9 |
| `recall` clean; brief (0249) | 522; 721 | 513; 663 |
| `recall` other rips: clean; brief | 174; 171 | 172; 168 |
| Mixes, inverted halves; control halves (0250) | 1/100; 95/100 | 93/100; 95/100 |
| `robust` inverted; 1, 5 ms; noise 0 dB (of 80) | 0; 70, 37; 29 | 80; 80, 80; 19 |
| Scans (tracks) | 11, 12, 17 | the same |
| Stakka scan, search (A B B A) | 24.3, 24.6 s | 27.3, 27.4 s |

The bar: 1 (no new wrong file), 3 (fault gains), 4 (related versions),
5 (noise 0 dB at least 15) and 6 (cost at most 25%) pass; 2 (no
confident identification lost on clean or mono queries, tolerance
max(1, 1%)) fails on all four `recall` sets. The nine clean losses are
one source (Organikismness - Echos, side to mid -7.3 dB); over recall's
clean excerpts the variant keeps a median 0.98 of the downmix's hits at
-20 dB or less and 0.39-0.49 above -10 dB.

### Decided this session, open to change

- The bar's tolerance for item 2 is max(1, 1% of a run's positive
  queries); the known Cool Rock pair is exempt only for its own
  held-out excerpt.
- The census's classes come from session 42's synthetic faults and were
  checked on them before the NAS sample (95 of 100 per class or more).
- `mixes` at NAS scale (a harness change not foreseen in the
  pre-registration); panels, `recall`, mixes and `robust` ran at the
  same time for the time box, and panel 2 in its own work directory;
  `recall`'s damaged sets were not run.

### Questions this leaves for us

- Whether to try a combination that equals today's power where the mid
  dominates and recovers an inverted channel where the side does (for
  example the larger of the mid's and the side's power per bin); not
  measured. It would need `recall`'s wide sources and mono queries
  first.
- How often channel faults occur in the mixes searched (not measured);
  in the rips they are about 1 in 250 inverted.

### Questions for listening (none answered by agents)

- None new. Pending: Star Trails, Into Music. Optionally: whether the
  three plainly inverted NAS rips of 0248 (Usual Suspects - Advent,
  Keltech - Axis, Plasmic - The Weapon) sound hollow in mono.

### Handoff for the next session

1. Builds: `work/bin/s62-start/` (= `s61-final`), `s62-run/` (every run;
   before the `sweep_index` refactor and the review's fixes, which leave
   reports unchanged), `s62-final/` (master at the end).
2. Reports (scratch): `work/s62/w/reports/library-nas-dnb-peaks/` (the
   downmix) and its `variant-power-sum-queries/`: panel 1
   (`shared-material-seed-2026-development.json`), `recall-seed-2026*.json`,
   `mixes-stereo-*-seed-*.json`, `robust-stereo.json`, `scan-*.json`,
   the census `stereo-width-other-seed-2026.json`; panel 2 in
   `work/s62/w2/reports/library-nas-dnb-peaks/`; the cost runs in
   `work/s62/cost/`; comparisons `work/s62/bar-*.json`; recall's widths
   `work/s62/wwidth/`; session 42's faults measured in
   `work/s62/w/reports/stereo-width-files-*.json`.
3. Scripts (`sh`, from the repository root): `work/s62/start.sh`,
   `variant.sh`, `setup.sh`, `census.sh`, `panels.sh`, `panel2.sh`,
   `recall.sh`, `mixes.sh`, `rest.sh`, `cost.sh`, `final.sh`; the
   census's exclusions `work/s62/census-exclude.txt`. Commands:

   ```sh
   E=work/bin/s62-final/gunfinger-eval; N=~/.local/share/gunfinger/nas-dnb-peaks
   $E --work work/s62/w --other-peaks-dir $N --jobs 16 stereo-width --other --seed 2026 --sample 2000 --exclude work/s62/census-exclude.txt
   $E --work work/s62/w --other-peaks-dir $N --panels work/s33/panels --front-end power-sum-queries recall --queries clean,brief
   $E --work work/s62/w --other-peaks-dir $N --panels work/s33/panels --front-end power-sum-queries shared docs/panels/shared-material-seed-2026-development.json
   $E --work work/s62/w --other-peaks-dir $N --panels work/s33/panels mixes --seed 2026 --count 8 --stereo-fault stereo-inverted
   python3 -I scripts/analysis/query_side_bar.py OUT.json DOWNMIX_DIR VARIANT_DIR
   ```

4. Renders: `work/s62/w/` holds reflink copies (7.3 GB apparent, shared
   blocks) of the robust, shared, recall and stereo-mix renders, and
   panel 2's renders (`work/s62/w2/shared/`); safe to delete.
5. Next, if we continue: the question above; otherwise the stereo line
   of work rests here (decision note, "Session 62").

### What else to know

- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. Only the three development mixes
  (named explicitly) and generated mixes were searched. The census left
  out the NAS root's Ed Rush & Optical file, 0065's too-long files and
  mixed-CD folders and the ignore file's files. The NAS was only read;
  `work/peaks` only read.
- The pre-registration was committed (`de3be59`) before any run. Slips
  and departures are in the brief: 0248 was written three seconds after
  0249's runs began; 0249's and 0250's runs overlapped; the clippy
  limit broken by the `mixes` change was caught by the final checks and
  fixed without changing any report.
- A read-only review by a separate agent (05:22-05:29) confirmed the
  default paths, the census code and the bar's reading; its numbers and
  wording points are fixed (brief 62, 05:30).
- Final checks (05:29-05:39) with `work/bin/s62-final/` (master at
  `a40e5e1`): `scripts/check.sh` green (4:37); `regress
  session-54-linux` (0:51) and `regress session-54-linux-nas` (3:46)
  identical, 8 of 8 reports equal to their baselines' apart from
  `wall_seconds`; `docs/pair-verdicts.txt`, the answer log and the NAS
  clusters file at their start digests; no answer log written, no
  manifest or panel changed.

## Session 63

### What changed

The product, the second pass's default and every rule are unchanged;
both regressions stay identical. Proposal 3 of the 2026-10-29 note was
pre-registered (`0326d32`) and measured: how much the second pass loses
by measuring each candidate at the chain's speed. Experiments 0251-0253;
the decision note `docs/research/2026-10-30-second-pass-fit-decision.md`.
Added, opt-in and tested (`383d91a`):

- `Options::second_pass_fit` in the core (`SecondPassFit`, default
  `ChainSpeed`; `search/refine/fit.rs`), set only by the harness's
  `--second-pass-fit grid|cross|iterated`: the best of 41 speeds within
  ±0.4% with the offset refitted (the grid, a reference), the same
  chosen on alternate windows and counted on the others (the split), and
  two rounds of the hits' drift within ±0.4% (the bounded fit), each for
  candidates of 80 hits or more. `refine.rs` gained a `Candidate` holding
  the inputs its measurements share.
- `scripts/analysis/second_pass_fit.py`: a fit's reports against the
  default's, query by query (`queries`, `shared`, `scans`, `moved`).

In short: the chain's speed costs hits only on short chains, where it is
the mean of the rungs (15 s plays, 0.049% off); there each fit gains
18-27 confident answers in `recall` and loses 1-3, and the split
suggests that most of the gain is not selection. The grid also lifts wrong versions; even
the split lets a related version that shares the played passage cross
240. The bounded fit fails its registered bar (4 confident answers lost;
the margin at 262 tracks 6.7% lower). No ADR proposal.

### Experiments 0251-0253 (NAS scale unless noted)

| Run | Default | Grid | Split | Bounded fit |
|---|--:|--:|--:|--:|
| `recall` brief plays: confident right (0251, 0252) | 721 | +5 -1 | +5 -1 | +5 -1 |
| `recall` brief plays, other rips only | 171 | +22 | +13 -2 | +13 -1 |
| Sweeps and `recall` clean: confident right changed (no scan track moves) | | +3 | -2 | +1 |
| Brief plays' speed error, median | 0.049% | 0.014% | 0.012% | 0.004% |
| Calibrate's strongest false, 262; NAS | 100; 282 | 110; 288 | 100; 279 | 107; 282 |
| Panels: related or other newly confident | | 27 | 16 | 0 |
| Echos (911) under today's lists; the old ones (0253) | 503; 455 | 603; 602 | | 602; 602 |
| Stakka scan's search time (alone) | 24.2 s | 37.3 s | 37.3 s | 24.6 s |

The bounded fit against the ADR bar: B1 (no new wrong or related
confident), B3 (19 gained against the split's 18), B5 (+1.6%) and B6
(fewer moved queries under the perturbation) pass; B2 (4 confident right
answers lost) and B4 at 262 tracks (the Clockwork remix's shared passage
100 to 107 hits, margin 6.90× to 6.44×) fail.

### Decided this session, open to change

- The fits live in the core's options, as `--speed-per-stretch` does,
  because the second pass's measurements are private to `refine.rs`;
  only the harness sets them.
- The floor of 80 hits (a third of the rule) for every fit; the largest
  grid-to-default ratio seen was 2.13.
- The default's reports at NAS scale are session 62's downmix runs and
  the baselines; the default's `sweep --other-rips` (with and without one
  extra rung pair) ran this session.
- The experiments were renumbered so that each file preceded the next
  one's runs: 0252 the bounded fit, 0253 the perturbation (with the
  bounded fit in it, for B6). Panel 2 ran beside panel 1 for the time
  box. The cost runs followed 0253 and are recorded in the decision note.
- The fits stay in the tree as the requested diagnostic, the reference
  for a short-chain fit; delete them when that is decided or dropped.
- The weak-true set in the script is every right detection of 80-719
  hits, wider than registered (brief 63, 09:10); bar (b) fails either
  way.
- One run beyond the registration: `sweep --other-rips --extra-rungs 1`
  under each fit, for 0226's Siren Bass case (today's ladder does not
  lose it).

### Questions this leaves for us

- Whether to measure the bounded fit only where `chain_speed` falls back
  to the mean of rungs (short chains), beside the minimum-span question;
  the decision note's recommendation.
- Whether a related version that shares the played passage and crosses
  240 only once its alignment is right (Phantom 2018 in panel 2, Sonar's
  in panel 1) should count against such a fit as a new wrong answer, as
  the registration did.

### Questions for listening (none answered by agents)

- None new. Pending: Star Trails, Into Music; optionally session 62's
  three inverted NAS rips.

### Handoff for the next session

1. Builds: `work/bin/s63-start/` (= `s62-final`), `s63-run/` (every
   run), `s63-final/` (master at the end).
2. Reports (scratch, NAS scale under `reports/library-nas-dnb-peaks/`):
   the grid in `work/s63/wg` (standard protocol, other rips, scans,
   `recall`) and `wpg`, `wpg2` (panels 1 and 2); the split in `wc`,
   `wpc`, `wpc2`; the bounded fit in `wi`, `wir`, `wir2`, `wpi1`,
   `wpi2`; the default's other rips in `wd`; 262 tracks in `w262`; the
   perturbation in `wlike` (default), `wlike2` (grid), `wlike3` (bounded
   fit); the cost runs in `work/s63/cost-*`. Summaries
   `work/s63/summary/` (copied to `docs/experiments/data/0251-*` to
   `0253-*`).
3. Scripts (`sh`, from the repository root): `work/s63/setup.sh`
   (scratch directories; `DIRS=...` for others), `start.sh`,
   `run-build.sh`, `q-grid.sh`, `q-cross.sh`, `p-*.sh`, `p2-*.sh`,
   `q-262.sh`, `q-siren.sh`, `q-iterated.sh`, `r-iterated.sh`,
   `r2-iterated.sh`, `p-iterated.sh`, `like.sh`, `cost.sh`,
   `summary-0251.sh`, `sum-iterated.sh`, `sum-like.sh`, `table.py`,
   `final.sh`. For example:

   ```sh
   E=work/bin/s63-final/gunfinger-eval; N=~/.local/share/gunfinger/nas-dnb-peaks
   $E --work work/s63/wi --other-peaks-dir $N --panels work/s33/panels --second-pass-fit iterated recall --queries clean,brief
   $E --work work/s63/wg --other-peaks-dir $N --panels work/s33/panels --second-pass-fit grid regress session-54-linux-nas
   python3 -I scripts/analysis/second_pass_fit.py queries DEFAULT.json VARIANT.json
   python3 -I scripts/analysis/second_pass_fit.py moved TODAY.json LIKE.json 910 911
   ```

4. Renders: the scratch directories hold reflink copies of the sweep,
   `recall` and panel renders (shared blocks); safe to delete.

### What else to know

- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. Only the three development mixes
  (named explicitly) were scanned; nothing ran on 0065's candidate mixes
  or the NAS root's broadcast file. The NAS was only read; `work/peaks`
  only read; the pre-prune saved index (`work/s36/w/indexes/`) only read.
- Under heavy load (five queues at once) the grid and the split made a
  `regress session-54-linux-nas` take 17 minutes and panel 1 85 minutes;
  the bounded fit costs about what the default does.
- A read-only review by a separate agent (08:51-09:07) confirmed the
  default path and the fits against the pre-registration; it found a
  counting error in `second_pass_fit.py shared` (panel 2's played
  detections) and several overstatements, fixed (brief 63, 09:10), and
  code points left for later (the same entry).
- Final checks with `work/bin/s63-final/` (08:52-09:02, at `f4532c9`;
  the Rust code last changed in `383d91a`): `scripts/check.sh` green
  (5:00); both regressions identical (0:53; 3:49); the verdicts file,
  the NAS clusters file and the answer log at their start digests; the
  CLI's JSON report of the Stakka mix identical from `s63-start` and
  `s63-final`. The review's fixes after them touch only documents and a
  Python script.

## Session 64

### What changed

The product did not change: the `gunfinger` binary is byte-identical to
`s63-final`'s. With the user's approval of 2026-10-30, the NAS store
lost the peak records and tag notes of the four uploads that the NAS
`.gunfingerignore` has left out since that evening, each heard to hold
more than one recording (session 58's pack): the i-witness-dnb uploads
`Bad Company - Spider (Optiv & BTK Remix) [CUT].m4a`, `Break - Wine
[CUT].m4a` and `500 Mills (Counterstrike) - Vaccine.m4a`, and the
3zb-gangstep-dnb upload `OMR - Bad Vibes.opus`. Store-only runs and the
harness's `--other-peaks-dir` no longer search them. One harness
addition: `sweep --set-aside-like FILE`, as `recall`'s (session 59),
opt-in and tested, to trace the changes that lists crossing the
set-aside cut cause.

In short: the prune changed nothing that is scored. Nine weak
candidates of the pruned files (6-8 hits) and one clusters pair entry
are gone. Seven other sweep queries changed in weak candidates or, by about 1e-6
percentage points, in a speed, because lists crossed the set-aside cut (0242's
mechanism): under the old index's lists all 720 sweep queries equal the
old baseline less the pruned files' candidates. The Stakka scan's three
Clockwork detections moved in speed by less than 1e-5 percentage
points, by inference for the same reason (the scan was not run under
the old lists). `session-64-linux-nas` is the NAS-scale baseline from
now on.

### Experiment 0254

| | Before (session 54's baseline) | After |
|---|--:|--:|
| NAS store records, revision | 40,678, `1fc6fe50293ad330` | 40,674, `7210fc479da58376` |
| Sweep 2026: indexed assets; queries with identical detections | 40,575 | 40,571; 707 of 720 |
| Stakka scan; leave-outs 3, 11 | 11/11; 8/11; 0/11, 0 wrong | the same; 1 weak detection fewer each |
| Calibrate: weakest identifying, strongest false, false accepted; at 30+ hits | 704, 282, 7; 95 | the same |
| Clusters; pair entries; file | 284; 2,876; `ce293fe6165e...` | 284; 2,875; `c4d3da2e7e37...` |

- Of the pruned files: Bulletproof - Scarred at -5% (Spider), Bad
  Company - Pulse at 0% (Bad Vibes) and Item.Code - Magnetics at four
  speeds (Break - Wine) each lose one candidate; each Stakka scan loses
  the 500 Mills upload at 5:36-5:46 (7 hits). The pair entry is Item.Code
  - Magnetics against Break - Wine (9 hits, 20% coverage).
- Through the lists set aside (traced for the sweep): in Decoy at -1%
  and Konflict - RoadBlock at +1% one weak candidate gains or loses a
  hit; in Kemal - Animation at +8% one weak candidate moves from +4.8% to
  +4.0% and two new ones appear (6 and 8 hits); Recharger's fitted speed
  moves at four speeds (hits unchanged). In the sweep's index a list of
  9,919 postings lost 48 of the four files and left the set aside, and
  one of 9,911 entered. By inference, the Stakka scan's three Clockwork
  detections (hits unchanged) moved in speed the same way: in the whole
  index two lists left the set aside (9,939 and 9,940 to 9,938
  postings), and the first holds 11-14 postings of each Clockwork file.

### The evening's answers, carried

- The four same-title candidates are different titles: closed in
  session 54's notes and the roadmap; `docs/calibration.md` says that
  the strongest false (282, Cool Rock) is a confirmed different version.
  `docs/same-titles.txt` does not change.
- Vintage Dread 2000's disc 1 is separate tracks: session 58's notes
  record its five edge parts as false transitions of `parts` (shared
  material between separate tracks), a negative for any later rule.
- Three inverted rips confirmed by ear (Advent, Keltech - Axis,
  Plasmic - The Weapon; We - Hello World unsure): one sentence each in
  the roadmap's "Stereo faults before the downmix" and the stereo
  decision note.

### Times on the Linux machine (32 workers)

| Run | Time | Peak memory |
|---|--:|--:|
| `regress session-54-linux` (start; step 4) | 0:52; 0:51 | 0.4 GiB |
| `regress session-54-linux-nas` (start) | 3:53 | 5.4 GiB |
| `doctor` with the NAS configuration (before, beside the NAS regression; after) | 6:17; 0:04 | |
| `prune`; `prune --yes` | 0:04; 0:04 | |
| `clusters --from-peaks --reuse-pairs` | 0.2 s | |
| `regress session-54-linux-nas` on the pruned store (the saved index built again) | 5:41 | 5.4 GiB |
| `sweep --set-aside-like` (both indexes in memory) | 2:38 | 8.6 GiB |
| `scripts/check.sh` (step 4; final) | 4:55; 4:50 | |
| `regress session-54-linux-nas`, step 4's build, the saved index loaded | 3:46 | 5.2 GiB |
| `sweep --set-aside-like`, final build | 2:27 | 8.6 GiB |
| `regress session-54-linux`; `regress session-64-linux-nas` (final) | 0:52; 3:47 | 0.4; 5.3 GiB |

### Decided this session, open to change

- `doctor` and `prune` without `--yes` ran beside the start regressions
  (all only read; no timing compared), as in session 52.
- `--set-aside-like` was added to `sweep` only. The Stakka scan's
  Clockwork speeds are explained by inference from the whole index's
  moved lists, and the leave-out scans change only by the pruned
  detection. `sweep --set-aside-like` refuses any matching option other
  than the default, as `--rank-fullest-by` does, which is stricter than
  `recall`'s refusal (`--single-pass`, `--drop-fullest`).
- `baseline session-64-linux-nas` copies the reports of the final
  build's `regress session-54-linux-nas`, identical to step 3's.

### Questions this leaves for us

- None new from the prune. From session 58: whether `parts` needs a
  rule against versions of one tune, now with Vintage Dread 2000's five
  edge parts as known false transitions.

### Questions for listening (none answered by agents)

- None new. Pending: the pitch rides (Star Trails in the Stakka mix,
  Into Music in the 2003 mix) and the Planet Dust (DC Breaks Remix) cut
  (questions 5, 6 and 8 of `work/listening-2026-10-30/questions/07-pack-2026-10-30.json`,
  unanswered); the 65 candidates in `work/s58/different-name-files.txt`.

### Handoff for the next session

1. Baselines: `session-54-linux` (262 tracks, unchanged) and
   `session-64-linux-nas` (NAS scale, the pruned store); compare
   NAS-scale runs with `session-64-linux-nas`, not `session-54-linux-nas`.
2. The NAS store: 40,674 records, revision `7210fc479da58376`; no
   records of ignored files. The harness's saved index
   `work/indexes/library-nas-dnb-peaks.index` is current for it; the
   CLI's in `~/.cache/gunfinger/indexes/` is built again by the next
   `identify` with the NAS configuration. The pre-prune index (40,678
   records) survives as session 63's hard links (`work/s63/*/indexes/`)
   for `--set-aside-like`.
3. The NAS clusters file `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`:
   SHA-256 `c4d3da2e7e3720cbf4e79e4f7cf86d71493fc6349af639b323d814656a8b9e4f`,
   284 clusters, 2,875 pair entries.
4. Builds: `work/bin/s64-start/` (= `s63-final`; the prune and step 3),
   `s64-run/` (the first `--set-aside-like` sweep), `s64-step4/` (step
   4's regressions and the baseline), `s64-final/` (master; the final
   checks and the second `--set-aside-like` sweep). The `gunfinger`
   binary is the same in all four.
5. Data: `work/s64/` (listings and logs in `logs/`, `where-before.tsv`,
   `diff-pruned.txt`, `pruned-reports/`, the clusters file before and
   after, the store's revisions, the sweep under the old lists in
   `wlike/`); scripts `run.sh`, `start.sh`, `step1-list.sh`, `step2.sh`,
   `step3.sh`, `like.sh`, `step4.sh`, `final.sh`, `where.py`, `diff.py`,
   `same.py`, `cut_moved.py`, `list_holders.py`, `like_check.py`,
   `data.py`; the second sweep under the old lists in `wlike2/`.
6. Reports outside the baseline predate this prune and name some of the
   four files: in `work/reports/library-nas-dnb-peaks/`, the two
   damaged-brief `recall` reports of seed 2026 (one weak candidate each,
   6 hits) and `pairs-s40-pack.json` (a pair with Break - Wine, so `pair`
   on that list stops at a pruned file); session 58's `parts` reports in
   `work/s58/out/`. They change when run again. The scripts match the
   Bad Vibes upload by its folder, since `unxplored-dnb/` holds another
   copy, which is not ignored.

### What else to know

- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. The only mix searched was the
  Stakka development mix. Nothing ran on the candidate mixes of 0065 or
  on the NAS root's broadcast file. The NAS was only read; `prune --yes`
  ran once with the NAS configuration, after the listing of exactly the
  four files; `work/peaks` was only read.
- `docs/pair-verdicts.txt` and the answer log are at their start digests
  (`work/s64/listening-digests-start.txt`); no answer log was written; no
  manifest, committed panel or `docs/same-titles.txt` changed. No verdict
  line names any of the four files. The clusters file changed only by
  the dropped pair entry.
- A read-only review by a separate agent (21:31-21:38) confirmed every
  number and the rules; it found the experiment two lines too long, the
  scan's cause stated as fact where it is inferred, three sweep changes
  described too loosely, an unsaved check and errors in the brief's
  account of the first step 4 queue. All were fixed (brief 64, 21:38).
- Final checks with `work/bin/s64-final/` (21:39-21:51, at the review's
  fixes): `scripts/check.sh` green (4:50); `regress session-54-linux`
  (0:52) and `regress session-64-linux-nas` (3:47) identical, every
  report equal to its baseline's apart from `wall_seconds`; the sweep
  under the old lists the same with this build; `docs/pair-verdicts.txt`
  and the answer log at their start digests, the clusters file
  `c4d3da2e7e37...` (`work/s64/listening-digests-end.txt`). The baseline
  `session-64-linux-nas` was saved again with this build (identical).

## Session 65

### What changed

The product did not change: the `gunfinger` binary is byte-identical to
`s64-final`'s. The harness gained proposal 4's pilot, opt-in:
`mixes --kind drops --residual [--residual-over F] [--residual-timing
truth|detected]`. Every play of an indexed record that the search of the
whole mix finds confidently is an A; A's audible span is cut from the
decoded mix and searched four ways: plain (P), with query peaks picked
in an 8 x 8 neighbourhood (D, denser, no masking), masked (M), and, for
A of a quiet or brief block, masked with the seed's quiet and brief
partners' recordings left out of the index (MH). The mask
(`crates/gunfinger-eval/src/residual.rs`) renders A as the generator
mixed it (`play_audio`) from a time map, decodes it at 8 kHz through the
same FFmpeg path as the mix, fits A's gain per third-octave band and
block of 63 frames (the 30th percentile of mixture over reference power
at A's stronger points), removes `over` times A's estimated power from
each bin (to silence where A explains it all), and resynthesises the
excerpt by overlap-add, because the core analyses the query again on
every rung. A's reference is another indexed file of A's cluster when
its stored peaks align with the played file's (a corpus rip, or at NAS
scale a NAS file read from the store), else the played file. The drops
kind now draws its records with the corpus clusters at every scale (it
could not draw at NAS scale before; nothing changes at 262 tracks).
Default reports do not change (the regressions: "Final checks").

In short: with the generator's time map and A's own audio, removing A
recovers nearly every quiet and brief partner, mostly confidently, with
no confident answer that is not playing (262 tracks, development and
test seeds). Denser query peaks without a mask change little (39 and
40 confirmed against plain 41 and 36 at 262 tracks). With
another rip of A as the reference (the case of a real mix), at NAS
scale, the mask leaves A confident in about a quarter of the cases;
the partners confirmed rise less (50 and 55 of 64 against plain 40 and
35), the confident ones still clearly (32 and 34 against 8 and 12). On
a test seed the residual made one confident answer that is not playing:
a known related version of the record under the masked one. Of the
partners newly confirmed, a third to two thirds have a detection in the
plain excerpt (6 of 21, 16 of 28, 5 of 12, 14 of 20; the NAS runs search
the same mixes as the 262-track ones).

### Experiments 0255-0258

Quiet and brief partners (64 per pair of seeds), confirmed (possible or
confident) / confident:

| Run | Reference: played, other rip | P | D | M |
|---|---|--:|--:|--:|
| 0255, 262 tracks, development (2026, 2028), `over` 2 | 62, 2 | 41 / 8 | 39 / 6 | 62 / 47 |
| 0255, the same, `over` 1 | 62, 2 | 41 / 8 | 39 / 6 | 55 / 28 |
| 0256, 262 tracks, test (2036, 2039) | 60, 4 | 36 / 12 | 40 / 8 | 64 / 48 |
| 0257, NAS scale, development | 11, 53 | 40 / 8 | 37 / 6 | 50 / 32 |
| 0258, NAS scale, test | 6, 58 | 35 / 12 | 42 / 8 | 55 / 34 |

- Wrong confident: 0 in every M and MH search at 262 tracks and in
  0257. In 0258, M 2 (MH 1), both the Mobb Deep Vs Urban Takover - Cool
  Rock upload (the Cool Rock pair, by ear a different title): once it
  survives the mask of Cool Rock Stuff (also confident in P), once the
  residual makes it, Cool Rock Stuff playing under a masked Stakka &
  Skynet track (265 hits, below confident in P). P and D at NAS scale:
  2 each on both pairs of seeds (0257: the Kemal - Animation remix
  upload over the original, 0182's known related version, gone in M;
  0258: the Cool Rock upload, and Paragliders - Share Of Bitterness
  (Konflict Remix) over Konflict - Pendulum, 424 hits in P and 414 in
  the whole-mix search).
  M's wrong possibles: 0-10 against P's 181-342 (the shared-material
  lines under A go with A).
- What the mask leaves of A (confident after masking): from the played
  file 0 in every run (309, 317, 37, 32 cases); from a corpus rip
  15-19% (20 of 132 in 0255 and 0257, 24 of 128 in 0256 and 0258), from
  a NAS file a third (95 of 271 in 0257, 95 of 285 in 0258).
- Bs newly confirmed with a detection in the plain excerpt: 6 of 21
  (0255), 16 of 28 (0256), 5 of 12 (0257), 14 of 20 (0258).
- The pre-registration's reading: `over` 2 was frozen on the
  development seeds; estimated timing needed half of the newly
  confirmed Bs to have a plain detection there (6 of 21), so it was not
  tried.

### Times on the Linux machine (32 workers)

| Run | Time | Peak memory |
|---|--:|--:|
| `regress session-54-linux`; `regress session-64-linux-nas` (start) | 0:50; 3:45 | 0.4; 5.6 GB |
| `mixes --kind drops --residual`, 16 mixes, 262 tracks, cached renders (0255; about 220 cases of 3-4 searches each) | 2:38-2:44 | 4.2-4.4 GB |
| The same with the renders (0256) | 3:03-3:29 | 7.1 GB |
| The same at NAS scale (0257, 0258) | 11:05-12:19 | 14.4 GB |
| Panels of six seeds (`--count 0`) | 0:08-0:13 each | 0.5 GB |
| `scripts/check.sh` (after the harness) | 5:08 | |

Session 39 searched the same 16 mixes in about 1:00 at 262 tracks with
the harness (0184) and 3:33-3:38 at NAS scale with the CLI (0182).

### Final checks

`work/s65/final.sh` with `work/bin/s65-final/` (master after the
review's fixes): `scripts/check.sh` green (4:48, 671 tests); `gunfinger`
byte-identical to `s64-final`'s; `regress session-54-linux` (0:52) and
`regress session-64-linux-nas` (3:48) identical, every report equal to
its baseline's apart from `wall_seconds`; `docs/pair-verdicts.txt`, the
NAS clusters file (`c4d3da2e7e37...`) and the answer log at their start
digests (`work/s65/listening-digests-end.txt`). A first run of the
checks after the review failed only in the load-sensitive
`decode::watch` test ("Text file busy") and passed on the idle machine.

### Decided this session, open to change

- The residual is resynthesised audio searched by the unchanged search,
  not peaks handed to it: the core analyses each turntable rung again
  from audio. The harness takes `realfft` (the core's FFT crate) for it.
- Before any masked search the pre-registered floor of -30 dB was
  dropped: the synthetic test showed that a share kept keeps A's pattern
  of peaks wherever nothing else plays near it. A bin A explains entirely
  falls to silence.
- The rule frozen on the development seeds: `over` 2 (against 1).
- Estimated timing was not tried: the registered condition (half of the
  newly confirmed Bs with a plain detection, development seeds) was not
  met (6 of 21). It is implemented (`--residual-timing detected`, A's
  strongest confident detection's line, no fades or bass cuts) but has
  never run.
- NAS scale was added on both pairs of seeds (0257, 0258), since time
  allowed; for it the drops kind draws with the corpus clusters at every
  scale, and the residual's rip may be a NAS file.
- Test seeds 2036 and 2039: the first two from 2036 up whose panel keeps
  both Clockwork records indexed (session 48 used 2032 and 2034).
- Shared breaks and broadcast processing were not built (the drops kind
  has neither); printed-tracklist recall was not measured.

### Questions this leaves for us

- Why another rip leaves A in the residual (confident after masking in
  15-19% of the corpus rips' cases and a third of the NAS files'): a
  different master or a lossy upload, the render's seek by timestamps
  against the stored peaks' timeline (0127), or the rip's alignment.
  The reports hold each case's reference; the alignment's hits and
  speed are not stored.
- Whether confirmation should need evidence in the original mixture
  (about half of the newly confirmed partners have a plain detection),
  and how a rule keeps a related version of B out (0258's Cool Rock
  case) when A's removal strengthens both.

### Questions for listening (none answered by agents)

- Paragliders - Share Of Bitterness (Konflict Remix) (`1998 - WAQT004 -
  Paragliders - Share Of Bitterness - 192/2. ...`, NAS) confident at 414 hits over
  Konflict - Pendulum (`extra/2. Konflict - Pendulum.mp3`) in the
  generated mix 2036/mix-03 at 29:41-31:29, in the plain search at NAS
  scale: does the Paragliders file share material with Pendulum?
- Pending from before: the pitch rides (Star Trails, Into Music) and the
  Planet Dust (DC Breaks Remix) cut (questions 5, 6 and 8 of
  `work/listening-2026-10-30/questions/07-pack-2026-10-30.json`); the 65
  candidates in `work/s58/different-name-files.txt`.

### Handoff for the next session

1. Baselines unchanged: `session-54-linux` (262 tracks) and
   `session-64-linux-nas` (NAS scale). The NAS store: 40,674 records,
   revision `7210fc479da58376`; the NAS clusters file
   `c4d3da2e7e3720cbf4e79e4f7cf86d71493fc6349af639b323d814656a8b9e4f`,
   unchanged.
2. Builds: `work/bin/s65-start/` (= `s64-final`), `s65-run/` (0255,
   0256), `s65-run2/` (0257), `s65-run3/` (0258), `s65-final/` (master;
   the final checks). The `gunfinger` binary is the same in all.
3. Data: `work/s65/` (logs in `logs/`; scripts `run.sh`, `start.sh`,
   `smoke.sh`, `dev.sh`, `panels.sh`, `test.sh`, `nas.sh`,
   `nas-test.sh`, `final.sh`, `same.py`); the reports in
   `work/s65/wd/reports/ladder-turntable/` (development, 262 tracks),
   `work/s65/wt/...` (test seeds and their renders in `wt/mixes/`) and
   `work/s65/wn/reports/library-nas-dnb-peaks/ladder-turntable/` (NAS
   scale); panels of seeds 2030-2041 in `work/s65/panels`. Summaries:
   `python3 -I scripts/analysis/residual_pilot.py REPORT...`.
4. The next step for proposal 4, if taken: the other-rip residue first
   (above), then `--residual-timing detected`, then an evidence rule;
   each with its own pre-registration. Test seeds 2036 and 2039 are used
   now; 2040 and 2041 also keep both Clockwork records indexed.

## Session 66

### What changed

The product did not change: the `gunfinger` binary is byte-identical to
`s65-final`'s. The harness gained proposal 5's diagnostic, opt-in:
`shared --harmonic`. Each detection of 20 hits or more is measured at
the line through its ends: its file is decoded as indexed and rendered
there (`harmonic::reference`), the render and the query are each split
into a harmonic and a percussive part by median filtering (FitzGerald
2010, §2: 25 frames across time, 17 bins across frequency, soft mask
with power 2, resynthesised by session 65's overlap-add), and the share
of the reference's peaks found in the query (±1 bin, ±1 frame) is
counted for the harmonic part (H), the percussive part (P) and the whole
spectrum (D, the comparison arm), all with the core's picker at 8 x 8,
against the verifier's four chance shifts. With the option the
scenarios also get `killed` (a 24 dB an octave bass kill below 250 Hz)
and `damaged` (robust's combined damage at 3% fast) queries, and the
report is `<plan>-harmonic.json`. Default reports, renders and
regressions do not change. `scripts/analysis/harmonic_views.py` scores
reports as the pre-registration says.

In short: the harmonic view does not tell true plays from related
versions better than denser peaks without separation. At equal hits its
AUC is lower than the denser arm's on the development families (0.579
against 0.684) and on the frozen ones (0.667 against 0.724); above 240
hits every view separates (0.86-0.95). Below 240 hits the gap comes from
the blend partner's cases: split by role (post hoc), the played file's
cases score at least as well under H as under D in every band, the
partner's far worse (masking by the record it plays with, an
inference). The bass kill did not read as a contradiction. Not
proposed (`docs/research/2026-10-31-harmonic-view-decision.md`).

### Experiments 0259-0261

Hit-matched true plays retained at the development part's rate-0
threshold (T0), negatives kept, AUC, NAS scale:

| Part | Cases (+ at equal hits / -) | H kept; negatives; AUC | D kept; negatives; AUC | P AUC | Hits AUC | Verifier AUC |
|---|---|---|---|--:|--:|--:|
| Development, 8 families (0259) | 870 / 2,389 | 216; 0; 0.579 | 185; 0; 0.684 | 0.715 | 0.591 | 0.648 |
| Frozen, 7 families (0260) | 944 / 1,156 | 483; 6; 0.667 | 474; 4; 0.724 | 0.752 | 0.738 | 0.739 |

- By hit band (development; frozen), H against D: 20-59 hits 0.257 and
  0.468; 0.602 and 0.673. 60-239: 0.403 and 0.591; 0.184 and 0.414.
  240+: 0.913 and 0.882; 0.896 and 0.903.
- Reading rules: (1) on development, H at least 10 points above D at T0
  and AUC 0.05 above: not met (+3.6 points, -0.105); (2) on frozen, H's
  wrong rate at most D's: not met (0.5% against 0.3%); (3) the bass
  kill: met (H kept 32 of 46 and 43 of 49 killed plays, D 26 and 37).
- The Cool Rock pair (frozen): 105 negative cases, 80 confident, up to
  1,288 hits; kept at T0 by H 6 (two detections, of the 20 s and 30 s
  alone queries, each under three indexes), D 4 (two under two), P 0,
  the verifier 9, hits 68; its 75 hit-matched true plays, H 12, D 6.
  Rule (2) fails by case, as registered; by detection H and D tie.
- At 262 tracks (beside): the only indexed related pairs are Wikki
  Wikki ~ Defjammer (development) and Clockwork (frozen); the
  hit-matched true plays are mostly partner cases (development 14 of 15,
  frozen 14 of 25, the rest played-file break and damaged cases), and
  every measure scores them below the related files (frozen AUC H 0.114,
  D 0.120).
- The near-identical marker (D at least the other rips' 5th percentile)
  marked nearly every negative (2,332 of 2,389), so it separated
  nothing.
- Cost (0261): at most about 3.2 s of CPU per case, measured beside
  other runs (0.4 s of it the reference's decode, timed once by hand); peak memory 4.6 GiB at 262 tracks (0.62 without), 8.6-9.8
  GiB per NAS process.

### Times on the Linux machine (32 workers)

| Run | Time | Peak memory |
|---|--:|--:|
| `regress session-54-linux`; `regress session-64-linux-nas` (start) | 0:51; 3:48 | 0.4; 5.4 GiB |
| The same with the harness build | 1:00; 4:15 | 0.4; 5.3 GiB |
| `shared --harmonic`, development, NAS scale, four processes at once | 24:06-34:28 | 8.6-9.8 GiB each |
| The same, frozen | 7:44-16:36 | 7.2-8.7 GiB each |
| `shared --harmonic`, development; frozen, 262 tracks | 5:38; 7:42 | 4.6; 3.7 GiB |
| `shared`, development, 262 tracks, no view | 2:48 | 0.62 GiB |
| `scripts/check.sh` (beside the development run) | 5:47 | |

### Final checks

`work/s66/final.sh` with `work/bin/s66-final/` (master after the
review's fixes, `0757471`): `scripts/check.sh` green (4:55, 679 tests);
`gunfinger` byte-identical to `s65-final`'s; `regress session-54-linux`
(0:51) and `regress session-64-linux-nas` (3:54) identical, every
report equal to its baseline's apart from `wall_seconds`;
`docs/pair-verdicts.txt`, the NAS clusters file (`c4d3da2e7e37...`) and
the answer log at their start digests
(`work/s66/listening-digests-end.txt`).

### Decided this session, open to change

- The separation's lengths (25 frames, 17 bins), the soft mask, the 8 x 8
  picker for every view, the ±1 bin, ±1 frame tolerance and the 20-hit
  minimum were fixed before any data (brief 66).
- The family split: development dozer, fire, pyro, sonar, phantom-force,
  we-r-the-ones, tower-bass, wikki-wikki; frozen playground-shades,
  watermelon, star-trails, cool-rock, stalker, konflict, clockwork.
  Clockwork's family is in the first panel's validation half; none of
  that half's scenarios was used, its scenario came from `pair`, and it
  was used once, frozen, as calibrate's strongest false at 262 tracks.
- The plan names Defjammer's corpus copy, under which it is indexed
  (found by the smoke run, before any development run).
- The NAS runs were split into four processes over parts of each plan;
  each scenario is searched on its own indexes, so no result changes.
- Not proposed: no rule, no ADR. The harness option stays.

### Questions this leaves for us

- Whether the related files' sustained material in the shared passages
  is the played recording's (the scores suggest so where H stays high
  for a related file); listening to a few of the strongest negatives
  (Cool Rock's 20 s and 30 s alone queries among them) would say.
- Whether a view that removes the louder record first (session 65's
  residual) would give a blend partner's harmonic part back; not tried.

### Questions for listening (none answered by agents)

- None new beyond the above. Pending from before: the pitch rides (Star
  Trails, Into Music), the Planet Dust (DC Breaks Remix) cut, the
  Paragliders file against Konflict - Pendulum (session 65), the 65
  candidates in `work/s58/different-name-files.txt`.

### Handoff for the next session

1. Baselines unchanged: `session-54-linux` (262 tracks) and
   `session-64-linux-nas` (NAS scale). The NAS store: 40,674 records,
   revision `7210fc479da58376`; the NAS clusters file
   `c4d3da2e7e3720cbf4e79e4f7cf86d71493fc6349af639b323d814656a8b9e4f`,
   unchanged.
2. Builds: `work/bin/s66-start/` (= `s65-final`), `s66-run/` (the
   harness; every run), `s66-final/` (master; the final checks). The
   `gunfinger` binary is the same in all.
3. Data: `work/s66/` (logs in `logs/`; scripts `run.sh`, `start.sh`,
   `pair.sh`, `dev.sh`, `dev262.sh`, `frozen.sh`, `cost.sh`,
   `regress.sh`, `final.sh`, `same.py`; plans in `plans/`); reports in
   `work/s66/wn/reports/library-nas-dnb-peaks/` (NAS scale, the parts
   `shared-dev-[abcd]-harmonic.json` and `shared-frozen-[abcd]-harmonic.json`)
   and `work/s66/wc/reports/` (262 tracks); summaries with
   `python3 -I scripts/analysis/harmonic_views.py develop|frozen ...`.
4. If shared material is taken up again: above 240 hits the existing
   verifier separates as well as the new views without reference audio;
   below 240 none helps. The percussive part (beside) kept the fewest
   killed true plays. Real mixes' true plays remain the binding control
   (0116, 0118).

## Session 67

### What changed

The product did not change: no threshold, default or output moved, and
both regressions stay identical. The harness gained a polarity check,
opt-in and tested: `stereo::inverted` (both channels within 20 dB,
correlation at lag 0 of -0.5 or less, strongest there within ±80
samples), `variant-store --repair-inverted` (a flagged file's peaks are
picked from (L - R) / sqrt(2) of its two-channel decode; every other
file goes through the core's `decode`, so its record is byte-identical;
the report lists the repaired files), and `stereo-width`'s per-file
`repaired`, `windows` and `inverted_windows` (30 s windows at lag 0).
`scripts/analysis/polarity_census.py` summarises a census. Without the
option, `variant-store`'s report gains an empty `repaired` list and
`stereo-width` its new fields and one more printed line; `regress` is
unaffected. The rule,
the repair, the census, the recovery test, the controls and the bar
were pre-registered before any search (`faeadee`, brief 67).

In short: the check would repair about 0.08% of the NAS store (5 of
6,766 files sampled, about 31 records), 4 of them plainly inverted.
With those records repaired, the four plainly inverted recordings go
from never found to found in every correct-polarity excerpt and mix,
no wrong answer is added and every other record is byte-identical. The
registered bar still fails on one item: a NAS regression against the
repaired store moves sub-threshold candidates and one confident
detection's fitted speed, because the lists set aside are a share of
the whole index. No ADR proposal.

### Experiments 0262-0263 (NAS scale)

| Run | Result |
|---|---|
| Census, 6,000 files (seed 2026) and 0248's 2,000; 6,766 distinct (0262) | 5 repaired: Paradox - Filmscape (new), Usual Suspects - Advent, Plasmic - The Weapon, Keltech - Axis (plainly inverted), We - Hello World (uncertain); 0.08% (0.02-0.28%) of 40,659 |
| Correlations at lag 0 | none between -0.76 and -0.32 in 6,766 files; DJ Sense (+0.62) not repaired |
| Partly inverted (an unflagged file with a 30 s window at -0.5 or less) | 29, at most 2 of 7 windows; none inverted over most of its length |
| The four plainly inverted: excerpts; mix plays, T against R (0263) | 0 of 48; 0 of 4, against 48 of 48; 4 of 4 |
| Hello World: excerpts, T against R | 12 of 12 both (median hits 313 and 1,450) |
| Exploratory: the four as ripped (uncorrected), T against R | 48 of 48 against 0 of 48 |
| Other rips of the five, by audio | none in the library |
| New wrong confident answers under R | none |
| Records: corpus; NAS sample of 46 rebuilt today's way; checked | 265 of 265 identical; 46 of 46; 41 identical, 5 repaired |
| `regress session-64-linux-nas` against R | 709 of 720 sweep queries identical (9: one confident detection's fitted speed, by up to 2e-4 percentage points; 2: sub-threshold candidates); Stakka counts, calibrate unchanged |
| The 2003 mix and Nocny Trans scans, T against R | confident detections equal (41, 42); Nocny loses two 6-hit candidates |
| The same against T with an index built afresh | identical |
| R's sweep with today's lists set aside (`--set-aside-like`) | 720 of 720 identical |

The bar: item 1 (recovery) holds, 2 (other rips) is unmeasured, 3 (no
new wrong answer) holds, 4 (byte-identical records and identical
regressions apart from F's files) fails on the regression, 5 (at least 3
in 4 repairs plainly inverted) fails on the registered census (2 of 3)
and holds on the union with 0248 (4 of 5). The sweep's differences are
one list crossing the set-aside cut each way (hash 1,337,041 from 9,910
to 9,913 postings, into the set aside; 69,523 out), the mechanism of
0242 and 0254 (for the scans, an inference); session 64's prune of four
files moved the same kind of quantities.

### Times on the Linux machine

| Run | Time | Peak memory |
|---|--:|--:|
| `regress session-54-linux`; `regress session-64-linux-nas` (start build) | 0:51; 3:54 | 0.4; 5.3 GiB |
| The same with the run build | 0:52; 3:48 | 0.4; 5.3 GiB |
| Census, 6,000 files, 16 jobs, alone (642 h of audio) | 9:43 | 1.4 GiB |
| `variant-store --repair-inverted`: corpus (265 files); NAS sample (46) | 0:37; 0:07 | 2.3; 1.7 GiB |
| `identify --store-only --saved-index off`, 65 queries at NAS scale | 0:40 | 4.5 GiB |
| `regress session-64-linux-nas` against R; against T, index built | 6:51; 6:00 | 5.3 GiB |
| `sweep --set-aside-like` against R | 2:28 | 8.6 GiB |
| `scripts/check.sh` | 5:08 | |

### Final checks

`work/s67/final.sh` with `work/bin/s67-final/` (master at `90dc565`):
`scripts/check.sh` green (4:57, 683 tests); the
`gunfinger` binary byte-identical to `s66-final`'s; `regress
session-54-linux` (0:51) and `regress session-64-linux-nas` (3:46)
identical, every report equal to its baseline's apart from
`wall_seconds`; `docs/pair-verdicts.txt`, the NAS clusters file
(`c4d3da2e7e37...`) and the answer log at their start digests
(`work/s67/listening-digests-end.txt`). `s67-final`'s `gunfinger-eval`
is not byte-identical to `s67-run`'s (with debug information stripped
too); the only source change between them is the test case added after
the review, in `stereo.rs`'s test module, and both builds give
identical regressions.

### Decided this session, open to change

- The detector's threshold is the census's (brief 62), not tuned again;
  lag 0 replaces the census's strongest lag because the repair reverses
  polarity at lag 0. A repaired file is "plainly inverted" when its side
  is 10 dB or more above its mid (narrow once repaired), else
  "uncertain".
- The census is not a superset of 0248's, as the pre-registration
  assumed: session 64's prune of 4 uploads changed the uploads' shuffle.
  Estimates use the union of both censuses, and F is every file the rule
  repairs in either (brief 67, 02:28).
- The control sample added DJ Sense - Let You Go from 0248's report
  (46 files instead of 45).
- The exploratory as-ripped queries were added after the registered
  results were in; no bar item rests on them.
- The bar is read as registered: item 4 fails, so no ADR proposal;
  item 5 is reported both ways.
- The harness change was committed (`a33b304`) after the census ran on
  `s67-run`, built from the same, then uncommitted, code (see the final
  checks for the two builds).

### Questions this leaves for us

- Whether to re-register item 4 as "unflagged records byte-identical;
  no confident detection gained or lost, no identification count or
  calibrate number changed; other changes accounted for by the
  set-aside cut" (which `--set-aside-like` checks for `sweep` and
  `recall`, not yet for `scan`), and item 5 on a census drawn from
  today's store, and then write an ADR for the check. The decision note's "Session 67" compares
  a profile bump (every record rebuilt) with a per-record flag (one
  check pass of about an hour over the NAS store, about 31 records
  rewritten; a repaired record must change something the saved index's
  provenance holds, because today it holds the sources and the profile,
  not the peaks).
- Whether a mix made digitally from an inverted file matters: the repair
  loses such a query (the exploratory row).

### Questions for listening (none answered by agents)

- Paradox - Filmscape (`Paradox - Filmscape (off004)/a-paradox-filmscape-sour.mp3`):
  plainly inverted by measure (-0.99, side 22 dB above the mid); not
  heard yet.
- We - Hello World stays unsure by ear; by measure it is the uncertain
  case (8.6 dB), and the repair costs little either way.
- Optionally: We - Polynesian Inventor (the same release; -0.27 at lag
  0, 2 of 7 windows at -0.5 or less), a wide track or a partial fault.
- Pending from before: the pitch rides (Star Trails, Into Music), the
  Planet Dust (DC Breaks Remix) cut, the Paragliders file against
  Konflict - Pendulum, the 65 candidates in
  `work/s58/different-name-files.txt`.

### Handoff for the next session

1. Baselines unchanged: `session-54-linux` (262 tracks) and
   `session-64-linux-nas` (NAS scale). The NAS store: 40,674 records,
   revision `7210fc479da58376`, only read; the NAS clusters file
   `c4d3da2e7e3720cbf4e79e4f7cf86d71493fc6349af639b323d814656a8b9e4f`,
   unchanged.
2. Builds: `work/bin/s67-start/` (= `s66-final`), `s67-run/` (every
   run), `s67-final/` (master; the final checks).
3. Data in `work/s67/`: logs in `logs/` (`queue.log` has every exit
   line); scripts `run.sh`, `start.sh`, `census.sh`, `recovery.sh`,
   `search.sh`, `asripped.sh`, `setup.sh`, `rebuilt.sh`, `setaside.sh`,
   `final.sh`, `queries.py`, `picks.py`, `compare.py`, `same.py`; the
   census report `w/reports/library-nas-dnb-peaks/stereo-width-other-seed-2026.json`
   and its summary `census-polarity.json`; the stores `stores/T/` and
   `stores/R/` (reflink copies of the NAS store; R has the five
   repaired records); the queries and plan in `q/`, the reports in
   `rep/T/` and `rep/R/`, the score `recovery-score.json`; the control
   stores in `c/` (the copied audio of its scratch library deleted); harness work directories
   `wt/` (T, the saved index hard-linked), `wr/` (R, with its own saved
   index, 4.7 GB) and `wt2/` (T, index removed). All safe to delete.
4. Commands:

   ```sh
   E=work/bin/s67-final/gunfinger-eval; N=~/.local/share/gunfinger/nas-dnb-peaks
   $E --work work/s67/w --other-peaks-dir $N --jobs 16 stereo-width --other --seed 2026 --sample 6000 --exclude work/s62/census-exclude.txt
   $E --corpus SCRATCH --peaks-dir SCRATCH_STORE variant-store --out OUT --repair-inverted
   $E --work work/s67/wr --other-peaks-dir work/s67/stores/R/nas-dnb-peaks sweep --set-aside-like work/indexes/library-nas-dnb-peaks.index
   python3 -I scripts/analysis/polarity_census.py REPORT OUT.json [EARLIER_REPORT]
   ```

5. If the check is taken further: re-register item 4 first (above),
   then the ADR. With today's harness, a scratch library of the files
   to check (copied with `cp -p`) and `variant-store --repair-inverted`
   build the repaired records; `--set-aside-like` separates the cut's
   move from the records' own effect.

### What else to know

- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. Only the development mixes
  (named explicitly: Stakka through `regress`, the 2003 mix and Nocny
  Trans through `scan`) and generated mixes were searched. The census left out the NAS root's Ed Rush & Optical
  file, 0065's too-long files and mixed-CD folders and the ignore
  file's files. The NAS was only read; `work/peaks` only read.
- Slips, all in brief 67: the pre-registration's heading first carried
  wrong times (corrected); `recovery.sh`'s first `identify` runs lacked
  the subcommand and exited at once (rerun by `search.sh`).
- A read-only review by a separate agent (02:59-03:11) found 19 points:
  item 5's two readings, an unsupported "could not have held", a unit
  test that did not isolate the lag condition (added), the regression's
  differences described too loosely, and wording; all fixed (brief 67,
  03:15).

## Session 68

### What changed

The product did not change: no threshold, default or output moved, and
both regressions stay identical. The harness gained `gunfinger-eval
counterexamples` (`counterexamples.rs`, wired in `commands/mixes.rs`),
opt-in and tested: it draws small generated mixes (a held-out lead-in of
30 s, the target of 40-120 s, a partner for 30 s after it) under
transformations within fixed bounds (speed ±6%, a 1-3% ride over 5-15
s, a bass kill and a blend of 10-60 s, brief 42's channel faults over
the mix's second half), renders them with `mixes::render`, searches and
scores them with the generated mixes' code, and minimizes each failure
greedily (drop one transformation at a time while the failure persists,
then halve each one left up to 3 times). Failures: the indexed target
with no confident detection (lost), found but with 30 s or more of its
play uncovered (gap), or a confident file matching no play (wrong).
Each failure's recipe goes to `<out>/recipes/NNNN.json`; `--replay`
renders and searches one again and checks that its seed, ladder and
index match. `scripts/analysis/counterexample_classes.py` summarises a
report. The axes, bounds, definitions, minimizer, classes and the bar
were pre-registered before the budgeted search (`7440c8e`, brief 68).

In short: the search finds only known failures and pairs of them. Every
play lost or gap needs a channel fault or a blend; speed, bass kills and
rides alone never failed. The wrong answers appear at NAS scale only and
need no transformation (three related-version families). Its value is
as a paired check of a variant with the same seed, at about 16-17
minutes a run.

### Experiments 0264-0266

| Run | Result |
|---|---|
| 2,000 samples at 262 tracks (0266, the rerun) | 188 failures in 17 classes: 159 gaps, 29 lost, 0 wrong |
| The same first 500 at NAS scale (0266) | 72 failures in 11 classes: 45 gaps, 11 lost, 16 wrong |
| Largest classes (262) | gap: channel-fault 85 (68 families); gap: ride+channel-fault 20, ride+blend+channel-fault 20, ride+blend 15; lost: blend 9, blend+channel-fault 11 |
| Alone, of 1,504 indexed targets | speed (42), a bass kill (55), the base (61): at most 1.3 s uncovered; a ride (54) 15.2 s; a blend (53) loses one |
| Wrong (NAS), no transformation | The Nine's Evol Intent VIP over The Nine (12), Cool Rock Shook Ones over an Aphrodite file (3), Pyro - Ignorance over Time Is Broken (1) |
| 12 minimal recipes replayed (0265, again in 0266) | 12 hold, outcomes equal to a microsecond |
| Against the roadmap | each class a known failure (faults 0198, double drops 0184, related versions) or two in one play; the pairs no entry names: a ride of 0.5-3% whose later side a blend or fault loses |
| Criterion | (b) holds (known classes from minimal recipes of 1-2 transformations, replayed); (a) only if a pair counts as new |
| Cost | 262: 16:30, 157 CPU s and 5.3 s per failure; NAS: 17:10, 389 CPU s and 14.3 s per failure |

0264 is the first run, superseded by 0266: it took the target's level
from the scorer's strongest detection by hits (`score_play`), so a
detection short of the rule's windows could hide a weaker confident one.
With the registered definition one failure goes (#92) and one recipe
minimizes otherwise; the rest are the same.

### Times on the Linux machine

| Run | Time | Peak memory |
|---|--:|--:|
| `regress session-54-linux`; `regress session-64-linux-nas` (start build) | 0:51; 3:47 | 0.4; 5.4 GiB |
| The same with the run build | 0:51; 3:47 | 0.4; 5.3 GiB |
| `counterexamples`, 2,000 samples at 262 tracks, 32 workers | 16:30 | 5.3 GiB |
| `counterexamples`, 500 samples at NAS scale | 17:10 | 9.6 GiB |
| `counterexamples --replay`, one recipe: 262; NAS | 3-7 s; 14-21 s | 0.3; 4.4 GiB |
| `scripts/check.sh` (run build) | 5:08 | |

### Final checks

`work/s68/final.sh` with `work/bin/s68-final/` (master at `c987a6e`;
no code change after it): `scripts/check.sh` green (5:00, 691 tests);
the `gunfinger` binary byte-identical to `s67-final`'s and the
`gunfinger-eval` binary to `s68-rerun`'s; `regress session-54-linux`
(0:51) and `regress session-64-linux-nas` (4:01) identical, every report
equal to its baseline's apart from `wall_seconds`;
`docs/pair-verdicts.txt`, the NAS clusters file (`c4d3da2e7e37...`) and
the answer log at their start digests
(`work/s68/listening-digests-end.txt`).

### Decided this session, open to change

- The bounds are the generator's ranges as registered, but the blend has
  no fades and may cover the whole target, and the kill lasts up to 60 s
  (a bass swap at most 30 s): harsher than `mixes::crossfade`. Every
  lost play with a blend is a double drop (0184's construction). Kept as
  registered for the rerun; stated in 0266.
- A gap failure (30 s or more uncovered in a found play) was added after
  the first smoke run and before the pre-registration, so that a ride's
  lost side could be found.
- "Not new" for a pair of known failures in one play was decided after
  the runs; the literal reading would count `gap: ride+blend` as new
  and meet criterion (a). Both readings are in 0265 and 0266.
- The NAS run drew the same 500 bases as the 262 run's first 500.
- The minimizer drops in the draw's order (speed, ride, kill, blend,
  fault), so where two minimal sets exist the fault tends to stay; it
  does not retry drops after halving (as registered).

### Questions this leaves for us

- Whether to run `counterexamples` with the same seed beside a variant
  (for example a speed per stretch, `--speed-per-stretch`, against the
  ride pairs), as a paired check: the samples are the same, so classes
  gained or lost compare directly.
- Whether to widen the axes: fades on the blend (the generator's
  crossfade), the fault's placement within the play, other EQ, quiet and
  beatmatched partners, key-locked plays. With today's axes the search
  repeats known classes.

### Questions for listening (none answered by agents)

None new: nothing in this session needs listening.

### Handoff

1. State: master clean after the final commit; builds
   `work/bin/s68-final/` (master), `s68-run`, `s68-run2`, `s68-rerun`
   (the reported runs) and `s68-replay`.
2. Data: `work/s68/c262b/` and `work/s68/cnasb/` (reports and 188 and
   72 recipes; renders deleted), the first runs in `work/s68/c262/` and
   `work/s68/cnas/`, replays under each `replays/`, logs in
   `work/s68/logs/`, about 11 MB in all.
3. To run it again or beside a variant:
   ```sh
   E=work/bin/s68-final/gunfinger-eval; N=~/.local/share/gunfinger/nas-dnb-peaks
   $E counterexamples --seed 2026 --samples 2000 --out OUT
   $E --other-peaks-dir $N counterexamples --seed 2026 --samples 500 --out OUT
   $E counterexamples --out OUT --replay OUT/recipes/0007.json
   python3 -I scripts/analysis/counterexample_classes.py OUT/counterexamples-seed-2026.json SUMMARY.json
   ```
   Matching options (`--speed-per-stretch` and the like) are global and
   apply to the search; `--seed` other than 2026 writes a new sweep
   panel into `docs/panels/` (`Plan::for_seed`), so keep 2026 or pass
   `--panels` to a scratch copy.

### What else to know

- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. Only generated mixes were
  searched (and Stakka through `regress`). The NAS was only read;
  `work/peaks` only read.
- Slips: the first NAS run exited at once because `counterexamples` was
  not among the commands that take `--other-peaks-dir` (added,
  `d77e4ae`); the first replay build was copied from a binary built
  before the detections change and compared floats exactly (rerun with
  `424185d`).
- serde_json (without `float_roundtrip`) reads some of a recipe's floats
  one unit in the last place off, so a replay's play ends can move by
  about 1e-14 s; replays compare outcomes to a microsecond. Enabling
  the feature would touch every report the harness reads, so it was left
  alone.
- A read-only review by a separate agent (04:37-04:48) found 13 points:
  the target's level (fixed and rerun), the blend's and kill's distance
  from the generator (stated), miscounts in 0264 and the notes (The
  Nine 12, not 13), the ride range and the 0176 citation, "one or two
  more", the post-hoc reading of the criterion, the float explanation,
  stale references, replay context checks (added), the minimizer's
  order, the fault's fixed placement, and small code points (wrong
  files now collected once each); all fixed or stated (brief 68, 04:50).

## Session 69

### What changed

The product did not change: no threshold, default or output moved, and
both regressions stay identical. The harness gained `gunfinger-eval
passages` (`passages.rs`, wired in `commands/mixes.rs`), opt-in and
tested, a pilot of proposal 6 of the proposals note (recurring
unidentified passages across a batch of mixes). It draws a batch of 12
generated mixes in which 16 recordings left out of the index occur three
times each, with different partners, cue points, bass swaps (the long
mixes' crossfades) and speeds: occurrences at 1.04-1.08 and 0.92-0.96,
opposite deck settings, and a third at 0.94-1.06; for half of the
subjects the third occurrence shares no source time with the others.
Controls: those non-overlapping excerpts, the shared-material panel's
development pairs (Dozer, Fire, Sonar as related versions; Pyro and
Watermelon as different tunes sharing a passage), and a 12 s insert in
6 mixes. Each mix is searched against the 262-track index; its
unidentified spans (30 s or more without a confident detection), or the
whole mix, are indexed in memory as pieces of at most 600 s and searched
against each other at ±8% (the product's ladder) and ±18%
(`--relative-speed-range`, 91 rungs). A link is a confident detection
under the frozen rule, reported pairwise with its spans; only the two
directions of one match are joined, and links are scored against the
construction (right, insert, one recording at another passage, shared
passage, other). `scripts/analysis/passages_summary.py` summarises the
reports. Pre-registered in `084d659` (brief 69) before any run.

In short: the links are precise as passages but recall from the
unidentified spans is low. Over ten batches no link joined unrelated
records, and shared-material links stay within 0.2 s (gaps) or 12.3 s
(whole) of their passage. From the unidentified spans only half of the
recurring passages are found, because a held-out record's time in a gap
is short; whole mixes find 85%. ±18% is needed for opposite deck
settings and costs 2.33 times the product's ladder.

### Experiments 0267-0268

| Run | Result |
|---|---|
| Registered (seeds 6901-6902), gaps mode, ±18% | 30 of 60 subjects' positives (bar 80%), 23 of 43 above 8% (bar 70%): fails (1); no "other" link and 0 of 32 control pairs linked: (2) holds; shared links at most 0.2 s past their passage: (3) holds |
| Ten batches, ±18%: gaps; whole | 192 / 312; 266 / 312 positives found |
| Ten batches, ±8%: gaps; whole | 77 / 312; 100 / 312 (none of the 112 beyond 12% relative speed) |
| Why gaps mode misses | 77 of 580 held-out plays have no unidentified time (28 inside their neighbours' crossfades, 49 with at most 21 s between them); the median play keeps 58% of its time in a gap |
| Found spans against the true overlap | gaps: 19 s late, 19 s early, half covered (medians); whole: 9 s, 9 s, 0.7 |
| Wrong links, ten batches, ±18% | other 0; shared passage 20 of 50 pairs (Fire 10, Pyro 10, Dozer 0-1, Sonar and Watermelon 0); one recording at another place 15 (gaps) and 37 (whole), 51 of 52 at the right relative speed and 10-178 s apart in the track, 9 and 11 joining control pairs |
| The insert | never linked: 12 s is shorter than the rule's windows, so it could not test bridging |
| Cost per batch (0268) | 1:15-1:22, 1,146-1,325 CPU s, 2.33-2.56 GiB; pairwise thread seconds: gaps 79 (±8%) and 185 (±18%), whole 176 and 409; ±18% costs 2.33 times ±8% |

### Times on the Linux machine

| Run | Time | Peak memory |
|---|--:|--:|
| `regress session-54-linux`; `regress session-64-linux-nas` (start build) | 0:51; 4:01 | 0.4; 5.3 GiB |
| The same with the run build | 0:51; 3:53 | 0.4; 5.3 GiB |
| `passages`, one batch of 12 mixes (2.6-2.85 h), both modes and ranges, 32 workers | 1:15-1:22 | 2.33-2.56 GiB |
| `scripts/check.sh` (run build, after the batches; final build) | 8:53; 7:36 | |
| `regress session-54-linux`; `regress session-64-linux-nas` (final build) | 0:52; 3:59 | 0.4; 5.3 GiB |

### Final checks

`work/s69/final.sh` with `work/bin/s69-final/` (master at `e9dda8a`;
no code change after it): its `gunfinger` byte-identical to
`s68-final`'s; seed 6901 run again gives the run build's report apart
from timings and the control flag's new name; `regress
session-54-linux` (0:52) and `regress session-64-linux-nas` (3:59)
identical, every report equal to its baseline's apart from
`wall_seconds`; `scripts/check.sh` green (7:36, 699 tests);
`docs/pair-verdicts.txt`, the NAS clusters file (`c4d3da2e7e37...`) and
the answer log at their start digests
(`work/s69/listening-digests-end.txt`).

### Decided this session, open to change

- Gaps mode takes spans of 30 s without a confident detection (not the
  product's gaps of confident plays' segments at 45 s), so that it
  reaches every missed DJ-length track by 0159; even so, plays inside
  or between long crossfades leave nothing to search.
- A correspondence joins only links between the same two mixes that
  overlap in both, the first such one in order; within one pair of
  mixes the result can depend on the links' order.
- Eight batches beyond the two registered ran beside them, decided
  before the runs; the bar was read on the registered two.
- No NAS-scale batch: the subjects' other rips in the NAS store would
  have to be left out first.
- The insert as built (12 s) cannot pass the rule's windows; a longer
  insert, or one at another speed, is needed to test whether an insert
  bridges two records.

### Questions this leaves for us

- Whether to search whole mixes, or unidentified spans with a shorter
  minimum widened over the crossfades around them, and report a
  correspondence only with its spans (never a title or a track).
- How the pairwise search's cost grows with an archive (every mix a
  query against every other); not measured beyond a batch of 2.6-2.85 h.
- Whether links between one recording's different places (its own
  repeats) need a rule of their own before such links are shown to
  anyone.

### Questions for listening (none answered by agents)

None new: nothing in this session needs listening.

### Handoff

1. State: master clean after the final commit; builds
   `work/bin/s69-final/` (master), `s69-run` (the reported runs) and
   `s69-start`.
2. Data: `work/s69/batches/` (ten reports; renders deleted),
   `work/s69/final-batch/` (seed 6901 again with the final build),
   summaries `work/s69/summary-registered.json` and
   `work/s69/summary-all.json`, logs in `work/s69/logs/`; about 5 MB.
3. To run it again:
   ```sh
   E=work/bin/s69-final/gunfinger-eval
   $E passages --seed 6901 --out OUT --other-root /mnt/atlas/Music/dnb
   $E passages --seed 6901 --out OUT --other-root /mnt/atlas/Music/dnb --mode whole --relative-speed-range 0.18
   python3 -I scripts/analysis/passages_summary.py SUMMARY.json OUT/passages-seed-6901.json
   ```
   The NAS root is read only to render the panel's NAS files; nothing at
   the NAS root plays. `--extra-rungs` is refused (the ranges set the
   ladders).

### What else to know

- The held-out test set was not touched: no command named it, and no
  manifest or report of it was opened. Only generated mixes were
  searched (and Stakka through `regress`). The NAS was only read;
  `work/peaks` only read.
- A read-only review by a separate agent (06:17-06:24) found no product
  change and no error in the draw or the scoring; it corrected the
  reason gaps mode misses plays (crossfades for 28 of 77; spans under
  30 s for 49), the insert's limit, an unmarked inference, the shared
  links' reach, unrecorded deviations, units and medians, and asked for
  house-style fixes (an enum for the control flag, a named result for
  the pairwise search, `draw_batch` split, no panicking lookup); all
  fixed or stated (brief 69, 06:17-06:24).

