# 0280: render receipts with a recipe, and what they refused

**Question** (brief 78; code `696dd06`). A cached render was reused when its size matched its
completion receipt (the external review of 2026-11-05, item 3). The receipt now holds
the render's recipe: each source's path, size and modification time, the caller's steps
(excerpt, playback, filters, partner, fault, seed, encoding) and the renderer
(`render::RENDERER`, FFmpeg's version and its libraries' lines). Which renders does it refuse, and does
rendering them again move any figure? **Change.** Harness only: `render::once` behind
every render cache; FFmpeg runs under the core's stall timeout (`watch::run_reporting`
for encodes to a file). **Command.** `work/s78/regress.sh` with `s78-run`, then
`s78-final`: `regress session-54-linux` and `regress session-75-linux-nas`; the 2003
and Nocny scans now run inside `regress` (`work/s78/logs/`).

| Run | Renders refused | Result | Wall |
|---|--:|---|--:|
| Corpus regress, `s78-run` | 720 (the sweep, seed 2026) | the 4 held reports identical; 2 new incomplete | 1:23 |
| Corpus regress, `s78-final` | 0 | the same | 1:05 |
| NAS regress, `s78-final` | 0 (the same sweep renders) | the same | 4:49 |

- Why refused: all 720 receipts held a size alone, the older form. The renders date
  from 2026-10-18 19:03, before FFmpeg 8.1.3 was installed (23:00). One of them
  rendered again by hand differs in one byte of the ID3 tag, the muxer's version
  (`Lavf62.12.102` against `.103`), and decodes to identical samples; the regress sweeps show 720 of 720 queries with
  identical detections at both scales. Rendering the 720 again took about 18 s
  (1:23 against 1:05, by difference).
- Not yet refused: 40,514 more old receipts under `work/` (earlier sessions' scratch
  directories, `work/sweep/seed-2027` and `seed-2028`, `work/recall/`, `work/robust/`,
  `work/shared/`), each refused once on its next use. No search ran on them here.
- The new scans (262 tracks; NAS scale): 2003 mix 7/7 and 12/12, Nocny 9/9 and 17/17,
  0 wrong, as 0238 and 0232 recorded. Against the NAS reports of 2026-10-21, the only
  confident difference is the Into Music segment ADR 0021 added (0232); the weak
  differences (6-20 hits) are of files the NAS store gained or lost since.

**Conclusion.** No figure moved. Receipts written before session 78 are refused once and
rendered again; with FFmpeg 8.1.3 on the Linux machine that gave the same audio for
the sweep. Baselines `session-78-linux` and `session-78-linux-nas` hold the sweep, the
three development scans and Stakka's leave-outs.
