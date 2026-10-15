# 0111: the shared-material scenarios at NAS scale, development half

**Question.** Session 20, step 3: with today's matcher, how often is a
related version a confident wrong title over the panel's passages (0110)?

**Change.** `gunfinger-eval shared` takes `--other-peaks-dir` (corpus
and NAS, 38,127 files, the saved index with leave-outs), renders NAS
files, takes each recording's files from the plan, and adds `before` (5,
10, 20 s of own material, then up to 30 s of the passage), `weak` (30 s
alone, 20 s beyond, through a dull 64 kbps transfer), `blend` (20 s
beyond, a partner at equal level over the context) and `break` (5 s
looped to 60 and 240 s); 10 new tests. One command: `gunfinger-eval
--other-peaks-dir ~/.local/share/gunfinger/nas-dnb-peaks shared
docs/panels/shared-material-seed-2026-development.json` (32 min, 5.4
GB peak); `shared_outcomes.py`; data `data/0111-shared-scenarios.json`.

| Played left out (main case), 14 directions | alone | looped | beyond | before | weak | blend | break |
|---|--:|--:|--:|--:|--:|--:|--:|
| Queries | 42 | 42 | 42 | 22 | 28 | 14 | 28 |
| Wrong confident | 4 | 34 | 33 | 8 | 15 | 12 | 0 |
| Possible only / nothing | 20 / 18 | 2 / 6 | 3 / 6 | 8 / 6 | 9 / 4 | 0 / 2 | 2 / 26 |

- Wrong confident in 106 of 218 queries and 12 of 14 directions (not
  Watermelon ~ Monotone, at most 46 hits): the related recording in 96,
  another version in 58, no other file. Weakest: Sonar's revision, 240
  hits in 16 windows; strongest: Fire's Rogue Unit Remix and VIP inside
  each other, 4,473-4,497 (a pair no one has judged).
- It takes enough of the passage: at 30 s or less only dense copies
  (Fire's remix ~ VIP, both Pyro directions, 244-1,095 hits); Dozer ~ its
  bootleg from 48 s, Fire's remixes from 60 s. Own material after the
  passage adds R at most +41 (Dozer) and +51 (Sonar's revision) at 30 s.
  A 5 s break looped to 240 s is never confident (R at most 32 hits).
- With everything indexed, each of the 106 wrong titles stands beside
  the played file's confident play. Positive control (7 directions with
  another rip): that rip confident in 65 of 106 queries, missing in the
  short ones. The control pair (no shared passage): R at most 25 hits.

**Conclusions.** On judged pairs, about a minute of shared passage (30 s
if copied densely) makes the other version a confident wrong title at NAS
scale, with or without the played record indexed; 0112 asks why.
