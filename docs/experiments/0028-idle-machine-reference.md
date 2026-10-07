# 0028: reference measurements of today's code on an idle machine

**Question.** Item 1 of session 4: where search time goes; time and peak
memory from 262 to 31,964 assets, without the load of 0012-0027.

**Command.** Session-start binaries, 10 workers, both ladders, through
`work/scripts/timed.sh` (`time -l`, load and swap logged; 3 rounds,
median [range]; background about 10% of the CPU): `scan --synthetic-copies
N` (the 56-minute development mix), `identify --config /dev/null`,
`index`; `sample` every 1 ms (262 assets) or 10 ms (26,462), stacks
attributed by function name. Data: `data/0028-idle-machine-reference.json`.

| Development scan | Assets | Wall s | CPU s | Peak footprint |
|---|--:|--:|--:|--:|
| Today's matcher; skip at 240 | 262 | 33.6 [33.5-34.0]; 31.1 [30.9-31.1] | 264; 238 | 399 MB; 293 MB |
| Today's matcher; skip at 240 | 8,122 | 261.5 [260.8-261.7]; 99.9 [99.8-101.6] | 1,926; 700 | 5.30 GB; 2.12 GB |
| Today's matcher; skip at 240 | 26,462 | 765.6 [764.0-778.5]; 240.3 [239.9-240.7] | 5,814; 1,710 | 9.80 GB; 4.43 GB |
| Today's matcher; skip at 240 | 31,964 | 912.3 [909.8-913.3]; 281.9 [281.4-282.1] | 6,925; 1,999 | 11.24 GB; 5.27 GB |

Every scan 11/11, 0 wrong; strongest chance on a copy 31 hits (today's),
25 (skip). Peaks repeat within 1% from 8,122 assets, wall within 2%.
`identify` at 262 assets: turntable 24.4 s [24.2-24.4], 169 s CPU, 286
MB; both playbacks 34.1 s [33.9-34.9], 264 s, 374 MB; skip at 240, both
(harness `memory`) 31.4 s [30.9-31.7], 240 s, 289 MB.

| CPU samples (today's; skip) | 262 assets | 26,462 assets |
|---|--:|--:|
| Analysing the query on each rung (STFT, peaks) | 80%; 88% | 3.6%; 12% |
| Sorting each window's hits | 17%; 6.8% | 87%; 77% |
| Clustering offsets; posting scans | 1.5%, 1.7%; 0.5%, 1.3% | 4.8%, 2.3%; 4.4%, 2.4% |
| Second pass; index build | -, 0.1%; 3.0%, 0.1% | -, 0.9%; 0.8%, 2.9% |

Decoding, distinct lines, chaining: 0.1% or less. **Indexing** 262 tracks
(29.3 h): 52.5 s [52.0-52.5], 312 s user, 153 s system, 953 MB; 0.20 s
per track, 1.8 s per hour of audio: 20,000 tracks in about 67 minutes.

**Conclusions.** (1) Search CPU grows linearly, about 0.21 s per asset
(today's) and 0.055 s (skip at 240, 3.5× cheaper at 31,964 assets). (2)
31,964 assets fit in 11.2 GB for an hour-long mix. (3) At scale the time
is sorting each window's hits; at 262 it is analysis on 82 rungs.
