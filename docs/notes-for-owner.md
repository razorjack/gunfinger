# Notes for the owner

Findings from the autonomous session that are worth your attention, newest
last, after a morning summary.

## Morning summary

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
4. Key lock works: `--playback both` (or `playback = "both"` in the
   configuration file) finds CDJ/master-tempo playback at ±2-8% (0010) and
   passes the full protocol with unchanged thresholds at about 1.3 times
   the search time (0016).
5. Robustness: 32 kinds of damage, no false identification (0009).

**Decisions for you.**

- Should key lock be on by default? It costs about a third more search
  time and your sets are vinyl, so I left it off; `playback = "both"` in
  the configuration file turns it on for a library.
- Should the fullest 1% of posting lists be dropped by default? It passes
  the protocol, but every hit count changes, so I would confirm it with
  one test evaluation (3 left). `docs/calibration.md` ("Measured, not
  adopted") says how.
- Try `gunfinger listen` and `review` once: I could not hear them (no
  audio device here).

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
search time: 82 rungs instead of 41, measured later at about 1.3 times
the search time (experiment 0016). I left the default on the turntable
ladder because your sets are vinyl, and added `identify --playback both`
(or `key-lock`); key-locked plays are marked `(key lock)`, and `listen`
time-stretches them instead of resampling. If you identify CD or digital
sets, use `--playback both`; making it the default would need the full
evaluation with both ladders first.

### `listen` and `review` are untested with real audio output

This session has no audio device: ffplay reported "audio open failed". I
checked the ffplay commands with `listen --print` and ran the same filters
through FFmpeg into a null output, but I have not heard them. Please try
`gunfinger listen <report> --at <time>` once; if ffplay misbehaves, the
printed commands show exactly what runs.

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
