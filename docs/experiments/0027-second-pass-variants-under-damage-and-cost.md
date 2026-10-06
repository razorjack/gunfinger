# 0027: the second-pass variants under damage, padded, and their cost

**Question.** The rest of item 6 (0026 has the protocol): every robust
condition with the filter on and off, both ladders, the padded index, cost.

**Command.** `robust` (seed 2026, 38 conditions) with `--ladder both` and
`turntable`; `scan` and `robust --only control,blend--6db,combined` with
`--synthetic-copies 30` (8,122 assets); `memory --synthetic-copies 100
--minutes 5 [--count-lines]`, 1 worker (26,462 assets). Run at 200 hits;
confident at 240 recomputed from the saved hits. Data:
`data/0027-second-pass-variants-under-damage-and-cost.json`.

| Robust (excerpts of 40 per row) | Default | Second pass | Drop | Skip |
|---|--:|--:|--:|--:|
| Both: confident at 200 (at 240) | 1,959 | 2,031 (1,997) | 1,954 (1,884) | 2,031 (1,995) |
| Both: possible or better | 2,258 | 2,274 | 2,230 | 2,273 |
| Both: strongest wrong; wrong shown as possible | 104; 46 | 108; 43 | 76; 23 | 108; 44 |
| Turntable: confident at 200 (at 240) | 1,759 | 1,861 (1,805) | 1,760 (1,686) | 1,860 (1,802) |
| Turntable: possible or better | 2,128 | 2,146 | 2,097 | 2,144 |
| Combined damage, of 80: confident (at 240); possible+ | 7; 64 | 7 (4); 65 | 3 (3); 53 | 7 (4); 65 |
| Padded: development scan; CPU s | 11/11; 1,844 | | 11/11; 686 | 11/11; 689 |
| Padded: strongest chance on a reversed copy | 31 | | 18 | 25 |
| Padded robust, of 240: confident (at 240) | 157 | | 150 (141) | 159 (156) |
| 26,462 assets: CPU s; first-pass distinct lines | 457-462; 1,744,450 | 473; same | 193; 112,114 | 190-197; 112,114 |

No variant gives a wrong confident answer in any row. Skip at 240 against
the default: needle skip 1 → 31 of 80, beatmatched blends +4, broadcast
+3, blends +5; combined 7 → 4, pink noise at 10 dB 77 → 75, Opus 80 → 79.
Drop at 240 loses under echo, clipping and noise (-11 to -18 of 80 each).

**Cost** (user CPU, loaded machine). Robust, both ladders: second pass
6,836 s, skip 6,667 s, drop 6,257 s; sweeps: second pass 1,688-1,714 s,
skip 1,556-1,591 s, drop 1,533-1,538 s. Peak memory at 26,462 assets
could not be compared: the binary of 0021 peaked at 6.5 GB today against
4.1 GB then. Drop builds a second index (5.6 GB while building).

**Conclusions.** (1) At 240, skip confirms 36 more excerpts than today
under damage and loses only where little evidence is left; drop confirms
75 fewer. (2) The filter pays at scale: 57-63% less search CPU and 94%
fewer first-pass lines; at 262 tracks it saves 2-10%.
