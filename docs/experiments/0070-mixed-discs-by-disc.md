# 0070: mixed discs by disc, after the owner's prune

**Question.** Item 8a of session 11: are there mixed discs among the
folders indexed since experiment 0065, scored disc by disc as the owner's
new patterns split releases (`<folder>/2*`)?

**What was indexed since.** Against session 10's record list (32,441),
the store at `f9f9bc706195a71b` holds 1,252 new records and 97 fewer
(the prune). All 1,252 are YouTube uploads in three channels
(`butch-french` 760, `cryptic-jungle` 442, `3zb-gangstep-dnb` 50),
written 2026-10-10 17:28-18:03. A channel folder is a flat dump in title
order, not a release in track order, so its boundaries say nothing about
mixing. No release folder was indexed since 0065.

**Method.** `scripts/analysis/folder_edges.py <store> 4 --by-disc` (new
option; 10:54 under `nice` beside the clusters search, 297 s CPU): as in
0065, a boundary is joined when the last second of a file and the first
second of the next both hold more than half the file's mean peak
density; a folder whose names all start with three digits is scored per
disc (first digit). Data: `data/0070-disc-edges.json`; list
`work/s11/mixed-disc-candidates.tsv`.

**Result.** 449 units (412 folders, 37 discs of 26 folders). Two score
0.75 or more: Shy FX's *Just An Example* and the Mampi Swift EP, both
kept by the owner after session 9 (0065: 33 folders, the rest now
ignored). No disc reaches 0.75. The unmixed discs beside the owner's new
patterns score 0.00-0.10 (Blazin `1*`, Inside The Machine `1*`, Molten
Beats `1*` 0.08, Essential Rewindz `3*`, Vintage Dread `1*`, Soul
Survivors `2*`, 21st Century Grooves `1*`), so the patterns took the
mixed discs only. In between, with at least 6 boundaries, for the ear:

| Score | Joined | Unit |
|--:|--:|---|
| 0.62 | 8/13 | `skynet_and_stakka-point_of_arrival-1999-sour/` |
| 0.60 | 6/10 | `va-listen_up-hlcd9-2000-sour/` |
| 0.55 | 6/11 | `jonny_l-magnetic-1998-sour/` |
| 0.50 | 5/10 | `2001 - VRSCD003 - Ed Rush & Optical - The Creeps [Virus]/CD1/` (kept by the owner) |
| 0.50 | 6/12 | `[RUFF 57] Various - Analog Steroids (CD)/` |
| 0.44 | 4/9 | `va-breakbeat_science-scincd001-1996-sour/`, disc `2*` |
| 0.40 | 4/10 | `bad_company-fear_lp/` |

0.10-0.39: 88 units; below 0.10: 340.

**Conclusions.** The owner's 39 patterns leave no folder or disc that
scores like a mixed CD. Seven units score in between; a gapless album
scores the same, so each needs the ear. Nothing was written to the
ignore file.
