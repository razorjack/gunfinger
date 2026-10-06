# Notes for the owner

Findings from the autonomous session that are worth your attention, newest
last. A morning summary goes at the top when the session ends.

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

### Key lock is solved, at twice the search time; the default is unchanged

Experiment 0010: rungs that stretch time but not frequency find every
key-locked excerpt at ±2%, ±5% and +8% (40 of 40 each, hits as strong as
untouched audio), and adding them to the turntable ladder changed nothing
for vinyl-style audio and created no false identification. The cost is
search time: 82 rungs instead of 41. I left the default on the turntable
ladder because your sets are vinyl, and added `identify --playback both`
(or `key-lock`); key-locked plays are marked `(key lock)`, and `listen`
time-stretches them instead of resampling. If you identify CD or digital
sets, use `--playback both`; making it the default would need the full
evaluation with both ladders first.
