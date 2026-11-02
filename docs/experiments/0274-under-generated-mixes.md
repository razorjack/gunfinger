# 0274: `identify --under` on generated mixes at NAS scale

**Question.** What does `--under` (ADR 0025) add to generated mixes'
tracklists against the default, and is anything wrong (brief 73, G)?

**Commands.** `work/s73/gen.sh`: `mixes --kind drops --tagged
work/s39/tags-262.json` on unused seeds 2050 and 2053 (16 mixes each;
2053 replaced 2051, which drew no pair) and `--kind long` on 2052 (4
mixes of 72-84 min). `work/s73/effect.sh`: `identify`, NAS
configuration, `--exclude-from` each seed's held-out recordings,
`--save-dir`, default (`s73-start`) and `--under` (the timing variant,
JSON equal to the product's: 0275), `show`; `tracklist_score.py score`,
`under-summary`, `under-fates`.

**Result.** In all 36 mixes the plays are identical, no entry is lost
and no gap line changes. Drops mixes, both seeds, truth plays printed:

| Partner (second record) | Plays | Default | `--under` | Possible under only |
|---|--:|--:|--:|--:|
| quiet, 9 dB down, inside the first | 32 | 2 | 18 | 11 |
| brief, inside the first | 32 | 6 | 18 | 13 |
| unrelated drop, inside | 32 | 20 | 29 | 1 |
| pair drop (related version), inside | 20 | 11 | 11 | 0 |

- Entries gained: 38, all right; 37 name a partner the default left
  out, one repeats a track the default prints (another upload of Jo-S -
  Afterburner, 6.7 s apart in line, tagged differently: ADR 0017 keeps
  them apart). Wrong confident finds: 0 of 56; possible 85, 3 matching
  no track playing. Start: median 1.6 s late (0.5-5.0 s, one 9.8 s).
- Shared material held back only pair drops' related versions (22
  confident detections, 11 partners), as designed. The rule "one track
  with a confident play" dropped two right partners: Black Out (333
  hits, confident) and a quiet partner found as possible (111 hits); each
  line put its file's start within 0.5 and 3.5 s of the file start of a
  confident play of another track ending 33-36 s before the find.
- Long mixes: nothing found under a confident play.

**Conclusion.** The option prints 37 partners the default leaves out
(quiet 2 to 18, brief 6 to 18, of 32 each), with no wrong line. The
start test of its fourth rule dropped two right partners.
