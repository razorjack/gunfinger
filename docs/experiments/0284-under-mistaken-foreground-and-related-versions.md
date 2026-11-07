# 0284: `identify --under` with a mistaken foreground or a left-out partner

**Question.** With the louder or the quieter record's files out of the
library, does the option print a wrong confident partner (brief 80)?

**Commands.** As 0283: `--case mistaken-over` (8008), `related-under`
(8009), 12 mixes, no `--tagged`, `s80-run2`; `--exclude-from` adds each
left-out recording's NAS cluster files (`exclude-left-out`), all mixes.
Caveats: left-out records play at a drawn speed, not beatmatched; two
left-out pairs may share a family; the exclusion follows clusters only,
so a same-title China Cup upload stayed indexed (also 0283's controls).

**Result.** 24 left-out plays per case; 7 and 5 panel families, 88 and 85
recordings. "Script": `tracklist_score.py` (no same-title credit).

| Case | Gained (right) | Confident under (wrong, script) | Possible under (matching nothing, script) |
|---|--:|--:|--:|
| mistaken-over | 11 (8) | 22 (3) | 15 (2) |
| related-under | 3 (3) | 8 (0) | 8 (7) |

- **P1 failed in mistaken-over: 2 entries** (script 3), in the pair
  drops of mixes 01, 04 and 05, all of The Nine and China Cup, both out
  of the library because other mixes leave them out. 87 s and 92 s of
  `The Nine (Evol Intent VIP)` under a China Cup upload while The Nine
  plays: a related version of the played partner, the kind ADR 0025
  accepts, exempted by the registration only in related-under. 55 s of
  `ƆƐI3C - China Cup` under the VIP while China Cup plays has the right
  title (`docs/same-titles.txt`). Why the shared-material drop kept it: open.
- The default prints over 9 of the 24 left-out louder records (6 related
  versions: Ignorance 4, The Nine VIP 2; 2 China Cup uploads; the quiet
  partner once); its 25 wrong confident plays (11 China Cup upload, 10
  VIP, 4 Ignorance) lie mostly on the mix's own pair plays.
- related-under: no related version of a left-out partner confident.
  P2 not hit (1 of 8 matches no played family): of the script's 7, 5
  Ignorance and 1 Sonar (Mark System Revision) are of a left-out
  partner's family; Phace - Brainwave is found while its VIP plays. P3
  not hit in either case.

**Conclusion.** The bar failed in mistaken-over through 2 related versions
of a partner that plays; no unrelated title was printed.
