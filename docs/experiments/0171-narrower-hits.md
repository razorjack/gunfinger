# 0171: grouped hits as 4 bytes, raw hits as 6

**Question.** Session 36, step 3 (the research note's item 4): with 0169 in
master, do narrower hits make the NAS-scale search faster by the note's
stop point (3% at 16 workers, or removing a 32-worker penalty), exactly?

**Variants** (deleted; sources `work/s36/lines.rs.narrow4` and `.raw6`).
- *Grouped as 4 bytes* (`s36-narrow4`): once grouped, a hit keeps the
  posting's frame (`u16`) and its anchor's position after the window's
  first anchor (`u16`); its asset is known from its place. A window whose
  anchors span more than a `u16` keeps the 8-byte form (a per-window
  branch). Grouping and the per-asset loop became generic over the two
  forms, with closures giving each hit's frame and anchor.
- *Raw as 6 bytes* (`s36-raw6`, on top of it): a part is taken as two
  arrays, the postings copied whole from each list (4 bytes) and the
  relative anchors (2 bytes).

**Method.** The Stakka scan at NAS scale, `s36-step1` (0169's build) against
both, one round per job count, each variant twice between two base runs,
otherwise idle (`work/s36/ab-hits.sh`, `work/s36/ab-hits-clean.tsv`).

| Build | Search, 16 jobs | User CPU | Search, 32 jobs | User CPU |
|---|--:|--:|--:|--:|
| 0169 (8-byte hits) | 45.2, 45.3 s | 673-674 s | 38.1, 38.0 s | 1,099-1,101 s |
| grouped as 4 bytes | 47.3, 47.3 s | 705 s | 38.8, 38.4 s | 1,112-1,114 s |
| raw as 6 bytes too | 48.8, 49.1 s | 729-734 s | 39.2, 39.4 s | 1,135-1,136 s |

Every report identical to session 33's apart from `wall_seconds`; peak
memory unchanged (4.8-4.9 GiB). The last base run at 32 jobs failed to start
(the script's run label grew past the file name limit) and was run again
alone at once.

**Conclusion.** Both lose: 4.4% and 8.2% more search time at 16 jobs,
1.5% and 3.5% at 32, with more CPU; no second round, as the ranges do not overlap.
By inference, with parts of 1,024 assets the hit buffers already stay in
cache, so halving their bytes saves no memory traffic, while the narrow
form adds work per hit (rebuilding the anchor's position, two arrays).
Not kept; the 8-byte `Hit` stays.
