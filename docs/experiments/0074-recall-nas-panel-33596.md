# 0074: the NAS recall panel's development half at 33,596 records

**Question.** Item 6 of session 11: 0068's measurement after the
prune and 0071's clusters. How often does an excerpt of a NAS record
outside the corpus recordings return its own recording, and what wrong
and held-out answers appear?

**Command.** `gunfinger-eval --other-peaks-dir <NAS store> recall --seed
2026` (1afd8cc, store `f9f9bc706195a71b`): 1,700 s, 14,960 s CPU, 6.76
GB. The same panel (`docs/panels/recall-seed-2026.json`), not drawn
again; two development sources the patterns leave out and the
prune removed are named and left out (Essential Rewindz's Stealth, Soul
Survivors' Jupiter Fields), so 58 indexed and 20 held-out sources are
searched, 702 excerpts; 33,587 assets indexed. Data:
`data/0074-recall-nas-panel-33596.json`.

| Kind of source | 0068: recalled; wrong; held out confident | Now |
|---|--:|--:|
| Scene releases | 180/180; 21; 0 of 63 | 162/162; 0; 0 of 63 |
| YouTube channels | 180/180; 9; 0 of 63 | 180/180; 9; 0 of 63 |
| Elsewhere | 180/180; 0; 0 of 54 | 180/180; 0; 0 of 54 |
| Each speed from -8% to +8% | 60/60, 1-4 wrong | 58/58, 1 wrong |

**Changes.** The 21 wrong answers of 0068's scene releases came from
Essential Rewindz's Stealth, a mixed-disc track that is now pruned; its
source is left out, so they are gone and 18 excerpts fewer are searched.
The 9 left are the two DYkast "Punishment" uploads (385-472 hits, as in
0068), item 4 of the rebuilt listening pack. No miss. The weakest
identifying detection is still 710 hits; the strongest on held-out
audio is still 101 hits (Tsunami's remix against the original).

**Conclusions.** Recall on records outside the corpus recordings holds at
33,587 assets: 522/522. The only wrong answers are one upload pair to
judge by ear. The validation half stays unsearched.
