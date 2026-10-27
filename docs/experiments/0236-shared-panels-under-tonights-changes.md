# 0236: the shared panels' development halves under tonight's changes

**Question.** Session 57, group 1 (pre-registration
`docs/research/2026-10-27-rerun-under-tonights-changes.md`): what do the
prune, ADR 0021 and ADR 0022 change on both shared panels' development
halves at NAS scale, and does a related version's second alignment more
than 0.6% apart reach 240 hits (ADR 0021's residual risk)?
**Method.** `shared` on `docs/panels/shared-material-seed-2026-development.json`
and `shared-material-2-...-development.json`, `--other-peaks-dir`,
`--panels work/s33/panels`, build `s57-start` (= master), in
`work/s57/wshared` on reflink copies of the old renders, both at once
(25:59, 19:07). Against the newest default reports (sessions 44, 45) and
the `--apart-in-speed 0.006` ones (38, 45): `variant_compare.py shared`,
`work/s57/diffs.py`, `residual.py` (`work/s57/out/`). Validation halves
not searched.

| Old default -> today | Panel 1 (622 queries) | Panel 2 (440) |
|---|--:|--:|
| Confident, roles played; related; other; partner | 516; 430; 475; 8, unchanged | 545; 12; 3; 0, unchanged |
| Queries gaining a confident `other`; losing a confident played or partner | 0; 0 | 0; 0 |
| Added, kept more than 0.6% apart (ADR 0021): weak; possible | 185; 0 | 81; 2 |
| Removed, overlapping within 0.6% a newly kept one | 0 | 2 weak |
| Removed, a pruned file's (weak) | 24 (Drumtrip, at most 6 hits) | 3 (the T-Minus vocal mix upload, 6) |
| Against the old `--apart-in-speed 0.006` report | only the 24 pruned | only the 3 pruned |

- The residual risk: panel 1 keeps 185 (183 `other`, 2 `related`; at
  most 24 hits), panel 2 keeps 83 (60 `other`, 14 `related`, 9 `played`;
  at most 61, the two possible ones of Phantom Force in its own break),
  0.60-14.2% in speed from the overlapping detection of their file. The
  strongest of a related version: 55 hits (Phantom Force's upload in
  `phantom-force-2`'s break, 2.2% from 123 hits). None reaches 240.
- Three of panel 1's additions (DJ Alibi - Free Spirit, 12 hits) overlap
  one of equal hits that the old rule ranked first; `diffs.py` counts a
  tie as overlapping (first read as "other", corrected).
- ADR 0022 does not apply: `shared` scores by cluster alone (neither
  half names a China Cup file; its 8 and 6 detections are unchanged).
**Conclusion.** Every difference is explained: each panel is its variant
run of session 38 or 45 less the pruned files' weak detections. No
confident detection changes in any query, and ADR 0021's residual risk
did not occur. No finding.
