# 0267: recurring unidentified passages across a batch of mixes, retrieval and wrong links

**Question.** Proposal 6's pilot (brief 69, pre-registered in `084d659`): when mixes' unidentified spans are indexed in memory and searched against each other, are a held-out recording's occurrences in other mixes linked with their times, at opposite deck settings too, while non-overlapping excerpts of one source, records sharing a passage and related versions stay apart and nothing propagates?
**Method.** `gunfinger-eval passages` (`work/bin/s69-run/`, `ebdb991`; `work/s69/queue.sh`): per batch 12 generated mixes (2.6-2.85 h), 16 subjects out of the index in 3 mixes each (speeds 1.04-1.08, 0.92-0.96, 0.94-1.06; the odd subjects' third occurrence shares no source time with the others), indexed partners between them, long crossfades with bass swaps, the panel's development pairs (Dozer, Fire, Sonar, Pyro, Watermelon) one side each in two mixes, and a 12 s insert in 6 mixes. Gaps mode indexes the spans of 30 s or more without a confident detection at 262 tracks; whole mode indexes the whole mixes. Links are confident detections under the frozen rule, joined only when two between the same mixes overlap in both. Registered seeds 6901-6902; 6903-6910 run beside them (decided before the runs). `scripts/analysis/passages_summary.py`; data `data/0267-recurring-passages.json`.

| Subjects' positives found (source overlap of 30 s or more) | Gaps ±8% | Gaps ±18% | Whole ±8% | Whole ±18% |
|---|--:|--:|--:|--:|
| Registered, seeds 6901-6902: all; above 8% relative speed | 10 / 60; 3 / 43 | **30 / 60; 23 / 43** | 16 / 60; 5 / 43 | 48 / 60; 37 / 43 |
| Ten batches: all | 77 / 312 | 192 / 312 | 100 / 312 | 266 / 312 |
| by relative speed 0-4 / 4-8 / 8-12 / 12-18% | 29/51, 28/42, 20/107, 0/112 | 29/51, 28/42, 73/107, 62/112 | 41/51, 36/42, 23/107, 0/112 | 41/51, 36/42, 91/107, 98/112 |
| by overlap 30-60 / 60-90 / 90 s or more | 9/97, 35/149, 33/66 | 25/97, 102/149, 65/66 | 23/97, 43/149, 34/66 | 60/97, 140/149, 66/66 |
| Found ones' spans: median start late, end early; overlap covered | 20.6, 21.9 s; 0.5 | 19.2, 18.7 s; 0.5 | 9.6, 9.6 s; 0.7 | 9.3, 9.0 s; 0.7 |
| Wrong, ten batches: different recordings outside the pairs ("other") | 0 | 0 | 0 | 0 |
| Shared-material pairs linked (of 50): Fire, Pyro, Dozer | 10, 9, 0 | 10, 10, 0 | 10, 8, 1 | 10, 9, 1 |
| their spans' furthest reach past the panel's passage | 0.2 s | 0.2 s | 12.3 s | 12.3 s |
| One recording, another passage: links; control pairs linked (of 160) | 10; 6 | 15; 9 | 31; 8 | 37; 11 |
| Partners recurring across mixes, found (of 72; indexed, so in no gap) | 0 | 0 | 59 | 63 |

- The registered bar fails on (1): in gaps mode at ±18%, 50% of the subjects' positives are found (80% registered) and 53% above 8% (70%); no batch of the ten reaches 80% (47-78%). (2) holds on both registered seeds (no "other" link, 0 of 32 control pairs linked; at most 2 per batch in all ten) and (3) holds (0.2 s past a passage at most).
- Gaps mode loses what the library search leaves: 77 of 580 held-out plays (13%) have no unidentified time at all (28 lie inside their neighbours' crossfades; 49 leave at most 21 s between them, under the 30 s minimum), and the median play keeps 58% of its time in a gap; found spans then cover half the overlap. Whole mode finds 85% at ±18%: 94% of the positives overlapping 60-90 s and all of those overlapping more (gaps mode: 68% and 98%).
- The product's ladder (±8%) misses all 112 positives beyond 12% and most at 8-12%: opposite deck settings need the wider range.
- Of the 52 "another passage" links at ±18%, 51 run at their two plays' relative speed (within 0.01) with the track positions 10-178 s apart: by inference the record's own repeated material; 9 and 11 of them join a control pair (excerpts with no source time in common). No link joined unrelated records. The 12 s insert never made a confident link: as built it is shorter than the rule's windows and than a positive (30 s), so it could not test whether an insert bridges two records.
- Fire's remix and Pyro's two tunes link in almost every batch, within their shared passage; Sonar and Watermelon never, Dozer once.

**Conclusion.** Pairwise links between mixes are precise as passages: no unrelated link, and shared-material links that say no more than the shared passage. But over unidentified spans only half of the recurring passages are found, because a held-out record's time in a gap is short; whole mixes recover 85%. The wider range is necessary for opposite deck settings. Not worth a design note by the registered bar; a next step would index whole mixes.
