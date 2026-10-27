# 0238: recall and the other development scans under tonight's changes

**Question.** Session 57, group 3 (pre-registration of 2026-10-27): what do
the prune and ADR 0021-0022 change in `recall` (8 conditions, session 33) and
the 2003 mix's and Nocny's scans at NAS scale (sessions 33-34)?
**Method.** Build `s57-start`, `--other-peaks-dir`, `--panels work/s33/panels`,
`work/s57/wrecall`, `wscans`; old reports in `work/reports/library-nas-dnb-peaks/`
(and `variant-apart-0.006/`, session 38); `work/s57/{counts,diffs}.py`. Traces:
session 33's binary on today's store, clean and brief; `--overlap-by-spans`, brief.

| Recall (old -> today) | Recalled | Wrong | Pruned Drumtrip's confident |
|---|--:|--:|--:|
| Clean; damaged | 522/522; 4/522 | 0 | 8 removed; 0 |
| Brief; brief damaged | 721/1044; 0/1044 | 0 | 8; 0 |
| Other rips only: clean; damaged | 175 -> 174 of 180; 0 | 0 | 8; 0 |
| Other rips only: brief; brief damaged | 176 -> 171 of 360; 0 | 0 | 8; 0 |

- Recall lost 1 and 5 excerpts, all Known Unknown - Rollers Edit (other rips
  only; +8%; brief -5, -1, 0, +1, +8%): their one confident right detection
  was the pruned track-fix Drumtrip copy (434 hits at +6.8% in the first);
  no other rip reaches the rule. The prune's.
- ADR 0021: 6 weak added over the 8. No same-title credit; held-out: 0.
- The prune also moves detections of files never pruned: weak 6-hit
  candidates (2 in clean, 1 in other rips), a refit, and in brief query
  911 (Organikismness - Echos at +1%) a confident right one, 455 hits at
  +1.147% -> 503 at +0.933%. Session 33's binary on today's store gives
  those of clean and brief, `--overlap-by-spans` today's brief report; the
  other rips' weak one was not traced (inferred alike).

| Scan, NAS scale | Identified (of tracks) | Wrong | Differences |
|---|--:|--:|---|
| 2003 mix; leave-out 3; 12 | 12/12; 9/12; 0/12 | 0 | Into Music's middle segment (1,392 hits, confident) and 11-12 weak added |
| Nocny; leave-out 3 (2026-2028); 17 | 17/17; 14/17 each; 0/17 | 0 | 5 weak added; 1 weak pruned (Propaganda, 7 hits) |
| 262 tracks: 2003 mix; Nocny | 7/7; 9/9 | 0 | none; 1 weak added |

Against session 38's `--apart-in-speed 0.006` reports the NAS scans differ only
by Propaganda; Into Music's segment is ADR 0021's intended change (0232).
**Conclusion.** Explained: recall lost 6 excerpts whose only identifying rip
was pruned; the store's change moved one confident right detection (455 ->
503 hits). Scans: no identification or level changes. No finding.
