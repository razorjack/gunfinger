# 0272: `identify --under`, a first look (not the measurement)

**Question.** Does `--under` (session 72, ADR 0025, proposed) print what
it should on a real and a generated mix, and what does it add to the
time? Session 73 measures cost and recall; this only looks.

**Commands.** `work/s72/look.sh` (build `work/bin/s72-run2/`): `identify
MIX [--under] --verbose --format json`, then `show` (`work/s72/look2/`). Mixes: the Stakka & Skynet mix (development) and
generated `drops-seed-2026/mix-00` (session 39's render, a development
seed), at 262 tracks and NAS scale; one run each, the machine otherwise
idle, 32 workers.

**Result.**

| Mix, scale | Entries searched under | `identify` without; with | The step | Peak memory without; with |
|---|--:|--:|--:|--:|
| Stakka, 262 tracks | 11 | 5.8 s; 29.3 s | 23.5 s | 191; 536 MB |
| Stakka, NAS scale | 11 | 18.4 s; 51.4 s | 32.9 s | 5.0; 5.2 GB |
| mix-00, 262 tracks | 16 | 2.9 s; 18.8 s | 15.9 s | 150; 374 MB |
| mix-00, NAS scale | 17 | 13.8 s; 33.3 s | 19.5 s | 4.9; 5.0 GB |

- Stakka: nothing found under a confident play at either scale.
- mix-00: one new entry at both scales, `6. 11:08 Muffler - Nutcracker
  (Original Mix) (under 5.)` (confident, 541 hits, 11:08-12:27): the
  quiet partner of the generator's block (11:05-12:33, 9 dB down) under
  Rascal & Klone - Snake Eyes, which the plain search had as possible
  only (207 hits; at NAS scale 221 and 138), left out of the tracklist.
  At NAS scale an `Extended Mix` upload lines up with it (424 hits) and
  joins the entry (`6. 11:07 ...`, `titles disagree:`). No other line
  changes.
- A first pass, before two fixes (brief 72, 05:00), printed the two NAS
  files as two entries and also found two right partners as possible
  through other uploads of tracks already confident there (Echo Chamber,
  Gridlok - Dilusion); the fourth drop rule now drops those.

**Conclusion.** The option prints the partner the masked residual is
known to find, and nothing else here. It costs 1.0-3.0 s per confident
entry: 5-6.5 times `identify`'s time at 262 tracks and 2.4-2.8 times at
NAS scale, with 0.2-0.35 GB more memory at 262 tracks. Whether that is
acceptable by default is session 73's question.
