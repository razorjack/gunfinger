# 0284: `identify --under` with a mistaken foreground or a left-out partner

**Question.** When the louder record's file is out of the library, so
that a related version is its best match, or the quieter record's file
is out while a related version stays, does the option print a wrong
confident partner (brief 80, P1-P3)?

**Commands.** As 0283: `mixes --kind drops --case mistaken-over` (seed
8008) and `--case related-under` (8009), 12 mixes each, without
`--tagged`; each case's `--exclude-from` adds every NAS file of its
left-out recordings (`tracklist_score.py exclude-left-out`), for all 12
mixes. Build `s80-run2`.

**Result.** 24 left-out plays per case; 7 and 5 panel families, 88 and 85 recordings.

| Case | Gained (right) | Confident under (wrong) | Possible under (matching nothing, by the script) |
|---|--:|--:|--:|
| mistaken-over | 11 (8) | 22 (3) | 15 (2) |
| related-under | 3 (3) | 8 (0) | 8 (7) |

- mistaken-over: over 9 of the 24 left-out louder records the default
  prints a confident entry (7 related versions: Pyro - Ignorance 4, The
  Nine (Evol Intent VIP) 2, Spider Monkey 1; 2 China Cup uploads), over
  15 nothing. Taking those out gains 1 right brief partner; no wrong one.
- P1 hit in mistaken-over, 3 entries, all in the mix's own pair drop of
  The Nine and China Cup, both left out by other mixes' blocks: 55 s of
  `ƆƐI3C - China Cup` under The Nine VIP while China Cup plays (our
  same-title judgment; the right title), and 87 s and 92 s of `The Nine
  (Evol Intent VIP)` under a China Cup upload while The Nine plays: a
  related version of the played partner, the kind ADR 0025 accepts,
  which the registration exempted in related-under only.
- related-under: no related version of a left-out partner is confident.
  The script's 7 possible finds are 5 related versions of left-out
  partners (Pyro - Ignorance 4, Sonar (Mark System Revision) 1), Phace -
  Brainwave under its VIP, and another upload of Decoy, which plays.
  P2 is hit by the script, not by the records playing.
- P3 not hit: no default line lost, no gap line changed.

**Conclusion.** A related version taken out as the louder record left no
wrong partner; the wrong titles are related versions of a record playing.
