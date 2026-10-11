# 0087: the harness on a saved index at NAS scale

**Question.** Item 4 of session 12 (ADR 0014): do NAS-scale harness runs
that load a saved index, or remove a leave-out's files from it, report
what runs that build their index report, and how much faster are they?

**Change.** `Padding::index` loads, or builds and saves, the index of the
corpus records and the whole other store
(`work/indexes/library-nas-dnb-peaks.index`), then removes the files the
run leaves out in place (`Index::without_assets`). Tests: against builds
with and without files left out; a damaged digest, a truncated file, a
new store record and a changed corpus record each rebuild the file.

**Method.** `work/s12/item4.sh` (idle machine, store `f9f9bc706195a71b`,
`work/bin/s12-harness/`): `regress`, then `scan stakka-skynet-knowledge`
with `--saved-index rebuild`, `use` and `off`, and `--leave-out 3 --seed
2026` with `use` (filtered) and `off`. Session 11's reports were copied
to `work/s12/before/` first and put back after.

| Run | Index | Wall | CPU | Peak memory |
|---|---|--:|--:|--:|
| scan, rebuild | built 80.4 s, saved 3.0 s | 192.4 s | 1,044 s | 5.00 GB |
| scan, saved | loaded 1.9 s | 112.0 s | 960 s | 4.99 GB |
| scan, off | built in memory | 189.0 s | 1,037 s | 4.98 GB |
| leave-out 3, saved | loaded 2.0 s, 3 clusters removed | 107.2 s | 958 s | 5.00 GB |
| leave-out 3, off | built without them | 179.6 s | 1,027 s | 4.98 GB |

Search 102.4-104.0 s in every run. The file: 33,618 assets, 3.80 GB.

**Result.** The three development scans are identical apart from
`wall_seconds` (11/11, 0 wrong, 3,681 detections, 38 plays), and so are
the two leave-outs (8/11, 3,670 detections); both equal session 11's
reports (`1afd8cc`). Later, `sweep --seed 2026` on the saved index (48
held-out clusters removed; `0a625f3`): byte-identical to session 11's
report, 641 s. `regress`: identical. Data `data/0087-*.json`.

**Conclusion.** Each NAS-scale run skips the 77-82 s build: the
development scan takes 112 s instead of 189 s, a leave-out 107 s instead
of 180 s, with identical reports and peak memory. Cost: a second 3.8 GB
file on disk beside the CLI's.
