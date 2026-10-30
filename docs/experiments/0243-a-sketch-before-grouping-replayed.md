# 0243: a sketch before grouping, replayed on captured windows

**Question** (brief 60, proposal 2 of the 2026-10-29 note). If the hits of
a part are first counted in a small table of folded pairs per asset, and
only the assets it flags are grouped, how many assets and hits does it
flag, are the lines unchanged, and what does `lines` cost?
**Method.** Pre-registered (brief 60, `4d9687a`); the Linux machine. A diagnostic build, never
committed, captured the lists, points and rung of sampled windows of the
Stakka scan (one window and rung in 400 at NAS scale, one in 40 at 262
tracks; both scans' reports identical to the baselines'). A prototype
(`work/s60/sketch.diff`, never committed) adds `PartSketch`: a row of
`M + 1` pair bytes per asset of a part, slots folded mod `M`, the filter's
word add and test; grouping counts and scatters only flagged assets. An
ignored test replays each window through today's `lines` and the layouts
F64, F256, F1024 (best of 3, one thread), compares the lines by their bits
and in order, and counts each asset with 3 hits or more alone. Data
`work/s60/replay-*.txt`.

| Of assets with 3 hits or more (hits) | 262 tracks | NAS scale |
|---|--:|--:|
| Windows; hits | 707; 11.6 M | 80; 141.1 M |
| `may_form_line` passes | 5.16% (10.8%) | 2.05% (4.3%) |
| F64 flags (66.6 KB a worker) | 97.0% (99.3%) | 91.3% (97.1%) |
| F256 flags (263 KB) | 75.5% (86.8%) | 54.2% (70.2%) |
| F1024 flags (1.05 MB) | 24.7% (37.3%) | 12.4% (21.3%) |
| `lines`, today | 0.074 s | 0.822 s |
| `lines`, F64 / F256 / F1024 | +26.0% / +24.2% / +9.7% | +33.7% / +35.2% / +13.1% |

Lines identical, bit for bit and in order, in every window under every
layout and in a dense synthetic window where every asset passes (there
the sketch adds 4.5-5.8% to `lines`). No window had inexact phases.

**Conclusion.** The sketch is conservative (tests and replays agree);
the fewer folded slots, the more it passes: at 44 hits an asset, most
find 3 in a pair of 64 or 256 slots. At NAS scale F1024 still scatters
21% of the hits, and fills and counts in 1 MiB per part;
on one thread every layout is slower than today's (by inference, the
counts and the fill cost more than the scatters saved). F1024 has the
lowest replay CPU and goes to the full searches (0244), where 32 workers
share the cache and the scatter costs more (0199).
