# 0297: the families behind the decisions, and the split audit

**Question.** Session 87 (brief 87; no bar, nothing adopted): how many distinct recording or relation families do the experiments behind ADR 0019, 0021, 0022, the shared-material rules and 0255-0275 rest on, and does any family have files on both sides of a panel's split?

**Method.** `gunfinger-eval families` builds a key from stored relations only: the clusters in use (corpus, NAS), the library map, session 86's scratch library-wide clusters, the verdicts, `docs/same-titles.txt` and the two frozen shared panels' families (four relations are recorded only there: Clockwork, Fractles, Watermelon, The Nine). Tiers: recording; title (plus same-title judgments); relation (plus every verdict pair, panel family, and stored pair covering 20% with 30 hits or more; coverage alone chained 679 files through chance alignments of median 9 hits). With the scratch file: 11,946 files in 5,074 relation families of two or more (largest 23, China Cup with The Nine); without it 285. `family_audit.py` checks every panel, `family_reread.py` and `family_counts.py` the saved data (file names looked up by name). Nothing searched. `data/0297-families-and-split-audit.json`.

**Split audit (both keys).** No family at any tier has files on both sides of any split. Recall: 80 + 80 sources, 80 and 80 families at every tier, none shared, none twice within a half; the only stored pair across is a 6-hit chance alignment. Only 30 development and 37 validation sources are in any join, so for the rest the audit rests on the library-wide clustering finding no other rip (candidates from a 20 s excerpt, 0295). Shared panels 1 and 2: development against validation, and panel 1 against panel 2, nothing shared, not even a stored pair of any strength. Sweep panels (seeds 7, 2026-2029) and shared-mixes panels: no held-out recording has a rip left indexed; at the title tier seed 2028's three held-out China Cup files leave the judged `ƆƐI3C - China Cup.opus` indexed, which scoring credits as right (ADR 0022). Seeds 2027 and 2028 each hold a held-out draw related to an indexed draw (shared material, by design).

**Items against families** (relation tier; largest family's share):

| Exp | Decision | Items | Count | Families | Largest |
|---|---|---|--:|--:|--:|
| 0091 | ADR 0019 | 0076 passages, played file indexed | 92 | 5 | 59% |
| 0107 | ADR 0019 | possible plays left out: sweeps NAS; recall NAS | 73; 55 | 7; 1 | 37%; 100% |
| 0175 | ADR 0021 | lost sides of rides, seeds 3521; 3522 | 93; 99 | 76; 80 | 4%; 3% |
| 0232 | ADR 0021 | plays added, NAS 2003 scan | 13 | 12 | 15% |
| 0188 | ADR 0022 | false detections (by query) | 84 | 34 | 12% |
| 0230 | ADR 0022 | new wrong confident, seed 2027, L5 | 10 | 1 | 100% |
| 0116, 0118, 0134 E2 | shared | wrong related, played left out | 87 | 4 | 77% (Fire) |
| 0116; 0118 | shared | true drawn plays; through other rips | 438; 277 | 138; 112 | 5%; 6% |
| 0124 | shared | negatives; positives | 312; 75 | 4; 4 | 80%; 47% |
| 0134 | shared | E1 true plays, development mixes | 108 | 40 | 6% |
| 0215; 0216 | shared | shared plays per run (4 runs each) | 31-51; 34-59 | 1 each | 100% (Clockwork) |
| 0215; 0216 | shared | unrelated playing per run | 20-37; 15-41 | 16-20; 12-14 | 10-27% |
| 0255-0258 | 0255-0275 | oracle rows (by quiet record) | 113-115 | 70-75 | 15-17% |
| 0264, 0266; 0265 | 0255-0275 | NAS wrong answers; minimal recipes | 4; 12 | 2; 11 | 75%; 17% |
| 0274 (saved reports) | 0255-0275 | drops plays, seeds 2050; 2053 | 256; 256 | 102; 95 | (16 mixes each) |

- Not decisions on queries: 0262 (a census of the store), 0263 (5 flagged files, 5 families), 0269 and 0271 (2-3 wrong items, 1-2 families).
- No file names in saved data: 0176, 0177, 0180, 0211, 0233, 0259, 0260, 0267; no data file: 0261, 0268, 0270, 0272, 0273, 0275 (0274's reports are in `work/s73`). 0092 and 0100-0102 rest on the three development mixes.
- Family-weighted, 0124's rejections (Fire 144 of 248, Dozer 13 of 16, Pyro 0 of 32, Sonar 0 of 16) are 35%, not 50%; 0134's E2 (37 of 67, 7 of 9, 1 of 2, 0 of 9) is 46%, not 52%.
- Saved reports by family: the NAS sweep's held-out wrong answers (7 of 180, the Cool Rock pair) are one family of 20 (95% 0-11.7% by family); recall other rips, brief 48.9% (39.4-57.2%) over 20 families; 0274's drops recall 85.2% and 84.8% by play, 80.0% and 79.2% by family (tracks drawn more often are found more often).

**Conclusion.** The splits hold: no recording or relation crosses any panel's split, so `recall`'s tag key has not let a recording into both halves. The decisions do not: ADR 0019's containment evidence rests on 5 relation families, the version verifier's and 0116-0118's negatives on 4 (Fire 77-80%), and the marked-play rules' shared class on 1 (Clockwork) in every run. ADR 0021's, ADR 0022's 0188 and the residual work rest on tens of families.
