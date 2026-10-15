# 0119: the FFT lengths' variant on the existing panels

**Question.** Session 22, part 1 of proposal 2's retrieval study (rules in
`docs/brief-22.md`, committed first): what does factor2 in the first
pass's turntable rungs (0109, 0117) change on each existing panel?

**Method.** Session 21's patch at `9aecef4` (worktree, never merged),
each build with its own `--work`, store `b285da9984422fb5`;
`fft_compare.py` (files per answer kind; bounds to 1.0 s); `data/0119-*`.

| Panel, today -> variant | Right confident | Wrong conf. | Possible right | Bounds > 1 s, per run |
|---|--:|--:|--:|--:|
| Sweeps 2026-2029, 262 (2,880 queries) | 2,160 = | 0 = | = | |
| Other rips, NAS, 2026-2029 (1,413) | 1,296 -> 1,297 | 134 = | -1, +1 | |
| Recall development half, NAS (702) | 522 = | 57 = | = | |
| 3 dev. mixes, 262: scan, leave 3, all | 27, 18, 0 = | 0 = | = | 22 |
| 3 dev. mixes, NAS: scan, all left out | 40, 0 = | 9, 9 = (0115) | = | 43, 7 |
| Generated mixes 2026, 7 (168 plays) | 125 = | 0 = | 11 = | 5 (2 worse) |
| Grid seed 2026 (960) | 693 = | 0 = | 255 -> 253 | |
| Robust seed 2026 (2,600 indexed) | 1,941 -> 1,945 | 0 = | -6, +6 | |

- Confident: 2003 mix (NAS), the tail segment at 5:41-6:43 passes from
  The Specialist's files (253 -> 46 hits) to Sex Drive's (195 -> 631),
  all credited to track 2; Into Music's `extra/h-` rip loses a 695-hit
  segment (679 -> 20 at 262) inside a play that stays confident. Robust:
  20 lost, 24 gained, each crossing the rule (29 at 3 windows, 28 at 240
  hits, 13 both): needle skip 12/3, pitch ride 7/20, beatmatched 1/0,
  clipping 0/1. Files: Blindside's rip -1% and 3 recall rips possible ->
  confident; a C4C - Synergy rip at +1% confident -> possible.
- Possible: grid 10 s Swarm (1.054; 235 -> none), Lava Flow (0.962;
  276 -> 48, 11 s off); Fractles 61 -> 54 (-1%), 40 -> 70 (-5%); Omni
  Trio's Silent Storm Remix added (70 hits). Fade-in starts: Kosmic 2.31
  -> 3.63 s late, Kontempt 2.30 -> 3.71; 3 closer. Key-lock: same bytes.

**Part 2 needs** a `sweep` speed list (now on-rung and midway only),
sub-hop offsets, the first pass traced, a first-pass call path, timing.

**Conclusion.** No wrong confident answer added; no development track
lost. Edge answers move both ways, so the criterion "no right confident or
possible file lost" and the boundary criterion fail. Not adopted.
