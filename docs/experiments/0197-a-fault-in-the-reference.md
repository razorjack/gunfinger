# 0197: a stereo fault in the indexed file rather than the query

**Question.** Session 42, step 5b (design in brief 42, 06:34, and 07:01):
when the library's rip carries the fault and the mix is clean, does the
fault cost what it costs in the query (0194)?

**Change.** None to the code. `work/s42/ref.sh`: the first 10 indexed
draws' corpus files, decoded whole and encoded again as stereo MP3s at
128 kbit/s at 44.1 kHz, unchanged and with the right channel inverted,
silent, or 44 or 221 samples late (FFmpeg `pan` and `adelay`), each copy
in its own scratch library and store (`gunfinger index`, 0.8 s each;
`work/peaks` and the NAS store untouched). Each clean `stereo-control`
query of those draws (20: two speeds) searched with the CLI against each
copy's store alone (`identify --saved-index off --format json`), so the
CLI's merging of same-audio plays cannot hide a copy. The store holds
10 files, so this measures evidence, not wrong answers.

**Command.** `sh work/s42/step5b.sh` (also `regress session-32-linux`
with the run build, identical); `work/s42/ref.py`. Data:
`data/0197-a-fault-in-the-reference.json`.

| Copy indexed (20 clean queries) | Confident | Possible | Median hits vs unchanged copy | The same fault in the query (0194, of 80) |
|---|--:|--:|--:|--:|
| Unchanged (stereo, re-encoded) | 20 | 0 | 1.00 (658-1,320 hits) | 80 |
| Right channel inverted | 0 | 2 | 0.00 | 0 |
| Right channel silent | 19 | 1 | 0.80 | 78 |
| Right channel 1 ms late | 18 | 2 | 0.37 | 70 |
| Right channel 5 ms late | 4 | 16 | 0.20 | 37 |

No query had a confident play of another file of the store.

**Conclusion.** An inverted or silent channel costs the same in the rip
as in the query: an inverted one leaves nothing to match. Delays keep
the same share of hits (0.37 and 0.20 against 0.30 and 0.20) but fewer
confident queries here (5 ms: 4 of 20 against 37 of 80); the reason is
not measured. A rip with an inverted channel would be missed in every
mix, and its peak record, picked from the downmix, does not show it.
