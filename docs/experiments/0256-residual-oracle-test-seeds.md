# 0256: the masked residual with the true time map, test seeds

**Question.** Brief 65, step 4: does 0255's frozen rule (`over` 2, the generator's time map) give the same picture on two seeds nothing was tuned on?
**Method.** Seeds 2036 and 2039 (the first two from 2036 up whose panel keeps both Clockwork records indexed; panels drawn with `--count 0`, `work/s65/panels`), rendered for this run: `gunfinger-eval --work work/s65/wt --panels work/s65/panels --ladder turntable mixes --kind drops --seed 2036|2039 --count 16 --other-root /mnt/atlas/Music/dnb --tagged work/s39/tags-262.json --residual --residual-over 2` (3:29 and 3:03 with the renders, 7.1 GB peak), 262 tracks. 445 cases: the played file as reference in 317, another rip in 128. `scripts/analysis/residual_pilot.py`.

B inside A, both seeds (candidate / confirmed / confident):

| B inside A | n | P | D | M |
|---|--:|--:|--:|--:|
| Quiet (-9 dB) | 32 | 21 / 15 / 3 | 25 / 16 / 2 | 32 / 32 / 24 |
| Brief (20-40 s) | 32 | 31 / 21 / 9 | 32 / 24 / 6 | 32 / 32 / 24 |
| Unrelated drop, inside | 32 | 32 / 32 / 25 | 32 / 31 / 24 | 32 / 32 / 32 |
| Pair drop, inside | 17 | 17 / 17 / 9 | 17 / 17 / 9 | 17 / 17 / 17 |

- Wrong confident: 0 in every arm and case, and 0 in the whole-mix searches; MH (64 excerpts, B's recording left out) 0 confident, 0 possible. Wrong possible: P 196, D 156, M 0.
- Median hits of B's best detection: quiet 134 (P, 21 found) to 390 (M); brief 107 to 325.
- What the mask leaves of A: from the played file, 0 confident and 11 possible of 317; from another rip, 24 confident and 9 possible of 128.
- 4 of the 64 quiet and brief As had another rip; their 4 Bs are confirmed in M (3 confident).
- Of the 28 quiet and brief Bs that M confirms and P does not, 16 have a plain detection of two windows or more.

**Against the pre-registration.** The development reading holds on the test seeds: M confirms every quiet and brief B (64 against P 36 and D 40) with no wrong confident answer in M or MH. Here 16 of the 28 newly confirmed have a plain detection (over half), but the rule for estimated timing was read on the development seeds (0255: 6 of 21), so it is not tried.

**What it shows.** The oracle's upper bound reproduces on fresh seeds: with the true time map, masking A finds the records under it nearly always, mostly confidently, and adds no wrong answer in these mixes; it also removes the possible shared-material lines under A (196 to 0). Denser query peaks without masking change little. **Does not show** estimated timing, another rip for most As (4 of 64), broadcast processing or NAS scale (0257). Data `data/0256-residual-oracle-test-seeds.json`.
