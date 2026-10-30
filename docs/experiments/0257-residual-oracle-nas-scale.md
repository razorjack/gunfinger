# 0257: the masked residual with the true time map at NAS scale, other rips as the reference

**Question.** Brief 65, step 3: with the NAS store indexed, does 0255's frozen rule keep its recall and its precision, and what does it give when A's reference is another rip of A's recording (the NAS holds one for most As)?
**Method.** The development seeds' renders (copied to `work/s65/wn`), `gunfinger-eval --work work/s65/wn --panels work/s65/panels --ladder turntable --other-peaks-dir ~/.local/share/gunfinger/nas-dnb-peaks mixes --kind drops --seed 2026|2028 --count 16 --other-root /mnt/atlas/Music/dnb --tagged work/s39/tags-262.json --residual --residual-over 2` (11:29 and 12:19, 14.2 GB peak). The drops kind now draws with the corpus clusters at every scale (it could not draw at NAS scale before); all 32 plans equal session 39's. A's reference is the first other indexed file of its NAS cluster: a corpus rip (132 cases), a NAS file (271), else the played file (37).

B inside A, both seeds (candidate / confirmed / confident):

| B inside A | n | P | D | M | M, A from another rip |
|---|--:|--:|--:|--:|--:|
| Quiet (-9 dB) | 32 | 19 / 15 / 4 | 23 / 13 / 4 | 29 / 21 / 14 | 27: 24 / 16 / 10 (P 17 / 13 / 4) |
| Brief (20-40 s) | 32 | 29 / 25 / 4 | 29 / 24 / 2 | 31 / 29 / 18 | 26: 25 / 23 / 12 (P 23 / 19 / 2) |
| Unrelated drop, inside | 32 | 32 / 31 / 23 | 32 / 31 / 23 | 32 / 32 / 29 | 28: 28 / 28 / 25 |
| Pair drop, inside | 19 | 19 / 19 / 12 | 19 / 19 / 11 | 19 / 19 / 19 | 19: 19 / 19 / 19 |

- Wrong confident: P 2 and D 2, both the Kemal - Animation (Silent Extent Remix) upload over the original Animation (seed 2028, mix-02; also in the whole-mix search, as in 0182); M 0, MH 0 (and 0 possible). Wrong possible: P 292, D 252, M 4 (Clockwork's files, an Aphrodite upload).
- What the mask leaves of A: from the played file 0 confident of 37; from a corpus rip 20 confident of 132; from a NAS file 95 confident and 22 possible of 271.
- With the played file as reference (11 quiet and brief As) M confirms all 11, 10 confidently, as at 262 tracks.
- Of the 12 quiet and brief Bs that M confirms and P does not, 5 have a plain detection.

**Against 0255.** Precision holds at NAS scale: no confident answer that is not playing, and the one related version the plain search names confidently is gone. Recall falls with the reference: quiet and brief confirmed 50 of 64 (P 40), confident 32 (P 8), against 62 and 47 at 262 tracks, where the played audio was the reference in 62 of 64. The rows with the played file match 0255's; the loss follows the reference.

**What it shows.** Most of the oracle's gain rests on A's own audio. From another rip the mask leaves A confident in a third of the NAS files' cases and in 15% of the corpus rips', and confirms 7 more quiet and brief Bs than plain (39 against 32, of 53) instead of nearly all. Confident Bs still rise from 6 to 22 of those 53. **Does not show** why a rip leaves A behind: a different master or lossy upload, an offset between a seek by timestamps and the stored peaks' timeline (0127), or its alignment (not measured), nor estimated timing. Data `data/0257-residual-oracle-nas-scale.json`.
