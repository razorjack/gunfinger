# 0239: sweeps 2027-2029, robust and grid under tonight's changes

**Question.** Session 57, group 4 (pre-registration of 2026-10-27): what do
the prune and ADR 0021-0022 change in the sweeps of seeds 2027-2029, in
calibrate over seeds 2026-2029, in `robust`'s rides and in `grid`?
**Method.** Build `s57-start`, `work/s57/wsweeps` (with the baselines' seed
2026 sweeps and Stakka scans, for calibrate) and `wrobust`. Old: session 40's
NAS sweeps, session 54's (before ADR 0022; after), session 38's robust and
its variant. Traces (brief 57): session 40's binary on today's store; today's
`--overlap-by-spans` (2027-2029, `grid`); `grid` by session 32's start build.

| Sweep (old -> today) | Recalled | Wrong confident | Same-title |
|---|--:|--:|--:|
| NAS 2027; 2028 | 540/540 | 11 -> 0; 7 -> 0 | 11; 7 |
| 262 tracks 2027; 2028 | 540/540 | 0 | 0 |
| 2029, NAS; 262 (no old report) | 540/540 | 0 | 0 |

- Against session 54's start build the NAS reports change only by ADR
  0022's credits (every detection identical); they equal session 54's run.
- Against session 40: ADR 0021 adds 8 and 4 weak; the prune removes 5 and
  17 pruned files' detections and, outside its pre-registered definition
  (attributed by trace; mechanism not traced), moves 9 confident refits, 2
  confident 2 hits lower, 26 weak (2027) and 1 weak (2028). Session 40's
  binary on today's store gives every move; today's `--overlap-by-spans`
  equals it. 2029: ADR 0021 adds 1 weak (262), 5 (NAS); nothing else.
- Calibrate, seeds 2026-2029 and the Stakka scans: at 262 tracks weakest
  identifying 675, strongest false 100, margin 6.75x, 0 accepted, 49 false
  at 30 hits or more, audio not in the index 21; at NAS scale 675, 282
  (Cool Rock Stuff), 7 accepted (all Cool Rock), 365 at 30 hits or more,
  18 same-title credits. 0233's 2026-2028 figures stand.
- `robust`'s five rides: confident 310 -> 313 (262) and 339 -> 343 (NAS),
  3 and 4 excerpts with a ride inside going from possible to confident
  (0180's gain); wrong 0 and 4 unchanged; equal to session 38's variant.
- `grid`: identical under both rules: 0, 65, 144 of 160 confident at 10,
  15, 20 s, 0 wrong. Session 32's build gives the same, so the Mac's 0,
  67, 146 (0047, 0137) differ by the machine (session 32: hits within
  3.7%), not by tonight's changes.
**Conclusion.** Every difference is explained by the prune, ADR 0021 or
ADR 0022; no new wrong answer, no lost identification, and the level
changes are `robust`'s rides that ADR 0021 was adopted to keep. No finding.
