# 0198: a stereo fault over the second half of generated mixes

**Question.** Session 42, step 5a (design in brief 42, 06:34): over a
stretch of a mix, with plays blended and cut as `mixes` makes them, does
a fault cost what it costs 30 s excerpts (0194)?

**Change.** Harness only (tested): `mixes --stereo-fault <condition>`,
short mixes only, renders each play in both channels (FFmpeg `pan` per
channel, one gain for the two) and faults the right channel from half
the mix's length to its end; stereo MP3 at 128 kbit/s, renders in
`work/mixes/<condition>-seed-<seed>/`. The mono path is unchanged: the
16 mixes of seed 2026 rendered again in a scratch work directory are
byte-identical to `work/mixes/seed-2026/`.

**Command.** `work/s42/mixes.sh`: `mixes --seed 2026|2027|2028 --count 8
--stereo-fault` each of four conditions at 262 tracks (23-26 s a run),
`seconds` over the 12 reports, `work/s42/mixes.py`. Data:
`data/0198-stereo-faults-in-generated-mixes.json`.

| 24 mixes, 223 indexed plays | Control | Inverted | Delay 5 ms | Noise 0 dB |
|---|--:|--:|--:|--:|
| Plays in the first half, confident | 91/99 | 91/99 | 91/99 | 91/99 |
| Plays across the middle, confident | 23/24 | 16/24 | 21/24 | 20/24 |
| Plays in the second half, confident | 95/100 | 1/100 | 37/100 | 39/100 |
| Seconds correct (of 7,721 indexed) | 94.1% | 48.1% | 77.3% | 75.2% |
| Seconds possible | 2.5% | 4.0% | 18.5% | 19.2% |
| Seconds unlabelled | 3.3% | 48.0% | 4.2% | 5.6% |
| Wrong confident plays | 0 | 0 | 0 | 0 |

Mislabelled seconds stay at 0.0-0.1% (noise: 4 seconds in one seed,
against 1 in its control); specificity 99.2-100% in every report.

**Conclusion.** Yes. With an inverted channel the search finds almost
nothing while it lasts: 1 of 100 plays in the faulted half is confident, and
nearly every second there is unlabelled. A 5 ms delay or noise at 0 dB
keeps about 2 in 5 plays confident and most of the rest possible, as on
the excerpts (0194: 37 and 29 of 80). Nothing is found wrongly.
