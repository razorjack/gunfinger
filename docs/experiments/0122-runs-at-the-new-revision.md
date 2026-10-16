# 0122: the development mixes and the panel at the new revision

**Question.** Session 23, step 4: rerun at `d71ba5333cff9a34` (39,354
records) under 0120's clusters, what changes against 0121 (the old
revision's reports under the same clusters)? The difference is the
library's; 0121's against the published numbers is the clusters'.

**Method.** `work/bin/s23-harness` (`gunfinger` as at the start), one
run at a time under `screen`; logs `work/s23/runs/`, `data/0122-*`.
Tonight's runs took 3-6 times as long as before (`shared` 112 min against
32; cause not found), so 4c shrank (brief, 09:25): the 36 rendered mixes
and the rips plan were not rerun; the three development mixes ran alone.

| Run (new clusters): old -> new revision | Right | Wrong confident |
|---|--:|--:|
| 3 development mixes, `--verify` scans: tracks | 40 = | 0 = |
| Every track left out (11, 12, 17): tracks | 7 -> 0 | 0 = |
| Possible plays matching no track | 10 -> 9 | |
| `shared`, development half: queries (218) | outcomes = | 106 = |

- Scans: four tracks are also credited to files stored tonight (Decoy,
  Dominion, Descent's Phuncatek Remix, Horns 2000); `1.mp3`'s possible
  play over Cannibal is gone (pruned). The leave-out runs, now with the
  new clusters, leave the newer uploads out: 0 of 40, no wrong answer.
  Under the old clusters the new scans would have 13 wrong (0121's 9 and
  the 4 files stored tonight).
- `shared`: every outcome as in 0121; a few hit counts move by 1; the
  copy of the blend partner Decoy stored tonight is a possible
  detection in 24 blend queries (6.1 GB, 112 min).
- 4c, the development mixes alone (`work/s23/shared-mixes-dev.json`,
  40 min, 5.5 GB): 108 true plays credited by the new scans (0121: 104),
  the 4 added through files stored tonight; none lost. Every confident
  detection keeps all its holders, so (c) is known for each play.

**Conclusion.** The library changed little: no outcome of the scans or
the panel moved, files stored tonight add 4 true plays, and `1.mp3`'s
pruning removes a possible play. 0121's changes came from the clusters.
