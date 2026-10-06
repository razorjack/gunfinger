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
