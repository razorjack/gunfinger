# 0292: targeted extra rungs (T1, T2): extra rungs over the gaps only, against the whole query

**Question** (brief 84, registered in `0d2c4da`). Do extra rungs searched only where nothing is
confident recover what whole-query ladders recover, at almost no cost? **Method.** `--gap-rungs K`
(`bd7d2a4`): stretches of 10 s or more that no segment of a confident play covers, padded by 10 s,
searched on the 4K rungs `--extra-rungs K` adds, joined by ADR 0021's second suppression. Arms L0,
L1, L2, T1, T2; seed 2026; every scored run exit 0; timing alone (03:07-03:18, L0 T1 L1 [T2 L2 L2
T2] L1 T1 L0, 32 workers). Data: `data/0292-targeted-extra-rungs.json`; `gap_search_*.py`.

| | L0 | L1 | T1 | L2 | T2 |
|---|--:|--:|--:|--:|--:|
| C1 edge sweep, 262: confident of 1,080; reach | 225; 8.4% | 352; 8.8% | 352; 8.8% | 470; 9.2% | 470; 9.2% |
| C1, NAS: confident; wrong (new); wrong possible | 322; 9 (0); 3 | 434; 9 (0); 9 | 434; 9 (0); 5 | 545; 9 (0); 15 | 545; 9 (0); 8 |
| C2 other copies, NAS: within 8.4% of 562 | 552 | 551 | 552 | 551 | 552 |
| C2: (8.4, 9.2] of 160; (8.4, 9.6] of 239 | 100; 101 | 153 | 153 | 227 | 227 |
| C2: wrong confident; wrong possible | 0; 24 | 0; 29 | 0; 24 | 0; 34 | 0; 26 |
| C3 protocol: new confident answers (262; NAS) | | | 0; 0 | | 0; 1 |
| C4 edge mixes: edge plays past 8.4% of 92 (262; NAS) | 28; 41 | 44; 56 | 44; 50 | 72; 76 | 65; 67 |
| the same without the returning play (of 69) | 22; 29 | 30; 38 | 34; 38 | 50; 54 | 51; 52 |
| C5 robust, 262 (2,600): confident; wrong | 1,938; 0 | 1,938; 0 | 1,938; 0 | | |
| C6 NAS CPU: Stakka 688.7 s, 2003 555.9, Nocny 883.5 | ×1 | ×1.047, 1.045, 1.046 | ×1.001, 1.000, 1.003 | ×1.095 | ×1.004 |

C4's other plays: 108 (262) and 109 (NAS) of 116 in every arm; its one wrong answer is L0's. C6's
ranges lie within 0.8% of each median. T searches 0 s of Stakka and 2003 at NAS scale, 6.1% of
Nocny; at 262 tracks 0, 39% and 54%. C3 under T1: no answer changes (63 of 720 NAS sweep queries
change only below 60 hits); calibrate as L0 (262: 690, 100; NAS: 704, 282). T2 adds 0227's bootleg
(Cool Rock Stuff on "Cool Rock Shook Ones" at +5%, 268 hits), as 0227 inferred. **C4's losses.**
The returning play (position 7 of 12, three plays after its confident first play) is always an edge
play here; `plays::group` joins the default's possible detection of it to that play (within 90 s),
so its time counts as covered. 6 of T1's 8 losses against L1 at NAS scale are that play (3 keep the
default's evidence unchanged; 3 are found more weakly over a partly covered stretch); 2 are found
more weakly over the gap (184 against 1,538 hits; by inference chains across a default and an extra
rung, which form only in L1). T1 finds 2 plays L1 misses. **Against the registration.** T1: G1
holds (8.8% > 8.4%); G2 holds (+33.1 points; 552 within 8.4%, where L1 loses the Siren Bass query);
G3 holds at 262 tracks (44 = 44) and fails at NAS scale (50 against 56, 6.5 points, 5 allowed); W
and C hold. T2: G1 and G2 (+52.7) hold; G3 fails (7.6 and 9.8 points); W fails. Neither passes. By
0229's counts T1's reach leaves 19 copies alone for a deck at ±5% (L0 29), T2's 7. **Conclusion.**
Over the gaps alone, extra rungs give the whole-query ladders' sweep recall, plus the query those
lose, with fewer wrong possible answers, for under 0.3% of CPU on the development mixes at NAS scale
(L1 4.5-4.7%); they miss edge plays whose time a confident play of the same file covers.
