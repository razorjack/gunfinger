# 0095: one tracklist entry per track: files tagged alike over one passage

**Question.** Session 16, step 3: the 2003 mix at NAS scale lists DJ Hidden
- Skektics twice (42:13 and 42:15). Why, and what does grouping by tags
change in the saved reports of the development mixes?
**Cause** (`work/merge-check/2003-s15.json`). The alien5ive upload (1,984
hits, +0.98%, from 1:01.5 of its file) and two vinyl rips (6,591 and 4,399
hits, +4.32%, from 0:21.8) overlap for 99.9% of the shorter play, but the
track would have started at 41:13.1 in the mix for the upload and at
41:54.8 for the rips: 41.7 s apart (about 40 s more lead-in), past
`SAME_RECORDING_SECONDS` (5 s). All three files' artist and title tags
normalise to `dj hidden` / `skektics`.
**Change** (output only; ADR 0017, "One entry per track"). Two plays over
the same passage (overlap of at least half the shorter play) are one
entry when they are one recording, as before, or when their own files'
artist and title tags compare equal by letters and digits, ignoring case
(`same_tagged_track`); chains join, whatever the play order. Plays
without both tags, and possible plays marked as sharing material, never
join by tags; the core and the JSON report are untouched. **Command.**
`scripts/analysis/tracklist_changes.py` with the builds `s16-start` and
`s16-step3`, on the 30 saved `identify` reports of the two mixes (11
distinct) and 7 fresh ones from the start build (both sizes, playbacks).

| Reports (library) | Entries | Cue tracks | What changed |
|---|--:|--:|---|
| 2003 mix, sessions 11-16, 21 files, 5 distinct (NAS) | 16→15 or 15→14 | 13→12 | Skektics: one entry, `42:13 Dj Hidden - Skektics` |
| 2003 mix, session 9, before the prune (NAS) | 21→18 | 16→13 | Skektics; a Future Cut - The Specialist extract of DJ Marky's Audio Architecture CD (2:52) joins `2:10 Future Cut - Sex Drive (titles disagree: Future Cut - The Specialist)`; a Phantom Force extract of Contagious Drum & Bass Vol 1 (11:21) joins `10:48 Digital and Spirit - Phantom Force` |
| stakka mix, 3 reports (NAS) | 13→13 | 11 | none |
| stakka mix, 10 reports; 2003 mix, 2 (262 tracks) | 12, 1, 7 | same | none: the corpus store has no tag notes, so these reports carry no tags |

**What it shows.** In reports of the current library (the pruned
NAS store) the rule changes one entry, the one a reviewer saw.
The two extra joins in session 9's report are mixed-CD extracts, now left
out by the NAS `.gunfingerignore`, tagged with the title of the track they
hold. No possible play joined, and nothing joined by tags alone that
carried another title. **Does not show** how often files tagged alike are
different recordings (edits, VIPs tagged without "VIP"); by our
rule they are one entry. Names differing only in case are listed once
(the upload's "DJ HIDDEN - skektics" is not repeated). Data
`data/0095-tracklist-entries-by-tags.json`.
