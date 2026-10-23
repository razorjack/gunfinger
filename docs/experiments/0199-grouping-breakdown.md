# 0199: where grouping's time goes, and parts of 256 to 2,048 assets

**Question.** Session 43, step 1: what do grouping's count pass, prefix
sum and scatter cost; how are hits spread; does another part size pay
now that the filter is cheaper (0192)?

**Method.** A diagnostic build of master, never committed
(`work/s43/diag.diff`): per part, each worker sums `Instant` intervals
(taking, counting, prefix sum, scatter, then filter, sorting and lines)
and counts from `ends`; builds at parts of 256, 512, 1,024 and 2,048
assets. The Stakka scan at NAS scale, 32 jobs then 16 (reversed order),
otherwise idle; all eight reports identical to the start build's apart
from `wall_seconds`. Data `data/0199-grouping-diag.txt`.

| CPU in the first pass, 32 (16) jobs | 256 | 512 | 1,024 | 2,048 |
|---|--:|--:|--:|--:|
| Whole first pass | 809 (498) s | 770 (480) | 750 (467) | 774 (469) |
| Taking parts | 153 (84) | 107 (64) | 84 (48) | 102 (48) |
| Count pass | 53 (33) | 54 (33) | 53 (33) | 54 (33) |
| Prefix sum | 0.7 (0.4) | 0.6 (0.4) | 0.6 (0.3) | 0.5 (0.3) |
| Scatter | 199 (154) | 204 (156) | 207 (158) | 218 (160) |
| Filter, sorting, lines | 227 (127) | 228 (128) | 228 (128) | 224 (128) |

At 1,024 assets and 32 (16) jobs, per hit: taking 1.69 (0.96) ns, the
count pass 1.07 (0.66) ns, the scatter 4.16 (3.18) ns, 27.6% (34.0%) of
the first pass; query analysis 163 (90) s. **Counts**: 1,105,360 parts
of 45,060 hits on average (98.8% hold 16,384 or more; at most 82,303);
607 list runs a part, 74 hits a run; 1,017 assets with a hit a part, 44
hits each (of those: fewer than 8 hits 1.2%, 8-15 4.6%, 16-31 26.6%,
32-63 51.4%, 64-127 15.0%, 128 or more 1.1%; at most 5,678). The
scatter's writes touch 6,522 cache lines a part summed per asset (5,633
if packed). Rooms of 32, 64, 96, 128 hits an asset leave 35.1%, 8.4%,
2.6%, 1.1% of hits beyond (in 65.4%, 15.4%, 3.5%, 1.1% of those assets).

**Conclusion.** The scatter is the cost, about 4 ns a hit at 32 workers
whatever the part size; at 256 assets its places (16 KiB of lines) fit
in L1 and it is no cheaper, so by inference the number of places is not
what limits it. Smaller parts make taking dearer (more, shorter runs);
1,024 assets has the lowest first-pass CPU at both job counts, so a part
size alone is not a candidate (brief 43's pre-registration).
