# 0231: the five ignored NAS files pruned, against session 33's baseline

**Question** (`docs/brief-52.md`, steps 2-4). The NAS `.gunfingerignore`
leaves out five files whose peak records the store still held: the three
tracks of `knowledge-the_lowdown_vol_7-track_fix-1999-sour/`, the Space
Cake `[CUT]` upload and the T-Minus vocal mix upload (status, 2026-10-25
and 2026-10-27). What does pruning them change at NAS scale?
**Method.** `gunfinger --config ~/.config/gunfinger/nas-dnb.toml prune
--yes` after `doctor` and `prune` listed exactly the five (5 peak records,
5 tag notes); `clusters --from-peaks --reuse-pairs`; `regress
session-33-linux-nas` with `work/bin/s52-start` (= `s51-final`; the saved
index built again); `baseline session-52-linux-nas`; `work/s52/diff.py`
compares every query and detection; `data/0231-prune-five-ignored.json`.

| | Session 33's baseline | After the prune |
|---|--:|--:|
| NAS store records (revision) | 40,683 (`56433d74d0919a08`) | 40,678 (`1fc6fe50293ad330`) |
| Sweep 2026: indexed assets | 40,580 | 40,575 |
| Sweep queries with identical detections | | 718 of 720 |
| Stakka scan; leave-outs 3, 11 | 11/11; 8/11; 0/11, 0 wrong | identical, every detection |
| Calibrate: weakest identifying, strongest false | 704, 282 | 704, 282 |
| Calibrate: false accepted; candidates at 30+ hits | 7; 95 | 7; 95 |
| Clusters; pair entries | 284; 2,877 | 284; 2,876 |

- The two sweep queries that change each lose one sub-threshold
  candidate of a pruned file, 7 hits in 2 windows, and nothing takes its
  place: Yvon Is On at 0% (the Space Cake upload) and The Reprise at +1%
  (the track-fix Kraken). No confident or possible detection changes.
- The clusters lose one pair entry, the corpus T-Minus against the vocal
  mix upload (356 hits, 58% coverage, judged different by ear); the
  clusters are unchanged. File `ce293fe6165e...` (was `ac53014b8a87...`).
- None of the five was in the baseline's scan reports, a cluster, the
  library map, a sweep or recall panel, or the corpus store (older
  NAS-scale reports outside the baseline do name some). The upload is in
  three committed panels (shared material, validation half; two
  `shared-mixes` plans, as an excluded file and a recording).
**Conclusion.** The prune changes nothing that is scored: every
difference is a weak candidate or a pair entry of a pruned file.
`session-52-linux-nas` replaces `session-33-linux-nas` as the NAS-scale
baseline; `session-32-linux` stays.
