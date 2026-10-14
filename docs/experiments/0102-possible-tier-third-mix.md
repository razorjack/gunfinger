# 0102: the possible tier on the third development mix

**Question.** Session 18, step 4: with 0092's method, how many possible
plays per hour does the third development mix (0100) show at 60, 80, 100
and 130 hits, of what, and is any true play in it found only as possible?
Measurement only; the threshold stays 60.

**Method.** `possible_tier.py` with the new mix's NAS scan and its three
leave-outs added (counted apart, and with the two earlier mixes), session
18's clusters and the store's tags now (`work/s18/tags.json`). A play is
possible when not confident and its strongest segment reaches the
threshold; labels: right, related (by audio: a 30-hit pair between the
clusters or a `different` verdict; else by tags), or unrelated.

| Threshold (hits) | 60 | 80 | 100 | 130 |
|---|--:|--:|--:|--:|
| New mix (1.11 h): possible plays per hour | 3.6 | 3.6 | 2.7 | 1.8 |
| of which related by audio | 4 of 4 | 4 | 3 | 2 |
| New mix's leave-outs (3.34 h): per hour | 3.6 | 3.6 | 2.7 | 1.8 |
| Three development mixes (2.82 h): per hour | 3.2 | 2.8 | 1.8 | 1.1 |
| True plays found only as possible (new mix, leave-outs) | 0 | 0 | 0 | 0 |

- The four, by strongest segment: Mampi Swift - 2000 (197 hits, during
  Stab 2 Def; 827 hits between them in `pair`), the untagged `1.mp3`
  (161, during Cannibal, holding a piece of it 6% faster), the Klute
  Remix (116 and 96, two files, during the VIP; 166-263 in `pair`). The
  same four appear in each leave-out, whatever is left out.
- The new mix's strongest detection matching no track, 66 hits in 5
  windows at 44:33-45:16, is labelled unrelated by the script but is
  Stab 2 Def itself: by ear it is blended in from 44:40, and the
  slot's window opens at 45:23. It joins Stab 2 Def's confident play.
- The strongest "unrelated" stay Meltdown (98) and Rubba Rock (64); the
  two were heard as shared material, which the script does not read.

**Conclusions.** The new mix adds 4 possible plays, all shared material
with the confident play over them, none unrelated, and no true play that
depends on the tier. With the listening answers of 2026-10-14, no possible
play in any development mix is chance. A higher threshold hides one Klute
Remix file at 100, both at 130; `1.mp3` and Mampi Swift stay to 130.
Data `data/0102-possible-tier-third-mix.json`.
