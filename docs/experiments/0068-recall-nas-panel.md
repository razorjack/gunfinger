# 0068: recall beyond the corpus recordings, the NAS panel's development half

**Question.** Item 5 of session 10 (roadmap, "Recall beyond the corpus
recordings"): at NAS scale, how often does an excerpt of a NAS record
outside the corpus recordings return its own recording, per speed and
kind of source; what wrong and held-out answers appear; why misses fail.

**Command.** `gunfinger-eval --other-peaks-dir <NAS store> recall --seed
2026` (d77df95, store `5701f221f7b48ee1`, 0066's clusters): 1,603 s,
14,736 s CPU, 6.5 GB. Panel `docs/panels/recall-seed-2026.json`, drawn
and rendered before any search: 60 indexed and 20 held-out sources (20
indexed per kind), 9 speeds, the sweep's lengths and encoding; 32,432
assets indexed, 20 clusters held out. Data: `data/0068-recall-nas-panel.json`.

| Kind of source | Recalled | Wrong answers | Held out: confident |
|---|--:|--:|--:|
| Scene releases (`__full_scene/`) | 180/180 | 21 | 0 of 63 |
| YouTube channels | 180/180 | 9 | 0 of 63 |
| Elsewhere (label packs, vinyl rips) | 180/180 | 0 | 0 of 54 |
| All, each speed from -8% to +8% | 60/60 | 1-4 per speed | |

No miss, so no miss reason to report (never a candidate, too few hits,
too few windows). The weakest identifying detection is 710 hits (an
album track filed elsewhere, at -8%). The 30 wrong answers are two pairs, both
borderline in 0066 and both in the listening pack: one indexed source is
a track of Essential Rewindz's mixed disc 1 (Future Cut - Stealth), which
finds three unmixed copies of Stealth at 7 speeds (246-365 hits); and one
DYkast "Punishment" upload finds another channel's upload at all 9
speeds (up to 472 hits; 44% coverage). The strongest false candidate on
held-out audio is 101 hits (a "Tsunami (Remix)" against the original),
possible, not confident.

**Conclusions.** Recall on records outside the corpus recordings matches
the corpus sweeps: 540/540 at every speed and kind. Its wrong answers
are copies the clusters keep apart, a mixed-disc track and an upload
pair, not chance; the owner's verdicts on pack items 3, 5, 6 and 7, or an
ignore pattern for the mixed disc, decide them. The validation half
stays unsearched.
