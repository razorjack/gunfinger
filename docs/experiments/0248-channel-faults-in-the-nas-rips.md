# 0248: how many NAS rips carry a channel fault

**Question.** Brief 62, step 2 (pre-registered, `de3be59`): how many NAS
rips are mono, inverted in one channel, delayed, or one channel only?

**Change.** Harness only (`fcf385d`): `stereo-width` measures each file's
balance and channel correlation at lag 0 and strongest within ±80
samples (10 ms at 8 kHz), and classes it: one channel (beyond ±20 dB),
inverted (strongest -0.5 or less), delayed (strongest at 4 samples or
more, at least 0.5 and 0.05 above lag 0), else its width (0196).

**Command.** Classes checked first on session 42's faults: inverted 96
of 100, silent 100, 1 and 5 ms 96 each, no clean, noise or 0.2 ms render
flagged; 0197's copies 10 of 10 (brief 62, 03:33). Then `stereo-width
--other --seed 2026 --sample 2000 --exclude work/s62/census-exclude.txt`
(16 jobs, 6:23, read only; the exclusions: the root's Ed Rush & Optical
file, 0065's too-long files and mixed-CD folders, the ignore file);
`stereo_census.py`. Data: `data/0248-channel-faults-in-the-nas-rips.json`.

| Class (2,000 drawn of 40,663) | Scene (667 of 3,527) | Uploads (667 of 34,575) | Elsewhere (666 of 2,561) | Share, weighted (95%) |
|---|--:|--:|--:|--:|
| One channel | 0 | 0 | 0 | 0% (to 0.57%) |
| Inverted | 2 | 3 | 0 | 0.41% (0.14-1.25%), about 170 files |
| Delayed | 0 | 0 | 1 | 0.01% (to 0.59%) |
| Mono; nearly mono | 25; 207 | 31; 176 | 7; 139 | 4.3%; 26.4% |
| Narrow; wide | 395; 38 | 405; 52 | 474; 45 | 61.3%; 7.6% |

Median side to mid -17.3 dB (0196's corpus: -17.7). Intervals: each
kind's Wilson bounds added by weight.

**Flagged.** Plainly inverted (correlation -0.92 to -0.95, side 13-16 dB
above the mid): Usual Suspects - Advent (scene), Keltech - Axis, Plasmic
- The Weapon (uploads); We - Hello World (-0.76) likely. DJ Sense - Let
You Go (+0.62 at lag 0, -0.64 at 69 samples) is by inference a periodic
bass line, counted as registered. The delay: a `sety/` track whose left
channel is the right's, 40 samples (5 ms) late (correlation 1.00).

**Conclusion.** Channel faults are rare in the NAS rips: about 1 in 250
inverted (3-4 of the 5 by inspection), delays and missing channels 1 in
2,000 or fewer. By 0197 such a rip is missed whatever the query.
