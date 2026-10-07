# 0037: clusters of the corpus recordings at NAS scale

**Question.** Item 3 of session 5 (NAS plan step 4): do the 17 corpus
clusters reappear among the NAS records, which further rips of the
corpus recordings does the NAS hold, and does the 80% rule still
separate rips from other recordings?

**Command.** `gunfinger-eval --other-peaks-dir <NAS store> clusters
--from-peaks`: the 26,890 NAS records indexed alone; each corpus record's
stored peaks searched at 0.98-1.02, then each new same-recording NAS
file, until none is new. Rounds of 262, 164 and 6 queries; 4,328 s wall,
38,056 s CPU, 6.7 GB peak. Data: `data/0037-nas-clusters.json`.

**Result.** The 17 corpus clusters reappear exactly (none only in one
library). 170 NAS files are further rips of corpus recordings; with the
copies, 233 clusters have duplicates (686 files, 433 of them NAS files).

| Coverage of the shorter file | 262 tracks (0002) | NAS scale |
|---|--:|--:|
| Weakest same-recording pair | 0.984 | 0.825 |
| Strongest pair of different recordings | 0.39 | 0.737 |
| Pairs from 40% to 80% (borderline, for the owner) | 0 | 76 |

Clusters whose members' tags disagree, found by content (for the owner):
the INFRA011 rips tagged Sex Drive and Specialist hold The Specialist and
Razor's Edge; an alien5ive upload tagged "Mindscape - New Deal" holds
Noisia & Phace - Outsource (Misanthrop Remix); an i-witness upload tagged
"Future Cut - Horns 2000 (Dylan Remix)" holds DJ Ink - Ice Age (Digital
& Spirit remix); the untagged `b-unknown-udfr014` matches an upload
tagged "Profound Noize - Luminous Remix". Two clusters hold tracks of a
mixed CD (Underfire UDFRCD003, Dangerous Drums Volume 2): its Night Lore
(5:11) and Global Report (2:09) probably carry blends with neighbours.

**Conclusions.** (1) The clusters from stored peaks scale: no corpus
cluster moved. (2) The gap that made 80% safe has narrowed from
0.39-0.98 to 0.74-0.83. Many of the 76 borderline pairs carry the same
artist and title (alien5ive vinyl uploads, other rips): probably rips
the rule keeps apart, whose detections then count as wrong. (3) A
mislabelled or longer upload can join a cluster; none joins two corpus
recordings.
