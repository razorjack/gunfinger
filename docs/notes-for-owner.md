# Notes for the owner

Findings from the autonomous sessions that are worth your attention, newest
last, after a summary of each session (session 4 first).

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
| Extra rungs (`--extra-rungs 3`) | for: every speed to ±9%; against: 14% more CPU | only if your decks go past ±8% |
| Speed per stretch (`--speed-per-stretch`) | for: more evidence on drifting plays, no CPU; against: no level changes in any set, false candidates gain too | keep opt-in |
| Trimming weak ends (`--trim-ends`) | against: 219 s of true play lost to remove 85 s of overshoot | do not adopt |

**Decisions for you.**

- Adopt skip at 240 (session 3's open decision)? Item 1 adds that it is
  3.5× cheaper at 31,964 assets and peaks at half the memory.
- The link rules, the band profile and extra rungs, as in the table.
  Each changes detections; the band profile also means extracting every
  peak record again.
- Test-set evaluations (3 left): none spent. One evaluation of the
  chosen matcher with what you adopt would confirm it.
- If you have a minute, listen to whether Bad Company - China Cup shares
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
   (you confirmed it by ear) reaches 119 hits, which put that pass's rule
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

**Decisions for you.**

- Adopt the second pass with common hashes skipped, at 240 hits? The
  case is in "The case for adopting the second pass with common hashes
  skipped" below. I would confirm it with one test-set evaluation first.
- Test-set evaluations (3 left): the test mix has never been searched
  with both playbacks, and Sick Note (209 hits) would most likely drop to
  possible under the second pass. One evaluation of each matcher would
  show both; I spent none.
- A 10 s minimum span instead of 3 windows: whether a 15 s play should
  count is your call. Fixing chance lines in chains should come first;
  both change detections and need the full protocol.
- Peaks spread more evenly across frequency bands: it changes the peak
  profile, so every peak record would be extracted again.
- If you have a minute, listen to whether
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
2. Memory is the first wall: the scan at 26,000 assets swapped beside your
   open applications (0012). I did not measure what used the memory: the
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

**Decisions for you.**

- Key lock by default: decided on 2026-10-06, on. Your
  `~/.config/gunfinger/config.toml` sets `playback = "turntable"`.
- The test set has only been searched with turntable playback. One
  evaluation (3 left) would confirm 15/16 with both; I did not spend it,
  since the development protocol passes unchanged (0016).
- Should the fullest 1% of posting lists be dropped by default? It passes
  the protocol, but every hit count changes, so I would confirm it with
  one test evaluation (3 left). `docs/calibration.md` ("Measured, not
  adopted") says how.
- `listen` and `review`: tested by the owner on 2026-10-06; every snippet
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
tempo-only ladder next. If you know whether your target mixes include
key-locked CDJ or digital sets, that decides how much this matters.

### Key lock is solved; the default is unchanged

Experiment 0010: rungs that stretch time but not frequency find every
key-locked excerpt at ±2%, ±5% and +8% (40 of 40 each, hits as strong as
untouched audio), and adding them to the turntable ladder changed nothing
for vinyl-style audio and created no false identification. The cost is
search time: 82 rungs instead of 41, measured later at 1.57 times the
CPU time (experiments 0016, 0018). I left the default on the turntable
ladder because your sets are vinyl, and added `identify --playback both`
(or `key-lock`); key-locked plays are marked `(key lock)`, and `listen`
time-stretches them instead of resampling. If you identify CD or digital
sets, use `--playback both`; making it the default would need the full
evaluation with both ladders first.

### `listen` and `review` were untested with real audio output

This session has no audio device: ffplay reported "audio open failed". I
checked the ffplay commands with `listen --print` and ran the same filters
through FFmpeg into a null output, but I have not heard them.

Update, 2026-10-06: the owner ran `listen` at several moments of the
development mix and `review` with play numbers, `start`, `end` and `at`;
everything worked.

### A hundredfold library barely moves chance; memory is the first limit

Experiment 0012 padded the index with reversed, stretched copies of every
record (up to 21,109 assets, about 80 times your library). Chance
alignments grow slowly: an excerpt's typical strongest wrong candidate went
from 7 to 12 hits, and over the whole development mix at 8,122 assets the
strongest chance alignment was 30 hits (possible needs 60, confident 200).
Results on real tracks did not change at all. Reversed copies contain no
shared breaks or remixes, so this is a lower bound; real false candidates
(remixes, 91-95 hits) remain the ones that matter, and breakbeat hardcore
is the case to measure when you add it.

What did break: the development scan at 26,462 assets needed more memory
than was free beside your open applications; it swapped and was stopped
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
(129); 12 of the 21 pairs are Aphrodite with Aphrodite. When you add
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

### Owner's verdict: the Clockwork remix at 20:22 is shared material

`listen` at 20:30 of the development mix played the mix, Clockwork at 3:32
and the Stakka remix at 6:46 (its possible play). Owner, by ear: at that
moment the remix is the original's lead, an early neurofunk synth, exactly
as in the original, plus a pad; the rest of the remix is a different track
with its own bass and drums. The possible play is therefore a correct
report of shared material, acceptable as it is, and that fragment alone
cannot tell the two recordings apart. It is the same passage experiment
0015 found (129 hits, remix 6:39-6:58 against the original 3:25-3:44).

### Key lock on by default; its cost was 1.57×, not 1.3×

Your decision: both playbacks by default, so digital DJs get key-locked
plays without configuring anything, and vinyl users turn it off in the
configuration file (README, "Key lock and vinyl"). Your own file sets
`playback = "turntable"`, so your searches are unchanged. The harness
default ladder is `both` too; turntable-only reports moved to
`work/reports/ladder-turntable/`. Measured in CPU time on the development
mix, both playbacks cost 261 s against 166 s, 1.57 times; the 1.3 I gave
before came from wall times on a loaded machine (experiment 0018). Same 14
plays, none with key lock.

Two smaller command-line changes went with it: `identify` keeps quiet
about timings and files left out of the index unless you pass `--verbose`
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
(experiment 0019). Detections are identical. On your library, `identify`
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
recording's path differ. Reports you saved before today record none of
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
it needs the full protocol and your judgement on whether a 15 s play
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
are thinner than a single draw suggested. If you have a minute, listen to
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
fullest lists skipped the first pass finds its shared passage (the one you
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

My recommendation, for you to decide: adopt skip with the second pass
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

Item 2 (commit d4d8fb1) searches a block of 12 windows on every rung at
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

Two opt-in rules (commit afc731c; experiment 0030): `--nearby-rungs`
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
recommendation, for you to decide: adopt both rules with whichever
matcher you choose; with skip at 240 they add margin at no cost.

### Speed per stretch: more hits where speed drifts, no level changes

`--speed-per-stretch` (commit afc731c; experiment 0032) lets the second
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
the anchor's band (commit d533142). Anchors at 2-4 kHz account for 64%
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
the machine was loaded by then (inference).

For adoption (variant a): the most evidence per posting, in the real
mix, where it matters; better margins under both matchers. Against:
every peak record must be extracted again (about 0.2 s per track), the
gain rests on one development mix, and related records reach the
possible tier more easily. My recommendation, for you to decide: adopt
variant a after choosing the matcher, confirmed by one test-set
evaluation, and listen to whether China Cup and The Nine share material.

### Trimming weak chain ends costs more true play than it removes

`--trim-ends` (commit afc731c; experiment 0031) leaves lines at either
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
ranges). Against: 14% CPU for plays that may never happen in your
mixes. My recommendation: adopt it only if you play past ±8%.

## After session 4

### Owner's verdict: China Cup and The Nine share their drums

Asked in session 4 (experiment 0034: with the band profile, a held-out
Bad Company - China Cup finds The Nine with 43-50 hits). Owner, as a DJ:
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
identical. The analysis (`docs/nas-plan.md`) waits for your NAS index;
its records predate tags, which a second `gunfinger index` fills in from
the files' headers.

## Session 5 findings

Measurements at the size of your NAS collection, from its peak store
alone (`docs/brief-5.md`). The NAS was never read and the store was not
written.

### The NAS index run was limited by reading the files, not by the CPU

Your run read 181 GB of audio in 17,317 s: 84 Mbit/s, 6.2 s per hour of
audio. The local disk run of experiment 0028 took 1.8 s per hour of
audio, so the CPU would have finished the NAS in about 84 minutes
instead of 4.8 hours (inference: it assumes AAC and Opus decode as
cheaply as MP3). Peaks and postings per second equal the corpus's
(37.5 and 74.8 per second), so the front end treats the YouTube AAC and
Opus files like the scene MP3s. Tags cover 99.3% of records; 144 have
none, 125 of them in the scene folders. Experiment 0035.

### 15 corpus files have no copy on the NAS

For you to check (experiment 0036). `map-library` finds an identical
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
these 15 as corpus files, so the index holds 26,905 assets.
