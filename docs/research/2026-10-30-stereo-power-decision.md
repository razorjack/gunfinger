# Stereo power: what it recovers, what it costs (2026-10-30)

**Status: measured (session 61, experiments 0245-0247; session 62, the
query side alone at NAS scale, 0248-0250; session 67, a polarity check
of the records at indexing, 0262-0263); not adopted.**
The product, its front end and every default are unchanged. The variant
is an opt-in harness front end (`--front-end`, `variant-store`); this
note gives our recommendation for whether to take it further.

## The variant

Each channel gets its own STFT (today's window, size and hop, and each
rung's in the search), and the peaks are picked from `P = |X_L|^2 +
|X_R|^2` with no further scaling. FFmpeg's downmix is `M = (L + R) /
sqrt(2)`; with `S = (L - R) / sqrt(2)`, `P = |X_M|^2 + |X_S|^2`, today's
power plus the side's. A mono source gives today's power: a stereo file
with equal channels exactly in exact arithmetic, a one-channel file too
(FFmpeg's `-ac 2` gives each channel the sample at 1/sqrt(2)). Inverting
either channel leaves `P` unchanged to the bit (a unit test). Records
carry their own profile identifier (`... channels=power-sum`).

## What it recovers

- **Faults in the query** (0245, 0246; 80 indexed excerpts): an
  inverted channel 0 to 80 confident, delays of 1 and 5 ms 70 and 37 to
  80, a silent channel 78 to 78-80. Over the inverted half of 24
  generated mixes, 1 of 100 plays confident to 93-94.
- **Faults in the rip** (0246; only with a store built by the variant):
  the 10 copies of 0197 inverted, silent, 1 ms and 5 ms late, 0, 19, 18
  and 4 of 20 confident to 20 each.
- **Query-only** (0245, today's records): the inverted channel and the
  delays are won back against today's downmix records, at 262 tracks
  and at NAS scale (the generated mixes ran at 262 tracks only); a
  silent channel stays at 78. Mono queries keep every confident and
  possible count (a few best-hit counts move by 1-2), and clean stereo
  queries keep 95% of the hits and every detection.

## What it costs

- **Noise in one channel is worse.** At 0 dB, 29 of 80 confident under
  the downmix, 18-19 with power-sum queries and 23 with both sides
  built by the variant; by inference the downmix adds the shared signal
  in amplitude, 3 dB more than the power sum does.
- **Mono queries against power-sum records lose marginal excerpts**
  (0246): 6 of 38 default `robust` conditions, rendered mono, fall by
  2-4 of 80 (low-pass 800 Hz, needle skip, clipping, pink noise at 10
  dB, blends at 0 and -6 dB); mono queries keep 0.93-0.99 of the hits.
  This fails the registered bar (within 1 of 80). A mono recording of a
  mix (a tape transfer, a mono broadcast) is the case it stands for.
- **Search time.** Power-sum queries cost 12% more wall time and 13%
  more CPU on the Stakka scan at NAS scale (27.6-27.7 s against
  24.6-24.8 s, 32 jobs, the Linux machine), 45-73% more at 262 tracks
  (one scan; `robust`'s median excerpt), and 2% more peak memory (0247).
- **Records.** Density is unchanged (0.1% fewer peaks and postings).
  Building the corpus store takes the same time (by inference because
  FFmpeg's decoding dominates, 86% of a first index's CPU in 0099). Its
  peak memory at 32 jobs was 3 times today's, an upper bound: the
  harness's two-channel decoder (`stereo::decode_stereo`) holds FFmpeg's
  whole output and copies it twice, where the core's decoder streams into
  one buffer.
- **Adoption for records means a new profile and every reference
  rebuilt** (ADR 0003's invalidation rule): the NAS store's 40,678
  records decoded again from the NAS. Scaling the corpus build (29.6 h
  of audio in 15.7 s at 32 jobs) by audio, the 3,433 h measured in 0064
  take about 30 minutes on the Linux machine, and by the growth in
  records since (40,678 against 33,596) about 37; an inference that
  assumes reading over Ethernet keeps up. The Mac's estimate for the
  same 3,433 h was 1.78 h (0099). The saved NAS index is rebuilt too. Old records
  cannot be searched under the new profile, so the corpus store, the
  NAS store and every baseline move together.

## Risks

- No new wrong file appeared for any draw in either phase. At NAS scale
  the only wrong confident answer is session 40's known pair (the
  held-out Cool Rock Stuff excerpt as the Mobb Deep / Urban Takover
  upload), which the downmix also gives on the clean controls; with
  power-sum queries it appears in 16 held-out queries against 7, because
  the variant keeps that excerpt in the fault rows. Wrong possible
  answers among indexed excerpts go from 0 to 1-3 in the recovered rows,
  as many as `stereo-control`'s 2. Query-side adoption would carry that pair into more
  faulted queries. The variant was not run on the shared-material panels, on
  `recall`'s other-rip panels or on generated mixes at NAS scale.
- How often an inverted or delayed channel occurs in the NAS rips or in
  real mixes is still unknown (0196): the gain is certain per fault, its
  frequency is not.
- Noise in one channel and mono queries against new records move the
  other way; both would need their own look before records change.

## Recommendation (ours to take)

Do not change the record profile: the registered bar for an ADR
proposal is not met, and the rebuild moves every store and baseline for
a fault of unknown frequency. The query side alone is the better
candidate: it wins back inverted channels and delays against today's
records, needs no rebuild, keeps mono queries' counts and costs 12% more
search time at NAS scale. It should get its own pre-registration
(at NAS scale: the shared-material panels, `recall`'s other rips, the
generated mixes, the noise conditions and the frozen acceptance rule
of the proposals note) and, if it passes, an ADR for the CLI's
decoding. Faults in the rip stay unrecovered without a rebuild.

## Session 62: the query side alone, against its bar

Session 62 pre-registered one frozen rule (today's confidence rule,
matcher and scoring, applied to power-sum queries against today's
records) and a bar for an ADR proposal (`docs/brief-62.md`), then ran
both front ends on the same rendered queries at NAS scale.

| Bar item | Result | |
|---|---|---|
| 1. No new wrong confident file | none in any run; the known Cool Rock pair only for its own held-out excerpt (5 stereo queries to 14, and its 2 mono `control` queries under both, whose file is not named) | passes |
| 2. No confident identification lost on clean or mono queries (tolerance max(1, 1%)) | panels, mixes, `robust`'s controls and the scans lose none; `recall` loses 9 of 522 clean excerpts, 58 of 721 brief plays, 2 of 174 and net 3 of 171 with other rips only | **fails** |
| 3. Fault gains at NAS scale | inverted, 1 and 5 ms 80 of 80; 93 of 100 plays in the inverted halves of 24 mixes | passes |
| 4. Related versions named | panel 1 326 to 309, panel 2 9 to 9 | passes |
| 5. Noise in one channel | 0 dB 29 to 19 (bar: 15); 10 and -10 dB unchanged | passes |
| 6. Cost | 12% more search time, 12.5% more CPU, 2% more memory (Stakka scan, A B B A) | passes |

**Why it fails: wide sources.** The losses follow the source's width.
Over `recall`'s clean excerpts the variant keeps 0.98 of the downmix's
hits (median) where the side is 20 dB or more below the mid, 0.77 at -15
to -10 dB and 0.39-0.49 above -10 dB; the nine clean losses are one
source at -7.3 dB (Organikismness - Echos), and 51 of the 58 brief plays
lost lie above -15 dB. By inference, a record picked from the downmix
holds the mid's peaks; a query picked from mid plus side adds the side's
peaks where the side is loud, and those are not in the record. By
inference, session 61 did not see it because it ran no `recall` and
`robust`'s excerpts are mostly narrow (0196: 5 of 50 wide); 7.5% of the
NAS rips are wide (0248).

**How often the faults occur (0248).** In 2,000 NAS rips, 5 are
inverted in one channel by the registered rule (about 0.4% of the
store, 0.14-1.25%; 3-4 of them plainly so), 1 delayed and none with one
channel missing. Faults in the mixes searched are still not measured.
An inverted rip is missed whatever the query (0197): only new records
would recover it. By ear (2026-10-30), three of the census's flagged
rips are inverted in one channel (Usual Suspects - Advent, Keltech -
Axis (Low Level Mix), Plasmic - The Weapon); We - Hello World is unsure.

**Recommendation (ours to take).** No ADR proposal: the query side's
registered bar fails, the records' bar failed in session 61, and the
faults it recovers are rare in the rips (about 1 in 250) and not
measured in real mixes. `identify` and `explain` keep FFmpeg's
downmix. If the question returns, a combination that equals today's
power where the mid dominates and recovers an inverted channel where
the side does (for example the larger of the mid's and the side's power
in each bin) would be the next candidate, measured first on wide
sources (`recall`'s panel) and on mono queries; that is an idea, not
measured. The harness keeps `--front-end` and the census
(`stereo-width --other`) for it.

## Session 67: a polarity check of the records at indexing

Session 67 measured the narrower record-side idea: at indexing, decode
both channels, and only when they are strongly anti-correlated pick
the peaks from the downmix with one channel's polarity reversed, (L -
R) / sqrt(2); every other file goes through the core's decoding as
today. The rule was frozen before any search (`docs/brief-67.md`): the
correlation at lag 0 -0.5 or less, strongest there within ±80 samples,
both channels within 20 dB. It is the census's threshold, which takes
96 of 100 of session 42's inverted renders and no other fault. The
harness has it as `variant-store --repair-inverted` and
`stereo-width`'s `repaired`; the product is unchanged.

**What the check would change in the store (0262).** In 6,766 distinct
NAS files (a new census of 6,000 and 0248's 2,000), it repairs five:
Paradox - Filmscape, Usual Suspects - Advent, Plasmic - The Weapon and
Keltech - Axis, plainly inverted (the side 13-22 dB above the mid, so
narrow or narrower once repaired), and We - Hello World (8.6 dB),
uncertain. That is about 0.08% of the store, about 31 records (95%:
0.02-0.28%). 0248's 0.4% rested on 3 uploads in 667. The new census
alone, as registered, repairs three (Filmscape, Advent, Hello World):
2 of 3 plainly inverted, below the registered 3 in 4; the union of both
censuses, decided after the census because the draw was not the
superset of 0248's that the pre-registration assumed, gives 4 of 5. No
correlation in the census lies between -0.76 and -0.32, so the
threshold is not near any file. Of the new census's 6,000 files, 29
unflagged ones have a 30 s window at -0.5 or less, none in more than
29% of its windows; by inference stereo effects of the recordings
(three rips of one track share them).

**What it recovers (0263).** Against a copy of the NAS store with the
five records repaired (R) and an unchanged copy (T), searched by
`identify` with correct-polarity queries: the four plainly inverted
recordings go from 0 to 48 of 48 confident excerpts and from 0 to 4 of 4
confident plays in generated mixes; Hello World is confident under both
(313 against 1,450 median hits). No other rip of the five exists in the
library by audio, so the cross-rip case is unmeasured. No new wrong
confident answer appeared in any search. In the rip's own polarity
(exploratory) the repair costs what it gains: the four are found by 0
of 48 such excerpts under R. A mix made digitally from the same
inverted file would be that case, a vinyl play would not (an
inference).

**What else moves.** Every other record is byte-identical: 265 of 265
corpus records and 41 of 41 unflagged NAS files rebuilt with the check.
But a NAS regression against R is not identical: 11 of 720 sweep
queries change, 9 of them in the fitted speed of one confident
detection (Cause 4 Concern - Abduction, by up to 2e-4 percentage
points, hits equal) and 2 in sub-threshold candidates (6-7 hits); the
sweep's mean speed errors move in the sixth digit. No confident
detection is gained or lost, and identification counts and calibrate
are unchanged; the other development scans keep every confident
detection, and the Nocny Trans scan loses two 6-hit candidates. The
cause is the lists the matcher sets aside (ADR 0014; 0242, 0254): the
five records change postings, so one list crosses the cut each way;
with today's lists set aside R's sweep equals the baseline in all 720
queries (for the scans, which have no such option, an inference). A
change of postings can move a list across the cut, as session 64's
prune of four files did (0254), though session 52's moved none in the
whole index (ADR 0014); the registered item asked for more than any
change of records can promise.

**How adoption would work (not decided).**

- *A profile bump.* `Profile` gains a channel handling for checked
  records; ADR 0003's invalidation rule then re-extracts every record:
  the NAS store's 40,674 files decoded again from the NAS, both as two
  channels (the check; 0262's rate, 617 files a minute at 16 jobs,
  about 66 minutes, includes the census's lag search over ±80 samples)
  and as today's downmix (about 37 minutes by session 61's estimate),
  unless records are picked from the two-channel decode for every
  file, which would change every record slightly and need its own
  regression. Every baseline and saved index moves with the profile.
- *A per-record flag.* Unflagged records keep their bytes and profile;
  `index` runs the check once over every existing file (about 66
  minutes for the NAS store, then one two-channel decode per new file)
  and rewrites only the flagged files' records, about 31. The store must
  remember which files were checked, and a repaired record must differ
  in something the saved index's provenance holds (today: the sources
  and the profile, not the peaks), for example its own profile
  identifier that the search accepts beside today's; otherwise a saved
  index of the old records would pass for the new ones. Results change
  only through those records and the cut above.

**Recommendation (ours to take).** No ADR proposal from this session:
by the registered rule, item 4 fails as written (and item 5 holds only
on the union of both censuses), although what item 4 was meant to
protect holds (unflagged records byte-identical, no confident answer
gained or lost). The measured case for the check is narrow but clean: it
finds about 1 file in 1,300, recovers the plainly inverted ones in
full, adds no wrong answer and leaves every other record alone. If we
take it further, the next step is to re-register item 4 as "unflagged
records byte-identical; no confident detection gained or lost and no
identification count or calibrate number changed; other changes
accounted for by the set-aside cut", which `--set-aside-like` checks
for `sweep` and `recall` only (`scan` would need the option), and item
5 on a census drawn from today's store; then write the ADR for the
per-record route, the smaller change: about 31 records rewritten after
one pass of about an hour. The Hello World repair costs little either
way (0263), and listening would settle whether it is a fault; Paradox -
Filmscape, plainly inverted by measure, has not been heard. The harness
keeps `variant-store --repair-inverted` and `stereo-width`'s verdict,
which such a re-registration would reuse.
