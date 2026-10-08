# 0050: the grouped tracklist from the NAS store alone

**Question.** Item 7 of session 6: plays of different records over the
same stretch of the mix are now one tracklist entry at any speed (the
mix time at which each record's track would have started, its own speed
taken out, within 5 s), with the other names as `(also: ...)`. Does the
development mix against the NAS store give 13 lines (11 tracks, 2
possible plays), and what does the new default matcher cost there?

**Command.** `gunfinger --config /dev/null --peaks-dir <NAS store>
--min-track 1:30 --max-track 15:00 identify <development mix>
--store-only --playback both -f json -v`, then `show -f tracklist`; one
timed run on the frozen store (revision `d428ee9585936326`, recorded in
the report). Data: `data/0050-grouped-tracklist.json`.

| Store-only `identify` | Fitted (default) | Single pass, 0044 |
|---|--:|--:|
| Records; postings; index build | 27,042; 759.4 M; 56.5 s | 26,890; 755.2 M; 43.6 s |
| Wall; CPU; peak footprint | 263 s; 1,943 s; 3.98 GB | 856 s; 6,109 s; 8.35 GB |
| Plays: confident; possible | 28; 2 | 28; 2 |
| Tracklist lines | 13 | 20 (by name) |

The 13 lines are the 11 tracks and the 2 known possible plays (Star
Trails' Synergy remix at 10:28, the Clockwork remix at 20:22), each
marked with the entry it shares material with. Every track's records
form one entry: 2-4 plays at speeds up to 3.4% apart (Night Lore 0.9927
to 1.0268, Kontempt 0.9834 to 1.0293), named after the strongest play,
the other spellings after it ("Stakka And Skynet - Nightlore (also:
Stakka And Skynet - Night Lore, STAKKA and SKYNET - knight lore, Stakka &
Skynet - Nightlore)"). The item 1 binary's `show` gives 20 lines for
the same report.

**Conclusions.** The count is as expected, with no merged neighbours:
tracks that overlap in a blend (Decoy and Star Trails, 45 s; Kontempt
and Bios-Fear, 54 s) stay apart. The default matcher identifies the
mix from the store alone in 4.4 minutes and 4 GB, as the harness
predicted in session 5 (0042: 259 s, 4.01 GB).
