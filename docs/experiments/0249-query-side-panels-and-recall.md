# 0249: power-sum queries at NAS scale: shared panels and recall

**Question.** Brief 62's runs 1-2, frozen rule: do power-sum queries
against today's records keep the shared panels' and `recall`'s confident
answers, with no new wrong file and no more related versions?

**Change.** Harness only (`fcf385d`): `shared` and `recall` take
`--front-end`; the verifier runs only under the downmix.

**Command.** `work/s62/panels.sh`, `panel2.sh` (both development
halves), `recall.sh` (`--queries clean,brief`, with and without
`--other-rips-only`; damaged sets not run), NAS scale, `--panels work/s33/panels`, downmix and
`power-sum-queries` by `s62-run` on the same renders, 03:41-05:13 beside
0250's runs; `query_side_bar.py`; widths by `stereo-width --files` on
recall's 702 renders. Data: `data/0249-query-side-panels-and-recall.json`.

| Confident right (downmix -> power sum) | Result | Lost; won |
|---|--:|--:|
| Panel 1, played file (everything, source left out) | 269 -> 269 | 0; 0 |
| Panel 2, the same | 208 -> 208 | 0; 0 |
| `recall` clean; brief plays (indexed) | 522 -> 513; 721 -> 663 | 9; 0 and 58; 0 |
| Other rips only: clean; brief | 174 -> 172; 171 -> 168 | 2; 0 and 6; 3 |

Wrong confident (query, file) pairs: none new anywhere; `recall` has
none under either; panel `other` pairs 422 and 3 under both. Related
versions named: panel 1 326 -> 309 (17 fewer, all on stereo queries:
`alone`, `beyond`, `before`), panel 2 9 -> 9.

**Width.** The 9 clean losses are the nine speeds of one source,
Organikismness - Echos (side to mid -7.3 dB; 854-1,105 hits to
176-226). Over recall's clean excerpts the variant's best right hits
against the downmix's, by side to mid: 0.98 at -20 dB or less (180),
0.94 to -15 (199), 0.77 to -10 (92), 0.49 to -6 (33), 0.39 above (18).
51 of the 58 brief plays lost lie above -15 dB.

**Conclusion.** The bar fails (item 2: `recall` loses 9, 58, 2 and 3
net; tolerances 5.2, 7.2, 1.7, 1.7). Panels hold; nothing new is wrong. By
inference, today's records hold the mid's peaks; peaks picked from mid
plus side add the side's where it is loud, so wide sources lose hits.
`robust`'s excerpts are mostly narrow (0196), which hid it in 0245.
