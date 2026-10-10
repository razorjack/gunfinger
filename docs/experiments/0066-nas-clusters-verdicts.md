# 0066: the NAS clusters with the owner's verdicts, at 32,441 records

**Question.** Item 3 of session 10: what the verdicts, the new folders'
rips and the pruned mixed CDs change against session 6's clusters (0049).

**Command.** `gunfinger-eval --other-peaks-dir <NAS store> clusters
--from-peaks --manifest razorjack-2003-03-29 --manifest
stakka-skynet-knowledge --recall-panel 2026 --cut-sparsest` (d77df95,
store `5701f221f7b48ee1`). Without `--cut-sparsest` the run stopped after
8,138 s: Sonar's PRO012 rip joins the corpus Sonar (100%, 7,258 hits) and
the revision judged different (99%, 467 hits, 1.3 hits/s), so that join
is cut. `clusters_change.py`; data `data/0066-nas-clusters-verdicts.json`.

| NAS clusters | Session 6 (0049) | Now |
|---|--:|--:|
| Records; queries (rounds) | 27,042; 478 | 32,441; 639 (347, 290, 2) |
| Clusters; files in them (NAS files) | 233; 732 (479) | 264; 867 (613) |
| Weakest same; strongest different (unjudged) | 0.840; 0.766 | 0.803; 0.794 |
| Borderline pairs (40-80%); without a verdict | 28; 28 | 61; 33 (19 groups) |
| Sparse joins (under 10% of the median) | 3 | 0; 1 join cut |
| Wall; CPU; peak | 5,106 s; 46,736 s; 4.59 GB | 8,063 s; 75,059 s; 5.41 GB |

50 corpus clusters change. Five gain the file judged the same (uploads
of China Cup, The Specialist, Coma, Alien Girl, fractles); Phoenix was
joined already; Synthesis VIP's verdict adds nothing (its Dangerous Drums
2 file is ignored and pruned); Sonar gains the PRO012 rip and loses the
revision. 46 gain 60 files indexed since session 6, among them mixed-disc
tracks covering 80% of themselves with one track: four of Inside The
Machine's disc 2, Essential Rewindz's Warhead, Blazin's "Skynet & Stakka
- Mix" (with Analogue Spikes). Five lose a pruned Dangerous Drums 2
track. 31 clusters hold no corpus file. Most of the 40 new borderline
pairs come from Essential Rewindz's mixed discs, not in the ignore file
(Stealth against four unmixed copies and four of Loxy's Caution; The
Nine; Funktion), then the Prizna "Fire" versions and two Dykast uploads.
27 of the 80 panel sources join another file, none a corpus recording;
one indexed source is a mixed-disc track (Essential Rewindz's Stealth).

**Conclusions.** The verdicts take effect; the cut, the only guess, goes
to the owner first. New scene folders add rips to a fifth of the corpus
clusters; unlisted mixed discs add joins and borderline pairs.
