# 0089: listing the NAS while files are added to it

**Question.** With ADR 0015 (`26bc727`, `6b377cb`), does the NAS listing
finish while files are added to it, with the same plays through the
library, and does the fix hold in a real race?

**Method.** `doctor --config nas-dnb.toml` 14 times (store
`f9f9bc706195a71b`, read-only): 1-4 with `work/bin/s13-fix/` (a build
ran beside run 4), 5-6 with `s13-review`, 7-14 alternating `s13-start`
and `s13-review`, idle. `identify` of the 2003 mix through the library
(no `--store-only`, `-f json -v`) with `s13-review`, then `s13-start`. A
race on a local disk (`work/s13/race.sh`, `race-prune.sh`): 200
folders x 20 files while `churn.py` deletes and recreates files and
renames folders. Data `data/0089-vanishing-files-on-the-nas.json`.

| `doctor` | Runs | Audio files | Vanished | Seconds |
|---|--:|--:|--:|--:|
| `s13-fix`, 18:31-18:45 | 4 | 36,550-36,565 | 0 | 189-253 |
| `s13-review`, 18:56-18:58 | 2 | 36,584-36,585 | 0 | 31-44 |
| `s13-start` / `s13-review`, alternated | 4 / 4 | 36,592-36,600 | 0 | 40.1-47.7 / 28.0-28.9 |

`identify`: 87.8 s (`s13-review`), 94.6 s (`s13-start`), the saved index
loaded, 26 plays (24 confident), nothing vanished, reports byte-identical.

| Race on a local disk | Nothing vanished | Some vanished | Could not list |
|---|--:|--:|--:|
| `doctor`, start build (30) | 20 | - | 10 |
| `doctor`, `s13-review` (30) | 21 | 9 | 0 |
| `prune --yes`, start build (20) | 16 | - | 4 |
| `prune --yes`, `s13-review` (20) | 14 | 6 (refused) | 0 |

**Conclusions.** Nothing vanished in 16 NAS listings (the NAS grew by 50
audio files in 35 minutes), so the NAS did not exercise the fix; the race
did: every listing finished, the note named the first vanished path
(`1 file or folder (`folder-172/14.mp3`)`), and `prune` refused instead of
failing. Both builds find the same plays through the library. `doctor` is
about 12 s faster with `s13-review`, consistent with not reading the
metadata of about 5,900 files passed over by name or ignored (about 2 ms
each). Runs 1-4 took 189-253 s, later runs of both builds 28-48 s: the
NAS answered faster after 18:45, for reasons not measured.
