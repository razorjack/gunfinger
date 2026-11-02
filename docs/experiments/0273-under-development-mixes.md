# 0273: `identify --under` on the development mixes and Stakka's leave-outs

**Question.** Does `--under` (ADR 0025) change real mixes' tracklists or
print anything wrong, also with tracks missing (brief 73, D and L)?

**Commands.** `work/s73/cost.sh` (D): `identify -f json` of the three
development mixes, default and `--under` (`s73-start`), at 262 tracks
and NAS scale, the whole library; `show`. `work/s73/effect.sh`,
`leave-nas.sh` (L): Stakka with `--exclude-from` each of the 14
leave-out patterns of `leave_out_design.py` (every file of the left-out
slots' duplicate clusters), default and `--under` (the timing variant,
JSON equal to the product's) at 262 tracks, `--under` only at NAS scale
(the default there is the report without `under`; D and 0274 show the
plays identical). `under_score.py diff`, `manifest`, `fates`.

**Result.**

| Input | Tracklists | Entries lost; gained | Gap lines changed | Plays found under (level) |
|---|--:|--:|--:|---|
| D, 262 tracks | 3 | 0; 0 | 0 | none |
| D, NAS scale | 3 | 0; 0 | 0 | 1 possible (razorjack) |
| L, 262 tracks | 14 | 0; 0 | 0 | none |
| L, NAS scale | 14 | 0; 0 | 0 | none |

- The one find: Carlito & Addiction - Into Music (vinyl rip), possible,
  88 hits, 22:14-22:52, under Omni Trio - Kinetic, 75 s before the
  tracklist's own Into Music entry (24:07, another upload; manifest slot
  7 at 24:24, a start Gunfinger gave). By the manifest it lies in slot 6
  and is not right; whether the next track is already in the blend there
  only listening can tell. Possible: not a wrong answer (ADR 0006), and
  out of the tracklist.
- What the step dropped (timing variant): on D and L every detection at
  possible or above in a residual was of the entry's own files
  (`taken-out`) or of a track confident within 90 s (`named-nearby`);
  the shared-material check held back nothing. Plays of D and gaps are
  identical with and without the option.

**Conclusion.** On the development mixes and Stakka's leave-outs the
option changes no tracklist line: no entry gained or lost, nothing
wrong. Its value shows on double drops (0274), not here.
