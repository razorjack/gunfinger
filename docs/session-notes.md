# Session notes

Findings from the autonomous sessions that are worth noting, newest
last, after a summary of each session (session 7 first).

## Session 7 summary

**Nothing changed for users.** The default matcher, the confidence rule,
the clusters, the manifests and `docs/pair-verdicts.txt` are as session 6
left them; `gunfinger-core` and the CLI were not touched. New harness
diagnostics: `gunfinger-eval --verify` (a peak verifier), `gunfinger-eval
shared` (shared-material scenarios), `sweep --other-rips` (other rips
only) and supported time in `pair`. Nothing was listened to. The NAS was
not read; its peak store was used read-only and is unchanged at the end
(27,042 records, revision `d428ee9585936326`, digest `bf35ea64...e9ae4f5`).

**Test-set evaluation 3** (we approved it): 14/16, 0 wrong, pass. Sick
Note is possible at 233 hits, 7 below the rule, as predicted; Sin is
possible at 123. 2 evaluations are left.

**The peak verifier** (experiments 0054, 0056, 0058): identifying
detections find a median 0.85 of the found file's peaks, chance about
0.01. But at equal hits its share separates true from false detections no
better than hits (AUC 0.953 against 0.950 at 262 tracks), and worse
against related records: weak blend partners (0.16-0.24) look like
shared passages (0.11-0.41). **Proposal: no verifier rule**; keep it as a
diagnostic and as an aid for ordering what to listen to.

**Shared material** (experiment 0055): Clockwork ~ its remix and China
Cup ~ The Nine, alone, looped to 240 s, into distinctive material, with
the played recording left out: nothing reaches the rule, and looping
does not add hits.

**Supported time** (experiment 0057): coverage and the seconds that hold
aligned hits order pairs differently. The Sonar revision joins hold hits
on 0.39-0.43 of the file; China Cup and The Specialist against the
uploads the clusters keep apart, 0.96-0.99 at two offsets.

**Relative speed at NAS scale** (experiment 0059): with each excerpt's
source and identical copies left out, other rips alone answer 1,213 of
1,305 queries. Recall holds to 9% of speed relative to the nearest
remaining file and is gone from 10.2%; `--extra-rungs 3` adds 9.2-9.5%
only. Every miss inside 9% is a weak copy found below the rule. The 24
wrong answers all come from two listening-list pairs.

**Decisions to make.**

- The listening list after session 7 (under "Session 7 findings"): the
  same pairs, now with supported time and verifier shares beside them.
  China Cup ~ its Prototype upload is held almost whole at two offsets
  and verifies like another upload of one recording; it causes most
  wrong answers left.
- Whether to adopt the verifier in any form (proposed: no rule).
- How far the decks go past ±5-6%: that decides whether a wider or a
  targeted speed search is worth its cost for plays against an
  off-speed copy (experiment 0059).
- Mixotic could not be downloaded: Google Drive answers with a sign-in
  page. A browser download into `work/datasets/mixotic/` would let a
  later session record its layout.

## Session 6 summary

**What changed for users.** `identify` and `explain` search with the
fitted matcher by default (ADR 0008): the fullest 1% of posting lists
skipped while looking for candidates, the link rules, each candidate
measured again at its fitted speed, confident at 240 hits in 3 windows,
possible at 60. `--single-pass` gives the matcher before. Saved reports
record the matcher, so `identify --save-dir` searches each mix again
once. The tracklist and the cue sheet list each recording once, with
other spellings as `(also: ...)` (13 lines for the development mix from
the NAS store instead of 20). `index` reads MP3s in a WAV container
behind an ID3 tag, passes over hidden folders, and tolerates audio that
stops up to 1% (at least 1 s) short of its declared length. The
installed binary was not replaced.

**Evidence for the new default.** At 262 tracks (experiment 0047) the
candidate baseline is reproduced exactly; sweeps 2026-2029 recall
540/540 each with 0 wrong; the development set 11/11 and leave-outs 0
wrong; margin 6.65× (4.15× before); the development scan's detections
matching no track fall from 6,096 to 69; mix 10's Dominion and the
grid's 10 s plays become possible (they were confident only through a
chance window). At NAS scale (experiments 0050, 0051) the development
mix takes 263 s, 1,943 CPU seconds and 4 GB from the store alone (856 s
and 8.4 GB before), and identifies 11 of 11 with 0 wrong.

**Clusters.** `clusters` searches ±8% and keeps the better of two
alignments per pair (experiments 0048, 0049). The 17 corpus clusters
reappear at both sizes; 46 more NAS uploads join their recordings,
including the 6 behind session 5's wrong identifications. Re-scored,
session 5's reports owe 465 of 505 false confident sweep detections to
rips the clusters missed. The verdicts by ear go into `docs/pair-verdicts.txt`
(empty; format in its header).

**At NAS scale now** (experiments 0051, 0053): the four sweep seeds
recall 540/540 each, with 0, 24, 8 and 8 wrong answers (session 5: 91,
174, 126, 114). All 40 come from three pairs: China Cup against its
Prototype upload, Coma against Spraycan, and Synthesis VIP against the
mixed CD's "Synthesis (Remix)". Margin 1.29× under the rules (0.11×),
3.36× if those three pairs are one recording each; unrelated chance
63-70 hits.

**Scaling** (experiment 0052): recall and the development mix hold at
1,000, 3,000 and 9,000 NAS records; unrelated chance is 21-34 hits up
to 3,200 assets and 64 from 9,200; CPU grows linearly with the records.

**Decisions to make.**

- The listening list ("Listening list after session 6" below): China Cup
  ~ its Prototype upload, Coma ~ Spraycan, Synthesis VIP ~ "Synthesis
  (Remix)" (the three pairs behind every wrong answer left), The Nine ~
  Evol Intent VIP, the borderline pairs, and DJ
  Trace - Sonar against its "Mark System Revision", which the clusters
  now join.
- The possible tier: 60 while unrelated chance stays below 30 hits (to
  about 3,000 assets), twice that chance above (130-140 at NAS scale;
  experiment 0052). It stays 60 until we decide.
- The next test-set evaluation (3 left) should measure the new default
  matcher on the test mix with both playbacks: 15/16 and Sin possible
  under the single pass; Sick Note (209 hits then) most likely becomes
  possible.

## Session 5 summary

**Default detection is unchanged and no test-set evaluation was spent**
(3 left). The only code added is harness and analysis code (`gunfinger-
eval fullest`, scripts in `scripts/analysis/`); `regress session-5-start`
stayed identical. Everything comes from the NAS peak store and `corpus/`:
the NAS was never read, and the store was not written. At the end its
file counts, digest and library revision `d709390272e41475` were as at
the start.

**Done.** Items 1-10 of `docs/brief-5.md` (experiments 0035-0044, data
in `docs/experiments/data/`) and item 12 in part: the link rules'
development scan under both matchers and sweep 2026 under skip at 240
(experiment 0045). Today's sweep with the rules, item 11 (two more of
today's sweeps) and item 13 (a scaling curve) did not fit: one of
today's sweeps takes 66-69 minutes at this size.

**What I learned.**

1. Recall holds at NAS scale (26,914 assets) under both matchers: every
   indexed sweep excerpt (2,160 under skip at 240, 1,080 under today's)
   and every development track, with the same weakest evidence as at
   262 tracks (661 and 403 hits).
2. Under the rules the margins are gone, because other uploads of the
   same tracks count as false: the clusters do not join them. Many
   YouTube uploads are 2.9-4.7% faster than the rips, beyond the
   clustering ladder of 0.98-1.02; they cause the 6 wrong
   identifications in the development mix. Without the same-name
   uploads and two pairs to check manually, the margin is 3.0× under skip
   at 240 and 2.02× under today's matcher, where The Nine against its
   Evol Intent VIP reaches the rule of 200 exactly.
3. Unrelated chance reaches 67-78 hits, above the possible tier of 60.
   The calibration register's condition for raising the tier (to about
   twice that, 140-156) is met; I did not change it.
4. Skip at 240 keeps its lead at this size: 3.3× less wall time, 3.1×
   less CPU, 48% of the memory, 4.3% of the false candidates, and higher
   weakest evidence. The fullest lists it skips are spread over the
   whole collection, so it hides no record (0043).
5. The scale proxy predicted time and memory within 5-18% and chance
   badly (real tracks reach 1.3-2.7 times the copies' hits).
6. `identify --store-only` gives the harness's plays exactly, in 14
   minutes and 8.4 GB under today's matcher; its tracklist lists a track
   once per tag spelling.
7. The NAS index run over Wi-Fi read 84 Mbit/s, limited by reading.

The band profile (`Profile::spread`) could not be measured at NAS scale:
it needs new peak records, which means reading the NAS audio.

**Decisions to make.**

- Listen to the pairs in "Session 5 findings" below: the 6 wrong
  identifications, Synthesis VIP ~ "Synthesis (Remix)", Coma ~ Spraycan,
  The Nine ~ Evol Intent VIP, and the borderline cluster pairs.
- Widen the clustering ladder to ±8% (the search's range), so fast
  uploads join their recordings, then rerun `clusters` at NAS scale. A
  proposal, not implemented; it changes ground truth, which is our decision.
- Adopt skip at 240: NAS scale strengthens the case of session 4. The
  link rules with it changed no level or answer at NAS scale (0045).
- The possible tier: keep 60, or raise it to about 150 as the register
  says.

## Session 4 summary

**Default detection is unchanged and no test-set evaluation was spent** (2
of 5 used, 3 left). The changes meant to keep results identical (item 2's
block-wise search, item 8) were checked with `gunfinger-eval regress
session-4-start`: identical detections. Everything that changes
detections is opt-in and went through the full protocol under today's
matcher and skip at 240: sweeps 2026-2029, the development scan,
leave-outs 3 and 11, calibrate, regress, and the generated mixes.

**Done.** All eight items (`docs/session-4-checklist.md` has the
detail), in experiments 0028-0034, each with its data in
`docs/experiments/data/`:

1. Idle-machine reference: profiles, scale runs from 262 to 31,964
   assets with both matchers, `identify`, indexing (0028).
2. Search memory at scale: the search merges lines a block of 12 windows
   at a time; identical detections, peaks 14-45% lower (0029).
3. Chance lines in chains: two opt-in link rules (0030).
4. Speed per stretch in the second pass, opt-in (0032).
5. Peaks across bands: postings and true hits by band; two variant
   profiles, each in its own store (0034).
6. Trimming weak chain ends, opt-in (0031).
7. The edge of the ladder and extra rungs, opt-in (0033).
8. Small engineering: the peak store names its library, a second
   library in the harness, `doctor` counts only files in the length
   range, shared material shown in the report; and the advice to run
   `gunfinger index` now names the library.

**What I learned**, item 1 first, since it bears on the matcher decision:

1. Search CPU grows linearly with the library: about 0.21 s per asset
   for the 56-minute development mix with today's matcher, 0.055 s with
   skip at 240. At 31,964 assets that mix takes 15.2 minutes today and
   4.7 with skip at 240, 3.5× less, at half the memory. At scale 77-87%
   of the CPU sorts each window's hits; at 262 assets analysing the
   query on 82 rungs dominates. Indexing takes 0.2 s per track.
2. Memory at scale was mostly freed hit buffers kept by macOS's
   allocator, not lines. After item 2 the development mix at 26,462
   assets peaks at 7.7 GB (3.8 GB with skip at 240).
3. The peak picker spends most of the index on high anchors that the
   mix does not keep: 2-4 kHz anchors are 64% of the postings the plays
   look up and give 21% of their true hits. Neighbourhoods that widen
   with frequency give the mix 34% more true hits from 36% fewer
   postings looked up and raise both matchers' margins, but related
   records find each other more easily.
4. Chance lines: under today's matcher the link rules give what skip at
   240 already gives (boundaries, 68% fewer false candidates); under skip
   at 240 they raise the margin from 5.53× to 6.65×.
5. The ladder holds to ±8.4% and is gone at ±9%; three extra rungs cover
   ±9% for 14% more CPU and change nothing else.
6. Speed per stretch recovers hits where speed drifts slowly (Star
   Trails +11%) and changes no level. Trimming weak chain ends costs 2.6
   s of true play per second of overshoot removed.

**Opt-in results and their cases** (details in "Session 4 findings"
below):

| Variant | Case for and against | My recommendation |
|---|---|---|
| Link rules (`--nearby-rungs --strong-gaps`) | for: right boundaries and 68% fewer false candidates under today's matcher, margin 5.53× → 6.65× under skip, no CPU; against: a speed change over 0.6% between windows splits a play into segments | adopt, with either matcher |
| Band profile, variant a (`Profile::spread`) | for: development mix +34% true hits, 9% fewer postings, margins 4.15× → 4.96× and 5.53× → 5.82×; against: every record extracted again, related records reach the possible tier more easily, one development mix | adopt after choosing the matcher, confirmed by one test evaluation |
| Extra rungs (`--extra-rungs 3`) | for: every speed to ±9%; against: 14% more CPU | only if the decks go past ±8% |
| Speed per stretch (`--speed-per-stretch`) | for: more evidence on drifting plays, no CPU; against: no level changes in any set, false candidates gain too | keep opt-in |
| Trimming weak ends (`--trim-ends`) | against: 219 s of true play lost to remove 85 s of overshoot | do not adopt |

**Decisions to make.**

- Adopt skip at 240 (session 3's open decision)? Item 1 adds that it is
  3.5× cheaper at 31,964 assets and peaks at half the memory.
- The link rules, the band profile and extra rungs, as in the table.
  Each changes detections; the band profile also means extracting every
  peak record again.
- Test-set evaluations (3 left): none spent. One evaluation of the
  chosen matcher with what we adopt would confirm it.
- Worth checking by ear: whether Bad Company - China Cup shares
  material with The Nine: with the band profile, China Cup held out
  finds The Nine with 43-50 hits. Answered: they share their drums (see
  "After session 4" at the end).

## Session 3 summary

**Default detection is unchanged and no test-set evaluation was spent** (2
of 5 used, 3 left). Every change to the core was checked with `gunfinger-eval
regress session-3-start`: identical detections.

**Done.** All seven items (`docs/session-3-checklist.md` has the detail),
in experiments 0019-0027, each with its data in `docs/experiments/data/`:

1. Memory by phase: the index is built in two passes and nothing else
   stays loaded, 3.1 GB at 26,462 assets instead of about 16 GB (0019);
   search memory at scale measured (0021).
2. Reports record the settings they were made with, `--save-dir` searches
   again when any differ, writes are atomic; `explain --windows`.
3. Generated mixes with exact truth, and brief plays slid across the
   window grid (0020).
4. Where the development mix loses evidence (0022); new robust
   conditions: wow, broadcast processing, beatmatched blends, combined.
5. The second pass at each candidate's fitted speed, opt-in, with its own
   null and calibration (0024).
6. The second pass with the most common hashes set aside: the protocol,
   every robust condition, the padded index and the cost; its rule set at
   240 hits from calibration data (0026, 0027). The case for adoption is
   below.
7. Sweep panels kept per seed and four seeds (0025); duplicate clusters
   from stored peaks, a seventh of the CPU (0023).

**What I learned**, in order of how much it should change plans:

1. Shared material, not chance, now sets the threshold. Measured by the
   second pass, the passage the Clockwork remix shares with the original
   (confirmed by ear) reaches 119 hits, which put that pass's rule
   at 240. Chance alignments with reversed copies stay at 18-31 hits.
   Hits cannot tell a shared passage from the original; the roadmap's
   "confidence statistic for shared material" is needed sooner than
   expected.
2. Skipping the most common hashes while looking for candidates, and
   counting them again in the second pass, keeps the evidence and removes
   most of the cost at scale: 94% fewer false candidates, 57-63% less
   search CPU at 8,122 and 26,462 assets, and 94% fewer first-pass lines
   at 26,462. Emptying those lists, as proposed in session 2, loses too
   much evidence.
3. The development mix keeps 13% of the reference hashes its plays could
   match. The mix itself (records, mastering, recording, AAC) costs most,
   and most at 1-4 kHz, where the peak picker spends three quarters of the
   hash budget (0022).
4. Whether a 15-20 s play is confident depends on where the 10 s windows
   fall; a 10 s minimum span would fix that (0020, offline). Chains can
   also take in a chance line up to 30 s away.
5. Over four sweep draws the margins are narrower than one draw showed:
   200 sits 2.0 times below the weakest identifying detection, not 2.5
   (0025).

**Decisions to make.**

- Adopt the second pass with common hashes skipped, at 240 hits? The
  case is in "The case for adopting the second pass with common hashes
  skipped" below. I would confirm it with one test-set evaluation first.
- Test-set evaluations (3 left): the test mix has never been searched
  with both playbacks, and Sick Note (209 hits) would most likely drop to
  possible under the second pass. One evaluation of each matcher would
  show both; I spent none.
- A 10 s minimum span instead of 3 windows: whether a 15 s play should
  count is our call. Fixing chance lines in chains should come first;
  both change detections and need the full protocol.
- Peaks spread more evenly across frequency bands: it changes the peak
  profile, so every peak record would be extracted again.
- Worth checking by ear: whether
  `a-unknown-udfr014-(synthesis_vip)` shares material with Muffler -
  Bleak (28 hits, the strongest on audio not in the index).

## Session 2 summary

**Default detection is unchanged and no test-set evaluation was spent** (2
of 5 used). Every change to the core was checked with `gunfinger-eval
regress session-2-start` or the development scan: identical detections.

**Done.** Everything on the command-line list (`docs/session-2-checklist.md`
has the detail): colour, position in the track, same-audio rows collapsed,
timeline, `explain`, `listen`, `review`, export formats and `show`, batch
identification, completions and man page, `--quiet` and progress, a
configuration file, `doctor`, `prune`, remembered failed files. Test
infrastructure without CI: a synthetic end-to-end test through the real
binary, `baseline`/`regress`, property tests, and a determinism check.
Eleven experiments (0007-0017), most on transformed copies of the library,
each with its data in `docs/experiments/data/`.

**What I learned overnight**, in order of how much it should change plans:

1. Shared material, not chance, sets the thresholds as the library grows.
   A library padded to 21,109 assets with reversed copies barely moved
   chance (strongest 30 hits; experiment 0012), while two Aphrodite records
   that share a break give each other 130 hits in 19 s (0015). Jungle and
   breakbeat hardcore will have much more of that.
2. Memory is the first wall: the scan at 26,000 assets swapped
   (0012). I did not measure what used the memory: the
   peak records stay loaded after the index is built, and building it
   turns the whole library into points first, so an on-disk index alone
   may not be the fix (roadmap, "Memory by phase").
3. Emptying the fullest 1% of posting lists is worth adopting before the
   library grows: a query scans 62% fewer postings, own-track hits drop
   17%, chance evidence drops by more than half, and the full protocol
   passes with a wider margin (5.86× instead of 5.27×; experiments 0013,
   0017). It saves no time at 262 tracks, where lookups are a small part
   of the search. Triplet hashes would scan 25-79 times fewer postings but
   lose about half their evidence under heavy damage (0014, offline).
4. Key lock works: `--playback both` finds CDJ/master-tempo playback at
   ±2-8% (0010) and passes the full protocol with unchanged thresholds at
   1.57 times the CPU time (0016, 0018). It is the default since
   2026-10-06.
5. Robustness: 32 kinds of damage, no false identification (0009).

**Decisions to make.**

- Key lock by default: decided on 2026-10-06, on. My
  `~/.config/gunfinger/config.toml` sets `playback = "turntable"`.
- The test set has only been searched with turntable playback. One
  evaluation (3 left) would confirm 15/16 with both; I did not spend it,
  since the development protocol passes unchanged (0016).
- Should the fullest 1% of posting lists be dropped by default? It passes
  the protocol, but every hit count changes, so I would confirm it with
  one test evaluation (3 left). `docs/calibration.md` ("Measured, not
  adopted") says how.
- `listen` and `review`: tested manually on 2026-10-06; every snippet
  worked.

## Findings

### Search is deterministic; the elegance pass changed speeds by one ulp

Three runs of the development scan give bit-identical reports (experiment
0007). The tiny speed differences I mentioned earlier (at most 4.4e-16)
came from the elegance pass rewriting the speed fit, not from run-to-run
variation. My earlier claim that the elegance pass gave "identical reports"
held to 9 decimal places, not bit for bit; `docs/status.md` and
`docs/calibration.md` now say so.

### Shared drum sounds can stretch a detection past the end of a record

In the synthetic end-to-end test, when every track used near-identical kicks
on the same beat grid, one track's detection ran 26 s past its end, across
an unrelated insert (experiment 0008). With track-specific kicks it ended
within half a second of the truth. Real tracks rarely share identical drum
sounds, but jungle and breakbeat hardcore share sampled breaks, so this is
worth remembering for ADR 0007 and for detection boundaries in general.

### Some library tags are truncated or repeat the artist

`--format tracklist` and `--format cue` name tracks from their tags. On the
development mix two tags look wrong: `Stakka & Skynet feat. Kemal &
Rob Data - Bios-Fear.mp3` has the artist `Stakka & Skynet feat. Kemal an`,
cut at 30 characters (the ID3v1 limit, so the file probably has only ID3v1
tags), and the TeeBee remix of Side Effects has the artist repeated in its
title. Gunfinger now drops a repeated artist; truncated tags it cannot fix.
Retagging those files, or a tag-cleanup pass over the library, would make
tracklists cleaner. Detection is unaffected: it never reads tags.

### Robustness: no false identifications under 32 kinds of damage; key lock is the gap

Experiment 0009 played 40 indexed and 10 held-out excerpts through EQ,
filters, telephone band, echo, clipping, noise, low-bitrate MP3/AAC/Opus,
blends with another track, voice-over, speech inserts, needle skips, pitch
rides, speeds outside ±8% and key lock. Nothing produced a wrong confident
answer, and the strongest false match on audio not in the index had 26 hits
against the possible tier's 60. Damage only costs evidence: noise at 10 dB
SNR, clipping, echo and a blend at -6 dB keep 30-40% of the hits, enough on
any real play of a few minutes.

Key lock (CDJ "master tempo": tempo changes, pitch does not) is the one
realistic case the algorithm misses: at ±2% nothing is confident and 35 of
40 excerpts are only possible; at ±5% nothing is found. Vinyl cannot do
this, but CD and digital sets, and some radio edits, can. I am testing a
tempo-only ladder next. Whether the target mixes include
key-locked CDJ or digital sets decides how much this matters.

### Key lock is solved; the default is unchanged

Experiment 0010: rungs that stretch time but not frequency find every
key-locked excerpt at ±2%, ±5% and +8% (40 of 40 each, hits as strong as
untouched audio), and adding them to the turntable ladder changed nothing
for vinyl-style audio and created no false identification. The cost is
search time: 82 rungs instead of 41, measured later at 1.57 times the
CPU time (experiments 0016, 0018). I left the default on the turntable
ladder because my sets are vinyl, and added `identify --playback both`
(or `key-lock`); key-locked plays are marked `(key lock)`, and `listen`
time-stretches them instead of resampling. To identify CD or digital
sets, use `--playback both`; making it the default would need the full
evaluation with both ladders first.

### `listen` and `review` were untested with real audio output

ffplay reported "audio open failed" (no audio device). I
checked the ffplay commands with `listen --print` and ran the same filters
through FFmpeg into a null output, but I have not heard them.

Update, 2026-10-06: `listen` was run at several moments of the
development mix and `review` with play numbers, `start`, `end` and `at`;
everything worked.

### A hundredfold library barely moves chance; memory is the first limit

Experiment 0012 padded the index with reversed, stretched copies of every
record (up to 21,109 assets, about 80 times the library). Chance
alignments grow slowly: an excerpt's typical strongest wrong candidate went
from 7 to 12 hits, and over the whole development mix at 8,122 assets the
strongest chance alignment was 30 hits (possible needs 60, confident 200).
Results on real tracks did not change at all. Reversed copies contain no
shared breaks or remixes, so this is a lower bound; real false candidates
(remixes, 91-95 hits) remain the ones that matter, and breakbeat hardcore
is the case to measure when it is added.

What did break: the development scan at 26,462 assets needed more memory
than the machine had free; it swapped and was stopped
before it finished. Later correction: what used the memory was not
measured, so the on-disk index (ADR 0005) is not necessarily the fix; the
roadmap lists the measurement.

### Summing a play's segments is not worth it yet

Experiment 0011 checked, on existing reports, what happens if a play's
segments were added up. Summing every segment lets unrelated audio reach
109-125 hits on long stretches; summing only segments of 60 hits or more
creates no false group, but would not have helped any track we know about
(Sin's two segments sum to 168, still under 200). No change.

### Shared breaks: Aphrodite's records find each other

Experiment 0015 matched the library against itself to list different
recordings that share material. 21 pairs share 30 hits or more and 5 reach
the possible tier, none the confident rule. The strongest is two Aphrodite
drum versions (130 hits in 19 s), then the Stakka remix of Clockwork
(129); 12 of the 21 pairs are Aphrodite with Aphrodite. When we add
jungle and breakbeat hardcore, expect many more of these; they are the
false candidates that will push the thresholds, not chance.

### Common hashes: most lookups, most chance, little evidence

Experiments 0013 and 0017: the fullest 1% of posting lists hold 16% of the
postings but 62% of what a query scans, and they carry most chance
alignments. Emptying them keeps every identification in the sweep and the
development set, lowers own-track hits by 17%, and widens the margin. I
kept it as a harness option (`--drop-fullest`) rather than changing the
default; see the decision above.

### Timings this night are noisy

The machine's load average stayed above 100 while the harness ran (swap
was still full after the 26,000-asset scan), so the same development scan
took between 25 and 72 s. I interleaved variants and report medians, and
counted lookups exactly where it mattered. Absolute times in experiments
0012-0017 should not be compared with the 22 s of experiment 0005.

### Verdict by ear: the Clockwork remix at 20:22 is shared material

`listen` at 20:30 of the development mix played the mix, Clockwork at 3:32
and the Stakka remix at 6:46 (its possible play). By ear: at that
moment the remix is the original's lead, an early neurofunk synth, exactly
as in the original, plus a pad; the rest of the remix is a different track
with its own bass and drums. The possible play is therefore a correct
report of shared material, acceptable as it is, and that fragment alone
cannot tell the two recordings apart. It is the same passage experiment
0015 found (129 hits, remix 6:39-6:58 against the original 3:25-3:44).

### Key lock on by default; its cost was 1.57×, not 1.3×

Our decision: both playbacks by default, so digital DJs get key-locked
plays without configuring anything, and vinyl users turn it off in the
configuration file (README, "Key lock and vinyl"). The NAS configuration sets
`playback = "turntable"`, so its searches are unchanged. The harness
default ladder is `both` too; turntable-only reports moved to
`work/reports/ladder-turntable/`. Measured in CPU time on the development
mix, both playbacks cost 261 s against 166 s, 1.57 times; the 1.3 I gave
before came from wall times on a loaded machine (experiment 0018). Same 14
plays, none with key lock.

Two smaller command-line changes went with it: `identify` keeps quiet
about timings and files left out of the index unless `--verbose` is passed
(files that need `gunfinger index` still get a one-line warning), and
saved reports record their playback, so `--save-dir` searches a recording
again when the playback differs.

## Session 3 findings

### Memory: the index build is fixed; the search is the next limit

The swap at 26,462 assets in session 2 came from building the index:
every peak record, a copy of every peak as a hashing point, and the
postings were in memory at once (5.0 GB at 8,122 assets, about 16 GB at
26,462). The index is now built in two passes that read one peak record
at a time (count, then fill), and `identify` keeps no records afterwards:
the build at 26,462 assets peaks at 3.1 GB, which is the index itself
(experiment 0019). Detections are identical. On the library, `identify`
of the development mix peaks at 369 MB instead of 395 MB; the cost is
reading the peak store twice (about 3% more CPU).

At scale the search phase is now what limits memory. A 10-minute query
at 8,122 assets peaks at 1.9 GB with 1 worker and 4.7 GB with 10: each
worker holds one speed rung's hits for a 10 s window, about 300 MB at
that size. On a 25,000-track library with 10 workers that would not fit
beside other applications. Dropping the most common hashes (experiments
0013, 0017) cuts the hits a query scans by 62% and is the obvious first
lever; fewer workers (`--jobs`) is the immediate workaround.

### Saved reports record how they were made

Each JSON report now records the peak profile, hash design, matching
settings, confidence rule and a library revision (a digest of the
indexed files' paths, sizes and modification times), as well as the
`--duration` asked for. `identify --save-dir` searches a recording again
when any of these, the playback, the part of the recording or the
recording's path differ. Reports saved before today record none of
this, so each will be searched once more. Reports are now written
through a temporary file and a rename: an interrupted run leaves the
previous report intact. (An interrupted write was already caught on the
next batch run, because a truncated report cannot be read; it was lost,
though.)

### `explain --windows` shows each window's evidence

`explain --windows` lists, per 10 s window, the lines of hits behind each
candidate and which candidate's chain took them. It shows something
worth knowing about the Clockwork remix at 20:22 in the development mix:
its possible play (96 hits in 3 windows) takes two strong windows on the
+1.2% turntable rung and, as its third window, a 4-hit line on a
key-locked rung at +5.2%. Chains may join lines from distant rungs, so a
weak chance line can supply the third window. It does not change any
confident result here, but the 3-window rule is less strict than it
looks.

### Brief plays: whether 15-20 s counts depends on the window grid

Generated mixes with exact truth (12 mixes, 115 plays of indexed tracks,
seed 2026) found every play: 107 confident, 8 possible, no wrong answer
(experiment 0020). All 8 possible plays last 20-26 s and span only 2 of
the 10 s windows; 7 have 265-490 hits, well above 200. Sliding the same
brief plays across the window grid in 1 s steps shows the cause: a 15 s
play is confident at 76 of 160 positions and a 20 s play at 148, and
every one of the 16 15 s plays changes verdict with the offset alone.

Requiring 10 s between the first and last aligned hit, in place of 3
windows, makes every 15 and 20 s play confident at every offset. It
changes nothing in the sweep or the development scans (no detection
there has 200 hits in fewer than 3 windows), and no false candidate in
the generated audio came near 200 hits (46 at most). It is a change to
the frozen rule, so it is only an offline comparison for now; adopting
it needs the full protocol and our judgement on whether a 15 s play
should count as an identification.

The grid also shows chains taking a chance 3-hit line up to 30 s away
(two empty windows are allowed between lines). That adds a window and up
to 30 s of span, and it moved the fitted speed by up to 2.4%. All the
confident 10 s plays in the grid come from this, almost all of one
track. Both rules count such lines. Requiring linked lines to come from
nearby rungs, or a stronger line across a gap, would close it; not yet
tried.

### At scale, search memory grows with the length of the mix

At 26,462 assets (the scale proxy) a 10-minute query peaks at 4.9 GB, of
which 3.0 GB is the index; a 5-minute query peaks at 4.1 GB, and a second
worker changes nothing (experiment 0021). What grows is what the search
keeps for the whole query, by inference the lines of every rung, since
chance lines multiply with the library. An hour-long mix at that size
would need about 14 GB. Fewer workers do not help there; the common-hash
filter and merging the rungs' lines as each rung finishes would.

### Where the development mix loses evidence

The 11 identified plays of the development mix keep 13% of the reference
hashes they could match. Clean renders of the same stretches at the same
speeds keep 48% on the rung the search used and 63% at the exact speed
(experiment 0022). The mix itself (the records, their mastering, the
recording of the mix and its AAC encoding) costs most, and most in the
top two octaves: 74% of reference hashes are anchored at 1-4 kHz, and the
mix keeps 6% of those at 2-4 kHz against 36% below 250 Hz. Blended
windows keep a quarter of what solo windows keep. Analysing a play at its
exact speed instead of the nearest rung adds only 5% here (a third for
the clean renders), so a second pass can do little for this mix.

A question for later, not this session: the peak picker spends most of
the hash budget where vinyl mixes keep least. Peaks spread more evenly
over the bands might keep more evidence, but that changes the profile and
means extracting all peaks again.

### Wow, broadcast processing and beatmatched blends

Wow at 0.55 and 0.75 Hz (±0.2%) costs nothing. FM-style broadcast
processing (two compressors and a limiter) loses about two thirds of the
hits; 32-33 of 40 excerpts stay confident and all but one are possible
or better. A beatmatched partner costs as much as an unmatched one.
Everything together (wow, a beatmatched partner at -6 dB, broadcast
processing, AAC at 64 kbit/s) leaves 3-4 of 40 confident and 32 of 40
possible. There is no wrong answer in any of these.

### Before adding tracks: fixed sweep panels and clusters without decoding

The sweep's held-out recordings and excerpts are now saved per seed in
`docs/panels/` the first time a seed is used, and reused after. Tracks
added later are indexed; new rips of a held-out recording stay held out.
Before this, adding tracks would have redrawn every excerpt while the
cached renders (named by number and speed) stayed, so the harness would
have scored old audio against new truth without noticing. `robust`,
`mixes`, `grid` and `hash-cost` use the same panels.

`gunfinger-eval clusters --from-peaks` searches each file's stored peaks
instead of decoding it. It finds the same 17 duplicate clusters as the
exhaustive search, with a seventh of the CPU, and on controlled cases (a
faster AAC rip, a crop, two edits, an A-B-C chain, a mash-up) both give
the designed clusters (experiment 0023). It compares with the clusters in
use and does not replace them. One property of the criterion to know
before adding edits: an edit that removes a section from the middle of a
track is not a duplicate of the original, because no single alignment
covers 80% of it.

### The second pass at the fitted speed works, under the frozen rule

`--second-pass` (harness only) analyses each candidate's span once more
at the speed its chain fitted and counts hits against that recording
alone (experiment 0024). Clean excerpts that sit between two rungs of the
speed ladder gain about 40% of hits and reach the level of excerpts that
sit on a rung; false candidates do not gain (strongest still 97 hits).
The confidence margin widens from 5.16× to 7.01×. Under the frozen rule
of 200 hits in 3 windows, recall rises under damage and falls nowhere:
needle skips 0-1 → 15-18 of 40, broadcast processing 32-33 → 37,
a beatmatched partner at -6 dB 33-34 → 36-37, an equal-level blend 13-14
→ 17; no wrong answer appears. The development set stays 11/11. It costs
about 8% more analysis.

Two limits. One fitted speed per play loses where the speed wanders
during the play: Star Trails in the development mix plays 0.1-0.3% above
its fitted speed in many windows and loses 15% of its hits (still
identified). And the second pass does not change how the 3-window rule
depends on the window grid; it does remove the chance lines that made
10 s plays confident.

### Four sweep draws show narrower margins than one

The sweep now runs on saved panels, and seeds 2027, 2028 and 2029 were
added to 2026 (experiment 0025). Every draw recalls all 540 indexed
excerpts with no wrong answer. Over the four draws the weakest
identifying detection is 403 hits (Fibre Optix - Sin at -3%, between two
rungs), so 200 sits 2.02 times below it instead of 2.5. The strongest hit
on audio that is not in the index is 28 (a held-out VIP excerpt aligned
with Muffler - Bleak), so the possible tier's 60 sits 2.14 times above it
instead of 3.16. Nothing crosses either threshold, but the safety factors
are thinner than a single draw suggested. Worth checking by ear:
whether `a-unknown-udfr014-(synthesis_vip)` and Muffler - Bleak share
material; if they do, the 28 is a near-version, not chance.

### The second pass with the common-hash filter: skip, do not drop

The package in item 6 is the second pass plus the fullest 1% of posting
lists set aside. I measured two forms of it (experiment 0026). *Drop*
empties those lists, as in session 2. *Skip* leaves them out only while
looking for candidates; the second pass still counts them. Over four sweep
seeds and the development scans both pass the protocol (every sweep
540/540, development 11/11, no wrong answer), and both cut false
candidates by 94% and the development scan's detections from 6,114 to
232. Drop also lowers own-track evidence to 72-96% of today's; skip keeps
the second pass's evidence (85-117% of today's).

The threshold comes from calibration data alone, by the rule in
`docs/calibration.md` (about twice the strongest false candidate, well
below the weakest identifying detection). The frozen 200 does not satisfy
it for skip: the Clockwork remix reaches 119 hits there, because with the
fullest lists skipped the first pass finds its shared passage (the one
confirmed by ear) as one chain, and the second pass counts all of it. The
opt-in second pass's rule is therefore now 240 hits in 3 windows (possible
tier unchanged at 60). It sits 2.0-2.6 times above the strongest false
candidate and 2.2-2.8 times below the weakest identifying detection for
all three second-pass variants. Default detection is unchanged (`regress`:
identical).

What 240 means for the test set is an inference, not a measurement: Sick
Note had 209 hits there. On the development mix the second pass changes
own hits by -15% to +17%, so Sick Note would most likely be shown as
possible, not confident, under any second-pass variant at 240. Only a
test-set evaluation would show it; I did not spend one.

### Under damage and at scale, skip at 240 does better than today

Every robust condition, with both ladders and with turntable alone, the
padded index and the cost (experiment 0027). With both ladders, 2,600
excerpts of indexed tracks: today's matcher confirms 1,959; skip at 240
confirms 1,995 and drop at 240 confirms 1,884. Possible or better: 2,258
today, 2,273 skip, 2,230 drop. No variant gives a wrong confident answer
anywhere; the strongest wrong candidate is 104 today, 108 with skip and
76 with drop. Skip gains on needle skips (1 → 31 of 80), beatmatched and
plain blends and broadcast processing, and loses where little evidence is
left: combined damage 7 → 4 of 80, pink noise at 10 dB 77 → 75, Opus
80 → 79. The turntable ladder shows the same pattern.

The filter pays at scale. On the padded index (8,122 assets) and the
scale proxy (26,462) both filtered variants use 57-63% less search CPU
than today's matcher, and the first pass keeps 112,000 distinct lines
instead of 1.74 million for 5 minutes of the development mix. At 262
tracks they save 2-10%. I could not measure peak memory reliably today:
the build used in experiment 0021 peaked at 6.5 GB on this machine, against
4.1 GB then.

## The case for adopting the second pass with common hashes skipped

What it is: `search_twice` on an index from `Index::skipping_fullest(0.01)`,
confident at 240 hits in 3 windows, possible at 60. Today it is a harness
option (`--second-pass --skip-fullest 0.01`); `identify` and `explain` do
not use it.

For it:

- Evidence is measured at each play's own speed, so the ladder's rungs no
  longer shape it: over four sweep draws the weakest identifying
  detection rises from 403 to 658 hits and the margin from 4.15× to 5.53×
  (experiment 0026).
- Under damage 36 more excerpts are confirmed with both ladders, needle
  skips most, and no wrong answer appears (experiment 0027).
- False candidates fall by 94% and the development scan's detections from
  6,114 to 232, so `explain` and the JSON report carry far less chance
  evidence.
- At 8,000-26,000 assets the search needs 57-63% less CPU and keeps 94%
  fewer lines, which is the memory the search holds at that size.

Against it:

- The confident threshold rises from 200 to 240, set by shared material:
  the Clockwork remix's shared passage measures 119 hits. Plays between
  200 and 240 hits after the second pass become possible: combined damage
  7 → 4 of 80. The generated mixes and the window grid do not change.
- Sick Note had 209 hits on the test set; it would most likely be shown as
  possible (inference from the development mix, where own hits change by
  -15% to +17%). A test-set evaluation (3 left) would show it.
- One fitted speed per play: a play whose speed wanders loses evidence
  (Star Trails, -15%).
- At 262 tracks it saves little time; most of the gain is in margin and
  robustness.

Dropping the lists instead (`--drop-fullest`) is not worth it: it keeps
72-96% of today's own-track evidence and confirms 75 fewer damaged
excerpts at 240.

My recommendation, for us to decide: adopt skip with the second pass
before the library grows past a few thousand tracks, after one test-set
evaluation to see Sick Note and the rest of the test mix under it. If
losing confident status for plays near 200-240 hits is not acceptable,
keep today's matcher and revisit when the confidence statistic for shared
material (roadmap) is solved, since that is what forced 240.

## Session 4 findings

### Idle-machine reference: where the time goes, and what scale costs

Measured without load this time (experiment 0028; three rounds each,
wall times within 2%). The development scan (56 minutes) takes 34 s at
262 assets, 4.4 minutes at 8,122, 12.8 at 26,462 and 15.2 at 31,964
with today's matcher; skip at 240 takes 31 s, 1.7, 4.0 and 4.7 minutes.
Search CPU grows by about 0.21 s per asset with today's matcher and 0.055
s with skip at 240, which is 3.5× cheaper at 31,964 assets. Peaks: 11.2
GB with today's matcher and 5.3 GB with skip at 31,964 assets, before
item 2's change.

Where the time goes depends on size. At 262 assets, 80-88% of the CPU
analyses the query on 82 rungs (STFT and peak picking); sorting each
window's hits is second (7-17%). At 26,462 assets sorting the hits is
77-87% of the CPU and analysis 4-12%; looking up postings is 2%. Every
window collects millions of hits (up to 201 MB per worker), and they are
sorted by asset and offset to find the lines. Any correct sort gives the
same lines, so a faster one (grouping by asset first, then sorting each
asset's few hundred hits) would keep detections identical; not tried.
Indexing takes 0.2 s per track at 10 workers: about 67 minutes for
20,000 tracks on a local disk (inference).

### Search memory at scale: lines, hit buffers and the allocator's cache

On an idle machine, scanning the 56-minute development mix at 26,462
assets (the scale proxy) with 10 workers peaks at 9.8 GB with today's
code (three rounds, within 1 MB of each other), 4.4 GB with skip at 240.
What it holds: the index (3.0 GB); each worker's hits for one window (up
to 201 MB per worker); every rung's lines until chains are built (12.1
million lines, 579 MB, per 10 minutes of query; 7.3 million after
merging); and freed memory: each rung grew its own hit buffer and freed
it, and macOS keeps freed large blocks cached. With that cache off
(`MallocLargeCache=0`) the same scan peaks at 6.8 GB at the same CPU.
On 10 minutes of query the cache was most of the peak (15.7 GB against
5.0 GB). So experiment 0021's inference (lines of every rung, about 11
GB for an hour-long mix) had the right ingredient and the wrong size.

Item 2 (commit db43197) searches a block of 12 windows on every rung at
a time, merges each block's lines once every rung has searched it, and
gives each worker one hit buffer. Detections are identical (`regress`,
and every scale report). Peaks fall from 5.3 to 2.9 GB at 8,122 assets
and from 9.8 to 7.7 GB at 26,462 (skip at 240: 2.1 to 1.4 GB, 4.4 to 3.8
GB); wall time falls 5-12% at under 1% more CPU (experiment 0029). What
is left at 26,462 assets is the index (3.0 GB), the merged lines of the
whole mix (29.3 million, 1.4 GB), the workers' hit buffers (about 2 GB)
and the final sort of the lines. Two further identical changes would take
off about 1.2 GB (16-byte hits, and sorting the lines without a half-size
copy); not done.

### Chance lines in chains: two link rules, case for adoption

Two opt-in rules (commit 8847dcb; experiment 0030): `--nearby-rungs`
links only lines within 0.6% of speed (1.5 steps), and `--strong-gaps`
links across an empty window only lines of 10 hits or more. On the
Clockwork remix at 20:22 the chain's third window was a 3-hit key-locked
line at -8.00% beside lines at +1.2-1.6%; with the rules the detection is
102 hits in 2 windows instead of 105 in 3.

Under today's matcher the rules change no sweep result (2,160/2,160,
weakest identifying 403, margin 4.15×), cut false candidates by 68% and
the mixes' boundary overshoot from 86 to 7.6 s. One mix play becomes
possible: mix 10's Dominion (21.5 s, 537 hits in 2 windows), which was
confident only through a chance line 23 s earlier. The grid's 10 s plays
(8 confident, all through chance windows) become possible, and mean
spans now match the true lengths. Today's matcher with the rules gives
what skip at 240 already gives on mixes and grid. Under skip at 240 the
rules raise the margin from 5.53× to 6.65× (the remix's 119-hit chain
becomes 94 hits) and change nothing else.

For adoption: no recall is lost except confidence that came from chance;
boundaries are right; chaining is under 0.1% of the CPU, so the rules
cost nothing measurable. Against: a speed change of more than 0.6% between
windows splits a play into segments (Star Trails at 13:20, +4.5% to
+3.1%: two confident segments of one play in the report). My
recommendation, for us to decide: adopt both rules with whichever
matcher we choose; with skip at 240 they add margin at no cost.

### Speed per stretch: more hits where speed drifts, no level changes

`--speed-per-stretch` (commit 8847dcb; experiment 0032) lets the second
pass fit a speed for each stretch of 3 windows and measure it again when
that speed is 0.05% or more from the play's. It exists only with the
second pass, so it was measured under skip at 240. Star Trails gains 11%
(1,558 to 1,733 hits) and the robust pitch ride (2% over 30 s) 9%; wow
(±0.2% at 0.55 and 0.75 Hz) is too fast for a 30 s stretch and is
unchanged, as are blends and beatmatched partners. Some plays lose a
little (Dominion -3.2%, Pathogen -1.3%) because the new measurement
replaces the old one, and false candidates gain too (strongest in the
mixes 42 to 50, in sweep 2026 99 to 109). It costs no measurable CPU on
the development scan (241 s either way, three rounds). Nothing changes
level in the sweeps, development scans, mixes or grid.

For adoption: it recovers evidence exactly where the brief expected, for
free. Against: no detection in our sets needs it, and it lifts the
strongest false candidates by 10-19%. My recommendation:
keep it opt-in, and look again if a real play near the 240-hit rule
turns out to be a slow pitch ride.

### Peaks across bands: a third more evidence in the mix, and more shared material

On the development mix's 11 identified plays (390 windows, the true rung
only), `loss` now counts the postings looked up beside the true hits, by
the anchor's band (commit 718d861). Anchors at 2-4 kHz account for 64%
of the postings looked up and 21% of the true hits: 0.65 hits per 1,000
postings. Anchors at 125-250 Hz give 18.05 per 1,000, at 250-500 Hz
13.09, at 500-1,000 Hz 8.16, at 1-2 kHz 2.64, and below 125 Hz 3.19
(kicks are common, so their lists are long). Below 1 kHz, 15% of the
postings give 52% of the hits. The mix keeps 45-47% of a clean render's
hashes below 250 Hz and 11% at 2-4 kHz.

Two variant profiles (experiment 0034) widen each peak's neighbourhood
in proportion to its frequency, 9.4% of the bin either way (12 bins at
1 kHz, as today), within 4-24 bins (a) or 4-16 (b), so the picker keeps
more peaks below 1 kHz and fewer above. Each library was extracted into
its own store under `work/variants/`; `work/peaks` is untouched. Both
pass the full protocol under both matchers: sweeps 2,160/2,160,
development 11/11, leave-outs and mixes 0 wrong, the same mix levels.

Variant a has 9% fewer postings. The development mix keeps 19.0% of the
reference hashes instead of 13.0%, and its plays look up 36% fewer
postings for 34% more true hits (32% under skip at 240; Star Trails
+72%, every track at least +11%). Over four seeds the weakest
identifying detection rises from 403 to 511 hits under today's matcher
(skip at 240: 658 either way) and the margin from 4.15× to 4.96× (5.53×
to 5.82×). Robust recall on clean excerpts is level (1,955 against 1,959
confident; under skip 1,993 against 1,995); a loses on blends at equal
level and gains on codecs and low-passes. Variant b keeps more high
peaks: 6% more postings, 41% more true hits, the best robust recall
(1,991 confident), but a stronger Clockwork remix (128 hits; 135 under
skip), so smaller margins (4.31×, 5.64×).

The cost is shared material. False candidates of 30 hits or more rise
from 55 to 94 under today's matcher (41 to 65 under skip) with variant
a; nearly all are remixes, VIPs and records of the same artist
(Aphrodite, Kemal, Stakka & Skynet). The strongest on audio not in the
index is a held-out Bad Company - China Cup finding The Nine: 28 to 43
hits (24 to 50 under skip), near the possible tier's 60. Fewer postings
looked up should also make search cheaper at scale; not measured, since
the machine had other load by then (inference).

For adoption (variant a): the most evidence per posting, in the real
mix, where it matters; better margins under both matchers. Against:
every peak record must be extracted again (about 0.2 s per track), the
gain rests on one development mix, and related records reach the
possible tier more easily. My recommendation, for us to decide: adopt
variant a after choosing the matcher, confirmed by one test-set
evaluation, and listen to whether China Cup and The Nine share material.

### Trimming weak chain ends costs more true play than it removes

`--trim-ends` (commit 8847dcb; experiment 0031) leaves lines at either
end of a chain with under a quarter of the chain's median hits out of
the boundaries. Detection levels are unchanged under both matchers. On
the generated mixes' exact boundaries, under today's matcher, it removes
84.7 s of overshoot from 6 plays (86.0 to 1.3 s) and cuts 218.8 s of
true play from 72 of 115 plays (median 3.1 s, largest 8.2 s): a play's
first and last windows are often partial or under a crossfade, so they
are as weak as a chance line (inference). Under skip at 240 there was
only 7.6 s of overshoot to remove, and trimming costs 221 s the same way.
The link rules (0030) remove 78.4 s of overshoot at 22.2 s of true play.
My recommendation: do not adopt trimming.

### The edge of the ladder: three more rungs cover ±9% for 14% more CPU

Experiment 0033. With today's ladders (±8%) recall stays 40 of 40 to
±8.4%, then falls: today's matcher has 39, 12 and 0 confident at -8.6%,
-8.8% and -9% (+8.8%: 35); skip at 240 holds one step further (35 and 7
at -8.8% and -9%). `--extra-rungs 3` adds three rungs at each end of both
ladders (to ±9.2%, 94 rungs instead of 82): every edge speed is then 40
of 40 under both matchers, with no wrong answer. It costs 14% more
search CPU (development scan, three rounds: 268 to 306 s today's, 243 to
278 s skip) and 8-11% more false candidates, all far below the rule;
sweeps 2026-2029, margins, the development scans, mixes and grid are
otherwise unchanged.

For adoption: cheap insurance if a deck's fader goes past 8% (the
classic Technics SL-1200 stops at ±8%; some decks and CDJs offer wider
ranges). Against: 14% CPU for plays that may never happen in the mixes
searched. My recommendation: adopt it only if the decks go past ±8%.

## After session 4

### Verdict by ear: China Cup and The Nine share their drums

Asked in session 4 (experiment 0034: with the band profile, a held-out
Bad Company - China Cup finds The Nine with 43-50 hits). By ear:
they are definitely different tracks, but mixed into each other they are
so alike that a listener who knows neither may not notice the change.
They have the same rhythmic structure and a very similar snare, probably
the same drum loop, or at least the same snare sample EQ'd differently.

Reported, not checked here: producer forum threads (Dogs On Acid,
2002-2012) say the two use the same drums, assembled from layered and
resampled hits rather than a known break, and that Fresh & Vegas -
Mekon, by two of Bad Company's members, has them too.

So the 24-51 hits between them (experiment 0034, across profiles and
matchers) are shared material, not chance, as with the Clockwork remix
at 20:22: the kind of evidence the threshold must stay above, and the
kind the band profile makes stronger. They stay
separate recordings (no manifest or cluster change). Both are in the
test mix (The Nine at 2:23, China Cup at 1:02:23), where a play of one
reported during the other at the possible tier would be shared material.
Step 5 of `docs/nas-plan.md` measures the pair at NAS scale, with Mekon
if the collection holds it.

### Identifying without the library; evaluating against the NAS

Merged after session 4 (brief `docs/brief-nas.md`, checklist
`docs/nas-checklist.md`). The peak store keeps each file's artist, title
and album beside its record, reports carry them, and `identify`,
`explain` and `stats` search the store's own records when no library is
given or it cannot be read (`--store-only` forces it), so a copy of the
store identifies and names tracks on another computer. The harness takes
`--other-peaks-dir` with the NAS store: `map-library` pairs corpus files
with their NAS copies, `clusters --from-peaks` finds the NAS's other rips
of the corpus recordings, and the standard evaluation then runs at NAS
scale, counting those rips as correct. `regress session-4-start`:
identical. The analysis (`docs/nas-plan.md`) waits for the NAS index;
its records predate tags, which a second `gunfinger index` fills in from
the files' headers.

## Session 5 findings

Measurements at the size of the NAS collection, from its peak store
alone (`docs/brief-5.md`). The NAS was never read and the store was not
written.

### The NAS index run was limited by reading the files, not by the CPU

The index run read 181 GB of audio in 17,317 s: 84 Mbit/s, 6.2 s per hour of
audio. The local disk run of experiment 0028 took 1.8 s per hour of
audio, so the CPU would have finished the NAS in about 84 minutes
instead of 4.8 hours (inference: it assumes AAC and Opus decode as
cheaply as MP3). Peaks and postings per second equal the corpus's
(37.5 and 74.8 per second), so the front end treats the YouTube AAC and
Opus files like the scene MP3s. Tags cover 99.3% of records; 144 have
none, 125 of them in the scene folders. Experiment 0035.

### 15 corpus files have no copy on the NAS

To check manually (experiment 0036). `map-library` finds an identical
record for 247 of the 262 corpus files. Eleven files at the corpus
library's root have no record and no skip note on the NAS, so the NAS
dnb library seems not to hold them: Ed Rush & Optical - Compound, Dozer,
Fixation, Glass Eye, Lithosphere, Mystery Machine, Point Blank and
Wormhole; Fortran - Splinter; Optical - Millennium and Slip Thru. The
NAS has other rips of Millennium and Slip Thru (`dfect-dnb`) and remixes
of several. Four more have NAS copies that failed to open with "Invalid
data found when processing input", probably among the 27 MP3s inside a
WAV container: SKC & Cord - Swarm, SKC - Recharger (Black Sun Empire
remix), Usual Suspects - Bleach, Cause 4 Concern - Give It 2 Em. The
corpus copies decode, so they may be repaired versions. Dozer, Fixation
and Slip Thru are test-set references: the NAS-scale evaluation keeps
these 15 as corpus files. The 247 copies are 238 NAS records, since 9
stand for two corpus rips each, so the index holds 26,914 assets.

### The corpus clusters reappear on the NAS; 170 further rips

`clusters --from-peaks` against the NAS records (experiment 0037, 72
minutes) finds the 17 corpus clusters exactly, and 170 NAS files that
are further rips of corpus recordings (mostly YouTube uploads). The
evaluation counts their detections as correct.

The 80% rule has less room than at 262 tracks: rips of one recording now
cover as little as 82.5% of each other, and two files the rule calls
different reach 73.7% (at 262 tracks: 98.4% and 39%). To check
by ear, the 76 pairs between 40% and 80% coverage, in
`docs/experiments/data/0037-nas-clusters.json` (`borderline`). Many have
the same artist and title, such as Bad Company - China Cup against the
alien5ive upload "BAD COMPANY - china cup" (68%, 3,219 hits) and Night
Lore against "STAKKA and SKYNET - knight lore" (68%). If they are rips,
the rule keeps them apart, so a mix detection of one during the other
counts as wrong; I expect such cases in the development scan below.

Tags that disagree with the audio, found by content: the INFRA011 rips
tagged "Sex Drive" and "Specialist" hold Future Cut - The Specialist and
Razor's Edge; the alien5ive upload tagged "Mindscape - New Deal" holds
Noisia & Phace - Outsource (Misanthrop Remix); the i-witness upload
tagged "Future Cut - Horns 2000 (Dylan Remix)" holds DJ Ink - Ice Age
(Digital & Spirit remix); the untagged `b-unknown-udfr014` matches an
upload tagged "Profound Noize - Luminous Remix". Two clusters hold
tracks of the mixed CD Underfire UDFRCD003 (Dangerous Drums Volume 2),
which probably carry blends with their neighbours.

### Related recordings at NAS scale: shared material, rarely unrelated

From the same search (experiment 0038): 292 pairs of different
recordings share 30 hits or more (21 at 262 tracks), 63 reach the
possible tier and 22 reach 200. Of those 22, by tags and paths (not by
ear), 16 are probably one recording under another name or credit, 4 are
versions (The Nine against the Evol Intent VIP: 4,930 hits), and 2 are
one artist's other title, to check manually: Bad Company - Coma against
Spraycan (2,318 hits over 229 s, one release folder) and Pyro - Time Is
Broken against Ignorance (858). The strongest unrelated pair is Stakka
& K.Tee - Synthesis VIP against Dillinja - No Way Out (Remix), 153 hits
in 40 s: a shared sample, perhaps.

China Cup and The Nine do not reach 30 hits against each other in this
search. The NAS holds Fresh & Vegas - "Meekon" (spelled so in its tags,
`dfect-dnb`); The Nine finds it with 31-33 hits.

### At NAS scale the development mix has 6 wrong identifications: faster uploads of the tracks played

To check by ear. Under the rules every confident detection the
clusters do not explain counts as wrong. The development scan at NAS
scale with skip at 240 identifies 11 of 11 and has 6 such detections,
each at the same time as the true play, of a YouTube upload of the same
track (the full table with today's matcher follows in experiment 0039):

| Mix time | Upload (NAS path under `__youtube_archivists/`) | Hits | Faster than the corpus rip |
|---|---|--:|--:|
| 0:00-5:02 | `alien5ive/UNDERFIRE RECORDINGS [ UDFR 016 ： STAKKA and SKYNET - knight lore - ] drum and bass.m4a` | 3,383 | 2.95% |
| 0:00-5:03 | `i-witness-dnb/Stakka & Skynet - Nightlore.m4a` | 2,687 | 3.44% |
| 9:43-13:59 | `alien5ive/AUDIO BLUEPRINT [ ABPR013 ： KEMAL & ROB DATA - star trails - ] drum and bass.opus` | 1,786 | 2.94% |
| 14:27-18:04 | `alien5ive/AUDIO BLUEPRINT RECORDINGS [ ABPR 014 ： SKYNET & STAKKA - pathogen - ] drum and bass.m4a` | 1,350 | 3.94% |
| 31:25-38:02 | `alien5ive/UNDERFIRE RECORDINGS [ udfr017 ： KEMAL & PAUL RESET - kontempt - ] drum and bass.m4a` | 3,902 | 4.67% |
| 43:22-49:57 | `alien5ive/AUDIO BLUEPRINT [ ABPR012 ： SKYNET & STAKKA - logistics - ] drum and bass.opus` | 5,845 | 2.97% |

"Faster" is the ratio of the two detections' fitted speeds (an
inference: the uploads would play 3-4.7% fast, perhaps sped up when they
were uploaded). The clusters search rips within 0.98-1.02 of each other,
so a rip 3% fast cannot join its recording's cluster, and the 80% rule
never sees it. If these are confirmed as the same recordings, they are
correct identifications: the clustering ladder, not the matcher, is
what calls them wrong. Widening the clustering ladder to the full ±8% of
the speed ladder would be the general fix (not done: ground truth stays
fixed this session).

### The development mix at NAS scale: every track found, chance at 40-48 hits

Experiment 0039. Both matchers identify 11 of 11 against the 26,914
assets, with the same weakest evidence as at 262 tracks (1,839 hits
today, 1,557 with skip at 240); leave-outs 3 and 11 find nothing
confident in the left-out slots besides the 6 fast uploads above, which
appear in every scan. Without those 6, the strongest false candidates
are the shared material already known (the Stakka remix of Clockwork,
94 and 119 hits) and the Synergy remix of Star Trails (75-79 hits, a
new possible play at 10:27). Unrelated chance rises from 21-24 hits to
40 (Heretik - Biodome at 39:12, today's) and 48 (Genetix - Crunch at
6:11, skip at 240), below the possible tier of 60. Cost: today's matcher
864.6 s, 6,097 s CPU, 8.37 GB; skip at 240 260.6 s, 1,963 s CPU, 4.01
GB: 3.3 times faster at half the memory.

### Skip at 240 over four sweep draws at NAS scale: recall holds, the rule-level margin is gone

Experiment 0040. All 2,160 indexed excerpts are found (weakest 661 hits,
658 at 262 tracks), but 523 confident detections are false under the
rules. 507 of them are other uploads of the excerpted track with the
same artist and title (47 pairs, by tags and paths; one is even titled
"Falcon - The Stand (HD Audio & Speed Up)"). None appears at -8%, which
fits uploads about 3% fast. To check by ear, the two pairs that
are not:

- `extra/a-unknown-udfr014-(synthesis_vip)-sour.mp3` (Stakka & K.Tee -
  Synthesis VIP) against `Underfire UDFRCD003 - Dangerous Drums Volume 2
  (2000)/CD1/02-Stakka_And_K_Tee-Synthesis_(Remix)-sour.mp3`, up to 511
  hits: perhaps the same remix under two names.
- `extra/02 Coma.mp3` (Bad Company - Coma, from `(2000) Coma & Spraycan
  (DSCI4)`) against `__youtube_archivists/i-witness-dnb/Bad Company -
  Spraycan.opus`, up to 341 hits; the NAS's own Coma and Spraycan share
  2,318 hits over 229 s (experiment 0038), so one file is probably
  mislabelled.

If both pairs are confirmed as one recording each, the strongest false
candidate is The Nine against its Evol Intent VIP (220 hits) and the
margin 3.0×; at 262 tracks it was 5.53×. Unrelated chance (different
artist and title) reaches 67-70 hits, above the possible tier of 60.

### Today's matcher over two sweep draws at NAS scale: the same picture

Experiment 0041. Seeds 2026 and 2027 find all 1,080 indexed excerpts,
with the same weakest evidence as at 262 tracks (403 hits, Fibre Optix -
Sin at -3%). Of 285 confident false detections, 276 are other uploads
with the same artist and title, 8 are Coma ~ Spraycan again (342 hits),
and one is new, to check by ear: `extra/A- The_Nine.mp3` (Bad
Company - The Nine) against `__youtube_archivists/i-witness-dnb/Bad
Company - The Nine (Evol Intent VIP).m4a`, 200 hits in 3 windows at +5%,
exactly at the rule (skip at 240 keeps it out at 220). The margin is
1.18× without the same-name pairs and 3.20× without the two pairs as
well (Ant Miles - China Town ~ Sea Of Chaos, 126). Unrelated chance
reaches 78 hits (Clockwork ~ Simon Static - Rubba Rock). Each seed takes
about 66-69 minutes and 36,000-38,000 s of CPU, three times skip at 240.

### The scale proxy predicted cost well and chance badly

Experiment 0042. The 100 reversed copies of the corpus (26,462 assets)
had 760 million postings; the NAS index has 756 million. At NAS scale
the development mix takes 1.18 times the proxy's wall time under today's
matcher (848 s), 1.05 times its CPU and 1.10 times its memory (8.4 GB);
under skip at 240, 1.13, 1.14 and 1.06 (259 s, 4.0 GB). Chance is
another matter: unrelated real tracks reach 1.3-2.7 times the hits of
the copies, and every wrong identification at NAS scale is another rip
or upload of the track played, which a reversed copy cannot be. Plan
with the proxy for time and memory, not for margins.

### The fullest posting lists are spread over the whole collection

Experiment 0043. The 1% of lists that skip at 240 sets aside hold 18.7%
of the postings, and a typical record has 18.6% of its postings there
(at most 34%, Axis Of Evil - Ignition Sequence). No small set of
"famous break" records holds them: the top 1% of records hold 1.9% of
those postings. Under today's matcher, records with more postings in
those lists are false candidates more often (rank correlation 0.72);
skip at 240 removes most of that (0.24) and keeps 4.3% of the false
candidates, while the heaviest records keep their own hits when played.

### The store alone identifies the mix as the harness does

Experiment 0044. `identify --store-only` against the NAS store (internal
disk; no external disk was attached) builds a 3.0 GB index in 44 s and
identifies the development mix in 856 s with 8.35 GB, with the store's
revision `d709390272e41475` in the report. Its 30 plays have exactly the
harness's hits. The tracklist has 20 lines for 11 tracks and 2 possible
plays: a track found on several uploads appears once per tag spelling
("STAKKA and SKYNET - knight lore" and "Stakka And Skynet - Nightlore").
Grouping plays that cover the same mix time would list each track once;
I have not implemented it.

### Correction: the NAS-scale index holds 26,914 assets

Experiments 0036 and 0039 said 26,905. The 247 corpus files with a copy
are copies of 238 NAS records (9 records stand for two corpus rips
each), so the index holds 262 + 26,890 - 238 = 26,914 assets, as
`memory` reports. The earlier notes are corrected.

### The link rules at NAS scale: fewer chance lines, one split under today's matcher

Experiment 0045. Under skip at 240 the link rules change no level and
no answer at NAS scale: the development scan keeps 11/11 with the same 6
wrong uploads, sweep seed 2026 keeps 540/540 with the same 91 wrong
answers and weakest 680, and false candidates halve (66% fewer in the
mix; unrelated chance 48 → 32 there). Under today's matcher they remove
80% of the mix's chance detections but split Star Trails at 13:20 into
two segments, so the alien5ive upload counts as wrong twice (7 wrong).
The case for adopting them together with skip at 240 holds.

## Session 6 findings

Session 6 (`docs/brief-6.md`): the store fixes, skip at 240 with the link
rules as the default matcher, clusters that join fast uploads, the
NAS-scale protocol under both, and grouped tracklists.

### Store fixes: three corpus files would now be indexable; not indexed

`index` now reads an MP3 inside a WAV (RIFF) container behind an ID3v2
tag with the tag skipped (`-skip_initial_bytes`), and takes the file's
artist and title from the tag (`ffprobe -f mp3`); passes over folders
whose name starts with a dot ("hidden folder" in the skipped files); and
accepts audio that stops short of its declared length by up to the
larger of 1 s and 1% of that length. Probes of the 8 NAS files in
question (read-only): the 6 damaged at the end fall 1.27-2.38 s short
against tolerances of 2.92-4.91 s and now pass; the 2 truncated files
fall 16.18 and 21.19 s short against 4.09 and 3.32 s and still fail.

Worth knowing: three corpus files failed in session 1 under the 1 s
tolerance and would pass now: `extra/03-Profound_Noize-Dropzone-sour.mp3`
(1.3 s of 445.1 s missing), `extra/kosheen [resist] -02- hide u.mp3` (1.4
of 292.4) and `extra/rawkuts-gridlok-rkt337-2001-sour/a-gridlok-dilusion-sour.mp3`
(1.3 of 459.2). The rules of this session forbid re-extracting
`work/peaks`, so they stay unindexed there (their skip notes remain), and
the 262-track results do not change. A plain `gunfinger index` of the
corpus would not retry them either (the skip notes still apply);
`--retry-skipped` would. Their NAS copies are among the six that item 3
indexes, so at NAS scale they are searchable as NAS records.
No corpus file is an MP3 inside a WAV container (checked: none of the 219
corpus files with an ID3v2 tag has "RIFF" after it), and the corpus has
no hidden folders, so neither fix touches the corpus.

### The NAS store re-indexed: 32 more records, frozen at revision d428ee9585936326

Experiment 0046: the one `index --retry-skipped` of the session added
32 records (26 MP3s in a WAV container and the 6 files damaged at the
end); the two truncated files still fail, as intended, and `prune
--yes` removed only the stale note of the `.incomplete` download. Two
things worth knowing:

- `sety/-_MaYdaY_-15.Grudzien.2000...RADIO_LODZ_NOCNY_TRANS.mp3` was one
  of the 27 RIFF files; it opens now and is a radio show longer than
  15:00, so it has a "too long" note instead of a record.
- Six of the RIFF files have no artist or title after indexing (Drum
  Kru - Thin Air and Poltergeist, Cause 4 Concern - Give It 2 Em, RAM
  Trilogy - Incarnate, Reflections and Titan); tracklists show their
  file names. I did not look further, since the NAS was read only by
  `index` and `prune`.

The four corpus files whose NAS copies were among the RIFF files still
have no identical NAS record: the NAS files now decode, but not to the
same peaks as the corpus copies (which are plain MP3s).

### The fitted matcher is the default; every change at 262 tracks was predicted

`identify`, `explain` and the harness now skip the fullest 1% of posting
lists while looking for candidates, link chains by the two rules and
measure each candidate again at its fitted speed, with 240 hits in 3
windows for confident and 60 for possible (ADR 0008). `--single-pass`
gives the matcher before (200 hits). Experiment 0047 ran the full
protocol: the candidate baseline is reproduced exactly; sweeps 540/540
for seeds 2026-2029 with 0 wrong; development 11/11, leave-outs 0 wrong;
margin 6.65× (single pass 4.15×). True plays have 0.84-1.16 times the
hits; the development scan's detections matching no track fall from
6,096 to 69. Two kinds of play lose their confident level, both through
a chance window before: mix 10's Dominion (21.5 s, now possible, 525
hits in 2 windows) and the grid's 10 s plays (8 → 0 confident). Star
Trails splits at 13:20, where the DJ changes speed.

One behaviour is new and only seen in the synthetic end-to-end mix: after
a needle skip of 4 s into a track whose kick repeats every bar, the
second pass, which measures up to 10 s before a chain, takes in the
kicks before the skip, and the part before the skip is dropped as
overlapped. The play stays confident; its start moves from 1:36 to 1:45.
A real skip would only do this where the drums before and after it are
identical.

Saved reports record the matcher, so `identify --save-dir` searches each
mix again once. Sick Note in the test mix (209 hits under the single pass)
would most likely become possible (inference from experiment 0026). The
next test-set evaluation, with three left, should measure this matcher
on the test mix with both playbacks: 15/16 and Sin possible under the
single pass.

### A store's library path is compared without resolving symlinks

Found while testing item 4 from a copy of the tree under `/tmp`, a
symlink to `/private/tmp` on macOS: the end-to-end test
`a_copy_of_the_peak_store_names_tracks_without_the_library` fails there
because the store records the library's resolved path and
`identify --library` is compared with it as given ("the peak store at
... holds the records of the library at /private/tmp/..., not
/tmp/..."). Both name one folder. It passes in the repository, whose
path has no symlink. Comparing the resolved paths would fix it; not
changed this session.

### The 6 fast uploads of the development mix: the clustering ladder, not edits

Experiment 0048 searched each corpus rip against its upload alone. The
uploads play 3.0-4.6% faster than the rips. Session 5's clustering
ladder stopped at 0.98, 0.9-2.6% from the pairs' speeds: three pairs kept
44-55% of the track in pieces at one offset, the other three almost
nothing. On the full turntable ladder each pair is one alignment over
the whole of both files (4,142-6,153 hits), at the fitted speed alone
too; key lock adds nothing, and nothing points to a cut or an edit.
`clusters` now searches the turntable ladder from 0.92 to 1.08, measures
each candidate pair again at its fitted speed, and counts coverage in
the shorter file's own seconds (an upload 4% fast is 4% shorter).

### The clusters now join fast uploads; 92% of session 5's false confident detections were rips

`clusters` searches the turntable ladder from 0.92 to 1.08 and keeps the
better of two alignments per pair: the one on the ladder and the pair
measured again alone at its fitted speed (experiment 0049). At 262
tracks the 17 clusters are unchanged and the gap widened (the strongest
different pair falls from 0.385 to 0.302 coverage, two Aphrodite records
with shared breaks: the old pairs above 30% were chance). At NAS scale the 17 corpus clusters reappear; 46 more
NAS files join, nearly all uploads 3-7% off their rips' speed, so 732
files form the 233 clusters, and the borderline pairs fall from 76 to 28.
All 6 uploads that were the development mix's wrong identifications now
join their recordings.

Re-scored with the new clusters, session 5's reports explain 465 of the
505 confident false sweep detections under skip at 240 and 242 of 267
under the single pass. What remains is four pairs, to judge by
ear (in the listening list below, after item 6).

The gap is narrow: the weakest same-recording pair covers 0.840 (the
i-witness upload of Phoenix), the strongest different pair 0.766 (the
INFRA011 rip tagged "Sex Drive", which holds The Specialist, against
the alien5ive INFRA012 upload: probably the same recording with a cut).
One join to check: "DJ Trace - Sonar (Mark System Revision)" (i-witness)
joins DJ Trace - Sonar at 91% coverage but only 246 hits; if the
revision is another master or mix, mark the pair `different`.

`docs/pair-verdicts.txt` takes the verdicts by ear: one line per pair, `same`
or `different`, then the two paths, separated by tabs (format in its
header). `clusters` follows a verdict over the 80% rule; rerun
`clusters --from-peaks` with `--other-peaks-dir` after editing it (85
minutes for the NAS).

### At NAS scale the development mix has no wrong identification; the margin comes back

Experiment 0051 ran the NAS-scale protocol under the new default and
the new clusters. The development scan identifies 11 of 11 with 0
wrong (session 5: 6), leave-outs 3 and 11 have 0 wrong, sweep 2026
recalls 540 of 540 with 0 wrong (91), sweep 2027 540 of 540 with 24
wrong (174). Calibrate's margin is 1.64× under the rules (session 5:
0.11×); the strongest false candidate is now a borderline pair, not an
unclustered upload. Without China Cup's Prototype upload it is 1.97×,
without Coma ~ Spraycan as well 3.41× (The Nine's VIP, 197 hits on
audio not in the index). Unrelated chance stays at 63-70 hits. Costs
match session 5: the development scan 255 s and 1,950 s CPU, a sweep
1,300 s and 12,100 s CPU.

### Listening list after session 6

Pairs the clusters keep apart, by ear. `same` or `different` for any of
them goes into `docs/pair-verdicts.txt`. The first three cause every
wrong answer left in the NAS-scale protocol (experiment 0051).

| Pair (tags; path under the NAS root, `second-library/` left out) | Coverage | Confident false; strongest; where |
|---|--:|---|
| Bad Company - China Cup (corpus `Bad Company - China Cup.mp3`, `extra/02. Bad Company - China Cup.mp3`) ~ "BAD COMPANY - china cup" (`__youtube_archivists/alien5ive/PROTOTYPE RECORDINGS [ PRO 001 UK ： BAD COMPANY - china cup - ] drum and bass.opus`) | 65% | 16; 408 hits; sweep 2027, excerpts at 2:52 and 2:37 |
| Bad Company - Coma (`extra/02 Coma.mp3`) ~ Bad Company - Spraycan (`__youtube_archivists/i-witness-dnb/Bad Company - Spraycan.opus`) | 58% | 8; 341 hits; sweep 2027, excerpt at 4:16 |
| Bad Company - The Nine (corpus rips) ~ Bad Company - The Nine (Evol Intent VIP) (`__youtube_archivists/i-witness-dnb/...`) | 48-50% | 0; 197 hits (possible) |
| Stakka + K.Tee - Synthesis VIP (`extra/a-unknown-udfr014-(synthesis_vip)-sour.mp3`) ~ "Synthesis (Remix)" on the mixed CD (`Underfire UDFRCD003 - Dangerous Drums Volume 2 (2000)/CD1/02-Stakka_And_K_Tee-Synthesis_(Remix)-sour.mp3`) | 59% | 0; 115 hits (possible) |
| Future Cut - The Specialist (corpus `extra/01-future_cut-the_specialist-trt.mp3`; the INFRA011 rip tagged "Sex Drive"; i-witness) ~ alien5ive INFRA 012 upload | 77% | 0 |
| Ed Rush, Optical & Fierce - Alien Girl (corpus) ~ i-witness upload | 76% | 0 |
| Stratus - Waves (`extra/b-stratus-waves-sour.mp3`, the Sonix WEB file) ~ each other and the alien5ive INCIDENT 002 upload | 74% | 0 |
| Falcon - The Stand (3zb upload ~ alien5ive CITRUS 003 upload); falcon - fractles (`extra/a. falcon - fractles.mp3` ~ alien5ive CITRUS 003 upload) | 64%; 57% | 0; fractles 134 hits (possible) |
| Pyro - Time Is Broken (`extra/A. Pyro -- Time Is Broken.mp3`) ~ Pyro - Ignorance (3zb upload) | 44% | 0 |
| Ed Rush & Optical - Dozer (corpus) ~ "Dozer (2025 Notjamie Bootleg)" (i-witness) | 48% | 0 |
| DJ Trace - Sonar (corpus) ~ "Sonar (Mark System Revision)" (i-witness): joined at 91% with 246 hits | 91% | joined; check that it is the same recording |

Two borderline pairs are chance spans of 6 and 15 hits (Aphrodite -
Playground Shades ~ Critikal - Interlude; Cause 4 Concern - Night Gasp ~
Ray Keith - Dark Soldier VIP) and need no verdict; two more are the
mixed CD's Luminous remix against Global Report next to it (44-45%).

### Chance against index size: the possible tier holds to about 3,000 tracks

Experiment 0052 ran the development scan and sweep 2026 with 1,000,
3,000 and 9,000 of the NAS records (a seeded draw, each part of the
next). Recall stays 540/540 with 0 wrong and the development mix 11/11
at every size. The strongest unrelated false candidate (different artist
and title) is 21 hits at 262 tracks, 27 at 1,200 assets, 34 at 3,200 and
64 from 9,200: one coincidence (Clockwork against Simon Static - Rubba
Rock) sets it from 9,000 on, and two sweep draws at full size reach
63-70. Cost grows linearly: about 0.39 CPU seconds per indexed record
per sweep and 0.064 per record for the 56-minute mix.

Proposal (the tier stays at 60 this session): keep 60 while the
strongest unrelated chance measured at the library's size stays below
30 hits, otherwise make the tier twice that chance and measure again
when the library doubles. That gives 60 up to about 3,000 assets and
130-140 for the NAS collection. At 140 the development mix's two
shared-material notes (Star Trails' Synergy remix at 79 hits, the
Clockwork remix at 94) would no longer be shown. Our call.

### Four sweep seeds at NAS scale: three pairs make every wrong answer

Item 10 fitted in: sweeps 2028 and 2029 under the new default
(experiment 0053) recall 540/540 each with 8 wrong answers each (session
5: 126 and 114). Over the four seeds, calibrate finds 2,179 identifying
detections, none below the rule, and 40 confident false ones from three
pairs: China Cup ~ its Prototype upload (24), Coma ~ Spraycan (8) and
Synthesis VIP ~ the mixed CD's "Synthesis (Remix)" (8, 511 hits, the
strongest false candidate). The margin is 1.29× under the rules; if
each pair is one recording, 3.36× (The Nine's VIP, 197 hits). If they
are different recordings, different recordings share up to 511 hits,
past the rule of 240: then the rule, or how shared material is shown,
needs our decision.

## After session 6

### An outside review: two harness fixes and two corrections

We asked another agent (Astra) to review the project after session 6.
I checked its points against the code and the saved reports. Four of
them change the picture; the first two are now handled by `clusters`.

- **A `different` verdict could be undone through a third file.**
  Clusters join every chain of same-recording links, and a `different`
  verdict removed only its own pair's link. DJ Trace - Sonar shows the
  case: the corpus file joins the dfect upload "Trace - Sonar.m4a" at
  98% (24,774 hits), and that upload joins the "Mark System Revision" at
  91% (246 hits). A verdict on the corpus file against the revision
  alone would have left all three in one cluster. `clusters` now stops
  and names the chain, each link with its evidence. With the NAS store
  it checks the last run's pairs before searching, so this case stops
  in a quarter of a second, not after 85 minutes. A pair judged both
  `same` and `different` is an error too.
- **Coverage does not show how much evidence an alignment has.** It is
  the span from the first to the last hit. The Sonar revision covers 91%
  with 0.7 hits per second of aligned span; the median join has 18.6,
  the 5th percentile 6.5. `clusters` now prints each pair's hits per
  second and lists joins under a tenth of the median, to judge by ear. At
  NAS scale that is the two Sonar revision pairs and the i-witness
  upload of Phoenix against the TECH012 vinyl rip (99% coverage, 1.8
  hits per second); none at 262 tracks. It is a flag for listening; the
  criterion and the clusters are unchanged.
- **The rule's headroom is 1.22×, not 3.36×.** If the three pairs of
  the listening list are each one recording, the strongest false
  candidate is The Nine against its Evol Intent VIP at 197 hits, 1.22×
  below the rule of 240. The 3.36× margin compares it with the weakest
  identifying detection (661). The register asks for about 2×; raising
  the rule to about 400 would lose brief plays, so this belongs to the
  question of a statistic for shared material (roadmap).
- **The ladder's limit is relative to the copy in the collection.**
  Session 4 asked whether the decks go past ±8%. The search compares
  the mix with the indexed file, and copies of one recording differ in
  speed: of the 872 pairs of files joined in the NAS clusters, 180
  differ by more than 2%, 17 by more than 4%, and one "Speed Up" upload
  by 7.8%. A play at +5.65% against a copy 4% slow is at +10%, where
  recall is gone (±9%, experiment 0033). It matters when that copy is
  the only one indexed. Not measured yet; the roadmap describes the
  sweep that would measure it.

The other points are in the roadmap: a challenge set for shared
material, a minimum span based on the time that hits support, the
possible tier measured at each size instead of fixed breakpoints, recall
on NAS recordings beyond the corpus, diagnostics for failed queries,
hit sorting before any hash change, the common-hash filter across
shards, and a pair review that plays two files aligned. The review also
noted that 15/16 on the test mix is the single pass's result; the README
already said so, and the calibration register now does too.

For the verdicts by ear: if the "Mark System Revision" is another recording,
judge it `different` against both the corpus `DJ Trace - Sonar.m4a` and
`second-library/__youtube_archivists/dfect-dnb/Trace - Sonar.m4a`;
`clusters` names any link left. Please also listen to
`second-library/__youtube_archivists/i-witness-dnb/Drumsound & Simon
Bassline Smith - Phoenix.m4a` against
`second-library/[TECH012]_-_Drumsound_&_Simon_Bassline_Smith_-_Badman_&_Phoenix_Vinyl_(2001)/[AA]_-_Phoenix.mp3`.

### What the fingerprinting papers suggest

Astra also read the papers on the problems open here. I checked its
points against the papers (now listed with section numbers in
`docs/references.md`) and added them to the roadmap.

- **A peak verifier is the most promising new experiment.** Qfp
  (Sonnleitner & Widmer 2016, §VI-C) confirms each match by looking for
  the reference's nearby peaks in the query, with some tolerance in time
  and frequency. In a comparison on DJ mixes, that verification raised
  the share of unknown passages left unanswered from 64.7% to 92.7% on
  the Mixotic mixes, for 87.6% instead of 88.9% of known seconds
  identified (Sonnleitner et al. 2016, table 2). Gunfinger's second pass
  counts only exact pair hashes. The roadmap item "Evidence beyond exact
  pair hashes" lays out the verifier as a harness diagnostic first,
  then a measure of what exact hashing loses (Panako 2.0's near-exact
  hashing, Six 2021), then denser query peaks.
- **Public DJ-mix data without spending a test evaluation.** Mixotic (10
  real techno and house mixes, 723 reference tracks) and UnmixDB
  (generated mixes from the same tracks with exact cue points and
  speeds). Another genre, so they test how the design generalises; the
  roadmap proposes scoring them per second, as the DJ-mix paper does.
- **Shared samples as a test category.** Sample identification counts
  the sampled recording as the right answer; for Gunfinger it is a
  wrong one. The roadmap's shared-material challenge set gains four
  scenarios, among them a short shared break repeated for a long time.
  It should run before anything that adds evidence is adopted, the
  verifier included.

The papers also explain choices made without citing them: the line
search is Wang's offset histogram, the confidence rule follows Wang's
threshold from the strongest wrong track (which is why it moves with
library size), and the 2-4% speed differences between rips match
Panako's note that replay speed changes by accident when analogue media
is digitised. The ADRs, the calibration register and the code comments
now cite them.

## Session 7 findings

### Test-set evaluation 3: 14/16, Sick Note possible as predicted

We approved one evaluation in the session 7 prompt. One scan of the test
mix under the default matcher with both playbacks: 14 of 16 referenced
tracks, 0 wrong identifications, pass. Sick Note is possible at 233 hits
in 4 windows, 7 below the rule of 240 (209 under the single pass, which
was confident under its rule of 200), and Sin is possible at 123 hits
(84 before). The strongest detection matching no track has 17 hits.
Nothing changed because of it; 2 evaluations are left
(`docs/experiments/test-set-ledger.md`).

### Shared material does not reach the rule in any scenario

Experiment 0055 built queries from two passages that recordings
judged different share: Clockwork against its Stakka remix (about 19 s,
112-124 hits between the files) and China Cup against The Nine (about
23 s of shared drums), in both directions. Each passage was searched
alone (10 s and whole), looped to 60, 120 and 240 s, and running 10-30 s
into the played file's own material, against the full index, with the
played recording and its rips left out, and with only the source file
left out while another rip stayed.

- With the played recording left out, the related one reaches 114-144
  hits (possible) for Clockwork and 13-39 (weak) for China Cup ~ The
  Nine. Nothing is confident.
- Looping does not add up: each repeat starts a new alignment, so the
  strongest related detection stays at 144 hits at 240 s. A rule that
  summed a play's segments would make it confident.
- Running into distinctive material adds about 40 hits per second to the
  played file and nothing to the related one.
- The positive control works: with only the source left out, its other
  rip answers with the same hits and levels.

### Supported time: the sparse joins hold hits on 39-72% of the file

Coverage is the span from the first to the last hit of one alignment.
`pair` now also reports supported time: the share of the shorter file's
seconds that hold an aligned hit at the fitted speed (experiment 0057,
NAS peak store, no audio read from the NAS).

- Random joins: one alignment, supported time equal to coverage
  (0.98-1.00).
- The two Sonar joins to the "Mark System Revision" rest on one ladder
  line at 0.7 hits per second (coverage 0.89-0.91). At the fitted speed
  the hits fall into 4-6 alignments that hold 0.39-0.43 of the file.
  Phoenix's i-witness upload holds 0.63-0.72 against the TECH012 rips.
- Several pairs the clusters keep apart are held almost whole, at two
  offsets: China Cup against its Prototype upload (0.96-0.98, coverage
  0.65-0.68) and The Specialist against the INFRA 012 upload
  (0.97-0.99). One cut or moved section would do this. The Nine
  against its Evol Intent VIP is held at 0.86-0.96 in 4-5 alignments.
- Coma ~ Spraycan 0.76; Synthesis VIP ~ "Synthesis (Remix)" over 1.0
  (sections that repeat match twice); Global Report ~ Luminous and the
  two chance pairs 0-0.30.

Neither coverage nor supported time decides whether two files are one
recording: a VIP can share most of its seconds. The numbers are in the
listening list below, as aids for listening.

### The peak verifier separates no better than hit counts; proposal: no rule

`gunfinger-eval --verify` checks each detection after Qfp (Sonnleitner &
Widmer 2016, §VI-C): it takes the found file's reference peaks in the
aligned span and looks for them in the query at the detection's speed,
within ±1 bin and ±1 frame, each 10 s window moved by the offset of its
densest run of hash hits. It reports the share of reference peaks found
and the same share with the reference shifted by ±5 and ±6.2 s (chance).
The tolerance was chosen from three settings on the development scan and
sweep 2026, then frozen (experiment 0054). It is a harness diagnostic;
nothing in `identify` uses it.

- Identifying detections find most of the reference: median 0.85 of the
  peaks at 262 tracks against 0.004-0.010 by chance. Whole plays in the
  development mix find 0.34-0.67 (blends and EQ).
- At 262 tracks (experiment 0056; seeds 2027-2029, leave-outs 3 and 11,
  `robust --only combined`, `mixes --count 12`, `grid`; 7,243
  detections), at equal hits the share separates identifying from false
  detections no better than hits do (AUC 0.953 against 0.950), and
  worse against related records (0.827 against 0.865). The weakest true
  plays, blend partners at 40-66 hits, find 0.16-0.24 of their peaks;
  shared passages find as much (the Clockwork remix 0.27-0.41, Aphrodite
  - Fanfare against its Dubstyle version 0.11-0.22).
- At NAS scale (experiment 0058; development scan and sweep 2027, 40,295
  detections), sweep excerpts find 0.72-0.92 of their own file's peaks,
  a median of 0.70 of another rip's and 0.65 of another upload's (5th
  percentiles 0.44 and 0.47). The listening list's pairs: China Cup ~ its
  Prototype upload 0.59-0.61 and Coma ~ Spraycan 0.56-0.58, in the range
  of other uploads of one recording; The Nine ~ Evol Intent VIP
  0.36-0.54 and Synthesis VIP ~ "Synthesis (Remix)" 0.34-0.41, where
  other rips' lowest values and shared passages meet.

The verifier asks whether the found file's audio is in the query, and a
shared passage is in the query. Telling a weak true play from a shared
passage needs evidence the related recording lacks, and neither hash
hits nor peak shares give it.

**Proposal: no verifier rule** for the confidence levels; keep
`--verify` as a diagnostic. Two uses could pay
off later. Showing the share beside each pair in a listening list, as
below, sets an order for listening. And a play whose share is far
below its file's usual share for its hits (a lossy upload, a remaster,
a mixed-CD edit) is worth flagging when the clusters are reviewed. The
roadmap's other steps under "Evidence beyond exact pair hashes" (what
exact hashing loses, denser query peaks) are about recall under damage,
which this does not measure.

### Two more pairs for the clusters, found by the verifier's groups

- The untagged B side of UDFR014 (`extra/b-unknown-udfr014-sour.mp3`) is
  joined to the mixed CD's track 07, "Luminous (Remix)" (Dangerous Drums
  Volume 2, CD1). Its sweep excerpts also find track 06, Kraken -
  Meatball, at 93-112 hits: most likely the CD's mix into track 07, so
  the audio really is in that file. The same mixed CD holds "Synthesis
  (Remix)", which the listening list pairs with UDFR014's A side.
- Falcon - The Stand finds an untagged NAS file,
  `second-library/1.mp3`, at 65-70 hits (0.29-0.31 of its peaks). It has
  no tags to tell what it is.

### Listening list after session 7

Judged on 2026-10-09 ("The verdicts by ear" under "After session 7").
The same pairs as after session 6, with this session's measures as aids
for listening; none is a verdict, and `docs/pair-verdicts.txt` is still
empty. Coverage: the clusters' alignment, first to last hit. Supported:
the shorter file's seconds holding aligned hits at the fitted speed, over
all alignments (0057; above 1 where sections repeat). Share: reference
peaks the verifier finds in sweep 2027's detections (0058); other
uploads of one recording have a median of 0.65 and a 5th percentile of
0.47.

| Pair | Coverage | Supported (alignments) | Share; sweep 2027 detections |
|---|--:|---|---|
| China Cup (4 files) ~ Prototype upload | 0.65-0.68 | 0.96-0.98 (2) | 0.59-0.61; 16 confident, 305-408 hits |
| Coma ~ Spraycan | 0.59 | 0.76 (4) | 0.56-0.58; 8 confident, 285-341 hits |
| The Nine (6 files) ~ Evol Intent VIP | 0.48-0.50 | 0.86-0.96 (4-5) | 0.36-0.54; possible, 84-197 hits |
| Synthesis VIP; its i-witness upload ~ "Synthesis (Remix)" | 0.59; 0.53 | 1.36; 1.03 (12; 8) | 0.34-0.41; possible, 94-115 hits |
| Future Cut - The Specialist (3 files) ~ INFRA 012 upload | 0.77 | 0.97-0.99 (2) | not detected in sweep 2027 |
| Ed Rush, Optical & Fierce - Alien Girl ~ i-witness upload | 0.76 | 0.81 (6) | - |
| Stratus - Waves (2 files) ~ WEB file | 0.74 | 0.75 (2-3) | - |
| Falcon - The Stand; fractles | 0.64; 0.57 | 0.84 (4); 0.48 (9) | - |
| Pyro - Time Is Broken ~ Ignorance | 0.44 | 0.51 (6) | - |
| Ed Rush & Optical - Dozer ~ Notjamie bootleg | 0.49 | 0.34 (5) | - |
| DJ Trace - Sonar; dfect upload ~ "Mark System Revision" (joined) | 0.89; 0.91 | 0.39; 0.43 (4; 6) | - |
| Phoenix: i-witness upload ~ TECH012 rip; corpus rip ~ upload (joined) | 0.99; 0.84 | 0.72; 0.63 (4; 5) | - |

Read together: China Cup's pair is held almost whole at two offsets and
verifies like another upload, as one cut or moved section in one
recording would; it is also the pair behind most wrong answers. The
Sonar revision joins on less supported time than any other join measured.
The Nine's VIP shares most of its seconds at several offsets but fewer
of its peaks, as a rearranged version would.

### Other rips alone: the ladder holds to 9% of relative speed

Experiment 0059 asks what session 6's review asked: a play is compared
with the indexed file, and copies of one recording differ in speed. Four
NAS-scale sweeps (`sweep --other-rips`) left out each excerpt's source
file and its identical copies and kept the cluster's other files: 33-40
excerpts per seed, 1,305 queries. Relative speed combines the excerpt's
speed with the remaining file's speed from the clusters' pairs.

- Recall: 718/746 within 4%, 404/424 at 4-8%, 91/100 at 8-9%, 0/4 at
  9.2-10%, 0/31 at 10.2-12.1%. Three extra rungs at each end recover 3
  of the 4 at 9.2-9.5% and nothing beyond, as on the corpus copies
  (experiment 0033).
- Inside 9% every miss is found, below the rule, by the remaining copy:
  another vinyl rip of Spirit - Out of Control (124-182 hits), Phoenix's
  i-witness upload (25-47), DSCI4 uploads of Sinthetix - Cryogenic and
  SKC - Lobotomy (170-213), uploads of Cause 4 Concern - Blindside and
  Ed Rush & Optical - Funktion (213-239, just below 240). These copies
  lose evidence; speed is not the cause.
- 881 of the 1,213 answers come from uploads in the YouTube archive.
- Wrong answers: 24, from China Cup's Prototype upload (16) and
  Synthesis VIP's mixed-CD version (8), both on the listening list.
- 233 NAS clusters: 55 span more than 2% in speed, 7 more than 4%, one
  more than 6% (Falcon - The Stand, 11.5% over 6 files).

A play at +8% against a copy 3% slow, with no other copy indexed, is not
found. Whether that matters depends on how often the decks go past
about 5-6% and how often the only copy is off-speed. A wider ladder costs
search time for every query; adding rungs only when a play's best
candidate sits at the ladder's end would be cheaper. I changed nothing;
the roadmap item now has the numbers.


## After session 7

### My 2003 mix is the second development set

Experiment 0060: `identify` against the NAS store finds all 12 tracks of
`razorjack-2003-03-29` confident, with 0 wrong, confirmed by ear.
We made it a development set. Its manifest was drafted from that output,
with manual corrections: Phantom Force starts at 10:47 (alone from 11:50;
Gunfinger is first confident at 11:55), Kinetic's play at 17:23 is a
tease, and Out Of Control was most likely the Drum & Bass Arena LP file.
Since its starts are Gunfinger's own boundaries except track 4's, it
cannot measure boundary accuracy beyond that track.

Two things to know:

- The best-scoring file is not always one that could have been played in
  2003: for Out Of Control a rip of a 2005 vinyl scores 3.4 times the
  Arena file; for Balderdash a recent upload outscores it. Scoring is
  unaffected (every file of a recording counts), but hits do not tell
  which file was played.
- Five of its tracks exist only on the NAS, so the harness can score them
  only once manifests can reference NAS files; the roadmap ("More
  development mixes") lists the steps. The verdict by ear on Future Cut - The
  Specialist against the alien5ive INFRA 012 upload decides whether that
  upload's confident detection in this mix is right.

### The verdicts by ear (2026-10-09)

Written into `docs/pair-verdicts.txt` as given:

- `same`: China Cup ~ the Prototype upload; Synthesis VIP ~ the mixed
  CD's "Synthesis (Remix)"; The Specialist ~ the INFRA 012 upload (a
  needle skip at 4:54); Coma ~ the i-witness "Spraycan"; Phoenix ~ its
  i-witness upload (a poor, bass-light transfer); Alien Girl ~ its
  i-witness upload; fractles ~ the CITRUS 003 upload; the corpus Sonar ~
  the dfect copy.
- `different`: Sonar ~ the "Mark System Revision", against the corpus
  file and the dfect copy (mixed differently, hi-hats from the start);
  The Nine ~ the Evol Intent VIP (its 3:34-4:45 is a 135 bpm slowdown
  the original does not have).

Applied to the pairs of the last NAS clusters run, no verdict
contradicts a join. Seven clusters change: six gain the file judged
`same`, and Sonar's loses the revision. The verdicts take effect at the
next `clusters` run with `--other-peaks-dir` (85 minutes). All 40 wrong
answers of sweeps 2027-2029 at NAS scale (experiment 0053) and the 24
of experiment 0059 come from pairs now judged `same`. The strongest
false candidate left is The Nine against its VIP (197-220 hits, below
the rule of 240; margin 3.36× by experiment 0053's estimate), now
confirmed as different recordings: the VIP plays the original's first
3:34 unchanged.

China Cup: the speed difference heard by ear (about 1.3%) is fitted, and
wow and flutter are far too small to move a match by seconds. Before
2:00 the best alignment sits 6-22 s away from the main one, depending
on the rip, as an intro of repeating bars matches at several offsets;
the single alignment that coverage measures ends there. Alien Girl: the
alignments put the upload 13.5 s later after 1:42 than before 1:25, and
by ear there is no difference; the verdict is `same` either way. The 12:42
that macOS shows for the Prototype `.opus` is not what FFmpeg decodes:
Gunfinger reads 6:13.

### Two releases with swapped titles (experiment 0061)

Earlier research on both releases checked against the audio (the release
history itself is not verified):

- `Infrared-INFRA011-Future_Cut-2000-sour/`: "Sex Drive" is The
  Specialist and "Specialist" is Razor's Edge. Neither joins Moving
  Fusion's Sexdrive on the NAS.
- `(2000) Coma & Spraycan (DSCI4)/`: `02 Coma.mp3` (the corpus copy
  `extra/02 Coma.mp3`) is the i-witness "Spraycan", and `01 Spraycan.mp3`
  shares nothing with either. By the Discogs lengths, the folder's
  titles are swapped. To confirm: Coma reportedly has electronic bleeps
  in its intro; listen to `01 Spraycan.mp3`.

Gunfinger prints a play's title from the tags of its best file, so a
mislabelled file that scores best gives a wrong title on a right match:
track 2 of my 2003 mix appears as "Future Cut - Sex Drive". Retagging
is a manual task; the roadmap has a smaller change on Gunfinger's side.

### Mixed-CD tracks in the library

"Synthesis (Remix)" is one track of a DJ-mixed CD, so its tail already
blends into the next track and it should not be indexed. Our proposal,
a `.gunfingerignore` file at the library root, is in the roadmap
("Library ignore file"). Folders to check for mixed discs: `Underfire
UDFRCD003 - Dangerous Drums Volume 2 (2000)` (mixed); `2001 - VRSCD003
- Ed Rush & Optical - The Creeps [Virus]/CD2` (the album's tracks
reordered, plus Bleep Bleep and Kerb Crawler); `2002 - CPT003 - Kemal
vs. Rob F & Impulse - 256` (tagged with the compilations "Stateside
Sessions" and "Bass Drive").

## Session 8

### The library ignore file

`.gunfingerignore` at the library root leaves files and folders out of
the library (ADR 0009; the README's "Leaving folders out"). The syntax
is a subset of `.gitignore`: `#` comments, a pattern starting with `/`
or with a `/` in its middle is a path from the root, a trailing `/`
matches folders only, `*` matches within one name, `?` one character
and `**` any number of folders. Matching is case-sensitive, and square
brackets are literal, so `[Virus]` is written as it is. A leading `!`
and a backslash are errors that name the file and line and say what to
write instead; such an error stops every command that reads the
library.

`index`, `identify`, `explain` and `stats` with a library, `doctor`,
`prune` and the harness's library scans treat ignored files as files
not in the library. `doctor` shows whether it read the file, how many
audio files each pattern leaves out, a warning for a pattern that
leaves out none, and the records of ignored files apart from those of
files gone from the library. `prune` lists the two groups apart and
`prune --yes` deletes both. Searches without the library
(`--store-only`, or no library given) and the harness's
`--other-peaks-dir` read only the store, so they still find ignored
files until the store is pruned. Detection is unchanged (`regress
session-8-start`: 720/720 sweep queries and 87, 93 and 79 scan
detections identical); `scripts/check.sh` is green.

### The first ignore file on the NAS

`/Volumes/atlas/Music/dnb/.gunfingerignore` holds the four lines we
gave (254 bytes, written 17:18). Nothing else on the NAS was written:
the root's listing gained only this file, and the root folder's
modification time moved to 17:18, as creating a file does. macOS
attached its
`com.apple.provenance` attribute to the file, as it does to every file
written from this terminal; the share keeps it with the file, and no
`._` file appeared.

`doctor --config ~/.config/gunfinger/nas-dnb.toml` (2:21, mostly
listing the NAS):

- 30,216 audio files; the ignore file's line 3 (Dangerous Drums) leaves
  out 20 audio files and line 4 (The Creeps' CD2) 13. The Creeps' CD1
  stays: its 11 records are current.
- 33 peak records and 33 tag notes are for the 33 ignored files. No
  record or note is for a file gone from the NAS.
- Unchanged by pruning: 1,812 audio files are not indexed yet (added to
  the NAS since indexing), 1 record is out of date, and 1,396 files
  were passed over (2 failed to decode, 108 too short, 1,286 too long).

`prune --config ~/.config/gunfinger/nas-dnb.toml`, without `--yes` (27
s): it would delete 66 files, the 33 peak records and 33 tag notes of
the ignored files, and nothing else (no gone files, no skip notes, no
temporary files). A read-only check of the record headers agrees: 20
records under Dangerous Drums, 13 under The Creeps' CD2.

To prune, from the repository root:

```sh
target/release/gunfinger prune --config ~/.config/gunfinger/nas-dnb.toml --yes
```

Afterwards the store holds 27,009 records, and its revision changes
from `d428ee9585936326` to `6a4349c86e2582df` (computed read-only by
`scripts/analysis/store_revision.py`'s method on the records that
remain). The next session starts from that new baseline; NAS-scale
results before it (experiments 0035-0061) include the 33 files.
`2002 - CPT003 - Kemal vs. Rob F & Impulse - 256`, the third folder in
"Mixed-CD tracks in the library", is not in the file.

## Session 9

### What we need to decide

1. **Mixed CDs in the new folders.** The ignore file does not cover
   them, so they are indexed:
   - A scene copy of Dangerous Drums Volume 2 (20 records):
     `/Volumes/atlas/Music/dnb/__full_scene/dnb_scene - 2000 - Part 2/2000-08/va-dangerous_drums_2-udfrcd002-2000-sour/`.
     The ignore file's line 3 is anchored at the root, so it leaves out only the
     old folder. The development mix finds this copy's Night Lore
     (3,443 hits).
   - Three mixed CDs give my 2003 mix extra lines and one confident
     wrong title (experiment 0063): `Contagious Drum & Bass Vol 1`
     (`.../dnb_scene - 2000 - Part 2/2000-09/va-contagious_drum_and_bass_volume_1-concd1-2000-sour/`),
     DJ Marky's Audio Architecture
     (`.../dnb_scene - 2000 - Part 3/2000-11/dj_marky-audio_architecture-2000-sour/`)
     and Dieselboy's System Upgrade
     (`.../dnb_scene - 2000 - Part 1/2000-02/dieselboy-system_upgrade-2000-sour/`).
     To hear it: in `13-mc_mc_and_rushour-music_maker_(majistrate_remix)-sour.mp3`
     of Contagious, 5:00 to the end (6:11) is already Phantom Force,
     the next track; Gunfinger names "Music Maker" confidently at
     10:54-14:17 of my mix from that stretch alone.
   - 30 more new folders and 3 old ones look mixed by their track
     boundaries (below, and `work/s9/mixed-cd-candidates.tsv`). A
     gapless album scores the same, so each needs a check by ear: do the
     tracks run into each other?

   Timing: the prompt keeps the NAS store at `b8bb402f0ba23761` for the
   next session too, and store-only runs (`--other-peaks-dir`, the
   clusters) see ignored files until the store is pruned. If the next session runs
   `clusters` at NAS scale, these folders will be in it. Either add the
   lines and prune first (the next session then starts from a new
   revision, which its prompt should name), or let it do only the code
   steps and keep the clusters run for after the prune.

2. **Roles for candidate mixes** (no `identify` was run on any). The
   strongest candidates: my own `Razor Jack - 2003-08-27.mp3` (20.7
   min, "c4c dnb mix", at the NAS root), which can be checked by ear;
   single-file mixed CDs whose tracklists are published (Stakka &
   Skynet's Clockwork mix CD, Andy C's Ram Raiders The Mix, DJ TeeBee's
   Through The Eyes Of A Scorpion, Doc Scott's Hidden Rooms 3 mix,
   Bryan G's V Classic), good held-out sets because their ground truth
   does not come from Gunfinger; and 136 sets and radio shows of
   1999-2000 in the scene folders (Virus and Ray Keith on Kiss FM, Bad
   Company, Ed Rush, Konflict and Usual Suspects live, Grooverider on
   Radio One). The NAS root also holds
   `EM_1999_02_07_-_Ed_Rush_Optical.mp3`, by name the broadcast of the
   held-out test set; I did not compare it. Please confirm, and keep it
   out of any development use.

3. **Damaged files** (13 files, 12 recordings; they cannot be indexed):
   replace them if they should be in the library. Seconds missing, and
   "zeroed" where the gap is zero-filled (an incomplete transfer):

   ```text
   21.2 s         04 - paul b & dissident - technecium.mp3
   16.2 s         TI028 - Decoder - Hord 39; Cobra [2000] (Tech Itch Recordings)/B_-_Decoder_-_Cobra.mp3
   16.2 s         __full_scene/-= Tech Itch Recordings =-/TI028 - Decoder - Hord 39; Cobra [2000] (Tech Itch Recordings)/B_-_Decoder_-_Cobra.mp3
   23.8 s         __full_scene/-= Tech Itch Recordings =-/TI027 - Biostacis - Biotactics EP [2000] (Tech Itch Recordings)/C_-_Biostacis_-_Titanium.mp3
   31.0 s         __full_scene/-= Tech Itch Recordings =-/TI027 - Biostacis - Biotactics EP [2000] (Tech Itch Recordings)/D_-_Biostacis_-_Dos.mp3
   10.5 s zeroed  __full_scene/1999/1999-08/1999-08-22/underfire_recordings-blazin-cd_rip-1999-sour/205_-_skynet_&_stakka_-_mix-sour.mp3
   61.8 s zeroed  __full_scene/1999/1999-08/1999-08-22/underfire_recordings-blazin-cd_rip-1999-sour/209_-_skynet_&_stakka_-_mix-sour.mp3
   70.7 s         __full_scene/Certificate 18/CERT1850 - Panoptica - 02-04 (2001)/a. panoptica - torero t.mp3
   59.9 s         __full_scene/dnb_scene - 2000 - Part 1/2000-01/dillinja-never_believe-2000-sour/01-dillinja-never_believe-2000-sour.mp3
   42.6 s zeroed  __full_scene/dnb_scene - 2000 - Part 2/2000-09/cert18-klute-fear_of_people-cert18cd009-2000-sour/03-klute-chicks-sour.mp3
   18.6 s         __full_scene/dnb_scene - 2000 - Part 3/2000-10/v_recordings-planet_v_remixes_part_2-velprmx02-2000-sour/a-krust-burning_2000_(lemon_d_remix)-sour.mp3
    5.4 s zeroed  __full_scene/dnb_scene - 2000 - Part 3/2000-11/moving_shadow-basic_unit-shadow109-1997-sour/y-basic_unit-silver_wolf-sour.mp3
   18.8 s zeroed  __full_scene/dnb_scene - 2000 - Part 3/2000-12/razors_edge-goldie_vs_dillinja-razors002-1995-sour/a-goldie-jah_(vip_rollers_mix_by_peshay)-sour.mp3
   ```

   The first two are session 6's truncated files. No file is waiting to
   be indexed.

### Past 32,768 assets (ADR 0010, experiment 0062)

A posting now holds 16 frame bits and 16 asset bits: 65,536 assets and
tracks up to 17:28, still 4 bytes per posting at full frame resolution.
The default track limit is 17:00 (was 20:00); the NAS configuration
stops at 15:00 and the corpus's longest track is 9.6 minutes, so
nothing that gets indexed changes. With `--max-track` above 17:28 and a longer
record, searches stop and name the file and the limit, and `doctor`
marks it as a problem. Detection at 262 tracks is identical. Before the
change, `stats` with the configuration stopped after 57 s with "the
index holds at most 32768 assets". The next limit is 65,536 assets;
then the index is split into shards (ADR 0007). The NAS store is at 50%.

### The store at 32,905 records (experiment 0063)

`stats`: 922 million postings, 3.70 GB, built in 65-72 s. Identifying
the development mix from the store takes 5:15 and 4.76 GB, my 2003 mix
4:26 and 4.66 GB, about 20% more than at 27,042 records. The development
mix gives the same 13 tracklist lines. My 2003 mix still finds all 12
tracks; Phantom Force now starts at 10:48 (a PHUD1 vinyl rip in the new
scene folders, 6,513 hits), close to the 10:47 heard by ear. Most other new
files that appear are other rips of the tracks played; the rest come
from the mixed CDs above.

### The NAS over Ethernet (experiment 0064)

`doctor` took 44 s (2:21 over Wi-Fi), after two earlier listings. 34,470
audio files: 32,905 indexed, 1,565 passed over (13 damaged, 129 too
short, 1,423 too long), none waiting and none gone. Indexing three new
folders (376 files) ran at 1.9 s per hour of audio, the local-disk rate:
the CPU is the limit now, not the network, and a full re-index would
take about 1.8 hours (5.9 over Wi-Fi). Every record decoded again
matched the store bit for bit, and the store's revision is unchanged.

### Long files and mixed CDs (experiment 0065)

The 1,423 files over 15:00 hold 1,264 hours: 561 alien5ive game videos,
366 YouTube mixes and shows (155 naming 1990-2003), 247 other uploads,
136 scene sets, radio shows and mixed CDs, 96 radio shows and sets in
`sety/`, 17 files at the root. The full list with lengths and tags is
`work/s9/candidate-mixes.tsv`.

Folders whose tracks run into each other at 75% or more of their
boundaries, the share in front (new unless marked; paths in
`work/s9/mixed-cd-candidates.tsv`):

```text
1.00 decoder_n_substance-encounters-1999-sour
1.00 knowledge-the_lowdown_vol_7_mixed_by_marky-1999-sour
1.00 shy_fx-just_an_example-1995-sour
1.00 va-california_drum_and_bass-1997-sour
1.00 720-two_revolutions_mixed_by_blame-2nd_revolution-1999-sour
1.00 dj_zinc-beats_by_design_ep_(mixed)-tpr12025cd-1999-sour
1.00 va-thermodynamix_vol_1-2cd-(2000)-sour/cd2
1.00 va-knowledge_cover_cd_may-2000-sour
1.00 emotif_presents_the_a1_sound_carrier_ii-emf2cdlp005-2000-sour (and an old copy at the root)
1.00 pascal-a_new_p_funk_era_(mixed)-tpr12026cd-2000-sour
1.00 va-beyond_voltage_2-fuzecd003-2000-sour
1.00 va-dangerous_drums_2-udfrcd002-2000-sour/cd2 (cd1 0.88)
1.00 va-bootleggers_3-tovblcd03-2000-sour
1.00 va-contagious_drum_and_bass_volume_1-concd1-2000-sour
1.00 va-jungle_vibes-col477877_2-1994-sour
1.00 betacd001-john_b-catalyst-1999-sour/catalyst-solid_cd2 (old)
1.00 mampi_swift-blade_runners_ep-bld003-2002-sour (old)
0.97 va-world_dance_(mixed_by_kenny_ken_and_dj_ss)-hlplcd5-1997-sour
0.96 va-moving_shadow_10th_anniversary_cd-msx001cd-2000-sour
0.94 dieselboy-system_upgrade-2000-sour
0.93 knowledge-the_lowdown_vol_6-mixed_by_decoder-1999-sour
0.92 various_mixed_by_dj_freedom-metro_breaks_nxt_level-2000-sour
0.92 dj_marky-audio_architecture-2000-sour
0.91 va-blueprint_archive-abprcd003-1999-sour
0.85 dj_patife-presents_sounds_of_drumnbass-1999-sour
0.83 r_a_w-ultra_mix_drum_n_bass-1997-sour
0.83 va-mixmag_live_volume_22-mmlcd022-1996-sour
0.82 dream_magazine-for_those_who_remember-1997-sour
0.80 720-two_revolutions_mixed_by_blame-1st_revolution-1999-sour
0.77 va-jungle_vibes_2-sel9-1995-sour
0.76 knowledge-l_double_and_harry_love-the_lowdown_8-1999-sour
```

Releases with a mixed disc beside unmixed tracks in one folder score in
between: Blazin (its 2xx files, 0.35), Inside The Machine
(`vdbcrcd001`, 0.48) and Molten Beats (its bonus mix CD, 0.52). A
pattern such as `2*` inside such a folder leaves out one disc's files.

## After session 9

### The mixed-CD judgments

`/Volumes/atlas/Music/dnb/.gunfingerignore` now holds 31 patterns: the 2
of session 8 and 29 written on request. We marked 15 folders of
session 9's third group mixed (the two copies of The A1 Sound Carrier II
under one line) and left Shy FX's *Just An Example* and the Mampi Swift
EP indexed. By our choice the file also leaves out the first two
groups: the scene copy of Dangerous Drums 2, Contagious Drum & Bass Vol
1, DJ Marky's Audio Architecture, Dieselboy's System Upgrade, and the 11
folders whose names say they are mixed. The new lines have no leading
`/`, so each leaves out its folder wherever it sits; Thermodynamix's
`cd2` is `**/va-thermodynamix_vol_1-2cd-(2000)-sour/cd2/`.

`doctor` read all 31 patterns, each leaving out the expected folder
(504 audio files in all). The prune deleted 935 files: 464 peak
records, 7 skip notes and 464 tag notes. The NAS store now holds 32,441
records and 1,558 skip notes, revision `5701f221f7b48ee1`. The verdict by ear
on Synthesis VIP against the Dangerous Drums 2 track now names an
ignored file.

## Session 10

### What needs doing: the listening pack

`work/listening/README.md` lists 30 pairs of library files in the order
to listen; each has a folder under `work/listening/` with `sheet.txt`
(both full paths, coverage, supported time, each alignment with its
times, where the files stop lining up) and clips rendered from the files:
8 s of one file then the same 8 s of the other at the first one's speed
(`*-a-then-b.mp3`), and the two aligned in stereo, one file per side
(`*-stereo.mp3`), once inside the main alignment and once at each place
where they stop lining up. A verdict goes into `docs/pair-verdicts.txt`
as the sheet's last line shows. Then `gunfinger-eval --other-peaks-dir
~/.local/share/gunfinger/nas-dnb-peaks clusters --from-peaks
--reuse-pairs` applies the verdicts by ear to this session's pairs in about a
second (add `--cut-sparsest` while item 1 is unjudged).

1. **One cut join first** (item 1): the PRO012 rip of DJ Trace's Sonar,
   `/Volumes/atlas/Music/dnb/__full_scene/dnb_scene - 2000 - Part 2/2000-08/trace-sonar-pro012-2000-sour/a-trace-sonar-pro012-sour.mp3`,
   against the "Mark System Revision",
   `/Volumes/atlas/Music/dnb/__youtube_archivists/i-witness-dnb/DJ Trace - Sonar (Mark System Revision).opus`.
   Below, "A verdict the clusters run could not keep".
2. **19 borderline pairs** (items 2-20, one alignment covers 40-80% of
   the shorter file), 13 of them new. Nine pair a track of Essential
   Rewindz's mixed discs with an unmixed copy (items 3-6, 8, 12-14, 16);
   an ignore pattern for those discs would settle them without
   listening. Items 2, 9, 10, 18, 19 and 20 were already borderline
   after session 6.
3. **10 control joins** (items 21-30), drawn by seed from session 6's
   joins, all still joins: what the same recording sounds like here.

No sparse join is left: the only two were the cut join's two
measurements.

Decisions besides the pack:

- **Mixed discs beside unmixed ones.** Session 9 listed Blazin (its
  `2xx` files) and Inside The Machine (`vdbcrcd001`) as folders with
  one mixed disc; Essential Rewindz
  (`va-essential_rewindz-mixed_by_grooverider-rhlpcd03-2000-sour`,
  discs 1 and 2 mixed, disc 3 unmixed) is a third. Their mixed tracks
  now cause all 24 wrong sweep answers (Essential Rewindz), 9 of the
  pack's 19 borderline pairs (Essential Rewindz) and 6 joins into corpus
  clusters (Inside The Machine's disc 2, Essential Rewindz's Warhead,
  Blazin's `210_-_skynet_&_stakka_-_mix-sour.mp3`). Patterns such as
  `**/va-essential_rewindz-mixed_by_grooverider-rhlpcd03-2000-sour/1*`
  and `.../2*` would leave out the mixed discs only. After a prune, the
  next clusters run and baseline would show the effect.
- **`--cut-sparsest`**: I kept it opt in. Whether `clusters` should cut
  instead of stopping by default is our decision; the cut is always listed.
- **The 2003 manifest's header comment** still says that five tracks are
  `[]`; the references are in place.

### The verdicts by ear at NAS scale (experiment 0066)

The clusters run with the verdicts by ear, the 2003 mix's NAS references and
the recall panel's 80 development sources as queries took 8,063 s (639
queries in three rounds, 75,059 s CPU, 5.41 GB): 264 clusters, against
233 in session 6.

- The `same` verdicts add five files to their corpus clusters: the
  uploads of China Cup (PRO 001 UK), The Specialist (INFRA012), Coma
  ("Spraycan"), Alien Girl and fractles. Phoenix's upload was joined
  already. The Synthesis VIP verdict adds nothing: its Dangerous Drums 2
  file is ignored and pruned.
- Sonar's cluster loses the "Mark System Revision" and gains the PRO012
  rip, with the cut described below.
- 46 corpus clusters gain 60 files indexed since session 6, mostly scene
  releases. Some are mixed-disc tracks that cover 80% of themselves with
  one track, so they join: four tracks of disc 2 of Bad Company's
  `bad_company-inside_the_machine-vdbcrcd001-2000-sour` (The Nine,
  Sentient, Oxygen, 4 Days), Warhead on Essential Rewindz's first disc,
  and `210_-_skynet_&_stakka_-_mix-sour.mp3` on Underfire's Blazin CD
  (with Analogue Spikes). None of the three folders is in the ignore
  file.
- Five clusters lose a Dangerous Drums 2 track (the prune).

### A verdict the clusters run could not keep, and the cut

The clusters run (2 h 16 min) first stopped at its end without a report:
the verdicts by ear keep the Sonar "Mark System Revision" apart from the corpus
Sonar and the dfect copy (lines 29 and 30), but the PRO012 rip, indexed
since session 6, joins both sides: the corpus Sonar at 100% (7,258 hits,
19.9 hits/s) and the revision at 99% (467 hits, 1.3 hits/s). I added no
verdict. `clusters` has a new opt-in `--cut-sparsest`: when joins chain
two files judged different, it cuts the chain's measured join with the
fewest hits per second (never one judged `same`) and lists it; the
report keeps it in `cut_links`. The second run used it and cut the
PRO012 rip's join with the revision. That keeps the revision apart from
all Sonar rips, as the two verdicts intend; it is the first item of the
listening pack. If the PRO012 rip and the revision are the same
recording, one of the Sonar verdicts is wrong instead.

### The new NAS baseline (experiment 0067)

With the new clusters, at 32,441 records: the development scan 11/11
with 0 wrong, leave-outs 3 and 11 8/11 and 0/11 with 0 wrong, and sweeps
2026-2029 540/540 each, as in session 6. The sweeps' wrong answers go
from 0, 24, 8, 8 to 4, 0, 8, 12. The verdicts by ear removed all 40 old ones
(China Cup, Coma, Synthesis VIP). All 24 new ones come from Essential
Rewindz's mixed disc 2: an excerpt of Funktion finds
`203-ed_rush_and_optical-funktion-sour.mp3` (up to 332 hits), one of The
Nine finds `206-bad_company-the_nine-sour.mp3` (up to 382). Those mixed
tracks cover 58-60% and 78% of themselves with the track, so the
clusters keep them apart and every detection of them counts as wrong.
Over the four seeds the weakest identifying detection is 661 hits and
the strongest false one 382, a margin of 1.73× (1.29× before). Without
same-name pairs the strongest false is The Nine's Evol Intent VIP, which
was judged different by ear, at 220 hits (3.00×). Unrelated chance stays at
63-70 hits outside that disc's blends. The rule stays 240 hits in 3
windows. Costs rose with the records: sweeps 1,535-1,584 s, 14,012-14,535
s CPU, 6.3-6.7 GB; scans 307-315 s.

My 2003 mix in the harness: at NAS scale 12/12 with 0 wrong, leave-out
3 9/12 with 0 wrong; at 262 tracks 7 of its 7 referenced tracks with 0
wrong and nothing even possible over the 5 NAS-only tracks, leave-out 3
4/7 with 0 wrong.

### Recall beyond the corpus recordings (experiment 0068)

The recall panel's development half: 60 indexed and 20 held-out NAS
records outside the corpus recordings' clusters, 20 indexed per kind
(scene releases, YouTube channels, everything else), each as a 30 s
excerpt at 9 speeds from -8% to +8%, searched against all 32,432 other
records. All 540 indexed excerpts return their own recording, at every
speed and for every kind; no held-out excerpt gets a confident answer
(the strongest is 101 hits, possible). So there is no miss to explain
(the harness now says, for each miss, whether it never became a
candidate, had too few hits, or had too few windows). The 30 wrong
answers are two pack pairs: Stealth on Essential Rewindz's mixed disc
1, a panel source, finds three unmixed copies of Stealth (up to 365
hits; items 3, 5 and 6), and the 3zb "DYkast • Punishment" upload finds
butch-french's "Dykast - Punishment" at every speed (up to 472 hits,
44% coverage; item 7). The validation half is drawn and kept, not
searched.

### The 2003 mix: Phantom Force and Kinetic (experiment 0069)

- With Phantom Force's two rips left out (the i-witness upload and the
  PHUD1 vinyl rip), nothing becomes confident between 11:48 and 14:40.
  Phantom 2018 stays possible (322 hits from 11:31, in three segments
  that each align the same minute of it, 1:01-1:56), Fracture's edit
  possible (118 hits at 13:46), Rift's Meltdown possible (98, key lock).
  So the track would be missed, with three possible notes and nothing
  wrong.
- With the PHUD1 rip alone left out, the start moves back to 11:55.8,
  where the upload's evidence begins. The PHUD1 rip's first segment,
  10:49-12:03 (track 0:58-2:15), covers an intro the upload never
  matches; by ear, the blend is at 10:47.
- Kinetic: the tease at 17:23.7-18:01.8 plays the track's 1:11-1:51 (199
  hits in 5 windows, possible on its own). Nothing of it follows until
  18:35.6, where the track starts again from 0:15 and stays to 25:18.8.
  `identify` joins both into one play from 17:23.

### What changed in the harness

- **Verdicts link only files a run searches.** A `same` verdict joins two
  files only when both belong to the libraries the run searches: the
  corpus library's files and, with the NAS store, the files it holds a
  current record of, never one the `.gunfingerignore` leaves out (the
  harness reads it from `/Volumes/atlas/Music/dnb` when the share is
  mounted). Every other verdict is printed with its line and adds
  nothing. In this session's run that is line 26, Synthesis VIP against
  the Dangerous Drums 2 track: the ignore file's line 3 leaves it out
  and the prune removed its record, so the VIP's cluster no longer
  reaches the mixed CD. A corpus-only `clusters` run now leaves all the
  NAS files of the verdicts out of the corpus clusters.
- **Manifests can reference NAS files** as `second-library/<path>`. With
  the NAS store, `validate` and `scan` check them against its records;
  without it they are set aside, so a track with no corpus reference
  counts as absent at 262 tracks, as before. As agreed, the five
  `# nas:` lines of `razorjack-2003-03-29` (tracks 1, 3, 4, 6 and 8) are
  now references; nothing else changed in the file. Its header comment
  still says that these five tracks are `[]`; it may need an update.
  `validate razorjack-2003-03-29`: 7 referenced and 5 absent at 262
  tracks, 12 referenced with the NAS store. `validate` takes a set name,
  so that this session never opened the test set's manifest.
- **A recall panel from the NAS** (`gunfinger-eval recall`,
  `docs/panels/recall-seed-2026.json`), drawn from tags and record
  headers before anything was searched: 80 development sources (60
  indexed, 20 held out; 20 indexed per kind: scene releases, YouTube
  channels, everything else) and 80 validation sources, from 29,057
  recording families split in two halves, so that one recording never
  sits in both. 406 records without an artist and title tag are not in
  the pool. Only the development half was searched.
- **`clusters --cut-sparsest`** (opt in): when joins chain two files
  judged different, it cuts the chain's measured join with the fewest
  hits per second, never one judged `same`, lists the cut, and
  keeps it in the report (`cut_links`). Without it `clusters` stops as
  before, but only after the search: a chain that only the new search
  finds costs the whole run (2 h 16 min this session). Keeping a
  stopped run's pairs for a quick second pass is in the roadmap.
- **`clusters --reuse-pairs`** (with the NAS store) clusters the last
  report's pairs again with the current verdicts in about a second,
  without searching. Pairs with a file pruned or ignored since are
  dropped; files indexed since are not searched. Checked on this
  session's report: with `--cut-sparsest` it gives the same 264 clusters
  and the same cut; without it, it stops on the Sonar chain in 1 s. Run
  it after judging pack items, and after a prune; run the full search
  after indexing new files. The check before a full search now reads the
  last run's pairs the same way, so a chain the last run found stops the
  next run at once. A search that a verdict stops anyway keeps its pairs
  in `duplicate-clusters-stopped.json` and says so; after a verdict,
  `clusters --from-peaks --reuse-pairs <that file>` clusters them
  without searching again.
- **`scripts/analysis/listening_pack.py`** builds the listening pack
  from a clusters report: `select` (cut joins, borderline pairs and
  sparse joins without a verdict, controls drawn by seed), `render`
  (sheets and FFmpeg clips), `readme` (the listening order).
- **`stats`, `identify` and `explain` say when the track length range
  leaves out every record**, with how many files, how long, and the
  options that widen it (roadmap, "No record inside the track length
  range").

### Worth knowing

- `corpus/library/extra/310-future_cut-horns_2000-sour.mp3` is identical
  to `310-future_cut-horns_2000-sour.mp3` on Essential Rewindz's unmixed
  third disc, so that copy is right where it is.
- At the end the NAS store is as
  at the start (32,441 records, revision `5701f221f7b48ee1`); `doctor` counts 34,642 audio files
  against 33,999 at the start (files were
  added to the NAS during
  the session), so 643 are not indexed yet.
- Every long run now starts detached under `screen`: the agent's shell
  stops a background command after about 40 minutes, which cut the first
  clusters run short at 02:27.

## Session 11

### What needs doing

**The listening pack, rebuilt (experiment 0071).** `work/listening/`
now holds 23 items; session 10's pack is kept in `work/listening-s10/`.
Step through it, from the repository (the release build is the
session's last code, `84fe0e5`), with

```sh
target/release/gunfinger-eval --other-peaks-dir ~/.local/share/gunfinger/nas-dnb-peaks pair-review
```

It shows each item without a verdict in short (both full paths, the
coverage, where the files stop lining up), plays its clips with `ffplay`
and, on `s` or `d`, appends the `same` or `different` line to
`docs/pair-verdicts.txt`. Enter or `n` plays the next clip, `p` the
previous, `r` replays, `k` skips the item, `q` quits; items with a
verdict are skipped, so a review can stop and resume. At the end it prints
the `clusters --from-peaks --reuse-pairs --cut-sparsest` command that
applies the verdicts in about a second.

1. **The cut Sonar join** (item 1), as in session 10: the PRO012 rip
   against the Mark System Revision, which was judged different from the
   corpus Sonar.
2. **Two new borderline pairs.** Item 2 is the test manifest's Mind
   Overload (`ram-ram_trilogy_chapter_2-ramm23-1998-sour/01-ram_trilogy-mind_overload.mp3`)
   against Molten Beats' `111-ram_trilogy-mind_overload-sour.mp3`: 79.3%
   coverage, 3,321 hits, an edit of the same recording with about 45 s
   left out (the strongest unjudged pair of the run). Item 10 is Ryme
   Tyme's T-Minus against its "Unreleased Vocal Mix" (58%).
3. **Ten borderline pairs from session 10** (items 3-9, 11-13), still
   without a verdict, and the **ten control joins** (items 14-23), all
   still joined.

The eight patterns settled session 10's nine Essential Rewindz items
(3-6, 8, 12-14, 16): each pairs a pruned file, so they are gone.

**Seven folders to judge by ear (experiment 0070).** The 1,252 records
indexed since session 10 are all YouTube uploads (butch-french,
cryptic-jungle, 3zb-gangstep-dnb), so no new release folder needed
checking. Scored disc by disc, no folder or disc reaches the 0.75 that
mixed CDs reach; Shy FX's *Just An Example* and the Mampi Swift EP, which
were kept, still do. These seven score in between. A gapless album scores
the same, so only listening to a few boundaries tells them apart. Nothing
was written to the ignore file.

| Score | Boundaries joined | Folder |
|--:|--:|---|
| 0.62 | 8 of 13 | `/Volumes/atlas/Music/dnb/__full_scene/1999/1999-08/1999-08-20/skynet_and_stakka-point_of_arrival-1999-sour/` |
| 0.60 | 6 of 10 | `/Volumes/atlas/Music/dnb/__full_scene/dnb_scene - 2000 - Part 3/2000-11/va-listen_up-hlcd9-2000-sour/` |
| 0.55 | 6 of 11 | `/Volumes/atlas/Music/dnb/__full_scene/1999/1999-12/1999-12-19/jonny_l-magnetic-1998-sour/` |
| 0.50 | 5 of 10 | `/Volumes/atlas/Music/dnb/2001 - VRSCD003 - Ed Rush & Optical - The Creeps [Virus]/CD1/` (kept after session 9) |
| 0.50 | 6 of 12 | `/Volumes/atlas/Music/dnb/[RUFF 57] Various - Analog Steroids (CD)/` |
| 0.44 | 4 of 9 | `/Volumes/atlas/Music/dnb/__full_scene/dnb_scene - 2000 - Part 1/2000-06/va-breakbeat_science-scincd001-1996-sour/`, disc `2*` only |
| 0.40 | 4 of 10 | `/Volumes/atlas/Music/dnb/bad_company-fear_lp/` |

A pattern for one disc has the form
`**/va-breakbeat_science-scincd001-1996-sour/2*`, as for Essential
Rewindz.

### Test evaluation 4 at NAS scale

As approved, the test mix was scanned once at NAS scale, with the
baseline's binary (`1afd8cc`) and evaluation 3's command plus
`--other-peaks-dir`. The ledger entry and a per-track prediction were
committed first (`fab29bb`, 03:42); the run went at 06:52, after items 5
and 6 matched session 10.

**21/23 referenced tracks identified, 2 wrong identifications**; Sin and
Sick Note possible; one possible play matching no track (The Nine's
ChaseR bootleg, 81 hits). Evaluations used: 4 of 5.

- All 14 tracks of evaluation 3 are found again, and the 7
  referenced on the NAS are all found (Asylum the weakest, 309 hits in
  16 windows).
- Wrong 1, predicted: Molten Beats'
  `ram_trilogy-molten_beats-1999-sour/111-ram_trilogy-mind_overload-sour.mp3`
  over Mind Overload (921 hits, beside the reference's 848). It is an
  edit of the same recording with about 45 s cut; one alignment covers
  79.3% of it, just under the 80% join. It is item 2 of the pack: if it is
  judged `same`, the score is recomputed from the stored report as
  21/23 with 1 wrong, without another run.
- Wrong 2, not predicted: Evol Intent's VIP of The Nine (489 hits in 16
  windows, 2:30-5:07, beside The Nine's 868), which was judged a
  different recording. The sweeps' 30 s excerpts give it 9-24% of the
  original's hits; over 2.5 minutes of the mix it reached 56%. This is
  the shared-material question below, not a threshold question.
- Tracks 3, 12, 16, 21 and 28: none of the other versions found
  (Terminal 2, the Orders mixes, the Marky & Bungle Funktion, the Clear
  Skyz remix and VIP, the Black Barrel Beachball) reaches even the
  possible tier; the strongest is the Beachball bootleg at 45 hits.

Nothing changes because of the result. The full output is in
`docs/experiments/test-set-ledger.md`.

### The saved index (ADR 0011, experiment 0072)

`identify`, `explain` and `stats` now save the index they build to
`~/.cache/gunfinger/indexes/nas-dnb-peaks-<digest>.index` and load it
on the next search while it is current; a stale, truncated or damaged
file is rebuilt and replaced. `--saved-index rebuild` forces a build,
`off` keeps the old in-memory behaviour. At 33,596 records the file is
3.8 GB on disk (the store is 2.59 GB), and the index is
ready in 2-3 s instead of 118-167 s (65-72 s on a quiet machine); my
2003 mix gives byte-identical reports either way, and `explain` takes 14
s. After `index` adds files, the next search rebuilds once. `doctor`
does not count the file yet; deleting it costs one rebuild.

### Other titles of one passage (item 3b)

When confident plays of different files cover the same passage and
their title tags disagree, each such play now names the others: in the
table a line "titled Future Cut - Sex Drive; other files here: FUTURE
CUT - the specialist (plays 5, 6, 7)", in the tracklist "(titles
disagree: Future Cut - The Specialist)" instead of "(also: ...)", and in
JSON `other_titles` with the play number, file and title. No title is
chosen; detections do not change. My 2003 mix's track 2 is the
example: play 4 is the INFRA011 rip tagged "Sex Drive".

### The NAS after the prune (experiments 0071, 0073, 0074)

- **Clusters** (0071): the eight patterns removed what they were
  written for. 13 corpus clusters change: six lose a pruned mixed-disc
  track, seven gain a new upload. 267 clusters; the strongest unjudged
  pair is Mind Overload's edit (79.3%).
- **Baseline** (0073): as session 10 in every score, and the prediction
  held: the 24 wrong sweep answers from Essential Rewindz are gone
  (2,160/2,160, 0 wrong). The strongest false candidate is The Nine's
  VIP at 220 hits, a 3.00× margin to the weakest identifying 661;
  unrelated chance 63-70 hits. My 2003 mix: 12/12 and 7/7, 0 wrong.
  The rule stays 240 hits in 3 windows.
- **Recall** (0074): 522/522 from 58 indexed NAS sources (the two that the
  prune removed are named and left out, the panel not drawn again); the
  only wrong answers are the two DYkast "Punishment" uploads (item 4 of
  the pack).

### Research

- **Where a NAS `identify` spends its time** (experiment 0075). With the
  saved index, my 2003 mix takes 221 s, 96.5% of it the search; the
  search spends 82% of its CPU sorting each window's hits by asset and
  offset. A sort that groups hits by asset first would give identical
  detections faster; that is the next code step for speed, behind
  `regress`.
- **Shared material at NAS scale** (experiment 0076). For the pairs
  judged different and the version families the clusters keep apart (17
  pairs of clusters, outside the five versions found through the test
  tracklist), each shared passage was played whole from one file and the
  other file's play read. In 18 of 34 directions the related file's play
  is confident: The Nine's VIP at 21.7 times the rule (5,204 hits over
  209 s; it plays the original's first 3:34 unchanged), the "Fire"
  versions 1.2-2.5 times, Sonar's revision 1.6-2.3 times, Tsunami's
  remix and Killa Bees' Optical & Fierce remix just over it. The sweeps'
  30 s excerpts never showed this (at most 220 hits): hits grow with the
  passage. Leaving the played file out of the index does not change the
  related play. So the confidence rule alone cannot keep a version off a
  shared passage. The played file's own play is always stronger: the
  related play reaches 2-24% of its hits, the VIP 55-60%. That ratio
  would tell the remixes apart, but not the VIP: in my 2003 mix,
  copies of one recording over one passage reach 0.30-1.00 of the
  strongest copy's hits (uploads against rips). Today the report marks
  only a possible play inside a confident one as shared material; a
  confident version over the same passage is named under "other files
  here" when its title differs. Measurement only; no rule changed.

### Worth knowing

- **Another agent worked on this repository all night.** A git worktree of it
  (branch `fix/prerelease-audit`, from `8038064`), ran builds, debug
  tests and `candidate-eval regress` on this corpus, and committed fixes
  from 03:24. It touched neither the NAS store nor `~/.cache/gunfinger/`.
  With it, the clusters search took 8 h 06 min for 8% more cycles than
  session 10's 2 h 14 min, and the session's timings are marked as
  measured under load. It is not a Claude session this one can message,
  so it was left alone.
- **`origin/master` was pushed** to `8038064` at 02:43, this session's
  commit of 01:29, from the git directory both worktrees share; this
  session pushes nothing, and `master` is now ahead of it.
- **A review of this session's code** (a read-only subagent) found
  nothing that changes detections or lets a stale or damaged index load,
  and four small defects, fixed in `84fe0e5`: `pair-review` now always
  prints `--cut-sparsest`; a search deletes another search's unfinished
  `.partial` index only after 10 minutes untouched, and also after a
  successful load; a damaged posting count can no longer overflow the
  size check.

### At the end

- The NAS store is as at the start: 33,596 records, revision
  `f9f9bc706195a71b`; nothing in this session wrote to it. `doctor`
  counts 35,573 audio files against 35,178 at the start: 395 new files,
  not indexed (an `index` run adds them; the next search then rebuilds
  the saved index once).
- `regress session-11-start` with the session's last code (`84fe0e5`) is
  identical: 720 of 720 sweep queries and every scan's detections.
- Research items a-c were done by 07:26, before the 10:00 stop. The
  brief names no further item, so nothing else ran.
- Left for later (roadmap): grouping hits by asset before sorting them
  (experiment 0075; identical detections, checked by `regress`); a
  statistic for shared material, tested on recordings not used in 0076;
  `doctor` counting the saved index file; the harness's runs using a
  saved index; a compressed saved file.

## After session 11: the pre-release fixes, merged

We asked for the other agent's branch (`fix/prerelease-audit`) to be
reviewed commit by commit and merged if sound. It is merged (`53d7ee0`):
18 commits of fixes and speed-ups. On the merged code `regress` is
identical, and my 2003 mix identified at NAS scale gives the same 37
segments in 181 s instead of 220 s (experiment 0084). What a user may
notice:

- `index` now exits with an error when a file fails to decode in that
  run (it still processes every file first).
- `identify`, `explain` and `stats` fall back to the store's records only
  when the library root is missing (the NAS unmounted). Any other read
  error stops the command; `--store-only` searches the records anyway
  (ADR 0012).
- A play is confident when any of its segments is; before, a segment
  with more hits in too few windows could make it possible (ADR 0013).
  The harness always scored plays this way, so no evaluation changes.
- `--save-dir` refuses to overwrite a report of another recording with
  the same file name; a report is searched again when the mix file's
  size or modification time changes.
- Copies with identical audio and another title now show in the
  "other files here" line: for my 2003 mix, the second INFRA012 rip and
  the scene copy of "Sex Drive".
- The harness renders its cached excerpts again once, so the next sweep
  or recall run takes longer, and recall needs the NAS mounted.

## Session 12

### What changed

**Searches at NAS scale take half the time.** A window of the search
holds 0.6-2.0 million hits at 33,596 records, and sorting them took 78%
of the CPU (experiment 0085). Asset numbers are dense, so a counting sort
now groups each window's hits by asset in two passes, and each asset's
offsets are sorted as before. Detections stay the same: lines depend only
on each asset's offsets in sorted order, and a unit test compares the
lines with the old sort on large mixed buffers (experiment 0086).

| My 2003 mix, `identify` at NAS scale (saved index loaded) | Before | Now |
|---|--:|--:|
| Wall time | 167-171 s | 86-92 s |
| CPU time | 1,545 s | 774 s |
| Peak memory | 4.78 GB | 4.80 GB |

Every report was byte-identical to the start build's, and `regress`
stayed identical after every change.

**The harness loads a saved index too** (ADR 0014, experiment 0087).
Runs with `--other-peaks-dir` keep the index of the corpus and the NAS
store in `work/indexes/library-nas-dnb-peaks.index` and load it while
current; a leave-out or a sweep's held-out clusters are removed from it
in memory, which gives exactly the index built without them. The
development scan at NAS scale takes 112 s instead of 189 s, a leave-out
107 s instead of 180 s, with identical reports.

**Disk use.** The harness's file is another 3.8 GB, beside
the 3.8 GB of `identify`'s file in `~/.cache/gunfinger/indexes/`. Either
can be deleted; the next run that needs it builds it again (80-120 s).

**`doctor`'s new lines.** A "saved index" section at the end:

```text
saved index
  ok       ~/.cache/gunfinger/indexes/nas-dnb-peaks-23935cfc56936b67.index, 3.8 GB: current by its header and length; searches load it, checking the rest as they read it
```

It reads the file's header and offsets table (8 MB), not its postings;
a search still checks those and the digest, and rebuilds a damaged file.
Instead of `current` it may say `current for searches with --store-only, out of
date for the library` (the file was saved by a `--store-only` search and
the library leaves out some stored files), `out of date (...)`, `cannot
be read (...)` or `none yet`; each says what the next search does. Below
it, every other file in that folder with its size: saved indexes of other
peak stores, or unfinished saves. `doctor` deletes nothing.

### Worth knowing

- **Files were added to the NAS during the session.** `doctor` counted 36,305 audio files at 15:45
  and 36,376 at 16:30, against 35,573 at the end of session 11: 1,198
  are not indexed yet. The first `doctor` run (15:42) could not finish
  listing the library ("No such file or directory": most likely a file
  moved while it listed) and reported the store alone; the later two
  listed it. A plain `identify`
  stops on such an error (ADR 0012); `--store-only` searches the records
  regardless. The peak store was not touched: 33,596 records, revision
  `f9f9bc706195a71b`, at the start and at the end.
- **When the new files are indexed**, the next `identify` and the next
  harness run each rebuild their saved index once (80-170 s), and
  `doctor` says "out of date" until then.
- **The harness's sweep on the saved index** (seed 2026 at NAS scale,
  the 48 held-out clusters removed in memory) gave a report byte-identical
  to session 11's, in 641 s.
- Session 11's NAS reports were copied to `work/s12/before/` and put back
  after each run that overwrote them; this session's are in `work/s12/`.
- **A review of this session's code** (a read-only subagent) found
  nothing that can change detections. Fixed after it: `doctor`'s
  "current" line now says it read the header and length only; the
  `--store-only` check reads the store's headers only when needed; ADR
  0014 and AGENTS.md say that a change to how postings are made must
  change `hash::design()`'s tag or run with `--saved-index rebuild`, as a
  saved index is checked against files and settings, not code.

### Decisions waiting

Nothing new from this session. Still waiting, unchanged: the listening
pack (`pair-review`), the seven folders to judge by ear, and the questions
the brief left out of scope (the containment rule for confident plays,
shared material, the possible tier and edits, a clusters search, test
evaluation 5).

### Left for later

- The next speed steps, with identical detections (experiment 0088):
  sorting each asset's 30-60 offsets is now 39% of the NAS search's CPU,
  grouping and building lines 32%, the query analysis 19%.
- A compressed saved index (it needs `u64` byte offsets past 4.29 GB,
  ADR 0011), which would shrink both 3.8 GB files.

## Session 13

### What changed

**Commands no longer fail when another program moves a file while the library is
listed.** Before, a file or folder that disappeared between being listed
and being read stopped the whole listing with "No such file or
directory". `identify` then stopped too (ADR 0012), as did `index`,
`prune`, and `doctor`'s library check. Now a folder or audio file that
is gone when it is read is left out and counted, in one line that names
the first path (ADR 0015):

```text
warning: this run leaves out what vanished while /Volumes/atlas/Music/dnb was listed (moved or deleted meanwhile): 1 file or folder (`channel/Track.m4a`)
```

- **`doctor`** shows the same as a note in its library section.
- **Other files** (partial `.part` files, thumbnails, `.nfo`) are passed
  over by their names without being read, so they never count.
- **Other errors still stop**, such as permission errors and a share that
  answers with errors.
- **The root is never skipped.** If anything vanished, the root is read
  again at the end. If it is gone, an unmounted share for example,
  `identify` searches the store's records as before.
- **`prune` waits for a listing in which nothing vanished.** It refuses
  even with `--yes --force`, names what vanished, and asks to be run
  again. While another program changes the library it may take a few tries.

**Checked on the NAS (experiment 0089).**
- `doctor` ran 14 times and `identify` of my 2003 mix twice through the
  library, without `--store-only`. All 16 listings finished, while the
  NAS grew from 36,550 to 36,600 audio files. None of them happened to
  meet a vanishing file.
- The two `identify` reports (start build and fixed build) are
  byte-identical: 26 plays, 24 confident, 88-95 s each.
- A staged race on a local disk exercised the fix. Under constant
  renames and deletions, the old build could not list 10 of 30 times; the
  new one listed all 30 (9 with vanished entries).
- `doctor` on the NAS takes about 28 s instead of 40 s, because the files
  it passes over by name are no longer read. Earlier (before
  18:45) both builds' runs took 189-253 s; the NAS answered faster
  afterwards, for reasons not measured.

### Worth knowing

- The peak store was not touched: 33,596 records, revision
  `f9f9bc706195a71b`, at the start and at the end. The 1,422 new files on
  the NAS are not indexed, as requested. The saved index in
  `~/.cache/gunfinger/indexes/` is unchanged (one file, current).
- Detections did not change: `regress session-13-start` was identical
  after each change.
- A read-only review of the change found no defect in the listing, the
  root rule or `prune`'s refusal. After it:
  - files passed over by name are not read;
  - the vanished paths are kept and the first one named, so a path that
    vanishes from every listing can be found;
  - `doctor`'s advice on gone records says `prune` waits for a complete
    listing.

### Decisions waiting

Nothing new. Still waiting, unchanged: the listening pack, the seven
folders to judge by ear, the containment rule, shared material, the
possible tier and edits, a clusters search, test evaluation 5.

### Left for later

- A share that drops and comes back while it is being listed would leave
  that one run with a partial library. The warning shows the count, and
  `prune` refuses such a listing. Checking the root's device before and
  after the listing would also catch a root that is itself a mount point.
  Not needed for `/Volumes/atlas/Music/dnb`, which is a folder inside the
  share.
- The speed work of experiment 0088, unchanged.

## Session 14

### What changed

**A listening page: `gunfinger-eval audition`.** From the repository:

```sh
target/release/gunfinger-eval audition
```

It starts a small server on `127.0.0.1` and opens a page in the default
browser (Chrome is tested; Safari is not, see below). The queue on the
left lists the questions: the 23 items of the listening pack, then the
seven folders. A question plays its files in lanes on one timeline, so
switching lanes keeps the musical moment. The main keys:

- Space plays or pauses; `1`, `2`, `3` switch lanes; Tab goes back to
  the lane before.
- Left and Right seek 5 s (Shift: 30 s); Up and Down jump to the
  previous or next marker.
- `L` loops 8 s; `M` turns speed matching off and on; `B` plays two
  lanes at once, left and right.
- `S`, `D` and `U` answer a pair; `Z` undoes; `?` shows every key.

**Listen first to pack items 1 and 2, then the seven folders.** The page
opens item 1.

- Item 1 has three lanes: A is the Mark System Revision, B the PRO012
  rip, and lane 3 the corpus original (`corpus/library/DJ Trace -
  Sonar.m4a`), placed on A's timeline. `S` says B is the same recording
  as A (the revision), `D` that it is not; lane 3 makes it possible to hear whether B
  is the original instead.
- Item 2: B follows A in three alignments and jumps at 334.6 s of A. The
  markers show where the files stop lining up.
- A folder plays the last 20 s of each track, then the first 20 s of the
  next. Up and Down go from one boundary to the next. `Y` mixed CD, `N`
  separate tracks, `U` unsure.

**Where answers go.**
- Every answer goes to `work/audition/answers.jsonl`.
- A pair's `S` or `D` also appends to `docs/pair-verdicts.txt` the line
  `pair-review` writes; `U` writes only the log.
- After an answer the page opens the next open question.
- `Z` takes back the latest answer, then the one before. It opens that
  question and removes its verdict line if the line is still the
  file's last; otherwise it leaves the file alone and says so.
- Holding a key does not repeat an answer.
- After new pair verdicts, apply them as after `pair-review`:
  `target/release/gunfinger-eval --other-peaks-dir ~/.local/share/gunfinger/nas-dnb-peaks clusters --from-peaks --reuse-pairs --cut-sparsest`.
- `pair-review` and `listen.zsh` still work as before.

**My 2003 mix's plays, if wanted.** `questions detections`
makes one question per play of a saved report: is this detection right?
The 27 plays of my 2003 mix are in
`work/audition/detections/2003-s12.json`. They are left out of the
default queue so it holds only what is waiting for an answer. To add them:

```sh
target/release/gunfinger-eval audition work/audition/questions/*.json work/audition/detections/2003-s12.json
```

Their answers (`Y` right, `N` wrong, `U` unsure) go to the answer log
only; nothing reads them yet.

### Worth knowing

- **No listening file was written.** `docs/pair-verdicts.txt` is
  unchanged, and `work/audition/answers.jsonl` does not exist yet. Every
  check used a copy of the verdicts file and an answer log in `/tmp`.
- **The page was checked in Chrome on real files:** pack items 1 and 2,
  control join 14, the folders Skynet & Stakka and Bad Company - Fear
  LP, and play 4 of the 2003 mix. The checks covered:
  - switching lanes at the same moment;
  - speed matching, the loop and the two-lane blend;
  - markers;
  - answers and undo, including undo after a later line was added;
  - refusal of a verdict that contradicts the file.

  Screenshots are in `work/s14/`: `screenshot-2-pack-item-1-sharp.png`,
  `screenshot-3-pack-item-2.png`, `screenshot-4-folder-jonny-l.png` and
  `screenshot-5-detection-play-4.png`.
- **Speed:**
  - The server cuts a 30 s window of a NAS file in about 0.1 s.
  - After a far jump, the first sound waits about a second while the
    windows load.
  - The page keeps at most 24 windows, about 250 MB.
- **A review by a second agent found 14 defects; 13 are fixed.** Among
  them:
  - playback never stopped at the end of a question;
  - holding an answer key answered every question opened after it;
  - a failed decode stayed silent until a reload; it now retries.

  The one not fixed needs our decision (below).
- **Safari is untested.** It needs version 15.4 or later. If it
  misbehaves, Chrome works.
- **Detections did not change.** `regress session-14-start` was
  identical. The NAS peak store was not touched.

### Decisions waiting

- **The Rust version.** The workspace declares Rust 1.85, but the code
  already needs 1.88 (`let` chains in the core). Raising it to 1.89
  would allow a lock so that two `audition` servers cannot write one
  answer log at once. Without the lock, run one at a time. Two servers
  collide only if two answers land within the same few milliseconds.
- Still waiting, unchanged: the containment rule, shared material, the
  possible tier and edits, a clusters search, test evaluation 5.

### Left for later

- **A timeout on FFmpeg.** If the NAS hangs, window requests wait and
  can hold up answers.
- **Report edits.** Detection answers turned into edits of a report, kept
  when the mix is identified again (roadmap, "Report edits").

## Session 15

### What changed

**`identify` at NAS scale takes 31% less time** (experiment 0093). My 2003
mix takes 64-66 s instead of 93-97 s, with 37% less CPU and the same memory
(4.8 GB). The reports are byte-identical. At 33,596 files nearly every
file has a few chance hits in each 10 s window, and the search used to
sort them all, which took 40% of its CPU. It now counts each file's hits
in 4-frame buckets first and sorts only the 1.8% that could line up. Every
file that could form a line is still sorted, so no detection changes.
Nothing to do: the next build includes it.

**The verdicts by ear of 2026-10-12 are applied** (experiment 0090). All 24 new
lines were honoured:

- Two joins at NAS scale: Stratus - Waves (the Sonix WEB file joins
  INCIDENT002's cluster), and the two DYkast "Punishment" uploads (a new
  cluster). 267 -> 268 clusters, 874 -> 877 files in clusters.
- 22 confirm what the clusters already held: 12 files together (the ten
  control joins, the corpus Sonar ~ the PRO012 rip, Falcon - The Stand)
  and 10 apart (the Sonar revision ~ PRO012, Mind Overload, the three
  "Fire" remixes, T-Minus, Dozer, Pyro, Aphrodite ~ Critikal, Night Gasp ~
  Dark Soldier).
- Development scores did not change at either size; the five NAS
  development scans, run again, are byte-identical. In the recall panel 9
  wrong answers fall to 0 (the DYkast pair, now one recording).
- At 262 tracks every verdict names a NAS file, so the 17 clusters in use
  stay.
- **`--cut-sparsest` is no longer needed.** No chain of joins links two
  files judged different. To apply future verdicts:
  `target/release/gunfinger-eval --other-peaks-dir ~/.local/share/gunfinger/nas-dnb-peaks clusters --from-peaks --reuse-pairs`.
  Without the option, a future conflict stops `clusters` and names the
  chain. `pair-review` still prints the command with the option; with no
  chain the option cuts nothing.

### Decision: the containment rule (experiment 0091)

The candidate: a confident play that lies inside a stronger confident play
of a file from another cluster, over the same passage of the mix, is
reported as shared material instead of an identification.

Measured offline on every stored development report, over 24 settings:
which span the stronger play holds (its whole span, or one of its
passages: segments joined across gaps of 10 s or less), a tolerance of 0,
10 or 30 s, and an optional bound on the ratio of hits (none, 0.75, 0.5,
0.25).

- Data it was not designed on (development scans and leave-outs at both
  sizes, four sweeps at both sizes, recall, robust): no wrong confident
  play exists there, so the rule removes nothing and demotes nothing at
  any setting. The nearest right play in a real development mix would need
  110-176 s of tolerance.
- The generated mixes (262 tracks, 108 confident plays, all right) show
  the cost: holding the stronger play's whole span demotes 4-35 right brief
  plays, because a track that returns within 90 s becomes one play
  spanning the brief plays between; a 30 s tolerance demotes 4-30.
  Passages with 0-10 s demote none.
- New shared passages from the verdicts by ear of 2026-10-12 (T-Minus and its
  vocal mix, Dozer and the Notjamie bootleg, Time Is Broken and
  Ignorance), played whole as in experiment 0076: the related file is
  confident in 5 of 7 passages (6 plays). The rule removes 5 of them at
  every setting; the sixth is Dozer's bootleg, where the corpus Dozer has
  no NAS record, so nothing stronger plays. With the played recording's
  cluster left out, 4 of 6 passages keep a confident wrong play and the
  rule removes none.
- 0076's passages, which informed the settings: at passages with 10 s and
  no ratio bound, the 26 passages with a wrong confident play fall to 0,
  but 10 right plays are demoted (The Nine's other rips lie inside the
  VIP's stronger play). If the played file were missing and only weaker
  rips remained (simulated), 2 passages would lose the right recording and
  keep the VIP. A bound of 0.5 avoids that but keeps the VIP (55-63% of
  The Nine's hits).

What it shows: when the played recording is indexed and stronger, the
rule removes related versions without cost on development data. What it
does not show: it cannot remove a related version when the played
recording is not in the library, which is when a confident wrong answer
does the most harm, and no development mix has a confident related
version for it to act on. The data that informed the settings (0076, the
2003 mix's copy ratios) cannot test them.

Recommendation: do not adopt it yet. To measure it further, the
setting to test is passages, 10 s tolerance, with and without a 0.5 bound,
on a new development mix that plays versions whole (a VIP or remix played
for minutes), chosen by hand.

### Decision: the possible tier at NAS scale (experiments 0092, 0094)

A play is possible when it stays below the confident rule but its
strongest segment reaches the threshold (60 hits today). Counted on the
stored NAS-scale development reports, at 60, 80, 100 and 130 hits:

| Threshold | 60 | 80 | 100 | 130 |
|---|--:|--:|--:|--:|
| Possible plays per hour of the two development mixes | 2.9 | 2.3 | 1.2 | 0.6 |
| of which another version of the track playing; not explained | 4; 1 | 3; 1 | 2; 0 | 1; 0 |
| True plays only the tier finds (mixes, sweeps, recall at NAS scale) | 0 | 0 | 0 | 0 |
| Damaged excerpts at 262 tracks found only as possible (278), hidden | 0 | 48 | 76 | 128 |

- In the two mixes the tier shows the Synergy Remix of Star Trails (79
  hits), the Stakka remix of Clockwork (94; two identical files, one
  play), Phantom 2018 (146) and Fracture's edit of Phantom Force (118):
  versions of the track playing. The fifth is Rift feat. Souldrop -
  Meltdown (98 hits) during Phantom Force in my 2003 mix; see "To judge by
  ear".
- No true play in the development mixes, sweeps or recall at NAS scale is
  found only as possible: the tier adds no recall there. Brief clean plays
  (the window grid's 10-20 s, the generated mixes' 20-26 s plays) are
  possible because they span two windows, with 162-754 hits, so no
  threshold up to 130 hides them. What a higher threshold hides is heavily
  damaged audio: at 262 tracks, of 278 transformed excerpts found only as
  possible (combined damage, noise at 0 dB, blends at equal level), 48, 76
  or 128 at 80, 100 or 130.
- Chance: the strongest unrelated candidate in the sweeps is still
  Clockwork against Simon Static's Rubba Rock at 64 hits (experiment 0052),
  32 in the stakka mix, 36 on audio not in the index. Meltdown's 98 is
  either chance or shared material. Experiment 0094 tested the one
  relation a pair search misses: Meltdown does not hold the Phantom Force
  passage re-pitched (resampled to undo the pitch, it gives Phantom Force
  nothing, where a re-pitched copy gives 1,292 hits). Phantom Force alone,
  sped up as the mix plays it, still lines up with Meltdown at 53 hits, and
  the alignment lands 12 s apart in Meltdown in two searches (8 bars): a
  pattern that repeats in Meltdown, such as a shared drum break, or a
  rhythmic coincidence.

Recommendation: the decision hinges on Meltdown. ADR 0006 sets the tier
at twice the strongest unrelated candidate. If Meltdown shares material
with Phantom Force, that is 2 x 64 = 128, so 130: the mixes would show 0.6
possible plays per hour instead of 2.9, losing three of the four version
notes, and no development true play. If Meltdown is chance, it is 196,
close to the confident rule of 240: ADR 0006 drops the tier only when it
reaches the rule, but a tier from 196 to 239 hits would show almost
nothing. Until it has been checked by ear, keep 60: possible plays never count as
identifications, and at 2.9 per hour they cost little reading.

### To judge by ear

Do Rift feat. Souldrop - Meltdown and Digital & Spirit - Phantom Force
share a drum break or a loop? This is not a same-or-different verdict:
they are different tracks. The question is whether the 98 hits that put
Meltdown in my 2003 mix during Phantom Force come from material both
use (then the possible tier's threshold would be 130) or from chance
(then 196); see "Decision: the possible tier" above.

- Mix: /Users/razorjack/Projects/OpenSource/gunfinger/corpus/sets/razorjack-2003-03-29/razorjack-2003-03-29.mp3, 12:31-13:15 (Phantom Force alone, at +4.5%)
- Meltdown: /Volumes/atlas/Music/dnb/__youtube_archivists/i-witness-dnb/Rift feat. Souldrop - Meltdown.m4a, 0:44-1:42 (two searches line the mix up with 0:44-1:25 and with 0:56-1:42; Meltdown 4% slower than the mix at the same pitch)
- Phantom Force: /Volumes/atlas/Music/dnb/__youtube_archivists/i-witness-dnb/Digital & Spirit - Phantom Force.m4a, 2:47-3:33 (the same moment of the mix); the PHUD1 rip the mix matches best: /Volumes/atlas/Music/dnb/__full_scene/1999/1999-11/1999-11-01/phantom_audio-digital_and_spirit-phud1-1999-sour/a-digital_and_spirit-phantom_force-phud1-1999-sour.mp3, 2:44-3:30

### Worth knowing

- **No listening file or manifest was written.** `docs/pair-verdicts.txt`,
  `work/audition/answers.jsonl` and the manifests are unchanged. The NAS
  was only read; the NAS peak store was not written (33,596 records,
  revision `f9f9bc706195a71b` at the end, as at the start).
- **Detections did not change.** `regress session-15-start` was
  identical at the end, `scripts/check.sh` green, and the 2003 mix at NAS
  scale gave a byte-identical report with the final build.
- **A review by a second agent** found no defect in the search change and
  seven in the session's analysis scripts; all are fixed. One changed
  figures: the possible-tier count took a play of two identical files as
  two plays, so the figures above are the corrected ones (2.9 possible
  plays per hour at 60, not 3.5 as first written).
- **Timings.** The machine had other load during the timing runs;
  the old and new builds ran alternately (A B B A) to share
  it.
- **The test set was not touched**, nor were the five test-derived
  versions.

### Decisions waiting

- **The containment rule:** not adopted. To measure it further, choose a
  development mix that plays a VIP or remix for minutes.
- **The possible tier:** keep 60 until there is an answer to the question
  above; then 130 (a shared break) or 196 (chance).
- Still waiting, unchanged: the Rust version and the answer-log lock,
  report edits, test evaluation 5.

### Left for later

- **The next speed step.** Grouping each window's hits by file is now
  51% of the search's CPU and query analysis 30% (0093).
- **Shared breaks.** If Meltdown and Phantom Force share a break, other
  pairs in a drum & bass collection will too; measuring how often they
  reach the possible tier would come next.

## Session 16

### What changed

**`identify` prints a tracklist by default** (ADR 0017, our principle of
2026-10-13). My 2003 mix against the NAS collection
(`--playback turntable`), as `identify` and `show` now print it:

```text
 1.    0:01  Kemal and Rob Data - Konspiracy  (also: A-Konspiracy-Kemal_and_Rob_Data-sour, Kemal + Rob Data - Konspiracy)
 2.    2:10  Future Cut - Sex Drive  (titles disagree: Future Cut - The Specialist)
 3.    6:12  Narcosis - Escape Route  (also: AA-Escape_Route-Narcosis-sour)
 4.   10:48  Digital and Spirit - Phantom Force  (also: Digital & Spirit - Phantom Force)
 5.   11:31  Digital & Spirit - Phantom 2018 (possible; shares material with Digital and Spirit - Phantom Force)
 6.   13:46  Digital & Spirit - Phantom Force (Fracture's Astrophonica Edit) (possible; shares material with Digital and Spirit - Phantom Force)
 7.   14:40  Tee Bee - Two Faced
 8.   17:23  Omni Trio - Kinetic
 9.   24:24  Carlito & Addiction - Into Music
10.   29:13  Omni Trio - Artificial Life (Harp Tune)
11.   31:04  Total Science - Yvon Is On
12.   34:54  Marcus Intalex & ST Files - Balderdash  (also: Marcus Intalex & S.T.Files - Balderdash)
13.   39:06  Spirit - Out Of Control
14.   42:13  Dj Hidden - Skektics
```

- The table of plays is still there: `-f human`. JSON, CSV, the cue
  sheet and the timeline are unchanged. With several recordings each
  gets a `== name` heading; a recording with nothing found says `no
  tracks found`. The tracklist has no colour, so it copies as it is.
- **One entry per track.** Skektics used to be two lines, `42:13 DJ
  HIDDEN - skektics` and `42:15 Dj Hidden - Skektics`: the alien5ive
  upload has about 40 s more lead-in than the two vinyl rips, so the
  plays did not line up as one recording (experiment 0095). Plays over
  the same passage whose files' artist and title tags are equal (letters
  and digits, ignoring case) are now one entry however they line up. In
  reports of the current library this changes only that entry; files
  without both tags never join by title. The JSON report is unchanged.

**A file that hangs no longer stops a run** (ADR 0018). When a run of
FFmpeg or FFprobe writes nothing for two minutes, Gunfinger kills it and
the file fails with a message naming the likely cause:

```text
<path> could not be read: nothing came from `ffmpeg` for 120 s, so it was
stopped. The storage holding the file may be unresponsive, such as a NAS
that hangs; check that the file opens, then try again
```

- `index`: the file is listed as failed and the run ends unsuccessfully,
  as for any failure, but the file is **not** remembered as damaged: the
  next `index` tries it again without `--retry-skipped`. The other files
  carry on.
- `identify`: that recording fails with the message; with several
  recordings the others are still searched.
- The listening page: a window that hangs answers `503` with the message,
  and the page retries.
- Two minutes is deliberately long: a NAS waking its disks or an SMB
  reconnect can take tens of seconds, and a false stall would fail a whole
  `identify`. A healthy decode writes all the time it runs, so a long
  mix on a slow share is not affected. The value is fixed, not a setting.

**`audition` takes a lock on its answer log.** A second `gunfinger-eval
audition` on the same log says another server is answering into it and
exits (use the open page, or stop the first with Ctrl-C). Verdict lines
are checked and written under a lock on `docs/pair-verdicts.txt`, which
`pair-review` takes too. The workspace now needs Rust 1.89 or later.

### Worth knowing

- **No listening file or manifest was written.** `docs/pair-verdicts.txt`,
  `work/audition/answers.jsonl` and the manifests are unchanged (digests
  as at the start). The NAS was only read; the NAS peak store was not
  written (33,596 records, revision `f9f9bc706195a71b` at the end, as at
  the start).
- **Detections did not change.** `regress session-16-start` was identical
  with the final build and `scripts/check.sh` green.
- **The timeout was not tried against a real hung share** (none was
  available). It was checked with stand-in programs that hang: `identify`
  ended after 120 s with the message above and the stand-in was killed.
  In a test with a short timeout, `index` on a temporary store failed a
  file whose decode hangs, did not remember it, and indexed it on the
  next run.
- **Still blocking on a hung share:** listing the library, reading a
  file's size and date, and opening the recording before the search.
  These are not FFmpeg calls; the roadmap's "A share that hangs" lists
  them.
- **The test set was not touched**, nor the candidate mixes of experiment
  0065.
- **A review by a second agent** found two defects in the night's code
  and several smaller points; all are fixed:
  - `pair-review` checked for verdicts only when it started. A verdict
    given on the listening page while `pair-review` had the same item open
    could be contradicted by the next key, which leaves the verdicts file
    unreadable until the line is removed by hand. It now reads the file
    again under the lock and writes nothing for an item judged meanwhile.
  - A share that stays hung leaves a few threads behind for each stalled
    file. Had the system refused a new thread, the run would have crashed;
    now that file fails with the error.
  - A decode that writes only error messages for two minutes counts as
    running, so a damaged file on a slow share ends as damaged (and is
    remembered) rather than as a stall.
  - `titles disagree` is now decided over the whole entry, including
    plays joined only through a third; no stored report's tracklist
    changes.
  - A tracklist of an older report without tags reads each file's tags
    once, not once per mention.
  - Tests that run the real FFmpeg beside a hanging stand-in now allow
    5 s instead of 0.5 s, so a busy machine cannot fail them.
- **The same track can still be listed twice in one case:** a possible
  play inside a confident one is marked as sharing material, and such a
  play is never grouped (as this session's brief set), even when its tags
  match.
  It would print as `X (possible; shares material with X)`. No stored
  report of the development mixes has such a line.

### Decided tonight, open to change

- Tags that differ only in case name one entry, printed once (the
  strongest file's spelling).
- The table keeps the name `-f human`, with no alias.
- The stall timeout is 120 s and not a setting.
- A second `audition` server exits instead of waiting for the first.

### Decisions waiting

- Still waiting, unchanged: the containment rule, the possible tier's
  threshold, test evaluation 5.

### Left for later

- **The next speed step**, on an idle machine (the machine had other load this session,
  so no timings were taken).
- **A run that meets many stalls in a row** waits two minutes per file on
  each worker. Stopping it early would be a separate decision.

## Session 17

### What changed

**`identify` at NAS scale is 28% faster.** My 2003 mix against the NAS
collection (`--store-only --playback both`), the old and new builds
alternating on the idle machine: 44.1-44.3 s instead of 61.0-62.1 s,
377-382 s of CPU instead of 482-487 s, 4.33 GB of memory instead of
4.79 GB. The reports are byte-identical, and `regress` finds the same
detections in 152-154 s instead of 201 s. Three changes (experiments
0096-0098):

- A window's hits are grouped by track a part of the library at a time
  (4,096 files), taken straight from the index's posting lists, so the
  grouping stays in the processor's cache.
- Peak picking no longer branches on comparisons whose outcome changes
  from bin to bin: about 3 times faster, the same peaks. It runs in
  every search and every `index`.
- The 33,596 record headers are read on `--jobs` threads, and the mix is
  decoded while the index loads. The first search of the day, with the
  store not in memory, has its index loaded after 4.6-4.7 s instead of
  9.0-9.2 s.

**A first `index` is about 4% faster**, with every peak record and tag
note identical (temporary stores only): the corpus library 49.4-49.5 s
instead of 51.3-51.7 s; the NAS folder `__youtube_archivists/error-bat`
(18.9 h of audio) 35.2-35.4 s instead of 36.6-36.7 s. FFmpeg's decoding
and resampling is now 86% of a first index's CPU, and the run is within
13% of FFmpeg alone; its options are fixed, so little is left to gain
there (experiment 0099). A full first index of the NAS collection:
about 1.78 h instead of 1.84 h.

**A new kind of listening question**, `gunfinger-eval questions shared
PAIRS.json`: do two different recordings share material where they line
up (a break, a loop or a sample), or is the match chance? Its answers
(`shared`, `chance`, `unsure`) go to the answer log only, never to the
pair verdicts.

### To judge by ear

The two strongest matches between different recordings not yet heard,
both below the possible tier in the sweeps. Open them with:

```sh
target/release/gunfinger-eval audition work/audition/shared/s17-shared.json
```

(add `work/audition/questions/*.json` to have the queue as well). They
are outside the default queue. `S` shared material, `C` chance, `U`
unsure.

1. **Ed Rush & Optical - Watermelon ~ 300 Baud - Monotone (Mobiuz &
   Cycom Remix).** Do they share material, or is it chance?
   - A: `/Users/razorjack/Projects/OpenSource/gunfinger/corpus/library/Ed Rush & Optical - Watermelon.mp3`
   - B: `/Volumes/atlas/Music/dnb/__youtube_archivists/3zb-gangstep-dnb/300 Baud - Monotone (Mobiuz & Cycom Remix).opus`
   - The sweep found Monotone at 41 hits from Watermelon at 2:00.
     Searched whole, the strongest alignment is Watermelon 1:52-2:38
     against Monotone 0:34-1:18 (62 hits), key-locked: Monotone, played
     4.1% slower at its own pitch, lines up. It stops lining up at 2:38
     in Watermelon (1:18 in Monotone). The same stretch of Monotone,
     0:32-1:29, lines up again with Watermelon at 0:54, 2:54, 3:31,
     4:41, 5:30 and 6:00, always at that speed.
2. **Falcon - Fractles ~ Ryme Tyme - We Enter (Optical Remix v.3.0 Full
   Instrumental).** Do they share material, or is it chance?
   - A: `/Users/razorjack/Projects/OpenSource/gunfinger/corpus/library/extra/a. falcon - fractles.mp3`
   - B: `/Volumes/atlas/Music/dnb/__youtube_archivists/butch-french/Ryme Tyme - We Enter (Optical Remix v.3.0 Full Instrumental).opus`
   - The sweep found We Enter at 40 hits from Fractles at 3:54. Searched
     whole, the evidence is weak: the strongest alignment is Fractles
     8:34-9:17 against We Enter 0:26-1:11 (20 hits), turntable: We
     Enter, played 2.8% faster and higher, lines up. It stops lining up
     at 9:17 in Fractles (1:11 in We Enter). Seventeen alignments of
     7-20 hits, most onto We Enter 0:23-1:11, at speeds from +2.7% to
     +5.9%.

Speed matching (`M`) plays B at A's tempo; for the key-locked pair it
also moves Monotone's pitch, so turn it off to hear Monotone as it is.

### Worth knowing

- **No listening file or manifest was written.** `docs/pair-verdicts.txt`,
  `work/audition/answers.jsonl` and the manifests are unchanged (digests
  as at the start). The page was checked on the new questions with a
  copy of the verdicts file and an answer log in `/tmp`; nothing was
  answered. The NAS was only read; the NAS peak store was not written
  (33,596 records, revision `f9f9bc706195a71b` at the end, as at the
  start). Scratch stores were removed after the comparison.
- **Detections did not change.** `regress session-17-start` was
  identical with the final build (720 of 720 sweep queries, all 87, 93
  and 79 scan detections) and `scripts/check.sh` green (352 tests).
- **The timings were taken on the idle machine**, the old and new builds
  alternating (A B B A). Other processes used 0.2-1 core
  throughout, and about half a core more in two indexing runs.
- **The roadmap and the calibration register no longer call the 63-70
  hits between different recordings chance.** Every one checked since
  session 10 shares material (the answers by ear of 2026-10-12: Rubba Rock is
  the 64 from 9,200 assets), so the earlier proposal of a tier of
  130-140 at NAS scale rested on a match that is not chance. The
  strongest match not checked has 41 hits, the first question above. No
  threshold changed.
- **`--jobs` now also sets the threads that read the record headers**
  (`identify`, `explain`, `stats`, `doctor`). The system CPU of a search
  rose from about 6 s to 11-14 s with them; the wall time fell.
- **A catalog error waits for the first decode.** An error such as an
  empty index now appears once the first recording is decoded (seconds
  for a mix on a local disk), because both run at once.
- **A saved index whose posting lists are out of order is refused** and
  rebuilt, instead of crashing the search; no index Gunfinger writes is
  like that. The check runs while the index is read, at no measurable
  cost.
- **The test set was not touched**, nor the candidate mixes of experiment
  0065.
- **Two reviews by a second agent** found no change in output and two
  defects, both fixed:
  - The test of the new grouping covered only one part of the library,
    so a fault in moving from one part to the next would have passed.
    It now spans three parts, one without hits, and fails with that
    fault.
  - The grouping needs each posting list ordered by file, as every
    index Gunfinger writes is; a damaged or hand-made saved index that
    was not would have crashed the search. Such a file is now refused
    and rebuilt (above).
  - In the new producer: a pair named twice now gives an error naming
    it, an alignment of no length gets no shaded span, and markers less
    than 0.5 s apart are one stop for Up and Down.

### Decided this morning, open to change

- Parts of 4,096 files for grouping hits: 1,024 and 8,192 were slower.
- The shared-material questions follow every alignment of the strongest
  search of the audio, also the weak ones, since they are what the
  question is about. Their file sits outside the default queue, as my
  2003 mix's detections do.
- The evaluation harness lists records on every core whatever its
  `--jobs`; it passes its jobs where it builds an index.

### Decisions waiting

- Still waiting, unchanged: the containment rule, the possible tier's
  threshold (every match above 60 between different recordings checked
  so far shares material), test evaluation 5.

### Left for later

- **The next speed step for `identify`**: what is left is the offset
  pre-filter (about 22% of the CPU), grouping a part (20%), reading the
  postings (16%) and analysing the mix (26%).
- **A first `index`** is bound by FFmpeg, whose options are fixed.

## Session 18

### What changed

**The third development mix finds every track named, and nothing
wrong.** DJ Vertical on Nocny Trans, 2002-08-23 (experiment 0100):

| Run | Identified | Wrong | Possible, no track |
|---|--:|--:|--:|
| 262 tracks | 9 of 9 | 0 | 0 |
| NAS scale | 17 of 17 | 0 | 4 |
| NAS scale, 3 tracks left out (seeds 2026-2028) | 14 of 17 each | 0 | 4 |

There is no miss and no wrong play to explain. The weakest track is
Bulletproof - Anthrax (361 hits at 262 tracks, 364-386 at NAS scale; the
confident line is 240). Blindside's tease (38:00-38:48) is the possible
first part of both Blindside plays, confident from 39:31 and 39:42. The
other versions listed as related
share too little audio with the one played to come near a confident play
(0-38 hits in `pair`), except the Klute Remix of We R The Ones (166-263
hits), which shows as possible, and SUICIDE004's "We R The Ones 3",
which is the VIP itself. With a track left out, no other recording
becomes confident in its slot; the possible plays there are the same
ones as with every track indexed (Mampi Swift - 2000 in Stab 2 Def's
slot, the Klute Remix in the VIP's).

**The default tracklist names a right file for every track** (`identify
--store-only`, the configuration; experiment 0100). It has 21 entries.
Each of the 17 tracks has one confident entry holding a right file; 16
carry the right title. Entry 15 reads "Klute - We R The Ones 3 (titles
disagree: Klute - We R The Ones VIP)": SUICIDE004's "We R The Ones 3" is
the same recording as the VIP upload. No confident entry shows only a
wrong title. The other four entries are possible plays, each marked as
sharing material with the confident play over it:

- Mampi Swift - 2000, during Stab 2 Def (The Kraft is Mampi Swift; 827
  hits between the two in `pair`);
- the Klute Remix of We R The Ones, twice (two files; plays marked this
  way are never grouped), during the VIP;
- `1.mp3`, an untagged file at the NAS root, during Cannibal: its last
  31 s hold a piece of Cannibal 6% faster.

**`identify` is faster, with the same reports** (experiments 0104-0106):

- With a current saved index, my 2003 mix at NAS scale takes 43.0-44.5
  s instead of 44.2-45.3 s (3%): the index's postings are read and
  checked on a second thread while the first computes the file's
  digest.
- **The first search after `index` changes the store** (the saved index
  is then stale and is built again) takes 53-55 s instead of 116-130 s:
  the index is now built on every core, in 11-12 s instead of 73-87 s.
  The saved index it writes is byte-identical to the one the old build
  wrote.
- `stats` takes 20 s instead of 25 s.

### Decision: the containment rule (experiment 0101)

On the new mix, at 0091's setting (passages, 10 s, with and without the
0.5 bound), the rule has nothing to do: no wrong confident play to
remove, no right play demoted, at both sizes and in the leave-outs. The
closest call would need 99 s of tolerance. With the strongest file of
every identified track dropped, the tracks that also play confidently
from a weaker copy (13 of 39 at 262 tracks, 45 of 57 at NAS scale, in
the scans; the leave-outs were not simulated) keep it under the rule.
All 432 rows of 0091 are unchanged under the new clusters.
**Recommendation: do not adopt it.** On real mixes it has removed
nothing and cost nothing; its gain shows only on constructed passages,
and it cannot help where the played recording is missing. If it is adopted anyway, passages with 10 s
and the 0.5 bound never lost a right recording.

### The possible tier (experiment 0102)

The answers by ear of 2026-10-14 (Watermelon ~ Monotone and Fractles ~ We
Enter share material) are in the roadmap's "The possible tier at NAS
scale" and the calibration register. The new mix, at NAS scale:

| Threshold (hits) | 60 | 80 | 100 | 130 |
|---|--:|--:|--:|--:|
| Possible plays per hour, new mix | 3.6 | 3.6 | 2.7 | 1.8 |
| Possible plays per hour, three development mixes | 3.2 | 2.8 | 1.8 | 1.1 |

All four possible plays of the new mix share material with the track
playing (above); none is chance. No true play of the new mix is found
only as possible, at any threshold, nor in the leave-outs. A higher
threshold hides one Klute Remix file at 100 and both at 130; `1.mp3`
and Mampi Swift stay up to 130. No threshold changed.

### To judge by ear

No listening question this time: the ear is not needed for anything
measured.

- Track 17 (ID, 1:01:25-1:04:40 in
  `/Users/razorjack/Projects/OpenSource/gunfinger/corpus/sets/nocny-trans-vertical-2002-08-23/vertical.mp3`)
  has no candidate: nothing at least possible in any run, and the
  strongest alignment there has 9 hits, which is chance level.
- The TI029 C side is settled by `pair` (below).

To hear the facts behind the manifest proposals below:

1. **The alien5ive "cannibal" upload is not Cannibal.** `pair` finds no
   alignment at all between
   `/Volumes/atlas/Music/dnb/__youtube_archivists/alien5ive/FUZE RECORDINGS [ FUZE 29 ： SONIC - cannibal - ] drum and bass.m4a`
   and
   `/Volumes/atlas/Music/dnb/__youtube_archivists/butch-french/Sonic - Cannibal.m4a`
   (the file the scan credits, 860 hits at 55:24-58:33 in the mix). The
   alien5ive file is the same recording, end to end, as
   `/Volumes/atlas/Music/dnb/__youtube_archivists/3zb-gangstep-dnb/Sonic - Change.m4a`
   and
   `/Volumes/atlas/Music/dnb/__youtube_archivists/butch-french/Sonic - Change.m4a`
   (one duplicate cluster, 100% coverage).
2. **The TI029 C side is neither version.**
   `/Volumes/atlas/Music/dnb/__full_scene/-= Tech Itch Recordings =-/TI029 - Dieselboy - Patriot Games EP [2000] (Tech Itch Recordings)/C_-_Dieselboy_-_Descent.mp3`
   and
   `/Volumes/atlas/Music/dnb/__full_scene/dnb_scene - 2000 - Part 3/2000-11/dieselboy-patriot_games_ep-ti029-2000-sour/c-dieselboy-the_descent-sour.mp3`
   are one recording (30,783 hits over all 7:13). Against
   `/Volumes/atlas/Music/dnb/__youtube_archivists/i-witness-dnb/Dieselboy - The Descent (Phunckateck VIP).opus`
   the strongest alignment has 6 hits; against the 1999 original,
   `/Volumes/atlas/Music/dnb/__full_scene/1999/1999-12/1999-12-14/palm_pictures-dieselboy-the_descent-1999-sour/a-dieselboy-the_descent-1999-sour.mp3`,
   7 hits. Both are chance level.
3. **`1.mp3` holds part of Cannibal.** In `/Volumes/atlas/Music/dnb/1.mp3`
   (3:59, untagged), 3:28-3:58 lines up with Cannibal 1:53-2:25 (177
   hits), played 6% faster; it stops lining up at the file's end. In the
   mix it shows at 56:48-57:19, during Cannibal.

### Manifest proposals (to apply manually; nothing was changed)

In `corpus/sets/nocny-trans-vertical-2002-08-23/tracklist.toml`:

1. Track 15 (Sonic - Cannibal): take
   `second-library/__youtube_archivists/alien5ive/FUZE RECORDINGS [ FUZE 29 ： SONIC - cannibal - ] drum and bass.m4a`
   out of `reference` (it is the same recording as the two Change
   uploads, item 1 above), perhaps as a `# related:` line saying so.
2. Track 14 (Klute - We R The Ones (VIP)): move
   `commercial_suicide-suicide004-klute-2002-sour/b-klute-we_r_the_ones_3-sour.mp3`
   from `# related:` to `reference` (the same recording as the VIP
   upload: 7,106 hits, 99-100% coverage).
3. Track 4 (The Descent (Phunckateck VIP)): the two `# unclear:` TI029 C
   files hold neither the VIP nor the 1999 original (item 2 above). The
   note "may hold the original or this VIP" could say that, and the
   lines could become `# related:`.

Scores do not change with these: the scoring already counts duplicate
cluster members, and the alien5ive file is not credited anywhere.

### Worth knowing

- **No listening file or manifest was written.** `docs/pair-verdicts.txt`,
  `work/audition/answers.jsonl` and every manifest, the new one
  included, are unchanged (digests as at the start). The NAS was only
  read and the NAS peak store was not written (33,723 records, revision
  `c835c5531c325274` at the end, as at the start). The NAS report folder
  as it was before this session's runs, with the 268 clusters, is kept
  in `work/s18/before/`.
- **Detections did not change.** `regress session-18-start` was
  identical with each kept change and with the final build
  (`work/bin/s18-final/`); `scripts/check.sh` green (355 tests).
- **Two stale saved indexes were rebuilt.** The store had changed since
  they were written (33,723 records now), so the harness's and the CLI's
  saved indexes for the NAS store were built again at 03:41 and 03:48,
  over the old files.
- **The duplicate clusters were rebuilt** with the new manifest: 268 ->
  271 clusters, 877 -> 886 files. New clusters hold Stab 2 Def (4
  files), Mutated (2 uploads), the Konflict Nephelim (2), the We R The
  Ones VIP with "We R The Ones 3", and the alien5ive "cannibal" with the
  two Change uploads. The Molten Beats vinyl/CD clusters of System Error
  and Iron Lung are gone: no query of this run reaches them (one of the
  files is held out for the recall panel). Every stored NAS development
  score is unchanged under the new clusters.
- **The timings were not taken on an idle machine.** Other processes
  used 0-8.7% of a core during the timing series, and up
  to 0.6 core more. The builds alternated (A B B A),
  which shares slow drift between them.
- **The test set was not touched**, nor the candidate mixes of experiment
  0065.
- **A review by a second agent** found no change in output and no hang
  on any error path. It found weak spots; all but one are fixed:
  - The test of the threaded index load never reused a buffer, so a
    fault there would have passed; it now reads six chunks and fails
    with that fault.
  - The threaded build had no test of a record that cannot be read or
    one that fails in a later batch; both are tested now.
  - Code that turned a crash of the reading thread into an error could
    never run; the crash now surfaces as a crash, as before.
  - Several numbers in experiments 0100-0106 did not match their data
    (for example, the stale-index time above was first given from a
    run that does not save the index: 51 s); all are corrected.
  - One thing is left as it is: a damaged peak record with a large
    tail of garbage is now read whole before it is refused. Gunfinger
    writes records of a few hundred kB.

### Decided this morning, open to change

- The index is built 32 records at a time: batches of 128 were no
  faster and used 2% more memory.
- Reading peak records changed with the faster index load (0105): with
  the load change alone, the compiler stopped inlining the byte-by-byte
  record reading, and reading records slowed down (`stats` and building
  a stale index, measured). Records are now read whole and parsed from
  memory, which does not depend on that.
- Three ways of computing the pre-filter's slots once were measured and
  not kept (experiment 0103): the best saved 1.3% of CPU with no change
  in wall time.

### Decisions waiting

- The containment rule (recommended: do not adopt), the possible tier's
  threshold (no match between different recordings checked so far is
  chance), test evaluation 5, and the three manifest proposals above.

### Left for later

- **The search itself** is what is left of `identify`'s time on a
  current saved index: about 39 of 44 s. The pre-filter, grouping and
  taking hits from the lists are about 60% of its CPU and analysing the
  mix about 31%; no identical-output change found this session cut them.
- **The start of a search**: reading the 33,723 record headers (1.9 s)
  and loading the index (3.0 s, beside decoding the mix).
- **The two lost Molten Beats clusters** (above): a `clusters` run that
  also queries the recall panel's validation half would probably find
  them again; until then, a run that scores those files would count the
  other rip as wrong.

## Session 19

### What changed

Nothing in Gunfinger changed: `identify` finds the same plays and prints
the same reports in the same time as before the session (the final
binaries are byte-identical to the start build's). The session measured
the two proposals of the external review we chose
(`docs/research/2026-10-14-matching-proposals.md`).

**Proposal 1, integer arithmetic in the offset pre-filter: measured, not
kept** (experiment 0108). It keeps detections exact: the same lines,
0.017% more files passed on to sorting, byte-identical reports. But the
pre-filter and the sort after it became only about 13% cheaper, measured
against the unchanged analysis in two profiles, below the 20% the review
required before any timing. Computing the slot was a smaller part of the
filter than the review estimated; the rest is most likely counting each
hit into its slot and reading the hits. The code was removed.

**Proposal 2, FFT lengths of the turntable rungs: the diagnostic**
(experiment 0109). Each turntable rung analyses the mix with a window of
`round(1024 / speed)` samples, and some of these lengths are slow for
the FFT library: at 985 samples (the rung at 1.040) analysing a frame
takes 22.8 us, 20.3 of them in the FFT; at 1024 samples, 3.7 us. Measured on 60 s of my 2003 mix on one core, and
weighted by how much each rung analyses in a real `identify`:

| Window lengths | First-pass turntable analysis | With the key-locked rungs |
|---|--:|--:|
| today | 73.2 s | 86.6 s |
| nearest even (the review's proposal) | -21.8% | -18.4% |
| small prime factors, within 1 sample | -25.5% | -21.6% |
| small prime factors, within 2 samples | -36.5% | -30.8% |
| fastest measured, within 2 samples | -44.0% | -37.2% |

Builds that use the last two in the first pass were timed and not kept
(their reports differ). My 2003 mix at NAS scale with the
configuration (turntable playback) took 28.0 s and 27.4 s instead of
30.9 s (9-11% less); with `--playback both`, 46.2 s and 45.1 s instead
of 48.8 s (5-8% less). Memory did not change. Building the FFT plans is
0.04% of the analysis, so a plan cache would gain nothing and was not
tried.

### Decision: is a retrieval session for the FFT lengths worth running?

A window up to 2 samples (0.21%) longer or shorter changes the peaks, so
detections can change. On the 2003 mix the same files are confident, but
hits and boundaries move and one more possible play appears (Omni Trio -
Secret Life (Silent Storm Remix), 30:03-30:42, 70-71 hits). Before such a
change could be adopted, a retrieval study has to show it loses
nothing:

- a variant that changes the first pass only (the second pass keeps
  today's lengths), named in reports so that baselines and saved reports
  tell it apart;
- the diagnostic sweep first, adding speeds between rungs, several sample
  offsets, damaged excerpts, other rips, blends and short plays, with the
  first pass's lines traced before confidence and key-lock-only runs as
  controls;
- then the three development mixes at 262 tracks and NAS scale with
  leave-outs, and the generated mixes with a boundary tolerance fixed in
  advance;
- the review's criteria: no new wrong confident play, no lost right
  confident or possible file, shared-material matches reported apart,
  and at least 5% less wall time on an idle machine.

That is one session of about four hours: about 45 minutes to build and
test the variant, 1.5-2 hours of runs, an hour of comparison and half an
hour of timing.

**Recommendation: worth one session, not urgent.** The gain is real
(about 3 s of a 31 s search with the configuration) and the change is
small, but it would be the first speed change that alters peaks, and
precision comes before speed (ADR 0017). If it runs, test the rule
"small prime factors within 2 samples": the fastest measured lengths
gain 2 points more, but they are a table fitted to this machine and
this version of the FFT library.

### Worth knowing

- **No listening file or manifest was written.** `docs/pair-verdicts.txt`,
  `work/audition/answers.jsonl` and every manifest are unchanged (digests
  as at the start). The NAS was only listed, by `doctor`; the NAS peak
  store was not written (38,105 records, revision `b285da9984422fb5` at
  the end, as at the start).
- **Detections did not change.** `regress session-19-start` was identical
  with the final build (720 of 720 sweep queries, all 87, 93 and 79 scan
  detections) and `scripts/check.sh` green (357 tests).
- **The NAS times of sessions 17 and 18 are not comparable with these.**
  The store holds 38,105 records (33,723 in session 18): the 2003 mix at
  NAS scale now takes 48-50 s with `--playback both` (44 s before) and 31
  s with the configuration. The CLI's saved index was current.
- **The timings were contended.** Other processes used up to about one core. The
  builds alternated (A B B A). Nothing was adopted on these timings:
  proposal 1 was stopped by profile shares that agreed in a contended
  and a quieter series, and proposal 2 changed nothing.
- **The test set was not touched**, nor the candidate mixes of experiment
  0065, nor the recall panel's validation half.
- The harness's stale saved index (`work/indexes/library-nas-dnb-peaks.index`,
  3.8 GB, made at 33,723 records) is still there; nothing this session
  needed it. The next NAS-scale harness run rebuilds it.
- **A review by a second agent** found no change in output and agreed
  with both decisions. It found that one test of the removed code had
  lost its rounding case (an offset of 2 - 2^-53 rounds to exactly 2);
  corrected in the kept copy (`work/s19/lines.rs.p1`, 2 - 2^-52) and
  passing. It also found rounding and wording errors in the experiment
  notes, all corrected: experiment 0108 now leads with the comparison
  against the unchanged analysis (about 13%) rather than the share of
  CPU (7.5-9.5%, which understates a saving), and 0109's per-frame times
  are whole-analysis times, not FFT times.

### Decided this session, open to change

- Split profiles of the search stood in for the review's replay of
  captured hits, which no tool does yet.
- A plan cache counted as worth trying only if building plans took at
  least 3% of `identify`'s CPU; it takes 0.04% of the analysis.
- The "small prime factors" rule breaks ties by the length nearest 1024
  / speed, then the shorter one.

### Decisions waiting

- Whether to run the FFT-length retrieval study (above).
- Still waiting: the containment rule (recommended: do not adopt), the
  possible tier's threshold, test evaluation 5, and session 18's three
  manifest proposals.

### Left for later

- **The search's largest stages** at 38,105 records (a profile of the
  2003 mix): the pre-filter 26% of CPU, analysing the mix 25%, grouping
  hits by file 24%, taking hits from the posting lists 11%. Proposal 3
  of the research note (part boundaries computed once) addresses part of
  the last; proposals 3-5 are not measured.

## macOS and Linux (2026-10-14, not a numbered session)

### What changed

Gunfinger, the harness and the listening page now support macOS and
Linux (ADR 0020; AGENTS.md, "Platforms"). Nothing has run on Linux yet:
the Linux code was type-checked from the Mac, and the choice of each
command is tested here for both systems. On the Mac nothing behaves
differently: the same commands run, and only some messages changed.

- On Linux the listening page opens with `xdg-open`, and the reveal
  button opens the file's folder (on the Mac the Finder still selects
  the file). The button's tooltip now reads "Show the file in its
  folder". Use Chrome or Chromium there; Firefox has not been tried.
- Messages that tell the user to install FFmpeg name `brew install ffmpeg`
  and `sudo apt install ffmpeg`.
- `gunfinger-eval robust` speaks with eSpeak NG (`espeak-ng`) on Linux,
  so its speech conditions compare only between runs on one system.
- Detections, reports, the peak store and the saved index did not
  change (`regress` identical).

### Before using the Linux machine

- **Store-only searches** with a copy of the NAS peak store work with
  `--store-only`, with no `library` in the configuration, or with
  `library = "/Volumes/atlas/Music/dnb"` kept even though that path does
  not exist there. Without `--store-only`, another library path is
  refused, because the store names its library. The first search builds
  the saved index once.
- **With the NAS mounted on Linux**, mount it at `/Volumes/atlas` (a
  mount point, not a symbolic link, which is resolved before the paths
  are compared), or put the Linux path in the copy's `library.txt`.
  Pass the mount's path to `gunfinger-eval --second-library-root`,
  whose default is the Mac's.
- **Memory:** the Linux machine runs 32 workers by default, the Mac 10, and each
  worker keeps its own hit buffers. If memory runs short, set `jobs`.
- **Before any `index` from Linux against the NAS store**, mount the NAS
  and run, read-only:

  ```sh
  python3 -I scripts/analysis/store_identity.py <copy of nas-dnb-peaks> <library root on Linux>
  ```

  On the Mac it printed: 149 paths with two Unicode forms, all stored
  decomposed (NFD); 32,000 modification times with fractions of a
  second; all 169 files looked up found under the stored names, with
  size and time equal to the nanosecond. The second argument is the
  library's root on Linux, such as `/mnt/atlas/Music/dnb`, not the
  share's. On Linux, any file "only under the other Unicode form" or
  with a different time means `index` would index those files again
  under new names, or decode up to the 32,000 files with fractional
  times again. Then do not index from Linux; tell the next session,
  which needs an ADR to fix it.
- **The first `scripts/check.sh` on Linux** is the first real test
  there. A stand-in script test that fails with "Text file busy" is a
  known Linux race between test threads, not a platform defect: run the
  tests again.
- **Results may differ slightly between the machines.** FFmpeg's build,
  the FFT library's vector code (NEON on the Mac, AVX on the Linux machine) and
  the maths library differ, so a mix's peaks may differ in rare frames
  and hit counts by a few. A `regress` baseline is compared only on the
  machine that made it. To see the difference, run the same
  `identify --store-only -f json` of a development mix on both.

### Decided, open to change

- On Linux the reveal button opens the folder without selecting the
  file. Selecting it needs a D-Bus call that only some file managers
  answer; for a development tool the fallback it needs was not worth
  it.
- To type-check the Linux code from the Mac, Homebrew's `rustup`
  (keg-only, so Homebrew's `cargo` stays the default) now holds a Rust
  1.98.0 toolchain with the Linux target, 620 MB in `~/.rustup`.
  `scripts/check.sh` runs that check only with that toolchain first on
  `PATH` (AGENTS.md has the command); otherwise it prints a line and
  skips.
  `brew uninstall rustup` and deleting `~/.rustup` remove it.

## Session 20

### What changed

Nothing in Gunfinger changed: `identify` finds the same plays and prints
the same reports (`regress session-20-start` identical). The session
built the shared-material challenge set (the panel) from the judgements
by ear, ran its development half at NAS scale, and ran proposal 5 of
the research note (the identifiability diagnostic) on it. The harness
gained NAS-scale `shared` runs with four new kinds of query and a new
command, `gunfinger-eval identifiability`.

In short: about a minute of a passage that two versions share makes the
other version a confident wrong title at NAS scale, in 12 of the 14
directions measured, whether or not the played record is indexed. The
information that tells the versions apart is there in 10 of 14
directions: the other version's evidence is sparser than a rip of the
played record, clearly so where that record has rips of its own, and
the confidence rule counts hits without regard to how sparse they are.
But true plays in the development mixes are as sparse, so a threshold on
sparseness would lose true plays before it removed half of the wrong
ones. In the three development mixes the problem does not occur: with
every played recording left out, no related version is confident.

### The panel

`docs/panels/shared-material-seed-2026.json` (experiment 0110, made by
`scripts/analysis/shared_panel.py`). 12 recording families from the judgements
by ear:

- the 13 `different` lines of `docs/pair-verdicts.txt`;
- the two `shared` answers on the listening page: Watermelon ~ 300
  Baud - Monotone (Mobiuz & Cycom Remix), and Falcon - Fractles ~ Ryme
  Tyme - We Enter (Optical Remix v3.0);
- two judgements recorded in the docs: Clockwork ~ its Stakka remix
  (experiment 0026) and China Cup ~ The Nine (after session 4).

Each recording holds its rips: session 18's duplicate clusters, and 6
rips the clusters never saw, joined by the clustering rule (one
alignment covering 80% of the shorter file): Dozer's RSD 2023 copy, We
Enter (Optical RMX) at 87%, Mind Overload's RAMM 23 A, Playground
Shades' APH039 A, Sonar's PRO 012 A 1998 and The Nine's BCR01 A 1998.
Pairs no verdict covers are candidates, not members (the listening
questions below). The five test-derived versions are not in it.

Each judged pair gives two directions (each file played once).
`gunfinger-eval pair` gives each direction's shared passage, its
strongest alignment; under 25 hits the direction is a control (no shared
passage).

| Half | Families | Recordings | Directions (controls) | Files |
|---|---|--:|--:|--:|
| development | Dozer, Fire, Playground Shades, Pyro, Sonar, Watermelon | 14 | 16 (2) | 24 |
| validation | Clockwork, Fractles, Mind Overload, Night Gasp, T-Minus, The Nine | 13 | 14 (4) | 42 |

The split is by family with seed 2026; The Nine fell into the validation
half. That half has not been searched: it is kept to validate a
statistic frozen on the development half.

**The oracle** (`docs/brief-20.md`, written before anything was
measured): each query against each index is right confident, wrong
confident, possible only or nothing. A confident play of any rip of the
played recording is right. A confident play of the related recording,
of another version in the family, or of any other file is wrong. Files
whose tags carry the family's title but which nobody judged are counted
apart (none was confident). The blend's partner is set aside. With the
played recording left out of the index (the main case), possible only
or nothing is the right answer.

**To run the development half** (32 minutes, 5.4 GB of memory, 337 MB of
rendered queries in `work/shared/`, in a folder named after the plan and
a digest of it, so a changed plan renders afresh):

```sh
gunfinger-eval --other-peaks-dir ~/.local/share/gunfinger/nas-dnb-peaks \
  shared docs/panels/shared-material-seed-2026-development.json
python3 -I scripts/analysis/shared_outcomes.py docs/panels/shared-material-seed-2026.json \
  work/reports/library-nas-dnb-peaks/shared-material-seed-2026-development.json outcomes.json
```

The first command runs every query; the second applies the oracle.
`shared_panel.py freeze` writes only the development half's plan, so the
validation half's plan is written by the session that validates a
frozen statistic.

### Today's outcomes (experiment 0111)

Each direction's passage is played alone (10, 20, 30 s), looped (60,
120, 240 s), running 10, 20 or 30 s past the passage into the played
file's own material (beyond), after 5, 10 or 20 s of own material
(before), as a dull 64 kbps rip (weak: alone 30 s, beyond 20 s), under a
blend of Skynet & Stakka - Decoy at equal level over the context, and as
a 5 s break looped to 60 and 240 s. Three indexes: everything; the
played recording left out (the main case); the played file left out
while its other rips stay (the positive control).

| Played recording left out | alone | looped | beyond | before | weak | blend | break | all |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| Queries | 42 | 42 | 42 | 22 | 28 | 14 | 28 | 218 |
| Wrong confident | 4 | 34 | 33 | 8 | 15 | 12 | 0 | 106 |
| Possible only | 20 | 2 | 3 | 8 | 9 | 0 | 2 | 44 |
| Nothing | 18 | 6 | 6 | 6 | 4 | 2 | 26 | 68 |

- With everything indexed, the same 106 queries show a wrong confident
  title beside the played file's confident play; the other 112 are right
  confident (56), possible only (17) or nothing (39).
- With the played file left out and another indexed rip kept (6
  directions, 89 queries), that rip is confident in 65; in 44 of them a
  wrong title is confident too. Watermelon's other rip is a copy the
  corpus file stands for, so it is not indexed; the harness now skips
  that control, which removes those 17 queries and changes nothing else.
- No unjudged or unrelated file was confident anywhere. In the two
  control directions no file passed 25 hits; the related file reached
  11.
- It takes enough of the passage: at 30 s or less only dense copies
  (Fire's Rogue Unit Remix and VIP, both Pyro directions); Dozer and its
  bootleg from 58 s (the 48 s passage and 10 s more, or a 60 s loop),
  Fire's remixes from 60 s. A 5 s break looped to 240 s is never
  confident (at most 32 hits).

Every wrong confident query of the main case (hits of the wrong title):

- Dozer played, its 2025 Notjamie Bootleg confident (241-286): looped
  60, 120, 240 s; beyond 10, 20, 30 s; weak beyond; blend. The bootleg
  played, Dozer confident (252-271): the same 8 queries.
- Fire (original mix) played against the Rogue Unit Remix: the
  Remix (493-502) looped 120 s and beyond 10 s; the Nasty Jungle Remix
  (282-567) looped 60 s, beyond 20 and 30 s, weak beyond, blend; the VIP
  (499) looped 240 s. Against the VIP, over the same passage: the
  Rogue Unit Remix (493-502) looped 120 s and beyond 10 s; the Nasty
  Jungle Remix (282-567) looped 60 and 240 s, beyond 20 and 30 s, weak
  beyond, blend.
- Fire (original mix) played against the Nasty Jungle Remix: that remix
  (318-583) looped 60, 120, 240 s, beyond 10, 20, 30 s, weak beyond,
  blend. The Nasty Jungle Remix played: the original (321-600), the
  same 8.
- Fire's Rogue Unit Remix played: the VIP (897-4,473) in 13 of 17
  queries (all but alone 10 and 20 s and the two breaks). The VIP
  played: the Rogue Unit Remix (993-4,497), the same 13.
- Pyro - Time Is Broken played: Pyro - Ignorance (244-445) alone 30 s,
  looped 60, 120, 240 s, beyond 10, 20, 30 s, weak beyond, blend.
  Ignorance played: Time Is Broken (348-425) alone 30 s, looped, beyond,
  before 5 and 10 s, both weak, blend (12).
- Trace - Sonar played: the Mark System Revision (240-269) looped 240 s,
  weak beyond, blend. The revision played: Sonar (244-334) looped 60,
  120, 240 s, beyond 10, 20, 30 s, weak beyond, blend.
- Watermelon ~ Monotone and Playground Shades ~ Interlude: none (at most
  46 hits).

### The diagnostic (experiments 0112, 0114 and 0115)

For each direction, the played file's passage was searched against each
file of the related recording alone (R, the related file of the plan,
first) and against each rip of the played recording alone. Three
measures over the passage: audio (the spectra's correlation along the
alignment), peaks (the verifier's share of the file's peaks found) and
pairs (hits per second of the passage). Audio and peaks have a chance
value; pairs has none, although the rule asked for one. The rips of the
played recording are the noise floor; where it has only one file, the
poorest rip in the half (a poor upload of Pyro - Time Is Broken). The
rule was written before the run.

| Direction | Outcome by the rule | What decides it |
|---|---|---|
| Dozer, both | 3: survives, the evidence ignores it | R's audio 0.58 and peaks 0.33-0.34, below the RSD rip's 0.88 and 0.79 (Dozer played) or the pooled 0.66 and 0.38 (the bootleg played) |
| Fire, all six | 3 | R's audio 0.35-0.36 and peaks 0.34-0.35, below the JM2 rip's 0.72-0.75 and 0.61-0.64 (the original played) or the pooled floor (a remix played) |
| Pyro: Time Is Broken played | 3, by pairs only | Ignorance's pairs 2.3 a second against the rip's 6.0; its peaks (0.59) are above the rip's (0.38) |
| Pyro: Ignorance played (a 30 s passage) | 1: no distinguishing information | Time Is Broken's audio 0.716 against the poor upload's 0.655; the next poorest rip is 0.723, so this is thin |
| Sonar played | 3 | the revision's audio 0.57 and peaks 0.25 against the PRO 012 A rip's 0.82 and 0.61 |
| Sonar's revision played | separated today | R (`DJ Trace - Sonar.m4a`) is never confident; the PRO 012 A upload of the same recording is (334 hits), and by its measures (0.63, 0.39, 3.75 a second) it is outcome 3 by pairs only |
| Watermelon, both | separated today | the other file reaches 46 hits at most |
| Playground Shades | control | no shared passage |

How firm this is:

- Clear in the five directions whose played recording has rips of its
  own (Dozer played, Fire's original three times, Sonar played): R sits
  far below the poorest rip on every measure.
- Thin where the floor is the pooled upload: the peaks of Dozer's
  bootleg played and of Fire's remixes played sit 0.025-0.05 below it.
- Pyro - Time Is Broken played rests on pairs per passage second. R
  lines up in blocks at two offsets, so the strongest block undercounts
  it: per second of its aligned span R has 10.8 hits against the rip's
  9.8, which would make it outcome 2 (lost before peaks and pairs).
- No direction is outcome 2 by the rule.

Where the evidence falls: for Fire, R's alignment ends within 4 s of the
passage's end, while the played file runs on for 30 s; for Dozer,
Sonar's revision and Ignorance it runs 5-29 s past the passage, so the
copied region is longer than `pair`'s passage. In a loop each repeat
starts a new alignment: 2-16 detections of the related file, the
strongest 244-600 hits.

Outcome 3 compares R with a clean copy of the played record. Under the
same damage (a weak rip, a blend), R's evidence is sparse: 1.5-12.4
hits per second of its span, against 25-53 for the played file and
6.5-24 for its other rips. Experiment 0114 asked whether true plays in
the development mixes are denser. They are not: the 102 confident detections
credited to referenced tracks in the three development mixes (scanned
with the peak verifier at NAS scale) have 1.5-31.9 hits per second
(median 6.7 in the Vertical broadcast) and a verifier share of
0.24-0.77. A cut that keeps 95% of them removes 8 of the 106 wrong plays
(hits per second) or 12 (verifier share), far from the half proposal
5's review asks for. That comparison is not like-for-like: the wrong
plays come from clean renders of short queries (spans of 21-156 s, and
nested queries counted separately, 40 of them the unjudged Fire remix
and VIP), the true plays from mixes (62-418 s). Experiment 0115 looked
for wrong plays in the mixes themselves, with all 40 referenced tracks
left out: no file was confident except the newer uploads of the same
recordings (below), and the related versions stayed possible at 79-197
hits. So the development mixes give nothing to test a cut against.

### What it means for the next step

The three options the prompt names, each ours to decide:

- **A statistic** is justified in principle (outcome 3). The obvious
  one, a cut on how sparse the evidence is, is not supported by this
  session's comparison, though that comparison leans against it. The
  other candidate in the data is where R's alignment stops while the
  music continues (Fire). Either needs a like-for-like test: real mixes
  in which a related version plays over a long shared passage, which
  the development mixes do not have. Then the validation half.
- **A representation change** is not indicated: no direction lost its
  information before the peaks and pairs by the rule (Pyro - Time Is
  Broken would, counted per aligned span).
- **Reporting ambiguity** fits Pyro's 30 s passage (outcome 1) and is
  the fallback if no statistic separates: a map of passages that
  library files share, built from the files present, would mark a
  confident play whose passage another indexed version also holds. A
  change to the report is our decision (ADR 0017).

My recommendation: find or render mixes that play a related version over
a long shared passage (the library's Fire, Dozer or Sonar versions would do),
then measure the alignment-stop evidence and sparseness on them against
true plays, with the rule written first. The roadmap's other evidence
items and the FFT lengths' retrieval study now have this panel as a
required check.

### Listening requests

17 questions, all `same` or `different` for a pair, in
`work/audition/s20/`: `01-candidates.json` (16) and
`02-fire-remix-vip.json` (1). Open them with

```sh
gunfinger-eval audition work/audition/s20/01-candidates.json work/audition/s20/02-fire-remix-vip.json
```

The packs with clips and sheets are `work/audition/s20/pack/` and
`pack-2/`. Each verdict is about the version, not the sound quality or
the tags: `same` if both files would get the same tracklist line, and
`different` if they differ in even a few bars. If different, say whether
the part that lines up is shared material. A `different` verdict makes
the pair a candidate member of the panel; the frozen panel itself does
not change.

The question for each: "Are A and B one recording (two rips of the same
record, however they sound) or different recordings (they differ in even
a few bars)? If different, B is placed where it lines up with A: is that
shared material?" The times below are where they line up; they stop
lining up at the end of each range.

1. Asked because: the panel (validation, fractles): joined by the clustering rule at 87% coverage.
   A: `/Volumes/atlas/Music/dnb/__youtube_archivists/dfect-dnb/Ryme Tyme - We Enter (Optical RMX).m4a` (6:30.0)
   B: `/Volumes/atlas/Music/dnb/__youtube_archivists/butch-french/Ryme Tyme - We Enter (Optical Remix v.3.0 Full Instrumental).opus` (6:35.0)
   Lines up: A 0:00.5-5:40.0 with B 0:00.4-5:39.6 (2127 hits, B at -0.0%); A 5:40.9-5:59.8 with B 2:16.4-2:35.4 (107 hits, B at -0.0%).
2. Asked because: the Vertical mix: possible during Stab 2 Def.
   A: `/Volumes/atlas/Music/dnb/blade-the_kraft-bld002-2000-sour/aa-the_kraft-stab_2_def-sour.mp3` (7:29.3)
   B: `/Volumes/atlas/Music/dnb/__youtube_archivists/i-witness-dnb/Mampi Swift - 2000.m4a` (5:59.1)
   Lines up: A 0:50.0-2:10.0 with B 0:27.9-1:47.9 (787 hits, B at +0.0%); A 6:30.0-7:27.0 with B 5:01.2-5:58.2 (600 hits, B at +0.0%); A 0:11.5-0:49.7 with B 0:00.4-0:38.6 (419 hits, B at -0.0%).
3. Asked because: 0076, not judged.
   A: `/Volumes/atlas/Music/dnb/__full_scene/dnb_scene - 2000 - Part 1/2000-06/pwl_int-prizna_feat_demolition_man-nlbcdx18-1995-sour/02-prizna_feat_demolition_man-fire_(original_mix)-sour.mp3` (3:26.6)
   B: `/Volumes/atlas/Music/dnb/__full_scene/dnb_scene - 2000 - Part 1/2000-06/pwl_int-prizna_feat_demolition_man-nlbcdx18-1995-sour/06-prizna_feat_demolition_man-fire_(splash_remix)-sour.mp3` (5:39.6)
   Lines up: A 2:08.5-3:06.3 with B 3:18.9-4:20.1 (319 hits, B at +5.9%); A 0:53.9-1:58.6 with B 1:54.2-3:02.7 (299 hits, B at +5.8%); A 0:22.9-0:42.6 with B 1:09.9-1:30.8 (183 hits, B at +5.9%).
4. Asked because: 0076, not judged.
   A: `/Volumes/atlas/Music/dnb/__full_scene/dnb_scene - 2000 - Part 1/2000-04/va-drum_and_bass_assassins-dbascd01-2000-sour/209-usual_suspects-killa_bees-sour.mp3` (6:43.4)
   B: `/Volumes/atlas/Music/dnb/__full_scene/1999/1999-10/1999-10-05/renegade_hardware-armageddon-rh2000cd-1999-promo-sour/08-usual_suspects-killa_bees_(optical_and_fierce_remix)-sour.mp3` (5:23.4)
   Lines up: A 0:02.4-0:39.9 with B 0:27.3-1:04.8 (280 hits, B at -0.0%); A 6:20.1-6:38.9 with B 0:29.4-0:48.2 (177 hits, B at -0.0%); A 3:42.1-3:59.0 with B 0:48.2-1:05.1 (84 hits, B at -0.0%).
5. Asked because: 0076, not judged.
   A: `/Volumes/atlas/Music/dnb/__full_scene/dnb_scene - 2000 - Part 1/2000-06/pwl_int-prizna_feat_demolition_man-nlbcdx18-1995-sour/02-prizna_feat_demolition_man-fire_(original_mix)-sour.mp3` (3:26.6)
   B: `/Volumes/atlas/Music/dnb/__full_scene/dnb_scene - 2000 - Part 1/2000-06/pwl_int-prizna_feat_demolition_man-nlbcdx18-1995-sour/03-prizna_feat_demolition_man-fire_(urban_shkedown_rmx)-sour.mp3` (6:01.2)
   Lines up: A 0:22.1-1:15.3 with B 0:33.6-1:28.2 (209 hits, B at +2.6%); A 1:24.1-2:09.1 with B 2:32.6-3:18.6 (129 hits, B at +2.5%); A 2:41.8-2:54.4 with B 4:03.2-4:16.2 (72 hits, B at +2.6%).
6. Asked because: the 2003 mix: possible during Phantom Force.
   A: `/Volumes/atlas/Music/dnb/__youtube_archivists/i-witness-dnb/Digital & Spirit - Phantom Force.m4a` (6:27.1)
   B: `/Volumes/atlas/Music/dnb/__youtube_archivists/i-witness-dnb/Digital & Spirit - Phantom 2018.m4a` (5:43.1)
   Lines up: A 1:40.3-2:29.2 with B 1:01.3-1:50.1 (195 hits, B at -0.1%); A 2:50.1-3:29.2 with B 1:09.7-1:48.8 (156 hits, B at -0.1%); A 1:00.5-1:29.8 with B 1:14.5-1:43.9 (140 hits, B at -0.1%).
7. Asked because: 0076, not judged.
   A: `/Volumes/atlas/Music/dnb/__full_scene/dnb_scene - 2000 - Part 1/2000-06/pwl_int-prizna_feat_demolition_man-nlbcdx18-1995-sour/02-prizna_feat_demolition_man-fire_(original_mix)-sour.mp3` (3:26.6)
   B: `/Volumes/atlas/Music/dnb/__full_scene/dnb_scene - 2000 - Part 1/2000-06/va-jungle_massive_4-hfcd46-2cd-1995-sour/205-prizna_ft_demolition_man-fire(urban_shakedown_mix)-sour.mp3` (5:17.0)
   Lines up: A 0:00.4-1:17.6 with B 0:11.5-1:30.6 (194 hits, B at +2.6%); A 1:24.1-2:09.6 with B 2:32.6-3:19.2 (147 hits, B at +2.5%); A 2:41.8-2:54.4 with B 4:03.2-4:16.2 (66 hits, B at +2.6%).
8. Asked because: the Vertical mix: possible during Cannibal.
   A: `/Volumes/atlas/Music/dnb/__youtube_archivists/butch-french/Sonic - Cannibal.m4a` (6:49.1)
   B: `/Volumes/atlas/Music/dnb/1.mp3` (3:59.4)
   Lines up: A 1:52.7-2:24.9 with B 3:28.1-3:58.4 (177 hits, B at -5.8%).
9. Asked because: 0076, not judged.
   A: `/Volumes/atlas/Music/dnb/bad_company-fear_lp/02 bad company-the nine.mp3` (6:59.8)
   B: `/Volumes/atlas/Music/dnb/__youtube_archivists/i-witness-dnb/Bad Company - The Nine (ChaseR Bootleg).opus` (3:51.3)
   Lines up: A 0:47.0-1:25.0 with B 1:36.6-2:14.5 (145 hits, B at -0.0%).
10. Asked because: the 2003 mix: possible during Phantom Force.
   A: `/Volumes/atlas/Music/dnb/__youtube_archivists/i-witness-dnb/Digital & Spirit - Phantom Force.m4a` (6:27.1)
   B: `/Volumes/atlas/Music/dnb/__youtube_archivists/i-witness-dnb/Digital & Spirit - Phantom Force (Fracture's Astrophonica Edit).m4a` (5:00.0)
   Lines up: A 4:00.7-4:27.9 with B 2:09.4-2:36.7 (141 hits, B at -0.1%).
11. Asked because: 0076, not judged (A is a rip of the original mix).
   A: `/Volumes/atlas/Music/dnb/__full_scene/dnb_scene - 2000 - Part 1/2000-06/va-jungle_massive_2-hfcd43-2cd-1994-sour/108-demolition_man-fire-sour.mp3` (3:29.9)
   B: `/Volumes/atlas/Music/dnb/__full_scene/dnb_scene - 2000 - Part 1/2000-06/pwl_int-prizna_feat_demolition_man-nlbcdx18-1995-sour/01-prizna_feat_demolition_man-fire_(urban_shakedown_ed)-sour.mp3` (3:36.7)
   Lines up: A 1:26.8-1:58.5 with B 2:08.0-2:40.5 (140 hits, B at +2.5%); A 0:22.5-0:43.0 with B 0:28.8-0:49.9 (101 hits, B at +2.5%); A 0:14.1-0:21.0 with B 0:14.8-0:21.8 (89 hits, B at +2.5%).
12. Asked because: 0076, not judged.
   A: `/Volumes/atlas/Music/dnb/__full_scene/dnb_scene - 2000 - Part 1/2000-06/pwl_int-prizna_feat_demolition_man-nlbcdx18-1995-sour/02-prizna_feat_demolition_man-fire_(original_mix)-sour.mp3` (3:26.6)
   B: `/Volumes/atlas/Music/dnb/__full_scene/dnb_scene - 2000 - Part 1/2000-06/pwl_int-prizna_feat_demolition_man-nlbcdx18-1995-sour/07-prizna_feat_demolition_man-fire_(smokey_joe_remix)-sour.mp3` (7:00.8)
   Lines up: A 0:07.6-0:19.0 with B 0:01.0-0:12.4 (124 hits, B at +0.0%); A 0:22.0-0:37.5 with B 5:10.8-5:26.3 (102 hits, B at +0.0%).
13. Asked because: 0076 (not judged) and the 2003 mix.
   A: `/Volumes/atlas/Music/dnb/__youtube_archivists/alien5ive/AUDIO BLUEPRINT [ ABPR013 ： KEMAL & ROB DATA - star trails - ] drum and bass.opus` (7:21.6)
   B: `/Volumes/atlas/Music/dnb/__youtube_archivists/i-witness-dnb/Kemal & Rob Data - Star Trails (Synergy Remix).m4a` (3:05.1)
   Lines up: A 1:50.7-2:29.0 with B 0:21.2-0:59.3 (118 hits, B at -0.6%).
14. Asked because: the Vertical mix: possible during the VIP.
   A: `/Volumes/atlas/Music/dnb/__youtube_archivists/i-witness-dnb/Klute - We R The Ones VIP.m4a` (7:28.1)
   B: `/Volumes/atlas/Music/dnb/__youtube_archivists/i-witness-dnb/Klute - We R The Ones (Klute Remix).opus` (7:09.0)
   Lines up: A 6:20.4-7:09.9 with B 5:59.0-6:48.4 (108 hits, B at -0.0%); A 2:40.4-3:40.0 with B 2:41.2-3:40.7 (84 hits, B at -0.0%).
15. Asked because: 0076, not judged.
   A: `/Volumes/atlas/Music/dnb/__youtube_archivists/i-witness-dnb/Eddy Woo - Tsunami.m4a` (5:59.1)
   B: `/Volumes/atlas/Music/dnb/__youtube_archivists/butch-french/Eddy Woo - Tsunami (Remix).m4a` (5:37.1)
   Lines up: A 0:02.2-0:19.9 with B 0:07.4-0:25.0 (100 hits, B at -0.7%); A 0:20.3-0:39.7 with B 0:08.8-0:28.1 (78 hits, B at -0.7%).
16. Asked because: the 2003 mix: possible during Phantom Force; heard on 2026-10-12 to share the drum break, no verdict line.
   A: `/Volumes/atlas/Music/dnb/__youtube_archivists/i-witness-dnb/Digital & Spirit - Phantom Force.m4a` (6:27.1)
   B: `/Volumes/atlas/Music/dnb/__youtube_archivists/i-witness-dnb/Rift feat. Souldrop - Meltdown.m4a` (6:34.1)
   Lines up nowhere above 60 hits (strongest: A 1:31.2-2:09.6 with B 0:44.5-1:20.9, 14 hits).

17. Asked because: the panel's development half (0111): with either one
   played and its recording left out, the other is a wrong confident
   title (up to 4,473 and 4,497 hits), and nobody has judged the pair.
   A: `/Volumes/atlas/Music/dnb/__full_scene/dnb_scene - 2000 - Part 1/2000-06/pwl_int-prizna_feat_demolition_man-nlbcdx18-1995-sour/04-prizna_feat_demolition_man-fire_(rogue_unit_remix)-sour.mp3` (6:40.2)
   B: `/Volumes/atlas/Music/dnb/__full_scene/dnb_scene - 2000 - Part 1/2000-06/va-jungle_massive_3-hfcd44-1995-sour/02-demolition_man-fire_(rogue_unit_vip_remix)-sour.mp3` (5:21.0)
   Lines up: A 0:00.4-2:39.8 with B 0:00.3-2:39.7 (4470 hits); A 4:50.6-6:14.4 with B 3:54.7-5:18.5 (2367 hits); A 2:50.1-3:40.0 with B 2:27.6-3:17.6 (1272 hits).

Questions 1 and 9 involve files of the validation half (Fractles, The
Nine). `pair` aligned them for the sheets, as it did for every family
when the panel was built; no scenario was run on them.

### Proposed changes (to make manually)

- **No manifest change.**
- **The NAS duplicate clusters are behind the store.** At the current
  revision, 9 confident detections in the development mixes count as
  wrong only because their files are newer than session 18's clusters
  (33,723 records). Each covers 98.8-99.9% of the credited file, so the
  clustering rule makes them the same recordings: the hifi-junglist's
  Drum&BassArena (React REACTCD201) disc 2 tracks 3, 4, 5, 7 and 8
  (Spirit - Out Of Control, Total Science - Yvon Is On, Marcus Intalex &
  ST Files - Balderdash, Tee Bee - Two Faced, Carlito & DJ Addiction -
  Into Music) in the 2003 mix, and weedsurfer's Drumsound & Bassline
  Smith - Phoenix and the hifi-junglist's Trace & Fresh - Mutated
  (Version X) (U-PUNK-001 A) in the Vertical mix. Proposal: a later
  session runs `clusters` at the current revision. Not done tonight,
  because the panel was built from the present clusters file and a
  rerun would replace it.

### Worth knowing

- **No listening file or manifest was written.** `docs/pair-verdicts.txt`,
  `work/audition/answers.jsonl` and every manifest are unchanged
  (digests and modification times as at the start). The NAS was only
  read; the NAS peak store was not written (38,105 records, revision
  `b285da9984422fb5` at the end, as at the start).
- **Detections did not change.** `regress session-20-start` was
  identical with the final build (720 of 720 sweep queries, all 87, 93
  and 79 scan detections, the calibrate line known since session 19),
  and the final `gunfinger` binary is byte-identical to the start
  build's. `scripts/check.sh` is green (382 tests) with Homebrew's
  toolchain and with rustup's, which also ran the Linux clippy.
- **Proposal 3 stops** (experiment 0113). Split out of
  `WindowHits::take_part`, its binary searches are 1.3-1.4% of the 2003
  mix's CPU at NAS scale, under the 5% stop point; the copying of hits
  is 9.1%. A directory of part boundaries would save at most that, for
  about 72 MiB more memory. No code was kept.
- **The harness's saved index** (`work/indexes/library-nas-dnb-peaks.index`)
  was rebuilt at 38,127 files (the NAS store and the corpus); it is
  current.
- **The machine was not idle.** Other processes used up to one core and up to
  0.4 more during the profiles. Nothing was adopted on timings.
- **The test set was not touched**, nor the candidate mixes of
  experiment 0065. No scenario or diagnostic ran on either validation
  half; `pair` aligned the new panel's validation files once, for their
  passages when the panel was built and for the sheets of questions 1
  and 9.
- **A review by a second agent** (read only) found one bug, one
  misapplied rule and several overstated or wrong figures; all are
  fixed above:
  - `shared_panel.py join` used an undefined name, so the committed
    script could not rebuild the panel. Fixed; it now reproduces the
    panel byte for byte.
  - For Sonar's revision played, the diagnostic had counted the related
    recording as confident through another rip, while the rule speaks of
    R. It now measures every file of the related recording, applies the
    rule to R, and reports the confident file beside it.
  - Watermelon's positive control was void (its other rip is not
    indexed). The harness skips such a control now; the counts are 6
    directions and 89 queries.
  - Pyro - Time Is Broken's outcome rests on pairs per passage second,
    and 0114 is not like-for-like. Both are now said; 0115 looked for
    wrong plays in the mixes themselves.
  - The harness reports a query it cannot render instead of stopping, as
    the oracle says, keeps its renders under a digest of the plan, and
    leaves silent padding out of the audio measure (5 new tests). The
    diagnostic and the development half were rerun with these fixes:
    every detection of the development half is the same, from fresh
    renders, and the diagnostic's measures moved by at most 0.005 (its
    chance values by up to 0.02).

### Decided this session, open to change

- The panel holds only judged pairs; 0076's unjudged pairs and
  Meltdown ~ Phantom Force (heard as sharing a drum break on 2026-10-12,
  without a verdict line) are listening questions, not members.
- Fractles ~ We Enter is a control: `pair` finds at most 12 hits between
  them, though shared material was heard at a 20-40-hit detection.
- Rips the clusters never saw join a recording by the clustering rule
  (80% coverage) from library audio alone.
- The noise floor is the poorest rip of the played recording; where the
  recording has one file only, the poorest rip over the development
  half (a poor upload of Pyro - Time Is Broken, which decides Pyro's
  outcome 1).
- After step 6 finished early, a follow-up to step 4 (experiment 0114)
  compared the unused evidence with true plays, and after the review a
  like-for-like check (0115); both rules were written before they ran.
- The roadmap now makes this panel a required check: anything that adds or
  changes evidence runs the development half and may add no wrong
  confident query there.
- The diagnostic's rule is applied as written, to R, the plan's related
  file. Where another file of the related recording is the confident
  one (Sonar's revision played), it is reported beside the rule's
  outcome, not in its place. Pairs per aligned span are reported as a
  check, not used.

### Decisions waiting

- The next step for shared material (above).
- The 17 listening questions.
- Whether a later session reclusters the NAS library at the current
  revision.
- Still waiting: the containment rule (recommended: do not adopt), the
  possible tier's threshold, test evaluation 5, session 18's three
  manifest proposals, and the FFT lengths' retrieval study (now required to pass
  this panel).
