# 0246: a store built by the power sum (phase B)

**Question.** Brief 61's phase B: with the 262-track store built by the
variant and searched by its queries, do faults on either side recover,
and do the clean controls hold (the bar registered in brief 61)?

**Change.** Harness only: `variant-store` (records under
`Profile::POWER_SUM`, in `work/s61/`), `--front-end power-sum`. Its
downmix records equal `work/peaks`'s byte for byte (265 of 265).

**Command.** `work/s61/phase-b.sh` (02:35-02:47) at 262 tracks, against
phase A's downmix runs (0245); for the faulted rips, session 42's copies
of the first 10 indexed draws (0197) in place of their files, a store
per copy and front end, `robust --only stereo-control`. Data:
`data/0246-a-power-sum-store.json`.

| Confident (indexed) | Downmix | Power sum |
|---|--:|--:|
| `stereo-control`; inverted; silent (of 80) | 80; 0; 78 | 80; 80; 80 |
| Noise 10, 0, -10 dB; delay 0.2, 1, 5 ms | 80, 29, 0; 80, 70, 37 | 80, 23, 0; 80, 80, 80 |
| Mixes inverted from the middle: first half, across, second half | 91/99, 16/24, 1/100 | 91/99, 23/24, 94/100 |
| Mixes, control | 91/99, 23/24, 95/100 | the same |
| Rip unchanged, inverted, silent, 1 ms, 5 ms (of 20) | 20, 0, 19, 18, 4 | 20, 20, 20, 20, 20 |
| Mono `control` against those rips (of 80) | 80, 60, 80, 78, 67 | 80 each |
| Default conditions (mono renders) lower by 2 or more | | 6 of 38 |

The six: low-pass 800 Hz 80 to 76, needle skip 31 to 27, clipping 64 to
61, pink noise 10 dB 75 to 72, blend 0 dB 31 to 28, -6 dB 72 to 70; down
1: aac, beatmatched -6 and 0 dB, combined; up 1: opus, pitch ride,
broadcast (net 1,938 to 1,918 of 2,600: 23 lost, 3 won). Mono queries keep
0.93-0.99 of the hits, stereo ones 1.00 (medians against the downmix's).
Scans: all referenced tracks (11, 7, 9), the same files, hits 1.00,
1.01, 0.99. Wrong confident: none; wrong possible as in 0245.

**Conclusion.** Faults on either side recover in full: inverted, silent
and delayed channels in the query, in the rip, and over half a mix. The
registered bar is not met: 6 of 38 default conditions fall by 2-4 of
80. Their queries are mono renders: by inference the records now hold
the side's power, which a mono query lacks, so marginal excerpts lose a
few hits. Noise in one channel still costs more than under the downmix.
