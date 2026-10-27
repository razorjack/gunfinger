# 0237: the drops mixes of sessions 39 and 48 under tonight's changes

**Question.** Session 57, group 2 (pre-registration of 2026-10-27): what do
the prune and ADR 0021-0024 change in the double drops, in the harness's
scores and the CLI's tracklists at both scales; does a related version's
second alignment more than 0.6% apart reach 240 hits?
**Method.** Build `s57-start`, `work/s57/wdrops` on reflink copies of the
renders (none rendered again). Harness at 262 tracks, the old flags: drops
2026, 2028, their other pairs (ladder `both`); 2032, 2034 (turntable,
`--verify`), their other pairs; aligned drops 2026-2034 (12 reports). CLI
(`--saved-index off`): 262 tracks, store copies (drops, aligned; 8 x 16
mixes); NAS `--store-only --exclude-from`, also other pairs (12 x 16).
Old: sessions 39, 45, 48. `work/s57/{mixes_score,diffs,residual_mixes,cli_compare}.py`.

| Old -> today | Harness, 262 | CLI 262 | CLI NAS |
|---|--:|--:|--:|
| Scores: plays by level, wrong confident (0; other pairs 4, 19, 19, 10), same-title 0 | identical, all 12 | | |
| Added, kept more than 0.6% apart: weak; possible | 65; 1 | 0 | 0; 5 |
| Pruned upload's segments removed: confident; possible | | | 41; 8 (other pairs 2026, 2034) |
| Segments re-measured (hits +-1, speed under 1e-6): confident; possible | 0 | 0 | 18; 2 (seeds 2032, 2034) |
| Entries changed apart from gap lines | | 0 | the upload's 38 -> 0 |
| Gap lines; over a missed track; range holds it | | 128; 128; 128 | 337; 337; 333 (4 none: count 0) |

- Residual risk: 56 of the 66 kept detections are of a recording not
  playing (mostly the Clockwork pair's versions), at most 44 hits. The
  strongest (109, possible) is the corpus T-Minus playing at its own speed
  (other pairs 2026, mix-10), which the old rule dropped under its 155-hit
  alignment to the vocal mix upload, 11% faster. None reaches 240.
- The prune: in other pairs 2026 and 2034 the pruned T-Minus vocal mix
  upload loses its 23 and 15 entries; gap lines and possible lines of
  Crash (the upload's first 56 s), the corpus T-Minus and Trace & Optical
  - Kridian, which the upload's plays had covered (ADR 0019), take their
  place. No other confident play changes.
- Re-measured segments: session 48's own binary on today's pruned store
  gives today's segments in seed 2032 (16 reports), apart from ADR 0021's
  added possible Funktion segment: the store's change moves them, not the
  code (mechanism not traced). ADR 0022: no tracklist names China Cup.
**Conclusion.** Every difference is explained: the prune (the upload's
plays; re-measured segments, levels unchanged), ADR 0021 (weak and possible
detections), ADR 0023-0024 (gap lines over missed tracks). No finding.
