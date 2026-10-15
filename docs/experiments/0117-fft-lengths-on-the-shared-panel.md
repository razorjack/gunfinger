# 0117: the FFT lengths' variant on the shared-material panel

**Question.** Session 21, step 6 (rule in `docs/brief-21.md`, written
first), the first stage of proposal 2's retrieval study: does session 19's
first-pass variant factor2 (each turntable window within 2 samples of
`round(1024 / s)` with the smallest largest prime factor, 0109) change
wrong or right confident titles on the panel's development half?

**Method.** `work/s19/py/fft_variant.py factor2` on a detached worktree
at `49f4c77` (never merged; `work/s21/fft-factor2.diff`; refinement and
key-locked rungs unchanged). `gunfinger-eval --other-peaks-dir <NAS
store> shared docs/panels/shared-material-seed-2026-development.json`
(store `b285da9984422fb5`, 27 min, 5.4 GB) on session 20's renders (the
same render key), against session 20's final report:
`shared_compare.py`; data `data/0117-fft-lengths-shared.json`.

| 605 query results (3 indexes) | Today -> variant |
|---|--:|
| Wrong confident titles added | 3 (1 query x 3 indexes) |
| Wrong confident titles removed | 3 (1 query x 3 indexes) |
| Right confident files lost / gained | 0 / 0 |
| Wrong confident queries, played left out | 106 -> 105 |
| Other outcome changes (possible only / nothing) | 3 |

- Added: Fire's Rogue Unit VIP in `fire-3-looped-60s` (Fire's original
  played against the Nasty Jungle Remix) goes from 239 hits (possible) to
  240 in 6 windows; the query was already wrong confident through the
  remix, so no query becomes wrong.
- Removed: Sonar's Mark System Revision in `sonar-1-weak-beyond-20s`
  falls from 240 hits in 16 windows to 125 in 7 (played left out: possible
  only; the other two indexes: right confident).
- Detections change slightly almost everywhere (44 of 605 results
  identical; confident hits 0.92-1.11 of today's, median 1.00); three
  break queries move between possible only and nothing.

**Conclusion.** Both changes sit on the rule's edge (239 and 240 hits);
the variant adds no wrong confident query and loses no right file on the
development half, the roadmap's criterion, but adds one wrong confident title
inside a query already wrong. Nothing adopted; the rest of the retrieval
study (rips, damage, short plays, the development mixes, timing) is not run.
