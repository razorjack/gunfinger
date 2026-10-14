# 0107: possible plays inside a confident play leave the tracklist

**Question.** Our decision of 2026-10-14 (ADR 0019): the README's
example listed "Clockwork (Stakka Rmx) (possible; shares material with
Clockwork)" in a mix that plays the original, before the remix existed.
Which lines leave the default tracklist, and is a right play lost, if
"inside" is tested on the confident play's passages (segments joined
across gaps of at most 10 s, 0091) instead of its whole span? **Method.**
`python3 -I scripts/analysis/possible_marks.py OUT.json` on every stored
development report, counted per file. Sweeps and recall: 30 s excerpts
without detection times, spans taken as equal. A play of the track
playing is not marked; the reports store no alignment or tags, so a
confident play of the same duplicate cluster stands in. A right play left
out is lost when nothing else lists its track. Data `data/0107-*.json`.

| Reports | Possible plays | Marked: span / passage | Right among passage-marked | Lost |
|---|--:|--:|--:|--:|
| Development scans and leave-outs, 262 tracks | 3 | 2 / 2 | 0 | 0 |
| The same, NAS scale | 31 | 21 / 21 | 0 | 0 |
| Generated mixes, seeds 2026 and 7 (262) | 16 | 6 / 0 | 0 | 0 |
| Sweeps 2026-2029, 262 tracks / NAS | 37 / 291 | 13 / 73 both | 0 | 0 |
| Recall, NAS | 77 | 55 both | 0 | 0 |

- Development plays marked: Star Trails (Synergy Remix), Clockwork (Stakka
  Rmx), Phantom 2018, Meltdown, Fracture's edit of Phantom Force, Mampi
  Swift - 2000, the Klute Remix (two files), `1.mp3`; none right.
- The span test's 6 generated-mix marks are right brief plays (Kosmic,
  The Pulse on two files, The Nine on three) between two parts of a
  returning track; the passage test does not mark them.
- Without the same-track exception, sweeps and recall marked 37 / 260 and
  64, of which 24 / 187 and 9 right, each inside its own recording.

**Change.** `mark_shared_material` tests passages and skips a play that
lines up with, or is tagged like, a confident play over most of it; the
tracklist leaves out marked entries. **Result.** `show` of
`work/s18b/{stakka,2003,vertical}-nas.json`, `work/bin/s18-final/` against
the new build: 7 lines go (2, 3, 2); the two Klute Remix plays line up
with the VIP within 1 s and join its entry under `titles disagree:`;
other lines change only their numbers. `identify` of the README's mix at
262 tracks: 11 entries, Clockwork's remix gone. **Conclusion.** Kept.
