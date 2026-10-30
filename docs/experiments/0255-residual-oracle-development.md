# 0255: the masked residual with the true time map, development seeds

**Question.** Proposal 4, oracle first (brief 65's pre-registration, `668c091`): once A is found confidently, does removing what A explains let the search find the quieter or brief record B under it, with nothing confident that is not playing?
**Method.** `gunfinger-eval --work work/s65/wd --panels work/s65/panels --ladder turntable mixes --kind drops --seed 2026|2028 --count 16 --other-root /mnt/atlas/Music/dnb --tagged work/s39/tags-262.json --residual --residual-over 2|1` on session 39's renders (2:38-2:44 per seed and rule, 4.3 GB peak, 32 threads), 262 tracks. Every play found confidently in the whole mix is an A (441 cases); its audible span is searched plain (P), with query peaks in an 8 x 8 neighbourhood (D), masked (M) and masked without the quiet and brief Bs' recordings (MH). A's reference is the generator's render of A (the played file: 309 cases), or of another indexed rip placed by its stored peaks' alignment (132). `scripts/analysis/residual_pilot.py`.

B inside A, both seeds, `over` 2 (candidate / confirmed / confident):

| B inside A | n | P | D | M | M, `over` 1 |
|---|--:|--:|--:|--:|--:|
| Quiet (-9 dB) | 32 | 19 / 15 / 4 | 24 / 14 / 4 | 32 / 30 / 22 | 30 / 23 / 12 |
| Brief (20-40 s) | 32 | 28 / 26 / 4 | 30 / 25 / 2 | 32 / 32 / 25 | 32 / 32 / 16 |
| Unrelated drop, inside | 32 | 32 / 31 / 23 | 32 / 31 / 23 | 32 / 32 / 32 | 32 / 32 / 30 |
| Pair drop, inside | 19 | 19 / 19 / 12 | 19 / 19 / 11 | 19 / 19 / 19 | 19 / 19 / 19 |

- Wrong confident: 0 in every arm and case, with both rules; MH (the 64 quiet and brief excerpts, B's recording left out) 0 confident, 0 possible. Wrong possible: P 181, D 135, M 6 (`over` 2; all Clockwork's two files at 61-78 hits in 3 excerpts of other records), 0 (`over` 1).
- The plain excerpt agrees with the whole-mix search: quiet Bs 4 confident, 10-11 possible (0184: 4, 10). Median hits of B's best detection: quiet 97 (P, 19 found) to 374 (M, 32); brief 137 to 333.
- What the mask leaves of A (`over` 2): from the played file, 0 confident and 4 possible of 309; from another rip, 20 confident and 10 possible of 132 (17 of the 20 in the rips scenario). With `over` 1 A stays confident in 240 of 441.
- Only 2 of the 64 quiet and brief As have another indexed rip (both seeds); the quiet and brief rows rest on identical audio.
- Of the 21 quiet and brief Bs that M confirms and P does not, 6 have a plain detection of two windows or more.

**Against the pre-registration.** `over` 2 confirms 21 more quiet and brief Bs than P (62 against 41) and 23 more than D (39), with no wrong confident answer in M or MH; but 6 of the 21 have a detection in the plain excerpt, not half. By the registered reading, estimated timing is not tried in this session. The rule frozen for the test seeds: `over` 2 (more confirmed, no wrong confident answer).

**What it shows.** With A's exact time map and A's own audio, removing A's power recovers nearly every quiet and brief partner, and most confidently, without a confident absent record; denser query peaks without masking add candidates (quiet 24 against 19) but confirm fewer (39 against 41). The evidence is mostly not visible in the original mixture as a detection. **Does not show** the mask with a different rip of A for these blocks (2 cases), estimated timing, broadcast processing, or NAS scale; another rip left A confident in 20 of 132 cases, so identical audio carries much of the result. Data `data/0255-residual-oracle-development.json`.
